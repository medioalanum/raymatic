+++
title = "Static publishing without managing an SSG"
+++

<section class="hero">
  <div class="eyebrow">/ raymatic</div>
  <h1>Static publishing without managing an SSG.</h1>
  <p class="lede">Write content. Raymatic handles the publishing machinery.</p>
  <div class="actions"><a class="button primary" href="docs/index.html">Get started</a><a class="button" href="docs/index.html#writing-content">Documentation</a><a class="button" href="https://github.com/medioalanum/raymatic">GitHub ↗</a></div>
</section>

<section>
  <div class="eyebrow">create → write → preview → publish</div>
  <h2>A new publication is a small set of files. Nothing else to wire together.</h2>
  <div class="terminal"><div class="terminal-bar"><span class="dot"></span><span class="dot"></span><span class="dot"></span></div><pre>$ ray new my-site
Created publication at ./my-site

$ cd my-site
$ ray dev
Preview available at http://127.0.0.1:3000</pre></div>
  <div class="grid" style="margin-top:18px"><div class="card"><h3>my-site/</h3><pre class="tree"><span>content/</span>
<span>presentation/</span>
README.md</pre></div><div class="card"><h3>A page is a Markdown file.</h3><p>Use the current front matter convention and keep the content understandable outside Raymatic.</p><pre>+++
title = "Hello, world"
+++

# Hello, world</pre></div></div>
</section>

<section>
  <div class="eyebrow">philosophy</div>
  <h2>Publishing should be the work.<br>Not managing the publisher.</h2>
  <p class="section-copy">Static publishing tools often expose configuration, routing, integration, and build machinery that rarely deserves constant attention. Raymatic takes responsibility for routine operational complexity and leaves intentional publishing decisions to you.</p>
  <a class="button" href="philosophy/index.html">Read the full philosophy →</a>
</section>

<section>
  <div class="eyebrow">explainability</div>
  <h2>A failure should teach you something.</h2>
  <p class="section-copy">Diagnostics are part of the product, not raw compiler output. Every supported problem reports what failed, where, why, what was expected, and what to do next when that information is available.</p>
  <div class="card diagnostic"><strong>error[CONTENT002]: Missing required attribute: title</strong><pre>  --> content/hello-world.md:1:1
  expected: title = "A title"
  help: Add a title to the front matter.</pre></div>
</section>

<section>
  <div class="eyebrow">command surface</div>
  <h2>Four commands. The whole lifecycle.</h2>
  <div class="commands"><div class="command"><code>$ ray new</code><p>Creates a valid publication.</p></div><div class="command"><code>$ ray dev</code><p>Local preview, watching, and feedback.</p></div><div class="command"><code>$ ray check</code><p>Validates the publication.</p></div><div class="command"><code>$ ray build</code><p>Produces publishable static output.</p></div></div>
</section>

<section>
  <div class="eyebrow">principles</div>
  <div class="grid"><div class="card"><span class="number">01</span><h3>Strong defaults</h3><p>Routine publishing decisions should not require configuration.</p></div><div class="card"><span class="number">02</span><h3>Explainable behavior</h3><p>When something fails or changes, Raymatic helps explain why.</p></div><div class="card"><span class="number">03</span><h3>Portable content</h3><p>Content stays understandable outside Raymatic.</p></div><div class="card"><span class="number">04</span><h3>Intentional customization</h3><p>Customize when a publication genuinely needs to differ from its defaults.</p></div><div class="card"><span class="number">05</span><h3>Performance without ceremony</h3><p>Performance removes waiting and complexity, not just benchmark numbers.</p></div></div>
</section>

<section style="text-align:center"><div class="eyebrow">start publishing</div><h2>Make the publication the thing you manage.</h2><div class="actions" style="margin-top:28px"><a class="button primary" href="docs/index.html">Read the documentation</a><a class="button" href="https://github.com/medioalanum/raymatic">View on GitHub ↗</a></div></section>
