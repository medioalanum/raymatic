# MVP release readiness

## Decision

**Not ready for external product validation.** A real macOS/Apple Silicon
validation artifact is prepared and its golden path passes, but this repository
does not contain the authoritative technical performance budgets and has not
verified every intended supported platform. The Stage 15–16 gate therefore
cannot honestly be passed yet.

This is a readiness finding, not a request for feature expansion.

## Validation artifact

| Item | Value |
| --- | --- |
| Version | `v0.1.0` |
| Binary | `ray` |
| Invocation | `ray new`, `ray dev`, `ray check`, `ray build` |
| Tested release target | `aarch64-apple-darwin` |
| Archive | `raymatic-v0.1.0-aarch64-apple-darwin.tar.gz` |
| SHA-256 | `b544ac9c0cc7fff744ffd10150bf8ef38b9876830ee6ca8c6b75f7d58012beec` |
| GitHub distribution | [Draft release v0.1.0](https://github.com/medioalanum/raymatic/releases/tag/untagged-6a5ec93983abcf4bb492); not published. |
| Installation | Manual archive extraction and `PATH` placement; see `docs/INSTALL.md`. |
| Project structure | `content/`, `presentation/page.html`, `README.md`; build writes `output/`. |

The archive contains the release executable only. It represents the actual MVP;
there is no validation-only code path or reduced command surface.

## Supported platforms

Only **macOS on Apple Silicon** is supported for the first external validation
artifact, because it is the only release target tested end to end. Linux and
Windows CI exercise source checks, but no release artifact, watcher, server, or
installation workflow was verified there; they are not supported claims.

## Installation and first use

The manual installation procedure is in `docs/INSTALL.md`. A user can install
the binary, run `ray --help`, create a publication, start the preview, check it,
and build it without Rust or project-team intervention on the tested target.

The generated project README provides the workflow after `ray new`. Its user
concepts are limited to content, presentation, preview, validation, and output.

## CLI readiness

The release executable was verified for root help, help for all four commands,
and an unknown command. Help and version return success; an unknown command
returns exit code 2 with Clap's usage guidance. Publication and diagnostic
failures return exit code 1 and use stderr. `check` does not write production
output; `build` does only after a complete successful evaluation.

## Reliability and golden-path results

On the tested release artifact:

- `ray new` created a valid project.
- `ray check` and `ray build` succeeded and produced `output/index.html`.
- `ray dev` served the page at `127.0.0.1:3000`.
- A valid edit changed the preview revision from 1 to 2.
- A missing-title edit retained revision 2 and printed `CONTENT002` with source,
  cause, expected state, and remediation.
- Fixing the content advanced the revision to 3 and served the recovered page.
- Existing tests cover deterministic plans and diagnostic ordering, output
  preservation after failed validation, rendering failures, collisions, and
  broken internal references.

## Performance readiness

Using the optimized release executable and a generated one-page publication on
the local macOS/Apple Silicon host, `check` and `build` each measured below 0.01
seconds at `/usr/bin/time -p` resolution. The dev sequence above confirmed the
watch/rebuild/reload mechanism but did not measure browser paint timing.

No classification against the product's cold-start, first-preview, edit-to-
visible, check, or build budgets is possible: `TECHNICAL_REQUIREMENTS.md` is not
present in this repository. This is a validation blocker, not evidence of a
performance failure.

## Readiness classification

| Classification | Issue | Resolution |
| --- | --- | --- |
| Validation blocker | Product-derived performance budgets are unavailable for comparison. | Import `TECHNICAL_REQUIREMENTS.md`, measure the named budgets, and classify each result. |
| Validation blocker | Intended supported platforms are not defined and only macOS/Apple Silicon has end-to-end artifact coverage. | Import the authoritative platform decision or explicitly constrain the validation cohort to the tested target. |
| Known limitation | Fixed local address `127.0.0.1:3000`; a busy port prevents `dev`. | Documented; do not add a configuration surface during this stage. |
| Known limitation | Manual archive installation requires a `PATH` change. | Documented and uninstallable by deleting one executable. |
| Acceptable MVP constraint | Fixed content and presentation convention. | The generated project and README expose the convention directly. |
| Deferred capability | Package-manager integrations and multi-platform release automation. | Not required before validating the tested artifact with the appropriate cohort. |

## Blockers fixed in this stage

None. The release work found process and evidence blockers, rather than defects
that can be truthfully fixed without the missing authoritative inputs or testing
environments.
