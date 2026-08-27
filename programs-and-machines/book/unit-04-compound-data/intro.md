# Compound Data

Every program in this book so far has handled one thing at a time. One number,
one character, one boolean. Even the arrays that have appeared in examples were
borrowed from a chapter that had not happened yet.

Real programs handle thousands of things at once, and this unit is about what
that requires.

## The problem

Suppose you need the average of a hundred test scores. With what you have, you
would need a hundred variables:

```java
int score1 = 87;
int score2 = 92;
// ... ninety-eight more
```

Which is absurd for three separate reasons, and they are worth separating because
they point at three different needs.

**You cannot write a loop over them.** There is no way to say "the next one" —
`score1` and `score2` are unrelated names as far as the machine is concerned.
Chapter 6's transitions can consult state, and here there is no state that
represents *position among the scores*.

**The count must be known when you write the program.** A hundred and one scores
means editing the source. But the number of scores is data, and data is not
supposed to be in the source.

**There is no name for the collection.** You cannot pass "the scores" to a method,
because there is no such thing — there are a hundred things that you happen to
think of together.

Every part of this unit addresses one of those.

## The deeper problem

There is a fourth issue, and it is the one that makes this a unit rather than a
chapter.

Suppose you solve the first three: you have an array of a hundred scores. Now what
*is* it? A pile of numbers. Nothing about it says the numbers are scores, that
they lie between 0 and 100, that they are in the order the students sat the exam,
or that a score of −1 means absent.

Those facts are true and they are not in the data. They live in your head, or in a
comment, or in the code that happens to respect them — and any part of the program
can violate them without anything noticing.

That is Chapter 1's argument arriving for the fifth time. **A collection of values
means whatever we have agreed it means**, and the agreement is not in the bits.
The remedy this unit builds is the **abstract data type**: a way of stating the
agreement and making it enforceable rather than merely hoped for.

## What is here

**Chapter 15 — Many Things at Once.** Arrays. Contiguous storage, the index as
arithmetic, bounds checking, and two-dimensional data. The mechanism is Chapter
1's fixed width paying off directly.

**Chapter 16 — Contracts for Data.** The abstract data type and the
representation invariant — Chapter 11's contract, applied to data rather than to
process. Also the wrapper classes, autoboxing, and `null`, which is the most
consequential mistake in the language's ancestry.

**Chapter 17 — Growing Collections.** `ArrayList` and the collections framework.
What happens when the size is not known in advance, how growth actually works, and
generics as a promise about content.

**Chapter 18 — Text as Data.** Strings. Immutability and why it is the right
choice, building text efficiently, and turning characters into meaning — which is
Chapter 4's encodings, now something you manipulate rather than merely store.

## What changes here

Until now, a variable held a value. From Chapter 15 onwards, a variable usually
holds a *reference to a thing that holds values*, and Chapter 12's aliasing stops
being a preview and becomes the daily situation.

If Chapter 12's three demonstrations were not entirely comfortable, this is the
moment to reread them. Everything in this unit assumes them.
