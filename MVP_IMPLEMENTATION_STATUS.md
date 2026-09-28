# MVP implementation status

## Scope audit

| Item | Classification | Decision |
| --- | --- | --- |
| Create a self-contained publication | Required by MVP | Implemented by `ray new [path]`. |
| Explain the generated layout | Required by UX | Implemented with the generated `README.md`. |
| Write and render Markdown content | Required by MVP | Implemented through `content/**/*.md`, TOML front matter, and `presentation/page.html`. |
| Local edit-preview loop | Required by UX | Implemented by `ray dev` with coarse rebuilds and browser reload polling. |
| User-facing validation errors | Required by MVP and UX | Implemented with deterministic structured diagnostics. |
| Production output without partial invalid replacement | Implementation necessity | Implemented with output planning and staging. |
| Assets, themes, plugins, RSS, sitemaps, taxonomies, deployment, remote data, multilingual content, custom routing, relative-link resolution, persistent cache, build graph | Not required | Not implemented. |

## Implemented capabilities

`ray new [path]` creates a publication containing:

```text
README.md
content/index.md
presentation/page.html
```

The starter content validates and builds without edits. Its README identifies
where to create content, change presentation, preview, validate, build, and find
the generated site.

`ray dev` validates before binding the local preview server, then watches
`content/` and `presentation/`. A valid edit rebuilds the complete publication
and increments the preview revision. An invalid user edit reports diagnostics,
keeps the last valid preview, and recovers on the next valid change.

`ray check` runs discovery, parse, semantic construction, validation, rendering,
and output planning without writing production output. `ray build` uses the same
pipeline and replaces `output/` only after a successful plan is committed.

## Acceptance traceability

The authoritative `MVP.md` and `UX_SPEC.md` remain in the ChatGPT Raymatic
project rather than this repository. The table maps the identified golden-path
requirements to executable evidence; it must be reconciled against those source
files after they are imported before calling the MVP fully accepted.

| MVP / UX requirement | Status | Evidence |
| --- | --- | --- |
| Create a project | Pass | `new_creates_a_valid_publication_with_a_documented_structure`. |
| Understand generated structure | Pass | Generated `README.md`, asserted starter paths in the same CLI test. |
| Create and edit content | Pass | Generated `content/index.md`; pipeline fixtures render Markdown content. |
| Start local development | Pass | `ray dev` binds the local preview after its initial rebuild. |
| Edit content and presentation | Pass | Shared pipeline covers both source classes; `dev` watches both paths. |
| Encounter and understand a user error | Pass | Fixture snapshots cover malformed front matter, missing title, template failure, route collision, and broken internal reference. |
| Validate publication | Pass | `ray check` uses the shared pipeline; generated project test runs it successfully. |
| Produce production build | Pass | Generated project test runs `ray build`; output is `output/index.html`. |
| Understand generated output | Pass | Generated README names `output/`; CLI test asserts the generated file. |
| Preserve valid preview after an invalid edit and recover | Pass | `invalid_edit_keeps_last_preview_and_fix_recovers_with_new_revision`. |
| Deterministic output and diagnostics | Pass | Repeated output-plan and diagnostic-order tests. |
| Never commit invalid production output | Pass | `failed_validation_does_not_change_existing_output`. |

## Validation performed

- `cargo fmt --check` — passed.
- `cargo clippy --all-targets --all-features -- -D warnings` — passed.
- `cargo test` — passed: 20 integration and unit tests, plus doc tests.
- A newly generated publication completed `ray check` and `ray build`; its output
  was `output/index.html`.

## Performance measurements

On the local macOS host, using the already-built debug executable and the
generated one-page publication, `ray check` and `ray build` each reported less
than 0.01 seconds at the resolution of `/usr/bin/time -p`. This is a smoke
measurement, not a performance-budget claim. Edit-to-preview instrumentation is
available as `DevState::last_successful_rebuild`, but no browser-paint timing is
collected.

## Known limitations and unresolved defects

- Exact acceptance criteria and numeric performance budgets cannot be audited
  from the repository because `MVP.md` and `UX_SPEC.md` have not been imported.
- `ray dev` has a fixed `127.0.0.1:3000` address and no port option.
- The preview server returns simple static responses and injects a polling reload
  script only into HTML.
- Source discovery requires the conventional directories; missing structure is a
  project error rather than a structured publication diagnostic.

## Intentionally deferred capabilities

Assets, configuration files, custom routing, relative references, themes,
plugins, RSS, sitemaps, taxonomies, deployment integrations, image processing,
remote data, multilingual support, persistent caching, incremental builds, and a
dependency graph are deferred. None is required for the implemented golden path.

## Deviations from specification

No intentional product or architectural deviation was made. The repository lacks
the authoritative MVP and UX documents, so this status cannot claim formal
completion of any acceptance criterion whose exact wording was unavailable.
