# Assignment Is Not Equality

```java
n = n + 1;
```

If you have studied any mathematics, that line is nonsense. Subtract `n` from
both sides and you get `0 = 1`. No number satisfies it.

And yet it is one of the most common lines in all of programming. Resolving the
apparent contradiction takes one idea.

## Two different operators, one symbol

In mathematics, `=` states a relationship that holds. `x = 5` is a claim, and it
is either true or false, and it does not do anything.

In Java, `=` is a **command**. It says: evaluate the expression on the right, and
store the result in the cell named on the left. It is an action, performed at a
moment, and it changes the state.

So `n = n + 1` reads as:

1. Look up the current value of `n`.
2. Add 1 to it.
3. Store the result back into `n`.

If `n` held 3, it now holds 4. Nothing was claimed and nothing is contradictory.
The right side was evaluated using the *old* value; the left side received the
new one.

The two sides refer to different moments in time. That is the whole resolution,
and it is worth saying explicitly: **`=` is not symmetric.** `n = 5` is legal;
`5 = n` is not, because 5 is not a cell you can store into. Mathematical equality
does not care about order and assignment cares completely.

## Left and right are different kinds of thing

This asymmetry has a name worth knowing.

The left side of an assignment must be something that *has a location* — a cell
you can write to. The right side must be something that *has a value*.

```java
count = 5;           // ok: count has a location
count = other * 2;   // ok: the right side has a value
5 = count;           // error: 5 has no location
```

A variable can do both jobs: on the left it means "this cell", on the right it
means "the value in this cell". So in `n = n + 1`, the `n` on the left and the
`n` on the right mean different things, which is exactly why the line makes
sense.

## Why the symbol is confusing

Because it was a mistake, and everyone knows it was a mistake.

FORTRAN used `=` for assignment in 1957, and C inherited it, and Java inherited
it from C. Other languages made the other choice: Algol and Pascal use `:=`, and
read aloud as "becomes", which is much clearer.

Java's cost for reusing `=` is that it needed a *different* symbol for actual
equality, and chose `==`. Which produces the single most common typo in the
language:

```java
if (x = 5)     // assignment, not comparison
if (x == 5)    // comparison
```

In C this compiles and is a notorious source of bugs — the assignment produces a
value, which C treats as a condition. Java mostly saves you, because `if` requires
a `boolean` and `x = 5` produces an `int`, so the compiler rejects it.

Mostly. If `x` is a `boolean`, `if (x = true)` compiles cleanly and does the wrong
thing. It is rare, and it is exactly the sort of error that survives review
because it looks right.

## Order of evaluation

One consequence worth having explicitly:

```java
int x = 5;
int y = x;      // y gets a copy of the value 5
x = 10;         // x becomes 10
                // y is still 5
```

`y` received the *value* 5, not a connection to `x`. Later changes to `x` do not
reach it. For primitives this is exactly what the box picture predicts: two boxes,
one value copied from one to the other.

Hold on to how obvious that felt. In Unit V, `y = x` for objects copies the
*reference*, and changing the object through `x` does change what `y` sees. Same
syntax, different outcome, because what is in the box is different. When that
surprises you — and it will — come back to this paragraph.

## Compound assignment

Because `n = n + 1` is so common, there is shorthand:

```java
n += 1;      // same as n = n + 1
n -= 3;      // n = n - 3
n *= 2;      // n = n * 2
n /= 2;      // n = n / 2
n %= 7;      // n = n % 7
```

And for the very common case of adding or subtracting one:

```java
n++;         // increment by 1
n--;         // decrement by 1
```

There is a subtlety with `++` that you should know about and then avoid relying
on. `n++` and `++n` both increment; they differ in what the *expression* produces
— `n++` yields the old value, `++n` the new one:

```java
int a = 5;
int b = a++;    // b is 5, a is 6
int c = 5;
int d = ++c;    // d is 6, c is 6
```

Using this in an expression is legal, compact, and a reliable source of confusion.
My advice is to use `++` only as a statement on its own, where the distinction
does not arise, and to write anything more complicated as two lines. Code that
requires a rule to read is code that will be misread.

There is one hidden benefit to compound assignment worth knowing: `+=` performs
an implicit cast. `byte b = 10; b += 300;` compiles, where `b = b + 300;` does
not. That is convenient and it silently narrows, which is Chapter 2's truncation
arriving unannounced.

Next: what the type in a declaration actually promises.
