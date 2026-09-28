//! Command syntax only; publication semantics belong to the shared pipeline.
use std::path::PathBuf;

use clap::{Arg, Command as ClapCommand, value_parser};

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    New { path: PathBuf },
    Dev,
    Check,
    Build,
}

pub fn definition() -> ClapCommand {
    ClapCommand::new("ray")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Raymatic — static publishing")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .disable_help_subcommand(true)
        .subcommand(
            ClapCommand::new("new").about("Create a publication").arg(
                Arg::new("path")
                    .value_parser(value_parser!(PathBuf))
                    .default_value("."),
            ),
        )
        .subcommand(ClapCommand::new("dev").about("Preview and watch a publication"))
        .subcommand(ClapCommand::new("check").about("Validate a publication"))
        .subcommand(ClapCommand::new("build").about("Build a publication"))
}

pub fn parse() -> Command {
    let matches = definition().get_matches();
    match matches.subcommand() {
        Some(("new", args)) => Command::New {
            path: args
                .get_one::<PathBuf>("path")
                .expect("clap supplies the default path")
                .clone(),
        },
        Some(("dev", _)) => Command::Dev,
        Some(("check", _)) => Command::Check,
        Some(("build", _)) => Command::Build,
        _ => unreachable!("clap requires a declared subcommand"),
    }
}
