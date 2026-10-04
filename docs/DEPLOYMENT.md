# Deployment recipes

All recipes deploy the complete `output/` directory produced by `ray build`. The hosting provider is outside Raymatic's core binary.

## GitHub Pages

Build in a workflow and publish `output/` with the official Pages action. A minimal sequence is:

```sh
cargo build --release
ray build
```

Configure Pages to use the workflow artifact or the branch directory containing `output/`. Keep the generated sitemap and feeds at the site root.

## Netlify

Set the build command to `ray build` and the publish directory to `output`. Install or download the Raymatic binary in the build image, and fail the deployment if `ray check` fails.

## Cloudflare Pages

Use `ray build` as the build command and `output` as the output directory. No client-side runtime is required for ordinary Raymatic pages.

## Generic static hosting

Run `ray check`, then `ray build`, and upload the contents of `output/` to the provider's document root. Never upload `.raymatic-preview/`, source content, or presentation files as the production site.

## Release safety

- deploy only after a successful complete build;
- preserve the previous hosting artifact when validation fails;
- verify `index.html`, `sitemap.xml`, `robots.txt`, `feed.xml`, and `atom.xml` after upload;
- configure the public site URL at the hosting layer when absolute canonical URLs are required.
