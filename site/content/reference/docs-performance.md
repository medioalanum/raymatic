+++
title = "PERFORMANCE"
+++


Raymatic treats build speed and deterministic output as product behavior. The repository includes a small, dependency-free benchmark for the authoring loop:

```sh
cargo build --release
./scripts/benchmark.sh
```

The benchmark reports cold build, warm build, a build after changing one content file, and output file count and byte size for a generated starter plus medium (25 articles) and large (100 articles) fixtures.

The script reports observations rather than enforcing a machine-specific time limit. Compare runs on the same machine, toolchain, filesystem, and power mode. For release comparisons, record the commit, Rust toolchain, OS, CPU, and complete benchmark output.

The current implementation evaluates the complete publication on each build. This is intentional until dependency-aware invalidation is measured against realistic publications; adding a cache without evidence would make correctness and reproducibility harder to reason about.

Future benchmark fixtures should add small, medium, and large publications and compare equivalent output with Hugo, Zola, Pelican, and static Astro builds. Cross-tool comparisons must use the same content, publication outputs, and clean-machine conditions.

Use `./scripts/compare-benchmarks.sh` for an inventory-aware comparison run. It measures Raymatic and reports whether the other tools are installed; fair cross-tool numbers still require equivalent fixtures and tool-specific setup.
