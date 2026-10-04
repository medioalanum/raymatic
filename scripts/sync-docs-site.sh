#!/usr/bin/env bash
set -euo pipefail

project_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
site_root="$project_root/site"
reference_root="$site_root/content/reference"
rm -rf "$reference_root"
mkdir -p "$reference_root"

title_for() {
  basename "$1" .md | tr '_-' '  ' | awk '{ for (i=1; i<=NF; i++) $i=toupper(substr($i,1,1)) substr($i,2) }1'
}

printf '+++\ntitle = "Reference documentation"\n+++\n\n<div class="doc">\n<div class="eyebrow">reference</div>\n<h1>Everything documented in one place.</h1>\n<p>This section is generated from the English Markdown documentation maintained in the Raymatic repository.</p>\n\n<h2>Project and product</h2>\n<ul>\n' > "$reference_root/index.md"

while IFS= read -r file; do
  relative="${file#"$project_root/"}"
  slug="$(printf '%s' "$relative" | tr '/_' '--' | sed 's/\.md$//' | tr '[:upper:]' '[:lower:]')"
  title="$(title_for "$file")"
  destination="$reference_root/$slug.md"
  printf '+++\ntitle = "%s"\n+++\n\n' "$title" > "$destination"
  sed '1{/^# /d;}' "$file" \
    | sed -E 's#\]\((docs/|\.\./|/)([^)]*\.md)\)#](https://github.com/medioalanum/raymatic/blob/main/\2)#g' \
    | perl -pe 's#\]\((?!https?://)([^)]*\.md)\)#](https://github.com/medioalanum/raymatic/blob/main/$1)#g; s#\]\((?!https?://)([^)]*LICENSE[^)]*)\)#](https://github.com/medioalanum/raymatic/blob/main/LICENSE)#g; s#\]\((?:/)?experiments/([^)]*)\)#](https://github.com/medioalanum/raymatic/tree/main/experiments/$1)#g' \
    >> "$destination"
  printf '<li><a href="/raymatic/reference/%s/">%s</a><small> — %s</small></li>\n' "$slug" "$title" "$relative" >> "$reference_root/index.md"
done < <(
  {
    find "$project_root/docs" -maxdepth 1 -type f -name '*.md'
    find "$project_root" -maxdepth 1 -type f -name '*.md'
    find "$site_root" -maxdepth 1 -type f -name '*.md'
  } | sort -u
)

printf '</ul>\n</div>\n' >> "$reference_root/index.md"
