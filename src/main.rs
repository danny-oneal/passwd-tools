use std::error::Error as _;
use std::process::ExitCode;

use clap::Parser;
use passwd_tools::cli::Cli;

fn main() -> ExitCode {
    match passwd_tools::run(Cli::parse()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) if e.is_broken_pipe() => ExitCode::SUCCESS,
        Err(e) => {
            eprint!("error: {e}");
            let mut source = e.source();
            while let Some(s) = source {
                eprint!(": {s}");
                source = s.source();
            }
            eprintln!();
            ExitCode::FAILURE
        }
    }
}
