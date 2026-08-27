# Exercises

These build on Chapter 24's parser. If you did not write it, write it now —
Exercise 24.12 said so and meant it.

**25.1** Implement `eval` for `Num`, `Var` and `Bin`, and confirm that
`2 + 3 * 4` gives 14 and `(2 + 3) * 4` gives 20. Then explain, in one sentence,
where precedence is handled — because it is not in `eval`.

**25.2** Add the `%` operator to the tokenizer, parser and evaluator. Report how
many lines you changed in each of the three, and which change was the largest.

**25.3** Remove the division-by-zero check and run `print 1 / 0;`. Describe what
the user of your language sees, and say why that is a failure of the interpreter
rather than of the program.

**25.4** Change every `int` in the evaluator to `long`, then to `BigInteger`. Run
`print 2000000000 + 2000000000;` after each. Then say which arithmetic your
language had before you chose one.

**25.5** Replace `Bin`'s `char op` with an enum. Confirm that the `default` clause
in the operator switch becomes unnecessary and that the compiler now catches an
unhandled operator. Which error would you rather have?

**25.6** Give the tokenizer a set of reserved words, emitting a distinct token kind
for each. Confirm that `print = 5;` now fails with a sensible message rather than
being mysteriously rejected.

**25.7** Add a distinction between declaration and assignment: require `var x = 1;`
the first time and reject `var x` twice. Then find a typo your new rule catches
that the old one silently accepted.

**25.8** Add a `while` loop as a statement. Then write factorial with it and
compare with the recursive version. Which was easier to add to the interpreter, and
which is easier to read in the language?

**25.9** Write a static checker: a pass over the tree, after parsing and before
running, that reports every undefined variable and every wrong-arity call. Run it
on a program with three errors and confirm it reports all three, where the
evaluator would have stopped at the first.

**25.10** Instrument `eval` to count node visits. Report the counts for `fib(10)`,
`fib(15)` and `fib(20)`. Then say what kind of growth that is, and check your
answer against Chapter 32 when you get there.

**25.11** *Longer.* Add closures. Allow `def` inside a procedure body, store the
defining environment in the `Procedure` record, and use it as the parent in
`apply`. Then write a procedure that returns a procedure and confirm the inner one
still sees the outer one's parameter. This is about six lines and it is the single
largest capability the language is missing.

**25.12** *Longer.* Add a second value type — strings, say, with `+` meaning
concatenation. You will find that `eval` can no longer return `int`. Report every
place that had to change, and then say what a type checker would be for.

**25.13** [carries forward] Keep the interpreter. Chapter 27 uses it as the example
for reflection, and Chapter 32 uses its measured costs. If you have added features
of your own, keep those too.
