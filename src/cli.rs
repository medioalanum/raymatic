//! Command syntax only; publication semantics belong to the shared pipeline.
use std::path::PathBuf;

use clap::{Arg, ArgAction, Command as ClapCommand, value_parser};

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    New {
        path: PathBuf,
    },
    Dev,
    Check,
    Build,
    MigrateInspect {
        path: PathBuf,
    },
    MigrateImport {
        path: PathBuf,
        destination: PathBuf,
        generate_alt_text: bool,
    },
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
        .subcommand(
            ClapCommand::new("migrate")
                .about("Inspect or import a supported source publication")
                .subcommand_required(true)
                .subcommand(
                    ClapCommand::new("inspect")
                        .about("Inspect without writing")
                        .arg(
                            Arg::new("path")
                                .required(true)
                                .value_parser(value_parser!(PathBuf)),
                        ),
                )
                .subcommand(
                    ClapCommand::new("import")
                        .about("Import into a new directory")
                        .arg(
                            Arg::new("generate-alt-text")
                                .long("generate-alt-text")
                                .action(ArgAction::SetTrue)
                                .help(
                                    "Fill empty image alt text with a deterministic filename label",
                                ),
                        )
                        .arg(
                            Arg::new("path")
                                .required(true)
                                .value_parser(value_parser!(PathBuf)),
                        )
                        .arg(
                            Arg::new("destination")
                                .required(true)
                                .value_parser(value_parser!(PathBuf)),
                        ),
                ),
        )
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
        Some(("migrate", args)) => match args.subcommand() {
            Some(("inspect", values)) => Command::MigrateInspect {
                path: values
                    .get_one::<PathBuf>("path")
                    .expect("required by clap")
                    .clone(),
            },
            Some(("import", values)) => Command::MigrateImport {
                path: values
                    .get_one::<PathBuf>("path")
                    .expect("required by clap")
                    .clone(),
                destination: values
                    .get_one::<PathBuf>("destination")
                    .expect("required by clap")
                    .clone(),
                generate_alt_text: values.get_flag("generate-alt-text"),
            },
            _ => unreachable!("clap requires a declared subcommand"),
        },
        _ => unreachable!("clap requires a declared subcommand"),
    }
}
