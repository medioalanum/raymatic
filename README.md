# Raymatic

Stage 9 bootstrap of the Rust static publishing tool. Package: `raymatic`;
executable: `ray`. This is not yet a usable static site generator.

## Development

Install Rust using rustup. The repository pins Rust 1.98.1 (edition 2024),
including rustfmt and Clippy, in `rust-toolchain.toml`. No public MSRV promise.

```sh
cargo run -- --help
cargo run -- --version
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build
```

`ray check` evaluates and validates the first publication convention. `ray build`
evaluates it and safely replaces `output/` only after a complete successful
render. `ray dev` serves the latest valid preview at `http://127.0.0.1:3000`;
`ray new [path]` remains explicitly unimplemented.

The current vertical slice uses one convention:

```text
content/**/*.md           TOML front matter between +++ delimiters
presentation/page.html    MiniJinja template with {{ title }} and {{ body }}
output/index.html         generated static HTML
```

`title` is required. Invalid front matter, a missing title, an invalid template,
route collisions, and unresolved absolute internal links are source-aware,
structured diagnostics. Diagnostics are sorted by source path, then code.
Help/version exit successfully; invalid arguments exit with status 2.

## Structure

One Cargo package, one executable, thin `main.rs`, internal `lib.rs`.
`cli` parses commands; `project` handles the filesystem boundary; `content`
contains owned semantic types and provenance; `diagnostic` contains structured
diagnostics and terminal formatting; `pipeline` owns shared evaluation;
`output` contains the output plan types. `validate`, `render`, and `dev` own
their implemented pipeline responsibilities without public abstraction layers.

No workspace, async runtime, plugin architecture, cache or build graph.
First-party unsafe code is forbidden. The library is internal, not a stable SDK.

## Scope and next slice

This milestone establishes compilation, dispatch, semantic and diagnostic data,
source position conversion, tests, CI, and a full multi-content publishing path.
It does not create projects, support assets, custom addressing, relative-link
resolution or project configuration.

`dev` watches only `content/` and `presentation/`, coalesces events for 75 ms,
rebuilds the entire small publication, and serves from `.raymatic-preview`.
An invalid edit leaves the previous preview intact; a later valid rebuild advances
the revision endpoint at `/_raymatic/revision`, which triggers browser reloads.

Next: implement `new`, then add assets and their validation to the shared pipeline.

See [bootstrap notes](docs/BOOTSTRAP.md) for design traceability and staged decisions.
