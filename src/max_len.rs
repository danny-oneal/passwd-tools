//! The `max-len` command: keep lines with at most N characters.

use std::io::{BufRead, Write};

use crate::error::Result;
use crate::line_filter;

/// Copy lines from `input` to `output` whose length is at most `max_len`
/// characters.
///
/// Lines are copied verbatim, including their terminator. Length is counted in
/// Unicode scalar values, excluding the trailing `\n`; invalid UTF-8 bytes
/// count as one character each. A trailing `\r` counts toward the length.
///
/// # Errors
///
/// Returns an error if reading from `input` or writing to `output` fails.
pub fn filter<R: BufRead, W: Write>(input: R, output: W, max_len: usize) -> Result<()> {
    line_filter::filter(input, output, |len| len <= max_len)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(input: &[u8], max_len: usize) -> Vec<u8> {
        let mut out = Vec::new();
        filter(input, &mut out, max_len).unwrap();
        out
    }

    #[test]
    fn keeps_lines_at_or_below_threshold() {
        assert_eq!(run(b"ab\nabc\nabcd\n", 3), b"ab\nabc\n");
    }

    #[test]
    fn zero_keeps_only_empty_lines() {
        assert_eq!(run(b"\na\n", 0), b"\n");
    }

    #[test]
    fn preserves_missing_final_newline() {
        assert_eq!(run(b"abc\nabcd", 3), b"abc\n");
        assert_eq!(run(b"abcd\nabc", 3), b"abc");
    }

    #[test]
    fn counts_characters_not_bytes() {
        assert_eq!(run("é\n".as_bytes(), 1), "é\n".as_bytes());
        assert_eq!(run("éé\n".as_bytes(), 1), b"");
    }

    #[test]
    fn invalid_bytes_count_as_one_char_each() {
        assert_eq!(run(b"\xff\xfe\n", 2), b"\xff\xfe\n");
        assert_eq!(run(b"\xff\xfe\n", 1), b"");
    }

    #[test]
    fn carriage_return_counts_toward_length() {
        assert_eq!(run(b"ab\r\n", 2), b"");
        assert_eq!(run(b"ab\r\n", 3), b"ab\r\n");
    }
}
