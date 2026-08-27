# Recursion

A method may call itself.

That sentence provokes one of two reactions. Either it sounds circular and
impossible — how can a thing be defined in terms of itself without going round
forever? — or it sounds like an obscure trick with no practical use.

Both reactions are common and both are wrong, and clearing them up is what this
chapter is for.

The mechanism is already in place. Chapter 12 established that a frame belongs to
one *execution* of a method, not to the method. So two executions of the same
method have separate frames with separate variables, and there is nothing unusual
about one of them being started by the other. The stack handles it without
modification.

What is genuinely new is the way of thinking, and it is worth the effort because
some problems become dramatically easier under it. A tree has branches which are
trees. A directory contains directories. An arithmetic expression contains
expressions. When a structure is defined in terms of itself, a procedure defined
in terms of itself fits it exactly, and the alternative — managing your own stack
of pending work — is longer and harder to get right.

Chapter 9 asked you to prove a loop correct with an invariant: establish it, show
each iteration preserves it, and read off the result when the loop stops. I said
then that this is mathematical induction.

Recursion is the same induction, written so that the correspondence is visible
rather than implied. The base case *is* the base case. The recursive call *is* the
inductive step. When you write a recursive method, you are writing a proof by
induction, and the reason recursive code is often short is that the proof was
short.

That is not a metaphor and it is worth taking literally. It gives you a technique
for writing recursion that does not involve tracing anything.

**Self-Reference** covers the base case, the leap of faith that makes recursive
thinking possible, and the induction connection.

**Shapes of Process** is about the difference between a recursive *procedure* and
a recursive *process* — a distinction that explains why one recursive method runs
in constant space and another exhausts the stack, and why the naive Fibonacci is
catastrophically slow while the naive factorial is fine.

The natural response to a recursive method is to trace it: what calls what, in
what order, with what values.

Do that once, for a small case, to convince yourself the machine is not doing
magic. Then stop.

Tracing does not scale — five levels deep is already beyond comfortable — and more
importantly it is not how recursive code is written or read by people who are
fluent. They use the induction argument, which requires holding two things rather
than *n* things.

Section 13.1.2 teaches that explicitly, because it is the thing that turns
recursion from bewildering to obvious, and almost nobody is taught it directly.
