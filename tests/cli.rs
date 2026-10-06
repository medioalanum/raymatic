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
        assert!(
            stderr.contains("non-empty directory")
                || stderr.contains("Cannot inspect")
                || stderr.contains("Expected a directory"),
            "{command}: {stderr}"
        );
        assert_eq!(
            fs::read_to_string(root.path().join("output/index.html")).unwrap(),
            "existing publication"
        );
        assert_eq!(fs::read_dir(root.path()).unwrap().count(), 1);
    }
}

#[test]
fn new_creates_a_valid_publication_with_a_documented_structure() {
    let root = tempfile::tempdir().unwrap();
    let output = ray()
        .current_dir(root.path())
        .args(["new", "my site"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let project = root.path().join("my site");
    for path in ["content/index.md", "presentation/page.html", "README.md"] {
        assert!(project.join(path).is_file(), "{path}");
    }
    fs::write(
        project.join("content/notes/recent.md"),
        "+++\ntitle = \"A recent note\"\ndate = \"2026-10-04\"\nsummary = \"A generated index entry.\"\n+++\n\nA note.\n",
    )
    .unwrap();
    assert!(
        ray()
            .current_dir(&project)
            .arg("check")
            .status()
            .unwrap()
            .success()
    );
    assert!(
        ray()
            .current_dir(&project)
            .arg("build")
            .status()
            .unwrap()
            .success()
    );
    assert!(project.join("output/index.html").is_file());
    let html = fs::read_to_string(project.join("output/index.html")).unwrap();
    assert!(html.contains("A recent note"));
    assert!(html.contains("A generated index entry."));
    assert!(html.contains("rel=\"canonical\""));
    assert!(html.contains("property=\"og:title\""));
    assert!(html.contains("name=\"twitter:card\""));
}

#[test]
fn new_refuses_to_replace_an_existing_project() {
    let root = tempfile::tempdir().unwrap();
    let target = root.path().join("my site");
    fs::create_dir(&target).unwrap();
    fs::write(target.join("keep.txt"), "keep").unwrap();
    let output = ray()
        .current_dir(root.path())
        .args(["new", "my site"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    assert!(
        String::from_utf8(output.stderr)
            .unwrap()
            .contains("non-empty directory")
    );
    assert_eq!(fs::read_to_string(target.join("keep.txt")).unwrap(), "keep");
}

#[test]
fn pelican_inspection_is_non_destructive_and_import_creates_a_native_project() {
    let root = tempfile::tempdir().unwrap();
    let source = root.path().join("pelican");
    fs::create_dir_all(source.join("content/pages")).unwrap();
    fs::write(
        source.join("pelicanconf.py"),
        "SITENAME = 'Example'\nAUTHOR = 'Ada'\nPLUGINS = ['toc']\nTHEME = 'theme'\nARTICLE_URL = '{slug}/'\nSTATIC_PATHS = ['images']\n",
    )
    .unwrap();
    fs::write(
        source.join("content/hello.md"),
        "Title: Hello\nDate: 2026-10-06 10:00\nStatus: draft\nTags: rust, publishing\nSlug: hello\nAlias: /old-hello/\n\nRead [About](pages/about.md).\n",
    )
    .unwrap();
    fs::write(
        source.join("content/pages/about.md"),
        "Title: About\n\nAbout.\n",
    )
    .unwrap();
    fs::write(source.join("content/logo.txt"), "asset").unwrap();
    let before = fs::read_to_string(source.join("content/hello.md")).unwrap();
    let inspected = ray()
        .current_dir(root.path())
        .args(["migrate", "inspect", "pelican"])
        .output()
        .unwrap();
    assert!(inspected.status.success());
    let report = String::from_utf8(inspected.stdout).unwrap();
    assert!(report.contains("source: Pelican"));
    assert!(
        report.contains("| `content/pages/about.md` | transformable | Markdown to native page")
    );
    assert!(report.contains("plugins: detected; not executed"));
    assert!(report.contains("URL or SAVE_AS pattern: detected"));
    assert!(report.contains("STATIC_PATHS: detected"));
    let second_report = ray()
        .current_dir(root.path())
        .args(["migrate", "inspect", "pelican"])
        .output()
        .unwrap();
    assert_eq!(report, String::from_utf8(second_report.stdout).unwrap());
    assert_eq!(
        fs::read_to_string(source.join("content/hello.md")).unwrap(),
        before
    );
    assert!(
        ray()
            .current_dir(root.path())
            .args(["migrate", "import", "pelican", "imported"])
            .status()
            .unwrap()
            .success()
    );
    let imported = fs::read_to_string(root.path().join("imported/content/hello.md")).unwrap();
    assert!(imported.contains("title = \"Hello\""));
    assert!(imported.contains("address = \"/hello/\""));
    assert!(imported.contains("date = \"2026-10-06\""));
    assert!(imported.contains("draft = true"));
    assert!(imported.contains("aliases = [\"/old-hello/\"]"));
    assert!(imported.contains("[About](about/)"));
    assert!(
        fs::read_to_string(root.path().join("imported/content/about.md"))
            .unwrap()
            .contains("kind = \"page\"")
    );
    assert_eq!(
        fs::read_to_string(root.path().join("imported/assets/logo.txt")).unwrap(),
        "asset"
    );
    assert!(
        fs::read_to_string(root.path().join("imported/site.toml"))
            .unwrap()
            .contains("title = \"Example\"")
    );
    assert!(
        ray()
            .current_dir(root.path().join("imported"))
            .arg("check")
            .status()
            .unwrap()
            .success()
    );
}
