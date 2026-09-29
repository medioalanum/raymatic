# Product Direction Decision 02

## Status

Approved.

This decision supersedes the implementation hold in `POST_RELEASE_PRODUCT_DIRECTION.md` for the product-model capabilities explicitly named below. It does not invalidate the evidence recorded there.

## Context

Raymatic v0.1.0 proved a deliberately small publishing path and subsequent dogfooding exposed recurring pressure at the semantic boundary: publication attributes, explicit addressing, presentation selection, assets, and collection-like views over content.

The repository already contains partial semantic support for several of these concepts. `Attributes` currently contains only `title`; `Address` is derived from source paths; `Presentation` is global to the publication; and `Asset` exists as a semantic/output concept but is not discovered into the publication pipeline.

The next product objective is therefore not to add a broad feature surface. It is to complete the smallest coherent publishing model needed to represent a real blog without generator-specific workarounds while preserving Raymatic's low operational and conceptual surface.

## Decision

Proceed through small, independently justified implementation slices in this order:

1. Generalize `Attributes` only for publication intent required by the real-blog acceptance case.
2. Complete address resolution while preserving path-derived addresses as the default.
3. Add minimal presentation resolution without introducing a theme system or template hierarchy.
4. Activate assets through discovery, validation, output planning, and the existing safe output commit.
5. Introduce collections only as deterministic views over publication content when the preceding model makes their required semantics clear.
6. Validate the resulting model against a real-blog acceptance case.

Each slice must preserve the existing evaluation pipeline unless evidence demonstrates that the pipeline itself is insufficient:

`discover -> parse -> semantic resolution -> validate -> render -> plan -> commit`

## Product invariants

- The common path remains configuration-free where the product can infer intent safely.
- Configuration represents intentional deviation, not mandatory setup.
- Source filesystem paths, public addresses, and output paths remain distinct concepts.
- Content remains portable Markdown rather than Raymatic-owned content syntax.
- Diagnostics remain causal and actionable.
- Build output remains deterministic and committed safely only after a valid complete plan exists.
- New concepts must earn their place through a concrete publishing need.
- Complexity stays behind defaults until the user needs to express an intentional difference.

## Explicit non-goals

This decision does not authorize:

- plugin systems;
- theme frameworks or theme package management;
- template inheritance frameworks;
- arbitrary metadata/schema frameworks;
- query languages;
- build graphs;
- persistent caches;
- async runtimes for their own sake;
- deployment platforms or deployment automation;
- compatibility layers for Pelican, Hugo, Jekyll, or other generators;
- migration automation;
- feeds or sitemap generation unless separately justified later.

## Slice 1 gate: Attributes

The first implementation slice must evolve the existing `Attributes` type rather than replace it with an untyped metadata bag.

Before adding an attribute, the implementation must be able to answer:

1. What publication intent does the attribute preserve?
2. Why can it not be inferred reliably?
3. Does the real-blog acceptance case require it?
4. Can it remain optional without complicating first use?
5. What validation and diagnostic behavior does it require?

The slice is complete only when the new attributes flow coherently through parsing, semantic representation, validation/rendering where relevant, and tests without introducing capabilities belonging to later slices.

## Decision rule for subsequent slices

After every slice, reassess the next one against the implemented model. Do not pre-build abstractions for future slices. Prefer extending concrete semantic types over creating generalized frameworks.

## Acceptance direction

The eventual acceptance case is a real blog represented naturally in Raymatic. Success means the publication can preserve legitimate publishing intent without recreating the operational machinery of an established SSG and without adding special-case engine behavior for one source site.

## Relationship to previous direction

`POST_RELEASE_PRODUCT_DIRECTION.md` remains the evidence record that correctly concluded the initial evidence base did not yet justify immediate capability expansion. This decision records the subsequent product choice to proceed with the bounded model-completion sequence above. Where the previous document says not to implement these capabilities, this decision takes precedence.