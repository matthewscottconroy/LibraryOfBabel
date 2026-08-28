# Measuring Honestly

Two sections of this chapter have gone into building a way to predict what a
program will cost. This lesson is about where that apparatus stops working, and
rather than describe its limits I would rather show you one.

## A row that did not behave

Here is the bubble-sort table from two lessons ago, with one extra row on the
bottom. Read down the ratio column and stop when something goes wrong.

```
         n      bubble ms    ratio
     4,000            4.5        -
     8,000           14.8     3.28
    16,000           57.3     3.88
    32,000          236.3     4.13
    64,000        2,830.0    11.98
```

The first four rows converge on 4, as $O(n^2)$ predicts. The fifth is 12.

Your first instinct should be that somebody's laptop was busy compiling something.
Mine was. It is not that.

Three separate runs gave ratios of 11.98, 12.44 and 11.66, against a predicted 4.
Bubble sort at 64,000 elements is three times slower than its own complexity class
says it can be, reproducibly.

Something on this machine changes between 32,000 and 64,000 elements. The obvious
candidate is memory hierarchy — 64,000 `int`s is 256 kilobytes, and this machine's
caches are 48 kilobytes of L1 per core and 1 megabyte of L2 per core — but 32,000
elements is already 128 kilobytes and also exceeds L1, so a simple L1 boundary does
not explain it either. Establishing the cause would need hardware performance
counters, which is beyond what these measurements can show.

The honest statement is: **the measurement is reproducible, it disagrees with the
prediction by a factor of three, and this chapter has not established why.**

That row is left in this book deliberately, and I want to be plain about why.

The apparatus is genuinely useful. It is also a model, and models have residuals.
A chapter that quietly showed you only the rows that fit would be teaching you
something false about what analysis is — not about bubble sort, about analysis. You
would come away believing the predictions are tighter than they are, and the first
time reality disagreed with you in your own work you would assume you had made a
mistake.

## The four things big-O cannot see

The disagreement above is one instance of a general problem. Complexity analysis
models a machine that does not exist — one where every operation costs the same
and memory is uniform. Four ways the real machine differs.

**Memory hierarchy.** A value in L1 cache is available in about a nanosecond; one
in main memory takes a hundred. Chapter 15 measured a three-times difference
between row-major and column-major traversal of the same array, doing exactly the
same number of operations, both $O(n^2)$.

This is the largest single source of disagreement, and it is why an `ArrayList`
frequently beats a `LinkedList` at operations the theory says the linked structure
should win — Chapter 17 measured 2,589 milliseconds for one such case. The array's
elements are contiguous, so the processor's prefetcher is right; the list's are
scattered, so every step is a cache miss. Chapter 15's promise, now with the
explanation.

**Constant factors.** Two $O(n \log n)$ sorts can differ tenfold. `Arrays.sort` at
32,000 elements took 2.19 milliseconds and bubble sort took 236 — a hundred-fold
difference that is *partly* the complexity class and partly decades of tuning.

**Branch prediction.** A modern processor guesses which way a branch will go and
speculatively executes ahead. A predictable branch is nearly free; an
unpredictable one costs fifteen to twenty cycles. Sorting an array *before*
processing it can make a subsequent loop several times faster, purely because the
branches become predictable, with no change in operation count.

**The JIT.** Chapter 21 measured a devirtualized call at 1.4 nanoseconds and a
megamorphic one at 2.1. Chapter 26 measured an `IntStream` matching a hand-written
loop exactly, while `Stream<Integer>` took ten times as long. None of that is
visible to operation counting.

## What analysis is for, then

Given all that, the case for the apparatus is narrower than its prominence
suggests, and it is still strong.

**It predicts scaling.** This is the irreplaceable part. A measurement at $n =
1000$ tells you nothing about $n = 10^6$; the class does. An $O(n^2)$ algorithm
fine on today's data will not survive next year's, and no amount of profiling
today reveals that.

**It identifies catastrophes.** An accidental $O(n^2)$ — a `list.contains` inside
a loop, string concatenation in a loop, a nested scan — is a bug that testing on
small data will never find. Recognizing the shape in code review is worth more
than any profiler.

**It guides the choice of data structure.** The right structure is a change of
class; tuning the wrong one is a change of constant.

And what it is not for:

**Choosing between two implementations of the same class.** Measure.

**Deciding whether something is fast enough.** Measure.

**Justifying an optimization.** Measure first, and Chapter 18 explains at length
why.

## The discipline

The rule this chapter proposes, which is Chapter 18's with an addition:

> **Analyze to predict how it scales. Measure to know what it costs.**

Both, and for different questions. Analysis without measurement produces programs
optimized for the wrong constant. Measurement without analysis produces programs
that work in testing and fail when the data grows.

And, since the book has asked for it throughout: **derive claims rather than
asserting them, and show the arithmetic.** The golden-ratio check in Section
32.1.3 — a predicted growth factor of 1.618 confirmed by measured call counts to
three digits — is what that looks like when it works. The bubble-sort row above is
what it looks like when it does not, and reporting the second is as much a part of
the discipline as reporting the first.

Chapter 33 turns from how long a program takes to how much a message contains,
and the counting arguments start proving things impossible.
