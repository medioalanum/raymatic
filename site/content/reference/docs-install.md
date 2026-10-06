+++
title = "INSTALL"
+++


This validation build supports **macOS on Apple Silicon** (`aarch64-apple-darwin`).
It is a standalone executable named `ray`; Rust is not required to run it.

1. Download `raymatic-v0.2.0-aarch64-apple-darwin.tar.gz` from the release.
2. Verify its SHA-256 against the release checksum.
3. Extract the archive and move `ray` to a directory on your `PATH`:

   ```sh
   tar -xzf raymatic-v0.2.0-aarch64-apple-darwin.tar.gz
   mkdir -p "$HOME/.local/bin"
   mv ray "$HOME/.local/bin/ray"
   export PATH="$HOME/.local/bin:$PATH"
   ray --help
   ```

   Add the `export PATH=...` line to your shell startup file if it is not
   already present.

To remove this validation build, delete the installed `ray` executable. Upgrade
by replacing it with a later verified release artifact.

## First publication

```sh
ray new my-publication
cd my-publication
ray dev
```

Open `http://127.0.0.1:3000`. Write pages in `content/` and edit the shared
HTML in `presentation/page.html`. Run `ray check` to validate the publication,
then `ray build`; the static site is written to `output/`.
