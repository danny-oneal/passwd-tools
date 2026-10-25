#!/usr/bin/env bash
# Drive passwd-tools like a user and record proof.
#   drive.sh doctor
#   drive.sh run <label> [--stdin <file>] -- <args...>
# Evidence goes to target/verify/$VERIFY_RUN_ID/<label>/{cmd,stdin,stdout,stderr,exit}.
set -u
root="$(cd "$(dirname "$0")/../../../.." && pwd)"
bin="$root/target/debug/passwd-tools"
run_id="${VERIFY_RUN_ID:-$(date +%Y%m%d-%H%M%S)}"
ev="$root/target/verify/$run_id"

case "${1:-}" in
doctor)
  want="$(sed -n 's/^version = "\(.*\)"/\1/p' "$root/Cargo.toml" | head -1)"
  [ -x "$bin" ] || { echo "FAIL: $bin missing; run 'cargo build' in $root"; exit 1; }
  got="$("$bin" --version | awk '{print $2}')"
  [ "$got" = "$want" ] || { echo "FAIL: binary is $got, Cargo.toml says $want; rebuild"; exit 1; }
  [ "$bin" -nt "$root/src/lib.rs" ] && [ "$bin" -nt "$root/src/min_len.rs" ] && [ "$bin" -nt "$root/src/max_len.rs" ] \
    || { echo "FAIL: binary older than sources; run 'cargo build'"; exit 1; }
  echo "OK: $bin v$got, evidence dir $ev"
  ;;
run)
  label="$2"; shift 2
  stdin=/dev/null
  if [ "${1:-}" = "--stdin" ]; then stdin="$2"; shift 2; fi
  [ "${1:-}" = "--" ] && shift
  out="$ev/$label"; mkdir -p "$out"
  printf '%q ' passwd-tools "$@" >"$out/cmd"; echo >>"$out/cmd"
  [ "$stdin" = /dev/null ] || cp "$stdin" "$out/stdin"
  "$bin" "$@" <"$stdin" >"$out/stdout" 2>"$out/stderr"
  echo $? >"$out/exit"
  echo "[$label] exit=$(cat "$out/exit") stdout=$(wc -c <"$out/stdout")B stderr=$(wc -c <"$out/stderr")B"
  ;;
*) echo "usage: drive.sh doctor | run <label> [--stdin f] -- <args>"; exit 2 ;;
esac
