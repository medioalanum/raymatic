//! Complete output plans are validated before a staged production commit.
use crate::{AppError, content::Address, project::EnvironmentError};
use std::{
    collections::BTreeSet,
    fs,
    path::{Component, Path, PathBuf},
};

#[derive(Debug)]
pub struct OutputPlan {
    pub entries: Vec<OutputEntry>,
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

pub fn plan_html(pages: Vec<(String, PathBuf, Address)>) -> Result<OutputPlan, AppError> {
    let plan = OutputPlan {
        entries: pages
            .into_iter()
            .map(|(content, source, address)| OutputEntry {
                relative_path: output_path(&address),
                content: OutputContent::Bytes(content.into_bytes()),
                origin: OutputOrigin::Content { source, address },
            })
            .collect(),
    };
    validate(&plan)?;
    Ok(plan)
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
            return Err(AppError::Internal("Invalid output plan"));
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
