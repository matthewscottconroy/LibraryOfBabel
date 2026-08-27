# Contracts for Data

Chapter 15 gave you a hundred numbers in a row. This chapter asks what they mean.

An array of `int` is a pile of values. Nothing in it says they are test scores,
that they lie between 0 and 100, that they are ordered by student, or that −1
means absent. Those facts are true and they are not in the data — they live in
your head, in a comment, or in whichever code happens to respect them.

Anything that can reach the array can violate them, and nothing will notice.

This chapter is about closing that gap. The tool is the **abstract data type**:
state the agreement about what a collection of values means, and make it
enforceable rather than hoped for.

## The connection

Chapter 11 gave a method a contract — a precondition and a postcondition — and
observed that this was Chapter 9's loop invariant at a different scale. I promised
a third scale, and this is it.

A **representation invariant** is a claim about a data structure's contents that
must be true whenever anyone outside can observe it. Every operation may assume it
on entry and must restore it on exit.

Three scales, one technique:

| scale | claim | obligation |
|---|---|---|
| loop (Ch. 9) | invariant | each iteration preserves it |
| method (Ch. 11) | contract | caller establishes, method ensures |
| data (Ch. 16) | representation invariant | every operation preserves it |

That is the intellectual centre of the unit, and once you have it, Unit V's
classes become the mechanism for enforcing it rather than a new subject.

## What is here

**The Abstract Data Type** is the idea: separating what a thing *is* from how it
is *stored*, and the invariant that makes the separation worth having. Java's
mechanism for enforcement arrives in Chapter 19; here we establish what it is for,
because a mechanism whose purpose you do not understand is a mechanism you will
misuse.

**Boxes Around Primitives** is the concrete case study. The wrapper classes —
`Integer`, `Double` and the rest — exist because Java's primitives are not objects
and the collections of Chapter 17 hold only objects. Autoboxing hides the
conversion and hides it imperfectly, in ways that produce some of the most
surprising bugs in the language.

And then `null`, which is where the chapter ends and which its inventor has called
a billion-dollar mistake.

## A note on order

This chapter comes before classes, which may seem backwards — how can we discuss
data abstraction before the construct that provides it?

Deliberately. Chapter 19 will show you `private` and constructors and accessors,
and if you meet them without knowing what problem they solve, they look like
ceremony. Everyone who has been taught to write a getter for every field without
being told why has met this failure.

So: the problem first, then the mechanism.
