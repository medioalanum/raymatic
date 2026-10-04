# Upgrading Raymatic

Before upgrading, preserve the current binary and run `ray check` and `ray build` in a clean copy of the publication. Compare `output/` and `assets-manifest.json` after installing the new version.

For 0.x upgrades:

1. read the release notes and compatibility contract;
2. run the release smoke test or equivalent clean-project check;
3. rebuild a copy of the publication;
4. review addresses, feeds, sitemap, redirects, and metadata;
5. deploy only after the generated output is accepted.

Do not depend on undocumented template variables or internal Rust APIs. Prefer the documented native fields and keep custom attributes isolated behind your own presentation.
