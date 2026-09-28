use raymatic::{AppError, diagnostic, output, pipeline::evaluate};
use std::{fs, path::Path};

#[test]
fn valid_content_renders_expected_static_html() {
    let root = fixture("valid");
    let success = evaluate(root.path()).unwrap();
    output::commit(root.path(), &success.output).unwrap();
    assert_eq!(
        fs::read_to_string(root.path().join("output/index.html")).unwrap(),
        include_str!("fixtures/valid/expected.html").trim_end()
    );
}

#[test]
fn malformed_front_matter_is_structured_and_source_aware() {
    let root = fixture("malformed");
    let failure = evaluate(root.path()).unwrap_err();
    let AppError::InvalidPublication(diagnostics) = failure.error else {
        panic!("expected publication diagnostic")
    };
    assert_eq!(diagnostics[0].code.0, "CONTENT001");
    assert_eq!(
        diagnostics[0].primary.as_ref().unwrap().path,
        root.path().join("content/index.md")
    );
    assert!(diagnostics[0].primary.as_ref().unwrap().source.is_some());
}

#[test]
fn missing_title_is_a_source_aware_diagnostic() {
    let root = fixture("missing-title");
    let failure = evaluate(root.path()).unwrap_err();
    let AppError::InvalidPublication(diagnostics) = failure.error else {
        panic!("expected publication diagnostic")
    };
    assert_eq!(diagnostics[0].code.0, "CONTENT002");
    assert_eq!(
        diagnostics[0]
            .primary
            .as_ref()
            .unwrap()
            .source
            .as_ref()
            .unwrap()
            .line_column(0),
        Some((1, 1))
    );
}

#[test]
fn failed_validation_does_not_change_existing_output() {
    let root = fixture("missing-title");
    fs::create_dir(root.path().join("output")).unwrap();
    fs::write(root.path().join("output/index.html"), "old complete output").unwrap();
    assert!(evaluate(root.path()).is_err());
    assert_eq!(
        fs::read_to_string(root.path().join("output/index.html")).unwrap(),
        "old complete output"
    );
    assert!(!root.path().join("output.staging").exists());
}

#[test]
fn template_failure_is_a_structured_presentation_diagnostic() {
    let root = fixture("template-failure");
    let diagnostics = invalid_diagnostics(root.path());
    let diagnostic = diagnostics
        .iter()
        .find(|diagnostic| diagnostic.code.0 == "RENDER001")
        .unwrap();
    assert_eq!(
        diagnostic.primary.as_ref().unwrap().path,
        root.path().join("presentation/page.html")
    );
    assert!(
        diagnostic
            .explanation
            .as_deref()
            .unwrap()
            .contains("syntax")
    );
}

#[test]
fn route_collisions_are_stable_and_snapshot_their_explanation() {
    let root = fixture("route-collision");
    let diagnostics = invalid_diagnostics(root.path());
    let rendered = normalize(
        &diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.code.0 == "ADDR001")
            .map(diagnostic::render)
            .collect::<String>(),
        root.path(),
    );
    assert_eq!(
        rendered,
        include_str!("fixtures/route-collision/diagnostics.txt")
    );
}

#[test]
fn broken_references_are_source_aware_and_snapshot_their_explanation() {
    let root = fixture("broken-reference");
    let diagnostics = invalid_diagnostics(root.path());
    let rendered = normalize(
        &diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.code.0 == "REF001")
            .map(diagnostic::render)
            .collect::<String>(),
        root.path(),
    );
    assert_eq!(
        rendered,
        include_str!("fixtures/broken-reference/diagnostics.txt")
    );
}

#[test]
fn multiple_diagnostics_have_stable_path_then_code_order() {
    let root = fixture("multiple-errors");
    let first = invalid_diagnostics(root.path())
        .into_iter()
        .map(|diagnostic| (diagnostic.primary.unwrap().path, diagnostic.code.0))
        .collect::<Vec<_>>();
    let second = invalid_diagnostics(root.path())
        .into_iter()
        .map(|diagnostic| (diagnostic.primary.unwrap().path, diagnostic.code.0))
        .collect::<Vec<_>>();
    assert_eq!(first, second);
    assert_eq!(
        first.iter().map(|(_, code)| *code).collect::<Vec<_>>(),
        vec!["REF001", "CONTENT002"]
    );
}

fn normalize(text: &str, root: &Path) -> String {
    text.replace(&root.display().to_string(), "PLACEHOLDER")
}

fn invalid_diagnostics(root: &Path) -> Vec<raymatic::diagnostic::Diagnostic> {
    let failure = evaluate(root).unwrap_err();
    let AppError::InvalidPublication(diagnostics) = failure.error else {
        panic!("expected publication diagnostics")
    };
    diagnostics
}

fn fixture(name: &str) -> tempfile::TempDir {
    let root = tempfile::tempdir().unwrap();
    let source = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name);
    copy_tree(&source, root.path());
    root
}

fn copy_tree(from: &Path, to: &Path) {
    for entry in fs::read_dir(from).unwrap() {
        let entry = entry.unwrap();
        let target = to.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            fs::create_dir(&target).unwrap();
            copy_tree(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), target).unwrap();
        }
    }
}
