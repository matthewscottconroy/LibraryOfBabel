# Families of Types

Sometimes one kind of thing is a special case of another. A circle is a shape. A
savings account is an account. A `FileInputStream` is an `InputStream`.

**Inheritance** lets you say so, and **polymorphism** lets code written against
the general type work on any of the specific ones without knowing which.

That is genuinely valuable and it is also the most over-used construct in the
language. This chapter tries to give you both halves: what inheritance does, and
the principle that tells you when using it is a mistake.

The principle has a name and it belongs to Barbara Liskov, who has appeared five
times already for abstract data types and for the argument that restricting
visibility restricts what you must reason about. Here she supplies the test:

> If code works with the general type, it must keep working when handed any
> specific one.

That sounds like it could not fail. It fails constantly, and Section 21.2.3 shows
a case — a square that is a rectangle — where every line compiles, every method
looks right, and a caller gets 25 where they were entitled to 20.

The first section is inheritance itself: how a subclass extends a superclass, what
`super` does, what `@Override` is for, and the fact that every class in Java
already inherits from something. The second is polymorphism, which is the point of
the whole arrangement — one name, several behaviors, resolved while the program
runs rather than while it compiles. That distinction, between what the compiler
decides and what the runtime decides, is the sharpest idea in the chapter and it
completes the contrast Chapter 12 set up with overloading.

A warning before we start. If you have met object orientation before, you were
probably shown `Dog extends Animal` and left with the impression that hierarchies
are the center of the subject. Chapter 23 will argue the opposite — that
composition is right far more often — and this chapter is written to give you the
grounds for that judgment rather than a set of hierarchies to imitate.
