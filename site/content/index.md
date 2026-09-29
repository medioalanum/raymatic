+++
title = "Static publishing without managing an SSG"
+++

<section class="hero" aria-labelledby="hero-title"><div class="eyebrow">/ raymatic</div><h1 id="hero-title">Raymatic</h1><p class="sub">Static publishing without managing an SSG.</p><p class="desc">Write content.<br>Raymatic handles the publishing machinery.</p><div class="actions"><a class="button primary" href="docs/index.html">Get started →</a><a class="button" href="docs/index.html#writing-content">Documentation</a><a href="https://github.com/medioalanum/raymatic">GitHub ↗</a></div></section>

<section aria-label="Product demonstration"><p class="kicker">01 — Create &amp; preview</p><div class="terminal"><div class="bar">~/projects</div><pre>$ ray new my-site
Created publication at ./my-site

$ cd my-site
$ ray dev
Preview available at http://127.0.0.1:3000
Watching for changes…</pre></div><div class="grid"><div><p class="kicker">02 — A valid publication</p><div class="code"><pre>my-site/
├── content/
├── presentation/
└── README.md</pre></div></div><div><p class="kicker">03 — Write content</p><div class="code"><div class="bar">content/hello-world.md</div><pre>+++
title = "Hello, world"
+++

# Hello, world

My first Raymatic publication.</pre></div></div></div><div style="margin-top:20px"><p class="kicker">04 — Publish</p><div class="code"><pre>$ ray check
$ ray build
Static output written to output/</pre></div></div></section>

<section id="philosophy"><p class="kicker">Philosophy</p><h2>Publishing should be the work. Not managing the publisher.</h2><p class="muted">Traditional static publishing exposes machinery that should not need your attention: tool selection, plugin integration, configuration, build debugging, maintenance. The ceremony becomes the project.</p><p class="muted">Raymatic assumes responsibility for that routine operational complexity and leaves the intentional publishing decisions to you. Low operational surface, high explanatory power.</p><div class="flow" style="margin-top:24px"><pre>tool selection → integration → configuration → debugging → maintenance → publication
content <span style="color:var(--link)">→</span> presentation <span style="color:var(--link)">→</span> publication</pre></div></section>

<section><p class="kicker">Explainability</p><h2>When something breaks, Raymatic explains itself.</h2><p class="muted">A diagnostic should read like a colleague pointing at the exact line — what failed, where, why, what was expected, and what to do next.</p><div class="diagnostic" style="margin-top:24px"><strong>error[CONTENT002]: Missing required attribute: title</strong><pre>  --> content/hello-world.md:1:1
  expected: title = "A title"
  help: Add a title to the front matter.</pre></div></section>

<section><p class="kicker">Command surface</p><h2>Four commands. The whole lifecycle.</h2><div class="command-grid"><div class="command"><code>$ ray new</code><p>Create a valid publication.</p></div><div class="command"><code>$ ray dev</code><p>Local preview, watching and feedback.</p></div><div class="command"><code>$ ray check</code><p>Validate the publication.</p></div><div class="command"><code>$ ray build</code><p>Produce publishable static output.</p></div></div></section>

<section><p class="kicker">Principles</p><div class="principles"><div class="principle"><span class="num">01</span><h3>Strong defaults</h3><p>Routine publishing decisions should not require configuration.</p></div><div class="principle"><span class="num">02</span><h3>Explainable behavior</h3><p>When something fails or changes, Raymatic tells you what, where and why.</p></div><div class="principle"><span class="num">03</span><h3>Portable content</h3><p>Your writing stays plain files — readable and diffable outside Raymatic.</p></div><div class="principle"><span class="num">04</span><h3>Intentional customization</h3><p>Configuration is a deliberate deviation from defaults.</p></div><div class="principle"><span class="num">05</span><h3>Performance without ceremony</h3><p>Speed removes waiting and complexity, not just benchmarks.</p></div></div></section>

<section style="padding-bottom:35px"><p class="kicker">Start publishing</p><h2>Make the publication the thing you manage.</h2><div class="actions"><a class="button primary" href="docs/index.html">Read the documentation →</a><a class="button" href="https://github.com/medioalanum/raymatic">View on GitHub ↗</a></div></section>
