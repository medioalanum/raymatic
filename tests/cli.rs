use std::{fs, process::Command};

fn ray() -> Command {
    Command::new(env!("CARGO_BIN_EXE_ray"))
}

#[test]
fn help_and_version_succeed() {
    let output = ray().arg("--help").output().unwrap();
    assert!(output.status.success());
    let help = String::from_utf8(output.stdout).unwrap();
    for name in ["new", "dev", "check", "build"] {
        assert!(help.contains(name));
    }
    let version = ray().arg("--version").output().unwrap();
    assert!(version.status.success());
    assert_eq!(
        String::from_utf8(version.stdout).unwrap(),
        format!("ray {}\n", env!("CARGO_PKG_VERSION"))
    );
}

#[test]
fn invalid_cli_input_is_rejected() {
    for args in [
        vec![],
        vec!["publish"],
        vec!["check", "--unknown"],
        vec!["build", "unexpected"],
    ] {
        assert_eq!(ray().args(args).output().unwrap().status.code(), Some(2));
    }
}

#[test]
fn commands_fail_without_modifying_existing_files_when_publication_is_invalid() {
    let root = tempfile::tempdir().unwrap();
    fs::create_dir(root.path().join("output")).unwrap();
    fs::write(
        root.path().join("output/index.html"),
        "existing publication",
    )
    .unwrap();
    for command in ["new", "check", "build", "dev"] {
        let output = ray()
            .current_dir(root.path())
            .arg(command)
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(1), "{command}");
        assert!(output.stdout.is_empty());
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(stderr.contains("not implemented") || stderr.contains("Cannot inspect"));
        assert_eq!(
            fs::read_to_string(root.path().join("output/index.html")).unwrap(),
            "existing publication"
        );
        assert_eq!(fs::read_dir(root.path()).unwrap().count(), 1);
    }
}

#[test]
fn new_does_not_claim_to_create_a_publication() {
    let root = tempfile::tempdir().unwrap();
    let output = ray()
        .current_dir(root.path())
        .args(["new", "my site"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    assert!(!root.path().join("my site").exists());
}
