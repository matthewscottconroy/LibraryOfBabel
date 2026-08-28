# What No Program Can Do

This is the last technical chapter, and it proves something.

Not that a problem is slow, or awkward, or unsolved. That a problem is
**impossible** — that no program, in any language, on any machine, however fast,
however clever, can solve it. Ever.

The problem is this:

> Given a program and its input, determine whether the program eventually stops.

That is the **halting problem**, Turing proved in 1936 that no program can decide
it, and he did so before any computer existed. The proof is about a page and it
needs nothing you do not already have.

Section 34.1 gives it. The setup — a program that reads programs, which is Chapter
25 — then the contradiction, then a careful account of what the result does and
does not mean, because it is one of the most over-claimed results in the subject.

Section 34.2 takes a different route to the same territory. **Kolmogorov
complexity** measures the information in a string as the length of the shortest
program that prints it — Chapter 33's alternative, needing no probabilities. It is
a clean definition, it makes "random" precise, and it is uncomputable, for reasons
closely related to Section 34.1's.

Several debts close here.

Chapter 6 said we would do the halting proof properly, and Chapter 8 said the
knowledge in a machine lives in the arrangement rather than in any part. Chapter 9
posed the Collatz conjecture as an open problem about a six-line loop, and asked
what that means for the claim "this loop terminates". Chapter 24 said there are
more languages than there are finite descriptions of them, and promised a specific
natural language that no program recognizes. Chapter 33 extracted the counting
technique and said Chapter 34 would use it again.

All of them are paid here.

One thing to hold on to. This chapter is not pessimistic. A result that says
something is impossible is a result that stops you looking, tells you which
approximations are the honest ones, and explains why the tools you use behave as
they do — why a compiler warns rather than proves, why a static analyzer has false
positives, why an antivirus scanner cannot be complete. Knowing where the wall is
is more useful than believing there is no wall.
