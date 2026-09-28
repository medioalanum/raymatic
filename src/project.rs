//! Filesystem entry boundary. Project syntax is bound in the publishing slice.
use crate::AppError;
use std::{
    io,
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

pub fn create(_target: &Path) -> Result<(), AppError> {
    // TODO: write the authoritative minimum fixture once publishing can validate it.
    // Do not create an empty directory and claim it is a valid publication.
    Err(AppError::NotImplemented("Publication creation"))
}
