# Exercises

**22.1** Write an interface `Shape` with `double area()` and `String name()`, then
three unrelated classes implementing it. Put them in a `Shape[]` and print each
one's name and area. Confirm that removing `name()` from one class stops the
program compiling, and read the error message.

**22.2** Add a `default` method `describe()` to your `Shape` interface that returns
`name() + " has area " + area()`. Do not touch the three classes. Confirm all
three gain the method. Then override it in one of them and confirm the other two
are unaffected.

**22.3** Write a class implementing three interfaces at once. Then try to write a
class extending three classes and read the error. Explain in two sentences why
Java permits the first and not the second, in terms of state.

**22.4** Reimplement the `Account` example from Section 22.1.2, then add a third
subclass with a flat fee of 10. Verify that depositing 500 into each of the three
gives 1500, 1495 and 1490. You should not have modified `deposit`.

**22.5** Convert your `Account` hierarchy to use an interface plus a helper class
instead of an abstract class. Which version is shorter? Which would you rather
maintain if someone else needed to add a fourth kind of account? Write three
sentences.

**22.6** Take a class from an earlier chapter that has fields, a constructor,
getters, `equals`, `hashCode` and `toString`. Rewrite it as a record. Count the
lines you deleted. Then run any test you had for it and confirm the behavior is
unchanged.

**22.7** Write `record Fraction(int num, int den)` with a compact constructor that
rejects a zero denominator and reduces the fraction to lowest terms. Confirm that
`new Fraction(2, 4)` equals `new Fraction(1, 2)`. Explain why that would be much
harder to guarantee with a mutable class.

**22.8** Write an `enum Suit` with four constants, each carrying a symbol and a
color. Print all four with `values()`. Then write a `switch` over a `Suit` with no
`default`, add a fifth constant, and read the compiler error. Say what that error
saved you from.

**22.9** Benchmark `EnumSet` against `HashSet` for an enum with eight constants:
build each, then test membership ten million times. Report both times. Then
explain the difference in terms of what an `EnumSet` actually stores.

**22.10** *Design.* You are representing an HTTP response. Sketch it using a
record for the body and headers, an enum for the status class, and an interface
for anything that can be written to an output stream. Say what each of the three
constructs is buying you, specifically.

**22.11** *Longer.* [carries forward] Return to Exercise 21.9's library. Implement
it with an interface `Catalogued`, a record for the shared bibliographic data, and
an enum for the item kind. Then write a method that sorts a mixed list by title
without knowing any concrete type. Keep the code; Chapter 23 revisits the design.
