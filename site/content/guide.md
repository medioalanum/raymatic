+++
title = "Practical guide"
+++

<div class="doc">
<div class="eyebrow">guide</div>
<h1>The complete publishing loop.</h1>

<h2 id="project">Create a project</h2>
<pre>$ ray new my-site
$ cd my-site
$ ray dev</pre>
<p>The starter project contains <code>content/</code>, <code>presentation/</code>, <code>assets/</code>, a README, and an optional <code>site.toml</code>.</p>

<h2 id="front-matter">Write content</h2>
<pre>+++
title = "My first article"
summary = "A short description."
date = "2026-10-04"
tags = ["raymatic", "publishing"]
+++

# My first article

Write Markdown here.</pre>
<p>Native fields include title, date, category, tags, author, draft, language, summary, canonical, Open Graph, and Twitter metadata. Custom TOML attributes remain available to presentations.</p>

<h2 id="site-config">Configure identity</h2>
<pre>title = "My publication"
author = "Your name"
description = "A short publication description."
language = "en"
base_url = "https://example.com"
social_links = ["https://github.com/your-name"]</pre>
<p><code>base_url</code> makes canonical URLs, sitemap URLs, and feeds absolute. Drafts are never included in public feeds, taxonomy pages, or the sitemap.</p>

<h2 id="validate-build">Validate and build</h2>
<pre>$ ray check
$ ray build</pre>
<p><code>check</code> validates without replacing production output. <code>build</code> stages a complete output tree and commits it only after evaluation succeeds.</p>

<h2 id="migration">Migrate an existing site</h2>
<pre>$ ./scripts/migration-report.sh path/to/old-site</pre>
<p>The report identifies common Pelican, Hugo, Jekyll, and Astro conventions, suggests front matter mappings, and flags permalink or slug decisions that need redirects. Migration remains editorial work: review every address before deployment.</p>

<h2 id="release">Release confidence</h2>
<pre>$ cargo fmt --check
$ cargo clippy --locked --all-targets --all-features -- -D warnings
$ cargo test --locked
$ cargo build --locked --release
$ ./scripts/verify-release.sh</pre>
<p>CI exercises Linux, macOS, and Windows. Release packaging and compatibility notes are documented in the repository's English <a href="https://github.com/medioalanum/raymatic/tree/main/docs">documentation</a>.</p>
</div>
