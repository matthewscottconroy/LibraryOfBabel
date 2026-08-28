# Off-by-One as a Failed Proof

The most common bug in programming has a name, and the name is a joke about how
common it is: the off-by-one error, or OBOE.

```java
for (int i = 0; i <= names.length; i++)     // one too many
for (int i = 1; i < names.length; i++)      // skips the first
for (int i = 0; i < names.length - 1; i++)  // skips the last
```

Each looks reasonable. Each is wrong. And they are wrong in a way that eyeballing
does not catch, because the code looks right — that is why you wrote it.

This lesson argues that off-by-one is not carelessness. It is a **failed boundary
claim**, and the invariant is the tool that finds it.

## Why boundaries are hard

Because a range has two ends and four plausible ways to include them:

```
1 to 5 inclusive of both      1, 2, 3, 4, 5      5 values
1 to 5 excluding the last     1, 2, 3, 4         4 values
0 to 5 excluding the last     0, 1, 2, 3, 4      5 values
0 to 5 inclusive of both      0, 1, 2, 3, 4, 5   6 values
```

All four are legitimate; you need different ones in different places. And the
distinction lives in one character — `<` versus `<=`, or `0` versus `1` — so a
wrong choice looks exactly like a right one.

There is also the fencepost problem, which is the same thing in a different suit.
A fence 100 meters long with posts every 10 meters needs **eleven** posts, not
ten. Count the gaps and you get ten; count the posts and there is one more,
because both ends have one.

## Java's convention

Java, like C and most of its descendants, uses **half-open** ranges: include the
start, exclude the end.

```java
for (int i = 0; i < n; i++)
```

Elements 0 through *n*−1. *n* iterations.

This is not arbitrary, and the reasons are worth knowing because they tell you
when to follow the convention.

**The count is the difference.** A range from *a* to *b* half-open contains
*b* − *a* elements. No adjustment. `0` to `n` holds *n* things.

**Ranges join cleanly.** `0` to `k` followed by `k` to `n` covers `0` to `n`
exactly once, with no gap and no overlap. With inclusive ranges you must write
`k+1` at the join, and that `+1` is a place to be wrong.

**Empty is expressible without awkwardness.** `0` to `0` is empty, naturally. An
inclusive range needs an end below its start to be empty, which is a strange thing
to write.

Dijkstra wrote a short note in 1982 arguing exactly this, and it is one of the few
pieces of programming style advice with an actual proof attached.

The practical rule: **follow the convention.** Half-open, starting at 0. Not
because it is objectively superior in every case, but because everything in Java
already works that way — `length`, `size()`, `substring`, `subList` — and a loop
that departs from it will be misread.

## The invariant finds these

Here is the payoff for the last two lessons. Take a wrong loop:

```java
int max = a[0];
for (int i = 0; i < a.length; i++) {
    if (a[i] > max) max = a[i];
}
```

This one is not wrong, exactly — it works — but it is *sloppy*, and the invariant
shows why. State it:

> `max` is the largest of `a[0..i-1]`.

**Establishment:** before the loop, `i` is 0, so the claim is that `max` is the
largest of `a[0..-1]` — an empty range. But `max` was set to `a[0]`, which is not
the largest of nothing. **The invariant fails at establishment.**

The fix the invariant points to is starting the loop at 1:

```java
int max = a[0];
for (int i = 1; i < a.length; i++) {
    if (a[i] > max) max = a[i];
}
```

Now before the loop `i` is 1, and `max` is the largest of `a[0..0]`, which is
`a[0]`. Established.

The original wasted an iteration comparing `a[0]` with itself. Harmless here. In
a loop where the body has a side effect, the same misalignment is a real bug —
and the invariant caught the misalignment without needing the bug to manifest.

## The one that actually breaks

```java
for (int i = 0; i <= a.length; i++) {
    System.out.println(a[i]);
}
```

Invariant: *`a[0..i-1]` have been printed*. Establishment holds. Preservation
holds. Termination: the loop ends when `i` is `a.length + 1`, so the claim is
that `a[0..a.length]` were printed — one more element than the array has.

The proof produces a claim about an element that does not exist, and at run time
Java throws `ArrayIndexOutOfBoundsException`. Chapter 15 explains why the check
exists and what it costs; the point here is that the invariant predicted the
failure from the source text.

## Three habits

**Prefer the enhanced `for`.** If you do not need the index, do not have one. A
loop with no index cannot have an index error:

```java
for (String name : names) { ... }
```

**Check the endpoints deliberately.** When you write a loop with explicit bounds,
ask two questions before moving on: what is the first value, and what is the last?
Say them out loud. Most off-by-ones are caught in the two seconds that takes.

**Test the edges, not the middle.** A loop that works on a five-element array will
usually work on a fifty-element one. The inputs that find bugs are the empty
array, the one-element array, and the boundary values — because those are where
the four range conventions differ from each other.

## Closing the chapter

A loop is a transition that moves the program counter backwards, and that single
capability is what separates a program whose running time is bounded by its
length from one that can run for a billion steps or forever.

`while` is the primitive; `for` is an abbreviation that gathers the three moving
parts where they can be checked together; the enhanced `for` removes the index
entirely and with it a whole class of error.

And the section that matters: a loop invariant is a claim about the state that
survives every iteration, proved by establishment and preservation, and combined
with the loop's exit condition it establishes correctness **for all inputs** —
which testing cannot do, because Chapter 6 showed the state space is too large to
cover. Termination is a separate obligation, proved with a decreasing non-negative
quantity, and it is genuinely hard in general: a six-line loop can pose an
unsolved problem in mathematics.

Unit II is nearly done. One chapter remains, and it is the practical one: what to
do when a program is not doing what you think, which is a situation this chapter's
tools address and do not eliminate.
