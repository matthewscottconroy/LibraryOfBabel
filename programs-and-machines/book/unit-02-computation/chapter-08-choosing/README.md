# Choosing

Chapter 6 gave us transitions that change state. This chapter gives us
transitions that *consult* it — the machine looks at what it holds and decides
what to do next.

This is where a program stops being a fixed sequence and starts being responsive.
The parity machine of Chapter 6 already did it: the table had different entries
for input `0` and input `1`, and that difference is a decision.

The chapter goes deeper than `if` statements, and I want to say why before we
start.

Underneath every conditional is **boolean logic** — a small algebra of true and
false, worked out by George Boole in 1854 for reasons having nothing to do with
machines. In 1937 Claude Shannon noticed that this algebra describes electrical
switching circuits exactly: a switch is open or closed, a proposition is false or
true, and the same rules govern both.

That observation is why computers can be *designed*. Before it, a circuit was
something you built and tested. After it, a circuit was something you could
calculate — simplify with algebra, prove correct, derive from a specification.
Chapter 1 said two states won because two states are robust; Shannon's result is
the other half of the answer, which is that two states are also *calculable*.

So the first section builds the logic, connects it to gates, and shows that the
adder from Chapter 2 can be constructed from them. The second section is Java's
conditionals, which will look — as promised in Chapter 6 — like a transition
table with better notation.

## Why this is not review

If you have programmed before, `if` and `else` are familiar and you may be
tempted to skim.

Three things in this chapter are not obvious even to people who have used
conditionals for years: why `&&` and `&` are different operators and when the
difference bites; why boolean algebra lets you rewrite a condition into a form
that is provably equivalent and much easier to read; and why the same machinery
that computes AND also computes addition.

That third one is the chapter's payoff, and it is worth waiting for.
