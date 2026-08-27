# Exercises

**21.1** Write `Shape` with an `area()` method, and subclasses `Circle` and
`Square`. Put three of each in a `Shape[]`, loop over it, and print each area.
Then add a `Triangle` and confirm you did not have to change the loop.

**21.2** Take the `describe(Shape)` / `describe(Circle)` pair from Section 21.1.2.
Predict what each of these prints before running it:

```java
Shape a = new Circle(1);
Circle b = new Circle(1);
describe(a);
describe(b);
describe((Circle) a);
```

Explain each answer in terms of declared type versus actual type.

**21.3** Write a class that overrides a method but misspells the name —
`toStrng()` instead of `toString()`. Print the object. Then add `@Override` and
observe the compiler's response. Write one sentence on what `@Override` bought
you.

**21.4** Demonstrate the constructor trap. Write a superclass whose constructor
calls a method, and a subclass that overrides that method to use one of its own
fields. Run it. Explain the output in terms of the order in which the two
constructors run.

**21.5** Reproduce the square-rectangle violation. Then fix it two ways: once by
making both classes immutable, and once by making `Sq` hold a `Rect` in a field
instead of extending it. For each fix, say what a caller can no longer do.

**21.6** Field shadowing. Write `class A { String name = "A"; }` and
`class B extends A { String name = "B"; }`, then:

```java
B b = new B();
A a = b;
System.out.println(b.name);
System.out.println(a.name);
```

Both references point to the same object. Explain why the two lines differ, and
name the mechanism from Section 21.2.2 that this resembles.

**21.7** Look up `java.util.Stack` in the Java documentation and list three methods
it inherits from `Vector` that violate the stack discipline. Then look up
`ArrayDeque` and say what it does instead.

**21.8** Write a static method `name()` in a superclass and one in a subclass, then
call it through a superclass reference holding a subclass object. Explain the
result using the table from Section 21.2.2 — specifically, which `invoke`
instruction the compiler emits.

**21.9** *Design question, no code.* [carries forward] You are modeling a
library. You have `Book`, `Magazine`, `DVD`, and `AudioBook`. Sketch two designs: one using inheritance from
a common `Item`, one using composition around a shared `Catalogued` record. For
each, name one change that would be easy and one that would be painful. Keep your
answer; Chapter 23 asks about it again.

**21.10** *Longer.* Write an `Account` class with a non-negative-balance invariant
and a `withdraw` method that refuses overdrafts. Then write three subclasses: one
that respects the substitution principle, one that strengthens a precondition, and
one that weakens a postcondition. Write a method that takes an `Account` and is
correct for the first and broken by the other two. Show the output.

**21.11** *Measurement.* Adapt the dispatch benchmark from Section 21.2.2 on your
own machine. Compare a call site that sees one implementing class against one that
sees three. Report both times and the per-call difference in nanoseconds. Is the
result on your machine large enough to influence a design decision? Justify the
answer with the number, not with an opinion.
