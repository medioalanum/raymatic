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
    let mut report = format!(
        "Raymatic migration inspection\nsource: Pelican\npath: {}\n\n",
        source.display()
    );
    report.push_str("Preserve with transformation:\n");
    for file in &files {
        report.push_str(&format!(
            "- {}: Markdown content; inspect metadata and links before import\n",
            file.strip_prefix(source).unwrap().display()
        ));
    }
    report.push_str("\nReview required:\n- pelicanconf.py and publishconf.py are not executed\n- themes, plugins, templates, static-path selection, and generated output are not imported\n");
    Ok(report)
}

pub fn import(source: &Path, destination: &Path) -> Result<(), AppError> {
    let _ = inspect(source)?;
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
    let files = markdown_files(&source.join("content"))?;
    for file in files {
        let relative = file
            .strip_prefix(source.join("content"))
            .map_err(|_| AppError::Internal("Pelican content escaped its root"))?;
        let target = staging.join("content").join(relative);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent).map_err(io)?;
        }
        fs::write(target, convert(&fs::read_to_string(file).map_err(io)?)).map_err(io)?;
    }
    fs::write(staging.join("presentation/page.html"), "<!doctype html><html><head><meta charset=\"utf-8\"><title>{{ title }}</title></head><body>{{ body }}</body></html>\n").map_err(io)?;
    fs::write(staging.join("presentation/index.html"), "<!doctype html><html><head><meta charset=\"utf-8\"><title>{{ title }}</title></head><body>{{ body }}</body></html>\n").map_err(io)?;
    fs::write(staging.join("MIGRATION_REPORT.md"), inspect(source)?).map_err(io)?;
    if destination.exists() {
        fs::remove_dir(destination).map_err(io)?;
    }
    fs::rename(staging, destination).map_err(io)?;
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
fn convert(source: &str) -> String {
    let mut metadata = Vec::new();
    let mut body = Vec::new();
    let mut headers = true;
    for line in source.lines() {
        if headers {
            if let Some((key, value)) = line.split_once(':') {
                let value = value.trim();
                match key.trim().to_ascii_lowercase().as_str() {
                    "title" | "date" | "category" | "summary" | "author" => {
                        metadata.push(format!("{} = {:?}", key.trim().to_ascii_lowercase(), value))
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
                    _ => {}
                };
                continue;
            }
            headers = false;
        }
        body.push(line);
    }
    format!("+++\n{}\n+++\n{}\n", metadata.join("\n"), body.join("\n"))
}
fn io(error: std::io::Error) -> AppError {
    AppError::Operational(error.to_string())
}
