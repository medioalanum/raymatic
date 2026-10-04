//! Publication-wide rules consume semantic data and emit structured diagnostics.
use crate::{
    content::Publication,
    diagnostic::{Diagnostic, DiagnosticCode, SemanticObject, Severity, SourceLabel},
};
use std::collections::{BTreeMap, BTreeSet};

pub fn publication(publication: &Publication) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    for content in &publication.content {
        if let Some(date) = &content.attributes.date {
            let bytes = date.value.as_bytes();
            let valid = bytes.len() == 10
                && bytes[4] == b'-'
                && bytes[7] == b'-'
                && bytes[0..4].iter().all(u8::is_ascii_digit)
                && bytes[5..7].iter().all(u8::is_ascii_digit)
                && bytes[8..10].iter().all(u8::is_ascii_digit)
                && (1..=12).contains(&((bytes[5] - b'0') * 10 + bytes[6] - b'0'))
                && (1..=31).contains(&((bytes[8] - b'0') * 10 + bytes[9] - b'0'));
            if !valid {
                diagnostics.push(metadata_diagnostic(
                    content,
                    &date.span,
                    "Invalid publication date",
                    "Dates must use the ISO form YYYY-MM-DD for deterministic ordering and feeds.",
                    "date = \"2026-10-04\"",
                    "Replace the value with a valid calendar date in YYYY-MM-DD form.",
                ));
            }
        }
        for tag in &content.attributes.tags {
            if tag.trim().is_empty() {
                diagnostics.push(metadata_diagnostic(
                    content,
                    &content.attributes.title.span,
                    "Empty tag",
                    "An empty tag cannot produce a stable taxonomy address.",
                    "tags = [\"rust\", \"publishing\"]",
                    "Remove the empty tag or give it a descriptive name.",
                ));
            }
        }
        if !content.attributes.draft {
            for reference in &content.asset_references {
                let relative = reference.target.trim_start_matches("/assets/");
                let exists = publication
                    .assets
                    .iter()
                    .any(|asset| asset.relative_path.to_string_lossy() == relative);
                if !exists {
                    diagnostics.push(Diagnostic {
                        severity: Severity::Error,
                        code: DiagnosticCode("ASSET001"),
                        summary: "Asset reference does not resolve".into(),
                        explanation: Some(format!(
                            "This image references {} but no matching file exists in assets/.",
                            reference.target
                        )),
                        primary: Some(SourceLabel {
                            path: reference.span.source.path.clone(),
                            span: Some(reference.span.bytes.clone()),
                            source: Some(reference.span.source.clone()),
                            message: Some("missing asset".into()),
                        }),
                        related: vec![],
                        object: Some(SemanticObject::Reference {
                            target: reference.target.clone(),
                        }),
                        expected: Some("a file at the corresponding path under assets/".into()),
                        help: Some("Add the file to assets/ or correct the image path.".into()),
                    });
                }
                if reference.alt_empty {
                    diagnostics.push(Diagnostic {
                        severity: Severity::Error,
                        code: DiagnosticCode("ASSET002"),
                        summary: "Image is missing alternative text".into(),
                        explanation: Some(
                            "Images need descriptive alternative text for accessible HTML.".into(),
                        ),
                        primary: Some(SourceLabel {
                            path: reference.span.source.path.clone(),
                            span: Some(reference.span.bytes.clone()),
                            source: Some(reference.span.source.clone()),
                            message: Some("add alt text between [ and ]".into()),
                        }),
                        related: vec![],
                        object: Some(SemanticObject::Reference {
                            target: reference.target.clone(),
                        }),
                        expected: Some("![Description](/assets/image.png)".into()),
                        help: Some("Describe the meaningful content of the image.".into()),
                    });
                }
            }
        }
    }
    let mut addresses: BTreeMap<&str, Vec<&crate::content::Content>> = BTreeMap::new();
    for content in publication
        .content
        .iter()
        .filter(|content| !content.attributes.draft)
    {
        addresses
            .entry(content.address.as_path())
            .or_default()
            .push(content);
    }
    for (address, content) in addresses {
        if content.len() > 1 {
            for item in content {
                let explicit = item.address_span.as_ref();
                diagnostics.push(Diagnostic {
                    severity: Severity::Error,
                    code: DiagnosticCode("ADDR001"),
                    summary: "Route collision".into(),
                    explanation: Some(format!(
                        "Multiple content files resolve to the address {address}."
                    )),
                    primary: Some(SourceLabel {
                        path: item.source.path.clone(),
                        span: explicit
                            .map(|span| span.bytes.clone())
                            .or_else(|| item.attributes.title.span.as_ref().map(|span| span.bytes.clone())),
                        source: Some(item.source.clone()),
                        message: Some(if explicit.is_some() {
                            "this content explicitly selects the colliding address".into()
                        } else {
                            "this content derives the colliding address".into()
                        }),
                    }),
                    related: vec![],
                    object: Some(SemanticObject::Content {
                        address: Some(item.address.clone()),
                    }),
                    expected: Some("a unique address for every content file".into()),
                    help: Some(if explicit.is_some() {
                        "Choose a different explicit address or remove it to use the path-derived default.".into()
                    } else {
                        "Rename or move this content, or give it a different explicit address.".into()
                    }),
                });
            }
        }
    }
    let addresses: BTreeSet<&str> = publication
        .content
        .iter()
        .filter(|content| !content.attributes.draft)
        .map(|item| item.address.as_path())
        .collect();
    for content in publication
        .content
        .iter()
        .filter(|content| !content.attributes.draft)
    {
        for reference in &content.references {
            if !addresses.contains(reference.target.address()) {
                let target = reference.target.as_written();
                diagnostics.push(Diagnostic {
                    severity: Severity::Error,
                    code: DiagnosticCode("REF001"),
                    summary: "Internal reference does not resolve".into(),
                    explanation: Some(format!(
                        "This content links to {target} but no content produces the address {}.",
                        reference.target.address()
                    )),
                    primary: Some(SourceLabel {
                        path: reference.span.source.path.clone(),
                        span: Some(reference.span.bytes.clone()),
                        source: Some(reference.span.source.clone()),
                        message: Some("unknown internal target".into()),
                    }),
                    related: vec![],
                    object: Some(SemanticObject::Reference { target }),
                    expected: Some("an address produced by another content item".into()),
                    help: Some(
                        "Correct the target or add content that produces this address.".into(),
                    ),
                });
            }
        }
    }
    diagnostics
}

fn metadata_diagnostic(
    content: &crate::content::Content,
    span: &Option<crate::content::SourceSpan>,
    summary: &str,
    explanation: &str,
    expected: &str,
    help: &str,
) -> Diagnostic {
    Diagnostic {
        severity: Severity::Error,
        code: DiagnosticCode("META001"),
        summary: summary.into(),
        explanation: Some(explanation.into()),
        primary: Some(SourceLabel {
            path: content.source.path.clone(),
            span: span.as_ref().map(|value| value.bytes.clone()),
            source: Some(content.source.clone()),
            message: Some("invalid editorial metadata".into()),
        }),
        related: vec![],
        object: Some(SemanticObject::Content {
            address: Some(content.address.clone()),
        }),
        expected: Some(expected.into()),
        help: Some(help.into()),
    }
}
