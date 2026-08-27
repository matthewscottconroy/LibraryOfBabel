# The Turing Machine

In 1936 Alan Turing published a paper with the forbidding title *On Computable
Numbers, with an Application to the Entscheidungsproblem*. He was 23. The paper
answered a question posed by David Hilbert — whether there is a mechanical
procedure that can decide the truth of any mathematical statement — and the answer
was no.

To prove that, Turing needed something first: a precise definition of "mechanical
procedure". You cannot prove that no procedure exists until you have said exactly
what a procedure is. The machine he invented for that purpose is the one that
carries his name, and it was never meant to be built.

## What it is

Take our finite state machine and give it one thing: a tape.

- An **infinite tape** divided into cells, each holding one symbol.
- A **head** positioned over one cell, which can read and write.
- A **finite set of states**, exactly as before.
- A **table of rules**.

Each rule has the shape:

> If you are in state *q* and the head reads symbol *s*, then write symbol *s′*,
> move the head left or right by one cell, and go to state *q′*.

That is all. Read, write, move one square, change state.

The addition compared with the parity machine is small and total. The tape is
unbounded, so memory is no longer fixed. And the head can move *both ways*, so
the machine can revisit what it wrote — which means the tape serves as both input
and scratch paper.

## A machine that adds one

Let us make it concrete. Here is a machine that adds 1 to a binary number written
on the tape.

The trick is the one from Chapter 2: to add 1, start at the rightmost digit. A
`0` becomes `1` and you are done. A `1` becomes `0` and you carry, moving left.
Fall off the left end still carrying, and you write a new `1`.

Suppose the head starts on the rightmost digit, in state `CARRY`:

| state | reads | write | move | next state |
|---|---|---|---|---|
| CARRY | 1 | 0 | left | CARRY |
| CARRY | 0 | 1 | — | HALT |
| CARRY | blank | 1 | — | HALT |

Run it on `1011` with the head on the final `1`:

```
1011     CARRY reads 1  → write 0, move left
1010     CARRY reads 1  → write 0, move left
1000     CARRY reads 0  → write 1, halt
1100
```

`1011` is 11; `1100` is 12. The machine added one, and it did so by exactly the
procedure you would use with a pencil.

Three rules. That is the entire program.

## Why it matters

Now the claim that makes this famous. **Anything that can be computed at all can
be computed by such a machine.**

This is the **Church–Turing thesis**, arrived at independently by Turing and by
Alonzo Church in the same year through entirely different formalisms — Church
used a system of function definition called the lambda calculus, Turing used the
tape machine, and the two were proved equivalent.

It is a thesis rather than a theorem, because "can be computed at all" is not a
mathematical object and so nothing can be proved about it. It is a claim that a
formal definition captures an informal notion. Ninety years of trying to
construct counterexamples have produced none, and every alternative model anyone
has proposed — lambda calculus, recursive functions, cellular automata, any
programming language you can name — turns out to compute exactly the same set of
functions. Not more. Not less.

Take that seriously for a moment. Java, with its objects and threads and
libraries, computes precisely the same functions as a machine with a tape and
three rules. Everything Java adds is convenience — enormous, career-defining
convenience, but not power. A model with this property is called **Turing
complete**, and the term is a compliment about capability and a warning about
limits.

## The limit it was built to prove

Turing did not invent the machine to celebrate what computation can do. He
invented it to show what it cannot.

Having defined mechanical procedure precisely, he asked whether a machine could
exist that reads a description of another machine, plus its input, and determines
whether that machine ever halts. He proved no such machine can exist — the
**halting problem** is undecidable.

We will do the proof properly in Chapter 34, because it deserves the room and
because you will need Unit VI's idea of programs-as-data to feel its force. For
now, hold the shape: some questions about programs cannot be answered by any
program whatsoever. Not "we have not found the algorithm". No algorithm exists,
and we can prove it.

That is an unusual kind of knowledge, and it was obtained by first building a
machine simple enough to reason about completely.

## What to carry forward

The Turing machine is not how computers work, and nobody programs one. Its value
is as a *reference*: the simplest thing that is powerful enough, against which
every other model is measured.

And the specific idea that carries into the next lesson is this. The machine's
rules are a table. A table is data. Turing's key move in the halting proof was to
write a machine's table onto the tape of another machine — treating a program as
data to be read.

That move is the subject of the next lesson, and of Unit VI, and arguably of the
entire discipline.
