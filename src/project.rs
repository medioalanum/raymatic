//! Filesystem entry boundary. Project syntax is bound in the publishing slice.
use crate::AppError;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

#[derive(Debug, thiserror::Error)]
pub enum EnvironmentError {
    #[error("Cannot inspect {path}: {source}")]
    Inspect {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("Expected a directory at {0}")]
    NotDirectory(PathBuf),
    #[error("A previous output commit is incomplete at {0}")]
    ExistingCommitState(PathBuf),
    #[error("Cannot create a publication in non-empty directory {0}")]
    NonEmptyTarget(PathBuf),
}

pub struct DiscoveredContent {
    pub path: PathBuf,
    pub relative_path: PathBuf,
}

pub struct DiscoveredSources {
    pub content: Vec<DiscoveredContent>,
    pub presentation: PathBuf,
}

pub fn discover(root: &Path) -> Result<DiscoveredSources, EnvironmentError> {
    inspect_root(root)?;
    let content_root = root.join("content");
    let mut content = Vec::new();
    discover_markdown(&content_root, &content_root, &mut content)?;
    content.sort_by(|left, right| left.relative_path.cmp(&right.relative_path));
    Ok(DiscoveredSources {
        content,
        presentation: root.join("presentation/page.html"),
    })
}

fn discover_markdown(
    root: &Path,
    directory: &Path,
    output: &mut Vec<DiscoveredContent>,
) -> Result<(), EnvironmentError> {
    for entry in std::fs::read_dir(directory).map_err(|source| EnvironmentError::Inspect {
        path: directory.into(),
        source,
    })? {
        let entry = entry.map_err(|source| EnvironmentError::Inspect {
            path: directory.into(),
            source,
        })?;
        let path = entry.path();
        if entry
            .file_type()
            .map_err(|source| EnvironmentError::Inspect {
                path: path.clone(),
                source,
            })?
            .is_dir()
        {
            discover_markdown(root, &path, output)?;
        } else if path.extension().is_some_and(|extension| extension == "md") {
            output.push(DiscoveredContent {
                relative_path: path
                    .strip_prefix(root)
                    .expect("discovered under content root")
                    .into(),
                path,
            });
        }
    }
    Ok(())
}

pub fn read_text(path: &Path) -> Result<String, EnvironmentError> {
    std::fs::read_to_string(path).map_err(|source| EnvironmentError::Inspect {
        path: path.to_owned(),
        source,
    })
}

pub fn inspect_root(root: &Path) -> Result<(), EnvironmentError> {
    let metadata = std::fs::metadata(root).map_err(|source| EnvironmentError::Inspect {
        path: root.to_owned(),
        source,
    })?;
    if !metadata.is_dir() {
        return Err(EnvironmentError::NotDirectory(root.to_owned()));
    }
    Ok(())
}

pub fn create(target: &Path) -> Result<(), AppError> {
    if target.exists() {
        inspect_root(target)?;
        if fs::read_dir(target)
            .map_err(|source| EnvironmentError::Inspect {
                path: target.to_owned(),
                source,
            })?
            .next()
            .is_some()
        {
            return Err(EnvironmentError::NonEmptyTarget(target.to_owned()).into());
        }
    }

    let staging = target.with_extension("raymatic-new");
    if staging.exists() {
        return Err(EnvironmentError::ExistingCommitState(staging).into());
    }
    fs::create_dir_all(staging.join("content")).map_err(|source| EnvironmentError::Inspect {
        path: staging.clone(),
        source,
    })?;
    fs::create_dir_all(staging.join("presentation")).map_err(|source| {
        EnvironmentError::Inspect {
            path: staging.clone(),
            source,
        }
    })?;
    write_initial_files(&staging)?;

    if target.exists() {
        fs::remove_dir(target).map_err(|source| EnvironmentError::Inspect {
            path: target.to_owned(),
            source,
        })?;
    }
    fs::rename(&staging, target).map_err(|source| EnvironmentError::Inspect {
        path: target.to_owned(),
        source,
    })?;
    Ok(())
}

fn write_initial_files(root: &Path) -> Result<(), EnvironmentError> {
    for (relative_path, content) in [
        (
            "content/index.md",
            "+++\ntitle = \"Welcome to Raymatic\"\n+++\n\n# Welcome to Raymatic\n\nEdit this page, then run `ray dev`.\n",
        ),
        (
            "presentation/page.html",
            "<!doctype html>\n<html lang=\"en\">\n  <head>\n    <meta charset=\"utf-8\">\n    <title>{{ title }}</title>\n  </head>\n  <body>\n    <main>{{ body }}</main>\n  </body>\n</html>\n",
        ),
        (
            "README.md",
            "# Raymatic publication\n\n- Write pages in `content/`.\n- Change the shared HTML in `presentation/page.html`.\n- Run `ray dev` for a local preview.\n- Run `ray check` before `ray build`.\n- Find the production site in `output/` after `ray build`.\n",
        ),
    ] {
        let path = root.join(relative_path);
        fs::write(&path, content).map_err(|source| EnvironmentError::Inspect { path, source })?;
    }
    Ok(())
}
