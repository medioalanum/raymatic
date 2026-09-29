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
    /// One-based line and Unicode scalar column; invalid UTF-8 boundaries return None.
    pub fn line_column(&self, offset: usize) -> Option<(usize, usize)> {
        let prefix = self.text.get(..offset)?;
        let line = prefix.bytes().filter(|byte| *byte == b'\n').count() + 1;
        let column = prefix.rsplit('\n').next()?.chars().count() + 1;
        Some((line, column))
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
    pub fn derive(source_relative_path: &std::path::Path) -> Self {
        let mut path = source_relative_path.with_extension("");
        if path.file_name().is_some_and(|name| name == "index") {
            path.pop();
        }
        let segment = path.to_string_lossy().replace('\\', "/");
        if segment.is_empty() {
            Self("/".into())
        } else {
            Self(format!("/{segment}/"))
        }
    }
    pub fn as_path(&self) -> &str {
        &self.0
    }
    // TODO: add the constructor alongside authoritative UX address derivation.
    // No unchecked public constructor: filesystem paths are not addresses.
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FrontMatter {
    title: Option<String>,
    date: Option<String>,
    category: Option<String>,
    summary: Option<String>,
}

pub fn parse(
    source_relative_path: &std::path::Path,
    source: SourceFile,
) -> Result<Content, Box<Diagnostic>> {
    let (front_matter, body_range) = split_front_matter(&source)?;
    let attributes: FrontMatter = toml::from_str(front_matter).map_err(|error| {
        Box::new(diagnostic(
            "CONTENT001",
            "Malformed front matter",
            &source,
            0..front_matter.len().min(source.text.len()),
            Some(error.to_string()),
            Some("valid TOML using supported publication attributes"),
            Some("Correct the front matter syntax or remove unsupported attributes."),
        ))
    })?;
    let title = attributes.title.ok_or_else(|| {
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
    Ok(Content {
        source: source.clone(),
        attributes: Attributes {
            title: spanned_attribute(title, "title", &source),
            date: attributes
                .date
                .map(|value| spanned_attribute(value, "date", &source)),
            category: attributes
                .category
                .map(|value| spanned_attribute(value, "category", &source)),
            summary: attributes
                .summary
                .map(|value| spanned_attribute(value, "summary", &source)),
        },
        address: Address::derive(source_relative_path),
        references: references(&source, body_range.clone()),
        body: MarkdownBody { source, body_range },
    })
}

fn spanned_attribute(value: String, name: &str, source: &SourceFile) -> Spanned<String> {
    let start = source.text.find(name).unwrap_or(0);
    Spanned {
        value,
        span: Some(SourceSpan {
            source: source.clone(),
            bytes: start..start + name.len(),
        }),
    }
}

fn references(source: &SourceFile, body_range: Range<usize>) -> Vec<InternalReference> {
    let body = source
        .text
        .get(body_range.clone())
        .expect("body range belongs to source");
    let mut references = Vec::new();
    let mut start = 0;
    while let Some(open_relative) = body[start..].find("](") {
        let target_start = start + open_relative + 2;
        let Some(close_relative) = body[target_start..].find(')') else {
            break;
        };
        let target_end = target_start + close_relative;
        let target = &body[target_start..target_end];
        if target.starts_with('/') {
            references.push(InternalReference {
                target: target.into(),
                span: SourceSpan {
                    source: source.clone(),
                    bytes: body_range.start + target_start..body_range.start + target_end,
                },
            });
        }
        start = target_end + 1;
    }
    references
}

fn split_front_matter(source: &SourceFile) -> Result<(&str, Range<usize>), Box<Diagnostic>> {
    let text = &source.text;
    let Some(after_opening) = text.strip_prefix("+++\n") else {
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
    if after_opening.starts_with("+++\n") {
        return Ok(("", 8..text.len()));
    }
    let Some(closing_relative) = after_opening.find("\n+++") else {
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
    let front_start = 4;
    let front_end = front_start + closing_relative;
    let body_start = front_end + 5;
    Ok((&text[front_start..front_end], body_start..text.len()))
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
    /// Assets may be binary, so provenance retains their path without decoding text.
    pub source: PathBuf,
    pub relative_path: PathBuf,
}
