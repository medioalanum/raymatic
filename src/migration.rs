//! One-way, non-executing adapters for supported source publications.
use crate::AppError;
use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn inspect(source: &Path) -> Result<String, AppError> {
    if !source.is_dir() {
        return Err(AppError::Operational(format!(
            "source is not a directory: {}",
            source.display()
        )));
    }
    if !source.join("pelicanconf.py").is_file() {
        return Err(AppError::Operational(
            "only a Pelican project with pelicanconf.py is currently recognized".into(),
        ));
    }
    let files = markdown_files(&source.join("content"))?;
    let configuration = configuration(source)?;
    let mut report = format!(
        "Raymatic migration inspection\nsource: Pelican\npath: {}\n\n",
        source.display()
    );
    report.push_str("## Content findings\n\n| Source | Classification | Native handling |\n| --- | --- | --- |\n");
    for file in &files {
        let relative = file.strip_prefix(source).expect("discovered under source");
        let kind = if relative.starts_with("content/pages") {
            "page"
        } else {
            "article"
        };
        let text = fs::read_to_string(file).map_err(io)?;
        let missing_alt = empty_image_alt_count(&text);
        let accessibility = if missing_alt == 0 {
            String::new()
        } else {
            format!("; {missing_alt} image(s) need alt text")
        };
        report.push_str(&format!(
            "| `{}` | transformable | Markdown to native {kind}; inspect unsupported links{accessibility} |\n",
            relative.display(),
        ));
    }
    report.push_str("\n## Static configuration findings\n\n");
    for (key, value) in &configuration.values {
        report.push_str(&format!("- {key} = {value:?}: mapped to site.toml\n"));
    }
    if configuration.plugins {
        report.push_str("- plugins: detected; not executed or imported\n");
    }
    if configuration.theme {
        report.push_str("- theme: detected; not executed or imported\n");
    }
    if configuration.url_patterns {
        report.push_str("- URL or SAVE_AS pattern: detected; explicit source addresses and aliases are required for preservation\n");
    }
    if configuration.static_paths {
        report.push_str(
            "- STATIC_PATHS: detected; only portable files discovered under content/ are copied\n",
        );
    }
    report.push_str("\n## Review required\n\n- pelicanconf.py and publishconf.py are not executed\n- themes, plugins, templates, static-path selection, and generated output are not imported\n");
    Ok(report)
}

pub fn import(source: &Path, destination: &Path) -> Result<(), AppError> {
    let _ = inspect(source)?;
    let configuration = configuration(source)?;
    if destination.exists() && fs::read_dir(destination).map_err(io)?.next().is_some() {
        return Err(AppError::Operational(format!(
            "destination is not empty: {}",
            destination.display()
        )));
    }
    let staging = destination.with_extension("raymatic-import");
    if staging.exists() {
        return Err(AppError::Operational(format!(
            "incomplete import staging directory exists: {}",
            staging.display()
        )));
    }
    fs::create_dir_all(staging.join("content")).map_err(io)?;
    fs::create_dir_all(staging.join("presentation")).map_err(io)?;
    fs::create_dir_all(staging.join("assets")).map_err(io)?;
    let files = markdown_files(&source.join("content"))?;
    for file in files {
        let source_relative = file
            .strip_prefix(source.join("content"))
            .map_err(|_| AppError::Internal("Pelican content escaped its root"))?;
        let (relative, kind) = match source_relative.strip_prefix("pages") {
            Ok(page) => (page, "page"),
            Err(_) => (source_relative, "article"),
        };
        let target = staging.join("content").join(relative);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent).map_err(io)?;
        }
        fs::write(
            target,
            convert(&fs::read_to_string(file).map_err(io)?, kind),
        )
        .map_err(io)?;
    }
    copy_portable_assets(
        &source.join("content"),
        &source.join("content"),
        &staging.join("assets"),
    )?;
    fs::write(staging.join("presentation/page.html"), "<!doctype html><html><head><meta charset=\"utf-8\"><title>{{ title }}</title></head><body>{{ body }}</body></html>\n").map_err(io)?;
    fs::write(staging.join("presentation/index.html"), "<!doctype html><html><head><meta charset=\"utf-8\"><title>{{ title }}</title></head><body>{{ body }}</body></html>\n").map_err(io)?;
    fs::write(staging.join("MIGRATION_REPORT.md"), inspect(source)?).map_err(io)?;
    write_site_config(&staging, &configuration)?;
    crate::pipeline::evaluate(&staging).map_err(AppError::from)?;
    if destination.exists() {
        fs::remove_dir(destination).map_err(io)?;
    }
    fs::rename(staging, destination).map_err(io)?;
    Ok(())
}

#[derive(Default)]
struct PelicanConfiguration {
    values: Vec<(String, String)>,
    plugins: bool,
    theme: bool,
    url_patterns: bool,
    static_paths: bool,
}

fn configuration(source: &Path) -> Result<PelicanConfiguration, AppError> {
    let text = fs::read_to_string(source.join("pelicanconf.py")).map_err(io)?;
    let mut out = PelicanConfiguration::default();
    for line in text.lines() {
        let line = line.trim();
        out.plugins |= line.starts_with("PLUGINS") || line.contains("plugins");
        out.theme |= line.starts_with("THEME") || line.contains("theme");
        out.url_patterns |= line.contains("_URL") || line.contains("_SAVE_AS");
        out.static_paths |= line.starts_with("STATIC_PATHS");
        for (pelican, raymatic) in [
            ("SITENAME", "title"),
            ("AUTHOR", "author"),
            ("SITESUBTITLE", "description"),
            ("DEFAULT_LANG", "language"),
            ("SITEURL", "base_url"),
        ] {
            if let Some(value) = static_string(line, pelican) {
                out.values.push((raymatic.into(), value));
            }
        }
    }
    Ok(out)
}

fn static_string(line: &str, key: &str) -> Option<String> {
    let value = line
        .strip_prefix(key)?
        .trim_start()
        .strip_prefix('=')?
        .trim();
    let quote = value.chars().next()?;
    if quote != '\'' && quote != '"' {
        return None;
    }
    value
        .strip_prefix(quote)?
        .split(quote)
        .next()
        .map(str::to_owned)
}

fn write_site_config(root: &Path, configuration: &PelicanConfiguration) -> Result<(), AppError> {
    let text = configuration
        .values
        .iter()
        .map(|(key, value)| format!("{key} = {value:?}\n"))
        .collect::<String>();
    if !text.is_empty() {
        fs::write(root.join("site.toml"), text).map_err(io)?;
    }
    Ok(())
}

fn markdown_files(root: &Path) -> Result<Vec<PathBuf>, AppError> {
    if !root.is_dir() {
        return Ok(vec![]);
    }
    let mut out = vec![];
    walk(root, root, &mut out)?;
    out.sort();
    Ok(out)
}

fn empty_image_alt_count(source: &str) -> usize {
    source.match_indices("![](").count()
}

fn walk(root: &Path, directory: &Path, out: &mut Vec<PathBuf>) -> Result<(), AppError> {
    for entry in fs::read_dir(directory).map_err(io)? {
        let path = entry.map_err(io)?.path();
        if path.is_dir() {
            walk(root, &path, out)?;
        } else if path.extension().is_some_and(|e| e == "md") {
            out.push(path);
        }
    }
    let _ = root;
    Ok(())
}
fn copy_portable_assets(root: &Path, directory: &Path, destination: &Path) -> Result<(), AppError> {
    for entry in fs::read_dir(directory).map_err(io)? {
        let path = entry.map_err(io)?.path();
        let relative = path
            .strip_prefix(root)
            .map_err(|_| AppError::Internal("Pelican asset escaped its root"))?;
        if path.is_dir() {
            if relative
                .components()
                .next()
                .is_some_and(|component| component.as_os_str() == "theme")
            {
                continue;
            }
            copy_portable_assets(root, &path, destination)?;
        } else if !path.extension().is_some_and(|extension| extension == "md") {
            let target = destination.join(relative);
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent).map_err(io)?;
            }
            fs::copy(&path, target).map_err(io)?;
        }
    }
    Ok(())
}

fn convert(source: &str, kind: &str) -> String {
    if let Some((front_matter, body)) = yaml_front_matter(source) {
        return format!(
            "+++\nkind = {:?}\n{}\n+++\n{}\n",
            kind,
            yaml_metadata(front_matter),
            rewrite_markdown_links(body).replace("{static}/", "/assets/")
        );
    }
    let mut metadata = Vec::new();
    metadata.push(format!("kind = {:?}", kind));
    let mut body = Vec::new();
    let mut headers = true;
    for line in source.lines() {
        if headers {
            if let Some((key, value)) = line.split_once(':') {
                let value = value.trim();
                match key.trim().to_ascii_lowercase().as_str() {
                    "title" | "category" | "summary" | "author" => {
                        metadata.push(format!("{} = {:?}", key.trim().to_ascii_lowercase(), value))
                    }
                    "date" => {
                        metadata.push(format!("date = {:?}", value.get(..10).unwrap_or(value)))
                    }
                    "status" if value.eq_ignore_ascii_case("draft") => {
                        metadata.push("draft = true".into())
                    }
                    "tags" => metadata.push(format!(
                        "tags = {:?}",
                        value
                            .split(',')
                            .map(str::trim)
                            .filter(|v| !v.is_empty())
                            .collect::<Vec<_>>()
                    )),
                    "slug" => metadata.push(format!("address = {:?}", format!("/{}/", value))),
                    "alias" | "aliases" => {
                        let aliases = value
                            .split(',')
                            .map(str::trim)
                            .filter(|value| value.starts_with('/') && value.ends_with('/'))
                            .collect::<Vec<_>>();
                        if !aliases.is_empty() {
                            metadata.push(format!("aliases = {aliases:?}"));
                        }
                    }
                    _ => {}
                };
                continue;
            }
            headers = false;
        }
        body.push(line);
    }
    format!(
        "+++\n{}\n+++\n{}\n",
        metadata.join("\n"),
        rewrite_markdown_links(&body.join("\n"))
    )
}

fn yaml_front_matter(source: &str) -> Option<(&str, &str)> {
    let rest = source
        .trim_start_matches('\u{feff}')
        .strip_prefix("---\n")?;
    let close = rest.find("\n---")?;
    Some((&rest[..close], &rest[close + 4..]))
}

fn yaml_metadata(front_matter: &str) -> String {
    let mut values = Vec::new();
    let mut title = None;
    let mut key = "";
    for line in front_matter.lines() {
        let trimmed = line.trim();
        if line.starts_with(char::is_whitespace) {
            if let Some(value) = trimmed.strip_prefix("- ") {
                match key {
                    "tags" => values.push(format!("tags = [{value:?}]")),
                    "authors" if !values.iter().any(|value| value.starts_with("author =")) => {
                        values.push(format!("author = {value:?}"))
                    }
                    _ => {}
                }
            } else if key == "title" && !trimmed.is_empty() {
                title = Some(match title {
                    Some(existing) => format!("{existing} {trimmed}"),
                    None => trimmed.to_owned(),
                });
            }
        } else if let Some((name, value)) = trimmed.split_once(':') {
            key = name.trim();
            let value = value.trim();
            if !value.is_empty() && value != ">-" {
                match key {
                    "date" => values.push(format!("date = {:?}", value.get(..10).unwrap_or(value))),
                    "title" => title = Some(value.to_owned()),
                    "category" | "summary" => values.push(format!("{key} = {value:?}")),
                    _ => {}
                }
            }
        }
    }
    let mut tags = Vec::new();
    values.retain(|value| {
        if value.starts_with("tags = [") {
            tags.push(
                value
                    .trim_start_matches("tags = [")
                    .trim_end_matches(']')
                    .to_owned(),
            );
            false
        } else {
            true
        }
    });
    if !tags.is_empty() {
        values.push(format!("tags = [{}]", tags.join(", ")));
    }
    if let Some(title) = title {
        values.push(format!("title = {title:?}"));
    }
    values.join("\n")
}

fn rewrite_markdown_links(body: &str) -> String {
    body.replace("{filename}", "")
        .replace("](./pages/", "](")
        .replace("](pages/", "](")
        .replace("](/pages/", "](/")
        .replace(".html#", "/#")
        .replace(".html)", "/)")
        .replace(".md#", "/#")
        .replace(".md)", "/)")
}
fn io(error: std::io::Error) -> AppError {
    AppError::Operational(error.to_string())
}

#[cfg(test)]
mod tests {
    use super::empty_image_alt_count;

    #[test]
    fn detects_images_without_alt_text() {
        assert_eq!(empty_image_alt_count("![Diagram](diagram.png)"), 0);
        assert_eq!(empty_image_alt_count("![](one.png)\n![](two.png)"), 2);
    }
}
