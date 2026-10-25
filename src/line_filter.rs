//! Shared engine behind the `min-len` and `max-len` commands.
//!
//! Both commands copy input lines straight through to the output and differ
//! only in which lengths they keep. The line-reading loop and the character
//! count live here so that behavior exists in exactly one place.

use std::io::{BufRead, Write};

use crate::error::Result;

/// Copy each line of `input` to `output` when `keep` accepts its length,
/// measured in characters by `char_len` and excluding the trailing `\n`.
///
/// Lines are copied verbatim, including their terminator.
///
/// # Errors
///
/// Returns an error if reading from `input` or writing to `output` fails.
pub(crate) fn filter<R: BufRead, W: Write>(
    mut input: R,
    mut output: W,
    keep: impl Fn(usize) -> bool,
) -> Result<()> {
    let mut buf = Vec::new();
    loop {
        buf.clear();
        if input.read_until(b'\n', &mut buf)? == 0 {
            break;
        }
        let content = buf.strip_suffix(b"\n").unwrap_or(&buf);
        if keep(char_len(content)) {
            output.write_all(&buf)?;
        }
    }
    output.flush()?;
    Ok(())
}

fn char_len(bytes: &[u8]) -> usize {
    match std::str::from_utf8(bytes) {
        Ok(s) => s.chars().count(),
        Err(_) => String::from_utf8_lossy(bytes).chars().count(),
    }
}
