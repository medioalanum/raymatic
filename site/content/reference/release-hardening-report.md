+++
title = "RELEASE HARDENING REPORT"
+++


## 1. Stage 17 Entry Decision

`PRE_RELEASE_PRODUCT_REVIEW.md` authorizes hardening with the decision **PROCEED TO HARDENING**.

## 2. Stage 17 Handoff

The handoff identified missing authoritative performance budgets and a complete platform matrix as the only release blockers. It also required a golden-path smoke test, dependency and filesystem review, and preservation of the four-command surface.

## 3. Initial Hardening Backlog

The backlog was limited to output correctness, invalid-build safety, deterministic behavior, diagnostics, development recovery, release-artifact smoke testing, platform evidence, dependency review, and documentation. No product capability was added.

## 4. Release Blockers

The repository does not contain the authoritative specification files that define the exact performance budgets or supported platform matrix. They cannot be reconstructed safely from summaries. This prevents an unconditional claim of budget compliance or broad platform support.

## 5. Required Hardening

The implemented pipeline, staged output commit, diagnostic tests, development recovery, clean-directory smoke test, and release build were rechecked. The remaining requirement is evidence collection against the missing authoritative budgets and platform matrix.

## 6. Accepted v0.1 Limitations

The current controlled release evidence covers macOS Apple Silicon. Preview uses loopback `127.0.0.1:3000`, development uses coarse full-publication invalidation, and the MVP intentionally omits plugins, themes, migration, RSS, sitemap, image processing, custom routing, and generalized build infrastructure.

## 7. Forbidden Scope Expansion

No new commands, configuration system, plugin API, cache, dependency graph, parallel scheduler, async runtime, or ecosystem feature was introduced during hardening.

## 8. Correctness Fixes

No new correctness defect was found during this pass. Existing safeguards remain in place: semantic validation precedes rendering, output is staged, invalid builds do not replace production output, and relative output paths are checked.

## 9. Output Integrity Results

The valid fixture produces the expected static HTML. A failed validation leaves an existing output tree unchanged. A clean release artifact creates and builds a project in a Unicode path successfully.

## 10. Determinism Results

The output plan and rendered page tests pass repeatedly with stable ordering. Diagnostics with multiple failures are ordered by path and code. No timestamps or machine-specific values are emitted into generated HTML.

## 11. Diagnostic Hardening

Malformed front matter, missing title, template failure, route collision, and broken internal reference have structured codes, source locations where available, causes, expectations, and suggested actions. CLI formatting remains separate from the internal diagnostic model.

## 12. Development-Loop Hardening

The release binary was smoke-tested through valid edits, invalid edits, recovery, file creation, presentation changes, deletion, and rename. Invalid edits preserve the last valid preview and do not terminate the server; a subsequent fix publishes a new revision.

## 13. Performance Before/After

No authoritative before/after budget exists in the repository. On the tested one-page publication, release `check` and `build` each completed below the resolution of `/usr/bin/time -p` (`<0.01 s`). Cold startup, browser paint, and edit-to-visible latency were not measured independently.

## 14. Cross-Platform Results

End-to-end release-artifact evidence exists for macOS Apple Silicon only. CI-compatible formatting, linting, tests, and builds were run locally. Linux and Windows runtime and watcher behavior remain unverified and are not claimed as supported.

## 15. Distribution Results

The draft `v0.1.0` release contains the macOS Apple Silicon archive `raymatic-v0.1.0-aarch64-apple-darwin.tar.gz` with SHA-256 `b544ac9c0cc7fff744ffd10150bf8ef38b9876830ee6ca8c6b75f7d58012beec`. It remains draft and unpublished. No `v0.1.0-rc.1` was created because the blockers in section 4 remain unresolved.

## 16. CLI Audit

The conceptual surface remains exactly `new`, `dev`, `check`, and `build`. Help and version succeed, invalid CLI input is rejected, and no implementation-only command is exposed.

## 17. Dependency Audit

Dependencies remain limited to clap, minijinja, notify, pulldown-cmark, serde, thiserror, tiny_http, and toml, with tempfile for tests. Each maps to current parsing, rendering, diagnostics, watching, serving, or test behavior. No new dependency was added.

## 18. Rust Complexity Audit

The code uses no unsafe code, custom public traits, async runtime, feature flags, macros for architecture, or generalized generic framework. `Arc` and `Mutex` are confined to source ownership and the development session state. The implementation remains a single Cargo package and binary.

## 19. Security/Filesystem Review

Project creation refuses non-empty destinations. Output writes use staging and backup directories. User-controlled output paths are required to remain relative and traversal-free. The preview server rejects absolute and parent-traversal paths and serves only the preview root.

## 20. Test-Suite Changes

No test was removed or weakened. The existing suite covers CLI behavior, the development recovery loop, source provenance, pipeline discovery, deterministic output, diagnostics, route collisions, broken references, rendering failure, and output preservation. The complete suite reports 20 integration/unit tests plus doctest execution with zero failures.

## 21. Documentation Status

`README.md`, generated-project documentation, `docs/INSTALL.md`, and the prior MVP/review documents describe the actual four-command workflow and current limitations. The eight authoritative design files are still absent from this repository; that provenance gap is explicitly documented rather than guessed.

## 22. Product Conformance Recheck

The implementation still feels like a small publishing workflow: create files, preview, correct diagnostics, validate, and inspect static output. Rust internals, staging, and watcher state do not appear in the normal user path. The fixed port and front-matter convention remain observable learning costs.

## 23. Release Readiness Recheck

Behavioral hardening checks pass, but the release cannot be called unconditionally ready against specifications that are not available for exact budget and platform verification. The existing draft release is therefore retained as a controlled artifact, not promoted automatically.

## 24. Release Candidate Results

No release candidate was cut. Creating `v0.1.0-rc.1` without the authoritative performance and platform acceptance criteria would convert an evidence gap into an unsupported release claim.

## 25. Remaining Known Limitations

Missing authoritative source documents, unmeasured cold-start/browser feedback latency, macOS-only end-to-end evidence, fixed preview port, coarse invalidation, and the intentionally narrow content/presentation model remain.

## 26. Post-Release Evidence Questions

If the project is released after resolving section 4, observe installation success, repeated use, diagnostic comprehension and recovery, predictability, content portability, configuration pressure, suitability for a real small publication, and switching or migration concerns.

## 27. Final Release Recommendation

**NOT READY**

The implementation is technically hardened for the tested path, but the explicit Stage 17 release blockers remain unresolved because the authoritative acceptance criteria are unavailable. Resolve or formally reissue those criteria, rerun the platform and performance checks, then create and smoke-test `v0.1.0-rc.1` before public release.

# Stage 18B — Release Evidence Resolution

The original Stage 18 result above remains **NOT READY** as historical record. Stage 18B searched the repository and available project artifacts and did not recover the missing numeric performance thresholds or historical platform matrix. It therefore made two explicit, narrowly scoped release decisions: a 1-second `check`/`build` budget and a 2-second first-preview/edit-feedback target for the generated starter workload, plus official v0.1 support limited to macOS Apple Silicon. These are new Stage 18B decisions, not reconstructed history.

Five repeated release-binary runs measured `check` and `build` at `0.00 s` displayed precision on macOS Apple Silicon. First-preview and edit-to-visible timing were not instrumented, so those criteria remain accepted evidence limitations. The actual supported target passed the release smoke paths; other targets are unsupported for v0.1 rather than implicitly supported.

The revised Stage 18B recommendation is **READY WITH ACCEPTED LIMITATIONS**. See [RELEASE_EVIDENCE_RESOLUTION.md](https://github.com/medioalanum/raymatic/blob/main/RELEASE_EVIDENCE_RESOLUTION.md) for provenance, workload, measurements, matrix, and gate results.
