//! Command-line definition.

use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// Line-oriented text filters for wordlists and password files.
#[derive(Debug, Parser)]
#[command(version, about, arg_required_else_help = true)]
pub struct Cli {
    /// Command to run.
    #[command(subcommand)]
    pub command: Command,
}

/// Available commands.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Print lines that are at least `MIN_LEN` characters long.
    MinLen {
        /// Minimum line length, in characters.
        min_len: usize,
        /// Input file; reads standard input if omitted or `-`.
        file: Option<PathBuf>,
    },
    /// Print lines that are at most `MAX_LEN` characters long.
    MaxLen {
        /// Maximum line length, in characters.
        max_len: usize,
        /// Input file; reads standard input if omitted or `-`.
        file: Option<PathBuf>,
    },
    /// Print each line with `PREFIX` inserted at its start.
    Prepend {
        /// Text to insert at the start of every line.
        prefix: String,
        /// Input file; reads standard input if omitted or `-`.
        file: Option<PathBuf>,
    },
    /// Print each line with `SUFFIX` inserted at its end.
    Append {
        /// Text to insert at the end of every line.
        suffix: String,
        /// Input file; reads standard input if omitted or `-`.
        file: Option<PathBuf>,
    },
}
