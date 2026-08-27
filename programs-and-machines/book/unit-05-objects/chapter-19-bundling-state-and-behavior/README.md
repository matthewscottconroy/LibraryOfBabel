# Bundling State and Behavior

Here is the situation Unit IV left us in.

```java
String[] names = new String[100];
int count = 0;
```

Two variables with a relationship between them — `count` is how many of `names`
are in use — and that relationship is a representation invariant. Every operation
must preserve it. Nothing in the code stops any part of the program from writing
`names[50] = "x"` and breaking it.

A **class** fixes that by putting the two variables and the operations on them
inside one unit, and making the variables unreachable from outside. The set of
code that could break the invariant becomes the code you can see on one screen.

That is what a class is for. Everything else it does is secondary, and if you keep
that sentence in view the rest of the syntax stops looking arbitrary.

The chapter builds one properly — a bank account, because money has an invariant
everyone agrees about — and then takes apart what `private`, `public`, and
`static` actually mean. By the end the incantation from Chapter 5 will be fully
explained, and you are asked to go back and compare it with what you guessed.

The first section is the class itself: objects as little machines, fields,
constructors, and methods that guard state. The second is encapsulation — what
`private` is for, what a public surface is, and what `static` means, which is a
different idea that shares the same syntax and confuses everyone once.
