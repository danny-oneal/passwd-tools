# Prepend transform

`passwd-tools prepend PREFIX [FILE]` prints every line with PREFIX inserted at its start. Line content and its terminator are copied byte-for-byte; the prefix is written as raw bytes before the content and is never counted, trimmed, or decoded.

## Sub-features

- `prepend-each` writes the prefix before the content of every line.
- `prepend-empty-prefix` with an empty PREFIX copies input verbatim.
- `prepend-empty-lines` prefixes empty lines too.
- `prepend-final-newline` preserves a missing newline on the last line.
- `prepend-empty-input` produces no output for empty input.
- `prepend-verbatim` copies content bytes unchanged, including a trailing `\r` and invalid UTF-8.
- `prepend-stdin` reads stdin when FILE is omitted or `-`.
- `prepend-missing-file` reports the path and fails when FILE does not exist.
- `prepend-missing-arg` rejects a call with no PREFIX with exit code 2.

## How to get to it (user POV)

- `passwd-tools prepend 2024_ words.txt`
- `cat words.txt | passwd-tools prepend 2024_`
- `cat words.txt | passwd-tools prepend 2024_ -`

## Driving it with drive.sh

Preconditions: same as [min-len.md](./min-len.md): `drive.sh doctor` prints `OK`, `D`, `IN` and `VERIFY_RUN_ID` set, `$IN` exists.

- **prepend-each.** `printf 'a\nb\n' >$IN/a; $D run prepend-each -- prepend '>>' $IN/a`. Exit `0`; stdout is exactly `>>a\n>>b\n`.
- **prepend-empty-prefix.** `printf 'a\nb\n' >$IN/b; $D run prepend-empty-prefix -- prepend '' $IN/b`. Exit `0`; stdout is exactly `a\nb\n`.
- **prepend-empty-lines.** `printf '\na\n' >$IN/c; $D run prepend-empty-lines -- prepend x $IN/c`. Exit `0`; stdout is exactly `x\nxa\n`.
- **prepend-final-newline.** `printf 'a\nb' >$IN/d; $D run prepend-final-newline -- prepend '>' $IN/d`. Exit `0`; stdout is `>a\n>b` with no trailing newline (`xxd` ends in `62`).
- **prepend-empty-input.** `printf '' >$IN/e; $D run prepend-empty-input -- prepend '>' $IN/e`. Exit `0`; stdout is empty.
- **prepend-verbatim.** `printf 'ab\r\n\xff\n' >$IN/f; $D run prepend-verbatim -- prepend p $IN/f`. Exit `0`; stdout is `70 61 62 0d 0a 70 ff 0a` (`pab\r\np\xff\n`).
- **prepend-stdin.** `printf 'a\nb\n' >$IN/g; $D run prepend-stdin-default --stdin $IN/g -- prepend '>'; $D run prepend-stdin-dash --stdin $IN/g -- prepend '>' -`. Both exit `0` with stdout `>a\n>b\n`.
- **prepend-missing-file.** `$D run prepend-missing -- prepend '>' /nonexistent/file`. Exit `1`; stderr contains `error: cannot open /nonexistent/file`; stdout empty.
- **prepend-missing-arg.** `$D run prepend-missing-arg -- prepend`. Exit `2`; stdout empty; stderr is clap usage text (`required arguments were not provided`).
- **Proof.** Keep the whole `target/verify/$VERIFY_RUN_ID/` tree; each step's `cmd`, `stdout`, `stderr`, `exit` files are the evidence.

## Gotchas

- `prepend` is a transform, not a filter: it has its own loop in `src/prepend.rs` and does **not** share `src/line_filter.rs` with `min-len`/`max-len`. The prefix is pure bytes — length and UTF-8 validity never affect it, so there are no character-count cases to check.
- A PREFIX that begins with `-` is parsed as a flag by clap; pass it after `--` (e.g. `prepend -- -x file`). An empty PREFIX (`''`) is a valid no-op prefix, not a missing argument.
- Same shell-quoting traps as `min-len`: use `printf` to build inputs and `xxd` to compare output, since a terminal view hides `\r` and invalid bytes.
