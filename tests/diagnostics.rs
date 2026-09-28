use raymatic::{
    content::SourceFile,
    diagnostic::{self, Diagnostic, DiagnosticCode, SemanticObject, Severity, SourceLabel},
};

#[test]
fn unicode_positions_use_byte_offsets_and_scalar_columns() {
    let source = SourceFile {
        path: "content/page.md".into(),
        text: "Olá\r\n世界!".into(),
    };
    assert_eq!(source.line_column(0), Some((1, 1)));
    assert_eq!(source.line_column(6), Some((2, 1)));
    assert_eq!(source.line_column(12), Some((2, 3)));
    assert_eq!(source.line_column(3), None);
    assert_eq!(source.line_column(99), None);
}

#[test]
fn diagnostic_retains_source_after_original_owner_is_dropped() {
    let source = SourceFile {
        path: "page.md".into(),
        text: "Olá\nlink".into(),
    };
    let diagnostic = Diagnostic {
        severity: Severity::Error,
        code: DiagnosticCode("REF001"),
        summary: "Internal reference does not resolve".into(),
        explanation: Some("No content produces this address.".into()),
        primary: Some(SourceLabel {
            path: source.path.clone(),
            span: Some(5..9),
            source: Some(source.clone()),
            message: None,
        }),
        related: vec![],
        object: Some(SemanticObject::Reference {
            target: "/missing/".into(),
        }),
        expected: Some("an address produced by the publication".into()),
        help: Some("Correct the target.".into()),
    };
    drop(source);
    assert_eq!(
        diagnostic::render(&diagnostic),
        include_str!("fixtures/reference-diagnostic.txt")
    );
}
