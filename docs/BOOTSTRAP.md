# Bootstrap scope and design traceability

The implementation design was read from the full `IMPLEMENTATION_DESIGN.md`
preview in the ChatGPT project Raymatic, conversation
[Design Rust Implementation](https://chatgpt.com/c/6abaa479-c0b4-83ed-9c12-59f309e33e42).
Product contract, product/model/MVP/UX summaries and the final Rust decision were
reviewed in the preceding documentation analysis. The full MVP/UX and technical
requirements files have not been imported into this repository; summaries must
not substitute for their exact syntax or numeric budgets.

Rust is the final user decision. Historical Rust/Go gates are not implementation
prerequisites. No language comparison is planned.

The current user request explicitly limits this stage to a compiling bootstrap;
the full design describes the eventual MVP. Accordingly:

- Single package, Rust 2024, pinned toolchain, thin binary/internal library,
  owned semantic types, diagnostics and output-plan data follow the design.
- `clap` uses its builder API. `thiserror` represents real filesystem errors.
  `tempfile` isolates integration tests. Cargo.lock fixes dependency resolution.
- `pulldown-cmark`, MiniJinja, serde and TOML are used by the content-to-HTML
  path. `notify` and `tiny_http` provide the blocking local development loop.
- `validate`, `render`, and `dev` have concrete, narrow responsibilities and
  no trait-based abstraction layer.
- No address constructor or front-matter grammar is invented from summaries.
  Exact syntax/directory binding is deferred with actual publication creation.
- Assets retain source paths rather than requiring UTF-8 source text, so binary
  assets are representable. Source labels optionally retain owned source text
  so error locations remain available after parsing failures.
- Pipeline failures retain typed application errors; publication diagnostics
  remain a distinct variant. Unimplemented capabilities are not content errors.
- Unexecuted timing stages are `None`, not fictitious zero measurements.
  No performance-budget compliance is claimed for a non-publishing bootstrap.
- CI is configured for Linux, macOS and Windows; local execution only verifies
  the host platform. Remote CI has not run until this repository is pushed.

No product or architecture redesign is introduced. The current slice binds TOML
front matter (`+++` delimiters), `content/index.md`, `presentation/page.html`,
`{{ title }}`, `{{ body }}`, `/`, and `output/index.html`; this is a deliberately
small convention pending confirmation against the full MVP/UX artifacts.
These are milestone staging
choices and small concrete data-representation refinements to representative
Rust signatures. No production content or output is written in this stage.

## Validation diagnostics

`check` runs one shared pipeline over every Markdown file beneath `content/`, in
deterministic path order. Current structured diagnostic codes are `CONTENT001`
(malformed front matter), `CONTENT002` (missing title), `RENDER001` (template
failure), `ADDR001` (route collision), and `REF001` (unresolved absolute
internal reference). Diagnostics preserve byte spans and calculate line/column
only in the terminal presentation layer.

There is no project configuration format yet, so invalid-project-configuration
validation is intentionally not implemented. Missing required source directories
and files remain typed filesystem/project errors. Relative links, links embedded
in arbitrary Markdown constructs, assets and template-source spans remain beyond
this diagnostic slice.

## Development loop

`ray dev` starts a blocking local server on `127.0.0.1:3000`, watches only
`content/` and `presentation/`, coalesces notification noise for 75 ms, and
re-runs the shared evaluation pipeline. Successful results replace a complete
`.raymatic-preview` snapshot and increment an in-memory revision. Served HTML
polls `/_raymatic/revision` every 500 ms and reloads when that revision changes.
An invalid rebuild records and prints its structured diagnostics without changing
the existing preview or revision. The state records the duration from rebuild
start to preview commit; no numeric UX budget is asserted because the full
technical-requirements artifact is unavailable locally.
