+++
title = "RELEASE NOTES V0.1.0"
+++


Raymatic is a small Rust static publishing tool for turning a file-based publication into deterministic static HTML.

## Core workflow

```text
new → dev → check → build
```

- `ray new [path]` creates a starter publication.
- `ray dev` serves a local preview and watches content and presentation changes.
- `ray check` validates the publication without replacing production output.
- `ray build` writes the generated static site to `output/` after successful validation.

## What this release supports

- Markdown content with TOML front matter.
- Required page titles and deterministic HTML output.
- Source-aware diagnostics for malformed content, missing titles, template failures, route collisions, and broken internal references.
- Recovery in the development session after invalid edits.
- Safe staged replacement of generated output.

## Supported platform

v0.1.0 officially supports macOS Apple Silicon (`aarch64-apple-darwin`). Other operating systems and architectures are not supported by this release.

## Limitations

The preview uses `127.0.0.1:3000`. Development uses coarse full-publication rebuilds. Assets, custom routing, project configuration, plugins, themes, migration, RSS, sitemap, and image processing are deferred.

The performance thresholds and platform policy were explicitly established during Stage 18B because the original complete specification files were unavailable; they are release-baseline decisions, not historical claims.

## Stability

This is a 0.x release and should be treated as an early public experiment. The four-command workflow and documented behavior are the supported baseline, but broader API or behavioral stability is not promised.

## Feedback

Please report bugs, confusing behavior, documentation problems, and publishing workflows through the repository issue tracker. Include the Raymatic version, operating system, command, expected and actual behavior, relevant diagnostic, and a minimal reproduction when possible.
