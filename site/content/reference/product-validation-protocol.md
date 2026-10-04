+++
title = "PRODUCT VALIDATION PROTOCOL"
+++


## Purpose

Test whether representative developers can independently use Raymatic and
whether observed behavior supports or challenges the product thesis. This is a
task study, not a feature demonstration. Moderators must not describe Raymatic
as simple, predictable, explainable, Astral-inspired, or low-configuration.

## Hypotheses under test

| ID | Hypothesis | Primary evidence |
| --- | --- | --- |
| H1 | Static-publishing machinery creates meaningful operational friction. | Prior-tool context, setup decisions, and task observations. |
| H2 | Strong conventions work for a useful class of small technical publications. | Completion without immediate escape mechanisms. |
| H3 | Normal publishing requires fewer incidental decisions and concepts. | Decisions, configuration changes, and concepts named by participants. |
| H4 | Users can predict common behavior. | Predicted versus actual behavior before selected changes. |
| H5 | Product explanations enable recovery without implementation debugging. | Diagnostic comprehension and unassisted recovery. |
| H6 | Content remains understandable and recoverable outside the tool. | Source-only portability inspection. |
| H7 | Users perceive less ongoing machinery and coordination. | Maintenance interview and comparison evidence. |
| H8 | The improvement is meaningful enough to create adoption interest. | Willingness to try a new site or real content, distinguished from praise. |

## Participants

Recruit developers who are comfortable with files and version control, have
published or maintained content-oriented sites, value static output, and can
separate simplicity from missing capability. Prefer participants with prior SSG
experience. Exclude project contributors and people already familiar with this
project's design. Record prior tools and site-maintenance context for every
participant.

The release cohort must use a tested release target. At present that is macOS on
Apple Silicon, pending resolution of the readiness blockers.

## Session structure

Use a 60–90 minute moderated remote or in-person session. Give the participant
the release artifact and `docs/INSTALL.md`; otherwise provide no tutorial. Ask
them to think aloud when comfortable. Do not rescue a participant unless they
are completely blocked; record every intervention and its trigger.

### Tasks

1. Install the artifact and find how to invoke it.
2. Create a new technical publication and describe the generated structure.
3. Add multiple content files; before running the tool, predict their URLs.
4. Start local development and edit content.
5. Make one small presentation edit in `presentation/page.html`.
6. Create or receive malformed content; recover without explanation.
7. Create or receive a route collision, broken internal reference, or missing
   title; identify what failed, where, why, expected state, and remediation.
8. Before selected rename, reference, and collision changes, state expected
   behavior; compare prediction with observation.
9. Determine whether the publication is production-valid without being told the
   command name.
10. Produce the static site and locate the generated output.
11. Inspect the source without running Raymatic; identify portable content,
   presentation, generated output, and tool-specific information.

For participants with an established SSG, optionally compare an equivalent small
workflow in the tool they already know. Never ask them to learn a new competitor
or manipulate the other tool to favor Raymatic.

## Measures

Preserve raw values: time to `--help`, first valid publication, first preview,
successful check, and build; setup decisions; documentation lookups;
interventions; incorrect predictions; error diagnosis time; unassisted recovery;
configuration changes; output-location identification; and portability answers.
Do not create a composite score.

## Moderator interview

Ask only after task completion:

- What did you need to understand before publishing?
- What required the most attention?
- What surprised you?
- How did you decide what to do after an error?
- What would you maintain after six months?
- What would make you hesitate to use this for a real site?
- Which missing capabilities are blockers and which are preferences?

For experienced SSG users: what would prevent a trial on a new site or existing
content, and what uncertainty would stop a switch?

## Evidence log template

| Field | Record |
| --- | --- |
| Participant context | Prior publishing and SSG experience; no identifying details required. |
| Task | Number and scenario. |
| Observation | Concrete behavior, including failed attempts. |
| Intervention | None, or exact moderator action. |
| Quote | Verbatim when captured. |
| Measurement | Raw time/count/value and method. |
| Hypothesis affected | H1–H8. |
| Interpretation | Provisional reading. |
| Alternative explanation | What else could explain it. |
| Confidence | Low, medium, or high with rationale. |

## Classification and falsification

Classify every problem as implementation defect, UX defect, missing MVP
requirement, legitimate post-MVP capability, product-model failure, or
out-of-scope need. Do not convert a request into roadmap scope during a session.

Treat as falsification evidence when the proposition reduces to a generic faster
or simpler SSG, operational complexity is not meaningful, conventions require
unacceptable loss of control, explanations do not change recovery behavior,
migration cost overwhelms incremental value, a compelling workflow needs an
ecosystem-scale scope, or established tools already solve the defined problem.

Keep observation, recurring pattern, interpretation, hypothesis, and established
evidence distinct. A small qualitative study discovers failure modes; it does
not establish market-wide conclusions.
