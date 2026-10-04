+++
title = "MVP RELEASE READINESS"
+++


## 1. Executive Status

The implementation is **READY WITH REQUIRED HARDENING** for a controlled
pre-release review. The actual release binary works outside the development
repository on macOS Apple Silicon, and its golden path, diagnostics, output
integrity, determinism, and watcher recovery have been exercised. The artifact
must not be described as cross-platform or performance-budget compliant until
the authoritative platform matrix and budgets are available and measured.

This is a technical release decision. It does not claim product-market
validation, adoption, switching value, migration demand, or differentiation.

## 2. MVP Release Boundary

### Implemented and supported

- Technical Markdown publications in `content/**/*.md`.
- TOML front matter delimited by `+++`, with required `title`.
- Markdown body rendering into `presentation/page.html` using `title` and `body`.
- Addresses derived deterministically from content paths: `index.md` is `/`,
  nested `index.md` is its directory address, and other files are directory
  addresses.
- Absolute internal-reference validation for the supported link form.
- `new`, `dev`, `check`, and `build`.
- Deterministic HTML output in `output/` and staged output replacement.
- Structured diagnostics for malformed content, missing title, template failure,
  route collision, and unresolved absolute internal references.

### Implemented but experimental

- Polling-based browser reload in `dev`.
- A fixed local server address at `127.0.0.1:3000`.
- The current release artifact and archive layout.

### Explicitly unsupported

Assets, custom routing, relative-reference resolution, project configuration,
themes, plugins, RSS, sitemaps, taxonomies, deployment integrations, image
processing, remote data, multilingual content, persistent caching, incremental
builds, and a build graph.

### Unknown

Long-running maintenance behavior, large publications, performance against the
authoritative budgets, and behavior on platforms other than the tested target.

## 3. Release Artifact

| Field | Value |
| --- | --- |
| Version | `v0.1.0` |
| Binary | `ray` |
| Target | `aarch64-apple-darwin` |
| Archive | `raymatic-v0.1.0-aarch64-apple-darwin.tar.gz` |
| SHA-256 | `b544ac9c0cc7fff744ffd10150bf8ef38b9876830ee6ca8c6b75f7d58012beec` |
| Format | gzip-compressed tar archive containing the standalone executable |
| Invocation | `ray new`, `ray dev`, `ray check`, `ray build` |
| Installation location | Any directory on `PATH`, such as `$HOME/.local/bin/ray` |
| Project structure | `README.md`, `content/`, `presentation/page.html`; `output/` after build |
| Distribution | [GitHub draft release](https://github.com/medioalanum/raymatic/releases/tag/untagged-6a5ec93983abcf4bb492) |

## 4. Supported Platform Matrix

| Platform | Artifact | End-to-end status | Claim |
| --- | --- | --- | --- |
| macOS Apple Silicon | Release archive | Startup, new, dev, watch, check, build, paths, diagnostics verified | Supported for controlled pre-release |
| macOS Intel | None | Not tested | Unknown |
| Linux x86_64 | None | CI source checks only; no release artifact or watcher run | Not claimed |
| Windows x86_64 | None | CI source checks only; no release artifact or watcher run | Not claimed |

Required hardening: import the authoritative platform decision and either test
the intended matrix or explicitly narrow v0.1 to macOS Apple Silicon.

## 5. Installation Results

The archive was extracted into a temporary directory outside the repository.
From that clean-machine-like location, the standalone binary returned `ray 0.1.0`,
created a publication in a Unicode path (`publicação ü`), passed `check`, passed
`build`, and wrote `output/index.html`. No Cargo state, source-tree file,
developer-specific environment variable, or repository-relative asset was used.

The documented install flow is in `docs/INSTALL.md`. Uninstallation is deleting
one executable; upgrade is replacing it with a verified archive. No package
manager integration is required for this pre-release.

## 6. CLI Audit

The release binary was checked for root help, version, help for each command,
unknown command, and execution in generated and invalid projects.

- `--help` and `--version` return 0.
- `new`, `dev`, `check`, and `build` are the only conceptual commands.
- Unknown commands and invalid arguments return 2 with usage guidance.
- Publication and diagnostic failures return 1.
- Normal output is on stdout; diagnostics and operational errors are on stderr.
- `check` evaluates without committing production output.
- `build` commits only after evaluation and output planning succeed.
- The optional `new [path]` argument has a safe default and refuses non-empty
  targets without replacing their contents.

## 7. Golden-Path Results

Using the release executable: create publication; inspect generated README and
structure; add content; start `dev`; edit content; edit presentation; create a
missing-title error; observe `CONTENT002`; restore content; run `check`; run
`build`; inspect `output/index.html`. The flow completed successfully.

The release watcher produced revisions `1 → 2 → 2 → 3`: a valid edit advanced
the preview, the invalid edit retained the previous revision, and the correction
recovered automatically. A second smoke run covered a new file, presentation
edit, deletion, and rename with revisions `1 → 2 → 3`.

## 8. Correctness and Output Integrity

The pipeline is shared by `dev`, `check`, and `build`. Invalid content, template
failures, route collisions, and broken references prevent an output plan from
being committed. Existing output remains intact after failed validation. Output
paths are relative and reject absolute or parent-directory escapes. Route
collisions are diagnosed before planning. Repeated builds use staged directories
and replace the complete output only after all entries are written.

## 9. Determinism

Content discovery is sorted by relative path; diagnostics are sorted by source
path, code, and summary; addresses and output paths are derived functions; and
fixture tests compare repeated output plans and diagnostic ordering. The release
clean-machine run also produced the same generated structure and output path.
No known behavior depends on filesystem iteration order or watcher event order;
watcher events are coalesced before a coarse rebuild.

## 10. Diagnostic Audit

| Case | Code | Result |
| --- | --- | --- |
| Malformed front matter | `CONTENT001` | Source path, location where available, cause, expected syntax, and fix. |
| Missing title | `CONTENT002` | Source path/location, semantic cause, expected attribute, and fix. |
| Rendering failure | `RENDER001` | Presentation path, template explanation, expected template shape, and fix. |
| Route collision | `ADDR001` | Each colliding source, derived address, expected uniqueness, and fix. |
| Broken absolute internal reference | `REF001` | Link span, target, expected address, and fix. |

There is no project configuration format, so configuration diagnostics are not
applicable. Expected user errors are not presented as raw Rust errors. Source
provenance uses stable codes, error severity, owned paths, and source spans where
the parser provides them.

## 11. Development-Loop Audit

`dev` performs discover → evaluate → serve → watch → coarse rebuild → report →
refresh. Content, presentation, creation, deletion, rename, rapid successive
saves, invalid content, and correction were exercised. A recoverable content
error leaves the server and last valid preview alive. The server is started after
the initial evaluation, so an invalid project structure does not mask its error
with a port-binding failure.

## 12. Performance Measurements

Measured on macOS 15 / Apple Silicon, Rust release binary, one-page generated
publication, using `/usr/bin/time -p`:

| Path | Measurement | Classification |
| --- | --- | --- |
| `check` | `< 0.01 s` at timer resolution | Budget comparison unavailable |
| `build` | `< 0.01 s` at timer resolution | Budget comparison unavailable |
| Cold startup | Not separately measured | Unknown |
| First preview | Not separately measured | Unknown |
| Edit-to-visible | Revision recovery verified; browser paint not measured | Unknown |

The exact budgets from `TECHNICAL_REQUIREMENTS.md` are not present in this
repository. Required hardening is to import them and repeat measurements with
the specified methodology and representative publication sizes. No Rust
performance potential is used as evidence.

## 13. Cross-Platform Results

Only macOS Apple Silicon has a release artifact and end-to-end result. CI runs
formatting, Clippy, tests, and build checks on Linux, macOS, and Windows, but
successful compilation is not treated as platform support. No cross-platform
claim is made.

## 14. Dependency Audit

| Dependency | Purpose | Assessment |
| --- | --- | --- |
| `clap` | Four-command CLI and help/exit behavior | Required. |
| `pulldown-cmark` | Markdown rendering | Required. |
| `toml` + `serde` | Front matter parsing | Required by content convention. |
| `minijinja` | Presentation rendering | Required by the template model. |
| `notify` | Filesystem watching | Required by `dev`. |
| `tiny_http` | Local preview server | Required by `dev`. |
| `thiserror` | Typed environment/application errors | Required for error boundaries. |

`tempfile` is test-only. No unused direct dependency or overlapping experiment
was found. No dependency change is required for this release review.

## 15. Rust Complexity Audit

There are no custom traits, public generic frameworks, async runtime, feature
flags, or unsafe code. `Arc<str>` preserves source ownership for diagnostics;
`Arc<Mutex<DevState>>` connects the blocking server thread and watcher loop;
`Box<Diagnostic>` carries rich parser/render errors without unnecessary copying.
These complexities serve current requirements. Asset-related representation is
deferred and should remain under observation, but removing it now would alter
the documented semantic boundary without release evidence.

## 16. Test and Quality Results

Passed:

- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test` — 20 tests plus doc tests
- `cargo build --release`
- `cargo test --release`
- release archive clean-machine smoke test
- release watcher smoke tests for edit, invalid content, recovery, add, delete,
  rename, and presentation edit

## 17. Security/Safety Sanity Check

Output path validation rejects absolute and parent-directory paths. Staged
output prevents partial production replacement. Existing `new` targets are not
overwritten. The preview binds to loopback only. HTML templates are project-local
MiniJinja templates; no remote code or plugin execution exists. Malformed content
produces diagnostics rather than panics in exercised cases. A full dependency
security audit is outside this proportionate review and remains a release
hardening item if the distribution becomes public.

## 18. Documentation Readiness

`docs/INSTALL.md` covers installation, quick start, generated structure, content,
presentation, `dev`, `check`, `build`, and output location. Generated project
README explains the same workflow after `new`. CLI help covers command discovery;
diagnostics cover expected failures. The documentation does not teach internal
architecture or future capabilities.

## 19. Scope Audit

No new capability was implemented. Release blockers and hardening items are
evidence, platform, and measurement concerns. Package-manager integration,
plugins, themes, migration, RSS, sitemap, image processing, and generalized
extension hooks remain post-MVP capabilities.

## 20. Product Hypotheses Still Requiring Evidence

| Hypothesis | Existing evidence | Status |
| --- | --- | --- |
| Operational complexity is significant | Internal design rationale only | Inference only |
| Value exists beyond beginners | No participant sessions | No meaningful evidence |
| Strong defaults cover a useful common case | One generated publication and fixtures | Inference only |
| Explanations change user behavior | Structured diagnostics only | No meaningful evidence |
| Portability affects product choice | Portable file structure only | No meaningful evidence |
| Migration friction affects adoption | No migration session | No meaningful evidence |
| Migration intelligence would matter | No evidence | No meaningful evidence |
| The product remains substantially smaller than competitors | Four-command implementation only | Inference only |

## 21. Release Blockers

No correctness blocker was found for the tested target. Before final public v0.1,
the following evidence blockers must be resolved:

- import the authoritative performance budgets and measure against them;
- resolve and verify the intended platform matrix, or explicitly narrow v0.1 to
  macOS Apple Silicon;
- decide whether the draft release should become public after those checks.

## 22. Hardening Required

Import the missing authoritative documents into the repository, run the specified
performance methodology, and add release artifacts only for platforms actually
verified. Perform a proportionate dependency/security review before broad public
distribution.

## 23. Known Limitations

The server uses a fixed loopback port, browser paint timing is not measured,
relative references and assets are unsupported, and only macOS Apple Silicon has
a verified release artifact. These limitations are documented and do not alter
the MVP model.

## 24. Post-Release Evidence Questions

Observe whether developers can install and use the artifact without intervention,
whether diagnostics change recovery behavior, whether conventions remain useful
for experienced SSG users, whether portability affects choice, and whether the
reduced operational surface is valuable enough to overcome switching cost. These
are product hypotheses, not release claims.

## 25. Final Readiness Decision

**READY WITH REQUIRED HARDENING**

The actual MVP can proceed to controlled pre-release review because its tested
artifact is standalone, its golden path works outside the repository, and its
failure/recovery and output guarantees are exercised. It is not yet a final
public v0.1 claim because platform support and product-derived performance budgets
remain unresolved. No user-validation claim is made.
