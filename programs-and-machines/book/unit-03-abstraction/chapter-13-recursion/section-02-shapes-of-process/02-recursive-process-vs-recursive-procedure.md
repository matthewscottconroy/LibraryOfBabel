# Recursive Process vs. Recursive Procedure

There is a distinction hiding in this chapter that almost nobody points out, and
once you see it a great deal of confusing advice about recursion resolves itself.

**A recursive procedure** is a method whose text contains a call to itself. That
is a syntactic property — you can see it by looking.

**A recursive process** is an execution that accumulates pending work, so that the
machine must remember something at each level and finish it on the way back.

These are not the same, and a method can be one without the other.

## Two factorials

```java
static int factorial(int n) {
    if (n <= 1) return 1;
    return n * factorial(n - 1);
}
```

Recursive procedure, and the process is recursive too. Watch what is pending:

```
factorial(4)
  4 * factorial(3)
      4 * (3 * factorial(2))
          4 * (3 * (2 * factorial(1)))
          4 * (3 * (2 * 1))
          4 * (3 * 2)
          4 * 6
          24
```

The multiplications pile up. Nothing can be completed until the innermost call
returns, and the machine holds *n* pending multiplications. Space grows with the
input — visible on the stack as *n* frames.

Now:

```java
static int factorial(int n) {
    return factHelper(n, 1);
}

static int factHelper(int n, int acc) {
    if (n <= 1) return acc;
    return factHelper(n - 1, n * acc);
}
```

Still a recursive procedure. But look at the process:

```
factHelper(4, 1)
factHelper(3, 4)
factHelper(2, 12)
factHelper(1, 24)
24
```

**Nothing is pending.** Each call does its multiplication *before* recursing and
carries the result along in `acc`. When the base case is reached, the answer is
already there — it is returned directly, with no work to do on the way back.

The state at each step is entirely captured by the two arguments. That is an
**iterative process**: it could be run in constant space, because there is nothing
to remember.

## Tail calls

The second version is **tail recursive**: the recursive call is the very last thing
the method does. Its result is returned unchanged, with no further computation.

That property is what makes the process iterative. If nothing happens after the
call returns, the caller's frame is not needed after the call is made — so a
sufficiently clever runtime can reuse it rather than pushing a new one, turning
the recursion into a loop.

This is **tail-call elimination**, and languages designed with it — Scheme,
Haskell, Erlang — let you write loops as recursions with no space penalty at all.

**Java does not do it.** The accumulator version still pushes *n* frames and still
overflows at the same depth as the first version. The transformation buys you
nothing at runtime in Java.

The stated reasons are that stack traces are more useful when frames are preserved,
and that the JVM's security model has historically inspected the stack. Whether
these outweigh the benefit is argued about; the fact is that Java is on the side
that does not.

## Why the distinction is worth having

Three reasons, even in a language without tail calls.

**It tells you what the space cost is.** A recursive process needs stack
proportional to its depth; an iterative one does not need it in principle, even
though Java charges for it anyway.

**It tells you how easy the loop rewrite is.** A tail-recursive method converts to
a loop mechanically — the accumulator becomes a variable, the call becomes the
next iteration. A non-tail recursion does not, because the pending work has to go
somewhere, and that somewhere is a stack you build yourself.

**It is how you think in other languages.** In Scheme, or in functional Java, this
distinction is fundamental rather than academic.

## Which factorial to write

For Java, honestly: the plain recursive one, or a loop.

The accumulator version is longer, needs a helper method, and buys nothing the
JVM will honor. It is worth knowing about because the *idea* — carry the answer
forward rather than leaving work pending — is one you will use, particularly in
Chapter 26 when reductions appear.

But do not write accumulator-passing Java for performance. It does not do
anything.

## The general shape

The distinction is really about **where the state lives**.

In the first factorial, the pending multiplications are the state, and they live
on the stack implicitly. In the second, the state is `acc`, and it lives in an
argument explicitly.

That choice — implicit on the stack versus explicit in a variable — is exactly the
choice between recursion and iteration. Chapter 12 said the stack is a data
structure the machine gives you free. Using it is recursion. Declining it, and
keeping your state yourself, is a loop.

Neither is more fundamental. What differs is which one makes your problem's
structure visible, which is the subject of the last lesson.
