#![forbid(unsafe_code)]

use std::process::ExitCode;

fn main() -> ExitCode {
    let command = raymatic::cli::parse();
    let cwd = match std::env::current_dir() {
        Ok(path) => path,
        Err(error) => {
            eprintln!("error: Cannot determine the current directory: {error}");
            return ExitCode::FAILURE;
        }
    };
    match raymatic::run(command, &cwd) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprint!("{}", error.render());
            ExitCode::FAILURE
        }
    }
}
