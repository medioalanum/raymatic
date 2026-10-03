# GitHub Pages

Raymatic treats hosting as a consumer of the static output. A Pages workflow only needs to build the `ray` binary, validate the publication, build `output/`, and upload that directory.

The repository's own workflow in `.github/workflows/publish-site.yml` is the reference:

1. check out the repository;
2. install the pinned Rust toolchain;
3. build `ray`;
4. run `ray check` in the publication directory;
5. run `ray build` in the publication directory;
6. upload `output/` with the Pages artifact action;
7. deploy the artifact with the Pages deployment action.

A publication repository can use the same shape. Raymatic does not need a deployment configuration file or a hosting plugin.
