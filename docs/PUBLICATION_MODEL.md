# Raymatic publication model

Raymatic starts from publication intent and derives the incidental outputs.

## Author declares

Content declares a title and may declare date, summary, category, tags, author, draft state, address, language, image, and deliberate SEO overrides. It can also declare `kind = "home"`, `kind = "page"`, or `kind = "article"`. A home is published at `/`; pages are standalone and do not enter article-derived feeds, archives, or taxonomies; articles do. Existing projects retain the path convention: `content/index.md` is home and other content defaults to articles. Custom TOML attributes remain available for presentation-specific extensions.

When a public address changes, declare legacy slash-delimited paths with `aliases`. Raymatic emits a deterministic static redirect page for every alias and rejects collisions with a canonical address or another alias:

```toml
address = "/notes/routing/"
aliases = ["/routing/", "/old-notes/routing/"]
```

An optional `site.toml` declares publication identity once:

```toml
title = "My publication"
author = "Ada"
description = "Notes on building things."
language = "en"
base_url = "https://example.test"
```

Existing projects without this file use safe defaults.

## Raymatic derives

From those declarations it derives page addresses, indexes, archive and taxonomy pages, feeds, sitemap, robots metadata, SEO values, navigation, reading time, copied assets, and a complete staged output tree.

## Override rule

Use the convention when it expresses the intended publication. Use front matter for editorial exceptions such as an explicit address or canonical URL. Use a presentation file for visual and structural exceptions. Do not repeat derived values in every content file.

The stable authoring loop is:

```text
ray new → ray dev → ray check → ray build
```
