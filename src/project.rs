//! Filesystem entry boundary. Project syntax is bound in the publishing slice.
use crate::AppError;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

const DEFAULT_PAGE_TEMPLATE: &str = r###"<!doctype html>
<html lang="{{ language }}">
  <head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <meta name="description" content="{{ description }}">
    <link rel="canonical" href="{{ canonical_url }}">
    <meta property="og:type" content="{{ og_type }}">
    <meta property="og:title" content="{{ og_title }}">
    <meta property="og:description" content="{{ og_description }}">
    {% if og_image %}<meta property="og:image" content="{{ og_image }}">{% endif %}
    <meta name="twitter:card" content="{{ twitter_card }}">
    <title>{{ title }} · Raymatic</title>
    <style>
      :root{color-scheme:light;font-family:Georgia,serif;color:#202020;background:#f7f5f0}
      body{margin:0;line-height:1.7}.shell{max-width:760px;margin:0 auto;padding:1.25rem}
      .site-header{display:flex;justify-content:space-between;align-items:baseline;border-bottom:1px solid #d8d2c5}
      .wordmark{color:#202020;font:700 1.4rem system-ui,sans-serif;text-decoration:none}nav a{margin-left:1rem;color:#555;font:.9rem system-ui,sans-serif}
      main{padding-top:2.5rem}h1,h2{line-height:1.2}h1{font-size:2.2rem}a{color:#9a5c00}.post-meta{color:#716b60;font:.9rem system-ui,sans-serif}
      code,pre{background:#ece8df}code{padding:.1rem .25rem}pre{padding:1rem;overflow-x:auto}footer{margin-top:5rem;border-top:1px solid #d8d2c5;color:#716b60;font:.9rem system-ui,sans-serif}
    </style>
  </head>
  <body><header class="site-header shell"><a class="wordmark" href="/">{{ site_title }}</a><nav><a href="/archive/">Archive</a><a href="/feed.xml">Feed</a></nav></header><main class="shell">{{ body }}</main><footer class="shell">Published with {{ site_title }}.</footer></body>
</html>
"###;

const DEFAULT_INDEX_TEMPLATE: &str = r###"<!doctype html>
<html lang="{{ language }}"><head><meta charset="utf-8"><meta name="description" content="{{ description }}"><link rel="canonical" href="{{ canonical_url }}"><meta property="og:type" content="{{ og_type }}"><meta property="og:title" content="{{ og_title }}"><meta property="og:description" content="{{ og_description }}"><meta name="twitter:card" content="{{ twitter_card }}"><title>{{ title }}</title></head>
<body><header class="site-header shell"><a class="wordmark" href="/">{{ site_title }}</a><nav><a href="/archive/">Archive</a><a href="/feed.xml">Feed</a>{% for link in social_links %} <a href="{{ link }}">Social</a>{% endfor %}</nav></header><main class="shell">{{ body }}{% if recent %}<section><h2>Recent</h2><ul>{% for entry in recent %}<li><a href="{{ entry.address }}">{{ entry.title }}</a>{% if entry.summary %} — {{ entry.summary }}{% endif %}</li>{% endfor %}</ul></section>{% endif %}</main><footer class="shell">Published with {{ site_title }}.</footer></body></html>
"###;

#[derive(Debug, thiserror::Error)]
pub enum EnvironmentError {
    #[error("Cannot inspect {path}: {source}")]
    Inspect {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("Expected a directory at {0}")]
    NotDirectory(PathBuf),
    #[error("A previous output commit is incomplete at {0}")]
    ExistingCommitState(PathBuf),
    #[error("Cannot create a publication in non-empty directory {0}")]
    NonEmptyTarget(PathBuf),
    #[error("Invalid site.toml: {0}")]
    InvalidConfiguration(String),
}

pub struct DiscoveredContent {
    pub path: PathBuf,
    pub relative_path: PathBuf,
}

pub struct DiscoveredAsset {
    pub path: PathBuf,
    pub relative_path: PathBuf,
}

pub struct DiscoveredSources {
    pub content: Vec<DiscoveredContent>,
    pub presentation: PathBuf,
    pub assets: Vec<DiscoveredAsset>,
    pub site: crate::content::SiteConfig,
}

pub fn discover(root: &Path) -> Result<DiscoveredSources, EnvironmentError> {
    inspect_root(root)?;
    let content_root = root.join("content");
    let mut content = Vec::new();
    discover_markdown(&content_root, &content_root, &mut content)?;
    content.sort_by(|left, right| left.relative_path.cmp(&right.relative_path));

    let assets_root = root.join("assets");
    let mut assets = Vec::new();
    if assets_root.exists() {
        discover_assets(&assets_root, &assets_root, &mut assets)?;
        assets.sort_by(|left, right| left.relative_path.cmp(&right.relative_path));
    }

    Ok(DiscoveredSources {
        content,
        presentation: root.join("presentation/page.html"),
        assets,
        site: read_site_config(root)?,
    })
}

fn read_site_config(root: &Path) -> Result<crate::content::SiteConfig, EnvironmentError> {
    let path = root.join("site.toml");
    if !path.is_file() {
        return Ok(crate::content::SiteConfig::default());
    }
    let text = read_text(&path)?;
    toml::from_str(&text).map_err(|error| EnvironmentError::InvalidConfiguration(error.to_string()))
}

fn discover_markdown(
    root: &Path,
    directory: &Path,
    output: &mut Vec<DiscoveredContent>,
) -> Result<(), EnvironmentError> {
    for entry in std::fs::read_dir(directory).map_err(|source| EnvironmentError::Inspect {
        path: directory.into(),
        source,
    })? {
        let entry = entry.map_err(|source| EnvironmentError::Inspect {
            path: directory.into(),
            source,
        })?;
        let path = entry.path();
        if entry
            .file_type()
            .map_err(|source| EnvironmentError::Inspect {
                path: path.clone(),
                source,
            })?
            .is_dir()
        {
            discover_markdown(root, &path, output)?;
        } else if path.extension().is_some_and(|extension| extension == "md") {
            output.push(DiscoveredContent {
                relative_path: path
                    .strip_prefix(root)
                    .expect("discovered under content root")
                    .into(),
                path,
            });
        }
    }
    Ok(())
}

fn discover_assets(
    root: &Path,
    directory: &Path,
    output: &mut Vec<DiscoveredAsset>,
) -> Result<(), EnvironmentError> {
    for entry in std::fs::read_dir(directory).map_err(|source| EnvironmentError::Inspect {
        path: directory.into(),
        source,
    })? {
        let entry = entry.map_err(|source| EnvironmentError::Inspect {
            path: directory.into(),
            source,
        })?;
        let path = entry.path();
        if entry
            .file_type()
            .map_err(|source| EnvironmentError::Inspect {
                path: path.clone(),
                source,
            })?
            .is_dir()
        {
            discover_assets(root, &path, output)?;
        } else {
            output.push(DiscoveredAsset {
                relative_path: path
                    .strip_prefix(root)
                    .expect("discovered under assets root")
                    .into(),
                path,
            });
        }
    }
    Ok(())
}

pub fn read_text(path: &Path) -> Result<String, EnvironmentError> {
    std::fs::read_to_string(path).map_err(|source| EnvironmentError::Inspect {
        path: path.to_owned(),
        source,
    })
}

pub fn inspect_root(root: &Path) -> Result<(), EnvironmentError> {
    let metadata = std::fs::metadata(root).map_err(|source| EnvironmentError::Inspect {
        path: root.to_owned(),
        source,
    })?;
    if !metadata.is_dir() {
        return Err(EnvironmentError::NotDirectory(root.to_owned()));
    }
    Ok(())
}

pub fn create(target: &Path) -> Result<(), AppError> {
    if target.exists() {
        inspect_root(target)?;
        if fs::read_dir(target)
            .map_err(|source| EnvironmentError::Inspect {
                path: target.to_owned(),
                source,
            })?
            .next()
            .is_some()
        {
            return Err(EnvironmentError::NonEmptyTarget(target.to_owned()).into());
        }
    }

    let staging = target.with_extension("raymatic-new");
    if staging.exists() {
        return Err(EnvironmentError::ExistingCommitState(staging).into());
    }
    fs::create_dir_all(staging.join("content/notes")).map_err(|source| {
        EnvironmentError::Inspect {
            path: staging.clone(),
            source,
        }
    })?;
    fs::create_dir_all(staging.join("assets")).map_err(|source| EnvironmentError::Inspect {
        path: staging.clone(),
        source,
    })?;
    fs::create_dir_all(staging.join("presentation")).map_err(|source| {
        EnvironmentError::Inspect {
            path: staging.clone(),
            source,
        }
    })?;
    write_initial_files(&staging)?;

    if target.exists() {
        fs::remove_dir(target).map_err(|source| EnvironmentError::Inspect {
            path: target.to_owned(),
            source,
        })?;
    }
    fs::rename(&staging, target).map_err(|source| EnvironmentError::Inspect {
        path: target.to_owned(),
        source,
    })?;
    Ok(())
}

fn write_initial_files(root: &Path) -> Result<(), EnvironmentError> {
    for (relative_path, content) in [
        (
            "content/index.md",
            "+++\ntitle = \"Welcome to Raymatic\"\nsummary = \"A small publication built with Raymatic.\"\n+++\n\n# Welcome to Raymatic\n\nThis is your publication home page. Edit this file, then run `ray dev` to see the result.\n\nRead [the first note](/notes/first-note/) to learn the basic authoring loop.\n",
        ),
        (
            "content/notes/first-note.md",
            "+++\ntitle = \"Your first note\"\nsummary = \"The smallest useful Raymatic article.\"\n+++\n\n# Your first note\n\nWrite Markdown in `content/`, keep the shared presentation in `presentation/`, and use `ray check` before `ray build`.\n\nPut files such as images and stylesheets in `assets/`; Raymatic copies them into the generated publication.\n",
        ),
        (
            "presentation/page.html",
            "<!doctype html>\n<html lang=\"{{ language }}\">\n  <head>\n    <meta charset=\"utf-8\">\n    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n    <meta name=\"description\" content=\"{{ description }}\">\n    <link rel=\"canonical\" href=\"{{ canonical_url }}\">\n    <meta property=\"og:type\" content=\"{{ og_type }}\">\n    <meta property=\"og:title\" content=\"{{ title }}\">\n    <meta property=\"og:description\" content=\"{{ description }}\">\n    {% if og_image %}<meta property=\"og:image\" content=\"{{ og_image }}\">{% endif %}\n    <meta name=\"twitter:card\" content=\"summary_large_image\">\n    <meta name=\"twitter:title\" content=\"{{ title }}\">\n    <meta name=\"twitter:description\" content=\"{{ description }}\">\n    <title>{{ title }} · Raymatic</title>\n    <style>\n      :root{color-scheme:light;font-family:system-ui,-apple-system,sans-serif;color:#202124;background:#faf9f7}\n      body{max-width:48rem;margin:0 auto;padding:4rem 1.5rem;line-height:1.7}\n      main{background:#fff;padding:clamp(2rem,7vw,5rem);border:1px solid #e8e3dc;border-radius:1rem;box-shadow:0 1rem 3rem #332b2010}\n      h1{font-size:clamp(2rem,7vw,4rem);line-height:1.05;margin:0 0 1rem;letter-spacing:-.04em}\n      p{color:#5d5a55;font-size:1.1rem}\n      code{background:#f1eee9;padding:.15em .35em;border-radius:.3em}\n    </style>\n  </head>\n  <body>\n    <main>{{ body }}</main>\n  </body>\n</html>\n",
        ),
        (
            "presentation/index.html",
            "<!doctype html>\n<html lang=\"{{ language }}\">\n  <head>\n    <meta charset=\"utf-8\">\n    <meta name=\"description\" content=\"{{ description }}\">\n    <link rel=\"canonical\" href=\"{{ canonical_url }}\">\n    <meta property=\"og:type\" content=\"{{ og_type }}\">\n    <meta property=\"og:title\" content=\"{{ og_title }}\">\n    <meta property=\"og:description\" content=\"{{ og_description }}\">\n    {% if og_image %}<meta property=\"og:image\" content=\"{{ og_image }}\">{% endif %}\n    <meta name=\"twitter:card\" content=\"{{ twitter_card }}\">\n    <title>{{ title }}</title>\n    <style>\n      :root{color-scheme:light;font-family:Georgia,serif;color:#202020;background:#f7f5f0}\n      body{margin:0;line-height:1.7}\n      .shell{max-width:760px;margin:0 auto;padding:1.25rem}\n      .site-header{display:flex;justify-content:space-between;align-items:baseline;border-bottom:1px solid #d8d2c5}\n      .wordmark{color:#202020;font:700 1.4rem system-ui,sans-serif;text-decoration:none}\n      nav a{margin-left:1rem;color:#555;font:.9rem system-ui,sans-serif}\n      main{padding-top:2.5rem}\n      h1,h2{line-height:1.2}\n      h1{font-size:2.2rem}\n      a{color:#9a5c00}\n      .post-meta{color:#716b60;font:.9rem system-ui,sans-serif}\n      code,pre{background:#ece8df}\n      code{padding:.1rem .25rem}\n      pre{padding:1rem;overflow-x:auto}\n      footer{margin-top:5rem;border-top:1px solid #d8d2c5;color:#716b60;font:.9rem system-ui,sans-serif}\n    </style>\n  </head>\n  <body><header class=\"site-header shell\"><a class=\"wordmark\" href=\"/\">Raymatic</a><nav><a href=\"/archive/\">Archive</a><a href=\"/feed.xml\">Feed</a></nav></header><main class=\"shell\">{{ body }}</main><footer class=\"shell\">Published with Raymatic.</footer></body>\n</html>\n",
        ),
        (
            "README.md",
            "# Your Raymatic publication\n\nRaymatic turns Markdown content and one shared presentation into a deterministic static site. The generated project is intentionally small: start writing, and add conventions only when you need them.\n\n## The four-command loop\n\n```sh\nray new my-publication\ncd my-publication\nray dev\nray check\nray build\n```\n\nOpen the URL printed by `ray dev` while you write. The preview keeps the last valid revision available when an edit contains an error. `ray check` validates without changing `output/`; `ray build` replaces `output/` only after the whole publication is valid.\n\n## Where things live\n\n- `content/` — Markdown pages with TOML front matter between `+++` lines.\n- `presentation/page.html` — the shared HTML presentation, using `{{ title }}` and `{{ body }}`.\n- `assets/` — static files copied to the generated site at the same path.\n- `output/` — the production site, created by a successful `ray build`.\n\nA minimal page looks like this:\n\n```markdown\n+++\ntitle = \"My first page\"\nsummary = \"A short description.\"\n+++\n\nWrite your page here.\n```\n\n## When something fails\n\nRead the diagnostic in the terminal: it identifies the source, explains the problem, and suggests the expected form. Fix the source and save it; you do not need to recreate the project or delete `output/`.\n",
        ),
        (
            "site.toml",
            "title = \"Raymatic publication\"\nauthor = \"\"\ndescription = \"A publication built with Raymatic.\"\nlanguage = \"en\"\n\n# Set base_url when deploying to a public domain.\n",
        ),
    ] {
        let path = root.join(relative_path);
        fs::write(&path, content).map_err(|source| EnvironmentError::Inspect { path, source })?;
    }
    let page_path = root.join("presentation/page.html");
    fs::write(&page_path, DEFAULT_PAGE_TEMPLATE).map_err(|source| EnvironmentError::Inspect {
        path: page_path,
        source,
    })?;
    let index_path = root.join("presentation/index.html");
    fs::write(&index_path, DEFAULT_INDEX_TEMPLATE).map_err(|source| EnvironmentError::Inspect {
        path: index_path,
        source,
    })?;
    Ok(())
}
