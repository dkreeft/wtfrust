#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."

command -v rustc >/dev/null || { echo "rustc is required" >&2; exit 1; }

workdir="$(mktemp -d)"
trap 'rm -rf "$workdir"' EXIT

for source in examples/*.rs examples/compile_fail/*.rs; do
  name="$(basename "$source")"
  if [[ ! "$name" =~ ^[a-z][a-z0-9_]*\.rs$ ]]; then
    echo "FAIL: Rust filename must be snake_case: $source" >&2
    exit 1
  fi
done

for source in examples/*.rs; do
  name="$(basename "${source%.rs}")"
  rustc --edition=2024 "$source" -o "$workdir/$name"
  "$workdir/$name" > "$workdir/$name.actual"
  diff -u "expected/$name.txt" "$workdir/$name.actual"
  echo "PASS output: $name"
done

for source in examples/compile_fail/*.rs; do
  name="$(basename "${source%.rs}")"
  if rustc --edition=2024 "$source" -o "$workdir/$name" 2>"$workdir/$name.stderr"; then
    echo "FAIL: expected compilation to fail: $source" >&2
    exit 1
  fi
  echo "PASS compile-fail: $name"
done
