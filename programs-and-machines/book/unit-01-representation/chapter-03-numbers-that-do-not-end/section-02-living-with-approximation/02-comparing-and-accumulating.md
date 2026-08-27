# Comparing and Accumulating

Two operations cause most floating-point trouble. Comparing two values for
equality, and adding up a long list. Both have a fix, and both fixes are easy
once you see what is actually happening.

## Comparing

We established that `0.1 + 0.2 == 0.3` is `false`. Generalize the lesson: **testing
floating-point values for exact equality is almost always a mistake**, because two
computations that ought to agree mathematically will often land on adjacent grid
points instead of the same one.

The standard advice is to test whether the values are *close enough*:

```
Math.abs(a - b) < 0.000001
```

This is called an absolute-tolerance comparison, and it works when you know the
scale of your numbers in advance. It fails badly when you do not.

Suppose your tolerance is $10^{-6}$. If `a` and `b` are around $10^{-9}$ — plausible for
probabilities, or for values in metres that are really nanometres — then *every*
pair passes, because all such numbers are within $10^{-6}$ of each other. Your test
always says "equal" and has told you nothing.

Now suppose `a` and `b` are around $10^{16}$. From the last section, the gap between
adjacent doubles there is 2.0. Two values one representable step apart differ by
2, which is far more than $10^{-6}$, so your test always says "different" — even for
values that are as close as the format physically permits.

The fix is to scale the tolerance to the size of the numbers, which is called a
**relative** comparison:

```
Math.abs(a - b) <= 1e-9 * Math.max(Math.abs(a), Math.abs(b))
```

Read it as "within a billionth of the larger operand". Now the test means the
same thing at every magnitude.

You still need an absolute floor for the case where both values are near zero
(where a relative test becomes meaningless, since everything is a large multiple
of a tiny number), so production code usually combines both. But the important
habit is the one-line version of the idea: **decide what "close enough" means for
your problem, and say so explicitly.** There is no universal epsilon, and any
book that hands you one has skipped the thinking.

## The comparison that has no good answer

One warning before we leave the topic. If you write

```
if (total == 0.0)
```

be aware that this is true for both `+0.0` and `−0.0`, which is usually what you
want. But if you write

```
if (x == Double.NaN)
```

it is **always false**, including when `x` is NaN, because NaN is not equal to
anything including itself. To test for NaN you must use `Double.isNaN(x)`.

This trips people every single time. It is not an inconsistency; it is the
defined behavior, and it exists so that a NaN cannot masquerade as a legitimate
value in a comparison. But the natural-looking test does not work, and nothing
warns you.

## Accumulating

The second problem is subtler and does more damage in practice, because it
produces answers that are wrong without looking wrong.

Add one tenth ten times and watch:

```
after  1:  0.1
after  2:  0.2
after  3:  0.30000000000000004
after  4:  0.4
after  5:  0.5
after  6:  0.6
after  7:  0.7
after  8:  0.7999999999999999
after  9:  0.8999999999999999
after 10:  0.9999999999999999
```

Ten additions of a tenth, and the answer is not 1. Notice the error appearing at
step 3, vanishing at step 4 — the rounding happened to cancel — and reappearing at
step 8 for good.

That is 10 additions. Financial and scientific code routinely does millions, and
the errors do not politely cancel forever.

## Order matters

Here is the demonstration that surprises people most. Take one very large number
and one million ones, and add them up two ways.

Adding the large number first:

```
start at 1e16, then add 1.0 a million times   →  1.0E16
```

The million additions accomplished **nothing at all**. The gap between adjacent
doubles at $10^{16}$ is 2.0, so adding 1 produces a value less than half a step away,
which rounds straight back to where it started. Every single addition was
discarded.

Adding the ones first:

```
sum a million 1.0s (= 1000000.0), then add 1e16   →  1.0000000001E16
```

Which is correct: 10,000,000,001,000,000.

Same numbers, same operations, different order, and one answer is right while the
other has lost a million entirely. Floating-point addition is **not associative** —
`(a + b) + c` and `a + (b + c)` can differ — and that breaks an assumption so
basic that most people have never consciously held it.

The practical rule that falls out: **when summing values of widely differing
magnitudes, add the small ones first.** Sorting a list before summing it is
sometimes worth the cost.

## Doing better than sorting

There is a cleverer technique worth knowing about, because it is a lovely idea
and because you may meet it in library code.

**Kahan summation** keeps a second variable holding the error that was lost in the
previous addition, and feeds it back in on the next one:

```
sum = 0.0;  c = 0.0;
for each value x:
    y = x - c            // apply the correction from last time
    u = sum + y          // this addition loses some low-order bits
    c = (u - sum) - y    // recover exactly what was lost
    sum = u
```

The line `c = (u - sum) - y` looks like it should always be zero, and in exact
arithmetic it would be. In floating-point arithmetic it captures precisely the
part of `y` that did not survive the addition — which is then subtracted from the
next input, putting the lost information back.

Run the ten-tenths accumulation with this and you get exactly `1.0`.

The technique is named for William Kahan, who is the reason IEEE 754 exists at
all, and whom you will meet in this chapter's profiles.

## What to carry away

Do not test floating-point values with `==`; decide explicitly what closeness
means for your problem, and scale the tolerance to the magnitudes involved.

Do not assume a long sum is accurate. If magnitudes vary widely, order matters,
and there are techniques that do better.

And underneath both: floating-point operations are individually correct and
collectively lossy. Every single step gave the best possible answer. The loss is
in the accumulation of "best possible" over many steps, which is a different
thing from a mistake, and needs a different kind of vigilance.
