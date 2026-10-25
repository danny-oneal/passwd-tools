# Append transform

`passwd-tools append SUFFIX [FILE]` prints every line with SUFFIX inserted at its end, after the content and before the line terminator. Line content and its terminator are copied byte-for-byte; the suffix is written as raw bytes and is never counted, trimmed, or decoded.

## Sub-features

- `append-each` writes the suffix after the content of every line, before the `\n`.
- `append-empty-suffix` with an empty SUFFIX copies input verbatim.
- `append-empty-lines` suffixes empty lines too.
- `append-final-newline` preserves a missing newline on the last line.
- `append-empty-input` produces no output for empty input.
- `append-verbatim` copies content bytes unchanged; only `\n` terminates, so a trailing `\r` is content and the suffix follows it.
- `append-stdin` reads stdin when FILE is omitted or `-`.
- `append-missing-file` reports the path and fails when FILE does not exist.
- `append-missing-arg` rejects a call with no SUFFIX with exit code 2.

## How to get to it (user POV)

- `passwd-tools append _2024 words.txt`
- `cat words.txt | passwd-tools append _2024`
- `cat words.txt | passwd-tools append _2024 -`

## Driving it with drive.sh

Preconditions: same as [min-len.md](./min-len.md): `drive.sh doctor` prints `OK`, `D`, `IN` and `VERIFY_RUN_ID` set, `$IN` exists.

- **append-each.** `printf 'a\nb\n' >$IN/a; $D run append-each -- append '>>' $IN/a`. Exit `0`; stdout is exactly `a>>\nb>>\n`.
- **append-empty-suffix.** `printf 'a\nb\n' >$IN/b; $D run append-empty-suffix -- append '' $IN/b`. Exit `0`; stdout is exactly `a\nb\n`.
- **append-empty-lines.** `printf '\na\n' >$IN/c; $D run append-empty-lines -- append x $IN/c`. Exit `0`; stdout is exactly `x\nax\n`.
- **append-final-newline.** `printf 'a\nb' >$IN/d; $D run append-final-newline -- append '>' $IN/d`. Exit `0`; stdout is `a>\nb>` with no trailing newline (`xxd` ends in `3e`).
- **append-empty-input.** `printf '' >$IN/e; $D run append-empty-input -- append '>' $IN/e`. Exit `0`; stdout is empty.
- **append-verbatim.** `printf 'ab\r\n\xff\n' >$IN/f; $D run append-verbatim -- append p $IN/f`. Exit `0`; stdout is `61 62 0d 70 0a ff 70 0a` (`ab\rp\n\xffp\n`) — the suffix lands after the `\r` and before the `\n`.
- **append-stdin.** `printf 'a\nb\n' >$IN/g; $D run append-stdin-default --stdin $IN/g -- append '>'; $D run append-stdin-dash --stdin $IN/g -- append '>' -`. Both exit `0` with stdout `a>\nb>\n`.
- **append-missing-file.** `$D run append-missing -- append '>' /nonexistent/file`. Exit `1`; stderr contains `error: cannot open /nonexistent/file`; stdout empty.
- **append-missing-arg.** `$D run append-missing-arg -- append`. Exit `2`; stdout empty; stderr is clap usage text (`required arguments were not provided`).
- **Proof.** Keep the whole `target/verify/$VERIFY_RUN_ID/` tree; each step's `cmd`, `stdout`, `stderr`, `exit` files are the evidence.

## Gotchas

- The suffix goes **before** the terminator, not at the very end of the line's bytes: only `\n` is a terminator, so for a `\r\n` line the order is content, `\r`, suffix, `\n` (verify with `xxd`, never by eye). This placement is the whole point of the `append-verbatim` case.
- `append` is a transform, not a filter: its own loop lives in `src/append.rs` and it does **not** share `src/line_filter.rs`. The suffix is pure bytes, so there are no character-count cases to check.
- A SUFFIX that begins with `-` is parsed as a flag by clap; pass it after `--`. An empty SUFFIX (`''`) is a valid no-op, not a missing argument. Same `printf`/`xxd` quoting traps as `min-len`.
