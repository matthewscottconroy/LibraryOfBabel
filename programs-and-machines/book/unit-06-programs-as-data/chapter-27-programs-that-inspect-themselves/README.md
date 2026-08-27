# Programs That Inspect Themselves

Chapter 25 wrote a program that reads another program. This chapter closes the
unit with the version that reads *itself*.

A running Java program can ask what classes it contains, what fields and methods
they have, what modifiers and annotations they carry — and then read those fields,
call those methods, and construct those objects, all by name, all decided while
the program runs. That capability is **reflection**, and it is the last form of
this unit's idea: the program as data, where the data is now the program itself.

It is also how a great deal of the software you use works. JUnit finds your tests
by looking for a `@Test` annotation. Spring wires objects together by examining
constructors. Jackson serializes an object by walking its fields. Every one of
those tools was written before your classes existed and works on them anyway,
which is not possible without reflection.

Section 27.1 covers the mechanism — `Class` objects, `Field`, `Method`, invoking
by name — and then what it costs, which is measurable and larger than people
expect.

Section 27.2 turns to the two places where Java's type system reaches its limits.
**Erasure** is the one that will surprise you: the generics of Chapter 17 exist
only at compile time, and a `List<String>` at run time is a `List` with nothing to
say about strings. Then **annotations**, which are the mechanism by which extra
information is attached to code for other programs to read — and which is finally
where `@Override` gets explained, three chapters after you were told to write it
on everything.

A warning that belongs at the top. Reflection defeats the guarantees the rest of
this book has been building. It reads private fields. It calls private methods. It
assigns to `final`. Section 27.1.1 demonstrates all three in eight lines, and the
demonstration should be uncomfortable — Chapter 19 argued that a boundary you
cannot reach around is what makes reasoning possible, and this reaches around it.

The resolution is not that encapsulation was a lie. It is that reflection is a
tool for writing *frameworks*, which need to work on classes they have never seen,
and that using it in ordinary application code is nearly always a mistake. That
distinction is the chapter's practical content.
