# Searching and Sorting

These two problems are where complexity analysis was developed and they remain the
clearest examples, partly because one of them has a proven lower bound.

## Searching

**Unsorted data**: linear search, $O(n)$, and you cannot do better. To be certain a
value is absent you must look at every element.

**Sorted data**: binary search, $O(\log n)$.

**Hashed data**: $O(1)$ average, $O(n)$ worst case if every key collides.

The decision is about how many searches you will do.

One search on unsorted data: scan it. $O(n)$, and sorting first would cost more
than the search.

Many searches: pay $O(n \log n)$ to sort once, then $O(\log n)$ each. Break-even
is around $\log n$ searches, so for a thousand elements, roughly ten.

Very many searches: build a `HashMap`, $O(n)$ to construct and $O(1)$ each. Almost
always the right answer when the accesses are by key.

That is Chapter 17's advice — declare by interface, choose the implementation for
the access pattern — with the arithmetic behind it.

## Sorting

The comparison sorts worth knowing:

| algorithm | average | worst | space | stable |
|---|---|---|---|---|
| bubble | $O(n^2)$ | $O(n^2)$ | $O(1)$ | yes |
| insertion | $O(n^2)$ | $O(n^2)$ | $O(1)$ | yes |
| merge | $O(n \log n)$ | $O(n \log n)$ | $O(n)$ | yes |
| quick | $O(n \log n)$ | $O(n^2)$ | $O(\log n)$ | no |
| heap | $O(n \log n)$ | $O(n \log n)$ | $O(1)$ | no |

**Stable** means equal elements keep their relative order, which matters when you
sort by one field having already sorted by another.

**Insertion sort** is $O(n^2)$ and is genuinely the best choice for small arrays —
under about fifty elements — because its constant factor is tiny and it is
$O(n)$ on nearly sorted input. Real sort implementations use it for small
subarrays, which is Section 32.1.2's point about small inputs arriving in
production code.

**Merge sort** is the canonical divide-and-conquer, promised in Chapter 13: split
in half, sort both halves, merge. The recursion is $\log n$ deep and each level
does $O(n)$ work, giving $O(n \log n)$. It needs $O(n)$ extra space, which is its
one drawback and the reason it is not always chosen.

**Quicksort** partitions around a pivot and recurses on both sides. Average
$O(n \log n)$ with a small constant, which makes it the fastest in practice — and
$O(n^2)$ if the pivot is consistently bad, classically on already-sorted input
with a first-element pivot. Modern implementations choose the pivot carefully and
fall back to heap sort if the recursion goes too deep.

## Measured

Bubble sort against `Arrays.sort`, on the same random arrays, timed after warm-up:

```
         n      bubble ms    ratio   Arrays.sort ms
     4,000            4.5        -             2.28
     8,000           14.8     3.28             1.05
    16,000           57.3     3.88             1.05
    32,000          236.3     4.13             2.19
```

Two things.

**Bubble sort's ratios converge on 4.** Each doubling multiplies the time by
roughly four, which is the quadratic signature, and the convergence from 3.28 to
4.13 is the lower-order terms becoming negligible — exactly what the $n_0$ in
big-O's definition is about.

**`Arrays.sort` barely moves.** From 4,000 to 32,000 elements — eight times the
data — the time went from 2.28 ms to 2.19 ms. It is not that $n \log n$ is free;
it is that at these sizes the measurement is dominated by effects other than the
sorting, and the growth is not yet visible above the noise.

At $n = 32{,}000$, bubble sort takes 236 milliseconds and `Arrays.sort` takes 2.19.
A hundred-fold difference, and it doubles with every doubling of $n$.

Section 32.2.3 has one more row of this table, and it does not behave.

## The lower bound

A genuine impossibility result, and a mild preview of Chapter 34.

**No comparison sort can be faster than $O(n \log n)$ in the worst case.**

The argument is a counting one. A sorting algorithm's behavior is determined by
the answers to its comparisons, each of which is a yes or no. After $k$
comparisons there are at most $2^k$ distinct sequences of answers, so at most
$2^k$ distinct orderings the algorithm can distinguish.

There are $n!$ possible orderings, and the algorithm must be able to produce each
one. So:

$$2^k \ge n! \quad \Longrightarrow \quad k \ge \log_2 (n!)$$

And $\log_2(n!)$ is $\Theta(n \log n)$, by Stirling's approximation.

That is a statement about **every possible comparison sort**, including ones nobody
has invented. No cleverness helps, because the bound comes from counting how much
information the comparisons can carry.

The escape is to stop comparing. **Counting sort** and **radix sort** are $O(n)$
because they use the values as indices rather than comparing them — and they work
only when the values are constrained, small integers or fixed-length keys. The
lower bound applies to comparison sorting, and the way past a lower bound is
always to change the problem.

That structure — count the possibilities, count what the algorithm can distinguish,
conclude — is worth remembering. Chapter 33 uses it to prove that no compressor
shrinks everything, and Chapter 34's argument is a relative of it.

## In practice

**Do not write a sort.** `Arrays.sort` and `Collections.sort` are implemented by
specialists, tuned over decades, and better than what you will write.

Java uses **dual-pivot quicksort** for primitives — fast, and stability is
meaningless when the elements are indistinguishable — and **TimSort** for objects,
which is a merge sort that detects runs of already-ordered data and is stable and
frequently much better than $n \log n$ on real data, which is rarely random.

The useful knowledge is not how to write them but what they cost, what stability
means, and that `Arrays.sort` on objects is stable and on primitives is not.

Next: the other cost.
