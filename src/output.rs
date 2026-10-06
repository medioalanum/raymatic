//! Complete output plans are validated before a staged production commit.
use crate::{
    AppError,
    content::{Address, Asset},
    diagnostic::{Diagnostic, DiagnosticCode, SemanticObject, Severity},
    project::EnvironmentError,
};
use std::{
    collections::BTreeMap,
    collections::BTreeSet,
    fs,
    path::{Component, Path, PathBuf},
};

#[derive(Debug)]
pub struct OutputPlan {
    pub entries: Vec<OutputEntry>,
}

#[derive(Clone, Debug)]
pub struct FeedEntry {
    pub title: String,
    pub summary: Option<String>,
    pub date: Option<String>,
    pub address: String,
    pub category: Option<String>,
    pub tags: Vec<String>,
    pub author: Option<String>,
}

#[derive(Debug)]
pub struct OutputEntry {
    pub relative_path: PathBuf,
    pub content: OutputContent,
    pub origin: OutputOrigin,
}

#[derive(Debug)]
pub enum OutputContent {
    Bytes(Vec<u8>),
    CopyFile { source: PathBuf },
}

#[derive(Debug)]
pub enum OutputOrigin {
    Content { source: PathBuf, address: Address },
    Asset { source: PathBuf },
    Generated,
}

pub fn plan(
    pages: Vec<(String, PathBuf, Address)>,
    assets: &[Asset],
) -> Result<OutputPlan, AppError> {
    plan_with_feeds(pages, assets, &[], None)
}

pub fn plan_with_feeds(
    pages: Vec<(String, PathBuf, Address)>,
    assets: &[Asset],
    feed_entries: &[FeedEntry],
    base_url: Option<&str>,
) -> Result<OutputPlan, AppError> {
    plan_with_feeds_and_redirects(pages, assets, feed_entries, base_url, &[])
}

pub fn plan_with_feeds_and_redirects(
    pages: Vec<(String, PathBuf, Address)>,
    assets: &[Asset],
    feed_entries: &[FeedEntry],
    base_url: Option<&str>,
    redirects: &[(Address, Address)],
) -> Result<OutputPlan, AppError> {
    let absolute = |path: &str| absolute_url(base_url, path);
    let sitemap = sitemap(&pages, base_url);
    let mut entries: Vec<OutputEntry> = pages
        .into_iter()
        .map(|(content, source, address)| OutputEntry {
            relative_path: output_path(&address),
            content: OutputContent::Bytes(content.into_bytes()),
            origin: OutputOrigin::Content { source, address },
        })
        .collect();
    entries.extend(assets.iter().map(|asset| OutputEntry {
        relative_path: asset.relative_path.clone(),
        content: OutputContent::CopyFile {
            source: asset.source.clone(),
        },
        origin: OutputOrigin::Asset {
            source: asset.source.clone(),
        },
    }));
    entries.extend(redirects.iter().map(|(from, to)| OutputEntry {
        relative_path: output_path(from),
        content: OutputContent::Bytes(redirect_page(to.as_path()).into_bytes()),
        origin: OutputOrigin::Generated,
    }));
    entries.sort_by(|left, right| left.relative_path.cmp(&right.relative_path));
    entries.push(OutputEntry {
        relative_path: PathBuf::from("sitemap.xml"),
        content: OutputContent::Bytes(sitemap.into_bytes()),
        origin: OutputOrigin::Generated,
    });
    entries.push(asset_manifest(assets));
    add_archive_and_taxonomies(&mut entries, feed_entries, base_url);
    entries.push(OutputEntry {
        relative_path: PathBuf::from("robots.txt"),
        content: OutputContent::Bytes(b"User-agent: *\nAllow: /\nSitemap: /sitemap.xml\n".to_vec()),
        origin: OutputOrigin::Generated,
    });
    entries.push(OutputEntry {
        relative_path: PathBuf::from("feed.xml"),
        content: OutputContent::Bytes(rss_feed(feed_entries, &absolute).into_bytes()),
        origin: OutputOrigin::Generated,
    });
    entries.push(OutputEntry {
        relative_path: PathBuf::from("atom.xml"),
        content: OutputContent::Bytes(atom_feed(feed_entries, &absolute).into_bytes()),
        origin: OutputOrigin::Generated,
    });
    let plan = OutputPlan { entries };
    validate(&plan)?;
    Ok(plan)
}

fn redirect_page(target: &str) -> String {
    let mut html = String::from(
        "<!doctype html><html><head><meta charset=\"utf-8\"><meta http-equiv=\"refresh\" content=\"0; url=",
    );
    escape_html_into(&mut html, target);
    html.push_str("\"><link rel=\"canonical\" href=\"");
    escape_html_into(&mut html, target);
    html.push_str("\"></head><body><p>This page has moved to <a href=\"");
    escape_html_into(&mut html, target);
    html.push_str("\">");
    escape_html_into(&mut html, target);
    html.push_str("</a>.</p></body></html>\n");
    html
}

fn add_archive_and_taxonomies(
    entries: &mut Vec<OutputEntry>,
    feed_entries: &[FeedEntry],
    base_url: Option<&str>,
) {
    let mut archive = feed_entries.to_vec();
    archive.sort_by(|left, right| {
        right
            .date
            .cmp(&left.date)
            .then(left.address.cmp(&right.address))
    });
    entries.push(generated_page("archive/index.html", "Archive", &archive));

    let mut categories: BTreeMap<String, Vec<FeedEntry>> = BTreeMap::new();
    let mut tags: BTreeMap<String, Vec<FeedEntry>> = BTreeMap::new();
    for entry in feed_entries {
        if let Some(category) = &entry.category {
            categories
                .entry(category.clone())
                .or_default()
                .push(entry.clone());
        }
        for tag in &entry.tags {
            tags.entry(tag.clone()).or_default().push(entry.clone());
        }
    }
    for (name, items) in categories {
        let slug = slug(&name);
        entries.push(generated_page(
            &format!("categories/{slug}/index.html"),
            &format!("Category: {name}"),
            &items,
        ));
        entries.push(generated_feed(
            &format!("categories/{slug}/feed.xml"),
            &items,
            base_url,
        ));
    }
    for (name, items) in tags {
        let slug = slug(&name);
        entries.push(generated_page(
            &format!("tags/{slug}/index.html"),
            &format!("Tag: {name}"),
            &items,
        ));
        entries.push(generated_feed(
            &format!("tags/{slug}/feed.xml"),
            &items,
            base_url,
        ));
    }
}

fn generated_page(path: &str, title: &str, entries: &[FeedEntry]) -> OutputEntry {
    let mut html = String::from(
        "<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"><title>",
    );
    escape_html_into(&mut html, title);
    html.push_str("</title></head><body><main><h1>");
    escape_html_into(&mut html, title);
    html.push_str("</h1><ul>");
    for entry in entries {
        html.push_str("<li><a href=\"");
        escape_html_into(&mut html, &entry.address);
        html.push_str("\">");
        escape_html_into(&mut html, &entry.title);
        html.push_str("</a>");
        if let Some(date) = &entry.date {
            html.push_str(" <time>");
            escape_html_into(&mut html, date);
            html.push_str("</time>");
        }
        html.push_str("</li>");
    }
    html.push_str("</ul></main></body></html>\n");
    OutputEntry {
        relative_path: path.into(),
        content: OutputContent::Bytes(html.into_bytes()),
        origin: OutputOrigin::Generated,
    }
}

fn asset_manifest(assets: &[Asset]) -> OutputEntry {
    let mut json = String::from("{\n  \"assets\": [\n");
    for (index, asset) in assets.iter().enumerate() {
        if index > 0 {
            json.push_str(",\n");
        }
        json.push_str("    {\"path\":\"");
        escape_json_into(
            &mut json,
            &format!(
                "/{}",
                asset.relative_path.to_string_lossy().replace('\\', "/")
            ),
        );
        json.push_str("\",\"mime\":");
        json_string_or_null(&mut json, asset.mime.as_deref());
        json.push_str(",\"width\":");
        json_number_or_null(&mut json, asset.width);
        json.push_str(",\"height\":");
        json_number_or_null(&mut json, asset.height);
        json.push('}');
    }
    json.push_str("\n  ]\n}\n");
    OutputEntry {
        relative_path: "assets-manifest.json".into(),
        content: OutputContent::Bytes(json.into_bytes()),
        origin: OutputOrigin::Generated,
    }
}

fn escape_json_into(output: &mut String, value: &str) {
    for character in value.chars() {
        match character {
            '\\' => output.push_str("\\\\"),
            '"' => output.push_str("\\\""),
            '\n' => output.push_str("\\n"),
            character => output.push(character),
        }
    }
}

fn json_string_or_null(output: &mut String, value: Option<&str>) {
    match value {
        Some(value) => {
            output.push('"');
            escape_json_into(output, value);
            output.push('"');
        }
        None => output.push_str("null"),
    }
}

fn json_number_or_null(output: &mut String, value: Option<u32>) {
    match value {
        Some(value) => output.push_str(&value.to_string()),
        None => output.push_str("null"),
    }
}

fn generated_feed(path: &str, entries: &[FeedEntry], base_url: Option<&str>) -> OutputEntry {
    OutputEntry {
        relative_path: path.into(),
        content: OutputContent::Bytes(
            rss_feed(entries, &|path| absolute_url(base_url, path)).into_bytes(),
        ),
        origin: OutputOrigin::Generated,
    }
}

fn slug(value: &str) -> String {
    let mut slug = String::new();
    for character in value.chars().flat_map(char::to_lowercase) {
        if character.is_ascii_alphanumeric() {
            slug.push(character);
        } else if !slug.ends_with('-') {
            slug.push('-');
        }
    }
    slug.trim_matches('-').to_owned()
}

fn escape_html_into(output: &mut String, value: &str) {
    for character in value.chars() {
        match character {
            '&' => output.push_str("&amp;"),
            '<' => output.push_str("&lt;"),
            '>' => output.push_str("&gt;"),
            '"' => output.push_str("&quot;"),
            '\'' => output.push_str("&#39;"),
            character => output.push(character),
        }
    }
}

fn rss_feed(entries: &[FeedEntry], absolute: &impl Fn(&str) -> String) -> String {
    let mut xml = String::from(&format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<rss version=\"2.0\"><channel><title>Raymatic publication</title><link>{}</link><description>Published content</description>\n",
        absolute("/")
    ));
    for entry in entries {
        xml.push_str("<item><title>");
        escape_xml_into(&mut xml, &entry.title);
        xml.push_str("</title><link>");
        escape_xml_into(&mut xml, &absolute(&entry.address));
        xml.push_str("</link><guid>");
        escape_xml_into(&mut xml, &absolute(&entry.address));
        xml.push_str("</guid>");
        if let Some(date) = &entry.date {
            xml.push_str("<pubDate>");
            escape_xml_into(&mut xml, date);
            xml.push_str("</pubDate>");
        }
        if let Some(summary) = &entry.summary {
            xml.push_str("<description>");
            escape_xml_into(&mut xml, summary);
            xml.push_str("</description>");
        }
        if let Some(author) = &entry.author {
            xml.push_str("<author>");
            escape_xml_into(&mut xml, author);
            xml.push_str("</author>");
        }
        xml.push_str("</item>\n");
    }
    xml.push_str("</channel></rss>\n");
    xml
}

fn atom_feed(entries: &[FeedEntry], absolute: &impl Fn(&str) -> String) -> String {
    let mut xml = String::from(&format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<feed xmlns=\"http://www.w3.org/2005/Atom\"><title>Raymatic publication</title><id>{}</id><link href=\"{}\"/>\n",
        absolute("/"),
        absolute("/")
    ));
    for entry in entries {
        xml.push_str("<entry><title>");
        escape_xml_into(&mut xml, &entry.title);
        xml.push_str("</title><id>");
        escape_xml_into(&mut xml, &absolute(&entry.address));
        xml.push_str("</id><link href=\"");
        escape_xml_into(&mut xml, &absolute(&entry.address));
        xml.push_str("\"/>");
        if let Some(date) = &entry.date {
            xml.push_str("<updated>");
            escape_xml_into(&mut xml, date);
            xml.push_str("</updated>");
        }
        if let Some(summary) = &entry.summary {
            xml.push_str("<summary>");
            escape_xml_into(&mut xml, summary);
            xml.push_str("</summary>");
        }
        if let Some(author) = &entry.author {
            xml.push_str("<author><name>");
            escape_xml_into(&mut xml, author);
            xml.push_str("</name></author>");
        }
        xml.push_str("</entry>\n");
    }
    xml.push_str("</feed>\n");
    xml
}

fn sitemap(pages: &[(String, PathBuf, Address)], base_url: Option<&str>) -> String {
    let mut xml = String::from(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n",
    );
    for (_, _, address) in pages {
        xml.push_str("  <url><loc>");
        escape_xml_into(&mut xml, &absolute_url(base_url, address.as_path()));
        xml.push_str("</loc></url>\n");
    }
    xml.push_str("</urlset>\n");
    xml
}

fn absolute_url(base_url: Option<&str>, path: &str) -> String {
    base_url.map_or_else(
        || path.to_owned(),
        |base| {
            format!(
                "{}/{}",
                base.trim_end_matches('/'),
                path.trim_start_matches('/')
            )
        },
    )
}

fn escape_xml_into(output: &mut String, value: &str) {
    for character in value.chars() {
        match character {
            '&' => output.push_str("&amp;"),
            '<' => output.push_str("&lt;"),
            '>' => output.push_str("&gt;"),
            '"' => output.push_str("&quot;"),
            '\'' => output.push_str("&apos;"),
            character => output.push(character),
        }
    }
}

pub fn plan_html(pages: Vec<(String, PathBuf, Address)>) -> Result<OutputPlan, AppError> {
    plan(pages, &[])
}

fn output_path(address: &Address) -> PathBuf {
    let path = address.as_path().trim_matches('/');
    if path.is_empty() {
        PathBuf::from("index.html")
    } else {
        PathBuf::from(path).join("index.html")
    }
}

fn validate(plan: &OutputPlan) -> Result<(), AppError> {
    let mut paths = BTreeSet::new();
    for entry in &plan.entries {
        if entry.relative_path.is_absolute()
            || entry
                .relative_path
                .components()
                .any(|part| matches!(part, Component::ParentDir))
            || !paths.insert(entry.relative_path.clone())
        {
            return Err(AppError::InvalidPublication(vec![Diagnostic {
                severity: Severity::Error,
                code: DiagnosticCode("OUTPUT001"),
                summary: "Output path collision".into(),
                explanation: Some(format!(
                    "More than one publication input produces {}.",
                    entry.relative_path.display()
                )),
                primary: None,
                related: vec![],
                object: Some(SemanticObject::Publication),
                expected: Some("a unique output path for every page and asset".into()),
                help: Some(
                    "Rename the source, change its address, or move the asset so each output path is unique."
                        .into(),
                ),
            }]));
        }
    }
    Ok(())
}

pub fn commit(root: &Path, plan: &OutputPlan) -> Result<(), AppError> {
    commit_named(root, "output", plan)
}

pub fn commit_named(root: &Path, name: &str, plan: &OutputPlan) -> Result<(), AppError> {
    let output = root.join(name);
    let staging = root.join(format!("{name}.staging"));
    let backup = root.join(format!("{name}.backup"));
    if staging.exists() || backup.exists() {
        return Err(AppError::Environment(
            EnvironmentError::ExistingCommitState(staging),
        ));
    }
    fs::create_dir(&staging).map_err(|source| environment(&staging, source))?;
    for entry in &plan.entries {
        let target = staging.join(&entry.relative_path);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent).map_err(|source| environment(parent, source))?;
        }
        match &entry.content {
            OutputContent::Bytes(bytes) => {
                fs::write(&target, bytes).map_err(|source| environment(&target, source))?
            }
            OutputContent::CopyFile { source } => {
                fs::copy(source, &target)
                    .map_err(|source_error| environment(&target, source_error))?;
            }
        }
    }
    if output.exists() {
        fs::rename(&output, &backup).map_err(|source| environment(&output, source))?;
    }
    if let Err(source) = fs::rename(&staging, &output) {
        if backup.exists() {
            let _ = fs::rename(&backup, &output);
        }
        return Err(environment(&staging, source));
    }
    if backup.exists() {
        fs::remove_dir_all(&backup).map_err(|source| environment(&backup, source))?;
    }
    Ok(())
}

fn environment(path: &Path, source: std::io::Error) -> AppError {
    AppError::Environment(EnvironmentError::Inspect {
        path: path.into(),
        source,
    })
}
