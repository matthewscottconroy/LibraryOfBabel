# Choosing

In 1854 a largely self-taught mathematician named George Boole published a book
announcing that he had found the algebra of human thought. He called it *An
Investigation of the Laws of Thought*, and he meant the title. Reasoning, he
claimed, could be *calculated* — written in symbols, and manipulated by rules, the
way you manipulate numbers.

It was received as an elegant curiosity. There was no machine in the world for it
to be about.

Eighty-three years later a master's student at MIT read Boole and noticed
something the author could not possibly have noticed, because the thing did not
exist yet. Boole's algebra of true and false describes an electrical switch. A
switch is open or closed. A proposition is false or true. The rules are the same
rules. Claude Shannon wrote this down in 1937, and it may be the most consequential
master's thesis ever submitted.

Which is where this chapter is going, and it starts somewhere far more ordinary:
with a machine that looks at what it is holding and decides what to do next.

The transitions of Chapter 6 changed state. These *consult* it. That is the step
where a program stops being a fixed sequence of instructions and becomes something
that responds — though the parity machine was already doing it quietly, since its
table held different entries for input `0` and input `1`, and a difference like
that is a decision in everything but name.

Shannon's observation is the reason computers can be *designed* at all. Before
it, a circuit was something you built and then tested to find out what it did.
After it, a circuit was something you could calculate — simplify with algebra,
prove correct, derive from a specification before anyone picked up a soldering
iron.

Two states won, back in Chapter 1, because two states are robust against noise.
Here is the other half of that answer, and it is the half that mattered: two
states are also *calculable*.

So the first section builds the logic, connects it to gates, and shows that the
adder from Chapter 2 can be constructed from them. The second section is Java's
conditionals, which will look — as promised in Chapter 6 — like a transition
table with better notation.

If you have programmed before, `if` and `else` are familiar and you may be
tempted to skim.

Three things in this chapter are not obvious even to people who have used
conditionals for years: why `&&` and `&` are different operators and when the
difference bites; why boolean algebra lets you rewrite a condition into a form
that is provably equivalent and much easier to read; and why the same machinery
that computes AND also computes addition.

That third one is the chapter's payoff, and it is worth waiting for.
