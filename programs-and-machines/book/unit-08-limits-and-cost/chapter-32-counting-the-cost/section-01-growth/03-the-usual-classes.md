# The Usual Classes

There are seven growth classes you will meet in practice, and the useful way to
hold them is not as formulas.

It is as an answer to one question: **what happens when the data doubles?** Nothing.
One more step. Twice the work. Four times. The work squared. Those answers are what
you actually feel when a system grows, and the table below is mostly a way of
attaching names to them.

Seven classes cover nearly everything.

| class | name | doubling $n$ means | example |
|---|---|---|---|
| $O(1)$ | constant | no change | array index, hash lookup |
| $O(\log n)$ | logarithmic | one more step | binary search |
| $O(n)$ | linear | twice the work | scanning a list |
| $O(n \log n)$ | linearithmic | slightly over twice | good sorting |
| $O(n^2)$ | quadratic | four times | nested loops, bubble sort |
| $O(2^n)$ | exponential | squares the work | subsets, naive Fibonacci |
| $O(n!)$ | factorial | worse still | permutations |

The middle column is the useful one. It is what you feel when the data grows.

## What each is

**$O(1)$ — constant.** Independent of input size. `array[i]` is one multiplication
and one addition, as Chapter 15 derived, regardless of whether the array holds ten
elements or ten million. `HashMap.get` is a hash and a bucket access.

Not "fast" — *unchanging*. An $O(1)$ operation taking a millisecond is slower than
an $O(n)$ operation taking a nanosecond per element, for any $n$ under a million.

**$O(\log n)$ — logarithmic.** Each step discards a constant fraction. Binary
search, balanced-tree operations, and the number of digits in a number.

Effectively free. Ten million elements took 24 comparisons; a billion would take
30. There is no input size for which a $\log n$ algorithm is a problem.

**$O(n)$ — linear.** Look at each element a constant number of times. Scanning,
summing, copying, `contains` on a list.

Optimal for any problem that must examine all the input, which is a real lower
bound rather than a limitation — you cannot find the maximum of $n$ unsorted
numbers without looking at all $n$.

**$O(n \log n)$ — linearithmic.** Sorting by comparison, most divide-and-conquer.
It is very close to linear in practice: at a million elements, $\log_2 n$ is 20,
so it is twenty times a linear pass rather than a million times.

This is also a **proven lower bound** for comparison sorting, which is worth
knowing — no comparison sort can do better, and Section 32.2.1 sketches why.

**$O(n^2)$ — quadratic.** Every element against every other. Bubble sort, the
nested pair loop, the naive "is this list free of duplicates" check.

The boundary where growth starts to hurt. A million elements is $10^{12}$
operations — hours. Verified, each doubling multiplying the work by exactly four:

```
     1,000            499,500          -
     2,000          1,999,000       4.00
     4,000          7,998,000       4.00
     8,000         31,996,000       4.00
    16,000        127,992,000       4.00
```

**$O(2^n)$ — exponential.** Every subset, every path, every combination. Adding
*one* element doubles the work.

Verified, naive Fibonacci's call counts:

```
     n        fib calls      ratio
    10              177          -
    20           21,891     123.68
    30        2,692,537     123.00
    35       29,860,703      11.09
    40      331,160,281      11.09
```

Ten more in $n$ multiplies the calls by about 123; five more multiplies them by
about 11.1. Since $11.09^2 \approx 123$, the two agree, and the per-step factor is
$11.09^{1/5} \approx 1.618$ — the golden ratio, exactly as Chapter 13 said.

That is a nice check: the growth rate was predicted from the recurrence and the
measurement confirms it to three digits.

At $n = 40$ that is 331 million calls for a number you can compute with a
five-line loop in forty additions. Chapter 13's memoization removes it entirely.

**$O(n!)$ — factorial.** All orderings. The travelling salesman by brute force.
$20!$ is $2.4 \times 10^{18}$; at a billion per second that is 77 years.

## The sizes that matter

Rough guidance for a modern machine at roughly $10^9$ simple operations per
second:

| $n$ | $n^2$ | $2^n$ |
|---|---|---|
| 10 | instant | instant |
| 100 | instant | $10^{30}$ — never |
| 1,000 | instant | never |
| 10,000 | 0.1 s | never |
| 100,000 | 10 s | never |
| 1,000,000 | 3 hours | never |
| 10,000,000 | 12 days | never |

Two things to take from it.

**Quadratic is fine up to about ten thousand and painful past a hundred
thousand.** That is the practical boundary, and it is why an $O(n^2)$ algorithm in
a prototype is usually acceptable and in a production path usually is not.

**Exponential is never fine past about forty.** There is no faster machine that
helps: doubling the processor's speed buys you one more element.

## Improving a class

The classes are what you move between, and there are a few standard moves.

**$O(n^2) \to O(n)$: use a hash set.** The single most common improvement in
ordinary code. Nested loops checking membership become one loop and a `contains`
on a `HashSet`.

**$O(n) \to O(\log n)$: sort once, then binary search.** Worth it if you will
search many times, since the sort costs $O(n \log n)$ up front.

**$O(2^n) \to O(n)$: memoize.** Chapter 13's Fibonacci. The exponential cost came
from recomputation, and a cache removes it.

**$O(n \log n) \to O(n)$: exploit structure.** If the keys are small integers,
counting sort is linear. Comparison sorting's lower bound applies only to
comparison sorting.

Notice that all four are *changes of approach*, not micro-optimizations. That is
the general truth: **changing the class beats improving the constant, past some
size, and no amount of tuning changes the class.**

And the converse, which is Section 32.2.3's subject: below that size, the constant
is everything and the class is noise.

Next: sorting and searching, where the classes become concrete.
