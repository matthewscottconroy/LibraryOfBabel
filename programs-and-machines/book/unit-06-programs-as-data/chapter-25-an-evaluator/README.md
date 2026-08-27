# An Evaluator

Chapter 24 left you with a tree. This chapter makes it run.

By the end you will have an interpreter for a small language — numbers,
arithmetic, variables, procedures, conditionals, recursion — written in Java, in
about two hundred lines. It will compute factorials and Fibonacci numbers and
greatest common divisors. It will report undefined variables and division by zero
and wrong argument counts. It will overflow its stack on an infinite recursion.

It will, in other words, be a programming language, and you will have written it
in an afternoon.

That is the chapter's point, and the point is not the artifact. It is what happens
to your understanding of Java on the way. Every construct you have used for
twenty-four chapters is about to become something you implement:

A **variable lookup** becomes a hash-map access you typed.
A **scope** becomes an object with a pointer to another object.
A **function call** becomes a new environment and a recursive call.
**Shadowing** becomes the order you search two maps in.
An **undefined variable** becomes an `if` that found `null`.
**Infinite recursion** becomes the JVM's stack running out under yours.

None of that is metaphor. The evaluator really does these things, and after
writing it you will not be able to un-know that Java does them too.

Section 25.1 builds the core: the `eval`/`apply` structure that every interpreter
since 1960 has had, environments as the representation of scope, and arithmetic
evaluation over Chapter 24's tree.

Section 25.2 turns it into a language: names, procedure definitions, conditionals,
and then an honest accounting of what was built, what was left out, and what the
difference is between what we wrote and what `javac` does.

The debt is to McCarthy, promised twice already. His 1960 paper defined `eval` for
Lisp — a function that interprets Lisp expressions represented as Lisp data — and
Section 25.1.1 explains why that particular loop keeps being rediscovered.

Two habits, repeated from the unit introduction because this is where they matter.

Keep asking **which level is this?** There are two programs in play: the
interpreter, written in Java, and the program being interpreted, written in ours.
A variable named `x` could be either. Confusing them is the characteristic error
of this material and it happens to everyone.

And **run it constantly.** Every output in this chapter is real. Type the code,
run it, break it, and put it back.
