use raymatic::{AppError, pipeline::evaluate, project::EnvironmentError};
use std::fs;

#[test]
fn empty_directory_requires_the_conventional_content_file() {
    let root = tempfile::tempdir().unwrap();
    let failure = evaluate(root.path()).unwrap_err();
    assert!(matches!(
        failure.error,
        AppError::Environment(EnvironmentError::Inspect { .. })
    ));
    assert!(failure.timings.commit.is_none());
}

#[test]
fn missing_root_is_an_environment_error() {
    let root = tempfile::tempdir().unwrap();
    let failure = evaluate(&root.path().join("missing")).unwrap_err();
    assert!(matches!(
        failure.error,
        AppError::Environment(EnvironmentError::Inspect { .. })
    ));
}

#[test]
fn a_regular_file_is_not_a_project_directory() {
    let file = tempfile::NamedTempFile::new().unwrap();
    let failure = evaluate(file.path()).unwrap_err();
    assert!(matches!(
        failure.error,
        AppError::Environment(EnvironmentError::NotDirectory(_))
    ));
}

#[test]
fn valid_fixture_creates_a_deterministic_output_plan() {
    let root = fixture("valid");
    let first = evaluate(root.path()).unwrap();
    let second = evaluate(root.path()).unwrap();
    assert_eq!(first.output.entries.len(), 1);
    assert_eq!(
        first.output.entries[0].relative_path,
        std::path::Path::new("index.html")
    );
    assert!(first.timings.semantic.is_some());
    assert_eq!(
        format!("{:?}", first.output.entries),
        format!("{:?}", second.output.entries)
    );
    assert_eq!(first.publication.content[0].address.as_path(), "/");
}

#[test]
fn conventional_assets_are_preserved_and_planned_as_copies() {
    let root = fixture("valid");
    fs::create_dir_all(root.path().join("assets/css")).unwrap();
    fs::write(
        root.path().join("assets/css/site.css"),
        "body { margin: 0; }\n",
    )
    .unwrap();
    fs::write(root.path().join("assets/favicon.ico"), [0_u8, 1, 2, 3]).unwrap();

    let result = evaluate(root.path()).unwrap();
    assert_eq!(result.publication.assets.len(), 2);
    assert_eq!(
        result.publication.assets[0].relative_path,
        std::path::Path::new("css/site.css")
    );
    assert_eq!(
        result.publication.assets[1].relative_path,
        std::path::Path::new("favicon.ico")
    );

    let css = result
        .output
        .entries
        .iter()
        .find(|entry| entry.relative_path == std::path::Path::new("css/site.css"))
        .unwrap();
    assert!(
        matches!(&css.content, raymatic::output::OutputContent::CopyFile { source } if source == &root.path().join("assets/css/site.css"))
    );
    let icon = result
        .output
        .entries
        .iter()
        .find(|entry| entry.relative_path == std::path::Path::new("favicon.ico"))
        .unwrap();
    assert!(
        matches!(&icon.content, raymatic::output::OutputContent::CopyFile { source } if source == &root.path().join("assets/favicon.ico"))
    );
}

#[test]
fn asset_output_collision_is_rejected() {
    let root = fixture("valid");
    fs::create_dir_all(root.path().join("assets")).unwrap();
    fs::write(
        root.path().join("assets/index.html"),
        "not the rendered page",
    )
    .unwrap();

    let failure = evaluate(root.path()).unwrap_err();
    assert!(matches!(
        failure.error,
        AppError::Internal("Invalid output plan")
    ));
}

#[test]
fn default_presentation_remains_page_html_without_configuration() {
    let root = fixture("valid");
    let result = evaluate(root.path()).unwrap();
    assert!(result.publication.content[0].presentation.is_none());
    let rendered = rendered_html(&result);
    assert!(rendered.contains("<title>Hello Raymatic</title>"));
    assert!(rendered.contains("<h1>Hello</h1>"));
}

#[test]
fn explicit_presentation_selects_named_template() {
    let root = fixture("valid");
    fs::write(
        root.path().join("content/index.md"),
        "+++\ntitle = \"An article\"\npresentation = \"article\"\n+++\n\nHello.\n",
    )
    .unwrap();
    fs::write(
        root.path().join("presentation/article.html"),
        "<article data-presentation=\"article\">{{ title }}|{{ body }}</article>",
    )
    .unwrap();
    let result = evaluate(root.path()).unwrap();
    assert_eq!(
        result.publication.content[0]
            .presentation
            .as_ref()
            .unwrap()
            .value,
        "article"
    );
    assert!(
        rendered_html(&result)
            .contains("<article data-presentation=\"article\">An article|<p>Hello.</p>")
    );
}

#[test]
fn missing_explicit_presentation_is_source_aware() {
    let root = fixture("valid");
    fs::write(
        root.path().join("content/index.md"),
        "+++\ntitle = \"An article\"\npresentation = \"missing\"\n+++\n\nHello.\n",
    )
    .unwrap();
    let failure = evaluate(root.path()).unwrap_err();
    let AppError::InvalidPublication(diagnostics) = failure.error else {
        panic!("expected invalid publication");
    };
    let diagnostic = diagnostics
        .iter()
        .find(|diagnostic| diagnostic.code.0 == "PRESENTATION001")
        .expect("expected presentation diagnostic");
    assert_eq!(diagnostic.summary, "Presentation not found");
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
            .contains("presentation/missing.html")
    );
}

#[test]
fn explicit_address_overrides_the_path_derived_default() {
    let root = fixture("valid");
    fs::write(
        root.path().join("content/index.md"),
        "+++\ntitle = \"An article\"\naddress = \"/notes/rust/\"\n+++\n\nHello.\n",
    )
    .unwrap();
    let result = evaluate(root.path()).unwrap();
    assert_eq!(
        result.publication.content[0].address.as_path(),
        "/notes/rust/"
    );
    assert!(result.publication.content[0].address_span.is_some());
    assert_eq!(
        result.output.entries[0].relative_path,
        std::path::Path::new("notes/rust/index.html")
    );
}

#[test]
fn invalid_explicit_address_is_rejected_with_an_address_diagnostic() {
    let root = fixture("valid");
    fs::write(
        root.path().join("content/index.md"),
        "+++\ntitle = \"An article\"\naddress = \"notes/../rust\"\n+++\n\nHello.\n",
    )
    .unwrap();
    let failure = evaluate(root.path()).unwrap_err();
    let AppError::InvalidPublication(diagnostics) = failure.error else {
        panic!("expected invalid publication");
    };
    let diagnostic = diagnostics
        .iter()
        .find(|diagnostic| diagnostic.code.0 == "ADDR002")
        .expect("expected invalid address diagnostic");
    assert_eq!(diagnostic.summary, "Invalid explicit address");
    assert!(diagnostic.primary.as_ref().unwrap().span.is_some());
}

#[test]
fn publication_attributes_are_optional_and_reach_the_presentation() {
    let root = fixture("valid");
    fs::write(root.path().join("content/index.md"), "+++\ntitle = \"An article\"\ndate = \"2026-09-29\"\ncategory = \"Engineering\"\nsummary = \"A concise summary.\"\n+++\n\nHello.\n").unwrap();
    fs::write(
        root.path().join("presentation/page.html"),
        "{{ title }}|{{ date }}|{{ category }}|{{ summary }}|{{ body }}",
    )
    .unwrap();
    let result = evaluate(root.path()).unwrap();
    let attributes = &result.publication.content[0].attributes;
    assert_eq!(attributes.title.value, "An article");
    assert_eq!(attributes.date.as_ref().unwrap().value, "2026-09-29");
    assert_eq!(attributes.category.as_ref().unwrap().value, "Engineering");
    assert_eq!(
        attributes.summary.as_ref().unwrap().value,
        "A concise summary."
    );
    assert!(
        rendered_html(&result)
            .contains("An article|2026-09-29|Engineering|A concise summary.|<p>Hello.</p>")
    );
}

#[test]
fn unsupported_front_matter_is_rejected_instead_of_silently_ignored() {
    let root = fixture("valid");
    fs::write(
        root.path().join("content/index.md"),
        "+++\ntitle = \"An article\"\nmagic = \"hidden machinery\"\n+++\n\nHello.\n",
    )
    .unwrap();
    let failure = evaluate(root.path()).unwrap_err();
    let AppError::InvalidPublication(diagnostics) = failure.error else {
        panic!("expected invalid publication");
    };
    assert!(
        diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code.0 == "CONTENT001")
    );
}

fn rendered_html(result: &raymatic::pipeline::PipelineSuccess) -> String {
    let entry = result
        .output
        .entries
        .iter()
        .find(|entry| matches!(entry.content, raymatic::output::OutputContent::Bytes(_)))
        .expect("expected rendered HTML");
    match &entry.content {
        raymatic::output::OutputContent::Bytes(bytes) => String::from_utf8(bytes.clone()).unwrap(),
        raymatic::output::OutputContent::CopyFile { .. } => unreachable!(),
    }
}

fn fixture(name: &str) -> tempfile::TempDir {
    let root = tempfile::tempdir().unwrap();
    let source = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name);
    copy_tree(&source, root.path());
    root
}

fn copy_tree(from: &std::path::Path, to: &std::path::Path) {
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
