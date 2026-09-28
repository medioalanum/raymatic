+++
title = "Spacey: From Side Project to Business Case"
+++

*2026-09-22 10:00 · Career*

At Americanas, we ran four consumer brands off the same web platform: Americanas, Submarino, Shoptime, and Sou Barato. All of it, banners, campaign pages, and scheduled promotions, went through a third-party tool that handled advertising and content management across the sites.

The tool did a lot. That was part of the problem. The dashboard was confusing, packed with features and configuration options that made it difficult for Marketing to use on their own. For anything beyond the basics, such as a more sophisticated campaign, a delivery optimization, or a scheduling setup that needed to be just right, Marketing usually ended up pulling in Engineering, whether because the configuration itself was difficult or because the vendor's system wasn't behaving. On top of that, the platform had real reliability problems. It wasn't just complex. It broke, more than once.

We'd lived with that pattern across more than one event by then. At some point, we decided we'd had enough of it and started building our own version, not as a reaction to a single failure, but as something we believed in and kept working on while waiting for the right moment to prove it.

Black Friday at Americanas meant roughly a billion reais in sales, so we ran preview events beforehand to stress-test everything at scale. During one of those preview events, the vendor's system failed again. That was the moment. We already had our own solution ready to go, and this time, leadership let us test it on one of the brands.

Getting to that point hadn't been easy. Spacey wasn't some internal tool nobody would notice if it failed. It would sit on the platform serving four live brands, replacing an established vendor. I remember the skepticism clearly: why trade a consolidated partner for whatever a handful of engineers thought they could build better? The problems were real: Marketing's lack of autonomy, the time Engineering kept losing to configuration and vendor issues, and the operational risk we'd now seen materialize more than once. But replacing an established vendor is still a hard sell to leadership on operational grounds alone.

Cost became part of that pitch: eliminating the licensing spend, plus the operational costs that came with it, including the vendor's own consultants who periodically had to be brought in just to keep the platform running. We could also lean on cloud infrastructure the company already had and use caching to keep our own costs down. Some of those savings were straightforward to estimate, particularly the licensing costs. Others depended on assumptions about the architecture we were proposing and only became measurable once the system had been running for a while. In the end, the savings were in the range of BRL 1 million a year.

It worked. Not spectacularly and not with fanfare. It just did the job Marketing needed, without the friction the old tool demanded. That was enough to move to the other sites one at a time, until the vendor wasn't in the picture anymore.

We'd been calling our internal experiments "Kevin" projects for a while by then, Kevin Mitnick for one and Kevin Bacon for another, mostly because it made stand-ups more fun. This one became Kevin Spacey, and the timing lined up almost too well: *House of Cards* had just landed on Netflix and the whole team was watching it. The name stuck harder than any of the others.

Rebuilding the vendor's tool feature for feature was never the plan. The tool was consolidated and enormous for a reason, and matching it wasn't the point. What we built instead came out of working closely with the Marketing team: the relatively small set of capabilities they relied on every day, done well, instead of a system built to theoretically do anything.

Giving Marketing real autonomy wasn't something we discovered halfway through. It was the point from day one. Marketing wasn't just a stakeholder in this. They were the ones who would actually use what we were building, and we needed the Marketing director's buy-in to get any traction in Engineering in the first place. Spacey grew into a self-service content platform, where Marketing could configure layouts, components, and scheduled campaigns on their own, because that autonomy was what we'd set out to build, not a feature we backed into.

Looking back, there was a real business case behind Spacey, and the cost savings mattered when we needed leadership on board. But it started earlier than that: with a partner system that kept failing at the worst possible moments, a Marketing team that had been waiting for autonomy they never had, and a solution we'd already built and believed in, sitting ready for the moment someone would finally let us try it.
