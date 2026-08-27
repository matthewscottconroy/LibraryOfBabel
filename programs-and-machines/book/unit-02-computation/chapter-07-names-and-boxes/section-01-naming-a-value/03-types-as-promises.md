# Types as Promises

```java
int score = 0;
```

Unit I told you what `int` means to the machine: 32 bits, two's complement. This
lesson is about what it means to the *compiler*, which is a different and equally
useful question.

## A type is a promise you make

Writing `int score` is a commitment. You are telling the compiler: this cell will
only ever hold whole numbers in the `int` range, and I want you to hold me to it.

The compiler then does hold you to it:

```java
int score = 0;
score = "hello";     // error: incompatible types: String cannot be converted to int
```

That error arrives before the program runs. Chapter 5's principle, again: an
error caught by `javac` costs seconds, and the same error at run time might cost
a great deal more.

## What the promise buys

Three things, and they are worth separating because people usually name only the
first.

**Errors caught early.** The obvious one. A whole category of mistake — using a
value in a way its kind does not support — becomes impossible to ship.

**The compiler knows which operation you meant.** This one is more interesting.
Chapter 5 showed that `/` is a family: integer division for `int` operands,
floating-point division for `double`. The compiler picks the right member by
looking at the types. Without them, either the language must decide at run time —
which costs speed — or the programmer must write different operators for
different kinds of number, which some languages do.

The same applies to `+`, which is addition for numbers and concatenation for
strings. The types are how one symbol means two things without ambiguity.

**Space is known in advance.** The compiler knows an `int` needs 4 bytes and can
lay out memory before the program starts. This is Chapter 1's fixed-width
argument arriving in the compiler: knowing the size in advance is what allows
constant-time access.

## Static typing and its cost

Java is **statically typed**: every expression has a type known at compile time,
fixed before anything runs.

The alternative is **dynamic typing**, where values carry their types and checks
happen during execution. Python and JavaScript work this way. In such a language
you write `score = 0` with no declaration, and `score = "hello"` on the next line
is perfectly legal.

Neither approach is correct in general, and the argument between them is
long-running and genuinely unsettled. What is worth being clear about is the
trade:

Static typing catches a class of error before shipping, documents intent in the
code itself, and enables the compiler to generate faster code. It costs you
verbosity, and it rejects some programs that would have worked.

Dynamic typing is faster to write and more flexible, and defers those errors to
run time — where they are found by tests, or by users.

Java made its choice, and the rest of this book lives inside it. It is worth
knowing that it was a choice.

## Inference

Since Java 10 you may write `var` and let the compiler work out the type:

```java
var score = 0;               // inferred as int
var name = "Ada";            // inferred as String
var total = 0.0;             // inferred as double
```

This is *not* dynamic typing. The type is still fixed at compile time and still
enforced — `score = "hello"` remains an error. The only thing that changed is
that you did not have to type the word `int`.

`var` is useful when the type is long and obvious from the right-hand side, and
harmful when it is neither. Compare:

```java
var x = compute();                          // what is x? no idea
Map<String, List<Integer>> index = ...;     // clear, verbose
var index = new HashMap<String, List<Integer>>();   // clear and shorter
```

I will use explicit types through most of this book, on the grounds that when you
are learning, seeing the type written out is worth the extra characters.

## Constants

Marking a variable `final` promises it will not be reassigned:

```java
final int MAX_ATTEMPTS = 3;
MAX_ATTEMPTS = 4;      // error: cannot assign a value to final variable
```

This is a promise you make to yourself and to every future reader. When a reader
sees `final`, they know that name means one thing throughout its scope, and they
can stop tracking it — which is one fewer thing to hold in their head.

Use it more than you think you need to. The cost is one word; the benefit is
removing a variable from the set of things that might have changed. That set is
what makes code hard to read.

## The connection back

One last framing, because it ties the two units together.

In Unit I, a type was an *agreement about how to read a bit pattern*. In this
chapter, a type is a *promise the compiler enforces*. These are the same thing
from two directions.

The agreement says: these 32 bits are to be read as a two's complement integer.
The promise says: I will only put things in this cell that are meaningful under
that reading, and you may reject anything else.

Which is why a type error is not an inconvenience but a genuine warning. It means
you were about to store something under an agreement that does not cover it —
which, if allowed, is exactly Chapter 1's silent mismatch, the one that produces
`cafÃ©`. Java's type system exists to make that particular silence impossible.

Next: where a name lives and how long it lasts.
