# Designing Object Systems

You now have the whole vocabulary: classes, fields, methods, encapsulation,
equality, inheritance, polymorphism, interfaces, abstract classes, enums, records.
Six chapters of constructs.

This chapter is about choosing among them, which is a different skill and a harder
one. There is no compiler for design. A badly designed program runs exactly as
fast as a well designed one, passes the same tests, and produces the same output.
The difference shows up only later, when someone has to change it — and that
someone is usually you, six weeks on, having forgotten why.

So the question this chapter asks is not *does it work* but **what happens when
the requirements change**, and every principle here is an answer to it.

Section 23.1 is about responsibility: deciding which class should know a
particular fact. It contains the chapter's central argument, promised since
Chapter 21 — that composition is right far more often than inheritance, with
Bloch's demonstration of a set that miscounts because it inherited. It closes with
coupling and cohesion, which Chapter 14 introduced as vocabulary and which can now
be taken seriously.

Section 23.2 is about doing the work before the code. A little UML, treated as
sketching rather than as documentation, and then a design worked from a problem
statement to a set of types, with the wrong turns left in — because the useful
part of watching someone design is watching them change their mind.

A caution about the material. Object-oriented design accumulated a great deal of
doctrine in the 1990s, some of it valuable and some of it cargo cult, and you will
meet people who apply it with a rigor the original authors never intended. The
principles here are heuristics that pay for themselves. They are not laws, and a
design that follows all of them and is hard to read has failed at the only thing
that mattered.

The single question underneath all of it, and the one worth keeping when the
vocabulary fades: **what is likely to change, and have I put a boundary there?**
