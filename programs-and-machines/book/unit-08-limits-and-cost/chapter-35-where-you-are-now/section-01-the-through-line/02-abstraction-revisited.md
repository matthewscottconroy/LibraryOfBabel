# Abstraction Revisited

The book's second idea ran alongside the first from Chapter 11 onward, and it is
the one that took longer to say plainly.

An abstraction's value is not what is behind it. It is what it lets you stop
thinking about — and that framing explains why a method's name matters more than
its body, why `private` is worth the keystrokes, and why an interface with nothing
behind it at all is the purest form of the thing.

The second idea, and it ran alongside the first from Chapter 11 onward.

> An abstraction is a boundary. Its value is not what is behind it — it is what it
> lets you stop thinking about.

## The climb again

**Chapter 11.** Naming a process is not labeling something you already built; it
is the point of building it. A method's name is a promise and its body is nobody's
business, and once the name is good the body can change.

**Chapter 12.** The stack is what makes that possible. Each call gets its own
frame, so a method's locals cannot be disturbed by what it calls — which is the
mechanism underneath the promise.

**Chapter 13.** Recursion is a boundary in time: assume the smaller case works,
handle the base, and stop. The whole technique is trusting an abstraction you are
in the middle of building.

**Chapter 14.** One job per method, and the observation that a comment labeling a
section is a name looking for a method.

**Chapter 16.** The turn. A representation invariant is a boundary around a
promise about data, and its value is that the code which could break it becomes
small enough to check.

**Chapters 19 to 20.** `private` is the mechanism that makes the boundary real
rather than advisory, and Liskov's argument — that a convention anyone can reach
around is not a guarantee — is why it matters. Immutability is the strongest form:
a boundary that cannot be crossed because there is nothing to change.

**Chapters 21 to 23.** Polymorphism is a boundary that lets code work on types
written after it. Interfaces are the boundary with nothing behind it at all, which
is the purest form. And Chapter 23's whole argument — composition over
inheritance, weakest coupling that works — is about keeping boundaries narrow so
that changes stay local.

**Chapters 24 to 26.** A grammar is a boundary between form and meaning. `eval`
and `apply` are a boundary between what an expression is and what a procedure
does. A pure function is a boundary around a computation with nothing leaking out.

**Chapters 28 to 31.** And then the boundaries that are not yours. An exception
crosses one deliberately. A file format is a boundary with your future self. A
model and a view is a boundary you impose so that a change lands in one place. A
thread is a boundary that shared mutable state destroys.

**Chapters 32 to 34.** Finally, the limits of abstraction. Chapter 32's measured
gap between predicted and actual cost is the memory hierarchy leaking through an
abstraction that pretends memory is uniform. And Chapter 34's undecidability is
the statement that the boundary between syntax and behavior cannot be crossed by
any tool at all.

## Every abstraction leaks

Which is the honest ending for this idea.

`ArrayList` and `LinkedList` implement the same interface and Chapter 17 measured
2,589 milliseconds of difference, because contiguity is not in the interface and
the cache does not care about your types. `Stream<Integer>` and `IntStream` do the
same thing, and Chapter 26 measured ten times the cost, because boxing is not in
the abstraction. `Files.lines` and `Files.readString` read the same file, and one
works on ten gigabytes.

The response is not to distrust abstractions — you cannot write a program without
them — but to know **which** leak and **where**. The competent version of this
skill is knowing that `List` hides memory layout, that generics hide erasure, that
the JVM hides the processor, and being able to drop a level when the measurement
disagrees with the model.

That is why this book kept going down. Chapter 15 explained cache lines so that
Chapter 17's `LinkedList` result would make sense. Chapter 21 explained method
tables so that dispatch cost would not be magic. Chapter 27 explained erasure so
that `List<int>`'s absence would have a reason. **The leak is only mysterious if
you have never seen the layer below.**

## The two ideas are one idea

They looked separate for thirty-four chapters and they are not.

A **representation** is an agreement about what a pattern means. An
**abstraction** is an agreement about what you may rely on. Both are agreements;
both have parties; both fail in the same way, when one side assumes something the
other did not promise.

`equals` and `hashCode` are a representation decision — what counts as the same
object — and an abstraction contract — what a `HashMap` may rely on. They are the
same thing described twice, which is why Chapter 20 was hard.

A file format is a representation of data and an interface to your future self. A
grammar is a representation of structure and a boundary between parsing and
meaning. An interface is an abstraction with no implementation and a
representation of a capability.

The single question underneath the whole book, stated once at the end:

> **What is agreed, and by whom?**

Every bug that confused you was an answer you did not have. Every design decision
you will make is a choice about what to agree to. And Chapter 34's results are the
outer limit of the same question — the boundary of what can be agreed at all,
because some things no finite description reaches.

Next: what to do about it.
