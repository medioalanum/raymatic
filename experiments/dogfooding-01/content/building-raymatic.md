+++
title = "Building Raymatic: Rethinking Static Publishing from First Principles"
+++

*2026-09-28 · Raymatic*

Raymatic started with a question about the static-site-generator category: how much of publishing work is actually the author's intent, and how much is machinery inherited from the tools around it?

The project contract defines a small path from files to a deployable static publication. The MVP narrows that path to four commands: `new`, `dev`, `check`, and `build`. The architecture keeps parsing, semantic representation, validation, rendering, and output commit as explicit boundaries so that an error can still explain where it came from.

The first implementation was deliberately modest. Markdown content uses a TOML front matter block, a shared presentation template renders HTML, and a staged output commit protects the last valid publication when a source edit is invalid. The development loop rebuilds the small publication as a unit instead of introducing a cache or dependency graph before evidence requires one.

The release process exposed an important distinction. A passing build can establish implementation evidence, but it cannot validate the product thesis. Raymatic v0.1.0 therefore shipped as a bounded experiment, with macOS Apple Silicon as its supported target and the unsupported capabilities documented instead of hidden.

This Pelican dogfooding exercise is the next test. The existing blog is a real publication with its own visual identity, content, assets, URLs, and deployment workflow. Rebuilding a representative version with Raymatic shows which publishing machinery disappears, which complexity moves into content or presentation, and where the current product model meets a legitimate need it does not yet express.

The result should be judged by those observations, not by whether Raymatic reproduces every feature of Pelican. A small publishing tool earns its shape by making the author's common path clear and predictable.
