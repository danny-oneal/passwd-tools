# Minimum length filter

`passwd-tools min-len N [FILE]` prints only the lines that have at least N characters, copied byte-for-byte including their terminator.

## Sub-features

- `min-threshold` keeps lines with length >= N and drops shorter ones.
- `min-zero` with N = 0 keeps everything, empty lines included.
- `min-final-newline` preserves a missing newline on the last line.
- `min-chars` counts characters, not bytes.
- `min-invalid-utf8` counts each invalid byte as one character.
- `min-cr` counts a trailing `\r` toward the length.
- `min-stdin` reads stdin when FILE is omitted or `-`.
- `min-missing-file` reports the path and fails when FILE does not exist.
- `min-bad-number` rejects a non-numeric N with exit code 2.

## How to get to it (user POV)

- `passwd-tools min-len 8 words.txt`
- `cat words.txt | passwd-tools min-len 8`
- `cat words.txt | passwd-tools min-len 8 -`

## Driving it with drive.sh

Preconditions: `drive.sh doctor` prints `OK`; `export VERIFY_RUN_ID=...; D=.claude/skills/verify-passwd-tools/scripts/drive.sh; IN=target/verify/$VERIFY_RUN_ID/in; mkdir -p $IN`.

- **min-threshold.** `printf 'a\nabc\nabcd\n' >$IN/a; $D run min-threshold -- min-len 3 $IN/a`. Exit `0`; stdout is exactly `abc\nabcd\n`.
- **min-zero.** `printf '\na\n' >$IN/b; $D run min-zero -- min-len 0 $IN/b`. Exit `0`; stdout is exactly `\na\n`.
- **min-final-newline.** `printf 'abc\nabcd' >$IN/c; $D run min-final-newline -- min-len 3 $IN/c`. Exit `0`; stdout is `abc\nabcd` with no trailing newline (`xxd` ends in `64`).
- **min-chars.** `printf 'é\n' >$IN/d; $D run min-chars-1 -- min-len 2 $IN/d` then `printf 'éé\n' >$IN/e; $D run min-chars-2 -- min-len 2 $IN/e`. First stdout is empty; second is `éé\n`.
- **min-invalid-utf8.** `printf '\xff\xfe\n' >$IN/f; $D run min-bad-2 -- min-len 2 $IN/f; $D run min-bad-3 -- min-len 3 $IN/f`. First stdout is `ff fe 0a`; second is empty.
- **min-cr.** `printf 'ab\r\n' >$IN/g; $D run min-cr -- min-len 3 $IN/g`. stdout is `ab\r\n` (`61 62 0d 0a`).
- **min-stdin.** `printf 'a\nab\n' >$IN/h; $D run min-stdin-default --stdin $IN/h -- min-len 2; $D run min-stdin-dash --stdin $IN/h -- min-len 2 -`. Both exit `0` with stdout `ab\n`.
- **min-missing-file.** `$D run min-missing -- min-len 1 /nonexistent/file`. Exit `1`; stderr contains `error: cannot open /nonexistent/file`; stdout empty.
- **min-bad-number.** `$D run min-bad-number -- min-len x`. Exit `2`; stdout empty; stderr is clap usage text.
- **Proof.** Keep the whole `target/verify/$VERIFY_RUN_ID/` tree; each step's `cmd`, `stdout`, `stderr`, `exit` files are the evidence.

## Gotchas

- `echo` and `printf` differ on `\n` and `\xff` in some shells; use `printf` under bash.
- Comparing output with a terminal view hides `\r` and invalid bytes; use `xxd`.
- A broken downstream pipe (`| head -1`) must exit `0`; that path is covered by the broken-pipe handling in `src/main.rs`, not by this recipe.
