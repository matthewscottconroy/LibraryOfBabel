# Important Concepts

**The claim** — a computer holds patterns and changes them, and everything else is
an agreement layered on top. Stated in the preface, and made in eight registers.

**Representation as agreement, traced** — voltage and convention; two's complement
chosen to make one adder do both operations; floating point trading exactness for
range; UTF-8 as a prefix code; a step as an agreement about what changes; an array
as a block plus an addressing rule; a collection as values plus an invariant; an
object as a pattern plus a claim about what it is; a tree as a program because
`eval` treats it as one; a file as bytes plus a format; a connection as a fiction
maintained by two endpoints.

**The eight bugs are one bug** — unsigned misread, `double` equality, wrong
charset, `Integer` cache, mutable key, heap pollution, CSV comma, missing framing.
In each, the reader and the writer were operating under different agreements.

**Agreements are not arbitrary** — there are better and worse ones, and the
difference is what becomes possible afterward. A good representation makes the
operations you need cheap and the errors you fear impossible.

**Choosing a representation is design** — and usually the decision with the longest
reach, because a class can be refactored and a file format that people already have
cannot.

**Abstraction as boundary** — its value is not what is behind it but what it lets
you stop thinking about. Naming a process, the stack that protects a call's
locals, recursion as trusting an unfinished abstraction, the representation
invariant, `private` as the mechanism that makes a boundary real, polymorphism as a
boundary that admits types written later, and an interface as the boundary with
nothing behind it.

**Every abstraction leaks** — `ArrayList` against `LinkedList` at 2,589 ms,
`IntStream` against `Stream<Integer>` at ten times, the memory hierarchy showing
through a model that assumes uniform memory. The response is not distrust but
knowing which leak and where.

**Explaining the layer below is what makes a leak stop being mysterious** — cache
lines for the `LinkedList` result, method tables for dispatch cost, erasure for the
absence of `List<int>`.

**The two ideas are one** — a representation is an agreement about what a pattern
means, an abstraction an agreement about what you may rely on. Both have parties,
and both fail when one side assumes something the other did not promise.

**The single question** — what is agreed, and by whom? Every confusing bug is an
answer you did not have, and every design decision is a choice about what to agree
to.

**Chapter 34's results are the outer limit of the same question** — the boundary of
what can be agreed at all, since some things no finite description reaches.

**What you know** — you can read and write a program of a few hundred lines, you
have built a parser and an interpreter, and you know what to measure and what to
derive.

**What you do not** — maintenance, working with other people, anything large, most
of Java, and running something that has to stay up. The distance between a first
course and competence is mostly experience that cannot be read.

**The habit worth keeping** — when something surprises you, find out why. Not "it
works now"; the actual mechanism. It is the difference between getting steadily
better and plateauing early with a collection of things that seem to work.

**On being confused** — the useful question is never whether you are capable, but
what exactly you expected and what exactly happened instead. That question has a
findable answer, and finding it is the job.

**Take the reasoning, not the conclusions** — the book has opinions and some are
wrong. Where a chapter argued, the argument is what is on offer; where it measured,
the number is one machine on one day.

**The last claim** — the systems you use were designed by people, under
constraints, with trade-offs they could name. They are not natural facts and they
are not beyond you.
