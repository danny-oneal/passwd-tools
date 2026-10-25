//! The `prepend` command: insert a prefix at the start of every line.

use std::io::{BufRead, Write};

use crate::error::Result;

/// Copy each line of `input` to `output` with `prefix` written before its
/// content.
///
/// Every line receives the prefix, including empty lines. Line content and its
/// terminator are copied verbatim; the prefix is written as raw bytes before
/// the content and is never counted, trimmed, or decoded. A trailing line with
/// no terminator still receives the prefix.
///
/// # Errors
///
/// Returns an error if reading from `input` or writing to `output` fails.
pub fn filter<R: BufRead, W: Write>(mut input: R, mut output: W, prefix: &[u8]) -> Result<()> {
    let mut buf = Vec::new();
    loop {
        buf.clear();
        if input.read_until(b'\n', &mut buf)? == 0 {
            break;
        }
        output.write_all(prefix)?;
        output.write_all(&buf)?;
    }
    output.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(input: &[u8], prefix: &[u8]) -> Vec<u8> {
        let mut out = Vec::new();
        filter(input, &mut out, prefix).unwrap();
        out
    }

    #[test]
    fn prepends_prefix_to_each_line() {
        assert_eq!(run(b"a\nb\n", b">>"), b">>a\n>>b\n");
    }

    #[test]
    fn empty_prefix_copies_verbatim() {
        assert_eq!(run(b"a\nb\n", b""), b"a\nb\n");
    }

    #[test]
    fn prepends_to_empty_lines() {
        assert_eq!(run(b"\na\n", b"x"), b"x\nxa\n");
    }

    #[test]
    fn preserves_missing_final_newline() {
        assert_eq!(run(b"a\nb", b">"), b">a\n>b");
    }

    #[test]
    fn empty_input_yields_empty_output() {
        assert_eq!(run(b"", b">"), b"");
    }

    #[test]
    fn copies_content_bytes_verbatim_including_cr_and_invalid_utf8() {
        assert_eq!(run(b"ab\r\n\xff\n", b"p"), b"pab\r\np\xff\n");
    }

    #[test]
    fn prefix_bytes_written_as_is() {
        assert_eq!(run(b"a\n", "é".as_bytes()), "éa\n".as_bytes());
    }
}
