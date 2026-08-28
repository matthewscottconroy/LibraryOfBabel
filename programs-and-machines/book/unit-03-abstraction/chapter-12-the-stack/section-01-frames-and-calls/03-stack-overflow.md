# Stack Overflow

The stack is a fixed region. Push enough frames and it runs out.

```java
static int depth = 0;
static void recurse() {
    depth++;
    recurse();
}
```

Every call pushes a frame and never returns, so the frames accumulate:

```
StackOverflowError at depth ~22447
```

Twenty-two thousand frames on the machine used for this book — the number varies
with the JVM, the platform, and how much each frame holds.

## Why an Error and not an Exception

Java distinguishes them, and `StackOverflowError` is an `Error` rather than an
`Exception`. Chapter 28 covers the hierarchy properly; the short version is that
`Error` signals a condition a program is not expected to recover from.

That is a defensible classification. When the stack is exhausted, you cannot call
a method to handle it — calling a method needs a frame, and frames are what ran
out. Recovery is possible only after unwinding a long way, and by then the
program's state is usually incoherent.

You can catch it, as the demonstration above does, and outside of demonstrations
you generally should not.

## What it means

Almost always: **an unintended infinite recursion**. A method that calls itself
without a base case, or two methods that call each other in a cycle.

The usual causes:

**A missing base case.** The condition that should stop the recursion is absent.
Chapter 13 is about writing it.

**A base case that is never reached.** The condition exists but the recursive call
does not move towards it — `factorial(n)` calling `factorial(n)` rather than
`factorial(n - 1)`.

**Accidental mutual recursion.** `equals` calling `equals` on something that calls
back, or a `toString` that prints a field whose `toString` prints the original.
This one is common in Unit V and the trace makes it obvious once you look.

The stack trace for an overflow is long and repetitive, and that repetition is the
diagnosis. Look at the pattern:

```
	at Thing.compute(Thing.java:14)
	at Thing.helper(Thing.java:22)
	at Thing.compute(Thing.java:14)
	at Thing.helper(Thing.java:22)
	...
```

Two methods alternating tells you it is a mutual recursion between `compute` and
`helper`, and gives you both line numbers.

## Legitimate deep recursion

Sometimes the recursion is correct and merely too deep.

Processing a linked structure a hundred thousand elements long, recursively, will
exhaust the stack even though nothing is wrong with the logic. Three options:

**Rewrite as a loop.** Any recursion can be converted; Chapter 13 discusses when
this is easy and when it is painful.

**Increase the stack size.** `java -Xss4m` gives four megabytes. A legitimate fix
occasionally, and a way of postponing the problem usually.

**Restructure the recursion.** If it is *tail recursive* — the recursive call is
the last thing the method does — it can be mechanically converted to a loop, and
some languages do this for you.

Java does **not** eliminate tail calls. This is a deliberate decision, defended
partly on the grounds that stack traces are more useful when frames are not
removed, and it is a real limitation compared with languages that do. Section
13.2.2 returns to it.

## You have seen this shape four times now

Before we leave the mechanism, step back and look at what kind of limit this is.

The stack's boundedness is why Java can be memory-safe cheaply, and it is also why
some entirely reasonable programs need rewriting. That is a trade, and it is the
same shape as every trade in Unit I: a fixed-size region gives you speed and
simplicity, and takes away the cases that do not fit.

A fixed-width byte. An integer range that wraps. A floating-point grid with holes
between its points. And now a call stack with a floor.

Four instances of one pattern, and if you are starting to expect a fifth, you have
understood the pattern.

Next: what actually gets copied into a frame when a method is called.
