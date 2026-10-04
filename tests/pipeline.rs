use raymatic::{AppError, output::OutputContent, pipeline::evaluate, project::EnvironmentError};
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
    let paths: Vec<_> = first
        .output
        .entries
        .iter()
        .map(|entry| entry.relative_path.as_path())
        .collect();
    assert!(paths.contains(&std::path::Path::new("index.html")));
    assert!(paths.contains(&std::path::Path::new("sitemap.xml")));
    assert!(paths.contains(&std::path::Path::new("robots.txt")));
    assert!(first.timings.semantic.is_some());
    assert_eq!(
        format!("{:?}", first.output.entries),
        format!("{:?}", second.output.entries)
    );
    assert_eq!(first.publication.content[0].address.as_path(), "/");
}

#[test]
fn derived_reading_time_is_available_to_presentations() {
    let root = fixture("valid");
    fs::write(
        root.path().join("content/index.md"),
        "+++\ntitle = \"Reading\"\n+++\n\none two three four five six seven eight nine ten\n",
    )
    .unwrap();
    fs::write(
        root.path().join("presentation/page.html"),
        "{{ reading_minutes }}",
    )
    .unwrap();
    let result = evaluate(root.path()).unwrap();
    let OutputContent::Bytes(bytes) = &result.output.entries[0].content else {
        panic!("expected rendered HTML bytes")
    };
    assert_eq!(String::from_utf8_lossy(bytes), "1");
}

#[test]
fn seo_metadata_overrides_are_available_to_presentations() {
    let root = fixture("valid");
    fs::write(
        root.path().join("content/index.md"),
        "+++\ntitle = \"Source title\"\ncanonical = \"https://example.test/custom/\"\nog_title = \"Social title\"\nog_description = \"Social description\"\ntwitter_card = \"summary\"\n+++\n\nBody.\n",
    )
    .unwrap();
    fs::write(
        root.path().join("presentation/page.html"),
        "{{ canonical_url }}|{{ og_title }}|{{ og_description }}|{{ twitter_card }}",
    )
    .unwrap();
    let result = evaluate(root.path()).unwrap();
    let OutputContent::Bytes(bytes) = &result.output.entries[0].content else {
        panic!("expected rendered bytes")
    };
    assert_eq!(
        String::from_utf8_lossy(bytes),
        "https://example.test/custom/|Social title|Social description|summary"
    );
}

#[test]
fn optional_site_configuration_reaches_presentations() {
    let root = fixture("valid");
    fs::write(
        root.path().join("site.toml"),
        "title = \"My Notebook\"\nauthor = \"Ada\"\nlanguage = \"pt-BR\"\n",
    )
    .unwrap();
    fs::write(
        root.path().join("presentation/page.html"),
        "{{ site_title }}|{{ site_author }}|{{ site_language }}",
    )
    .unwrap();
    let result = evaluate(root.path()).unwrap();
    let OutputContent::Bytes(bytes) = &result.output.entries[0].content else {
        panic!("expected rendered bytes")
    };
    assert_eq!(String::from_utf8_lossy(bytes), "My Notebook|Ada|pt-BR");
}

#[test]
fn dated_content_receives_previous_and_next_navigation() {
    let root = fixture("valid");
    fs::write(
        root.path().join("content/older.md"),
        "+++\ntitle = \"Older\"\ndate = \"2025-01-01\"\n+++\n\nOlder.\n",
    )
    .unwrap();
    fs::write(
        root.path().join("content/newer.md"),
        "+++\ntitle = \"Newer\"\ndate = \"2027-01-01\"\n+++\n\nNewer.\n",
    )
    .unwrap();
    fs::write(
        root.path().join("presentation/page.html"),
        "{{ title }}|{% if previous %}{{ previous.title }}{% endif %}|{% if next %}{{ next.title }}{% endif %}",
    )
    .unwrap();
    let result = evaluate(root.path()).unwrap();
    let rendered = result
        .output
        .entries
        .iter()
        .find_map(|entry| {
            (entry.relative_path == std::path::Path::new("older/index.html")).then(|| match &entry
                .content
            {
                OutputContent::Bytes(bytes) => String::from_utf8_lossy(bytes).into_owned(),
                OutputContent::CopyFile { .. } => String::new(),
            })
        })
        .unwrap();
    assert_eq!(rendered, "Older||Newer");
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
    let mut png = b"\x89PNG\r\n\x1a\n".to_vec();
    png.extend_from_slice(&[0, 0, 0, 13, b'I', b'H', b'D', b'R']);
    png.extend_from_slice(&640_u32.to_be_bytes());
    png.extend_from_slice(&480_u32.to_be_bytes());
    fs::write(root.path().join("assets/hero.png"), png).unwrap();

    let result = evaluate(root.path()).unwrap();
    assert_eq!(result.publication.assets.len(), 3);
    assert_eq!(
        result.publication.assets[0].relative_path,
        std::path::Path::new("css/site.css")
    );
    assert_eq!(
        result.publication.assets[1].relative_path,
        std::path::Path::new("favicon.ico")
    );
    assert_eq!(
        result.publication.assets[2].mime.as_deref(),
        Some("image/png")
    );
    assert_eq!(result.publication.assets[2].width, Some(640));
    assert_eq!(result.publication.assets[2].height, Some(480));

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
    let AppError::InvalidPublication(diagnostics) = failure.error else {
        panic!("expected structured output diagnostic")
    };
    assert_eq!(diagnostics[0].code.0, "OUTPUT001");
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
fn root_content_uses_conventional_home_presentation_when_present() {
    let root = tempfile::tempdir().unwrap();
    fs::create_dir_all(root.path().join("content")).unwrap();
    fs::create_dir_all(root.path().join("presentation")).unwrap();
    fs::write(
        root.path().join("content/index.md"),
        "+++\ntitle = \"Home\"\n+++\n\nWelcome.\n",
    )
    .unwrap();
    fs::write(
        root.path().join("presentation/page.html"),
        "<article>page</article>",
    )
    .unwrap();
    fs::write(
        root.path().join("presentation/index.html"),
        "<main data-home> {{ body }} </main>",
    )
    .unwrap();

    let result = evaluate(root.path()).unwrap();
    let planned = result
        .output
        .entries
        .iter()
        .find(|entry| entry.relative_path == std::path::Path::new("index.html"))
        .unwrap();
    match &planned.content {
        OutputContent::Bytes(bytes) => {
            assert!(String::from_utf8_lossy(bytes).contains("data-home"))
        }
        OutputContent::CopyFile { .. } => panic!("home output must be rendered"),
    }
}

#[test]
fn home_presentation_receives_recent_publication_entries_in_stable_order() {
    let root = fixture("valid");
    fs::create_dir_all(root.path().join("content/notes")).unwrap();
    fs::write(
        root.path().join("content/notes/older.md"),
        "+++\ntitle = \"Older\"\ndate = \"2025-01-01\"\nsummary = \"Old summary\"\n+++\n\nOlder body.\n",
    )
    .unwrap();
    fs::write(
        root.path().join("content/notes/newer.md"),
        "+++\ntitle = \"Newer\"\ndate = \"2026-01-01\"\nsummary = \"New summary\"\n+++\n\nNewer body.\n",
    )
    .unwrap();
    fs::write(
        root.path().join("presentation/index.html"),
        "{% for item in recent %}{{ item.title }}|{{ item.date }}|{{ item.summary }}|{{ item.address }};{% endfor %}",
    )
    .unwrap();

    let result = evaluate(root.path()).unwrap();
    let rendered = rendered_html(&result);
    assert_eq!(
        rendered,
        "Newer|2026-01-01|New summary|/notes/newer/;Older|2025-01-01|Old summary|/notes/older/;"
    );
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
fn custom_attributes_remain_portable_and_reach_the_presentation() {
    let root = fixture("valid");
    fs::write(
        root.path().join("content/index.md"),
        "+++\ntitle = \"An article\"\ndate = \"2026-09-29\"\nsummary = \"A concise summary.\"\nreading_minutes = 4\nfeatured = true\ntags = [\"rust\", \"publishing\"]\n+++\n\nHello.\n",
    )
    .unwrap();
    fs::write(
        root.path().join("presentation/page.html"),
        "{{ title }}|{{ date }}|{{ summary }}|{{ attributes.reading_minutes }}|{{ attributes.featured }}|{{ attributes.tags|join(',') }}|{{ body }}",
    )
    .unwrap();

    let result = evaluate(root.path()).unwrap();
    let attributes = &result.publication.content[0].attributes;
    assert_eq!(attributes.custom["reading_minutes"].as_integer(), Some(4));
    assert_eq!(attributes.custom["featured"].as_bool(), Some(true));
    assert!(
        attributes.custom_spans["tags"].bytes.end > attributes.custom_spans["tags"].bytes.start
    );
    assert!(
        rendered_html(&result).contains(
            "An article|2026-09-29|A concise summary.|4|True|rust,publishing|<p>Hello.</p>"
        )
    );
}

#[test]
fn custom_front_matter_is_preserved_instead_of_rejected() {
    let root = fixture("valid");
    fs::write(
        root.path().join("content/index.md"),
        "+++\ntitle = \"An article\"\nmagic = \"hidden machinery\"\n+++\n\nHello.\n",
    )
    .unwrap();
    let result = evaluate(root.path()).unwrap();
    assert_eq!(
        result.publication.content[0].attributes.custom["magic"].as_str(),
        Some("hidden machinery")
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

#[test]
fn real_publication_fixture_preserves_editorial_intent() {
    let root = fixture("real-publication");
    let result = evaluate(root.path()).unwrap();
    assert_eq!(result.publication.content.len(), 3);
    assert!(result.output.entries.iter().any(|entry| {
        entry.relative_path == std::path::Path::new("notes/type-hints/index.html")
    }));
    assert!(
        result
            .output
            .entries
            .iter()
            .any(|entry| entry.relative_path == std::path::Path::new("brand.txt"))
    );
    let home = result
        .output
        .entries
        .iter()
        .find(|entry| entry.relative_path == std::path::Path::new("index.html"))
        .unwrap();
    let raymatic::output::OutputContent::Bytes(bytes) = &home.content else {
        panic!("expected rendered home")
    };
    let home = String::from_utf8(bytes.clone()).unwrap();
    assert!(home.contains("I finally stopped resisting type hints"));
    assert!(home.contains("/notes/type-hints/"));
}
