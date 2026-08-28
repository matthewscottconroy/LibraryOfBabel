# Space as a Cost

Time gets all the attention, and memory is the one that actually stops you.

A program that is too slow still finishes. A program that runs out of memory does
not, and the boundary arrives sooner than you would guess — Chapter 12's recursion
limit is about twenty thousand frames deep, which a linked list of a hundred
thousand elements will walk straight past.

The good news is that you already own the tools. Memory is analyzed exactly the way
time was — same notation, same counting, same habit of ignoring the constants and
watching the shape.

## Space complexity

Same notation, counting memory instead of operations, and usually counting only
the **extra** space beyond the input.

**$O(1)$** — a fixed number of variables regardless of input size. Bubble sort,
insertion sort, heap sort, and every loop with a counter and an accumulator.

**$O(\log n)$** — recursion $\log n$ deep, since each frame occupies stack.
Quicksort, binary search written recursively.

**$O(n)$** — a copy of the input, or an auxiliary array. Merge sort, memoization
tables, and any method that builds a result list.

**$O(n^2)$** — a matrix. Adjacency matrices, dynamic programming tables over two
sequences.

## The stack counts

Chapter 12's stack is space, and recursion depth is space complexity.

A recursive method $n$ frames deep uses $O(n)$ stack, and Chapter 12 measured this
machine's limit at about 22,447 frames. That is not a large number: a recursive
traversal of a linked list of a hundred thousand elements overflows, where the
iterative version does not.

This is one of the more useful practical consequences of the analysis. **A
recursion whose depth is proportional to the input is a program with an input-size
limit**, and the limit is much lower than memory would suggest. A recursion whose
depth is $\log n$ — divide and conquer — is fine forever, because $\log n$ of
anything is small.

## Measured

Actual memory, on this machine:

```
int[10,000,000]        : 41,943,040 bytes (4.2 per element)
ArrayList<Integer> 1e6 : 20,300,512 bytes (20.3 per element)
```

Four bytes per `int`, as Chapter 2 said, plus a small fixed overhead — the array
is exactly what it claims to be.

Twenty bytes per element for the `ArrayList<Integer>`. Five times as much, for the
same numbers.

The breakdown, and every piece is from an earlier chapter. Each `Integer` is an
object with a header — twelve to sixteen bytes on a 64-bit JVM — plus the four-byte
value, padded to a multiple of eight. The list additionally holds a reference to
each, four bytes with compressed pointers. So roughly sixteen plus four.

That is Chapter 16's boxing, measured in bytes rather than in nanoseconds, and it
is why `int[]` and `IntStream` exist and why Chapter 27's erasure — which forbids
`List<int>` — has a cost that shows up in a heap dump.

## The trade

Time and space trade against each other, in both directions, and recognizing the
trade is more useful than either analysis alone.

**Spend space to save time.** A cache. A hash index. Chapter 13's memoized
Fibonacci, which turned $O(2^n)$ time into $O(n)$ time and $O(n)$ space. A
precomputed lookup table.

**Spend time to save space.** Recomputing instead of storing. Compressing.
Streaming a file instead of reading it whole — Chapter 29's `Files.lines` against
`Files.readString`, which is exactly this trade with a name.

The right side depends on which is scarce, and on a modern machine that is usually
memory for large data and time for small.

## Amortized cost

One more idea, promised in Chapter 17.

`ArrayList.add` is usually $O(1)$ — write to the next slot. Occasionally the array
is full and it must allocate a larger one and copy everything, which is $O(n)$.

So what is `add`? Neither $O(1)$ nor $O(n)$ describes it honestly. The answer is
$O(1)$ **amortized**: over a long sequence of operations, the average is constant.

The proof is Chapter 17's sum of powers of two, promised there. Growing to capacity
$n$ by doubling copies

$$1 + 2 + 4 + \cdots + n/2 = n - 1$$

elements in total, over $n$ additions. Fewer than one copy per addition on average,
no matter how large $n$ gets. The rare expensive operation is paid for by the many
cheap ones.

Amortized analysis is why doubling is the right growth strategy and adding a
constant is not. Growing by ten each time would copy

$$10 + 20 + 30 + \cdots + n \approx n^2/20$$

elements, making `add` $O(n)$ amortized and building a list quadratic. The
difference between a good `ArrayList` and a catastrophic one is which growth rule
was chosen, and the analysis is the reason.

`HashMap` resizing works the same way, and so does every dynamic array in every
language.

## When space is the real problem

Three situations where memory rather than time decides.

**Data that does not fit.** A ten-gigabyte file on an eight-gigabyte machine must
be streamed. No amount of speed helps, and the algorithm must be one that works in
a single pass.

**Garbage collection pressure.** Allocating heavily costs time indirectly — the
collector must run, and on a large heap that can pause the program. A program
allocating a million short-lived objects per second may be slow for reasons no
operation count shows.

**Cache.** The subject of the next lesson, and the one where the analysis in this
chapter is least predictive.

Next: honesty about all of this.
