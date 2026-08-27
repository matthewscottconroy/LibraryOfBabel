# What a Variable Is

```java
int score = 0;
```

Three things happen in that line, and separating them is worth doing once,
carefully.

**A cell of state is reserved.** Somewhere in memory, 32 bits are set aside — the
`int` box of Chapter 2. It has an address, though you will never see it; Java
does not let you.

**A name is bound to that cell.** From here to the end of its scope, the word
`score` refers to that particular cell. The name exists only in the source code:
after compilation, the JVM works with a numbered slot, and the name is retained
only for debugging.

**An initial value is written.** The pattern for 0 goes into the cell.

So a variable is a *name for a cell holding a value*, and all three parts are
separately real. Chapter 6 would say: the cell is part of the machine's state,
the name is a convenience for the programmer, and the initialization was a
transition.

## The box picture, and its expiry date

The usual metaphor is a labeled box you can put things in and take things out of.
It is a good metaphor for primitives and I will use it throughout this unit.

I want to warn you now that it stops being accurate in Unit V. For objects, the
box does not contain the thing; it contains a *reference* to the thing, and two
boxes can hold references to the same object. That is the aliasing that Chapter
6 said this model does not have.

I am flagging this early because the box picture is where a specific
misunderstanding comes from, and it is easier to inoculate than to correct.
Everything in this chapter is about primitives, where the box is literal.

## Declaration and initialization

```java
int a;          // declaration: reserve the cell, bind the name
a = 5;          // initialization: write a value
int b = 5;      // both at once
```

Java is strict about the order. Read a local variable before writing it and the
program does not compile:

```
Uninit.java:4: error: variable x might not have been initialized
```

Note that this is a **compile-time** error. Chapter 5 said Java prefers to fail
early, and this is a good example: the compiler proves that on some path through
the code the variable might be read before assignment, and refuses the whole
program. Languages without this check let you read whatever was left in that
memory by whatever ran before, which produces bugs that change between runs.

The check is a proof, and like all proofs it is conservative. The compiler will
occasionally reject a program that would in fact always assign before reading,
because proving so is beyond what it attempts. When that happens, assigning an
initial value is the cost of the guarantee.

## Fields default, locals do not

There is an inconsistency here that catches everyone once.

Variables declared inside a method — **local variables** — must be assigned
before use, as above. Variables declared in a class — **fields**, which are Unit
V's subject — are given a default automatically: 0 for numbers, `false` for
`boolean`, `null` for references.

```java
static int counter;      // a field: silently 0
```

Why the difference? Because a field's lifetime is not tied to a single path
through the code. An object might be created here and its field read there, with
no way for the compiler to prove an assignment happened in between. Rather than
reject every such program, the language guarantees a defined starting value.

Locals have no such problem — the compiler can see every path through a method —
so the stricter, safer rule applies.

You will hear this described as a wart. I think it is a reasonable answer to two
genuinely different situations, but you do have to know it, because a field that
is silently 0 looks exactly like a field you remembered to initialize.

## Naming

The compiler does not care what you call things. Everyone else does, including
you in six weeks.

Java's conventions: variables and methods in `camelCase` starting lowercase;
classes in `PascalCase`; constants in `UPPER_SNAKE_CASE`. These are conventions,
not rules — the compiler accepts `int XYZ_12`— but conventions in a language this
widely used are close to mandatory, because violating them makes your code
surprising, and surprising code is misread.

More important than convention is that the name should say what the thing *is*:

```java
int d;                    // days? distance? diameter?
int daysUntilExpiry;      // no ambiguity
```

The second is longer and it is strictly better. You will read code far more often
than you write it, and a name is a message to a future reader who lacks your
current context. That reader is usually you.

The one place short names are right is where the scope is tiny and the meaning is
conventional: `i` for a loop index over a few lines is clearer than
`currentIndex`, because every programmer reads `i` instantly. Scope size is the
guide — the further a name travels, the more it must carry.

Next: the operator that is not equality.
