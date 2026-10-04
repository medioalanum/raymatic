# Raymatic publication model

Raymatic starts from publication intent and derives the incidental outputs.

## Author declares

Content declares a title and may declare date, summary, category, tags, author, draft state, address, language, image, and deliberate SEO overrides. Custom TOML attributes remain available for presentation-specific extensions.

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
