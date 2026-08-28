# Space as a Cost

Time gets all the attention. Memory is the one that actually stops you.

Think about what each failure looks like. A program that is too slow still
finishes — you wait, you grumble, you get your answer. A program that runs out of
memory does not finish at all. And the wall arrives far sooner than you would
guess: the recursion limit on this machine is about twenty thousand frames, and a
linked list of a hundred thousand elements will walk straight through it without
slowing down.

The good news is that you already own every tool you need. Memory gets analyzed
exactly the way time did — same notation, same counting, same habit of throwing
away the constants and watching only the shape.

## The same four shapes, counting bytes

One adjustment: we usually count only the **extra** space, beyond whatever the
input already occupies.

**$O(1)$** — a fixed number of variables no matter how big the input gets. Bubble
sort, insertion sort, heap sort, and every loop you have ever written with a
counter and an accumulator.

**$O(\log n)$** — recursion $\log n$ deep, since every frame is sitting on the
stack taking up room. Quicksort, and binary search written recursively.

**$O(n)$** — a copy of the input, or an auxiliary array alongside it. Merge sort,
memoization tables, and any method that builds up a result list.

**$O(n^2)$** — a matrix. Adjacency matrices, dynamic programming tables laid across
two sequences.

## The stack is space too

This is the one people forget, and it is the one that bites.

The call stack from Chapter 12 is memory. So recursion depth *is* space complexity,
and a recursive method running $n$ frames deep is using $O(n)$ of it. Chapter 12
measured this machine's ceiling at 22,447 frames.

That is not a big number. Sit with it for a second before moving on, because it has
a consequence people meet the hard way: a recursive walk down a linked list of a
hundred thousand elements will overflow, while the iterative version of the same
walk does not even notice.

So here is a rule worth carrying out of this chapter. **A recursion whose depth is
proportional to the input is a program with a maximum input size**, and that
maximum is far lower than the amount of memory in the machine would ever suggest. A
recursion whose depth is $\log n$ — anything divide-and-conquer — is fine forever,
because $\log n$ of anything at all is a small number.

## Measured

Actual memory on this machine. Before you read the second line, guess how many
bytes an `ArrayList<Integer>` spends per element, given that an `int` is four.

```
int[10,000,000]        : 41,943,040 bytes (4.2 per element)
ArrayList<Integer> 1e6 : 20,300,512 bytes (20.3 per element)
```

Four bytes per `int` plus a small fixed overhead, exactly as Chapter 2 promised.
The array is precisely what it says it is.

And then twenty bytes per element for the list. Five times the memory, holding the
same numbers.

Every piece of that overhead comes from a chapter you have already read. Each
`Integer` is a full object, so it carries an object header — twelve to sixteen
bytes on a 64-bit JVM — wrapped around its four bytes of actual value, then padded
out to a multiple of eight. On top of that the list holds a *reference* to each
one, four more bytes with compressed pointers. Sixteen plus four, near enough.

That is Chapter 16's boxing showing up again, measured this time in bytes instead
of nanoseconds. It is why `int[]` exists, why `IntStream` exists, and why the
erasure rule of Chapter 27 that forbids `List<int>` has a price you can see in a
heap dump.

## The trade, which runs both ways

Time and space buy each other, in both directions. Recognizing which direction you
are standing in is more useful than either analysis on its own.

**Spend space, save time.** A cache. A hash index. The memoized Fibonacci of
Chapter 13, which turned $O(2^n)$ time into $O(n)$ time and $O(n)$ space. Any
precomputed lookup table.

**Spend time, save space.** Recompute instead of storing. Compress. Stream a file
rather than reading the whole thing — which is `Files.lines` against
`Files.readString` in Chapter 29, this exact trade with two method names.

Which side you want depends on which resource is scarce, and on a modern machine
that usually means memory when the data is large and time when it is small.

## Amortized cost, finally

One idea left, and it was promised back in Chapter 17.

`ArrayList.add` is normally $O(1)$: write into the next free slot, done.
Occasionally the array is full, so it has to allocate a bigger one and copy
everything across, which is $O(n)$.

So what *is* `add`? Neither answer is honest. Calling it $O(n)$ slanders it, since
it is almost never that. Calling it $O(1)$ ignores a real cost.

The answer is $O(1)$ **amortized** — over a long run of operations, the average is
constant. And the proof is that sum of powers of two you were told would come back.
Growing to capacity $n$ by doubling copies

$$1 + 2 + 4 + \cdots + n/2 = n - 1$$

elements in total, spread across $n$ additions. Fewer than one copy per addition,
on average, however large $n$ gets. The rare expensive operation is paid for in
advance by all the cheap ones around it.

Now here is why this is more than an accounting trick. It tells you that doubling
is the *right* growth rule and that growing by a constant is not. Suppose you grew
by ten each time instead. The copying becomes

$$10 + 20 + 30 + \cdots + n \approx n^2/20$$

which makes `add` $O(n)$ amortized and turns building a list into a quadratic
operation. The entire difference between an `ArrayList` that works and one that is
a disaster is which growth rule somebody picked — and the analysis above is how
they knew which to pick.

`HashMap` resizing runs on the same argument, and so does every dynamic array in
every language you will ever use.

## Three times when space is the whole problem

**The data does not fit.** A ten-gigabyte file on an eight-gigabyte machine has to
be streamed, and no amount of speed rescues you. The algorithm has to be one that
works in a single pass.

**Garbage collection pressure.** Allocating heavily costs time, but it costs it
*indirectly* — the collector has to run, and on a large heap it can stop your
program while it does. A program churning through a million short-lived objects a
second can be slow for reasons that no operation count will ever reveal to you.

**Cache.** Which is the next lesson, and the place where everything in this chapter
is least able to predict what actually happens.

Next: honesty about all of this.
