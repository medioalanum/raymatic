#!/usr/bin/env bash
set -euo pipefail

version="${RAY_VERSION:-0.2.0}"
target="${1:-}"
if [[ -z "$target" ]]; then
  echo "usage: RAY_VERSION=0.2.0 $0 <target-triple>" >&2
  exit 2
fi
binary="target/$target/release/ray"
if [[ ! -x "$binary" ]]; then
  echo "release binary not found: $binary" >&2
  exit 1
fi
dist="dist"
mkdir -p "$dist"
archive="$dist/raymatic-v${version}-${target}.tar.gz"
tmp="$(mktemp -d "${TMPDIR:-/tmp}/raymatic-package.XXXXXX")"
trap 'rm -rf "$tmp"' EXIT
cp "$binary" "$tmp/ray"
cp README.md LICENSE "$tmp/"
tar -C "$tmp" -czf "$archive" .
shasum -a 256 "$archive" > "$archive.sha256"
printf 'created %s\nchecksum %s\n' "$archive" "$archive.sha256"
