+++
title = "MIGRATION"
+++


Raymatic provides a deliberately bounded Pelican workflow. It never executes Pelican configuration, plugins, templates, themes, or hooks:

```sh
ray migrate inspect path/to/pelican-project
ray migrate import path/to/pelican-project path/to/new-raymatic-project
```

For a source with empty image alternative text, use `--generate-alt-text` only when a deterministic filename label is acceptable:

```sh
ray migrate import --generate-alt-text path/to/pelican-project path/to/new-raymatic-project
```

`inspect` is non-destructive. `import` requires a new or empty destination, validates the generated project before publishing it, and leaves the source untouched. It supports conventional Markdown under `content/` with Pelican header metadata or YAML front matter; `content/pages/` becomes native pages and other Markdown becomes articles. Portable non-Markdown files become `assets/`, excluding the source `theme/` tree.

The importer normalizes `{static}` and `{filename}` links plus local Markdown and `.html` references. Inspection reports plugins, themes, URL rules, static-path configuration, and empty image alternative text. `--generate-alt-text` fills only empty descriptions with `Image: filename`; strict mode remains the default. Themes, plugins, generated output, arbitrary configuration, non-slash legacy URLs, and visual parity remain review items.

Raymatic migrates publication intent, not the incidental machinery of another generator. For manual work, start with a clean project:

```sh
ray new my-publication
cd my-publication
ray dev
```

Then move content in small batches and run `ray check` after each batch.

For a non-destructive first pass over an existing project, run:

```sh
./scripts/migration-report.sh path/to/existing-project > migration-report.md
```

The report detects common directory and configuration conventions and produces suggestions. It does not rewrite files or claim that a migration is complete.

## Concept mapping

| Existing system | Raymatic equivalent |
| --- | --- |
| Markdown content | `content/**/*.md` |
| Front matter | TOML between `+++` delimiters |
| Theme/layout | `presentation/*.html` |
| Static directory | `assets/` |
| Generated site | `output/` after `ray build` |
| Draft/unpublished flag | `draft = true` |
| Permalink | `address = "/path/"` |
| Tags and categories | native `tags` and `category` fields |

## Pelican

Move article and page Markdown into `content/`, translate metadata to TOML, and move static files into `assets/`. Replace Jinja theme inheritance with the shared Raymatic presentation or an explicit presentation selection. Preserve editorial intent; do not carry over Pelican plugin configuration unless the behavior is still needed after Raymatic derives its output.

## Hugo

Move page bundles and Markdown into `content/`. Translate YAML/TOML front matter to the Raymatic fields and use `address` only for intentional permalink deviations. Recreate shortcodes as ordinary Markdown or explicit presentation logic before introducing custom machinery.

## Jekyll

Move posts and pages into `content/`, convert YAML front matter to TOML, and place static files in `assets/`. Replace collection configuration with directories and native category/tag metadata where possible.

## Astro static projects

Move content collections into Markdown files and move public assets into `assets/`. Keep interactive islands out of the published common path; use them only where the publication genuinely requires client-side behavior. Rebuild layout intent in `presentation/` and verify the generated HTML with `ray check`.

## Migration checklist

1. Create and build a clean Raymatic starter.
2. Move one representative page and preserve its title, date, summary, address, and taxonomy.
3. Move referenced assets and fix root-relative links.
4. Run `ray check` and resolve diagnostics before moving more content.
5. Compare canonical addresses, feeds, sitemap, and output links.
6. Run `ray build` and deploy only `output/`.
