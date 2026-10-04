+++
title = "PRE RELEASE PRODUCT REVIEW"
+++


## 1. Executive Decision

**PROCEED TO HARDENING**

The MVP remains coherent with the intended product boundary, no known evidence
invalidates the thesis strongly enough to stop, and the release artifact is
bounded enough that real-world usage should produce useful evidence. This is an
internal release decision, not product validation. No external behavioral
evidence exists yet.

## 2. Original Product Thesis

The project intended to serve developers who maintain small, content-oriented
technical publications and value files, version control, static output, and low
operational overhead. The job is to move from content and presentation changes to
a preview, correction, and publishable static site without making the user manage
incidental publishing machinery.

The core value proposition is strong defaults plus high explanatory power: the
product owns incidental complexity while the user owns publishing intent. The
intended differentiation is **low operational surface + high explanatory power**.
The invariants are a small common path, intent-shaped configuration, product-owned
complexity, explanatory errors, friction-removing performance, portable content,
non-mandatory extensions, and complexity earned by a current requirement.

The non-goals are ecosystem-scale features such as plugins, themes, taxonomies,
RSS, sitemap generation, image processing, migration automation, remote data,
and a generalized build system. The golden path is create → understand → add
content → develop → edit → recover from an error → check → build → inspect output.

The exact Product Contract is not present in this repository; this reconstruction
uses the project summaries and prior status documents. Exact contract wording and
failure criteria remain an evidence/documentation limitation.

## 3. MVP Experiment

The MVP was intended to prove technical feasibility of the semantic pipeline and
to create a small, usable experiment: source content → parse → semantic model
with provenance → validate → render → output plan → safe static output. It also
tests the four-command surface and the diagnostic/recovery boundary.

It was not intended to prove market demand, adoption, migration value,
maintenance cost over time, competitive superiority, willingness to pay, or the
truth of the Product Contract. Internal tests prove behavior; they do not prove
that the behavior matters to users.

## 4. Implementation Traceability

| Implemented capability | Product/architecture reason | Match | UX leakage |
| --- | --- | --- | --- |
| `new` starter project | Golden path and strong defaults | Matches the generated structure and safe non-empty-target behavior | None observed; README explains only user-owned locations. |
| Markdown plus TOML front matter | Content model and required title validation | Matches the current convention | Front matter syntax is a convention users must learn; its necessity is not yet user-tested. |
| Semantic `Publication`/`Content`/`Address`/`Presentation` | Shared semantic pipeline and provenance | Matches architecture | Concepts remain internal during normal use. |
| `check` | Explicit validation before production output | Matches | No implementation machinery exposed. |
| `build` with output plan and staging | Output integrity invariant | Matches | `output/` is a clear user-visible result. |
| `dev` watcher/server/reload | Local edit-preview loop with coarse invalidation | Matches | Fixed port and polling reload are current constraints. |
| Structured diagnostics | Explainability and source provenance | Matches supported failure cases | No raw Rust error is primary for expected content errors. |
| `Asset` and copy-file output variants | Conceptual model representation | Not exercised by the MVP | Potential orphan complexity; defer judgment until authoritative design is available. |

No extra command, configuration file, plugin boundary, build graph, cache, async
runtime, or public extension interface was introduced. No missing required
golden-path capability is known from the available documents.

## 5. Product Drift Audit

| Finding | Classification | Assessment |
| --- | --- | --- |
| Fixed `127.0.0.1:3000` server address | Known constraint | Acceptable for a first controlled release; disclose it. |
| TOML `+++` convention | UX concern to observe | It is a deliberate default, but the learning cost is unknown. |
| README and generated README | Harmless internal/product support | Explains the minimum independent workflow without teaching architecture. |
| Asset representation without asset behavior | Harmless internal detail / possible orphan | Not exposed; no release action yet. |
| Rust types and synchronization | Harmless internal detail | No Rust concept appears in the CLI workflow. |
| No configuration system | Product boundary | Preserves low operational surface; may reveal missing legitimate intent after release. |

No product-contract violation or release blocker was found in the implemented
behavior. The primary drift risk is future scope pressure, not current behavior.

## 6. Product Invariants Audit

| Invariant | Status | Evidence or uncertainty |
| --- | --- | --- |
| Common path remains small | Preserved | Four commands and generated starter project; no external user test. |
| Configuration expresses intent | Insufficient evidence | No project configuration is needed; usefulness of the convention is unknown. |
| Complexity belongs to product before user | Preserved provisionally | Shared pipeline absorbs parsing, validation, rendering, and commit safety. |
| Errors explain the user's problem | Preserved provisionally | Codes and source-aware causes/expectations/help exist; behavioral effect is unknown. |
| Performance removes friction | At risk | Release smoke is fast, but authoritative budgets are unavailable. |
| Content remains portable | Preserved structurally | Markdown, templates, and static output are ordinary files; adoption value is unknown. |
| Extension is not mandatory assembly | Preserved | No extensions are required for the golden path. |
| Every capability earns complexity | Preserved provisionally | Direct dependencies map to current behavior; asset variants need later review. |

No invariant is classified as violated. Performance is the only invariant at risk
because its product-derived threshold cannot yet be checked.

## 7. Zero-Config Decision Audit

| User decision | Classification | Reason |
| --- | --- | --- |
| Choose the project path in `new` | Publishing intent | The user owns where the publication lives. |
| Add a `title` | Necessary content information | A page title cannot be safely inferred for the current model. |
| Place files under `content/` | Strong default | The product needs a discoverable content boundary. |
| Edit `presentation/page.html` | Intentional customization | The user chooses presentation changes. |
| Use `dev`, `check`, and `build` | Product workflow | Each command has a distinct publishing job. |
| Learn `+++` front matter and `title` | Historical convention / necessary current syntax | It is not yet clear whether this is a justified learning cost. |
| Know the fixed preview port | Implementation leakage risk | It is documented but not a publishing decision. |
| Understand `output/` | Necessary generated-output information | The user needs to locate the site to publish it. |

No user is asked to configure a build graph, theme, plugin, deployment, or
runtime. The front matter convention and fixed port are the two decisions to
observe in public usage rather than expand preemptively.

## 8. Command Surface Review

`new` creates the starting publication; `dev` provides the local edit loop;
`check` answers whether the publication is valid; `build` produces the static
publication. They are conceptually distinct and share one pipeline. Help exposes
no unnecessary flags. No additional command is required by the current MVP.

Requests for `serve`, `watch`, `clean`, `render`, or `explain` would currently
represent implementation leakage or post-MVP capability until a product need is
demonstrated.

## 9. Product Model Review

Publication, Content, Attributes, Address, and Presentation are used internally
to keep the pipeline explicit. A first-use user primarily sees content,
presentation, preview, validation, and output. Asset, Collection, and Override
are not required to understand the current golden path. No additional Rust or
pipeline concept is exposed in the CLI.

The available evidence supports the model as a coherent technical representation;
it does not show whether real users understand it naturally or whether its
conceptual budget survives a real publication.

## 10. Differentiation Sanity Check

Low operational surface is observable in the implementation: four commands, no
project configuration, generated structure, no required ecosystem assembly, and
coarse invalidation. High explanatory power is observable in diagnostics that
connect stable code → source → cause → expected state → remediation.

These properties are internally present but not externally demonstrated. The
release is justified as an experiment because the two properties are coherent and
testable, not because they are validated.

## 11. Generic SSG Test

After removing Rust, speed, architecture, and Astral language, the remaining
observable proposition is: a developer can create and publish a small technical
site through a compact file convention, while the tool validates publication
mistakes and preserves a valid preview during recovery.

That is more specific than “modern SSG,” but it may still collapse to “simpler
SSG with good diagnostics.” This is strategically weak until real developers
describe the distinction without being taught it. It does not block controlled
experimentation; it becomes a primary post-release evidence question.

## 12. Competitive Plausibility

The thesis remains coherent and testable enough to justify experimentation, but
the current repository contains no fresh competitive study and no user comparison.
Implementation quality, Rust, a single binary, and fewer dependencies are not
reasons for the product to exist. The reason remains the untested combination of
small operational surface and explanations that improve recovery.

Fresh research is needed before making claims about established SSG behavior,
market positioning, or migration advantage.

## 13. Rust Decision Review

Rust supports native distribution, owned semantic types, provenance, structured
diagnostics, and predictable synchronous behavior. The implementation uses no
custom traits, async runtime, feature flags, unsafe code, or generic framework.
Its concurrency is limited to the HTTP server thread and watcher state. Rust has
not become a user-facing concept or an additional setup requirement.

No concrete evidence shows that Rust is materially harming the product. Reopening
the technology decision is not warranted in this review.

## 14. Release Readiness Interpretation

The release readiness report supports controlled pre-release review:

- the standalone archive works outside the repository;
- the golden path and failure recovery work on macOS Apple Silicon;
- output integrity, determinism, diagnostics, and watcher behavior are exercised;
- the release boundary and unsupported capabilities are documented.

Required hardening remains bounded: import and measure against authoritative
performance budgets, resolve the platform matrix, and perform the proportionate
release dependency/security review. These are hardening tasks, not product-model
revisions. The fixed port and single verified platform are known limitations for
the controlled release.

## 15. Product Hypotheses Still Unvalidated

| Hypothesis | Evidence status |
| --- | --- |
| Operational complexity is meaningful | Inference only |
| Value exists beyond beginners | No meaningful evidence |
| Strong defaults cover a useful common case | Inference only from one generated project and fixtures |
| Explainability changes behavior | No meaningful evidence |
| Portability influences adoption | No meaningful evidence |
| Migration friction suppresses switching | No meaningful evidence |
| Migration intelligence could reduce that barrier | No meaningful evidence |
| The product can remain substantially smaller than competitors | Inference only |

None is called validated. The ship → observe → learn strategy is explicit.

## 16. Public Release Learning Objectives

The v0.1 artifact should teach whether developers install it, create real
publications, understand the conventions, recover from errors, predict output,
understand generated files, request configuration, attempt migration, and return
for repeated use. It should also reveal whether the small conceptual surface
survives content beyond the fixture and whether diagnostics change behavior.

## 17. Post-Release Evidence Signals

Collect real installations where measurable, successful real sites, repeat usage,
issues tied to actual workflows, documentation questions, configuration requests,
feature requests classified by need, migration attempts, contributions, real
performance failures, and abandonment reasons. Do not use stars or downloads as a
substitute for product evidence.

## 18. Risk Register

| Risk | Evidence | Likelihood | Impact | Mitigation timing |
| --- | --- | --- | --- | --- |
| Users cannot install on their platform | Only macOS Apple Silicon artifact verified | Medium | High | Before broad release; narrow matrix or add tested artifacts. |
| Performance budget is violated | Budgets unavailable | Unknown | High | Before final v0.1; import budgets and measure. |
| Fixed port blocks `dev` | Fixed `127.0.0.1:3000` | Medium | Medium | Disclose for controlled release; revisit only with evidence. |
| Conventions require explanation | `+++` and title are fixed syntax | Unknown | Medium | Observe real usage; do not add configuration yet. |
| Diagnostics are admired but do not improve recovery | No user sessions | Unknown | High | Post-release task-based observation. |
| Scope expands in response to requests | Unsupported capabilities are explicit | Medium | High | Keep a separate post-MVP queue; do not label features hardening. |
| Output integrity regression | Staging and tests currently pass | Low | High | Preserve quality suite and release smoke test. |

## 19. Required Pre-Release Actions

- Import the authoritative product and technical documents into the repository.
- Measure the named performance budgets using their specified methodology.
- Confirm the intended platform matrix and publish only tested artifacts.
- Run a proportionate dependency and release security review.
- Keep the release draft until these checks are complete.

## 20. Explicitly Deferred Questions

Do not resolve now: product-market demand, broad differentiation, migration
conversion, migration intelligence, long-term maintenance cost, retention,
willingness to pay, ecosystem viability, enterprise use, large-site scalability,
plugin/theme strategy, and package-manager integrations.

## 21. Final Decision and Rationale

**PROCEED TO HARDENING**

Given what is known, the MVP is coherent, bounded, and technically capable of
generating useful real-world evidence. No current product, technical, or
strategic reason is strong enough to prevent controlled public experimentation.
The decision does not say the thesis is validated. It says that completing the
bounded hardening work and exposing the real artifact is now more informative
than further speculative implementation.

### Stage 18 handoff

**Release blockers:** authoritative performance budgets and platform matrix must
be resolved before final public v0.1; the draft release remains unpublished.

**Required hardening:** reproduce the specified performance measurements, verify
each claimed platform artifact, run the proportionate dependency/security review,
and preserve the golden-path release smoke test.

**Accepted v0.1 limitations:** fixed loopback port, macOS Apple Silicon-only
controlled artifact, unsupported assets/configuration/custom routing, and no
product-validation claim.

**Forbidden scope expansion:** plugins, themes, migration automation, RSS,
sitemap, image processing, generalized extension hooks, build graphs, caches,
and extra commands.

**Post-release evidence questions:** installation success, repeated use,
diagnostic recovery, predictability, portability value, configuration pressure,
real-site fit, and switching/migration interest.

The most useful next experiment is therefore to complete the evidence and
security hardening above, then release the bounded artifact and observe real use.
The product is not validated; it is ready to earn that evidence.
