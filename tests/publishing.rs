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
fn successful_build_generates_sitemap_and_robots_without_drafts() {
    let root = fixture("valid");
    fs::write(
        root.path().join("content/draft.md"),
        "+++\ntitle = \"Unreleased\"\ndraft = true\n+++\n\nThis must not publish.\n",
    )
    .unwrap();
    let success = evaluate(root.path()).unwrap();
    output::commit(root.path(), &success.output).unwrap();
    let sitemap = fs::read_to_string(root.path().join("output/sitemap.xml")).unwrap();
    assert!(sitemap.contains("<loc>/</loc>"));
    assert!(!sitemap.contains("Unreleased"));
    assert_eq!(
        fs::read_to_string(root.path().join("output/robots.txt")).unwrap(),
        "User-agent: *\nAllow: /\nSitemap: /sitemap.xml\n"
    );
    assert!(!root.path().join("output/draft/index.html").exists());
    let rss = fs::read_to_string(root.path().join("output/feed.xml")).unwrap();
    let atom = fs::read_to_string(root.path().join("output/atom.xml")).unwrap();
    assert!(rss.contains("<rss version=\"2.0\">") && !rss.contains("Unreleased"));
    assert!(atom.contains("http://www.w3.org/2005/Atom") && !atom.contains("Unreleased"));
}

#[test]
fn invalid_editorial_date_has_a_recovery_diagnostic() {
    let root = fixture("valid");
    fs::write(
        root.path().join("content/index.md"),
        "+++\ntitle = \"Home\"\ndate = \"04/10/2026\"\n+++\n\nHome.\n",
    )
    .unwrap();
    let AppError::InvalidPublication(diagnostics) = evaluate(root.path()).unwrap_err().error else {
        panic!("expected publication diagnostics")
    };
    let diagnostic = diagnostics
        .iter()
        .find(|diagnostic| diagnostic.code.0 == "META001")
        .unwrap();
    assert!(diagnostic.summary.contains("date"));
    assert!(
        diagnostic
            .expected
            .as_deref()
            .unwrap()
            .contains("2026-10-04")
    );
    assert!(diagnostic.help.as_deref().unwrap().contains("Replace"));
}

#[test]
fn missing_image_asset_has_a_source_aware_diagnostic() {
    let root = fixture("valid");
    fs::write(
        root.path().join("content/index.md"),
        "+++\ntitle = \"Home\"\n+++\n\n![Missing](/assets/missing.png)\n",
    )
    .unwrap();
    let AppError::InvalidPublication(diagnostics) = evaluate(root.path()).unwrap_err().error else {
        panic!("expected publication diagnostics")
    };
    let diagnostic = diagnostics
        .iter()
        .find(|diagnostic| diagnostic.code.0 == "ASSET001")
        .unwrap();
    assert_eq!(
        diagnostic.primary.as_ref().unwrap().path,
        root.path().join("content/index.md")
    );
    assert!(diagnostic.help.as_deref().unwrap().contains("assets/"));
}

#[test]
fn image_query_strings_resolve_and_empty_alt_text_is_rejected() {
    let root = fixture("valid");
    fs::create_dir(root.path().join("assets")).unwrap();
    fs::write(root.path().join("assets/photo.png"), [0_u8, 1, 2]).unwrap();
    fs::write(
        root.path().join("content/index.md"),
        "+++\ntitle = \"Home\"\n+++\n\n![](/assets/photo.png?v=1#hero)\n",
    )
    .unwrap();
    let AppError::InvalidPublication(diagnostics) = evaluate(root.path()).unwrap_err().error else {
        panic!("expected publication diagnostics")
    };
    assert!(
        diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code.0 == "ASSET002")
    );
    assert!(
        !diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code.0 == "ASSET001")
    );
}

#[test]
fn output_collisions_are_structured_publication_diagnostics() {
    let root = fixture("valid");
    fs::create_dir(root.path().join("assets")).unwrap();
    fs::write(root.path().join("assets/index.html"), "shadow").unwrap();
    let AppError::InvalidPublication(diagnostics) = evaluate(root.path()).unwrap_err().error else {
        panic!("expected output collision diagnostics")
    };
    let diagnostic = diagnostics
        .iter()
        .find(|diagnostic| diagnostic.code.0 == "OUTPUT001")
        .unwrap();
    assert!(
        diagnostic
            .explanation
            .as_deref()
            .unwrap()
            .contains("index.html")
    );
    assert!(diagnostic.help.as_deref().unwrap().contains("Rename"));
}

#[test]
fn successful_build_generates_archive_taxonomy_pages_and_feeds() {
    let root = fixture("valid");
    fs::write(
        root.path().join("content/notes.md"),
        "+++\ntitle = \"Rust Notes\"\ndate = \"2026-10-04\"\nauthor = \"Raymatic Author\"\ncategory = \"Engineering\"\ntags = [\"rust\", \"publishing\"]\n+++\n\nA note.\n",
    )
    .unwrap();
    let success = evaluate(root.path()).unwrap();
    output::commit(root.path(), &success.output).unwrap();
    assert!(root.path().join("output/archive/index.html").is_file());
    let category =
        fs::read_to_string(root.path().join("output/categories/engineering/index.html")).unwrap();
    assert!(category.contains("Rust Notes"));
    assert!(
        root.path()
            .join("output/categories/engineering/feed.xml")
            .is_file()
    );
    assert!(root.path().join("output/tags/rust/index.html").is_file());
    assert!(
        root.path()
            .join("output/tags/publishing/feed.xml")
            .is_file()
    );
    assert!(
        fs::read_to_string(root.path().join("output/feed.xml"))
            .unwrap()
            .contains("Raymatic Author")
    );
}

#[test]
fn explicit_pages_do_not_enter_article_derived_surfaces() {
    let root = fixture("valid");
    fs::write(
        root.path().join("content/about.md"),
        "+++\ntitle = \"About\"\nkind = \"page\"\ncategory = \"Engineering\"\ntags = [\"publishing\"]\n+++\n\nAbout this publication.\n",
    )
    .unwrap();
    let success = evaluate(root.path()).unwrap();
    output::commit(root.path(), &success.output).unwrap();
    assert!(root.path().join("output/about/index.html").is_file());
    assert!(
        !root
            .path()
            .join("output/categories/engineering/index.html")
            .exists()
    );
    let feed = fs::read_to_string(root.path().join("output/feed.xml")).unwrap();
    assert!(!feed.contains("About"));
}

#[test]
fn aliases_emit_static_redirect_pages() {
    let root = fixture("valid");
    fs::write(
        root.path().join("content/notes.md"),
        "+++\ntitle = \"Notes\"\naddress = \"/notes/\"\naliases = [\"/old-notes/\"]\n+++\n\nNotes.\n",
    )
    .unwrap();
    let success = evaluate(root.path()).unwrap();
    output::commit(root.path(), &success.output).unwrap();
    let redirect = fs::read_to_string(root.path().join("output/old-notes/index.html")).unwrap();
    assert!(redirect.contains("url=/notes/"));
    assert!(redirect.contains("href=\"/notes/\""));
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
        .replace('\\', "/")
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
