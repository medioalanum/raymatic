+++
title = "Philosophy"
+++

<div class="doc">
<div class="eyebrow">philosophy</div>
<h1>Publishing should be the work. Not managing the publisher.</h1>
<p>Raymatic exists because static HTML generation is already solved. The question is what remains for a publisher to manage after choosing content and presentation.</p>

<h2>Own the incidental complexity</h2>
<p>Parsing, validation, rendering, preview, actionable diagnostics, and safe output replacement are routine publishing machinery. Raymatic takes responsibility for those parts in the common path.</p>

<h2>Keep intent with the author</h2>
<p>The author owns content, presentation intent, and explicit deviations from sensible defaults. Configuration should express an intentional publishing decision, not be the price of admission.</p>

<h2>Small operational surface</h2>
<p>The release has four conceptual commands:</p>
<pre>ray new
ray dev
ray check
ray build</pre>
<p>They correspond to creating, previewing, validating, and producing a static publication. The product does not require a plugin graph, theme marketplace, build cache, or deployment service for this path.</p>

<h2>Explainability is part of the product</h2>
<p>A failed publication should help its author move from failure to cause, location, expectation, and remediation. This is why Raymatic keeps structured diagnostics internally and formats them for the CLI at the boundary.</p>

<h2>Boundaries are evidence</h2>
<p>v0.1 deliberately omits assets, richer metadata, custom addressing, and other established SSG capabilities. Real usage may show that a missing concept is a legitimate publishing need. That evidence should shape the product; it should not be hidden behind accidental complexity.</p>
</div>
