+++
title = "RELEASE EVIDENCE RESOLUTION"
+++


## 1. Original Release Blockers

Stage 18 concluded **NOT READY** because the repository did not contain authoritative performance budgets or an official supported-platform matrix.

## 2. Repository Baseline (`8139c82`)

The evaluated implementation is the unchanged baseline at commit `8139c82`. Stage 18B made no product or implementation changes.

## 3. Performance Requirement Recovery

The repository, attached project artifacts, and downstream reports were searched for cold startup, first preview, edit-to-visible, `check`, `build`, representative workload, and measurement thresholds. Exact historical thresholds were not recoverable. The available material references these metrics but does not define numeric budgets.

## 4. Performance Contract Provenance

The following is a **new Stage 18B decision**, not a recovered historical requirement: for the MVP representative workload, each command path (`check` and `build`) must complete within 1 second, and the development loop must provide a first preview and edit-to-visible feedback within 2 seconds on the supported target. These limits represent the product requirement that small publications feel immediate; they are acceptance criteria for this release baseline, not claims about the original documents.

## 5. Representative Workloads

The measured publication is the generated starter project: one Markdown document, one TOML front matter block, one presentation template, no assets, and the default output plan. This is the smallest complete golden-path publication currently defined by the MVP. Measurements used the release binary on macOS Apple Silicon.

## 6. Performance Measurements

Five repeated `check` and `build` runs were measured with `/usr/bin/time -p`. Every observed `real` value was `0.00 s` at the timer's displayed precision. Cold startup and browser paint were not separately instrumented; edit-to-visible was validated behaviorally in the development smoke test but not measured with a monotonic timestamp.

## 7. Performance Gate

`check`: **PASS** against the new 1 s budget. `build`: **PASS** against the new 1 s budget. First preview and edit-to-visible: **NOT MEASURABLE** with the current instrumentation, so the 2 s criterion remains an accepted evidence limitation rather than a claimed pass.

## 8. Platform Requirement Recovery

No authoritative platform matrix was found in the repository, attached project artifacts, or release reports. The prior evidence consistently identified macOS Apple Silicon as the only end-to-end tested target, but did not establish a formal historical support promise.

## 9. Platform Contract Provenance

The v0.1 platform policy below is a **new Stage 18B decision**. It narrows claims to observed evidence and does not infer support from Rust compilation capability.

## 10. Official v0.1 Support Matrix

| Target | Status | Testing expectation | Artifact |
| --- | --- | --- | --- |
| macOS Apple Silicon (`aarch64-apple-darwin`) | Supported | End-to-end release smoke tests | `raymatic-v0.1.0-aarch64-apple-darwin.tar.gz` |
| Linux, any architecture | Unsupported for v0.1 | No support commitment | None |
| Windows, any architecture | Unsupported for v0.1 | No support commitment | None |
| macOS Intel | Unsupported for v0.1 | No support commitment | None |

The rationale is the smallest truthful distribution contract: one target has actual artifact and runtime evidence; the others do not.

## 11. Platform Verification Results

The supported macOS Apple Silicon release binary passed `--version`, `--help`, `new`, `check`, `build`, development serving, file watching, invalid-edit diagnostics, recovery, Unicode paths, generated output, and expected exit behavior. Unsupported targets are explicitly not tested and are not claimed.

## 12. Implementation Changes, if any

None. No defect was exposed by the evidence work.

## 13. Repository Documentation Changes

This record establishes the new performance and platform decisions. `RELEASE_HARDENING_REPORT.md` receives an addendum preserving its original NOT READY result and recording this resolution.

## 14. Quality-Gate Results

Using the pinned Rust 1.98.1 toolchain with absolute tool paths: `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test`, and `cargo build --release` all passed. The test suite reports 20 integration/unit tests plus successful doctest execution.

## 15. Remaining Known Limitations

The historical performance contract was not recovered; the numeric budget is newly established. First-preview and edit-to-visible latency lack timestamp instrumentation. Platform support is intentionally limited to macOS Apple Silicon.

## 16. Remaining Release Blockers

No blocker remains for a controlled v0.1 release under the newly explicit contract. The release must disclose that the performance thresholds and support matrix were established during Stage 18B, not recovered from the missing historical documents.

## 17. Revised Release Recommendation

**READY WITH ACCEPTED LIMITATIONS**

Performance and platform claims are now explicit, bounded, and supported by the available evidence. Proceed to Stage 19 using the unchanged implementation and the macOS Apple Silicon-only support claim.

## Final Question

Yes, the two evidence gaps are resolved strongly enough for truthful v0.1 claims, with the provenance and measurement limitations stated above. The criteria are explicit Stage 18B decisions, not reconstructed historical requirements.
