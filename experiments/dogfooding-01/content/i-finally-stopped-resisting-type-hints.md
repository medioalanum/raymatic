+++
title = "I Finally Stopped Resisting Type Hints"
+++

*2026-08-26 10:00 · Data Engineering*

For years, my relationship with Python typing was mostly denial.

I learned the language when type hints were not part of everyday Python, and
old habits tend to survive long after their original reasons disappear. Why
declare types in a language whose appeal was that you did not have to? I wrote
scripts, they worked, and I moved on.

That approach was fine while the scripts were small enough to fit in my head.
Then I started building a [finance tracker](https://github.com/medioalanum/privio) that was slightly larger than a
weekend experiment, and the cracks appeared quickly. A function expected a
string, but a value arriving from three files away was a number. Nothing
crashed. The output was simply wrong, and I had to trace the value backward to
understand why.

That was the point when untyped code stopped feeling convenient and started
feeling expensive.

## Python changed, and so did the code we write with it

Python type hints do not turn the language into Java, nor do they change how
Python behaves at runtime. Their first job is simpler: they make intent
explicit.

```python
from decimal import Decimal


def convert_amount(amount: Decimal, exchange_rate: Decimal) -> Decimal:
    return amount * exchange_rate
```

That signature is a small contract. A reader can understand what the function
expects and returns without opening its implementation. An editor can offer
better completion. A type checker can follow values across modules and point
out a mismatch before the incorrect value reaches a report, an API response,
or a database.

This matters because much of today's Python code is neither quick nor
disposable. Data pipelines, internal platforms, machine learning services, and
APIs often reach production, gain several contributors, and live much longer
than their authors expected. The flexibility that makes a prototype pleasant
can make a mature codebase difficult to reason about.

Type hints do not remove that flexibility. They give it boundaries.

## Why I am using ty, even in beta

Once I decided to annotate the code, I needed a tool that would actually read
the annotations. I chose [ty](https://docs.astral.sh/ty/), Astral's type checker
and language server.

The choice was partly practical. I already use `uv` to manage projects and
Ruff for linting and formatting, so `ty` fits naturally into the same workflow.
It is written in Rust, starts quickly, understands the project's virtual
environment, and can keep checking changes in watch mode.

Adding it to a project managed by `uv` is straightforward:

```bash
uv add --dev ty
uv run ty check
```

`ty` is still officially in beta and uses `0.0.x` versioning. Its diagnostics
and behavior may change between releases, and that is worth considering before
making it the only checker responsible for a large, established codebase. For
my personal project, the tradeoff makes sense. The feedback is fast enough to
stay in my development loop, and I am comfortable encountering a rough edge
occasionally.

More importantly, I am using the tool rather than postponing the habit while I
search for the perfect setup.

## When annotations start doing real work

Static analysis was enough to change my mind about type hints, but Pydantic and
FastAPI made the value much more concrete.

Consider a small model for an expense:

```python
from decimal import Decimal

from fastapi import FastAPI
from pydantic import BaseModel, Field


class Expense(BaseModel):
    description: str
    amount: Decimal = Field(gt=0)
    category: str


app = FastAPI()


@app.post("/expenses", response_model=Expense)
def create_expense(expense: Expense) -> Expense:
    return expense
```

The annotations now serve several purposes at once.

`ty` can analyze how `Expense` objects move through the application while I am
writing the code. Pydantic validates incoming data at runtime and rejects an
amount that does not satisfy the model. FastAPI uses the same model to describe
the request and response, validate the endpoint input, and generate an OpenAPI
schema with interactive documentation.

These tools operate at different moments. A static type checker catches
inconsistencies in paths it can analyze before the program runs. Pydantic deals
with data arriving at runtime, where annotations alone cannot protect you.
FastAPI connects those typed models to the HTTP boundary.

That distinction is important. Type hints are not runtime validation, and
runtime validation is not a replacement for static analysis. Together, they
cover different ways a bad value can enter or move through a system.

## Why this feels like modern Python

I would now reach for FastAPI before Flask when building a typed JSON API, not
because Flask is obsolete, but because FastAPI makes the contract part of the
normal path. The route, validation rules, Python model, and API documentation
are connected instead of being maintained as separate descriptions that can
quietly drift apart.

There is some extra ceremony here. My finance tracker does not need
enterprise-grade type safety. That is also why it is a good place to learn.
Habits are easier to build on a low-stakes project than during a production
incident.

I still like Python for the same reason I did years ago: it lets me get from an
idea to working code quickly. Type hints have not taken that away. They have
made it easier to return to the code a week later and understand what I meant.

If you have been avoiding type hints, do not wait for a codebase large enough
to force the issue. Annotate the next function you write, run a checker, and
pay attention to the first mistake it catches before you do. That moment is a
better argument than any style guide.
