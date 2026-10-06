# Migration guide

## Current status

Raymatic provides a deliberately bounded Pelican workflow. It never executes Pelican configuration, plugins, templates, themes, or hooks:

```sh
ray migrate inspect path/to/pelican-project
ray migrate import path/to/pelican-project path/to/new-raymatic-project
```

`inspect` is non-destructive. `import` requires a new or empty destination, writes a normal Raymatic project there, validates it with Raymatic before publishing the destination, and leaves the source untouched. The supported subset is conventional Markdown under `content/` with portable Pelican header metadata: title, date, category, tags, summary, author, and slug. Markdown under `content/pages/` becomes native `kind = "page"`; other Markdown becomes `kind = "article"`. Non-Markdown files under `content/` are copied to `assets/`, except the source `theme/` tree.

Themes, plugins, static-path selection, template behavior, redirects, arbitrary configuration, generated output, and links that need rewriting remain review items.

The repository includes an advisory inventory script:

```sh
./scripts/migration-report.sh path/to/existing-project > migration-report.md
```

It scans file names and simple front-matter patterns. It does not parse a source generator's configuration, rewrite files, validate preservation, or establish source-generator support. Review every suggestion before changing a publication.

## Manual migration workflow

Raymatic preserves publication intent where its native model can represent it. It does not preserve the incidental runtime machinery of another generator. Start with a clean project:

```sh
ray new my-publication
cd my-publication
ray dev
```

Then move content in small batches and run `ray check` after each batch.

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

## Source-specific guidance

The sections below are manual translation guidance, not automatic adapter specifications. Any source construct that is not represented in the concept mapping requires a deliberate native design choice or manual review.

### Pelican

Move article and page Markdown into `content/`, translate metadata to TOML, and move static files into `assets/`. Replace Jinja theme inheritance with the shared Raymatic presentation or an explicit presentation selection. Preserve editorial intent; do not carry over Pelican plugin configuration unless the behavior is still needed after Raymatic derives its output.

### Hugo

Move page bundles and Markdown into `content/`. Translate YAML/TOML front matter to the Raymatic fields and use `address` only for intentional permalink deviations. Recreate shortcodes as ordinary Markdown or explicit presentation logic before introducing custom machinery.

### Jekyll

Move posts and pages into `content/`, convert YAML front matter to TOML, and place static files in `assets/`. Replace collection configuration with directories and native category/tag metadata where possible.

### Astro static projects

Move content collections into Markdown files and move public assets into `assets/`. Keep interactive islands out of the published common path; use them only where the publication genuinely requires client-side behavior. Rebuild layout intent in `presentation/` and verify the generated HTML with `ray check`.

## Validation checklist

1. Create and build a clean Raymatic starter.
2. Move one representative page and preserve its title, date, summary, address, and taxonomy.
3. Move referenced assets and fix root-relative links.
4. Run `ray check` and resolve diagnostics before moving more content.
5. Compare canonical addresses, feeds, sitemap, output links, and legacy URL handling.
6. Run `ray build` twice and compare the resulting output before deployment.
7. Deploy only `output/` after the publication's URLs and generated metadata have been reviewed.
