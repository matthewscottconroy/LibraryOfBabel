# The Loop Invariant

Here is a loop that sums the numbers 1 to 5:

```java
int sum = 0;
for (int k = 1; k <= 5; k++) {
    sum += k;
}
```

How do you know it is right?

The usual answer is that you ran it and it printed 15. That is evidence, and it
is weaker than it looks: it tells you about one input. If the 5 were a variable
you would have tested one value of it.

Here is a different kind of answer.

## The claim

At the top of each iteration, this is true:

> **`sum` holds the total of the numbers from 1 up to `k - 1`.**

That sentence is the **loop invariant** — invariant because it does not vary. It
is true every time the loop reaches the top, on every iteration, for any value
the loop is run with.

Check it against a trace:

| at top of iteration | `k` | `sum` | invariant predicts | agrees? |
|---|---:|---:|---|---|
| 1st | 1 | 0 | sum of 1..0 — an empty range, so 0 | yes |
| 2nd | 2 | 1 | sum of 1..1 = 1 | yes |
| 3rd | 3 | 3 | sum of 1..2 = 3 | yes |
| 4th | 4 | 6 | sum of 1..3 = 6 | yes |
| 5th | 5 | 10 | sum of 1..4 = 10 | yes |
| after the loop | 6 | 15 | sum of 1..5 = 15 | yes |

It holds. But the table is still just a trace — five rows, and we said tracing was
the weak method. The point of the invariant is that we do not need the table.

## Proving it without tracing

Three obligations. This is the whole technique.

**1. Establishment.** Is the invariant true when the loop is first reached?

Before the first iteration, `k` is 1 and `sum` is 0. The invariant claims `sum` is
the total of 1 up to 0 — an empty range, whose total is 0. True.

**2. Preservation.** *If* the invariant holds at the top of an iteration, is it
still true at the top of the next one?

Assume `sum` is the total of 1 to `k-1`. The body runs `sum += k`, so `sum`
becomes the total of 1 to `k`. Then `k++` makes `k` one larger. Now `sum` is the
total of 1 to (new `k`) − 1, which is the invariant again. True.

**3. Termination.** When the loop stops, what does the invariant give us?

The loop stops when `k <= 5` is false, so `k` is 6. The invariant says `sum` is
the total of 1 to 5. Which is the answer we wanted.

Those three steps prove the loop correct **for every input**, not for five. We
never traced an iteration. We showed that whatever is true at the top of one
iteration is true at the top of the next, and that where it starts and where it
ends are what we need.

## Why this is induction

If that felt familiar, it should. It is mathematical induction, applied to a
program.

Induction proves a statement about all natural numbers by proving it for the
first, and proving that whenever it holds for one it holds for the next.
Establishment is the base case; preservation is the inductive step. The loop
supplies the "all natural numbers" by iterating.

Chapter 13 uses the same principle for recursion, where the resemblance is even
more direct. It is worth registering that the two constructs are the same idea:
a loop and a recursion are both a way of doing something an unbounded number of
times, and both are justified by induction.

## Finding an invariant

The question that produces one: **what does the variable I care about mean,
partway through?**

Not "what is its value" — that changes. What does it *mean*. In our loop, `sum`
means "the total so far", and the invariant is that sentence made precise about
how far "so far" goes.

Some standard ones:

```java
// find the largest
int max = a[0];
for (int i = 1; i < a.length; i++)
    if (a[i] > max) max = a[i];
// invariant: max is the largest of a[0..i-1]
```

```java
// linear search
int found = -1;
for (int i = 0; i < a.length; i++)
    if (a[i] == target) { found = i; break; }
// invariant: target does not appear in a[0..i-1]
```

```java
// count matches
int count = 0;
for (int i = 0; i < a.length; i++)
    if (matches(a[i])) count++;
// invariant: count is the number of matches in a[0..i-1]
```

Notice the shape they share. Each says something about **the part already
processed**, expressed in terms of the loop variable. That is what an invariant
almost always is, and knowing the shape makes them much easier to write.

Notice too that `a[0..i-1]` keeps appearing. The elements from the start up to
but not including the current position — which is exactly what "processed so far"
means when `i` is the next thing to look at. Getting that boundary right is the
subject two lessons from now.

## What it is for

Three uses, in increasing order of value.

**Checking a loop you wrote.** State the invariant, then check establishment and
preservation. If either fails you have found a bug, and you have found it without
running anything.

**Understanding a loop you did not write.** Ask what is true every time round.
Faster than tracing and more reliable.

**Writing a loop in the first place.** This is the real payoff, and it inverts
the usual order. Decide what the invariant should be, then write initialization
that establishes it, a body that preserves it, and a condition whose failure
gives you the result. The loop comes out correct by construction.

That third use takes practice and it is genuinely a different way of working.
Dijkstra, whom we meet in this chapter's profiles, spent a career arguing it is
the only way that scales.

## An honest note

Not every loop needs a written invariant, and I do not write one for every loop.

But when a loop is wrong and I cannot see why, the invariant finds it — because a
loop that is wrong is a loop whose invariant fails at establishment, or fails to
be preserved, or does not give the answer at termination, and checking those three
things is mechanical.

Ask the question. Write the sentence only when the answer is not immediate.

Next: the obligation the invariant does not cover.
