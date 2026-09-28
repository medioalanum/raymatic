use raymatic::dev::{DevState, Rebuild, rebuild_once};
use std::{fs, path::Path};

#[test]
fn invalid_edit_keeps_last_preview_and_fix_recovers_with_new_revision() {
    let root = fixture("valid");
    let mut state = DevState::default();
    assert_eq!(
        rebuild_once(root.path(), &mut state).unwrap(),
        Rebuild::Updated
    );
    let first_preview =
        fs::read_to_string(root.path().join(".raymatic-preview/index.html")).unwrap();
    assert_eq!(state.revision, 1);

    fs::write(
        root.path().join("content/index.md"),
        "+++\n+++\n\n# Untitled\n",
    )
    .unwrap();
    assert_eq!(
        rebuild_once(root.path(), &mut state).unwrap(),
        Rebuild::Invalid
    );
    assert_eq!(state.revision, 1);
    assert_eq!(
        fs::read_to_string(root.path().join(".raymatic-preview/index.html")).unwrap(),
        first_preview
    );
    assert_eq!(state.last_diagnostics[0].code.0, "CONTENT002");

    fs::write(
        root.path().join("content/index.md"),
        "+++\ntitle = \"Recovered\"\n+++\n\n# Recovered\n",
    )
    .unwrap();
    assert_eq!(
        rebuild_once(root.path(), &mut state).unwrap(),
        Rebuild::Updated
    );
    assert_eq!(state.revision, 2);
    assert!(state.last_diagnostics.is_empty());
    assert!(
        fs::read_to_string(root.path().join(".raymatic-preview/index.html"))
            .unwrap()
            .contains("Recovered")
    );
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
