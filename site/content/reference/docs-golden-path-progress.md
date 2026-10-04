+++
title = "GOLDEN PATH PROGRESS"
+++


Semantic expansion pauses after Slice 5. The next slices complete the existing
`new → dev/check → build` experience using the shared publication pipeline.
No additional command, configuration format, dependency graph or semantic type
is required by this phase.

## Slice 6 — Preview publication addresses

A publication linking to `/about/` already validates and builds an
`about/index.html` page, but the development server previously tried to read the
directory as a file and returned 404. URLs containing query strings also failed
to locate otherwise valid output. HTML responses lacked an explicit HTML content
type.

The preview now serves a directory's `index.html`, ignores query strings when
locating output, and identifies HTML responses as `text/html; charset=utf-8`.
Direct output files and the revision endpoint continue to work. Missing output
and parent-directory traversal remain 404 responses.

The regression test creates a publication with derived and explicit addresses,
evaluates the shared pipeline, and requests its preview through an HTTP server
on an ephemeral loopback port. It covers home, nested pages, direct HTML, query
strings, a static file, revision polling, missing pages and traversal. Existing
dev tests cover preserving the last valid preview and recovery after invalid
content.

## Source boundary

The Product Contract is available in the local ChatGPT project. Repository
traceability and the previous implementation-design summary describe the shared
pipeline, coarse rebuilds and four-command UX. The complete Product Model, MVP,
UX Specification, Technical Requirements, Architecture and Implementation Design
are not present in this checkout. This slice fixes existing behavior; it does
not claim formal conformance with unavailable acceptance text or numeric budgets.

## Next bounded candidate

Asset changes currently are not watched by `dev`, although assets participate in
`check` and `build`. A separate slice can cover creation, editing and removal of
optional `assets/` during a development session, with recovery and HTTP tests.

## Slice 7 — Watch conventional assets

The development watcher now observes the publication root and filters events to
`content/`, `presentation/`, and the optional `assets/` tree. This means creating
`assets/`, adding or editing a static file, and removing one all trigger the same
coarse rebuild used by the rest of the publication. Generated `output/` and
`.raymatic-preview/` changes are ignored, so a rebuild cannot trigger itself.

The semantic model and command surface remain unchanged. The source-event filter
has focused coverage for all three source trees, generated paths, and a path
outside the publication.

The local environment cannot bind the loopback socket used by the existing HTTP
preview regression test (`Operation not permitted`); CI remains the authoritative
cross-platform check for that test.
