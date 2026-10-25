# Maximum length filter

`passwd-tools max-len N [FILE]` prints only the lines that have at most N characters, copied byte-for-byte including their terminator.

## Sub-features

- `max-threshold` keeps lines with length <= N and drops longer ones.
- `max-zero` with N = 0 keeps only empty lines.
- `max-final-newline` preserves a missing newline on the last line when it is kept.
- `max-chars` counts characters, not bytes.
- `max-invalid-utf8` counts each invalid byte as one character.
- `max-cr` counts a trailing `\r` toward the length.
- `max-stdin` reads stdin when FILE is omitted or `-`.
- `max-missing-file` reports the path and fails when FILE does not exist.
- `max-bad-number` rejects a non-numeric N with exit code 2.

## How to get to it (user POV)

- `passwd-tools max-len 8 words.txt`
- `cat words.txt | passwd-tools max-len 8`
- `cat words.txt | passwd-tools max-len 8 -`

## Driving it with drive.sh

Preconditions: same as [min-len.md](./min-len.md): `drive.sh doctor` prints `OK`, `D`, `IN` and `VERIFY_RUN_ID` set, `$IN` exists.

- **max-threshold.** `printf 'ab\nabc\nabcd\n' >$IN/a; $D run max-threshold -- max-len 3 $IN/a`. Exit `0`; stdout is exactly `ab\nabc\n`.
- **max-zero.** `printf '\na\n' >$IN/b; $D run max-zero -- max-len 0 $IN/b`. Exit `0`; stdout is exactly `\n`.
- **max-final-newline.** `printf 'abcd\nabc' >$IN/c; $D run max-final-newline -- max-len 3 $IN/c`. Exit `0`; stdout is `abc` with no trailing newline (`xxd` ends in `63`).
- **max-chars.** `printf 'é\n' >$IN/d; $D run max-chars-1 -- max-len 1 $IN/d` then `printf 'éé\n' >$IN/e; $D run max-chars-2 -- max-len 1 $IN/e`. First stdout is `é\n`; second is empty.
- **max-invalid-utf8.** `printf '\xff\xfe\n' >$IN/f; $D run max-bad-2 -- max-len 2 $IN/f; $D run max-bad-1 -- max-len 1 $IN/f`. First stdout is `ff fe 0a`; second is empty.
- **max-cr.** `printf 'ab\r\n' >$IN/g; $D run max-cr-2 -- max-len 2 $IN/g; $D run max-cr-3 -- max-len 3 $IN/g`. First stdout is empty; second is `61 62 0d 0a`.
- **max-stdin.** `printf 'a\nabc\n' >$IN/h; $D run max-stdin-default --stdin $IN/h -- max-len 2; $D run max-stdin-dash --stdin $IN/h -- max-len 2 -`. Both exit `0` with stdout `a\n`.
- **max-missing-file.** `$D run max-missing -- max-len 1 /nonexistent/file`. Exit `1`; stderr contains `error: cannot open /nonexistent/file`; stdout empty.
- **max-bad-number.** `$D run max-bad-number -- max-len x`. Exit `2`; stdout empty; stderr is clap usage text.
- **Proof.** Keep the whole `target/verify/$VERIFY_RUN_ID/` tree.

## Gotchas

- `max-len` and `min-len` now share one engine (`char_len` and the line loop in `src/line_filter.rs`); each file (`src/max_len.rs`, `src/min_len.rs`) holds only its own threshold direction. A bug in the shared engine surfaces in both commands, so run both recipes to exercise it from each side.
- Same shell-quoting traps as `min-len`: use `printf` and `xxd`.
