# Counting Operations

Chapter 18 told you to measure rather than guess, and meant it. Here is the gap in
that advice.

Seconds depend on the processor, on the JIT's mood, on what else is running, on how
warm the cache is. They are the right unit for "is this fast enough today" and
useless for "will this survive ten times the data" — and the second question is
usually the one you actually have.

Chapter 18 measured seconds. Seconds depend on the processor, the JIT's mood, the
operating system's scheduling, what else is running, and how warm the cache is.
They are the right unit for "is this fast enough today" and the wrong one for "is
this algorithm any good".

So count **operations** instead: how many times the essential work happens, as a
function of the input size.

## An example

Searching an array of $n$ elements for a value.

**Linear search** — look at each element until you find it.

```java
for (int i = 0; i < a.length; i++)
    if (a[i] == x) return i;
```

Worst case, the value is last or absent: $n$ comparisons.

**Binary search** — on a sorted array, look at the middle, discard half, repeat.

```java
int lo = 0, hi = a.length - 1;
while (lo <= hi) {
    int mid = (lo + hi) >>> 1;
    if (a[mid] == x) return mid;
    else if (a[mid] < x) lo = mid + 1;
    else hi = mid - 1;
}
```

Each step halves what remains, so the number of steps is how many times $n$ can be
halved before reaching one — which is $\log_2 n$.

That is Chapter 9's promise, made concrete: *each observation halves what is left,
so a program of a thousand steps is narrowed in about ten questions and a million
in about twenty.*

## Measured

Counting the actual comparisons, worst case, for a range of sizes:

```
           n         linear         binary
       1,000          1,000             10
      10,000         10,000             14
     100,000        100,000             17
   1,000,000      1,000,000             20
  10,000,000     10,000,000             24
```

Read the two columns as a pair. The input grew by a factor of ten thousand; the
linear column grew by a factor of ten thousand, and the binary column grew from
10 to 24.

That is the entire argument for this chapter's existence. Ten million elements,
twenty-four comparisons. Adding another factor of a thousand would add ten more.

## Why counting is the right unit

**It is machine-independent.** Twenty-four comparisons is twenty-four comparisons
on any processor, in any language, in any decade. The seconds are not.

**It predicts.** From the counts above you can state what happens at a billion
elements without running anything. A timing cannot be extrapolated, because you do
not know which effects are linear in $n$ and which are not.

**It isolates the algorithm.** A slow implementation of binary search still beats
a fast implementation of linear search, past some size, and counting shows why.

## What to count

Not every operation — the one that dominates.

For a search, comparisons. For a sort, comparisons and swaps. For a graph
algorithm, edges examined. For a database query, rows touched. For anything
touching disk or network, the I/O operations, because those are thousands of times
more expensive than anything happening in memory.

The judgment is: **what does the program do most of, and what is expensive?** Get
that wrong and the analysis describes something that is not the bottleneck, which
is a more subtle error than counting incorrectly.

## Best, worst, and average

Three different questions, and confusing them causes trouble.

**Worst case.** The most operations over all inputs of size $n$. Linear search:
$n$. This is the default in this chapter and in most writing, because it is a
guarantee — a promise nothing can exceed.

**Best case.** The fewest. Linear search: 1, if the value is first. Almost always
useless, since you cannot rely on it.

**Average case.** The expected number over some distribution of inputs. Linear
search: about $n/2$ for a value present at a uniformly random position. Honest and
harder, because it requires knowing the distribution — and the distribution is
usually a guess.

Quicksort is the standard case where the distinction matters: its average is
$n \log n$ and its worst case is $n^2$, on already-sorted input with a naive pivot.
Section 32.2.1 returns to this.

Where an adversary chooses your input — anything reachable from a network — the
worst case is not hypothetical. Hash table collision attacks are exactly this:
deliberately chosen keys turning constant-time lookup into a linear scan, which is
why Java's `HashMap` converts long collision chains into trees.

## Counting a nested loop

The pattern from Chapter 15:

```java
for (int r = 0; r < n; r++)
    for (int c = r + 1; c < n; c++)
        consider(grid[r][c]);
```

The outer loop runs $n$ times; the inner runs $n - r - 1$ times. Total:

$$(n-1) + (n-2) + \cdots + 1 + 0 = \frac{n(n-1)}{2}$$

Verified:

```
         n              pairs      ratio
     1,000            499,500          -
     2,000          1,999,000       4.00
     4,000          7,998,000       4.00
     8,000         31,996,000       4.00
    16,000        127,992,000       4.00
```

$1000 \times 999 / 2 = 499{,}500$, exactly as computed. And each doubling of $n$
multiplies the work by four, which is the signature of $n^2$ growth — the $1/2$
does not affect the ratio, which is the first hint of why constants get discarded.

Note also that the `c = r + 1` idiom halves the work relative to `c = 0` and does
**not** change the growth. Both are quadratic. That is Chapter 15's remark, and
Section 32.1.2 explains why halving is invisible to this analysis.

Next: the notation for saying it.
