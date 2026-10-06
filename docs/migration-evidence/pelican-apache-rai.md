# Pelican evidence record: Apache Responsible AI

## Record

| Field | Value |
| --- | --- |
| Identifier | `pelican-apache-rai` |
| Source | `https://github.com/apache/rai-site` |
| Source generator | Pelican |
| Immutable revision | `60fc61bc3734f738d918a0c088c4fbf8ff8427f6` |
| License | Apache License 2.0, recorded in `LICENSE`. |
| Inventory method | Shallow clone at the pinned revision, followed by source and configuration inspection. |
| Why this project | A licensed, page-oriented public site. It provides a counterexample to article-first blog assumptions and exercises non-default routes, home-template selection, assets, plugins, and post-build tooling. |
| Regression form | Candidate for a minimized licensed fixture after attribution, NOTICE, and retained-license requirements are reviewed. Until then, use the pinned source inventory. |

## Source inventory

The source contains fifteen Markdown files under `content/pages/`, including `index.md`, `about.md`, `faq.md`, and policy pages. Every observed content item is a Pelican page, not an article. Each has `Title` and `license` metadata; the home page also has `Template: index`.

`pelicanconf.py` configures:

- `PAGE_PATHS = ["pages"]` and an absent `ARTICLE_PATHS = ["blog"]`;
- `PAGE_SAVE_AS = "{path_no_ext}.html"`, producing flat `.html` page routes;
- `STATIC_PATHS = ["."]`, which mixes CSS, JavaScript, fonts, images, and source-adjacent files under the content directory;
- a local theme in `content/theme`;
- `toc`, `spu`, `gfm`, `asfgenid`, and `asfrun` plugins;
- a post-build `pagefind.sh` invocation;
- disabled archive, feed, author, category, tag, and index outputs.

The source is useful specifically because it is not a conventional article publication. It does not provide evidence for article taxonomies, aliases, redirects, or date-time metadata.

## Observed source behavior

| Source location or construct | Publication intent | Classification | Raymatic status and required handling |
| --- | --- | --- | --- |
| `content/pages/*.md` | Standalone informational pages | Representable with transformation | Files can move into `content/`, but current Raymatic has no explicit page role. Without a native role, non-home pages enter archive, feed, taxonomy, and previous/next handling intended for articles. |
| `content/pages/index.md` with `Template: index` | Publication home presentation | Directly representable with convention | Transform to `content/index.md`; Raymatic selects `presentation/index.html` by convention. Report the source template mapping rather than preserving Pelican template syntax. |
| `PAGE_SAVE_AS = "{path_no_ext}.html"` | Stable flat public routes | Not directly representable | Current native addresses are slash-terminated directory paths. An importer must report each changed canonical route until the native address and alias decision is complete. |
| `Title` | Page title | Directly representable | Map to native `title`. |
| `license` content metadata | Page-level licensing statement | Ready with limitations | Preserve as custom TOML for presentation use only. It has no native derived-output behavior. |
| `SITENAME`, description, domain, URL | Publication identity | Representable with transformation | Map to `site.toml` where fields exist. Derived archive/feed identity remains a native blocker even though this source disables them. |
| `STATIC_PATHS = ["."]` | CSS, JavaScript, font, and image publication | Requires transformation and review | Copy selected publishable files into `assets/`. Do not copy Markdown, theme source, configuration, or build tooling. A future inspection report must list selection decisions. |
| Local Pelican theme and `Template: index` | Presentation | Presentation-specific | Recreate necessary presentation using native templates and assets. Do not execute Jinja templates or copy a theme runtime. |
| `toc`, `spu`, `gfm`, `asfgenid` | Markdown or generated-page processing | Source-generator-specific pending content evidence | Inspect actual affected source syntax. Preserve ordinary Markdown only; report unsupported extension or generated behavior. |
| `asfrun` plus `pagefind.sh` | Post-build search-index generation | Source-generator-specific | Never execute source build hooks. Report search as a separately evaluated product requirement. |
| Disabled archive, feed, taxonomy, and index outputs | Generator-output choice | Requires further native policy | Current Raymatic always emits several derived outputs. Determine whether a page-only publication needs a bounded way to suppress them or whether the difference is an acceptable reported deviation. |

## Findings for the native model

1. A source project can be a conventional static publication without articles. Raymatic needs a minimal explicit role distinction before import can prevent pages from becoming feed and archive entries.
2. The current address model cannot preserve flat `.html` canonical URLs. Aliases alone are not sufficient if the original route must remain canonical; the native URL policy needs an explicit decision.
3. Source-wide static paths cannot be copied mechanically. Import must plan and report selected assets rather than treating an entire source tree as an asset directory.
4. Pelican plugin and post-build behavior must remain inspection findings, not executable dependencies or migration runtime behavior.
5. This evidence does not justify adding general page roles, arbitrary output controls, search, or a plugin API without a focused native design slice and tests.

## Required follow-up

- Define the smallest native content-role contract and fixture tests for a home page, standalone pages, and articles.
- Decide whether Raymatic's canonical-address model remains directory-only or adds a bounded explicit-file-route capability. Do not decide from this one source alone.
- Add asset-selection diagnostics to the future inspection design; `STATIC_PATHS = ["."]` must never cause source configuration or templates to be copied blindly.
- Select the third Pelican source-runtime stress case, then proceed to Hugo and Jekyll evidence records.
