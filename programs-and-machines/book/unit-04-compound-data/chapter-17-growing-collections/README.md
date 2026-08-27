# Growing Collections

An array's size is fixed at creation, and Chapter 15 explained why: consecutive
storage means the space after the array belongs to something else.

That is a genuine obstacle. Most of the time you do not know how many things there
will be. Lines in a file, words in a document, orders placed today, players in a
game — the count is data, and data is not available when you write the program.

This chapter is about what you use instead, and — more usefully — about how it
works, because the collections library offers a dozen types and choosing well
requires knowing what each is doing underneath.

## The plan

**When the Size Is Not Known** starts with `ArrayList`, which is the one you will
use most, and then does something unusual for an introductory treatment: it shows
you how growth actually works. The answer is a nice piece of engineering, it
explains a performance characteristic that would otherwise look magical, and it is
a good example of an invariant from Chapter 16 doing real work.

Then the three shapes the library provides — `List`, `Set`, and `Map` — which
answer three different questions and are chosen by asking which question you have.

**Generics and Iteration** covers the angle brackets you have been seeing in
examples, what they promise, how to walk a collection safely, and a short guide to
choosing.

## What this chapter assumes

Chapter 16, and specifically two things from it.

Collections hold **objects**, so a collection of numbers holds `Integer` rather
than `int`, and everything Section 16.2.2 said about autoboxing applies constantly
in this chapter.

And a collection is an **abstract data type**. `List` is an interface — a set of
operations with contracts — and `ArrayList` and `LinkedList` are representations.
Section 16.1.1 argued that programming against the interface is what makes the
representation replaceable, and the collections library is the largest worked
example of that argument in the standard library.
