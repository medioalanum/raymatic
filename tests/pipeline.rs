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
    assert_eq!(
        format!("{:?}", first.output.entries),
        format!("{:?}", second.output.entries)
    );
    assert_eq!(first.publication.content[0].address.as_path(), "/");
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
