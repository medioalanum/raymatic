# Post-Release Product Direction

## 1. Executive Decision

Raymatic v0.1.0 generated useful technical and dogfooding evidence, but not enough independent user evidence to justify a new capability or a v0.2 plan. The smallest earned investment is one structured observation of an unfamiliar developer using the released artifact and README to create a small publication.

## 2. Evidence Baseline

The evidence describes Raymatic `0.1.0`, release commit `d3c4bf9`, published on 2026-09-28, with official support for macOS Apple Silicon. The scope is the four-command workflow, Markdown with title front matter, one shared presentation template, structured diagnostics, coarse development rebuilds, and deterministic static output. The current repository later added documentation and dogfooding records; no product code or release behavior changed.

## 3. Evidence Inventory

| Source | Period | Type | Scenario | Strength | Limitation |
| --- | --- | --- | --- | --- | --- |
| Release artifact smoke test | 2026-09-28 | Direct behavioral evidence / technical measurement | Clean extraction, `--version`, `--help`, `new`, `check`, `build` | Strong for the tested target | One platform and starter workload |
| Existing integration suite | pre/post-release | Direct behavioral evidence | Diagnostics, deterministic output, output safety, dev recovery | Strong for intended cases | Maintainer-authored fixtures |
| `DOGFOODING_01.md` | 2026-09-28 | Dogfooding evidence | Three-post Pelican publication plus one Raymatic article | Strong for capability and migration friction | Creator already understands Raymatic |
| `PELICAN_RAYMATIC_MAPPING.md` | 2026-09-28 | Internal assessment grounded in dogfooding | Concept and machinery mapping | Useful interpretive record | One source publication |
| GitHub public release | since publication | External usage signal | v0.1.0 distribution | One observable download | Download is not usage or adoption |
| GitHub issues/discussions | checked 2026-09-28 | External reported evidence | Public feedback search | No reports found | Absence is not positive evidence |

## 4. Evidence Quality and Limitations

The strongest evidence is direct behavior on macOS Apple Silicon and the completed migration. The weakest evidence concerns discoverability, onboarding, willingness to switch, and market demand. No unfamiliar developer session, external bug report, contribution, or second real publication is available. No claim of product validation is warranted.

## 5. Dogfooding Findings

The core pipeline represented five pages, built deterministically, preserved readable Markdown, and produced actionable diagnostics. The migration required workarounds for dates, categories, summaries, assets, and the homepage/article presentation distinction. These are observed creator dogfooding findings, not general market requirements.

## 6. External Usage Findings

No external usage report, issue, discussion, contribution, installation complaint, or feature request was found. The public release asset has one recorded download, which establishes distribution exposure only. The absence of feedback is unknown evidence, not evidence that the product is understood or useful.

## 7. Product Contract Reassessment

The contract's operational-complexity hypothesis is supported in one migration: Raymatic removed Python/uv setup, Pelican settings, plugins, theme submodules, and generator-specific deployment machinery. The hypothesis that this matters to target users remains untested. Explainability is supported behaviorally by diagnostics, but behavior change by unfamiliar users is untested. Portability is structurally supported by Markdown and HTML/CSS, but its influence on adoption is unknown.

## 8. Load-Bearing Assumptions

| Assumption | Status | Evidence |
| --- | --- | --- |
| Operational complexity is painful | Mixed | The real Pelican setup contains machinery; no user report says it is painful |
| The problem matters beyond beginners | Still untested | No external user evidence |
| Strong defaults cover a useful common case | Strengthened | Five-page dogfood publication worked with one template |
| Explainability changes behavior | Still untested | Diagnostics were actionable; unfamiliar-user recovery was not observed |
| Portability influences adoption | Still untested | Source stayed portable; no switching evidence |
| Migration friction suppresses switching | Mixed | One migration had meaningful gaps; no switching decision observed |
| Migration intelligence could reduce the barrier | Still untested | No migration tool or comparative study |
| Raymatic can remain substantially smaller | Strengthened provisionally | Fewer setup/deployment concepts; assets and metadata pressure remain |

## 9. Product Model Pressure

Real use put pressure on `Attributes`, `Address`, `Presentation`, and `Asset`. Dates, categories, summaries, and assets were meaningful publication intent rather than merely Pelican machinery. The single shared presentation template also made homepage/article differences awkward. The evidence does not yet establish the smallest model change or whether these needs recur across publications.

## 10. Low Operational Surface Assessment

Raymatic reduced setup, dependencies, configuration files, plugin wiring, theme distribution, and local build steps. It did not eliminate presentation work: the creator wrote a substantial inline template and manually represented metadata. The reduction is real for the tested path, but file count alone does not prove lower conceptual complexity.

## 11. Explainability Assessment

The missing-title test moved directly from failure to source location, cause, expectation, and remediation. Existing tests cover malformed front matter, rendering failure, route collision, and broken reference. This supports high explanatory power for known cases. There is no evidence yet that a new user can discover or act on these diagnostics without prior model knowledge.

## 12. Differentiation Assessment

After removing Rust, startup, and implementation quality, the observable distinction is a deliberately small publishing path with integrated validation, diagnostics, preview recovery, and safe output commit. That is more specific than “simple” or “modern,” but its value relative to established generators remains a hypothesis because no comparative user evidence exists.

## 13. Pelican → Raymatic Findings

Pelican machinery disappeared from the user workflow: Python environment management, settings files, plugin configuration, theme submodules, and Pages build orchestration. Complexity moved into a single template and Markdown conventions for metadata and addresses. Assets, archive generation, and structured summaries became harder or unavailable. The experiment did not reveal a need for Pelican compatibility.

## 14. Bugs and Reliability Findings

No implementation, release, diagnostic, or output-safety defect was found. The artifact passed clean-machine smoke tests, and the dogfooded publication passed `check`, `build`, and repeated hash comparison. No v0.1.1 reliability patch is currently justified by observed defects.

## 15. UX Findings

The core command sequence is coherent when the user already knows the model. The dogfooding author had to decide how to encode dates and categories and how to link generated page indexes. This is meaningful UX friction, but its severity for an unfamiliar user is unknown.

## 16. Documentation Findings

The README now documents the actual release, installation, four commands, diagnostics, and limitations. The dogfooding showed that documentation cannot fully compensate for unsupported assets or structured metadata. A clean unfamiliar-user walkthrough is still missing.

## 17. Capability Pressure

Observed pressure concerns asset copying, structured attributes, explicit addresses, and distinct page presentation. These are candidate problems, not approved features. Feeds, sitemaps, plugins, themes, migration automation, and deployment tooling had no post-release evidence and remain out of scope.

## 18. Migration Evidence

Migration was meaningfully difficult in one case, especially for assets, metadata, and page-specific presentation. Some friction was generic publishing work and some came from the existing Pelican site's structure. One creator migration is insufficient evidence for a generalized migration investment; another migration or external observation is needed.

## 19. Rust Decision Status

Rust remains closed as a decision. The release artifact was standalone, startup and build behavior were fast on the supported target, no reliability problem was observed, and no contributor evidence indicates Rust is suppressing participation. The platform boundary is a support-policy decision, not evidence against Rust.

## 20. Scope/Complexity Analysis

Adding assets, attributes, addresses, or page presentation would affect semantic types, validation, rendering, output planning, documentation, and tests. Each could enlarge the common path. The evidence currently earns investigation and observation, not implementation. A generalized framework would fail the common-path and complexity tests.

## 21. Candidate Next Investments

| Candidate | Evidence | User problem | Complexity | Learning value |
| --- | --- | --- | --- | --- |
| v0.1.1 reliability patch | No observed defect | None currently demonstrated | Low, but no target | Low |
| Add assets/attributes now | One dogfood migration | Preserve common publication intent | High semantic and UX cost | Moderate but biased |
| Build migration tooling | One Pelican migration | Reduce switching friction | High and likely premature | Low without another case |
| Improve onboarding through observation | No unfamiliar-user evidence | Discoverability and first-use uncertainty | Low | High |
| Run another real migration | One migration exists | Test recurrence of pressure | Low product change | High |
| Gather unfamiliar-user evidence | Evidence scarcity | Determine whether model is understandable | No product complexity | Highest next value |

## 22. Evidence Sufficiency

Another implementation change would teach less than another real usage case. The creator migration already identified candidate pressure; the unanswered question is whether those pressures are common, understandable, and adoption-relevant. The evidence base is too internally biased to justify v0.2 capability work.

## 23. Versioning Implications

No new version is justified now. No v0.1.1 defect patch is needed, and a v0.2 capability has not earned its complexity. The next objective should be completed before choosing a version.

## 24. Primary Product Direction

Gather more evidence.

## 25. Next Objective

Observe one developer unfamiliar with Raymatic using the released macOS Apple Silicon artifact and README to create, edit, check, preview, and build a small publication without maintainer intervention.

## 26. Explicit Non-Goals

Do not add assets, structured attributes, custom addressing, page kinds, migration tooling, plugins, themes, feeds, sitemap generation, deployment automation, caching, a build graph, or v0.2 planning during this next objective.

## 27. Success Evidence

Record whether the developer can install the artifact, understand the generated structure, author a page, recover from one intentional error, run `dev`, run `check`, run `build`, and locate the output. Capture questions, workarounds, time-to-first-preview, diagnostic interpretation, and any point where the product model is misunderstood.

## 28. Decision Gate

After that observation, compare the evidence with the dogfooding findings. Consider capability work only if the same legitimate need recurs and the observed workaround cost is material. Otherwise preserve the current scope and continue gathering evidence.

## 29. Open Questions

- Can an unfamiliar developer infer the `+++`/`title` convention from the README?
- Does the four-command surface feel like publishing rather than tool management?
- Do diagnostics lead to recovery without prior knowledge?
- Are attributes, assets, and page presentation recurring needs across another publication?
- Does the fixed loopback port create practical friction?
- Would a second migration reproduce the same pressure without importing Pelican machinery?

## Decision

**GATHER MORE EVIDENCE**

### Why

The release and dogfooding prove that the current implementation works for a bounded path and expose meaningful model pressure, but all post-release evidence is maintainer-generated and no external usage report exists. Expanding the product now would convert one migration's friction into an unearned roadmap.

### What happens next

Run one structured unfamiliar-developer observation against the unchanged v0.1.0 artifact and README.

### What we are explicitly not doing

No feature implementation, reliability refactor, migration framework, product-model redesign, platform expansion, or v0.2 plan.

### What would change this decision

Repeated independent evidence of the same material workflow failure, a concrete release defect, or a demonstrated capability need across another real publication would justify revisiting the direction.

### Final Question

The smallest next investment Raymatic has earned is an evidence session with an unfamiliar developer using the unchanged released product. It costs no new product complexity and directly tests whether the observed strengths and friction survive outside the creator's understanding.
