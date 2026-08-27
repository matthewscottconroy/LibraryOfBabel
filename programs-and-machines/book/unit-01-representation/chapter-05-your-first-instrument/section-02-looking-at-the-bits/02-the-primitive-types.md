# The Primitive Types

Here is where four chapters of theory become vocabulary.

Java has eight **primitive types**. Each is a fixed-width box with an agreement
about how to read it — which is to say, each is one of the things Chapters 2 and
3 described, with a keyword attached.

| Type | Bits | Holds | Range |
|---|---:|---|---|
| `byte` | 8 | two's complement integer | −128 .. 127 |
| `short` | 16 | two's complement integer | −32,768 .. 32,767 |
| `int` | 32 | two's complement integer | −2,147,483,648 .. 2,147,483,647 |
| `long` | 64 | two's complement integer | ±9.22 × $10^{18}$ |
| `float` | 32 | IEEE 754 single precision | ±3.40 × $10^{38}$, ~7 digits |
| `double` | 64 | IEEE 754 double precision | ±1.80 × $10^{308}$, ~16 digits |
| `char` | 16 | unsigned UTF-16 code unit | 0 .. 65,535 |
| `boolean` | — | `true` or `false` | — |

You should recognize every row. The integer ranges are the asymmetric two's
complement ranges of Chapter 2, and you can now derive any of them rather than
looking them up. The floating-point rows are Chapter 3's format at two sizes.
`char` is Chapter 4's 16-bit decision.

## Notes on the awkward ones

**`boolean` has no defined size.** The specification declines to say, because the
JVM does not have a one-bit storage unit — a lone `boolean` typically occupies a
whole byte or a whole word, and an array of them may be packed. Since you cannot
do arithmetic on booleans in Java, the size is not observable, so the
specification leaves implementations free.

**`char` is unsigned**, alone among the integer types. This is why `char` runs 0
to 65,535 rather than −32,768 to 32,767: there is no such thing as a negative
character. It does mean `char` behaves differently from `short` in arithmetic,
which occasionally surprises.

**`long` literals need an `L`.** Write `2147483648` and the compiler rejects it —
the literal is an `int` by default and that value does not fit. Write
`2147483648L` and it is a `long`. Use a capital `L`; a lowercase `l` is legal and
looks exactly like the digit 1.

**`float` literals need an `f`.** `3.14` is a `double`; `3.14f` is a `float`. You
will use `double` almost always, and `float` only when memory or bandwidth
genuinely demands it.

## Declaring and assigning

```java
int count = 42;
double price = 19.99;
char grade = 'A';
boolean done = false;
long population = 8000000000L;
```

The type comes first, then the name, then optionally `=` and an initial value.
Chapter 7 takes variables seriously; for now this is enough to write programs.

Note the quoting convention, which trips people: single quotes make a `char`,
double quotes make a `String`. `'A'` is one character; `"A"` is a string that
happens to contain one character. They are entirely different types, and `'AB'`
is an error.

## Arithmetic that will surprise you

Three behaviors follow directly from the last four chapters, and all three catch
beginners.

**Integer division truncates.**

```java
7 / 2      →  3
```

Not 3.5. Both operands are `int`, so the operation is integer division and the
fractional part is discarded — not rounded, discarded, so `-7 / 2` is `-3`.

To get 3.5, make one operand floating point: `7 / 2.0` is `3.5`.

The remainder operator `%` gives what division discarded: `7 % 2` is `1`.

**Integer overflow wraps, silently.**

```java
byte b = 127;
b++;             //  b is now -128
```

Chapter 2, in the flesh. No exception, no warning.

**Division by zero behaves differently for the two kinds of number.**

```java
1 / 0        →  throws ArithmeticException
1.0 / 0      →  Infinity
```

Integers have no representation for infinity, so the operation fails. Floating
point has one, per Chapter 3, so the operation succeeds and returns it. Same
operator, same-looking expression, entirely different outcomes — because the
types are different, and the types determine the agreement.

That last point is the one I most want you to hold on to. In Java, the type of an
expression decides which arithmetic is performed. `/` is not one operation; it is
a family, and which member you get depends on what you handed it.

## Widening and narrowing

Java converts between numeric types automatically when the conversion cannot lose
information — `int` to `long`, `int` to `double`. This is **widening**, and it is
silent because it is safe.

Going the other way can lose information, so Java requires you to ask explicitly
with a **cast**:

```java
double d = 3.99;
int i = (int) d;      //  i is 3 — truncated, not rounded
```

The `(int)` is you taking responsibility. And notice what it does: truncation
toward zero, not rounding. `(int) 3.99` is 3 and `(int) -3.99` is −3.

Narrowing an integer type keeps the low-order bits and discards the rest:

```java
int big = 300;
byte small = (byte) big;    //  small is 44
```

Which is Chapter 2's 200 + 100 example arriving from a different direction. 300
is `100101100`; keep the low eight bits and you have `00101100`, which read as a
signed byte is 44.

Nothing here is new. It is the same fixed-width arithmetic you already
understand, now with syntax.

Next, the instrument.
