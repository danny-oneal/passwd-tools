# Agent guide: passwd-tools

Guidance for any AI agent working in this repository. Written to be tool-agnostic;
"you" means whatever agent is reading this.

## What this is

`passwd-tools` is a small Rust CLI of line-oriented text filters and transforms for
wordlists and password files. It is non-interactive: input comes from stdin or a file,
results go to stdout, errors to stderr, and the exit code reports success. There is no
server and no persistent state — every run is one short-lived process.

- Cargo package and binary name: `passwd-tools`.
- Edition 2024, `rust-version` (MSRV) 1.85.
- Repository is public; `main` is the default and protected branch.

## Commands

| Subcommand | Purpose |
|---|---|
| `min-len N [FILE]` | Keep lines with at least N characters. |
| `max-len N [FILE]` | Keep lines with at most N characters. |
| `prepend PREFIX [FILE]` | Insert PREFIX at the start of every line. |
| `append SUFFIX [FILE]` | Insert SUFFIX at the end of every line, before the terminator. |

`FILE` is optional; omitted or `-` means read stdin.

## Source layout

- `src/main.rs` — thin binary. Parses args, calls `passwd_tools::run`, maps errors to
  exit codes. A broken downstream pipe (e.g. `| head`) exits `0`, not failure.
- `src/lib.rs` — `run(Cli)` dispatches each subcommand; `open_input` resolves a path or
  stdin (`-` / omitted = stdin). New modules are declared here.
- `src/cli.rs` — clap `derive` definitions (`Cli`, `Command`). Doc comments on each
  variant/field become the `--help` text, so keep them accurate.
- `src/error.rs` — `Error` (via `thiserror`): `Open { path, source }` and
  `Io(#[from] io::Error)`; `is_broken_pipe()`; `Result<T>` alias.
- `src/line_filter.rs` — shared engine behind the **length filters** only. `filter`
  reads line-by-line with `read_until(b'\n')` and copies kept lines verbatim; `char_len`
  counts characters. `min_len` and `max_len` are thin wrappers that differ only in the
  threshold direction.
- `src/prepend.rs`, `src/append.rs` — **transforms**, not filters. They have their own
  read loops and intentionally do **not** use `line_filter`.

## Behavioral conventions (repo-wide — honor these in any new command)

- **Only `\n` terminates a line.** A trailing `\r` is content: it counts toward length,
  and `append` inserts the suffix *after* the `\r` and *before* the `\n`.
- **Byte-for-byte output.** Lines (and their terminators) are copied verbatim. A missing
  final newline on the last line is preserved.
- **Length is counted in characters** (Unicode scalar values), not bytes. Invalid UTF-8
  is handled lossily, with each invalid byte counting as one character.
- **prepend/append work on raw bytes.** The prefix/suffix is never counted, trimmed, or
  decoded. Empty lines are transformed too. An empty prefix/suffix is a valid no-op.
- **Error style:** messages are lowercase with no trailing punctuation. A missing input
  file reports its path and exits `1`. A bad or missing argument exits `2` (from clap).

## Build, test, lint

```sh
cargo build
cargo test                 # unit tests (per-module #[cfg(test)]), tests/ integration, doctests
cargo clippy --all-targets # lints must stay clean
```

- Lints are strict: clippy `all` + `pedantic` at warn, and `unsafe_code = "forbid"`.
  Do not introduce `unsafe`.
- Release profile is tuned for size/speed (`lto = "fat"`, `codegen-units = 1`,
  `panic = "abort"`, `strip = true`); don't rely on unwinding.

## Working style

- **Test-driven.** Write tests before implementation. Each command has module-level unit
  tests; mirror the existing patterns (`prepend`/`append` are good templates for a new
  transform, `min_len`/`max_len` for a new filter).
- **Prove behavior with the real binary, not just unit calls.** Terminals hide `\r` and
  invalid bytes — build inputs with `printf` and compare output byte-for-byte with `xxd`
  or `cmp`.
- When adding a command, update: `src/cli.rs` (variant), `src/lib.rs` (module + dispatch
  arm), and the verification docs (see below).

## Verification tooling

Project-specific verification lives under `.claude/skills/` and drives the real CLI:

- `verify-passwd-tools/` — `scripts/drive.sh doctor` checks the binary is built and
  fresh; `scripts/drive.sh run <label> [--stdin f] -- <args>` runs it and records
  `cmd`/`stdin`/`stdout`/`stderr`/`exit` as evidence under `target/verify/`. Per-feature
  recipes are in `features/*.md`.
  - Each feature doc follows a fixed contract: an H1 and one paragraph, then exactly four
    H2s in order — `Sub-features`, `How to get to it (user POV)`, `Driving it with
    drive.sh`, `Gotchas`.
  - `drive.sh doctor` has a hard-coded staleness check against the source files. **When
    you add a new `src/*.rs` module, add it to that list** or the doctor will not notice
    a stale binary for the new command.
- `formal-verify-line-filters/` — exhaustive correctness check of the length filters.

## Git and GitHub workflow

- **`main` is protected: no direct pushes.** All changes go through a branch and a pull
  request. The protection (a repository ruleset) also forbids force-pushes and branch
  deletion, and applies to everyone including the owner.
- **Merges are squash-only.** Merge commits and rebase merges are disabled; the head
  branch is auto-deleted on merge. Keep PRs focused so the squashed commit is coherent.
- Branch naming seen in use: `feature/*`, `chore/*`, `docs/*`. Commit subjects are short
  and imperative.
- **Dependabot is active:** security alerts, automated security-fix PRs, and weekly
  `cargo` version-update PRs (`.github/dependabot.yml`). Expect dependency PRs to appear
  on their own; review and squash-merge them like any other.

## Environment notes / gotchas

- There is no top-level `README`; user-facing behavior is documented in the
  verification-skill feature docs and in the clap `--help` text (from doc comments).
- `gh repo edit --visibility` is unavailable in older `gh` (seen on 2.46); change
  visibility with `gh api -X PATCH repos/OWNER/REPO -f visibility=public`.
- Branch protection / rulesets require the repo to be public or on a paid plan.
