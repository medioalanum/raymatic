# Raymatic roadmap

## Purpose and decision

This roadmap supersedes `docs/ROADMAP_REMAINING.md` as the forward-looking plan. It is based on the implementation and tests in this repository, not on earlier plans or release records. Historical documents remain useful evidence, but do not define the current product contract.

The next objective is to prove that Raymatic can represent ordinary content-oriented publications as native Raymatic projects before it automates migration from another generator. Migration is an import-time workflow, not a compatibility runtime.

The release order is:

1. Publication-model completion and Migration Readiness.
2. Shared migration foundation and non-destructive inspection.
3. Pelican import.
4. Hugo import.
5. Jekyll import.

WordPress is not in this sequence. It becomes an investigation only after all three adapters reuse the same normalized model and the evidence shows the model generalizes beyond filesystem-oriented static generators.

## Current architecture

### Product surface

The public CLI has four commands: `ray new [path]`, `ray dev`, `ray check`, and `ray build`. It has no migration command. `ray new` creates a conventional project. `check` evaluates without writing production output. `build` evaluates and then replaces `output/`. `dev` builds a private `.raymatic-preview/` snapshot, serves it at loopback, and watches source directories.

The shared pipeline is:

```text
discover -> parse -> semantic publication -> validate -> render -> output plan -> staged commit
```

`check`, `build`, and `dev` use this evaluation path. `build` commits to `output/`; `dev` commits a successful revision to `.raymatic-preview/`. A failed validation leaves the previous production output or preview intact.

### Implemented architecture and evidence

| Capability | Current behavior | Evidence and status |
| --- | --- | --- |
| Content discovery | Recursively reads only `content/**/*.md`, in sorted relative-path order. | `src/project.rs`; tested indirectly in pipeline fixtures. Implemented and tested. |
| Front matter and content model | Requires TOML delimited by `+++`. `title` is required. Native optional fields are date, category, tags, author, draft, language, social/SEO overrides, summary, address, and presentation. Unknown TOML is retained for templates. | `src/content.rs`, `tests/pipeline.rs`. Implemented and tested, but unknown metadata has no schema or migration interpretation. |
| Markdown | Renders with `pulldown-cmark`; no shortcode, include, data-file, or executable-content layer exists. | `src/render.rs`. Implemented and tested for ordinary Markdown; unsupported constructs are intentional. |
| Addresses and links | Addresses derive from paths, with optional canonical slash-delimited `address`. Markdown internal links are canonicalized and validated against rendered content addresses. | `src/content.rs`, `src/render.rs`, `src/validate.rs`, `tests/references.rs`. Implemented and tested. No aliases or redirects. |
| Presentation | `presentation/page.html` is the default; `content/index.md` may use `presentation/index.html`; front matter can select one named flat template. Templates are MiniJinja with a fixed context. | `src/pipeline.rs`, `src/render.rs`, `docs/THEMING.md`. Implemented and tested. No template hierarchy, theme package, layouts directory, or generated-page theming. |
| Site metadata | Optional `site.toml` supplies title, author, description, language, base URL, and social links to page templates. | `src/content.rs`, `src/project.rs`, pipeline tests. Implemented and tested for template context. It is not watched by `dev`; generated feeds and archive pages use fixed metadata rather than this configuration. |
| Taxonomies and archive | Generates archive HTML plus one HTML page and RSS feed per category and tag. Names are slugged with a simple ASCII-oriented function. | `src/output.rs`, `tests/publishing.rs`. Implemented and tested. No configurable or custom taxonomy model, pagination, or template control. |
| Feeds, sitemap, robots | Generates RSS, Atom, sitemap, and `robots.txt`; drafts are excluded. `base_url` is applied to sitemap and feeds. | `src/output.rs`, tests, `scripts/verify-release.sh`. Implemented and tested at smoke-test level. Feed channel identity is hard-coded and date strings are not converted to standards-compliant feed timestamps. |
| Assets and images | Recursively copies `assets/`, validates Markdown images under `/assets/`, emits mime and basic PNG, JPEG, WebP dimensions in `assets-manifest.json`. | `src/project.rs`, `src/content.rs`, `src/output.rs`, `tests/pipeline.rs`, `tests/publishing.rs`. Implemented and tested. No image transformation, responsive variants, SVG dimensions, media pipeline, or asset URL rewriting. |
| Validation and diagnostics | Validates front matter, title, coarse ISO date shape, tags, route collisions, root-relative image existence and alt text, and internal content links. Diagnostics are sorted and source-aware. | `src/validate.rs`, `src/diagnostic.rs`, fixture snapshots. Implemented and tested. Validation does not cover all metadata semantics, redirects, generated-output links, template context use, or source-specific imports. |
| Output safety and determinism | Plans all output before writing; rejects duplicate or escaping output paths; stages then renames output with a backup rollback attempt. Deterministic ordering comes from sorted discovery and ordered maps. | `src/output.rs`, `tests/publishing.rs`, `scripts/verify-release.sh`. Implemented and tested. Crash recovery leaves a stale staging or backup directory as an explicit error. |
| Development server | Watches `content`, `presentation`, and `assets`, debounces for 75 ms, fully reevaluates when snapshots change, serves preview files and polling refresh behavior on a loopback port from 3000 through 3009. | `src/dev.rs`, `tests/dev.rs`. Implemented and tested for recovery. It is coarse rebuild behavior, not incremental compilation or a persistent parsed-content cache. |
| Configuration and extension points | `site.toml`, native front matter, custom TOML exposed as `attributes`, and explicit template selection are the only intentional author-facing extension points. | `src/content.rs`, `docs/PUBLICATION_MODEL.md`, `docs/THEMING.md`. Implemented. There is no plugin API, SDK, build graph, hooks, or compatibility mode. |
| Platforms and distribution | CI runs formatting, Clippy, tests, debug build, help, benchmarks, and release smoke checks on Linux, macOS, and Windows. The documented released archive is macOS Apple Silicon only. | `.github/workflows/ci.yml`, `docs/COMPATIBILITY.md`, release scripts. Source checks are cross-platform; distribution support is narrower. |

### Architectural findings

- The core representation is sufficiently explicit to grow by focused vertical slices. It already distinguishes source paths, public addresses, output paths, content metadata, presentation choice, assets, and output plans.
- The current generated publication surfaces are partly separate from presentation. Archive and taxonomy pages are hard-coded HTML, and feeds have hard-coded publication identity. This prevents a site from carrying its visual identity and metadata consistently through all derived outputs.
- The implementation supports one author, categories, and tags, but does not model page versus article as an explicit publication kind. Current behavior infers a home page only from `content/index.md`; navigation, feeds, and archive membership infer article-like content from every non-home, non-draft item.
- The project does not have incremental rendering. The watcher labels changed source classes but invokes full evaluation. This is a measured future optimization, not a current readiness requirement.
- `scripts/migration-report.sh` is a shell heuristic that scans paths and text. It is useful as an advisory inventory, but it is not deterministic structured analysis, does not parse source formats, has no fixture tests, and cannot be described as migration support.

## Migration readiness assessment

The following classifications answer whether conventional publication intent can be represented without importing generator machinery. They are not a promise of automatic import.

| Concept | Classification | Current basis and limitation |
| --- | --- | --- |
| Markdown content, title, date, summary, author, draft, tags, category | Ready with limitations | Native fields exist and tests cover their flow. Date validation is coarse and there is only one author field. |
| Pages versus articles | Missing but likely part of Raymatic's publication model | The current home-page special case and feed exclusion are path-based, not an explicit intent model. |
| Multiple authors | Missing but likely part of Raymatic's publication model | A conventional publication may need ordered author credits. Do not infer it from custom metadata without an output contract. |
| Custom taxonomies | Missing but likely part of Raymatic's publication model | Category and tag are native only; arbitrary source taxonomies should not become an unbounded query system. |
| Explicit URLs or permalinks | Ready | Native `address` is validated and tested. |
| Aliases and redirects | Missing but likely part of Raymatic's publication model | Preserving public URLs requires an explicit redirect artifact, not hosting-specific instructions alone. |
| Static assets and referenced images | Ready with limitations | Asset copying and `/assets/` image validation work. Page bundles, non-root asset references, SVG metadata, and rewriting do not. |
| Image metadata and responsive images | Ready with limitations for basic metadata; missing for responsive behavior | Dimensions are emitted in a manifest. No derivative or responsive rendering behavior exists. |
| Multiple layouts and presentation | Ready with limitations | Explicit flat template selection exists. There is no inheritance, type-level default, or generated-surface presentation. |
| Site metadata, language, canonical URL, SEO and social metadata | Ready with limitations | Page template context supports these values. Generated feeds, taxonomies, and robots do not consistently use them. |
| Archive, taxonomy pages, feeds, sitemap, robots | Ready with limitations | They are generated and smoke tested, but have fixed markup or metadata, no pagination, and limited standards validation. |
| Pagination | Missing but likely part of Raymatic's publication model | It is a common publication navigation behavior, but should be added only after fixture evidence establishes minimal semantics. |
| Previous and next navigation | Ready | Date-sorted entries reach page templates and tests cover order. |
| Internal and root-relative links | Ready with limitations | Markdown links are canonicalized and content targets validated. The parser is deliberately simple and does not cover all Markdown or HTML link forms. |
| Custom front matter | Ready with limitations | Preserved for presentation, but it does not automatically preserve source-generator behavior. |
| Shortcodes, embeds, template code, data files | Source-generator-specific and should not become a Raymatic core feature | Transform portable instances to Markdown, assets, or a deliberate native presentation. Report unresolved cases. |
| Theme inheritance, plugins, collections configuration, output hooks | Source-generator-specific and should not become a Raymatic core feature | Raymatic should import resulting publication intent, not a generator runtime. |
| Generated pages and custom output formats | Requires further evidence | Native derived pages exist, but arbitrary generated pages and output formats have no intentional model. Analyze representative projects before adding either. |

### Actual blockers before automatic import

Automatic migration must not start until the following work has completed. Image derivatives, parsed-content caches, a plugin system, generalized build graph, and source-generator emulation are not blockers and are not proposed.

| Pre-migration task | Current limitation and repository evidence | Priority | Desired behavior, implementation, tests, docs, and acceptance criteria |
| --- | --- | --- | --- |
| Define content role and derived-surface contract | Article-like membership is inferred from non-home paths in `src/pipeline.rs`; archive and taxonomies are fixed HTML in `src/output.rs`. | Blocking | Define the smallest explicit role model for home, page, and article, with defaults that preserve today's simple path convention. Decide derived-surface template context and metadata ownership. Implement only the representation and rendering needed by representative fixtures. Add fixtures for mixed pages/articles, deterministic ordering, and output snapshots. Update publication model and theming guides. Accept when a fixture can express a homepage, standalone pages, and articles without path hacks, while `ray new` remains configuration-free. |
| Make site identity consistent across derived output | `site.toml` is passed to page templates but feeds use `Raymatic publication` and generated archive/taxonomy HTML has fixed language and markup. | Blocking | Make native site title, description, language, base URL, and author available consistently where those outputs need them. Do not add arbitrary global template execution. Add page, feed, sitemap, archive, and taxonomy assertions plus invalid-configuration diagnostics. Update publication model and upgrade notes. Accept when a configured site has no Raymatic placeholder identity in production-derived output. |
| Preserve public addresses safely | `address` exists, but aliases and redirects do not. `docs/MIGRATION.md` currently tells users to review redirects outside the product. | Blocking for imports claiming URL preservation | Add a minimal native alias/redirect declaration with a clearly documented static output form, collision validation, and deterministic planning. Do not adopt source-specific permalink pattern languages or hosting adapters. Add direct address, alias collision, chain, and output golden tests. Document migration and deployment implications. Accept when an importer can retain a canonical address and report or generate each supported legacy route without silent loss. |
| Establish taxonomy and author boundaries | Only category, tags, and one author exist. `src/output.rs` creates category/tag routes using lossy ASCII slugs. | Blocking for sources whose ordinary published taxonomy cannot be mapped | Use readiness fixtures to decide whether multi-author and a bounded custom-taxonomy representation are necessary. If evidence requires them, add a typed, deterministic representation with collision diagnostics and a documented default. Do not implement arbitrary metadata queries. Add Unicode, collision, and fixture tests. Update publication model, theming, and migration compatibility tables. Accept when selected representative projects map all portable taxonomies and author credits, or inspection classifies the remainder for review. |
| Strengthen link and asset portability | Only Markdown links are scanned with string parsing; image validation recognizes `/assets/` paths. Page bundles and many HTML or reference forms have no model. | Blocking for import correctness, but not for all source content | Define the supported portable link and asset forms from fixture evidence. Implement parsing and transformation only for forms Raymatic promises to preserve. Emit diagnostics for unresolved references, source locations, and manual remediation. Add fixtures for relative links, root-relative links, fragments, image assets, and unsupported embedded forms. Update migration guide. Accept when imports neither silently break supported internal links nor claim unsupported forms are preserved. |
| Validate publication metadata and generated standards | Date checks accept impossible dates and feeds write plain date strings into feed fields. | Blocking for release-quality imported output | Validate actual dates and required URL/site values; produce standards-conformant feed timestamps or omit unavailable fields under a documented policy. Add unit tests and golden XML validation. Update compatibility and release documentation. Accept when `ray check` diagnoses invalid imported metadata before build and generated feeds pass the chosen structural validation. |
| Create Migration Readiness evidence | Only one historical Pelican dogfood fixture exists. The current `PELICAN_RAYMATIC_MAPPING.md` predates current capabilities. | Blocking | Build a versioned compatibility matrix and representative source fixtures as described below. Keep projects small, public, licensed for test use, and pinned by revision. Add a reproducible inventory command or test harness only after defining its report schema. Document contributor selection rules. Accept when the matrix supports a go or no-go decision for the shared importer foundation. |

## Migration Readiness phase

### Objective

Validate the publication model against real projects before writing an adapter. This phase may add native product-model slices from the blocker table, but it must not add a `ray migrate import` command or rewrite user projects.

### Fixture selection and analysis

Select at least three representative, publicly accessible projects for each source generator: Pelican, Hugo, and Jekyll. Pin each source revision and record its license, source URL, generator version when known, content count, and why it was selected. The set must include:

- a small conventional blog with posts, pages, tags or categories, assets, and an archive or feed;
- a project using explicit URLs, redirects, or a non-default content layout;
- a project that exposes source-specific machinery such as shortcodes, plugins, collections, bundles, includes, or custom output.

Use the existing `experiments/dogfooding-01` Pelican material as one evidence input, not as the sole representative fixture. Store a minimized, legally redistributable fixture or a machine-readable inventory plus retrieval instructions. Do not depend on a mutable branch tip.

For each project, record every observed concept in `docs/migration-compatibility.md` or a versioned data file consumed by that document. Classify it as directly representable, representable with transformation, presentation-specific, unsupported, or requiring human review. Record the source location, proposed native representation, URL effect, loss risk, test fixture, and decision. The matrix is the source of truth for an adapter's advertised support.

### Exit criteria

- The matrix covers the selected Pelican, Hugo, and Jekyll projects and distinguishes source facts from proposed Raymatic changes.
- Every portable concept in the matrix is either represented by an implemented, tested native feature or explicitly marked as a user-review case.
- The blocker tasks above are complete, deferred with evidence, or removed because fixtures show they are generator-specific.
- A representative native Raymatic project passes `ray check`, builds deterministically twice, preserves its documented canonical addresses, and has inspected generated feeds, sitemap, taxonomy/archive output, and assets.
- There is no request to introduce a plugin API, runtime compatibility layer, source configuration evaluator, or generalized build graph.
- A written decision approves or rejects implementation of the shared migration foundation. Rejection is a valid result.

## Migration architecture after readiness approval

Use a one-way, typed import boundary:

```text
source project
  -> source-specific adapter
  -> normalized migration representation
  -> compatibility analysis and review report
  -> native Raymatic project plan
  -> generated native Raymatic project
  -> ray check and ray build validation
```

The normalized representation should contain source provenance, content body, portable editorial metadata, intended content role, canonical address, legacy addresses, references/assets, presentation classification, and findings. It is an import-domain type, not a new runtime publication engine. Source adapters may parse their own front matter and layout conventions, but source names and configuration semantics must stop at the adapter boundary.

The first command should be non-destructive, for example:

```sh
ray migrate inspect <path>
```

It should emit a stable human-readable report and a machine-readable form suitable for fixtures. It reports what will be preserved, transformed, skipped, or needs review. It must never modify the source project.

Only after inspection has demonstrated coverage should import be added:

```sh
ray migrate import <path> <destination>
```

It must require a new or empty destination, never overwrite the source, emit its report alongside the generated project, and run native validation before reporting success. The result is ordinary `content/`, `presentation/`, `assets/`, `site.toml`, and redirect declarations. It must not need the source generator installed at build time.

Safety invariants:

- Never silently discard a meaningful known publication semantic.
- Preserve canonical public URLs where Raymatic can represent them; list every exception.
- Report unsupported and ambiguous constructs with source locations and an action.
- Make report and generated-project output deterministic for an identical input and adapter version.
- Treat generated content as normal user-owned Raymatic source after import.
- Do not execute source templates, plugins, shortcodes, configuration code, or arbitrary build hooks.

## Release sequence

### Milestone 1: Publication-model completion and Migration Readiness

**Objective:** Complete only the native publication-model gaps proven by the readiness fixtures and make a go or no-go decision for import.

**User-visible behavior:** A conventional content publication can express its roles, site identity, portable URLs and redirects, taxonomy and author semantics when evidence requires them, and link/asset diagnostics without source-generator configuration.

**Implementation scope:** Complete the blocker tasks, construct the compatibility matrix, and add representative native and source inventories. Keep evaluation and staged output architecture intact.

**Non-goals:** Automatic import, generator support claims, plugins, cache/build graph work, shortcodes, source configuration execution, responsive images unless evidence establishes it as a portable publication requirement.

**Tests and fixtures:** Add model-level unit tests; end-to-end fixtures for publication roles, aliases, identity, taxonomy/author boundaries, links/assets, and standards-compliant derived output; deterministic output hashes; migration-readiness inventories for each generator family.

**Documentation and communication:** Replace outdated migration promises with readiness status. Update `docs/PUBLICATION_MODEL.md`, `docs/THEMING.md`, `docs/COMPATIBILITY.md`, `docs/UPGRADING.md`, and the README only if the native authoring surface changes. Add compatibility-matrix guidance and a decision record. Update the website only for user-visible native capabilities. Add a CHANGELOG entry when released.

**Dogfooding and release validation:** Rebuild the repository website and the Pelican dogfood publication using only documented features. Run the quality gate below and review the matrix decision.

**Exit criteria:** Every readiness exit criterion passes, all new native semantics have documentation and fixtures, and the project has an explicit approval to build `migrate inspect`.

### Milestone 2: Shared migration foundation and inspection

**Objective:** Deliver a safe, adapter-neutral inspection workflow and normalized representation before any importer writes a project.

**User-visible behavior:** `ray migrate inspect <path>` identifies the source type only when evidence is sufficient and returns preservation, transformation, unsupported, and review findings without modifications.

**Implementation scope:** Add the CLI command, report schema, normalized migration representation, adapter interface internal to the binary, source detection, deterministic report rendering, and a fixture harness. Implement the minimum detector and adapter plumbing needed for the first adapter, but do not advertise import.

**Non-goals:** Importing content, modifying source projects, general plugin adapters, executing source configuration, claiming Pelican, Hugo, or Jekyll support beyond the tested inspection subset.

**Tests and fixtures:** CLI exit behavior; no-write source hash assertions; stable report golden tests; malformed/ambiguous project diagnostics; normalized-representation unit tests; all readiness inventories remain regression fixtures.

**Documentation and communication:** Add a migration architecture and inspection guide with the report vocabulary and safety guarantees. Update README only if `migrate inspect` is public. Add CHANGELOG and GitHub release notes explaining that inspection is not import. Website copy may link to the guide, but must not claim migration completion.

**Dogfooding and release validation:** Inspect the pinned Pelican dogfood source and representative Hugo/Jekyll inventories. Run the quality gate and verify reports are deterministic on supported CI platforms.

**Exit criteria:** The command is non-destructive, reports have golden coverage, source-specific concepts do not reach Raymatic runtime types, and a Pelican inspection fixture reaches the expected findings without manual report editing.

### Milestone 3: Pelican migration

**Objective:** Produce native Raymatic projects from the Pelican subset proven in the matrix.

**User-visible behavior:** `ray migrate inspect` identifies documented Pelican constructs. `ray migrate import` generates a new project for supported input and reports transformations and review items.

**Implementation scope:** Implement the Pelican adapter on the shared representation. Translate documented metadata, pages/articles, content, static assets, canonical addresses, supported legacy addresses, and presentation classification. Generate a deliberately small native presentation or map to documented templates. Validate the destination with `ray check`.

**Non-goals:** Executing `pelicanconf.py`, importing plugins, reproducing Jinja inheritance, copying themes verbatim, guaranteeing visual parity, or supporting untested Pelican conventions.

**Tests and fixtures:** Add pinned Pelican golden sources, expected inspection reports, native project snapshots, source-hash/no-overwrite tests, URL and asset comparisons, `ray check` and deterministic double-build tests. Keep Migration Readiness fixtures passing.

**Documentation and communication:** Publish a Pelican compatibility table with supported mappings, transformations, review cases, exclusions, and validation procedure. Update README and website only to state the tested support boundary. Add upgrade and compatibility notes if the new CLI or native schema affects existing users. Add CHANGELOG and release notes.

**Dogfooding and release validation:** Re-run `experiments/dogfooding-01` through inspection and import. Compare canonical routes, content counts, assets, feeds, sitemap, and manually selected structural output. The result must be deployable as an ordinary Raymatic project.

**Exit criteria:** Tested Pelican fixtures import deterministically, `ray check` succeeds, all findings are accounted for, public URL preservation is measured, and no Pelican runtime dependency or compatibility mode remains in the generated project.

### Milestone 4: Hugo migration

**Objective:** Add Hugo import by extending the shared architecture, not by adding a second migration pipeline.

**User-visible behavior:** Inspection and import support the documented Hugo subset and make page-bundle, shortcode, taxonomy, layout, and permalink limitations explicit.

**Implementation scope:** Add a Hugo adapter that normalizes documented front matter formats and filesystem conventions. Implement only transformations approved by the compatibility matrix. Reuse reports, project planning, validation, and destination safety from the Pelican work.

**Non-goals:** Hugo template execution, shortcode runtime, module resolution, multilingual emulation, arbitrary output formats, or importing Hugo configuration as Raymatic configuration.

**Tests and fixtures:** Add Hugo report/import goldens, page-bundle asset cases, and manual-review diagnostics. Run every Pelican migration fixture unchanged, plus all native quality tests. Add cross-adapter report determinism tests.

**Documentation and communication:** Add Hugo-specific compatibility documentation and update the common migration guide. Communicate exact supported Hugo conventions and review requirements. Update README/website only for the new public support claim; add CHANGELOG, release notes, and compatibility/upgrade notes as applicable.

**Dogfooding and release validation:** Import one representative Hugo fixture into a clean destination, then validate and build it twice. Re-run Pelican dogfooding and all migration goldens.

**Exit criteria:** Hugo support reuses the shared representation and import planner, tested Hugo projects satisfy their declared preservation goals, and Pelican behavior has no regression.

### Milestone 5: Jekyll migration

**Objective:** Add the documented Jekyll subset through the same normalized import boundary.

**User-visible behavior:** Inspection and import support tested posts, pages, front matter, static assets, permalinks, redirects, and declared review cases.

**Implementation scope:** Add a Jekyll adapter for tested filesystem and front matter conventions. Normalize posts and pages to native roles and addresses. Reuse alias/redirect, report, destination, and validation behavior.

**Non-goals:** Liquid execution, collections emulation beyond proven native roles, plugin execution, GitHub Pages compatibility emulation, data-file runtime, or custom output formats.

**Tests and fixtures:** Add Jekyll report/import goldens, dated-post and permalink cases, redirect cases, unsupported Liquid diagnostics, and destination safety checks. Run all Pelican and Hugo fixtures as mandatory regressions.

**Documentation and communication:** Add the Jekyll compatibility table, revise the common matrix, and explain how to review a migrated project. README and website changes are appropriate only once the supported subset is documented and tested. Add CHANGELOG, release notes, and relevant compatibility/upgrade notes.

**Dogfooding and release validation:** Import and validate a representative Jekyll project, then rerun Pelican and Hugo dogfooding. Compare content counts, canonical and legacy routes, assets, generated feeds, sitemap, and inspection findings.

**Exit criteria:** All three adapters use one normalized representation, project planner, report vocabulary, safety contract, and regression suite. Each adapter's tested scope is documented without broad compatibility claims.

### Future investigation: WordPress

Investigate WordPress only if the three completed adapters show that the normalized representation, report model, destination plan, and review workflow remain stable across their differences. The investigation gate is evidence that at least 90 percent of the portable concepts in the three adapter matrices use shared normalized fields and no adapter required a source-specific runtime behavior. WordPress is a different source shape and may need API, database export, media, and URL evidence. It is not a planned import release.

## Common release quality gate

Every release must satisfy the current repository commands, plus the listed additions when their capability exists:

```sh
cargo fmt --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked
cargo build --locked --release
./scripts/verify-release.sh
```

- CI must continue to run the quality suite on Linux, macOS, and Windows. Do not claim a distributed binary target without a built, tested artifact and a documented compatibility decision.
- Fixture and CLI tests must cover public commands, errors, diagnostics, no-write guarantees, deterministic ordering, and safe output replacement.
- Each new derived output must have a structural regression test. Each migration release must add inspection and import goldens, source-hash assertions, destination safety checks, and deterministic double-build checks.
- Adding Hugo must run Pelican fixtures. Adding Jekyll must run Pelican and Hugo fixtures. No adapter may lower a prior adapter's documented guarantee.
- Run `scripts/benchmark.sh` and record results for a release when model or migration work changes the build path. Do not add caching unless benchmark evidence demonstrates a user-facing need.
- Run release packaging and verify archive checksum, binary startup, `ray new`, `ray check`, and `ray build` for every published target. The current package script and release smoke test are necessary but do not create multi-platform distribution by themselves.
- Review documentation consistency: README, durable guides, compatibility, upgrading, migration matrix, release guide, website content, CHANGELOG, and release notes. Remove or mark contradicted historical claims before making a public claim.
- Rebuild the repository website in CI and dogfood the relevant publication before release. Confirm generated website documentation is synchronized rather than hand-edited under `site/content/reference/`.

## Documentation and communication strategy

| Surface | Rule |
| --- | --- |
| `README.md` | Discovery, positioning, installation, current command surface, short quick start, and only material public capability changes. It is not the complete manual. |
| `docs/` durable guides | Normative user and contributor guidance: installation, publication model, theming, deployment, compatibility, upgrading, release procedure, and migration. Keep each topic authoritative in one guide. |
| Architecture/contributor decisions | This roadmap and concise decision records explain current boundaries and accepted tradeoffs. Link to tests or implementation where behavior is normative. |
| Migration documentation | The compatibility matrix, shared safety contract, adapter-specific support tables, inspection report vocabulary, import workflow, and manual-review procedure. |
| Compatibility and upgrading | State supported source contract, platform/distribution reality, breaking changes, native schema changes, and before/after validation. |
| `CHANGELOG.md` | Concise user-visible changes, deprecations, fixes, migration support boundaries, and compatibility changes. |
| GitHub release notes | Installation artifact, target support, validation summary, major user-facing changes, upgrade actions, known limits, and links to durable guides. |
| Website | Product capability and release communication, concise guides, and links to durable technical documentation. Do not duplicate the full manual. |
| Historical root documents | Retain as dated evidence and decisions, clearly marked historical where they conflict with current code. Do not use them as feature documentation. |

The generated `site/content/reference/` copies are not independent documentation. `scripts/sync-docs-site.sh` recreates them, so edit their root or `docs/` source only.

### Documents requiring consolidation or historical marking

- `docs/ROADMAP_REMAINING.md` is superseded by this roadmap. Its ordering puts automatic migration ahead of the readiness evidence required here.
- `docs/MIGRATION.md` should be revised during Milestone 1. Its opening language implies a practical migration workflow even though the repository only has a heuristic shell report and no importer.
- `PELICAN_RAYMATIC_MAPPING.md` is outdated. It describes date, category, tags, assets, archive, and reading time as absent or workarounds, while current code implements them. Preserve it as the dated dogfooding mapping or replace it with a tested compatibility matrix.
- `MVP_IMPLEMENTATION_STATUS.md`, `MVP_RELEASE_READINESS.md`, `MVP_CONFORMANCE_REVIEW.md`, `PRE_RELEASE_PRODUCT_REVIEW.md`, `RELEASE_HARDENING_REPORT.md`, and `RELEASE_EVIDENCE_RESOLUTION.md` contain historical release-stage claims that conflict with the current implementation and README. Mark their scope and date prominently rather than treating them as current reference.
- `IMPLEMENTATION_PLAN_V2.md` and `PRODUCT_DIRECTION_DECISION_02.md` are historical decision inputs. The latter accurately says it supersedes an earlier hold, but neither should substitute for a current roadmap.
- The README repeats detailed capability lists, diagnostics, and performance/compatibility material that belong in durable guides. Keep it concise when the next user-visible release revises it.

## Evidence and progression metrics

Use evidence that measures preservation and clarity rather than implementation volume.

| Milestone | Required evidence to proceed |
| --- | --- |
| Migration Readiness | Matrix coverage for three representative projects per generator family; percentage of observed concepts directly representable or transformable; documented review and unsupported counts; native fixture builds and checks pass deterministically. |
| Inspection foundation | Zero source writes in tests; stable reports; percentage of detected concepts classified with source locations; no unclassified silent drops in fixtures. |
| Pelican import | Imported content count, canonical URL preservation rate, legacy URL treatment rate, successful `ray check` rate, deterministic import/output rate, and number of required manual interventions by category. |
| Hugo import | The same Pelican measures for Hugo plus proof that Pelican fixtures retain their prior results. |
| Jekyll import | The same measures for Jekyll plus proof that Pelican and Hugo fixtures retain their prior results. |

Do not use lines of code, number of source generators named, or raw build speed as success metrics. A lower coverage result is useful when it correctly identifies source-specific machinery that Raymatic should not adopt.
