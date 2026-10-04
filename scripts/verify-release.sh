#!/usr/bin/env bash
set -euo pipefail

ray_bin="${RAY_BIN:-target/release/ray}"
if [[ ! -x "$ray_bin" ]]; then
  echo "release binary not found: $ray_bin" >&2
  exit 1
fi
ray_bin="$(cd "$(dirname "$ray_bin")" && pwd)/$(basename "$ray_bin")"
root="$(mktemp -d "${TMPDIR:-/tmp}/raymatic-release.XXXXXX")"
trap 'rm -rf "$root"' EXIT
project="$root/publication"

"$ray_bin" new "$project" >/dev/null
(cd "$project" && "$ray_bin" check >/dev/null && "$ray_bin" build >/dev/null)
first_hash="$(find "$project/output" -type f -print0 | sort -z | xargs -0 shasum -a 256 | shasum -a 256 | awk '{print $1}')"
(cd "$project" && "$ray_bin" build >/dev/null)
second_hash="$(find "$project/output" -type f -print0 | sort -z | xargs -0 shasum -a 256 | shasum -a 256 | awk '{print $1}')"
if [[ "$first_hash" != "$second_hash" ]]; then
  echo "deterministic output check failed" >&2
  exit 1
fi

for required in index.html sitemap.xml robots.txt feed.xml atom.xml archive/index.html assets-manifest.json; do
  if [[ ! -f "$project/output/$required" ]]; then
    echo "missing release output: $required" >&2
    exit 1
  fi
done
printf 'release smoke test passed\noutput hash: %s\n' "$first_hash"
