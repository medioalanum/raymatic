#!/usr/bin/env bash
set -euo pipefail

ray_bin="${RAY_BIN:-target/release/ray}"
if [[ ! -x "$ray_bin" ]]; then
  echo "build Raymatic first: cargo build --release" >&2
  exit 1
fi
ray_bin="$(cd "$(dirname "$ray_bin")" && pwd)/$(basename "$ray_bin")"
root="$(mktemp -d "${TMPDIR:-/tmp}/raymatic-compare.XXXXXX")"
trap 'rm -rf "$root"' EXIT

printf '# Static generator benchmark comparison\n\n'
printf 'This run is observational; tools not installed on the machine are skipped.\n\n'
ray_project="$root/raymatic"
"$ray_bin" new "$ray_project" >/dev/null
start="$(date +%s%N)"
(cd "$ray_project" && "$ray_bin" build >/dev/null)
end="$(date +%s%N)"
printf '%-12s %8d ms\n' raymatic "$(( (end - start) / 1000000 ))"

for tool in hugo zola pelican astro; do
  if command -v "$tool" >/dev/null 2>&1; then
    printf '%-12s %s\n' "$tool" 'available — run an equivalent fixture benchmark before comparing'
  else
    printf '%-12s %s\n' "$tool" 'skipped — executable not installed'
  fi
done
