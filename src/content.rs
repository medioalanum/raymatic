//! Owned semantic data and source provenance, independent of parser libraries.
use crate::diagnostic::{Diagnostic, DiagnosticCode, SemanticObject, Severity, SourceLabel};
use serde::Deserialize;
use std::{ops::Range, path::PathBuf, sync::Arc};

#[derive(Clone, Debug)]
pub struct SourceFile {
    pub path: PathBuf,
    pub text: Arc<str>,
}

impl SourceFile {
    pub fn line_column(&self, offset: usize) -> Option<(usize, usize)> {
        let prefix = self.text.get(..offset)?;
        Some((
            prefix.bytes().filter(|b| *b == b'\n').count() + 1,
            prefix.rsplit('\n').next()?.chars().count() + 1,
        ))
    }
}

#[derive(Clone, Debug)]
pub struct SourceSpan {
    pub source: SourceFile,
    pub bytes: Range<usize>,
}

#[derive(Clone, Debug)]
pub struct Spanned<T> {
    pub value: T,
    pub span: Option<SourceSpan>,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Address(String);

impl Address {
    pub fn derive(p: &std::path::Path) -> Self {
        let mut p = p.with_extension("");
        if p.file_name().is_some_and(|n| n == "index") {
            p.pop();
        }
        let s = p.to_string_lossy().replace('\\', "/");
        if s.is_empty() {
            Self("/".into())
        } else {
            Self(format!("/{s}/"))
        }
    }

    fn explicit(v: String) -> Result<Self, &'static str> {
        if v == "/" {
            return Ok(Self(v));
        }
        if !v.starts_with('/') || !v.ends_with('/') {
            return Err("an address must start and end with /");
        }
        if v.contains("//") {
            return Err("an address cannot contain empty path segments");
        }
        if v.contains('\\') || v.contains('?') || v.contains('#') {
            return Err("an address must contain only a public path, without \\, ? or #");
        }
        if v.trim_matches('/')
            .split('/')
            .any(|s| s == "." || s == "..")
        {
            return Err("an address cannot contain . or .. path segments");
        }
        Ok(Self(v))
    }

    pub fn as_path(&self) -> &str {
        &self.0
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FrontMatter {
    title: Option<String>,
    date: Option<String>,
    category: Option<String>,
    summary: Option<String>,
    address: Option<String>,
    presentation: Option<String>,
}

pub fn parse(relative: &std::path::Path, source: SourceFile) -> Result<Content, Box<Diagnostic>> {
    let (fm, body_range) = split_front_matter(&source)?;
    let a: FrontMatter = toml::from_str(fm).map_err(|e| {
        Box::new(diagnostic(
            "CONTENT001",
            "Malformed front matter",
            &source,
            0..fm.len().min(source.text.len()),
            Some(e.to_string()),
            Some("valid TOML using supported publication attributes"),
            Some("Correct the front matter syntax or remove unsupported attributes."),
        ))
    })?;
    let title = a.title.ok_or_else(|| {
        Box::new(diagnostic(
            "CONTENT002",
            "Missing required attribute: title",
            &source,
            0..3,
            Some("The publication needs a title for this content.".into()),
            Some("title = \"A title\""),
            Some("Add a title to the front matter."),
        ))
    })?;
    let (address, address_span) = match a.address {
        Some(v) => {
            let span = attribute_span("address", &source);
            let address = Address::explicit(v).map_err(|r| {
                Box::new(diagnostic(
                    "ADDR002",
                    "Invalid explicit address",
                    &source,
                    span.bytes.clone(),
                    Some(r.into()),
                    Some("/ for the home page, or a canonical path such as /notes/rust/"),
                    Some(
                        "Use a leading and trailing slash and remove query, fragment, or traversal segments.",
                    ),
                ))
            })?;
            (address, Some(span))
        }
        None => (Address::derive(relative), None),
    };
    let presentation = a
        .presentation
        .map(|v| spanned_attribute(v, "presentation", &source));
    Ok(Content {
        source: source.clone(),
        attributes: Attributes {
            title: spanned_attribute(title, "title", &source),
            date: a.date.map(|v| spanned_attribute(v, "date", &source)),
            category: a
                .category
                .map(|v| spanned_attribute(v, "category", &source)),
            summary: a.summary.map(|v| spanned_attribute(v, "summary", &source)),
        },
        address,
        address_span,
        presentation,
        references: references(&source, body_range.clone()),
        body: MarkdownBody { source, body_range },
    })
}

fn attribute_span(name: &str, source: &SourceFile) -> SourceSpan {
    let start = source.text.find(name).unwrap_or(0);
    SourceSpan {
        source: source.clone(),
        bytes: start..start + name.len(),
    }
}

fn spanned_attribute(v: String, n: &str, s: &SourceFile) -> Spanned<String> {
    Spanned {
        value: v,
        span: Some(attribute_span(n, s)),
    }
}

fn references(source: &SourceFile, body_range: Range<usize>) -> Vec<InternalReference> {
    let body = source
        .text
        .get(body_range.clone())
        .expect("body range belongs to source");
    let mut out = vec![];
    let mut start = 0;
    while let Some(o) = body[start..].find("](") {
        let ts = start + o + 2;
        let Some(c) = body[ts..].find(')') else {
            break;
        };
        let te = ts + c;
        let target = &body[ts..te];
        if target.starts_with('/') {
            out.push(InternalReference {
                target: target.into(),
                span: SourceSpan {
                    source: source.clone(),
                    bytes: body_range.start + ts..body_range.start + te,
                },
            });
        }
        start = te + 1;
    }
    out
}

fn split_front_matter(source: &SourceFile) -> Result<(&str, Range<usize>), Box<Diagnostic>> {
    let text = &source.text;
    let Some(after) = text.strip_prefix("+++\n") else {
        return Err(Box::new(diagnostic(
            "CONTENT001",
            "Malformed front matter",
            source,
            0..0,
            Some("Content must begin with a TOML front-matter delimiter.".into()),
            Some("+++ followed by TOML and a closing +++ line"),
            Some("Start the file with +++."),
        )));
    };
    if after.starts_with("+++\n") {
        return Ok(("", 8..text.len()));
    }
    let Some(close) = after.find("\n+++") else {
        return Err(Box::new(diagnostic(
            "CONTENT001",
            "Malformed front matter",
            source,
            0..text.len(),
            Some("The opening front-matter delimiter has no closing delimiter.".into()),
            Some("a closing +++ line"),
            Some("Add a closing +++ line."),
        )));
    };
    let fs = 4;
    let fe = fs + close;
    Ok((&text[fs..fe], fe + 5..text.len()))
}

fn diagnostic(
    code: &'static str,
    summary: &str,
    source: &SourceFile,
    span: Range<usize>,
    explanation: Option<String>,
    expected: Option<&str>,
    help: Option<&str>,
) -> Diagnostic {
    Diagnostic {
        severity: Severity::Error,
        code: DiagnosticCode(code),
        summary: summary.into(),
        explanation,
        primary: Some(SourceLabel {
            path: source.path.clone(),
            span: Some(span),
            source: Some(source.clone()),
            message: None,
        }),
        related: vec![],
        object: Some(SemanticObject::Content { address: None }),
        expected: expected.map(Into::into),
        help: help.map(Into::into),
    }
}

#[derive(Debug)]
pub struct Publication {
    pub root: PathBuf,
    pub content: Vec<Content>,
    pub presentation: Presentation,
    pub assets: Vec<Asset>,
}

#[derive(Debug)]
pub struct Content {
    pub source: SourceFile,
    pub attributes: Attributes,
    pub address: Address,
    pub address_span: Option<SourceSpan>,
    pub presentation: Option<Spanned<String>>,
    pub body: MarkdownBody,
    pub references: Vec<InternalReference>,
}

#[derive(Debug)]
pub struct Attributes {
    pub title: Spanned<String>,
    pub date: Option<Spanned<String>>,
    pub category: Option<Spanned<String>>,
    pub summary: Option<Spanned<String>>,
}

#[derive(Debug)]
pub struct MarkdownBody {
    pub source: SourceFile,
    pub body_range: Range<usize>,
}

#[derive(Debug)]
pub struct InternalReference {
    pub target: String,
    pub span: SourceSpan,
}

#[derive(Debug)]
pub struct Presentation {
    pub source: SourceFile,
    pub template: String,
}

#[derive(Debug)]
pub struct Asset {
    pub source: PathBuf,
    pub relative_path: PathBuf,
}
