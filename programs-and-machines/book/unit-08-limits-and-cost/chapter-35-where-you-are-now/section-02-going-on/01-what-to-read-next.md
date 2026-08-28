# What to Read Next

Each chapter listed sources for its own material. This is the other question: given
all of it, what next?

The answer depends on what you are trying to become, so the list below is arranged
that way rather than by topic — and the second half, about what to *build*, matters
more than the first.

Each chapter's further reading listed sources for that chapter. This is the other
question: what to read next, given all of it.

## The three books after this one

If you read only three, these.

**Abelson and Sussman, *Structure and Interpretation of Computer Programs*.** Free
online. This book's Unit VI is modelled on it and said so. Read it now for the
same material in Scheme, which strips away Java's syntax and leaves the ideas
uncomfortably visible. Chapter 4 builds the metacircular evaluator you have
already built, and sections 4.2 and 4.3 then change it to be lazy and then
nondeterministic — by editing `eval`, which will startle you productively.

**Joshua Bloch, *Effective Java*, third edition.** Ninety items of specific,
argued advice from the person who wrote the collections framework. It has been
cited in a dozen chapters here, and it is the book that turns someone who can write
Java into someone who writes good Java. Read it item by item over months, not
straight through.

**Kernighan and Pike, *The Practice of Programming*.** Short, thirty years old, and
almost entirely still right. On debugging, testing, portability, and notation, from
two people who worked on Unix. The chapter on debugging is the best forty pages
written on the subject.

## By direction

**If you want to go deeper into computer science.**

Sipser's *Introduction to the Theory of Computation* for Chapter 34's material
properly. Cormen and colleagues, *Introduction to Algorithms*, as a reference for
Chapter 32's. Nisan and Schocken's *The Elements of Computing Systems* — the
Nand2Tetris book — which builds a computer from logic gates up to an operating
system, and is the perfect complement to this book because it climbs the same
ladder from the other end.

**If you want to build software well.**

Fowler's *Refactoring* for the vocabulary of design problems. Martin Kleppmann's
*Designing Data-Intensive Applications* for everything Unit VII gestured at.
Nygard's *Release It!* for what happens at scale. And Ousterhout's *A Philosophy of
Software Design*, which is short and argues one thing well: that the depth of a
module — how much it hides relative to its interface — is the measure that matters,
which is Section 35.1.2's claim stated as a design principle.

**If you want to understand the machine.**

Bryant and O'Hallaron, *Computer Systems: A Programmer's Perspective*. It is the
book for Chapter 15 and Section 32.2.3 — cache, memory hierarchy, linking, virtual
memory — and it will make the gaps between predicted and measured cost stop being
mysterious.

**If Unit VI was the part you liked.**

Nystrom's *Crafting Interpreters*, free online, which builds two complete
interpreters and is the natural next step from Chapter 25. Then Wirth's *Compiler
Construction*, also free, for the whole pipeline in under two hundred pages. Then
Friedman and Wand's *Essentials of Programming Languages*, which is a course in
language design conducted entirely by writing interpreters.

**If you want to write correct programs.**

Chapter 9's loop invariants were a first taste. Gries's *The Science of
Programming* is the deep version, and Dijkstra's *A Discipline of Programming* is
the original and is uncompromising. More practically, learn a proof assistant — Coq
or Lean — for a few weeks. It will change how you think about what a specification
is, and it is where Chapter 34's termination checkers live.

## What to build

Reading is the smaller half.

**Finish the interpreter.** Chapter 25's exercises listed what is missing: closures,
a second value type, a static checker, loops. Each is a small project and each
teaches something no reading does. Then give it a garbage collector, which is the
single most instructive thing you can build after an interpreter.

**Write something you will use.** The strongest predictor of learning is caring
whether it works. A tool for a hobby, a script that automates something tedious, a
program for a problem you actually have. The requirement to be *used* forces every
issue in Unit VII: it will meet a malformed file, a missing network, a user who
does the wrong thing.

**Read code you did not write.** Pick a small, well-regarded open-source project
and read it until you could explain its design. This is the skill professional
work consists of and it is almost never taught. Start with something under ten
thousand lines.

**Contribute something small.** A documentation fix, a test, a bug report with a
minimal reproduction. The process — reading the conventions, making a change that
fits, having it reviewed — teaches more about software than any exercise.

**Do something with a real user.** Even one. Watching somebody else use what you
built is the fastest available cure for a set of assumptions you cannot see from
the inside.

## The labs

If you are heading into a data structures course, you are ready for it, and it is
worth saying which parts of this book will carry you.

Chapters 15 through 18 gave arrays, lists, maps, sets, and their costs. Chapter 13
gave recursion and Chapter 12 gave the stack it runs on. Chapters 19 through 23
gave classes, interfaces, generics, and the design judgment to use them. Chapter 32
gave the analysis you will be asked to perform. Chapter 25 gave you an interpreter,
which is a larger program than most such courses assign.

What you will meet that this book did not cover: linked structures written by
hand, trees and balanced trees, graphs and their traversals, hash tables from the
inside, and heaps. All of them are Chapter 16's representation invariant applied to
a new shape, and the analysis is Chapter 32's applied to a new recurrence.

You have the foundation those things sit on. That was the point.

Next: the last word.
