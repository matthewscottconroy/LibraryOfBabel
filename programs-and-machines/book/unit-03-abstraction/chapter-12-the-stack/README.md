# The Stack

A method call is a small miracle and it is worth being curious about it.

The machine is executing instructions in `main`. It reaches a call to `largest`.
It jumps somewhere else entirely, runs a different sequence of instructions using
different variables, and then — this is the part that requires machinery — it
comes *back*, to exactly the right place, with `main`'s variables all where it
left them.

How does it know where to come back to? And where were `main`'s variables while
`largest` was running?

The answer is the **call stack**, and it is one of the small number of mechanisms
worth understanding in detail, because a great deal follows from it: why recursion
works, why a runaway recursion produces the specific error it does, why local
variables vanish between calls, what a stack trace is, and why Java's parameter
passing behaves the way it does.

## The sentence everyone gets wrong

This chapter also settles a question that generates more confusion than any other
in introductory Java.

> Does Java pass objects by reference?

The answer is no, and a great many books, courses, and confident colleagues will
tell you otherwise. What makes it difficult is that the wrong answer *predicts
correct behavior most of the time* — you can hold it for years and only
occasionally be surprised.

Section 12.2 takes it apart carefully. The correct statement is short: **Java is
pass-by-value, always; for objects, the value that is passed is a reference.**
Every consequence follows from that one sentence, including the two behaviors
that seem to contradict each other.

## What is here

**Frames and Calls** builds the mechanism. The call frame, the stack discipline
that makes returning possible, and what happens when the stack runs out.

**What Gets Passed** uses the mechanism to settle parameter passing, and covers
overloading — several methods sharing a name, resolved by their parameters.

## Why this level of detail

A fair question, since you can write working methods without any of it.

Because the alternative is a set of memorized rules that occasionally produce
wrong predictions and cannot be repaired. "Primitives are copied but objects are
not" is such a rule: it works until you assign to a parameter, at which point it
predicts the wrong thing and you have nowhere to go.

One mechanism, understood, replaces a dozen rules and never runs out.
