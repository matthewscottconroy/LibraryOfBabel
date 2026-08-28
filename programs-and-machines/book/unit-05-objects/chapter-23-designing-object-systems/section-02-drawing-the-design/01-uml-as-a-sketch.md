# UML as a Sketch

Ten minutes with a pen, before any code exists, is frequently the highest-value ten
minutes in a project. The same ten minutes spent after the code exists is worth
close to nothing.

That asymmetry is the whole of what follows, and it explains why this lesson
recommends drawing and also recommends throwing the drawing away.

**UML** — the Unified Modeling Language — is a notation for drawing software.
It was standardized in 1997, it has thirteen diagram types, and there was a period
when serious people believed programs would be drawn rather than written.

That did not happen, and the reasons are worth a sentence: diagrams detailed
enough to generate code are harder to write than the code, and diagrams that stay
useful are too vague to generate anything. The tools that promised round-trip
engineering are gone.

What survived is the useful part: **two diagram types, drawn by hand, thrown away
afterwards.** Martin Fowler's term is *UML as sketch*, and it is the only mode
this book recommends.

## The class diagram

A box per class. Name at the top, fields in the middle, methods at the bottom.
Leave out anything you are not currently thinking about.

```
┌──────────────────────┐
│ Playlist             │
├──────────────────────┤
│ - items : List<Item> │
├──────────────────────┤
│ + add(Item)          │
│ + totalMinutes() int │
└──────────────────────┘
```

`-` is private, `+` is public. Nobody will mind if you omit them.

Lines between boxes show relationships, and four are worth knowing:

```
  A ────────> B     A uses B (a field, or a parameter)

  A ────────◇ B     aggregation: A holds B, B outlives A

  A ────────◆ B     composition: A owns B, B dies with A

  A ────────▷ B     A extends or implements B
```

The hollow triangle is inheritance. In practice, drawing an arrow and writing what
it means beside it works just as well, and you will not misremember which diamond
is which.

The multiplicities matter more than the arrowheads. Writing `1` and `*` at the
ends — one playlist, many items — is where the actual thinking happens, because
"can there be more than one?" is a real question with consequences, and "can there
be zero?" is another.

## The sequence diagram

The other one that earns its place. Objects across the top, time running down,
arrows for calls.

```
  Deck          Scheduler        Progress
   │                │                │
   │  next(p,g,d)   │                │
   ├───────────────>│                │
   │                │   new(box,due) │
   │                ├───────────────>│
   │  <─────────────┤                │
   │   Progress     │                │
```

This is for questions of the form *who calls whom, in what order*, and it is the
right tool when a piece of behavior spans several objects. It is worth drawing
exactly when you cannot answer "and then what happens?" without one, and worth
throwing away as soon as you can.

## How to actually use it

The honest workflow, which takes ten minutes and is not ceremonious:

**Draw before you code, not after.** The value is in deciding, and it is gone once
the code exists. A diagram drawn afterwards is documentation, and documentation
rots.

**Draw only what you are unsure about.** Three boxes and two lines, on the part
that worries you. Diagramming a design you already understand is procrastination.

**Use a whiteboard or paper.** A drawing tool tempts you to align things, and
alignment is not thinking. Ugly is fine.

**Throw it away.** Or photograph it and attach it to whatever tracks the work. A
diagram you must keep current is a second copy of the code, and the second copy
will be wrong.

The exception is the diagram of a system that already exists and is large. A
one-page map of a codebase — ten boxes, the main dependencies, no methods — is
genuinely valuable to a newcomer and stays true for years, because the top-level
structure is what changes least.

## Why it works at all

Drawing forces you to name things and to commit to relationships, and it does so
in a medium where changing your mind is free.

That is the whole benefit, and it applies just as much to the index cards from
Section 23.1.1, or to writing the class names on a list, or to describing the
design out loud to somebody. The notation is not the point. Doing it before the
code is written is the point, because that is the last moment when a decision
costs nothing.

The failure mode to guard against is the opposite one — designing for a week and
writing nothing. Design is a hypothesis, and the code is the experiment. Ten
minutes with a pen, then write something and find out what you got wrong.

Next: exactly that, done in full.
