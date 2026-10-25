//! The `append` command: insert a suffix at the end of every line.

use std::io::{BufRead, Write};

use crate::error::Result;

/// Copy each line of `input` to `output` with `suffix` written after its
/// content, before the line terminator.
///
/// Every line receives the suffix, including empty lines. Line content and its
/// terminator are copied verbatim; the suffix is written as raw bytes between
/// the content and the trailing `\n` and is never counted, trimmed, or decoded.
/// Only `\n` is treated as a terminator, so a trailing `\r` is content and the
/// suffix follows it. A trailing line with no terminator still receives the
/// suffix.
///
/// # Errors
///
/// Returns an error if reading from `input` or writing to `output` fails.
pub fn filter<R: BufRead, W: Write>(mut input: R, mut output: W, suffix: &[u8]) -> Result<()> {
    let mut buf = Vec::new();
    loop {
        buf.clear();
        if input.read_until(b'\n', &mut buf)? == 0 {
            break;
        }
        let (content, term) = match buf.strip_suffix(b"\n") {
            Some(content) => (content, b"\n".as_slice()),
            None => (buf.as_slice(), b"".as_slice()),
        };
        output.write_all(content)?;
        output.write_all(suffix)?;
        output.write_all(term)?;
    }
    output.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(input: &[u8], suffix: &[u8]) -> Vec<u8> {
        let mut out = Vec::new();
        filter(input, &mut out, suffix).unwrap();
        out
    }

    #[test]
    fn appends_suffix_to_each_line() {
        assert_eq!(run(b"a\nb\n", b">>"), b"a>>\nb>>\n");
    }

    #[test]
    fn empty_suffix_copies_verbatim() {
        assert_eq!(run(b"a\nb\n", b""), b"a\nb\n");
    }

    #[test]
    fn appends_to_empty_lines() {
        assert_eq!(run(b"\na\n", b"x"), b"x\nax\n");
    }

    #[test]
    fn preserves_missing_final_newline() {
        assert_eq!(run(b"a\nb", b">"), b"a>\nb>");
    }

    #[test]
    fn empty_input_yields_empty_output() {
        assert_eq!(run(b"", b">"), b"");
    }

    #[test]
    fn suffix_goes_before_newline_and_after_carriage_return() {
        assert_eq!(run(b"ab\r\n\xff\n", b"p"), b"ab\rp\n\xffp\n");
    }

    #[test]
    fn suffix_bytes_written_as_is() {
        assert_eq!(run(b"a\n", "é".as_bytes()), "aé\n".as_bytes());
    }
}
