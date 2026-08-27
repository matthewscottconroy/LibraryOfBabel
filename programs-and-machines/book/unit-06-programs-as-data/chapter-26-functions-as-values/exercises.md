# Exercises

**26.1** Declare a functional interface `IntOp` and write `mapArray`. Use it with
three different lambdas. Then write the same three as anonymous classes and count
the lines you saved.

**26.2** Add a second abstract method to an interface marked `@FunctionalInterface`
and read the error. Then remove the annotation and try to use a lambda for it.
Which error is more useful?

**26.3** Write `IntOp adder(int n)` returning a lambda that captures `n`. Confirm
`adder(5).apply(1)` is 6 and `adder(100).apply(1)` is 101. Then explain where `n`
lives after `adder` has returned, in terms of Chapter 12.

**26.4** Try to modify a captured local variable inside a lambda. Read the error.
Then explain, in two sentences, why copying rather than referencing forces the
restriction.

**26.5** Reproduce the parallel counter from Section 26.1.2 with a million
elements. Run it five times and report all five answers. Then say what property of
the lambda the stream assumed and did not check.

**26.6** Rewrite each of these as a method reference: `s -> s.length()`,
`(a, b) -> Integer.sum(a, b)`, `() -> new ArrayList<String>()`,
`s -> System.out.println(s)`. Say which of the four kinds each one is.

**26.7** Write `map`, `filter` and `reduce` by hand, as in Section 26.2.1. Then
compute the sum of the squares of the even numbers from 1 to 10 with them, and
confirm you get 220.

**26.8** Use `peek` to count how many elements a pipeline actually examines.
Compare `findFirst` on a filter that matches early against one that matches late,
over a list of a thousand. Report both counts.

**26.9** Write a pipeline that groups a list of words by their first letter and
counts each group. Then write the same thing as a loop with `computeIfAbsent`.
Which would you rather maintain, and which would you rather explain?

**26.10** *Measurement.* Reproduce Section 26.2.3's benchmark on your machine: ten
million ints, filtered and squared and summed, as a loop, an `IntStream`, and a
`Stream<Integer>`. Report all three. Then explain the gap between the second and
third without using the word "streams".

**26.11** Take a method you wrote in an earlier chapter that both computes
something and modifies state. Split it into a pure function and a small caller
that does the modifying. Say what became testable that was not before.

**26.12** *Longer.* [carries forward] Return to Chapter 25's interpreter. Add
closures — store the defining environment in the `Procedure` record — and then add
first-class procedures, so a procedure can be passed as an argument and returned
as a value. Your language will then have what this chapter describes, and you will
have implemented it. Chapter 27 assumes the interpreter still runs.
