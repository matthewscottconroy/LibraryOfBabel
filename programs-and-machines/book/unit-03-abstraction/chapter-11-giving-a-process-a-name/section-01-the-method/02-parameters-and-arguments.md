# Parameters and Arguments

A method that did the same thing every time would be a poor sort of abstraction.
`largest` is worth having because you can point it at any array you like.

Information gets in through **parameters**.

```java
static int square(int n) {
    return n * n;
}
```

`n` is the parameter — a variable, declared in the header, that will be handed a
value when somebody calls.

```java
int result = square(7);
```

`7` is the **argument** — the actual value at the actual call.

Two words for what looks like one thing, and the distinction earns its keep. The
parameter is written once, in the definition. The argument is different every time
anybody calls. One `n`, a thousand different sevens.

## A parameter is a local variable that was filled in for you

Hold that sentence and most of the awkward questions answer themselves.

When `square(7)` runs, a fresh variable `n` is born, holding a **copy** of 7. It
lives exactly as long as the call does, and when the method returns it is gone —
along with everything else the call was keeping.

So you can assign to it, and it changes nothing anywhere else:

```java
static void tryToChange(int n) {
    n = 99;
}

int x = 5;
tryToChange(x);
System.out.println(x);      // 5
```

Five. The method received a copy, scribbled on the copy, and the copy was thrown
away when it returned. `x` never knew anything had happened.

This is **pass by value**, and Java does it for absolutely everything, without
exception. For primitives it is exactly as simple as it looks. For objects the
same rule produces an answer that catches nearly everyone, and it has a chapter of
its own coming.

One habit while we are here. Assigning to a parameter is legal and mostly a bad
idea, because a reader arriving mid-method expects the parameter to still hold
what the caller sent. If you want a version you can move around, take a copy and
say so:

```java
static int countdown(int start) {
    int n = start;         // now it is obvious that start is untouched
    while (n > 0) { ... }
}
```

## More than one

Comma-separated, and matched to arguments **by position**:

```java
static int max(int a, int b) {
    return a > b ? a : b;
}

max(3, 9);      // a is 3, b is 9
```

Position is the whole mechanism. There is nothing else. Which is fine with two,
and becomes a genuine hazard the moment several parameters share a type:

```java
drawRectangle(10, 20, 5, 3);
```

Quick — which of those is the height?

You cannot tell, and neither can the compiler, and neither can the person who
swapped two of them last Tuesday. That call compiles perfectly and draws the wrong
rectangle, and there is no error anywhere to lead you to it.

Some languages let you write `drawRectangle(x: 10, y: 20, ...)` and settle the
question at the call site. Java does not. What you have instead are three partial
defences:

**Take fewer parameters.** A method that wants six is usually a method that wants
restructuring. Somewhere around three or four is where you should start feeling
uneasy.

**Use types the compiler can tell apart.** It cannot catch two swapped `int`s. It
can catch a swapped `int` and `String` instantly. Later you will be able to make a
`Width` and a `Height` that refuse to be exchanged, and that is a real fix rather
than a mitigation.

**Be relentlessly consistent about order.** If one method takes `(row, column)`,
they all take `(row, column)`. It costs nothing and it means a reader's instinct is
usually right.

## How many is too many

Roughly: zero to two is comfortable, three is fine, four raises an eyebrow, five
or more is telling you something.

What it is usually telling you is one of two things. Either the method is doing
too much and wants splitting — or several of those parameters travel everywhere
together and are secretly one thing that does not have a name yet. A `Rectangle`
instead of four numbers. Unit V is where you get to make that second one, and it
is more often the right answer than people expect.

## When you genuinely do not know how many

Sometimes the count really is open:

```java
static int sum(int... xs) {
    int total = 0;
    for (int x : xs) total += x;
    return total;
}

sum(1, 2, 3, 4);      // 10
sum();                // 0
```

The `...` gathers whatever was passed into an array. This is how `System.out.printf`
manages to accept any number of values after its format string, which you have been
using since Chapter 5 without asking how.

Use it where the count is honestly unbounded. It is not a way of avoiding the
decision about what your parameters are.

## One word of the incantation

You have been typing this since your first program:

```java
public static void main(String[] args)
```

And we can now take a piece of it apart. `String[] args` is a parameter — an array
of strings, holding whatever somebody typed on the command line after the class
name.

```
$ java Hello Alice Bob
```

`args` arrives with two elements in it, `"Alice"` and `"Bob"`.

So `main` is not a magic word. It is a method, like the ones you have been writing
in this lesson, and the JVM is the thing that calls it. It takes a parameter for
the same reason `square` does: whoever starts the program might have something to
say to it.

That is one word explained. `static`, `void` and `public` are still outstanding,
and two of them come due before this chapter is over.

Next: how information gets back out.
