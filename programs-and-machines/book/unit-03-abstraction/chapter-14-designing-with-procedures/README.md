# Designing with Procedures

Chapter 11 showed you how to write a method. This chapter is about deciding what
the methods should be, which is a harder and less teachable question.

I want to be honest about that. There is no algorithm for decomposition. Two
competent programmers given the same problem will produce different divisions,
and both may be good. What exists is a set of heuristics, a vocabulary for talking
about the trade-offs, and a great deal of accumulated experience about which
divisions people regret.

That is worth having even though it is not a procedure. Most of the difference
between code that is pleasant to work with and code that is not lies in
decomposition, and it is a skill that improves with deliberate attention and
stagnates without it.

The chapter also covers testing, which belongs here for a reason worth stating.
Chapter 11 said a method is a contract. A test is **the contract, executed** — an
automatic check that the promise still holds. The two subjects are the same
subject: a method you can trust is a method whose contract is stated and checked.

## What is here

**Decomposition** is about finding the seams — where a computation naturally
divides — and about the principle that a method should have one job. This includes
the practical business of taking a long method apart, which is something you will
do constantly.

**Testing a Method** covers what a test is, how to choose cases that actually find
bugs, and the observation that tests are the most reliable documentation available
because they cannot drift out of date without failing.

## A note on when to do this

Decomposition is often taught as something you do first: analyze the problem,
identify the components, then write them.

That is not how it usually goes, and pretending otherwise sets people up to feel
they are doing it wrong. In practice you frequently write something that works,
notice its structure while writing it, and then reorganize. The reorganizing is
called **refactoring**, and it is normal rather than a sign of failure.

What matters is that the reorganizing happens. Code that was never divided does
not become divided on its own, and the cost of leaving it grows.
