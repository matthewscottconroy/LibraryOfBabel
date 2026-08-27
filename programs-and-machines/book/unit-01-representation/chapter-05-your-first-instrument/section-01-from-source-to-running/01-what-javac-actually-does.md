# What javac Actually Does

A processor executes instructions encoded as bit patterns — the fourth reading in
Chapter 1, where a pattern was "a machine instruction on some processor
somewhere". Your source file is text, encoded in UTF-8 as Chapter 4 described.
Something has to bridge the two, and that something is a **compiler**.

## Translation, not execution

The distinction worth fixing first: a compiler does not run your program. It
translates it, and produces a file. Running is a separate act, performed later,
possibly on a different machine.

```
Hello.java   ──javac──▶   Hello.class   ──java──▶   output
 (text)                   (bytecode)              (behavior)
```

Two commands, two steps:

```
javac Hello.java     # translate; produces Hello.class
java Hello           # run the translated form
```

Note the asymmetry, which confuses everyone once. You give `javac` a *filename*,
including the `.java`. You give `java` a *class name*, with no extension. The
first is a file operation; the second names a thing inside the running system,
and the `.class` file is found for you.

## What the compiler checks

Compilation is not only translation. `javac` refuses a great many programs, and
what it refuses tells you what kind of language Java is.

**Syntax.** Is this a well-formed Java program at all? A missing semicolon or an
unbalanced brace stops it here.

**Types.** Does every operation make sense for the values it is given? If you
declare a variable to hold whole numbers and then assign a piece of text to it,
`javac` refuses. This is the check that makes Java a **statically typed**
language: types are settled before the program runs, not discovered while it
runs.

**Definite assignment.** Is every local variable given a value before it is read?
Java will not let you read a local variable that might not have been set.

**Reachability.** Is there code after a `return` that can never execute? That is
an error, not a warning, on the grounds that it is always a mistake.

The general principle is worth naming, because it is a design philosophy and not
merely a list. **Java prefers to fail at compile time rather than at run time.**
An error caught by `javac` costs you a few seconds. The same error caught at run
time might cost you a production incident at three in the morning.

This is why the language makes you write things that feel redundant — declaring
types you think are obvious, catching exceptions you think cannot happen. Each
piece of ceremony buys a category of error that cannot survive to runtime. Whether
the trade is worth it is a real argument with reasonable people on both sides, and
you will form your own view. What matters now is that it *is* a trade, made
deliberately.

## What comes out

`javac Hello.java` produces `Hello.class`, and the contents are not machine code
for your processor. They are **bytecode** — instructions for an imaginary machine
that does not exist in silicon.

That sounds like a strange thing to produce. The next lesson is about why it is
one of the better ideas in the language.

## Running from source directly

Since Java 11 there is a shortcut for single-file programs:

```
java Hello.java
```

This compiles in memory and runs immediately, producing no `.class` file. It is
convenient for experiments and for everything in this chapter, and I will use it.

Do not let it hide the two-step model from you, though. The compilation still
happens; it is merely not left on disk. As soon as your program grows past one
file you will be back to `javac` and `java`, and by then the distinction needs to
be solid.
