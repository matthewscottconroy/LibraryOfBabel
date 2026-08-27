# Exercises

**32.1** Instrument linear and binary search to count comparisons. Report the
worst-case count for $n$ of 1,000 up to 10,000,000. Confirm that the binary column
grows by one when $n$ doubles.

**32.2** Give the big-O of each: a single loop over $n$; two loops in sequence;
two nested loops; a loop containing a binary search; a loop from 1 to $n$ doubling
the counter each step; two nested loops where the inner runs to the square root of
$n$.

**32.3** Find the bug: `for (String s : list) if (otherList.contains(s)) n++;`.
State its complexity, fix it, state the new complexity, and measure both at
$n = 50{,}000$.

**32.4** Write the nested pair loop with `c = 0` and with `c = r + 1`. Count the
iterations of each for $n = 1000$. Confirm the ratio, then explain why both are
$O(n^2)$.

**32.5** Instrument naive `fib` to count calls. Report the counts for $n$ of 10,
20, 30, 35 and 40. Compute the per-step growth factor from your numbers and
compare it to the golden ratio.

**32.6** Memoize your `fib` and report the call count for $n = 40$ again. State
the complexity before and after, in both time and space.

**32.7** *Measurement.* Time bubble sort and `Arrays.sort` on random arrays of
4,000 to 32,000 elements, after warm-up. Report the ratio between consecutive
bubble-sort timings and say what class it indicates.

**32.8** Extend Exercise 32.7 to 64,000 and 128,000. Report what you observe. If
your ratios stay near 4, say so; if they do not, say what you would need to
measure in order to find out why.

**32.9** Prove, using the counting argument from Section 32.2.1, that no
comparison sort can do better than $O(n \log n)$ in the worst case. Then explain in
one sentence why counting sort does not contradict it.

**32.10** *Measurement.* Measure the memory used by `int[1_000_000]` and by an
`ArrayList<Integer>` of the same values. Report bytes per element for each and
account for the difference using Chapters 16 and 27.

**32.11** Write a recursive method that walks a linked list and find the length at
which it overflows the stack on your machine. State the space complexity of the
recursive and iterative versions.

**32.12** Prove that `ArrayList.add` is $O(1)$ amortized when the array doubles.
Then compute the amortized cost if it grew by a constant 10 instead, and say what
that would do to building a list of a million elements.

**32.13** *Measurement.* Sum the elements of a two-dimensional array in row-major
and then in column-major order. Report both times. Both are $O(n^2)$ and do the
same number of operations; explain the difference.

**32.14** *Design, no code.* [carries forward] You have a method that is $O(n^2)$ and takes 40 ms on
today's data of 3,000 records. Your product expects 100,000 records next year.
Predict the running time. Then say what you would do, and what you would need to
measure before doing it.
