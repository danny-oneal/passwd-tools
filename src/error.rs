//! Error type shared by all commands.

use std::io;
use std::path::PathBuf;

/// Failure modes of the CLI.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An input file could not be opened.
    #[error("cannot open {}", path.display())]
    Open {
        /// The path that failed to open.
        path: PathBuf,
        /// The underlying I/O error.
        #[source]
        source: io::Error,
    },
    /// Reading input or writing output failed.
    #[error("i/o error")]
    Io(#[from] io::Error),
}

impl Error {
    /// Whether this error is a closed downstream pipe (e.g. `| head`).
    #[must_use]
    pub fn is_broken_pipe(&self) -> bool {
        matches!(self, Self::Io(e) if e.kind() == io::ErrorKind::BrokenPipe)
    }
}

/// Convenience alias for results using [`Error`].
pub type Result<T> = std::result::Result<T, Error>;
