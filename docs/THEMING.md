# Presentation and theming

`ray new` creates a complete, readable default presentation based on Raymatic's publication style: a paper-toned background, serif reading column, compact system navigation, warm links, responsive spacing, archive/feed links, and generated metadata.

## Small changes

Edit the generated `presentation/page.html` or `presentation/index.html` directly. The generated files are intentionally ordinary HTML and CSS. Keep the variables supplied by Raymatic when you want derived metadata and navigation:

```text
title, body, summary, date, category, tags, author,
canonical_url, description, og_title, og_description,
og_image, twitter_card, recent, previous, next, reading_minutes
```

## Different article presentation

Create `presentation/article.html` and select it deliberately:

```toml
+++
title = "A different kind of article"
presentation = "article"
+++
```

## Full replacement

Replace the generated presentations and assets with your own HTML, CSS, fonts, and icons. Raymatic does not require a theme package, JavaScript runtime, or plugin registry. `ray check` validates the replacement and `ray build` still owns safe output replacement.
