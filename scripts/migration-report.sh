#!/usr/bin/env bash
set -euo pipefail

source_root="${1:-.}"
if [[ ! -d "$source_root" ]]; then
  echo "migration source is not a directory: $source_root" >&2
  exit 1
fi

printf '# Raymatic migration report\n\nSource: `%s`\n\n' "$source_root"
printf 'The report is advisory. Review every suggestion and migrate into a clean `ray new` project.\n\n'
printf '## Detected conventions\n\n'
if find "$source_root" -type f -name '*.md' -print -quit | grep -q .; then
  printf '%s\n' '- Markdown content detected: move publication files into `content/`.'
fi
if find "$source_root" -type f \( -name '*.html' -o -name '*.jinja' -o -name '*.njk' \) -print -quit | grep -q .; then
  printf '%s\n' '- HTML/template files detected: consolidate presentation intent in `presentation/`.'
fi
for directory in static public assets images; do
  if [[ -d "$source_root/$directory" ]]; then
    printf '%s\n' "- ${directory}/ detected: review and copy public files into assets/."
  fi
done
if find "$source_root" -type f -path '*/_posts/*' -print -quit | grep -q .; then
  printf '%s\n' '- Jekyll-style `_posts/` detected: move posts into `content/` and translate date/title front matter.'
fi
if [[ -f "$source_root/pelicanconf.py" || -f "$source_root/publishconf.py" ]]; then
  printf '%s\n' '- Pelican configuration detected: migrate editorial intent, not plugin configuration.'
fi
if [[ -f "$source_root/config.toml" || -d "$source_root/layouts" ]]; then
  printf '%s\n' '- Hugo conventions detected: map layouts to presentations and use explicit addresses only for deviations.'
fi
if [[ -f "$source_root/astro.config.mjs" || -d "$source_root/src/content" ]]; then
  printf '%s\n' '- Astro conventions detected: move Markdown into `content/` and keep client-side islands out of the common path.'
fi

printf '\n## Front matter checklist\n\n'
printf '%s\n' '- Convert `---` delimiters to `+++`.' '- Map title, date, summary, category, tags, author, and draft to native Raymatic fields.' '- Run `ray check` after each batch.' '- Compare addresses, feeds, sitemap, and copied assets before deployment.'

printf '\n## Suggested transformations\n\n'
while IFS= read -r file; do
  if head -n 1 "$file" | grep -qx -- '---'; then
    printf '%s\n' "- ${file#\"$source_root/\"}: replace YAML delimiters with \`+++\` and map Title/Date/Category/Tags/Author/Draft to Raymatic fields."
  fi
done < <(find "$source_root" -type f \( -name '*.md' -o -name '*.markdown' \) -print)

printf '\n## URL and redirect review\n\n'
printf '%s\n' '- Derived Raymatic addresses follow the content path and end with `/`.' '- For Jekyll `_posts/` files, record each old permalink and its new `/slug/` address in deployment redirects.' '- For Pelican, compare generated `.html` paths with Raymatic addresses before switching DNS or hosting.'
if find "$source_root" -type f \( -name '*.md' -o -name '*.markdown' \) -print0 | xargs -0 grep -lE '(^|[[:space:]])(permalink|url|slug)[[:space:]]*:' >/dev/null 2>&1; then
  printf '%s\n' '- Permalink/url/slug metadata detected: preserve these values explicitly as Raymatic `address` fields and review redirects.'
fi
