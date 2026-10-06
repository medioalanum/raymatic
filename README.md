# Raymatic

![Raymatic — from content to a published site without incidental complexity](assets/raymatic-hero.png)

> **Live website:** [Open the official Raymatic site](https://medioalanum.github.io/raymatic/)

Raymatic is a publication-first static-site generator: it turns content and presentation intent into a deterministic static site while owning the incidental machinery around validation, preview, rendering, and safe output. Its goal is a Rails-like publishing experience with native performance and a small, explicit model.

The project is early, but its product contract is concrete: start with a useful publication, keep the authoring model visible, and make the common path require as few decisions as possible.

## Why Raymatic exists

Static HTML generation is well understood and established tools already do it well. Raymatic asks a narrower product question: can publishing a small content-oriented site require less operational machinery to understand, configure, assemble, and debug?

The intended boundary is simple: Raymatic owns incidental work such as parsing, validation, rendering, diagnostics, preview, and safe output replacement. The author owns content, presentation intent, and explicit publishing decisions. Configuration should express an intentional deviation, not be the price of admission.

## Current status

Raymatic v0.2.0 is the current public release. The published release artifact targets **macOS Apple Silicon** (`aarch64-apple-darwin`); the source and CI are also exercised on Linux and Windows. The current implementation is a Rust 1.98.1 binary named `ray`; Rust is not required to run a release artifact.

The release supports a small Markdown publication convention, structured diagnostics, a local development loop, and deterministic static output. The scope is intentionally narrow and the behavior may change during the 0.x series.

## Install

Download `raymatic-v0.2.0-aarch64-apple-darwin.tar.gz` from the [v0.2.0 release](https://github.com/medioalanum/raymatic/releases/tag/v0.2.0), verify its SHA-256 checksum, extract it, and place `ray` on your `PATH`. To package another target from source, use `scripts/package-release.sh` as described in [docs/RELEASE.md](docs/RELEASE.md).

The complete installation steps and checksum workflow are in [docs/INSTALL.md](docs/INSTALL.md). Prebuilt release availability is narrower than source/CI support; consult [the compatibility contract](docs/COMPATIBILITY.md) before choosing a target.

## Quick start

A new publication starts with one command and one shared presentation:

```sh
ray new my-publication
cd my-publication
ray dev
```

Open <http://127.0.0.1:3000>. Write in `content/`, change the presentation in `presentation/`, and keep the preview open while Raymatic validates and rebuilds the publication. When it is ready to publish:

```sh
ray check
ray build
```

The core commands are the product surface:

```text
new → dev → check → build
```

You do not need to assemble a theme, configure a plugin registry, or maintain a separate preview and production pipeline for the first publication.

The generated publication is written to `output/` only after a successful build.

## Core workflow

```text
ray new → ray dev → ray check → ray build
```

For the documented Pelican subset, use `ray migrate inspect <source>` before `ray migrate import <source> <destination>`. Inspection never changes the source and reports review items such as plugins, themes, URL rules, and missing image alternative text; import creates an ordinary validated Raymatic project in a new or empty destination.

- `new` creates a valid starter publication.
- `dev` serves the latest valid preview, watches `content/` and `presentation/`, rebuilds the small publication, and refreshes the browser when a valid revision is available.
- `check` validates content and presentation without replacing production output.
- `build` runs the same evaluation and commits a complete static output tree safely.

The current convention is:

```text
content/**/*.md           Markdown with TOML front matter between +++ delimiters
presentation/page.html    MiniJinja template using {{ title }} and {{ body }}
output/<page>/index.html  generated static HTML
```

The `title` attribute is required. A starter project includes one content page and one presentation template so the first successful build is visible without assembling a theme or plugin system.

## Diagnostics

Expected failures are structured and source-aware. Raymatic reports what failed, where it failed, why it failed, what was expected, and a next action when that information is available. The current cases include malformed front matter, a missing title, template/rendering failure, route collision, and unresolved absolute internal references. Diagnostics are sorted by source path and code for stable output.

An invalid edit during `dev` leaves the last valid preview running. Fixing the source allows the next rebuild to recover without restarting the session.

## Scope and limitations

The current publication model includes asset copying, explicit addresses, root-relative link canonicalization, native editorial metadata (`date`, `summary`, `category`, `tags`, `author`, and `draft`), and automatic sitemap, robots, RSS, Atom, archive, category, and tag outputs. Drafts remain available to authoring and validation but are excluded from production output and publication metadata. Image dimensions and `assets-manifest.json` are emitted; actual responsive derivatives and selective cached rebuilds remain future optimizations. Development currently classifies changes while safely rebuilding the publication and tries loopback ports `3000` through `3009`.

These are current product boundaries, not promises about a roadmap. The [Pelican dogfooding report](DOGFOODING_01.md) records what a real publication exposed at those boundaries.

## Development from source

The repository pins Rust 1.98.1 with edition 2024. Build and verify the source with:

```sh
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build --release
```

The package is `raymatic`; the executable is `ray`. The library is internal and is not a stable SDK. There is no async runtime, plugin architecture, parsed-content/template cache, generalized build graph, or public extension interface.

## Documentation

- [Installation and first publication](docs/INSTALL.md)
- [Bootstrap and design traceability](docs/BOOTSTRAP.md)
- [MVP implementation status](MVP_IMPLEMENTATION_STATUS.md)
- [MVP conformance review](MVP_CONFORMANCE_REVIEW.md)
- [Release evidence resolution](RELEASE_EVIDENCE_RESOLUTION.md)
- [Dogfooding report](DOGFOODING_01.md)
- [Pelican-to-Raymatic mapping](PELICAN_RAYMATIC_MAPPING.md)
- [Performance benchmark methodology](docs/PERFORMANCE.md)
- [Release confidence checks](docs/RELEASE.md)
- [Compatibility contract](docs/COMPATIBILITY.md)
- [Upgrade guide](docs/UPGRADING.md)
- [Migration guide](docs/MIGRATION.md)
- [Migration Readiness evidence](docs/MIGRATION_READINESS.md)
- [Deployment recipes](docs/DEPLOYMENT.md)
- [Publication model](docs/PUBLICATION_MODEL.md)
- [Theming and presentations](docs/THEMING.md)
- [Project roadmap](docs/ROADMAP.md)

The release and upgrade contract is summarized in [docs/COMPATIBILITY.md](docs/COMPATIBILITY.md) and [docs/UPGRADING.md](docs/UPGRADING.md).

For a first-pass migration inventory, run `./scripts/migration-report.sh path/to/source-project`.

## What Raymatic derives for you

From a small Markdown document and its editorial metadata, Raymatic derives:

- canonical content addresses and safe staged output;
- archive, category, and tag pages;
- RSS and Atom feeds, sitemap, and robots metadata;
- Open Graph, Twitter, language, and canonical HTML metadata;
- previous/next navigation and reading-time estimates;
- copied assets with missing-file and alt-text diagnostics;
- a development preview that keeps the last valid revision after an error.

Every convention has an explicit escape hatch through front matter or a presentation template. The common path remains the core commands shown above.

## Project structure

```text
content/                 Markdown publication sources
presentation/            HTML presentations and intentional overrides
assets/                  Static files copied to the site
output/                  Complete production output from ray build
```

Generated projects include a short README and a small working publication. The repository's own documentation is built from `site/` with Raymatic in CI.

## Diagnostics and recovery

Diagnostics are stable, source-aware contracts. They identify the source path and span when available, explain the cause, state the expected form, and suggest recovery. Common failures include malformed front matter, missing titles, invalid dates, route collisions, unresolved links, missing assets, empty image alt text, and presentation errors. A failed check or build never replaces a previous complete output.

## Performance and compatibility

Run `cargo build --release && ./scripts/benchmark.sh` to measure cold build, warm build, one-file rebuild, and output size. See [the benchmark methodology](docs/PERFORMANCE.md) before comparing machines or generators. CI tests Linux, macOS, and Windows; the release artifact and supported target list remain the authoritative compatibility statement.

## Contributing

Keep changes in small vertical slices: implementation, realistic fixture or integration test, documentation, and CI evidence. Run `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test`, and `cargo build --release` before opening a pull request. Product changes should preserve the `new → dev → check → build` workflow.

## Security and support

Please report security vulnerabilities privately through the repository's security contact rather than opening a public issue with exploit details. For usage questions and reproducible bugs, open an issue with the Raymatic version, operating system, command, source fixture, and diagnostic output. Do not include secrets or private publication content.

## License

Raymatic is released under the [MIT License](LICENSE).
