# passwd-tools verification map

Maintained source for verifying the user-facing behavior of `passwd-tools`. Read this index, then use the matching feature file as the recipe.

## Baseline preconditions

- Repo builds: `cargo build` succeeds and `scripts/drive.sh doctor` prints `OK`.
- `export VERIFY_RUN_ID=$(date +%Y%m%d-%H%M%S)` and create scratch inputs under `target/verify/$VERIFY_RUN_ID/in/`.
- Never rely on files outside the repo, except the deliberately missing `/nonexistent/file`.

## Driving conventions

- Start every recipe from the baseline; the CLI keeps no state between runs.
- Treat every command as literal. Flags and arguments stay unchanged.
- Build inputs with `printf` so bytes are exact.
- Run through `scripts/drive.sh run`; compare `stdout` with `cmp`.

## Proof and skip reporting

- Capture command, stdout, stderr and exit code for every step.
- Mutation-free: the CLI never writes files, so no second view is needed.
- Report an unreachable path with the attempted command and the unmet precondition.

## Feature entry contract

Each feature file starts with an H1 and one paragraph of user-visible behavior, then exactly four H2s in order: `Sub-features`, `How to get to it (user POV)`, `Driving it with drive.sh`, `Gotchas`.

## Features

- [Minimum length filter](./min-len.md) covers `min-len N [FILE]`: threshold, characters vs bytes, newline handling, stdin, errors.
- [Maximum length filter](./max-len.md) covers `max-len N [FILE]`: the same behaviors with an upper bound.
