# Dogfooding 01 — Rebuild of the medioalanum Pelican publication

## 1. Experiment Objective

Use the released Raymatic v0.1.0 binary to represent the real publication at [medioalanum.github.io](https://medioalanum.github.io/) without translating Pelican's architecture into Raymatic. The experiment tests publication intent, operational surface, visual fidelity, portability, and product-model pressure.

## 2. Raymatic Version

The tested binary reports `ray 0.1.0`, built from release commit `d3c4bf9`. The experiment used the normal public commands and did not use internal APIs. The source fixture is preserved in [experiments/dogfooding-01](experiments/dogfooding-01).

## 3. Pelican Baseline

The source repository is `medioalanum/medioalanum.github.io`, inspected at the shallow checkout available on 2026-09-28. The published site has three articles, a writing-stream homepage, article pages under slug directories, archive navigation, a custom theme, static images, and GitHub Pages deployment.

## 4. Existing Publication Inventory

Observable identity: a white paper-like page, Georgia body text, a large handwritten `medioalanum` wordmark, muted gray metadata, dark headings, red accent links, a centered approximately 720px reading column, generous whitespace, thin rules, responsive typography, article summaries on the homepage, and the closing “Simple on purpose.” image/caption. Navigation exposes GitHub and LinkedIn. Article URLs are `/slug/`.

The source contains three Markdown posts, `pelicanconf.py`, `publishconf.py`, `pyproject.toml`, `uv.lock`, a Git submodule for Elegant, a local `themes/medioalanum` theme, CSS, images, an SVG asset, a deployment workflow, and project documentation.

## 5. Visual/Behavioral Reproduction Contract

Equivalent means preserving the three representative articles, their titles and body semantics, slug-directory URLs where Raymatic supports them, the writing-stream concept, navigation links, article metadata, readable typography, colors, spacing, static output, and deployability. Byte-identical HTML, Pelican metadata names, plugin behavior, RSS, tags pages, archive generation, and the original GitHub Pages workflow are not required.

## 6. Pelican Operational Inventory

Pelican plus Markdown/Jinja provide parsing and rendering. `pelicanconf.py` expresses author, site name, timezone, paths, URL patterns, social links, pagination, summary behavior, and a `post_stats` plugin. `publishconf.py` supplies production URLs. `uv` and `uv.lock` manage Python dependencies. The theme submodule and local theme provide templates/CSS/assets. GitHub Actions runs the build and deploys the output to Pages.

## 7. Portable Content vs Pelican Machinery

Portable content is the article prose, titles, dates, categories, summaries, links, and image assets. Publication intent is the site name, navigation, article URL shape, readable stream, metadata display, and visual treatment. Presentation is the theme HTML/CSS. Pelican machinery is the `Title:`/`Date:`/`Slug:` header convention, `pelicanconf.py`, `publishconf.py`, plugin, theme submodule, `uv` environment, and Actions workflow. Deployment machinery is GitHub Pages configuration and the workflow.

## 8. Raymatic Implementation

The Raymatic version uses five content files (three migrated posts, a homepage, and a new project article) and one shared `presentation/page.html`. Metadata is reduced to the required `title`; dates and categories are explicit Markdown lines. The presentation is a single Raymatic template with inline CSS. No Pelican plugin, config file, submodule, Python environment, or compatibility layer was copied.

## 9. Content Migration

All three published articles were migrated into Raymatic Markdown with their prose preserved. One additional article, “Building Raymatic: Rethinking Static Publishing from First Principles,” was authored in Raymatic and built through the same path. The migration required removing Pelican's header block and retaining the meaningful metadata as visible content.

## 10. Presentation Migration

The custom theme's observable visual identity was reproduced in a compact inline stylesheet: paper background, Georgia body, system sans headings/navigation, muted metadata, red-brown links, centered reading column, rules, responsive layout, and footer. The handwritten font, exact homepage summary cards, author avatar, and closing cartoon were not reproduced because v0.1 has no asset-copy behavior.

## 11. Address/URL Migration

Raymatic generated one `index.html` per content stem under `output/<stem>/`. The migrated index links point to those generated files. Existing public `/slug/` URLs remain structurally compatible on a static server that resolves directory indexes, but the Raymatic output does not generate Pelican's `archives.html` or preserve every deployment-specific URL rule.

## 12. Asset Migration

No assets were copied into the Raymatic publication. The source includes `brand-banner.png`, `medioalanum-notes.svg`, a cartoon PNG, avatar SVG, and a font. This is a deliberate v0.1 capability gap, not an implementation defect discovered during the experiment.

## 13. Development Experience

`ray new`, editing `content/` and `presentation/page.html`, `ray check`, and `ray build` were sufficient to create and rebuild the publication. The same shared template renders the homepage and articles, so the experiment exposed a presentation limitation when a page needs different layout regions. No deployment step was attempted.

## 14. Diagnostic Evidence

Removing the homepage title produced the expected structured diagnostic: `CONTENT002`, source path and line, the missing attribute, expected `title = "A title"`, and help to add it. The invalid project failed `check` without replacing valid output. Other diagnostic cases remain covered by the existing Raymatic regression suite.

## 15. Production Build

`ray check` and `ray build` succeeded for the five-page Raymatic publication. Output contained five deterministic HTML files. A second build produced identical SHA-256 hashes for every output file. No live site was modified.

## 16. Visual Fidelity

The Raymatic version preserves the main typographic and spatial identity: paper background, serif reading text, narrow centered column, large dark headings, muted metadata, warm accent links, and responsive behavior. It differs materially in the handwritten wordmark font, image/asset presence, homepage article summaries, author treatment, and archive page. These differences are classified as capability gaps or deliberate simplifications, not hidden as parity.

## 17. Operational Comparison

Pelican requires Python/uv, locked dependencies, configuration files, a theme/submodule, plugin configuration, and GitHub Actions. Raymatic requires a binary, `content/`, one presentation template, and four commands. Raymatic removes Python environment and generator-specific deployment machinery, while the user currently carries more responsibility in the template and cannot copy assets.

## 18. Complexity Transfer Analysis

Complexity was removed from dependency installation, settings files, plugin wiring, theme submodules, and local server setup. It moved into one shared template and the need to express dates/categories manually in Markdown because v0.1 only models `title`. Asset and index behavior was not removed; it is currently unavailable. The smaller file count therefore does not imply complete parity.

## 19. What Worked

The core path worked on a real multi-article publication: ordinary Markdown remained readable, the semantic pipeline built all pages, the shared template was enough to establish the site's main visual identity, diagnostics were actionable, invalid content did not destroy output, and repeated builds were deterministic.

## 20. Friction Observed

The most significant friction was that v0.1 models only a title, while the publication meaningfully uses dates, categories, summaries, and slug behavior. The single template cannot naturally distinguish an index from an article. Assets and the archive page could not be represented. The existing article URLs are directory URLs, while Raymatic's generated files need an explicit `index.html` path in source links.

## 21. Workarounds Required

Dates and categories were written as visible Markdown lines. Homepage links were written to `slug/index.html`. The visual identity used inline CSS instead of copied theme CSS and omitted images. These workarounds are acceptable evidence for a narrow experiment but are not equivalent to a complete blog migration.

## 22. Implementation Defects

No implementation defect was found in the tested v0.1 path. The observed gaps are outside the released capability contract or represent presentation-model friction. The existing suite and dogfood build both passed.

## 23. UX/Documentation Issues

The public documentation explains the four commands and title front matter, but it does not explain how to represent dates, categories, multiple page kinds, or assets because those concepts are not in v0.1. A real publication makes that absence visible; documentation alone should not pretend those capabilities exist.

## 24. Legitimate Capability Gaps

Static asset copying, distinct index/article presentation, richer semantic metadata, and explicit address control are legitimate needs for this publication. They should be evaluated against other evidence before becoming product requirements.

## 25. Product Model Pressure

The experiment pressures the current model around attributes beyond `title`, asset representation, and page-level presentation. These appear closer to common publishing concepts than Pelican-specific machinery, but one publication is insufficient to decide their scope or shape.

## 26. Pelican-Specific Coupling

The header names, plugin `post_stats`, theme submodule, Jinja template family, `pelicanconf.py`, `publishconf.py`, `uv`, and GitHub Actions are generator/deployment machinery. The article prose and URL slugs are portable. The requirement to reproduce an archive page is partly publication intent and partly a consequence of how the current blog organizes navigation; it was not implemented in Raymatic.

## 27. Portability Findings

Raymatic content remains plain Markdown and the presentation remains ordinary HTML/CSS. If Raymatic disappeared, the articles and styles could be recovered manually. Pelican metadata is more expressive out of the box, while Raymatic's reduced metadata is easier to understand but loses structured date/category semantics.

## 28. Raymatic Article Authoring Experience

The new article was authored with the same `+++` title block and Markdown body as migrated content. `check` and `build` treated it identically to the other pages. This was predictable and easy for a single page, but it also confirmed that an article's date/category semantics are currently presentation/content conventions rather than semantic fields.

## 29. Known Limitations

The experiment does not reproduce assets, the custom font, archive/index generation, summaries as structured fields, category/tag pages, reading-time metadata, or the GitHub Pages deployment workflow. It also does not establish cross-publication requirements from one blog.

## 30. Evidence for Stage 20

Stage 20 should use this experiment as evidence that the core file-to-static pipeline is viable on real prose, while investigating whether attributes, assets, and page presentation are recurring publishing concepts. It should not infer that Pelican compatibility, a theme system, plugins, or deployment automation belong in Raymatic.

### Final Assessment

**SUCCESSFUL WITH FRICTION**

Raymatic v0.1 represented the publication's core content and recognizable visual direction with a small workflow, but meaningful workarounds were required for metadata, assets, and page-specific presentation.

### Stage 20 Handoff

#### Evidence supporting current decisions

- Four commands and a shared semantic pipeline handled five real pages.
- Plain Markdown and HTML/CSS remained portable.
- Diagnostics, safe output, and deterministic builds worked on the migrated publication.

#### Evidence challenging current decisions

- One required title field is too narrow to preserve several meaningful editorial attributes without conventions.
- One shared page template is insufficient for a homepage/article distinction without embedding layout logic in content.
- Assets are a real publication need, not only Pelican machinery.

#### Bugs requiring correction

None found.

#### UX/documentation problems

The public workflow does not explain how a real publication should model dates, categories, summaries, or asset files because v0.1 does not support them.

#### Capability requests discovered

Asset copying, structured attributes, explicit addresses, and page-kind presentation boundaries.

#### Product-model questions

Which of these concepts are common publishing intent, and which are demands imported from a particular blog's history?

#### Migration observations

Migration is straightforward for prose and basic presentation, but not lossless for metadata, assets, or generated indexes.

#### Questions still unanswered

Would multiple real publications require the same attributes and asset behavior? Is a richer model still smaller than repeated conventions? What is the smallest page/presentation boundary that preserves intent?

#### Things we should explicitly NOT change based on this experiment alone

Do not add Pelican compatibility, plugins, themes, RSS, sitemap generation, deployment automation, or a generalized build graph solely from this one migration.

### Final Question

Raymatic removed Python environments, generator settings, plugins, theme submodules, and deployment-specific build machinery. It moved some complexity into the shared template and Markdown conventions for metadata and URLs. Reality pushed against the Product Model at the boundaries of structured attributes, assets, and distinct page presentation; those pressures are evidence for Stage 20, not automatic feature mandates.
