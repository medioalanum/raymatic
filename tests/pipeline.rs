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
fn publication_attributes_are_optional_and_reach_the_presentation() {
    let root = fixture("valid");
    fs::write(
        root.path().join("content/index.md"),
        "+++\ntitle = \"An article\"\ndate = \"2026-09-29\"\ncategory = \"Engineering\"\nsummary = \"A concise summary.\"\n+++\n\nHello.\n",
    )
    .unwrap();
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

    let rendered = match &result.output.entries[0].content {
        raymatic::output::OutputContent::Bytes(bytes) => String::from_utf8(bytes.clone()).unwrap(),
        raymatic::output::OutputContent::CopyFile { .. } => panic!("expected rendered HTML"),
    };
    assert!(rendered.contains(
        "An article|2026-09-29|Engineering|A concise summary.|<p>Hello.</p>"
    ));
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
    assert!(diagnostics.iter().any(|diagnostic| diagnostic.code.0 == "CONTENT001"));
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
