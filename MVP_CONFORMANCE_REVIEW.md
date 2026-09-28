# MVP conformance review

## Basis and review boundary

This review inspected the current implementation, tests, generated project, and
the implementation notes in this repository. `PRODUCT_CONTRACT.md`,
`PRODUCT_MODEL.md`, `MVP.md`, `UX_SPEC.md`, `TECHNICAL_REQUIREMENTS.md`,
`TECHNOLOGY_DECISION.md`, `ARCHITECTURE.md`, and `IMPLEMENTATION_DESIGN.md` are
not versioned here. Their summaries were available through the Raymatic ChatGPT
project, but their exact wording and numeric budgets were not. Findings that
depend on those missing details are explicitly marked as unverified.

## Product test

For the supported path, using Raymatic feels like publishing a small site: one
command creates a working publication, a README names the three user-owned
locations, `dev` preserves a usable preview through invalid edits, `check`
reports actionable source-aware errors, and `build` produces `output/` only when
the complete publication succeeds.

The remaining SSG-like surface is deliberate and small: users must understand
the fixed `content/`, `presentation/`, and `output/` convention, and `dev` uses
a fixed local address. There are no configuration files, plugins, themes, build
graphs, caches, or extra commands.

## Findings

| Classification | Finding | Evidence and disposition |
| --- | --- | --- |
| Must fix before MVP validation | Semantic pipeline timing was recorded as an invented zero duration. | Fixed. Semantic publication construction is now measured with the same timing helper as the other stages; a pipeline test asserts that the timing exists. |
| Must fix before MVP validation | The root README described the project as an unusable bootstrap. | Fixed. It now describes the implemented MVP flow. |
| Should fix | The source model and output plan retain asset-related variants although asset handling is deferred. | `Asset`, `OutputContent::CopyFile`, and asset output provenance are unused. They preserve the documented conceptual model, but should be removed if the authoritative implementation design does not require representability before asset support. No change made in this review. |
| Should fix | `dev` uses the fixed address `127.0.0.1:3000`. | This is low configuration burden for the MVP, but a busy port ends the session. A configurable port is outside the current four-command scope unless UX explicitly requires it. No change made. |
| Acceptable | Front matter is TOML delimited by `+++`. | It is a fixed, documented convention with one required attribute, not a general configuration system. |
| Acceptable | Browser reload uses a 500 ms polling script. | It is the smallest live-refresh mechanism and preserves the coarse invalidation decision. |
| Acceptable | The dev server uses one background thread and shared mutex state. | The blocking HTTP server must continue while the watcher loop waits for changes. No async runtime, scheduler, or generalized event system exists. |
| Deliberately deferred | Assets, custom routing, relative references, configuration files, themes, plugins, RSS, sitemap, taxonomies, deployment, remote data, multilingual support, caching, and a build graph. | Listed as out of scope in the MVP status and not required for the current golden path. |
| Unverified | Exact MVP/UX acceptance criteria and performance budgets. | The authoritative source files are absent from the repository. The existing status maps the known golden path but cannot substitute for those files. |

## Conformance by concern

| Concern | Assessment | Evidence |
| --- | --- | --- |
| Conceptual and command surface | Conforms | Exactly `new`, `dev`, `check`, and `build`; no flags or commands for deferred systems. |
| Configuration burden | Conforms | No project configuration file; generated content and presentation are sufficient. |
| Content portability | Partially verified | Content is plain UTF-8 Markdown plus TOML front matter and static HTML output. Portability constraints beyond that cannot be checked without the product documents. |
| Predictability and determinism | Conforms | Source discovery and diagnostics are ordered; repeated evaluation has a deterministic output plan; output commits are staged. |
| Diagnostics and causal explainability | Conforms for supported failures | Five stable codes cover malformed content, missing title, rendering, address collision, and internal references; diagnostics carry path, span where available, expectation, explanation, and help. |
| Performance | Partially verified | Each one-page debug `check` and `build` measured below 0.01 seconds at `/usr/bin/time -p` resolution. Exact budgets and edit-to-paint measurement remain unverified. |
| Architecture | Conforms | One executable, modular monolith, one shared pipeline, owned semantic model, provenance, structured diagnostics, output planning, staged commit, and coarse invalidation. |
| Dependency and abstraction count | Conforms with one follow-up | Eight runtime crates map directly to CLI, parsing, templates, watching, HTTP, serialization, and errors. There are no project traits, generics, public extension points, feature flags, async runtime, or macro framework. The deferred asset variants are the only premature-looking representation. |
| Generated output | Conforms | The generated project builds to deterministic static `output/index.html`; invalid validation leaves the prior output intact. |

## Rust-specific audit

No custom traits, generic framework, public lifetime API, async runtime, feature
flags, or abstraction layer was found. `Arc<str>` owns source text across
diagnostics; `Arc<Mutex<DevState>>` is the narrow synchronization required by
the server thread and watcher loop. `Box<Diagnostic>` avoids copying a rich
diagnostic through parser and renderer error paths. These uses earn their
complexity in the current implementation.

## Validation

After the two Must Fix changes:

- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test`
- `cargo build`

All must pass before this review is considered complete.
