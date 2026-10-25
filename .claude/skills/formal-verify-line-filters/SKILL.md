---
name: formal-verify-line-filters
description: Exhaustively check, over every input up to a stated bound, that passwd-tools' min-len and max-len filters keep exactly the right lines, verbatim and in order. Use after any change to src/min_len.rs, src/max_len.rs, src/line_filter.rs, or char_len, or when asked whether the length filters are correct for all inputs.
---

# Formal check: line-length filters

## What is checked, in plain words

- **Promise.** `min-len N` keeps a line iff it has at least N characters, `max-len N` iff at most N. Kept lines come out byte-for-byte as they went in, in the same order.
- **Harm if broken.** A password list silently loses or keeps the wrong words, or its bytes are changed, and nothing reports it.
- **Why tests miss it.** The unit tests cover about a dozen hand-picked inputs. Bugs live at boundaries (N equal to the length, a missing final newline, `\r`, multi-byte or broken UTF-8).

## Tool and why

No external verifier. Kani (a Rust model checker) was the first choice, but it is an external install and was not available here. Instead `tests/bounded_check.rs` runs the **real** `min_len::filter` and `max_len::filter` on **every** byte string up to 6 bytes over an 8-byte alphabet (`a`, `\r`, `\n`, `c3 a9` = é, `e2 82` = start of a 3-byte character, `ff`), at every threshold N = 0..7. About 300,000 inputs, under 2 seconds, stock `cargo test`.

## Run

```sh
cargo test --test bounded_check -- --include-ignored
```

Expected today: 3 pass, 1 fails. The failure is a known finding (below), not a regression. A regression is any of the **three** passing tests failing, or the fourth failing with a different counterexample.

| Test | Property |
| --- | --- |
| `output_is_a_verbatim_ordered_subset_of_lines` | Output is some of the input's lines, unchanged, in order. |
| `threshold_matches_oracle` | A line is kept iff its length passes the threshold (>= for min, <= for max). |
| `min_and_max_partition_the_input` | For any N, `max-len N` and `min-len N+1` together keep every byte exactly once. |
| `invalid_bytes_count_one_each_as_documented` (ignored) | The doc comments' rule: each invalid byte counts as one character. |

## Known finding (open)

The doc comments on `filter` say invalid UTF-8 bytes count one character each. The code counts each maximal invalid sequence once (`String::from_utf8_lossy`).

Shortest counterexample: the two bytes `e2 a9` (an unfinished 3-byte character) with `min-len 2`. Documented length 2, so the line is kept. Actual length 1, so it is dropped.

Fixing the docs or the code is a product decision; the check is deliberately not weakened to hide this. When it is resolved, remove the `#[ignore]` on the fourth test.

## Scope: what a pass does and does not cover

- Covered: `min_len::filter`, `max_len::filter`, and the shared engine (`line_filter::filter` and `char_len`) in `src/line_filter.rs`, for inputs up to 6 bytes over the alphabet above and N up to 7.
- Not covered: longer inputs, other byte values, `run` and `open_input` in `src/lib.rs`, argument parsing, I/O errors, and the broken-pipe path. A pass says "no bug within this bound", not "no bug".
- The oracle for the threshold test uses the same length definition as the code (lossy decoding). It proves the comparison, line splitting and copying are right, not that the length definition matches the docs; that is the fourth test's job.

## When the check fails

1. Read the counterexample (`input=[...] n=...`). Convert the bytes to a line and run it through `passwd-tools` by hand (see `/verify-passwd-tools`) to confirm.
2. If a passing test now fails, it is a real bug in the change unless the change intentionally altered a documented behavior. In that case update the oracle in `tests/bounded_check.rs` and the doc comments together.
3. A failure in the fourth test with the counterexample above is the open finding, not new.
4. Widen the bound (`MAX_BYTES`, `ALPHABET`) only when you add behavior that needs bytes the alphabet lacks. Keep the run under a few seconds.

## Proof that the check can fail

Each of these edits to the real code was made, run, and reverted; each was caught:

| Edit | Failed tests |
| --- | --- |
| `min_len.rs`: `>=` to `>` | threshold, partition |
| `max_len.rs`: `<=` to `<` | threshold, partition |
| `line_filter.rs`: write the line without its `\n` | all three |
| `line_filter.rs`: strip `\r\n` instead of `\n` | threshold |

The comparison lives in each command's wrapper (`min_len.rs`, `max_len.rs`); the line loop and `char_len` live once in the shared engine (`line_filter.rs`). A length bug in the shared engine is symmetric across both filters, so `partition` (which only catches min and max disagreeing) no longer fires for it; the independent-oracle `threshold` test still does.

## Coverage record and sync

- Covers: `src/min_len.rs`, `src/max_len.rs`, `src/line_filter.rs`.
- Last confirmed against: SHA-256 of `cat src/min_len.rs src/max_len.rs src/line_filter.rs` = `73c2247f02b0ca9d1675272d1bd0f7028e15f823d4c017f3daae8c93575cec7b`. This repo has no commits on the working branch, so a content hash stands in for a commit.
- To check: `cat src/min_len.rs src/max_len.rs src/line_filter.rs | sha256sum`. If it differs, the covered code changed after the last confirmation; re-run the check, re-read the properties against the new code, and only then replace the hash above (or the commit, once the repo has history).
