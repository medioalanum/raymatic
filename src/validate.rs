//! Publication-wide rules consume semantic data and emit structured diagnostics.
use crate::{
    content::Publication,
    diagnostic::{Diagnostic, DiagnosticCode, SemanticObject, Severity, SourceLabel},
};
use std::collections::{BTreeMap, BTreeSet};

pub fn publication(publication: &Publication) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let mut addresses: BTreeMap<&str, Vec<&crate::content::Content>> = BTreeMap::new();
    for content in &publication.content {
        addresses
            .entry(content.address.as_path())
            .or_default()
            .push(content);
    }
    for (address, content) in addresses {
        if content.len() > 1 {
            for item in content {
                diagnostics.push(Diagnostic {
                    severity: Severity::Error,
                    code: DiagnosticCode("ADDR001"),
                    summary: "Route collision".into(),
                    explanation: Some(format!(
                        "Multiple content files derive the address {address}."
                    )),
                    primary: Some(SourceLabel {
                        path: item.source.path.clone(),
                        span: item
                            .attributes
                            .title
                            .span
                            .as_ref()
                            .map(|span| span.bytes.clone()),
                        source: Some(item.source.clone()),
                        message: Some("this content derives the colliding address".into()),
                    }),
                    related: vec![],
                    object: Some(SemanticObject::Content {
                        address: Some(item.address.clone()),
                    }),
                    expected: Some("a unique address for every content file".into()),
                    help: Some("Rename or move one of the colliding content files.".into()),
                });
            }
        }
    }
    let addresses: BTreeSet<&str> = publication
        .content
        .iter()
        .map(|item| item.address.as_path())
        .collect();
    for content in &publication.content {
        for reference in &content.references {
            if !addresses.contains(reference.target.as_str()) {
                diagnostics.push(Diagnostic {
                    severity: Severity::Error,
                    code: DiagnosticCode("REF001"),
                    summary: "Internal reference does not resolve".into(),
                    explanation: Some(format!(
                        "This content links to {} but no content produces that address.",
                        reference.target
                    )),
                    primary: Some(SourceLabel {
                        path: reference.span.source.path.clone(),
                        span: Some(reference.span.bytes.clone()),
                        source: Some(reference.span.source.clone()),
                        message: Some("unknown internal target".into()),
                    }),
                    related: vec![],
                    object: Some(SemanticObject::Reference {
                        target: reference.target.clone(),
                    }),
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
