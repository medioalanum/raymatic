# Migration Readiness

## Status

This document defines the evidence required before Raymatic implements automatic source-project inspection or import. It does not claim support for Pelican, Hugo, Jekyll, or another generator.

The current binary has no `ray migrate` command. `scripts/migration-report.sh` is an advisory heuristic and is not a compatibility analyzer.

## Decision to be made

The readiness phase must establish whether conventional publication intent from representative projects can become ordinary Raymatic source without retaining the source generator at build time. The possible outcomes are:

- approve a bounded shared inspection foundation;
- complete a native publication-model gap and repeat the analysis;
- reject automation for a concept that is source-generator-specific;
- defer an adapter because evidence is insufficient.

Approval does not mean that an importer exists. It only authorizes the non-destructive inspection milestone in the roadmap.

## Evidence record

Each selected project requires one record. Source material must be public, legally usable for analysis, pinned to an immutable revision, and either minimized into a redistributable fixture or described by a reproducible inventory procedure.

| Field | Required record |
| --- | --- |
| Identifier | Stable fixture identifier, source generator, repository URL, immutable revision, and license. |
| Representative reason | Which ordinary or difficult publication concept it exposes. |
| Inventory | Content count, page/post kinds, front-matter formats, assets, URLs, taxonomies, feeds, and generated surfaces. |
| Concept rows | Source location, observed behavior, category, proposed native representation, URL effect, and evidence link. |
| Outcome | Directly representable, transformable, presentation-specific, unsupported, or human review required. |
| Risk | Semantic-loss risk and the required diagnostic or confirmation. |
| Regression form | Fixture, inventory data, report golden file, or a documented reason it cannot be redistributed. |

## Candidate fixture portfolio

The following are research candidates, not approved compatibility fixtures. Pinning and license confirmation are required before they enter the test suite.

| Source | Candidate | Evidence purpose | Status |
| --- | --- | --- | --- |
| Pelican | [medioalanum evidence record](migration-evidence/pelican-medioalanum.md) | Real publication with articles, metadata, URL settings, assets, a local theme, a submodule, a plugin, feeds, and deployment configuration. | Recorded at `438f185f62720839d1f4f2583ee33c9fe0c7c93d`; inventory only because no repository license was found. |
| Pelican | [Apache Responsible AI evidence record](migration-evidence/pelican-apache-rai.md) | Licensed page-oriented site with flat `.html` URLs, static assets, home template selection, plugins, and a post-build search step. | Recorded at `60fc61bc3734f738d918a0c088c4fbf8ff8427f6` under Apache-2.0. |
| Pelican | A public site with plugins or template pages | Confirm that plugin and generated-page behavior is classified for review rather than imported. | To select and pin. |
| Hugo | A conventional blog with pages, posts, taxonomy, and static assets | Establish direct front-matter, metadata, URL, and asset transformations. | To select and pin. |
| Hugo | A project using page bundles and explicit permalinks | Determine whether bundle resources require a native asset/link capability or a review finding. | To select and pin. |
| Hugo | A project using shortcodes, multilingual content, or custom output formats | Confirm source-specific machinery is reported rather than executed. | To select and pin. |
| Jekyll | A conventional `_posts` site with pages and front matter | Establish post-date, title, metadata, and permalink transformations. | To select and pin. |
| Jekyll | A project using `redirect_from` or explicit permalinks | Validate the native alias and redirect requirement. | To select and pin. |
| Jekyll | A project using Liquid, collections, includes, or data files | Confirm source runtime behavior is classified as unsupported or review-required. | To select and pin. |

The candidate portfolio deliberately includes an ordinary publication, a URL or asset stress case, and a source-runtime stress case for each generator. It must not use only starter repositories or themes as compatibility evidence.

## Compatibility matrix

Each candidate record contributes rows to the matrix below. A status is a statement about current Raymatic code, not a promise that an importer will support it.

| Publication concept | Current Raymatic status | Required evidence before adapter work | Import policy if unchanged |
| --- | --- | --- | --- |
| Markdown, title, date, summary, single author, draft, category, tags | Native fields exist. Date validation is limited. | Verify source field semantics and date precision in selected projects. | Preserve or transform with an explicit report entry. |
| Home, page, and article roles | No explicit role model. Current behavior infers a home page from `content/index.md` and article-like membership from non-home content. | Mixed content fixtures for every source family. | Block automatic import until a minimal native role model is decided. |
| Canonical address | Native `address` exists and is validated. | Compare source canonical routes and trailing-slash behavior. | Preserve when the route can be represented. |
| Legacy URLs, aliases, redirects | No native redirect model. | Fixtures with source redirects and changed permalinks. | Block claims of URL-preserving import; report every legacy URL. |
| Site identity, language, canonical URL, SEO | Page template context exists; generated outputs do not consistently use it. | Compare site metadata needs across pages, feeds, archives, and taxonomies. | Complete native consistency before import. |
| Assets and Markdown images | Assets copy; root-relative `/assets/` image checking and basic image metadata exist. | Page-bundle, relative, SVG, and linked-download cases. | Preserve supported files; report unsupported reference forms. |
| Presentation and layouts | One default template, a conventional home template, and named flat templates exist. | Identify whether differences are presentation-only or require publication semantics. | Translate presentation intent; do not import theme engines. |
| Archive, taxonomy, feeds, sitemap, robots | Generated with fixed archive/taxonomy markup and partially fixed metadata. | Compare generated surface requirements in real publications. | Complete native identity/standards gaps or report differences. |
| Pagination and custom taxonomies | No native model. | Measure their frequency and semantics in selected projects. | Add a bounded native model only if evidence shows portable intent. |
| Multiple authors | No native model. | Select a project that uses it and record ordering/display requirements. | Report for review unless a native model is added. |
| Shortcodes, Liquid, Jinja templates, plugins, data files, hooks | No source-runtime execution model. | Source-runtime stress fixture for each family. | Never execute. Classify as presentation-specific, unsupported, or review-required. |
| Custom output formats and generator-specific generated pages | No general model. | Projects that use these constructs. | Report for review; do not add a general output framework without separate evidence. |

## Analysis procedure

1. Pin and inventory one candidate at a time.
2. Record every observed concept before proposing a mapping.
3. Classify the concept and link the source file or configuration location.
4. Distinguish publication intent from source runtime machinery.
5. Verify each claimed native mapping against Raymatic source and tests.
6. Record a loss-risk and required user action for every non-direct mapping.
7. Add the approved record as a regression artifact before changing the model or implementing inspection.
8. Re-run all earlier records whenever a native model change alters output, routing, metadata, or diagnostics.

## Exit criteria

- Three pinned projects per source family have completed records.
- Each record contains direct, transformed, presentation-specific, unsupported, and review-required classifications where applicable.
- Every direct mapping has an implementation and test reference; every transformed mapping has a deterministic transformation proposal and regression form.
- Native blockers are either implemented and tested, explicitly deferred with evidence, or rejected as source-specific.
- A representative native publication passes `ray check`, builds identically twice, and has reviewed canonical routes, assets, archive/taxonomy pages, feeds, sitemap, and robots output.
- The evidence does not require source configuration execution, plugin loading, template evaluation, a compatibility runtime, or a generalized build graph.
- A written decision explicitly approves or rejects the shared inspection foundation.

## Current blockers

- Explicit content roles for home, pages, and articles.
- Consistent site identity and standards-compliant metadata in all derived output.
- Native aliases or redirects for public URL preservation.
- Evidence-based decisions on multiple authors, bounded custom taxonomies, and pagination.
- Supported link and asset transformations with diagnostics for unresolved references.
- Stronger date and generated-feed validation.

These blockers are native publication-model questions. They must be resolved by focused vertical slices, not by embedding Pelican, Hugo, or Jekyll behavior in the Raymatic build runtime.
