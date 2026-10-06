//! Owned semantic data and source provenance, independent of parser libraries.
use crate::diagnostic::{Diagnostic, DiagnosticCode, SemanticObject, Severity, SourceLabel};
use serde::Deserialize;
use std::{collections::BTreeMap, ops::Range, path::PathBuf, sync::Arc};

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

    pub fn explicit(v: String) -> Result<Self, &'static str> {
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
struct FrontMatter {
    title: Option<String>,
    kind: Option<String>,
    date: Option<String>,
    category: Option<String>,
    tags: Option<Vec<String>>,
    author: Option<String>,
    draft: Option<bool>,
    language: Option<String>,
    og_image: Option<String>,
    canonical: Option<String>,
    og_title: Option<String>,
    og_description: Option<String>,
    twitter_card: Option<String>,
    summary: Option<String>,
    address: Option<String>,
    aliases: Option<Vec<String>>,
    presentation: Option<String>,
    #[serde(flatten)]
    custom: BTreeMap<String, toml::Value>,
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
    let kind = match a.kind.as_deref() {
        Some("home") => ContentKind::Home,
        Some("page") => ContentKind::Page,
        Some("article") => ContentKind::Article,
        Some(value) => {
            return Err(Box::new(diagnostic(
                "CONTENT003",
                "Invalid content kind",
                &source,
                attribute_span("kind", &source).bytes,
                Some(format!("`{value}` is not a supported content kind.")),
                Some("kind = \"home\", \"page\", or \"article\""),
                Some(
                    "Use a supported content kind or remove the attribute to use the path convention.",
                ),
            )));
        }
        None if relative == std::path::Path::new("index.md") && address.as_path() == "/" => {
            ContentKind::Home
        }
        None => ContentKind::Article,
    };
    let aliases = a
        .aliases
        .unwrap_or_default()
        .into_iter()
        .map(|value| {
            Address::explicit(value).map_err(|reason| {
                Box::new(diagnostic(
                    "ADDR003",
                    "Invalid alias address",
                    &source,
                    attribute_span("aliases", &source).bytes,
                    Some(reason.into()),
                    Some("aliases = [\"/former-address/\"]"),
                    Some("Use absolute slash-delimited public paths for aliases."),
                ))
            })
        })
        .collect::<Result<Vec<_>, Box<Diagnostic>>>()?;
    let presentation = a
        .presentation
        .map(|v| spanned_attribute(v, "presentation", &source));
    let mut custom = a.custom;
    if let Some(tags) = &a.tags {
        custom.insert(
            "tags".into(),
            toml::Value::Array(tags.iter().cloned().map(toml::Value::String).collect()),
        );
    }
    Ok(Content {
        source: source.clone(),
        attributes: Attributes {
            title: spanned_attribute(title, "title", &source),
            date: a.date.map(|v| spanned_attribute(v, "date", &source)),
            category: a
                .category
                .map(|v| spanned_attribute(v, "category", &source)),
            tags: a.tags.unwrap_or_default(),
            author: a.author.map(|v| spanned_attribute(v, "author", &source)),
            draft: a.draft.unwrap_or(false),
            language: a
                .language
                .map(|v| spanned_attribute(v, "language", &source)),
            og_image: a
                .og_image
                .map(|v| spanned_attribute(v, "og_image", &source)),
            canonical: a
                .canonical
                .map(|v| spanned_attribute(v, "canonical", &source)),
            og_title: a
                .og_title
                .map(|v| spanned_attribute(v, "og_title", &source)),
            og_description: a
                .og_description
                .map(|v| spanned_attribute(v, "og_description", &source)),
            twitter_card: a
                .twitter_card
                .map(|v| spanned_attribute(v, "twitter_card", &source)),
            summary: a.summary.map(|v| spanned_attribute(v, "summary", &source)),
            custom_spans: custom
                .keys()
                .map(|name| (name.clone(), attribute_span(name, &source)))
                .collect(),
            custom,
        },
        kind,
        address,
        address_span,
        aliases,
        presentation,
        references: references(relative, &source, body_range.clone()),
        asset_references: asset_references(&source, body_range.clone()),
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

fn references(
    relative: &std::path::Path,
    source: &SourceFile,
    body_range: Range<usize>,
) -> Vec<InternalReference> {
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
        let image = body[..start + o]
            .rfind('[')
            .is_some_and(|index| index > 0 && body.as_bytes()[index - 1] == b'!');
        if !image && let Some(target) = ReferenceTarget::parse(target, relative) {
            out.push(InternalReference {
                target,
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

fn asset_references(source: &SourceFile, body_range: Range<usize>) -> Vec<AssetReference> {
    let body = source
        .text
        .get(body_range.clone())
        .expect("body range belongs to source");
    let mut out = vec![];
    let mut start = 0;
    while let Some(offset) = body[start..].find("](") {
        let close_start = start + offset;
        let image = body[..close_start]
            .rfind('[')
            .is_some_and(|index| index > 0 && body.as_bytes()[index - 1] == b'!');
        if !image {
            start = close_start + 2;
            continue;
        }
        let target_start = close_start + 2;
        let Some(close) = body[target_start..].find(')') else {
            break;
        };
        let target_end = target_start + close;
        let target = &body[target_start..target_end];
        let suffix_start = target.find(['?', '#']).unwrap_or(target.len());
        let asset_path = &target[..suffix_start];
        if asset_path.starts_with("/assets/") {
            let alt_start = body[..close_start].rfind('[').map(|index| index + 1);
            out.push(AssetReference {
                target: asset_path.to_owned(),
                alt_empty: alt_start.is_some_and(|start| body[start..close_start].is_empty()),
                span: SourceSpan {
                    source: source.clone(),
                    bytes: body_range.start + target_start..body_range.start + target_end,
                },
            });
        }
        start = target_end + 1;
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
    pub site: SiteConfig,
    pub content: Vec<Content>,
    pub presentation: Presentation,
    pub assets: Vec<Asset>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct SiteConfig {
    pub title: Option<String>,
    pub author: Option<String>,
    pub description: Option<String>,
    pub language: Option<String>,
    pub base_url: Option<String>,
    #[serde(default)]
    pub social_links: Vec<String>,
}

#[derive(Debug)]
pub struct Content {
    pub source: SourceFile,
    pub attributes: Attributes,
    pub kind: ContentKind,
    pub address: Address,
    pub address_span: Option<SourceSpan>,
    pub aliases: Vec<Address>,
    pub presentation: Option<Spanned<String>>,
    pub body: MarkdownBody,
    pub references: Vec<InternalReference>,
    pub asset_references: Vec<AssetReference>,
}

/// Native publication intent. It controls participation in derived surfaces,
/// independently from a content file's path.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ContentKind {
    Home,
    Page,
    Article,
}

#[derive(Debug)]
pub struct Attributes {
    pub title: Spanned<String>,
    pub date: Option<Spanned<String>>,
    pub category: Option<Spanned<String>>,
    pub tags: Vec<String>,
    pub author: Option<Spanned<String>>,
    pub draft: bool,
    pub language: Option<Spanned<String>>,
    pub og_image: Option<Spanned<String>>,
    pub canonical: Option<Spanned<String>>,
    pub og_title: Option<Spanned<String>>,
    pub og_description: Option<Spanned<String>>,
    pub twitter_card: Option<Spanned<String>>,
    pub summary: Option<Spanned<String>>,
    pub custom: BTreeMap<String, toml::Value>,
    pub custom_spans: BTreeMap<String, SourceSpan>,
}

#[derive(Debug)]
pub struct MarkdownBody {
    pub source: SourceFile,
    pub body_range: Range<usize>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReferenceTarget {
    address: String,
    suffix: String,
}

impl ReferenceTarget {
    fn parse(value: &str, relative: &std::path::Path) -> Option<Self> {
        if value.starts_with("//")
            || value.starts_with("http:")
            || value.starts_with("https:")
            || value.starts_with("mailto:")
            || value.starts_with('#')
            || value.starts_with('?')
        {
            return None;
        }
        let suffix_start = value.find(['?', '#']).unwrap_or(value.len());
        let written_path = &value[..suffix_start];
        let path = if written_path.starts_with('/') {
            written_path.to_owned()
        } else {
            let mut components = vec![];
            if let Some(parent) = relative.parent() {
                components.extend(
                    parent
                        .components()
                        .filter_map(|component| component.as_os_str().to_str()),
                );
            }
            for component in written_path.split('/') {
                match component {
                    "" | "." => {}
                    ".." => {
                        components.pop();
                    }
                    value => components.push(value),
                }
            }
            format!("/{}", components.join("/"))
        };
        if path.is_empty() {
            return None;
        }
        let address = if path == "/" || path.ends_with('/') {
            path
        } else {
            format!("{path}/")
        };
        Some(Self {
            address,
            suffix: value[suffix_start..].into(),
        })
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn as_written(&self) -> String {
        format!("{}{}", self.address, self.suffix)
    }
}

#[derive(Debug)]
pub struct InternalReference {
    pub target: ReferenceTarget,
    pub span: SourceSpan,
}

#[derive(Debug)]
pub struct AssetReference {
    pub target: String,
    pub alt_empty: bool,
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
    pub mime: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

pub fn image_metadata(path: &std::path::Path) -> (Option<String>, Option<u32>, Option<u32>) {
    let mime = match path.extension().and_then(|extension| extension.to_str()) {
        Some("png") => Some("image/png".into()),
        Some("jpg" | "jpeg") => Some("image/jpeg".into()),
        Some("gif") => Some("image/gif".into()),
        Some("webp") => Some("image/webp".into()),
        _ => None,
    };
    let bytes = std::fs::read(path).ok();
    let dimensions = bytes.as_deref().and_then(|bytes| {
        if bytes.starts_with(b"\x89PNG\r\n\x1a\n") && bytes.len() >= 24 {
            Some((
                u32::from_be_bytes(bytes[16..20].try_into().ok()?),
                u32::from_be_bytes(bytes[20..24].try_into().ok()?),
            ))
        } else if bytes.starts_with(&[0xff, 0xd8]) {
            jpeg_dimensions(bytes)
        } else if bytes.starts_with(b"RIFF") && bytes.get(8..12) == Some(b"WEBP") {
            webp_dimensions(bytes)
        } else {
            None
        }
    });
    (
        mime,
        dimensions.map(|value| value.0),
        dimensions.map(|value| value.1),
    )
}

fn webp_dimensions(bytes: &[u8]) -> Option<(u32, u32)> {
    if bytes.get(12..16) != Some(b"VP8X") || bytes.len() < 30 {
        return None;
    }
    let width =
        1 + (u32::from(bytes[24]) | (u32::from(bytes[25]) << 8) | (u32::from(bytes[26]) << 16));
    let height =
        1 + (u32::from(bytes[27]) | (u32::from(bytes[28]) << 8) | (u32::from(bytes[29]) << 16));
    Some((width, height))
}

fn jpeg_dimensions(bytes: &[u8]) -> Option<(u32, u32)> {
    let mut index = 2;
    while index + 9 < bytes.len() {
        if bytes[index] != 0xff {
            index += 1;
            continue;
        }
        let marker = bytes[index + 1];
        let length = u16::from_be_bytes([bytes[index + 2], bytes[index + 3]]) as usize;
        if (0xc0..=0xc3).contains(&marker)
            || (0xc5..=0xc7).contains(&marker)
            || (0xc9..=0xcb).contains(&marker)
            || (0xcd..=0xcf).contains(&marker)
        {
            return Some((
                u16::from_be_bytes([bytes[index + 7], bytes[index + 8]]) as u32,
                u16::from_be_bytes([bytes[index + 5], bytes[index + 6]]) as u32,
            ));
        }
        index = index.saturating_add(2 + length);
    }
    None
}
