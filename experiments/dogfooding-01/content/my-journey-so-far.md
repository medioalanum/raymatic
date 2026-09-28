+++
title = "The Day I Realized I'd Rather Ask \"Why\" Than \"How\""
+++

*2026-08-25 10:00 · Career*

I'm Alan, and for a long time my job was to make sure other people's systems
talked to each other without falling over.

That's not a metaphor. Back in 2008, I was writing backend integrations for
banks and retailers in Rio de Janeiro. It was the kind of work where a single
missed edge case in a payment flow could become someone's very bad Monday. You
learn a lot about how software actually behaves, rather than how it behaves in
an architecture diagram, when you're the one getting paged about it.

At some point, I noticed that I was spending less time asking, "How do I build
this?" and more time asking, "Should we be building this at all, and why?" The
second question turned out to be more interesting to me than the code itself,
so I followed it. It took me from Accenture to product roles at Americanas,
Cielo, Stone, Pismo, which was later acquired by Visa, Grupo Fleury, and most
recently Liferay in Italy.

Along the way, my job title changed from engineer to product manager, but the
underlying obsession never did: high-volume systems that need to be boring in
the best possible sense. Reliable. Predictable. The kind of infrastructure
nobody thinks about until it breaks and that, ideally, never does.

I have picked up a few lessons over the years. None of them sounded especially
interesting until I had to learn them firsthand.

Data quality is not a chore you assign to someone else. It is the difference
between a platform that other teams trust and one they quietly build
workarounds for. I have spent more hours than I'd like to admit chasing down
why a metric looked wrong. Every time, fixing it mattered more than whatever
feature I was supposed to be prioritizing that week.

That is the reason I keep building small tools such as the [Veneto City Data API](https://github.com/medioalanum/veneto-city-data-api): working with source data makes those quality decisions concrete.

Monitoring only earns its keep if it changes what you build next. A dashboard
nobody reads during a planning meeting is just decoration.

Having written the kind of code I now make decisions about is, honestly, an
unfair advantage. When engineering tells me something is going to be painful,
I usually already know why. That means we can skip the part where I need
convincing and get straight to figuring out what to do about it.

These days, I'm based in Italy and working my way through a master's degree in
data engineering, mostly because I became curious about the tools underneath
the platforms I have spent my career managing: Python, SQL, Spark, and cloud
pipelines. It is slower going than I expected and more fun than I expected,
which feels about right.

This blog is where that curiosity is going to live. Expect notes on product
decisions for technical platforms, things I learn while wrestling with data
pipelines, and the occasional story from the trenches of API governance that
somehow turns out to be more entertaining than it sounds. I'll try to keep it
useful and honest rather than polished.

Thanks for reading this far. See you in the next one.
