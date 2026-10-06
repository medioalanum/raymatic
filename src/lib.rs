#![forbid(unsafe_code)]
//! Internal application entry points. No stable public Rust API is promised.

pub mod cli;
pub mod content;
pub mod dev;
pub mod diagnostic;
pub mod migration;
pub mod output;
pub mod pipeline;
pub mod project;
pub mod render;
pub mod validate;

use std::path::Path;

use cli::Command;
use diagnostic::Diagnostic;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("{0}")]
    Environment(#[from] project::EnvironmentError),
    #[error("Publication is invalid")]
    InvalidPublication(Vec<Diagnostic>),
    #[error("Internal error: {0}")]
    Internal(&'static str),
    #[error("{0}")]
    Operational(String),
}

impl AppError {
    pub fn render(&self) -> String {
        match self {
            Self::InvalidPublication(diagnostics) => diagnostics
                .iter()
                .map(diagnostic::render)
                .collect::<Vec<_>>()
                .join(""),
            Self::Environment(_) | Self::Internal(_) | Self::Operational(_) => {
                format!("error: {self}\n")
            }
        }
    }
}

pub fn run(command: Command, cwd: &Path) -> Result<(), AppError> {
    match command {
        Command::New { path } => project::create(&cwd.join(path)),
        Command::Check => {
            pipeline::evaluate(cwd)?;
            Ok(())
        }
        Command::Build => {
            let success = pipeline::evaluate(cwd)?;
            output::commit(cwd, &success.output)
        }
        Command::Dev => dev::run(cwd),
        Command::MigrateInspect { path } => {
            print!("{}", migration::inspect(&cwd.join(path))?);
            Ok(())
        }
        Command::MigrateImport {
            path,
            destination,
            generate_alt_text,
        } => migration::import(&cwd.join(path), &cwd.join(destination), generate_alt_text),
    }
}
