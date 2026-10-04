+++
title = "IMPLEMENTATION PLAN V2"
+++


## Purpose

This plan continues Raymatic from the current merged implementation. Its goal
is to make Raymatic a coherent static publishing product that can replace the
Pelican workflow for a small technical publication while preserving Raymatic's
own concepts and boundaries.

Replacement means that the author's publication can move to Raymatic with a
smaller and more explainable publishing system. It does not mean preserving
Pelican's front matter, theme hierarchy, plugin model, configuration files,
taxonomy machinery, or generated HTML structure.

## Decisions carried forward

Raymatic's product thesis is low operational surface plus high explanatory
power. The product owns incidental publishing complexity; the author owns
content, presentation intent, and explicit publishing decisions.

The common lifecycle remains exactly:

```text
ray new → ray dev → ray check → ray build
```

The user-facing concept budget remains:

```text
Publication · Content · Attributes · Address · Presentation · Asset
```

`Collection` is a progressive view over resolved content, not a query engine.
`Override` is a progressive escape path, not a general configuration system.

The current implementation already provides the shared semantic pipeline,
structured diagnostics, derived and explicit addresses, internal references,
conventional assets, safe output replacement, and a development preview. The
next work enriches that publication representation instead of adding parallel
subsystems.

## Definition of done

Raymatic is ready to replace the current Pelican workflow when a developer can:

1. create a publication with `ray new`;
2. add ordinary articles with portable Markdown and a small set of structured
   attributes;
3. obtain stable article addresses without configuring a routing system;
4. express a publication entry page that observes an ordered set of content;
5. choose or override presentation intentionally when the default is
   insufficient;
6. publish ordinary assets byte-for-byte;
7. understand validation failures from Raymatic's own diagnostics;
8. run the same four commands locally and in a clean GitHub Pages build;
9. migrate the author's publication intent without importing Pelican's
   architecture.

The output need not be byte-identical to Pelican. The acceptance question is
whether the resulting publication is coherent, recognizable, portable, and
maintainable with less machinery.

## Slices

### Slice 8 — Structured editorial attributes

Keep `title` required. Add the smallest useful set of typed attributes needed
by a content-oriented publication: date, summary, and optional scalar or list
values. Preserve source provenance for each attribute so invalid values and
conflicting decisions can be explained.

Dates must be suitable for deterministic ordering. Unknown attributes remain
portable data available to presentation; they do not automatically become
special core concepts. No schema language, validation framework, or Pelican
header compatibility is introduced.

Acceptance: an article can carry title, date, summary and a custom editorial
value; the body remains ordinary Markdown; malformed or ambiguous values
produce source-aware diagnostics; old title-only content remains valid.

### Slice 9 — Presentation resolution by convention

Define Raymatic's own presentation convention. The ordinary content path uses
the default article presentation. The publication entry content receives the
publication presentation by a deterministic convention. An explicit
presentation choice is an intentional override and must fail clearly when its
target does not exist; an absent optional convention falls back predictably.

Do not introduce a theme system, layout inheritance hierarchy, or page-kind
framework. The result should make an article and a publication entry expressible
without asking a first-use author to assemble templates.

Acceptance: a default publication can render an entry and articles with the
appropriate presentation; explicit selection has provenance and actionable
failure diagnostics.

### Slice 10 — Minimal publication view

Expose a deterministic ordered set of resolved content to the publication
presentation. The initial view supports only the operation the product needs:
recent content ordered by the known date attribute, with a deterministic
fallback for undated content.

This is a view over `Vec<Content>`, not a collection subsystem. Do not add
filters, joins, arbitrary predicates, taxonomies, pagination language, or a
query DSL.

Acceptance: the entry page can list recent articles with resolved addresses,
titles and summaries; ordering is stable; an empty or undated publication is
explained rather than silently producing surprising output.

### Slice 11 — Raymatic-native real-publication acceptance

Create a fixture derived from the author's blog, using Raymatic conventions:
portable Markdown, the minimal attributes, a small presentation set, and
conventional assets. Compare publication intent and user workflow, not Pelican's
file layout or HTML.

The fixture must cover the existing blog's article URLs, representative prose,
visual assets, homepage listing, internal links, and one deliberate invalid
edit. It should prove that the four-command workflow remains small and that
diagnostics lead to recovery.

This slice may reveal a missing Raymatic concept. It must not turn every
observed Pelican feature into a requirement. A need earns implementation only
when it is publication intent that Raymatic cannot express coherently.

### Slice 12 — Publish integration for a static host

Document and validate the smallest GitHub Pages workflow for a Raymatic
publication. The workflow should build the site and upload the resulting static
output without introducing a deployment subsystem into Raymatic itself.

The tool remains responsible for `check` and `build`; the hosting workflow
remains a small consumer of the output. Cross-platform release availability or
an explicit clean-build path must be resolved before recommending the migration
for the author's repository.

### Slice 13 — First-use and migration hardening

Use the real-publication fixture and one unfamiliar-user walkthrough to verify
that the generated project, README and diagnostics teach the Raymatic model
without an architecture manual. Add only documentation or small defaults that
remove demonstrated friction.

The exit criterion is not feature completeness. It is that the author can
maintain the publication through `new`, file edits, `dev`, `check`, and `build`
without carrying Pelican concepts in their head.

## Explicit exclusions

This plan does not add Pelican compatibility, a theme marketplace, plugins,
hooks, arbitrary configuration, a routing DSL, a taxonomy engine, a collection
query language, RSS, sitemap generation, image processing, asset transforms,
multilingual support, remote content, a persistent cache, a dependency graph,
or a general web-application framework.

Archives, categories, tags, and reading time are not automatic requirements.
They can enter a later slice only if the real-publication acceptance shows that
the underlying publication intent cannot be expressed by the existing concepts
and minimal views.

## Slice discipline

Each slice must start from the current `main`, preserve the four-command UX,
end in a functioning vertical behavior, include focused tests and diagnostics,
run formatting, Clippy, tests, build and CLI checks, and pass CI before merge.

If a proposed change introduces a new user-facing concept, first state which
acceptance criterion requires it and why inference, convention or an existing
concept cannot solve the problem. If that answer is unclear, defer the change.

## Current next step

Slice 8 — Structured editorial attributes is implemented in the current
working slice. The next implementation slice is Slice 9 — Presentation
resolution by convention. It should make a publication entry and ordinary
content render through Raymatic conventions without introducing a theme system
or a general page-kind framework.
