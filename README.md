# Raymatic

![Raymatic — from content to a published site without incidental complexity](assets/raymatic-hero.png)

> **Live website:** [Open the official Raymatic site](https://medioalanum.github.io/raymatic/)

Raymatic is a publication-first static-site generator: it turns content and presentation intent into a deterministic static site while owning the incidental machinery around validation, preview, rendering, and safe output. Its goal is a Rails-like publishing experience with native performance and a small, explicit model.

The project is early, but its product contract is concrete: start with a useful publication, keep the authoring model visible, and make the common path require as few decisions as possible.

## Why Raymatic exists

Static HTML generation is well understood and established tools already do it well. Raymatic asks a narrower product question: can publishing a small content-oriented site require less operational machinery to understand, configure, assemble, and debug?

The intended boundary is simple: Raymatic owns incidental work such as parsing, validation, rendering, diagnostics, preview, and safe output replacement. The author owns content, presentation intent, and explicit publishing decisions. Configuration should express an intentional deviation, not be the price of admission.

## Current status

Raymatic v0.1.0 is the first public release and supported experiment. The supported release artifact targets **macOS Apple Silicon** (`aarch64-apple-darwin`). The current implementation is a Rust 1.98.1 binary named `ray`; Rust is not required to run the release artifact.

The release supports a small Markdown publication convention, structured diagnostics, a local development loop, and deterministic static output. The scope is intentionally narrow and the behavior may change during the 0.x series.

## Install

Download `raymatic-v0.1.0-aarch64-apple-darwin.tar.gz` from the [v0.1.0 release](https://github.com/medioalanum/raymatic/releases/tag/v0.1.0), verify its SHA-256 checksum, extract it, and place `ray` on your `PATH`.

The complete installation steps and checksum workflow are in [docs/INSTALL.md](docs/INSTALL.md). Other operating systems and architectures are not supported by v0.1.0.

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

The four commands are the product surface:

```text
new → dev → check → build
```

You do not need to assemble a theme, configure a plugin registry, or maintain a separate preview and production pipeline for the first publication.

The generated publication is written to `output/` only after a successful build.

## Core workflow

```text
ray new → ray dev → ray check → ray build
```

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

The v0.1 publication model does not include asset copying, custom addressing, relative-link resolution, project configuration, plugins, themes, migration, RSS, sitemap generation, image processing, or a generalized build graph. Development uses coarse full-publication invalidation and the preview uses the fixed loopback address `127.0.0.1:3000`.

These are current product boundaries, not promises about a roadmap. The [Pelican dogfooding report](DOGFOODING_01.md) records what a real publication exposed at those boundaries.

## Development from source

The repository pins Rust 1.98.1 with edition 2024. Build and verify the source with:

```sh
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build --release
```

The package is `raymatic`; the executable is `ray`. The library is internal and is not a stable SDK. There is no async runtime, plugin architecture, cache, build graph, or public extension interface.

## Documentation

- [Installation and first publication](docs/INSTALL.md)
- [Bootstrap and design traceability](docs/BOOTSTRAP.md)
- [MVP implementation status](MVP_IMPLEMENTATION_STATUS.md)
- [MVP conformance review](MVP_CONFORMANCE_REVIEW.md)
- [Release evidence resolution](RELEASE_EVIDENCE_RESOLUTION.md)
- [Dogfooding report](DOGFOODING_01.md)
- [Pelican-to-Raymatic mapping](PELICAN_RAYMATIC_MAPPING.md)

## License

Raymatic is released under the [MIT License](LICENSE).
