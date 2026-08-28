# Passing Behavior

In the language you just built, a procedure was a value — a record you could store
in a map and hand around like a number.

Java has had the same thing since 2014, and this section is about what it means.
The short version: you can write a method whose *algorithm* is fixed and whose
*detail* is supplied by whoever calls it. Chapter 11 let you abstract over values.
This lets you abstract over the process itself.

Three lessons.

Functional interfaces first — what a lambda actually *is*, which is an
implementation of a one-method interface and nothing more exotic. Then lambda
syntax itself, and closures, which Chapter 25 defined and said our language did
not have. Then higher-order methods: methods that take behavior, return behavior,
or both.
