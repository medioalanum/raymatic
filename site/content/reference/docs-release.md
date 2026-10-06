+++
title = "RELEASE"
+++


Before a release, run:

```sh
cargo fmt --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked
cargo build --locked --release
./scripts/verify-release.sh
```

The release smoke test creates a clean project, runs `check` and `build`, verifies expected generated artifacts, and builds a deterministic output hash twice. Package archives should include the platform binary and a SHA-256 checksum. The supported target list and compatibility notes must be published with the release.

To package a locally built target:

```sh
cargo build --release --target aarch64-apple-darwin
RAY_VERSION=0.2.0 ./scripts/package-release.sh aarch64-apple-darwin
```

The corresponding compatibility and upgrade rules are documented in [COMPATIBILITY.md](https://github.com/medioalanum/raymatic/blob/main/COMPATIBILITY.md) and [UPGRADING.md](https://github.com/medioalanum/raymatic/blob/main/UPGRADING.md).
