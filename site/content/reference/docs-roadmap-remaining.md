+++
title = "ROADMAP REMAINING"
+++


The core convention-first publishing path is implemented and dogfooded. The remaining work is ordered by product value:

1. finish image processing: actual responsive variants, thumbnails, and stronger social-image conventions (dimensions and `assets-manifest.json` are now emitted);
2. complete richer default social-link labels (sitemap, RSS, Atom, and taxonomy RSS URLs now honor `base_url`);
3. replace the current per-file source snapshot with parsed-content/template caches (dependency classification is explicit, but evaluation remains full-build);
4. add automatic migration transformations beyond the advisory front-matter and URL/redirect review;
5. run installed-generator comparisons with equivalent populated fixtures (the comparison script now remains the reproducible entry point);
6. complete broader cross-platform filesystem fixtures and publish release archives from CI;
7. validate the public behavior contract against more external publications and versioned golden fixtures.

These are product gaps, not prerequisites for creating a conventional blog today. Each should remain a vertical slice with implementation, fixture, diagnostic coverage, documentation, CI evidence, and measured impact.
