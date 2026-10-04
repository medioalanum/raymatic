+++
title = "COMPATIBILITY"
+++


Raymatic 0.1.x supports the `ray new → ray dev → ray check → ray build` workflow on Linux, macOS, and Windows when built with the pinned Rust toolchain. Release archives are target-specific and contain the `ray` binary, README, and license.

The generated publication is ordinary static output. The supported source contract is Markdown with TOML front matter between `+++` delimiters, a shared MiniJinja presentation, `assets/`, and optional `site.toml`.

The Rust library is internal and is not a stable SDK. In the 0.x series, command behavior and template context may evolve between releases. Pin the Raymatic binary in CI and run `ray check` before deployment.

CI exercises Linux, macOS, and Windows. Release confidence additionally requires deterministic output from two consecutive builds and a SHA-256 archive checksum.
