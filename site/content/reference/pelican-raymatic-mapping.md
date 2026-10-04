+++
title = "PELICAN RAYMATIC MAPPING"
+++


| Pelican concept | Publication intent | Raymatic representation | Migration status |
| --- | --- | --- | --- |
| `content/*.md` | Portable article prose | `content/*.md` | Direct |
| `Title` metadata | Page title | `title` TOML front matter | Simplified |
| `Date` metadata | Publication date | Visible Markdown metadata line | Workaround |
| `Category`/`Tags` | Editorial classification | Visible Markdown metadata line | Workaround |
| `Summary` | Homepage excerpt | Short body/hand-authored index text | Workaround |
| `Slug` and `ARTICLE_SAVE_AS` | Stable article address | Content stem and generated `index.html` | Different model |
| Pelican article template | Article presentation | Shared `presentation/page.html` | Simplified |
| Pelican index template | Writing-stream homepage | `content/index.md` | Simplified |
| `themes/medioalanum` CSS | Typography, color, spacing | Inline CSS in presentation | Simplified |
| Theme font, cartoon, avatar | Visual assets | Not represented | Unsupported |
| `post_stats` plugin | Reading-time metadata | Not represented | Not needed in v0.1 experiment |
| `archives.html` | Archive navigation | Not generated | Unsupported |
| `pelicanconf.py` | Site and generator settings | Content/presentation conventions | Different model |
| `publishconf.py` | Production URL settings | Static output path | Not needed in Raymatic |
| `uv` and `uv.lock` | Python environment reproducibility | Released standalone binary | Not needed in Raymatic |
| Git submodule theme | Theme distribution | One checked-in presentation file | Not needed in Raymatic |
| GitHub Actions Pages workflow | Deployment | `output/` ready for a static host | Deployment concern |
