# Traversal Patterns

There are two ways to visit every cell of a grid. They do identical work — the same
number of reads, the same additions, the same total — and one of them is about
three times slower than the other on a large enough grid.

Here they are. Before reading on, decide which you would guess is which.

```java
// row by row
for (int r = 0; r < n; r++)
    for (int c = 0; c < n; c++)
        total += grid[r][c];

// column by column
for (int c = 0; c < n; c++)
    for (int r = 0; r < n; r++)
        total += grid[r][c];
```

Same cells, same count, same arithmetic, same answer.

On a 4000 × 4000 grid, measured on the machine this book was written on:

```
row by row       11.1 ms
column by column 33.7 ms
ratio            3.0x
```

Three times slower for visiting the same sixteen million cells in a different
order.

## Why

Because memory is not uniformly fast, and the reason is a hierarchy the
programming model hides.

A processor does not fetch one byte at a time. It fetches a **cache line** —
typically 64 bytes — into a small, very fast memory close to the core. Reading any
byte of that line afterwards is nearly free; reading a byte from a different line
means going out to main memory, which is on the order of a hundred times slower.

Now consider the two loops.

**Row by row** walks along one inner array, consecutively. Reading `grid[r][0]`
pulls in a cache line containing the next fifteen or so `int`s as well, so the
next fifteen accesses are free. One slow fetch per sixteen elements.

**Column by column** reads `grid[0][c]`, then `grid[1][c]` — a different array,
somewhere else entirely in the heap. Each access is a different cache line, so
each is a slow fetch. Sixteen times as many.

The measured ratio is 3x rather than 16x because the processor also *prefetches* —
it notices the pattern and fetches ahead — and because other costs are involved.
But the effect is unmistakable and it is entirely about memory layout.

This is called **locality of reference**, and it is one of the few
performance considerations worth knowing before you know anything else about
performance.

## The rule

**Walk data in the order it is stored.**

For Java's arrays-of-arrays, that means row index outermost, column index
innermost. The inner loop should vary the *last* index.

It applies more generally than grids. Iterating a collection in its natural order
is faster than jumping about in it, and this is part of why an `ArrayList` often
outperforms a `LinkedList` even for operations where the linked structure has the
better theoretical cost — Chapter 17 returns to it, and Chapter 32 gives the
vocabulary for why "theoretical cost" and "measured cost" can disagree.

## Other patterns

Some traversals you will write.

**The diagonal**, where row and column advance together:

```java
for (int i = 0; i < n; i++)
    total += grid[i][i];
```

**The upper triangle**, visiting each unordered pair once:

```java
for (int r = 0; r < n; r++)
    for (int c = r + 1; c < n; c++)
        consider(grid[r][c]);
```

Note `c = r + 1` — starting the inner loop from the outer index is the idiom for
"each pair once", and it is worth recognizing on sight. It halves the work, which
Chapter 32 will say does not change the complexity and does change the wall clock.

**Neighbors**, for grids where cells interact:

```java
for (int dr = -1; dr <= 1; dr++) {
    for (int dc = -1; dc <= 1; dc++) {
        if (dr == 0 && dc == 0) continue;          // skip the cell itself
        int nr = r + dr, nc = c + dc;
        if (nr >= 0 && nr < grid.length && nc >= 0 && nc < grid[nr].length) {
            consider(grid[nr][nc]);
        }
    }
}
```

Longer than it looks like it should be, and every line is necessary: the offsets
generate the eight neighbors, the `continue` skips the center, and the bounds
check handles cells at the edges. Cellular automata, image filters, and grid-based
games are all built on this shape, and getting the edge handling right is most of
the work.

## Closing the chapter

An array is a fixed-size sequence of same-typed values stored consecutively, and
every one of those constraints follows from one property: the index is arithmetic,
so access by position costs the same regardless of position.

Indices start at 0 because an index is an offset. Arrays cannot grow because the
space after them belongs to someone else. Elements share a type because the
address formula needs one size. Java checks every access against the bounds, which
costs a comparison the JIT can often remove, and which eliminates the buffer
overflow that accounts for a large share of the security defects in languages
without it.

Two-dimensional arrays are arrays of arrays — four objects for a 3 × 4 grid — so
rows are objects and columns are not, rows may differ in length, and copying one
level does not copy the rest. And the order you walk a grid can change its speed
threefold, because memory is a hierarchy and consecutive access is what it
rewards.

What an array still cannot do is grow, or say what it means. The next chapter takes
the second of those, and the one after takes the first.
