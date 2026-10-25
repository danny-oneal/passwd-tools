---
name: verify-passwd-tools
description: Drive the passwd-tools CLI (min-len and max-len line filters, plus prepend and append line transforms, for wordlists) the way a user does and capture proof. Use after any change to src/ or when asked to prove CLI behavior works.
---

# Verify passwd-tools

`passwd-tools` is a non-interactive CLI: stdin or a file in, filtered lines on stdout, errors on stderr, exit code. There is no server and no state, so verification is run-and-compare. Feature recipes are in [features/README.md](features/README.md); read it first, then the matching feature file.

## Launch

```sh
cargo build            # from the repo root; builds target/debug/passwd-tools
```

Ready means `scripts/drive.sh doctor` prints `OK`. Nothing to tear down: every drive is one short-lived process.

## Doctor

```sh
.claude/skills/verify-passwd-tools/scripts/drive.sh doctor
```

Read-only. Fails if the binary is missing, its version differs from `Cargo.toml`, or it is older than `src/lib.rs`, `src/min_len.rs`, `src/max_len.rs`, `src/line_filter.rs`, `src/prepend.rs` or `src/append.rs`. Run it first whenever results look wrong; rebuild on `FAIL`.

## Drive

```sh
export VERIFY_RUN_ID=$(date +%Y%m%d-%H%M%S)      # one id per proof run
D=.claude/skills/verify-passwd-tools/scripts/drive.sh
$D run <label> [--stdin <file>] -- <args...>
```

`run` executes the debug binary with the given args (stdin from `--stdin`, else empty), prints exit code and byte counts, and records `cmd`, `stdin`, `stdout`, `stderr`, `exit` under the evidence dir. Build inputs with `printf` into a file under `target/verify/$VERIFY_RUN_ID/` so byte-exact bytes (no trailing newline, `\r`, invalid UTF-8) are possible. Compare `stdout` byte-for-byte (`cmp`, or `xxd` for invalid bytes), not by eye.

## Evidence

Proof of a feature is the recorded `cmd`, `stdout`, `stderr`, and `exit` for each step in its recipe, plus a byte-exact comparison against the expected output. Standards:

- Drive the real binary through its CLI. Do not call `filter` from a test or add test-only flags.
- A run that only shows `exit=0` is not proof; show the filtered bytes.
- Failure paths need the exact stderr text and a non-zero exit.
- Report any entry point you could not reach with the command tried; never report it verified through a different path.

Evidence lives in `target/verify/<run-id>/` (git-ignored via `/target`) and cleanup never touches it.

## Cleanup

Nothing is running, so there is nothing to kill. Remove only your scratch inputs if you want, never `target/verify/`.

## Helpers

- `scripts/drive.sh doctor` and `scripts/drive.sh run <label> [--stdin <file>] -- <args>` as above. Both work from any directory.
