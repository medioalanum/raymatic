#!/usr/bin/env bash
set -euo pipefail

ray_bin="${RAY_BIN:-target/release/ray}"
if [[ ! -x "$ray_bin" ]]; then
  echo "benchmark binary not found: $ray_bin" >&2
  echo "build it with: cargo build --release" >&2
  exit 1
fi
ray_bin="$(cd "$(dirname "$ray_bin")" && pwd)/$(basename "$ray_bin")"

root="$(mktemp -d "${TMPDIR:-/tmp}/raymatic-benchmark.XXXXXX")"
trap 'rm -rf "$root"' EXIT
project="$root/publication"
"$ray_bin" new "$project" >/dev/null
cd "$project"

measure() {
  local label="$1"
  shift
  local start end elapsed
  start="$(date +%s%N)"
  "$@" >/dev/null
  end="$(date +%s%N)"
  elapsed=$(( (end - start) / 1000000 ))
  printf '%-18s %8d ms\n' "$label" "$elapsed"
}

printf 'Raymatic benchmark\n  binary: %s\n  project: generated starter\n\n' "$ray_bin"
measure cold-build "$ray_bin" build
measure warm-build "$ray_bin" build
printf '\n' >> "$project/content/notes/first-note.md"
measure one-file-build "$ray_bin" build

output_bytes="$(du -sk "$project/output" | awk '{print $1 * 1024}')"
output_files="$(find "$project/output" -type f | wc -l | tr -d ' ')"
printf '\noutput files: %s\noutput bytes: %s\n' "$output_files" "$output_bytes"

cd "$root"
for fixture in medium large; do
  if [[ "$fixture" == medium ]]; then
    count=25
  else
    count=100
  fi
  fixture_project="$root/$fixture"
  "$ray_bin" new "$fixture_project" >/dev/null
  for index in $(seq 1 "$count"); do
    printf '+++\ntitle = "Fixture article %s"\ndate = "2026-10-04"\nsummary = "Benchmark fixture article."\n+++\n\nThis article measures publication throughput.\n' "$index" > "$fixture_project/content/notes/article-$index.md"
  done
  printf '\n[%s fixture: %s articles]\n' "$fixture" "$count"
  (
    cd "$fixture_project"
    measure cold-build "$ray_bin" build
    measure warm-build "$ray_bin" build
    printf '\n' >> "content/notes/article-1.md"
    measure one-file-build "$ray_bin" build
    printf 'output files: %s\noutput bytes: %s\n' "$(find output -type f | wc -l | tr -d ' ')" "$(du -sk output | awk '{print $1 * 1024}')"
  )
done
