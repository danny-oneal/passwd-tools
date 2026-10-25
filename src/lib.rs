//! Line-oriented text filters for wordlists and password files.

#![warn(missing_docs)]

pub mod cli;
pub mod error;
pub mod max_len;
pub mod min_len;

use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter};
use std::path::Path;

use cli::{Cli, Command};
use error::{Error, Result};

/// Execute a parsed command line, writing results to standard output.
///
/// # Errors
///
/// Returns an error if the input cannot be opened, or on I/O failure.
pub fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Command::MinLen { min_len, file } => {
            let input = open_input(file.as_deref())?;
            let output = BufWriter::new(io::stdout().lock());
            min_len::filter(input, output, min_len)
        }
        Command::MaxLen { max_len, file } => {
            let input = open_input(file.as_deref())?;
            let output = BufWriter::new(io::stdout().lock());
            max_len::filter(input, output, max_len)
        }
    }
}

fn open_input(path: Option<&Path>) -> Result<Box<dyn BufRead>> {
    match path {
        None => Ok(Box::new(io::stdin().lock())),
        Some(p) if p.as_os_str() == "-" => Ok(Box::new(io::stdin().lock())),
        Some(p) => {
            let file = File::open(p).map_err(|source| Error::Open {
                path: p.to_path_buf(),
                source,
            })?;
            Ok(Box::new(BufReader::new(file)))
        }
    }
}
