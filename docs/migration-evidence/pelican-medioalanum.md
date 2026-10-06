# Pelican evidence record: medioalanum

## Record

| Field | Value |
| --- | --- |
| Identifier | `pelican-medioalanum` |
| Source | `https://github.com/medioalanum/medioalanum.github.io` |
| Source generator | Pelican |
| Immutable revision | `438f185f62720839d1f4f2583ee33c9fe0c7c93d` |
| Revision date | 2026-10-04 |
| License | No repository-level license file was found during the inventory. Do not copy source content into a redistributable Raymatic fixture without permission. |
| Inventory method | Shallow clone at the pinned revision, followed by source and configuration inspection. The Elegant submodule was not initialized. |
| Why this project | It is the source of the existing Raymatic dogfooding experiment and exercises portable editorial content plus URL configuration, static assets, a local theme, a submodule, a plugin, feeds, and GitHub Pages deployment. |
| Regression form | Reproducible external inventory. The existing `experiments/dogfooding-01` Raymatic output remains separate evidence, not a copy of this source project. |

## Source inventory

At the pinned revision, the source contains three top-level Markdown articles:

- `content/i-finally-stopped-resisting-type-hints.md`
- `content/my-journey-so-far.md`
- `content/why-we-named-a-project-after-kevin-spacey.md`

Each uses Pelican's header-style metadata, including `Title`, `Date`, `Category`, `Tags`, `Slug`, and `Summary`. Dates include a time component, for example `2026-08-26 10:00`.

The repository also contains:

- `pelicanconf.py` and `publishconf.py`;
- static files under `content/theme/`, a root banner, and an SVG asset;
- a local `themes/medioalanum` theme;
- an uninitialized `themes/elegant` submodule recorded at `b80ebd51779883426e4f17c066c7a0baaa037e75`;
- a `post_stats` plugin used for reading-time metadata;
- production site URL, Atom feed, category feed, archive, category, and tag configuration;
- GitHub Pages deployment configuration described by the repository README.

## Observed source behavior

| Source location or construct | Publication intent | Classification | Raymatic status and required handling |
| --- | --- | --- | --- |
| Article Markdown bodies | Portable editorial prose | Directly representable | Convert to `content/**/*.md` with TOML front matter. Preserve body Markdown subject to link and asset analysis. |
| `Title` | Content title | Directly representable | Map to native `title`. |
| `Date` with time | Publication date and ordering | Representable with transformation | Raymatic currently accepts only `YYYY-MM-DD`. The time component must be reported until a native precision policy is decided. |
| `Category` and `Tags` | Editorial classification | Directly representable | Map to native `category` and `tags`; preserve display casing. Validate taxonomy-slug collisions in future fixtures. |
| `Summary` | Listing and feed description | Directly representable | Map to native `summary`. |
| `Slug` plus `ARTICLE_URL = "{slug}/"` | Canonical article URL | Directly representable | Map the resulting canonical route to `address`. Do not import the Pelican pattern language. |
| `PAGE_URL` and `PAGE_SAVE_AS` | Page-specific routing convention | Requires further evidence | There are no page sources in this project. The readiness portfolio needs a Pelican page fixture before an adapter defines page behavior. |
| `SITENAME`, `SITESUBTITLE`, `AUTHOR`, `DEFAULT_LANG`, `SITEURL`, `SOCIAL` | Site identity and presentation metadata | Representable with transformation | Native `site.toml` supports title, author, description, language, base URL, and social links. Derived feed and archive identity must be completed before import can claim equivalent output. |
| `STATIC_PATHS` and local static files | Published visual assets | Ready with limitations | Copyable into `assets/`; current link and asset portability rules must be verified for theme-relative references and SVG. |
| `themes/medioalanum` | Presentation | Presentation-specific | Recreate visual intent with native templates and assets. Do not execute or copy Jinja inheritance as a runtime dependency. |
| Elegant submodule | Theme distribution mechanism | Source-generator-specific | Report for review. Do not retain a Git submodule or theme package requirement in an imported project. |
| `post_stats` | Reading-time display | Representable with transformation | Raymatic already supplies `reading_minutes` to page templates. Confirm calculation/display equivalence only if it matters to the publication. |
| `DIRECT_TEMPLATES`, archive, tag, and category settings | Derived publication surfaces | Ready with limitations | Raymatic creates archive and category/tag outputs, but paths, markup, site identity, and feeds differ. Inspection must report these differences. |
| Atom and category feed settings | Syndication | Ready with limitations | Raymatic generates RSS and Atom, but must validate standards and consistent identity before import claims preservation. |
| Markdown extension configuration | Markdown rendering choices | Requires human review | Map ordinary portable Markdown; report extension-specific syntax only when it occurs in content. Do not execute source configuration. |
| `pyproject.toml`, `uv`, and deployment workflow | Build and deployment machinery | Source-generator-specific | Do not import. Generate a normal native project and document deployment separately. |

## Findings for the native model

1. The project confirms that title, summary, category, tags, a single author, static assets, site identity, and simple slug-derived canonical URLs are portable publication concepts.
2. It confirms the need for a documented date-precision policy. A date-time source must not silently lose its time component.
3. It exposes the existing site-identity blocker: native page templates can use site metadata, but the generated archive, taxonomy, and feeds do not yet consistently represent it.
4. It does not justify importing theme inheritance, the Elegant submodule, `post_stats`, Python configuration, or GitHub Actions behavior.
5. It does not resolve page roles, aliases/redirects, multiple authors, custom taxonomies, pagination, or page-bundle behavior. Other portfolio records must provide that evidence.

## Required follow-up

- Add a licensed Pelican project containing pages and a non-default permalink or redirect case.
- Add a licensed Pelican project with plugin or template-page behavior to validate review diagnostics.
- Decide and test how a source date-time maps to the current native date field before any import implementation.
- Re-run this inventory at the pinned revision if the source is used to validate a future adapter claim.
