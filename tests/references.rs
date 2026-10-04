use raymatic::{AppError, output::OutputContent, pipeline::evaluate};
use std::fs;

#[test]
fn root_relative_reference_resolves_to_derived_address() {
    let root = project();
    write_content(root.path(), "index.md", "Home", "Read [about](/about).");
    write_content(root.path(), "about.md", "About", "About Raymatic.");

    let result = evaluate(root.path()).unwrap();
    let html = rendered_page(&result, "index.html");
    assert!(html.contains("href=\"/about/\""));
}

#[test]
fn reference_resolves_to_explicit_address_instead_of_source_path() {
    let root = project();
    write_content(
        root.path(),
        "index.md",
        "Home",
        "Read [notes](/notes/rust/#ownership).",
    );
    fs::write(
        root.path().join("content/article.md"),
        "+++\ntitle = \"Rust\"\naddress = \"/notes/rust/\"\n+++\n\n# Ownership\n",
    )
    .unwrap();

    let result = evaluate(root.path()).unwrap();
    let html = rendered_page(&result, "index.html");
    assert!(html.contains("href=\"/notes/rust/#ownership\""));
}

#[test]
fn content_relative_reference_resolves_from_the_source_directory() {
    let root = project();
    fs::create_dir(root.path().join("content/notes")).unwrap();
    write_content(
        root.path(),
        "notes/index.md",
        "Notes",
        "Read [the guide](guide#part).",
    );
    write_content(root.path(), "notes/guide.md", "Guide", "Guide.");

    let result = evaluate(root.path()).unwrap();
    let html = rendered_page(&result, "notes/index.html");
    assert!(html.contains("href=\"/notes/guide/#part\""));
}

#[test]
fn broken_internal_reference_reports_the_source_target() {
    let root = project();
    write_content(
        root.path(),
        "index.md",
        "Home",
        "Read [missing](/missing#details).",
    );

    let failure = evaluate(root.path()).unwrap_err();
    let AppError::InvalidPublication(diagnostics) = failure.error else {
        panic!("expected invalid publication");
    };
    let diagnostic = diagnostics
        .iter()
        .find(|diagnostic| diagnostic.code.0 == "REF001")
        .expect("expected reference diagnostic");
    assert_eq!(diagnostic.summary, "Internal reference does not resolve");
    assert_eq!(
        diagnostic.primary.as_ref().unwrap().path,
        root.path().join("content/index.md")
    );
    assert!(diagnostic.primary.as_ref().unwrap().span.is_some());
    assert!(
        diagnostic
            .explanation
            .as_deref()
            .unwrap()
            .contains("/missing/#details")
    );
}

#[test]
fn external_and_fragment_only_links_are_not_publication_references() {
    let root = project();
    write_content(
        root.path(),
        "index.md",
        "Home",
        "[web](https://example.com) [fragment](#section) [protocol](//example.com/path)",
    );

    let result = evaluate(root.path()).unwrap();
    assert!(result.publication.content[0].references.is_empty());
}

fn project() -> tempfile::TempDir {
    let root = tempfile::tempdir().unwrap();
    fs::create_dir(root.path().join("content")).unwrap();
    fs::create_dir(root.path().join("presentation")).unwrap();
    fs::write(
        root.path().join("presentation/page.html"),
        "<!doctype html><title>{{ title }}</title>{{ body }}",
    )
    .unwrap();
    root
}

fn write_content(root: &std::path::Path, name: &str, title: &str, body: &str) {
    fs::write(
        root.join("content").join(name),
        format!("+++\ntitle = \"{title}\"\n+++\n\n{body}\n"),
    )
    .unwrap();
}

fn rendered_page(result: &raymatic::pipeline::PipelineSuccess, path: &str) -> String {
    let entry = result
        .output
        .entries
        .iter()
        .find(|entry| entry.relative_path == std::path::Path::new(path))
        .expect("expected rendered page");
    match &entry.content {
        OutputContent::Bytes(bytes) => String::from_utf8(bytes.clone()).unwrap(),
        OutputContent::CopyFile { .. } => panic!("expected rendered page"),
    }
}
