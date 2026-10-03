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

pub struct DiscoveredAsset {
    pub path: PathBuf,
    pub relative_path: PathBuf,
}

pub struct DiscoveredSources {
    pub content: Vec<DiscoveredContent>,
    pub presentation: PathBuf,
    pub assets: Vec<DiscoveredAsset>,
}

pub fn discover(root: &Path) -> Result<DiscoveredSources, EnvironmentError> {
    inspect_root(root)?;
    let content_root = root.join("content");
    let mut content = Vec::new();
    discover_markdown(&content_root, &content_root, &mut content)?;
    content.sort_by(|left, right| left.relative_path.cmp(&right.relative_path));

    let assets_root = root.join("assets");
    let mut assets = Vec::new();
    if assets_root.exists() {
        discover_assets(&assets_root, &assets_root, &mut assets)?;
        assets.sort_by(|left, right| left.relative_path.cmp(&right.relative_path));
    }

    Ok(DiscoveredSources {
        content,
        presentation: root.join("presentation/page.html"),
        assets,
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

fn discover_assets(
    root: &Path,
    directory: &Path,
    output: &mut Vec<DiscoveredAsset>,
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
            discover_assets(root, &path, output)?;
        } else {
            output.push(DiscoveredAsset {
                relative_path: path
                    .strip_prefix(root)
                    .expect("discovered under assets root")
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
            "<!doctype html>\n<html lang=\"en\">\n  <head>\n    <meta charset=\"utf-8\">\n    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n    <title>{{ title }} · Raymatic</title>\n    <style>\n      :root{color-scheme:light;font-family:system-ui,-apple-system,sans-serif;color:#202124;background:#faf9f7}\n      body{max-width:48rem;margin:0 auto;padding:4rem 1.5rem;line-height:1.7}\n      main{background:#fff;padding:clamp(2rem,7vw,5rem);border:1px solid #e8e3dc;border-radius:1rem;box-shadow:0 1rem 3rem #332b2010}\n      h1{font-size:clamp(2rem,7vw,4rem);line-height:1.05;margin:0 0 1rem;letter-spacing:-.04em}\n      p{color:#5d5a55;font-size:1.1rem}\n      code{background:#f1eee9;padding:.15em .35em;border-radius:.3em}\n    </style>\n  </head>\n  <body>\n    <main>{{ body }}</main>\n  </body>\n</html>\n",
        ),
        (
            "presentation/index.html",
            "<!doctype html>\n<html lang=\"en\">\n  <head>\n    <meta charset=\"utf-8\">\n    <title>{{ title }}</title>\n  </head>\n  <body>\n    <main>{{ body }}</main>\n  </body>\n</html>\n",
        ),
        (
            "README.md",
            "# Raymatic publication\n\n- Write pages in `content/`.\n- Change the shared HTML in `presentation/page.html`.\n- Put static files in `assets/`; their paths are preserved in the generated site.\n- Run `ray dev` for a local preview.\n- Run `ray check` before `ray build`.\n- Find the production site in `output/` after `ray build`.\n",
        ),
    ] {
        let path = root.join(relative_path);
        fs::write(&path, content).map_err(|source| EnvironmentError::Inspect { path, source })?;
    }
    Ok(())
}
