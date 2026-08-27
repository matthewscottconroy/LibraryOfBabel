# The Base Case

Here is factorial, defined mathematically:

```
0! = 1
n! = n × (n-1)!        for n > 0
```

Note the shape. The second line defines factorial in terms of factorial — which
would be circular except for the first line, which does not.

In Java:

```java
static int factorial(int n) {
    if (n <= 1) return 1;              // base case
    return n * factorial(n - 1);       // recursive case
}
```

```java
factorial(5)      // 120
```

## The two parts

Every recursive method has exactly these:

**The base case** answers directly, without recursing. Here, factorial of 0 or 1
is 1, and we know that without asking anything.

**The recursive case** answers in terms of a *smaller* instance of the same
problem. Factorial of *n* is *n* times factorial of *n*−1.

Both are required, and each fails in a characteristic way.

**No base case** means the recursion never stops. Chapter 12 told you what that
produces: frames accumulate until `StackOverflowError`.

**A recursive case that does not shrink** is the same failure wearing a disguise.
If `factorial(n)` called `factorial(n)`, there is a base case and it is never
approached.

So the requirement is not merely "have a base case". It is:

> **Every recursive call must move strictly closer to a base case, and a base case
> must be reachable from every input the method accepts.**

That is Chapter 9's termination argument, unchanged. The variant here is *n*
itself: a non-negative integer that decreases by one on every call. It cannot
decrease forever, so the recursion stops.

Invariant and variant, contract and precondition, base case and decreasing
argument. The same two obligations keep reappearing because they are the two
things that can go wrong.

## Writing the base case first

A practical habit, and it is the one that makes recursion feel manageable.

**Write the base case before the recursive case.** Always. Ask: what is the
smallest input this method could get, and what is the answer for it?

For factorial, the smallest sensible input is 0, and the answer is 1.

For summing an array, the smallest input is an empty array, and the answer is 0.

For reversing a string, the smallest input is the empty string, and the answer is
the empty string.

Answering that question first does three things. It forces you to decide what the
method's domain actually is — which is the precondition of Chapter 11. It gives
you something concrete before you tackle the hard part. And it means the
termination argument is settled before you write the call that needs it.

Most broken recursions I have seen were written the other way round: the
interesting case first, the base case bolted on afterwards, and the boundary
wrong.

## What the machine does

One trace, once, so the mechanism is not mysterious. `factorial(4)`:

```
factorial(4)  →  4 * factorial(3)
                     factorial(3)  →  3 * factorial(2)
                                          factorial(2)  →  2 * factorial(1)
                                                               factorial(1)  →  1
                                          returns 2 * 1  = 2
                     returns 3 * 2  = 6
              returns 4 * 6  = 24
```

Four frames, stacked. Each suspended at its multiplication, waiting for the one
above. When `factorial(1)` returns 1, the stack unwinds and each pending
multiplication completes.

Nothing exotic. Chapter 12's stack, doing exactly what it does for any nested
calls; the fact that the frames belong to the same method is irrelevant to the
machine, because frames belong to executions.

Now, having seen it: **do not do this again.** The next lesson explains why.

## The limits are still there

Worth noticing, because it is a nice collision of chapters:

```java
factorial(12)     // 479001600
factorial(13)     // 1932053504
```

13! is 6,227,020,800, which does not fit in an `int`. The result you get is
Chapter 2's wraparound, and the recursion is entirely correct — the arithmetic
overflowed.

The iterative version gives the same wrong answer, which is the point. The defect
is in the type, not the technique, and `long` postpones it to 20! while
`BigInteger` removes it.

Next: how to think about a recursive method without tracing it.
