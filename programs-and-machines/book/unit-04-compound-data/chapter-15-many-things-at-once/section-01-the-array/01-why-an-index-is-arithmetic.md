# Why an Index Is Arithmetic

Memory is a numbered sequence of bytes. Chapter 1 called the numbering
*addressing* and said it was why widths had to be fixed. Here is the payoff.

## The layout

An array of five `int`s occupies twenty consecutive bytes — five elements, four
bytes each:

```
address:   1000  1004  1008  1012  1016
          ┌─────┬─────┬─────┬─────┬─────┐
          │  3  │  1  │  4  │  1  │  5  │
          └─────┴─────┴─────┴─────┴─────┘
index:       0     1     2     3     4
```

To find element *i*:

```
address = base + i × elementSize
```

For element 2: 1000 + 2 × 4 = 1008. One multiplication, one addition, done. No
searching, no comparison, no walking.

That is **constant-time access**, written O(1) in Chapter 32's notation, and it is
the property that makes arrays worth having.

## Why the constraints follow

Look at what the formula requires and every "restriction" of arrays becomes a
consequence.

**Elements must be the same type**, because the formula uses a single
`elementSize`. Mixed sizes would make position 47 unfindable without inspecting
the preceding 47 elements — which is linear, not constant, and defeats the point.

**Storage must be consecutive**, because the formula adds a simple offset. Scatter
the elements and there is no arithmetic that finds them.

**The size is fixed**, because consecutive storage means the space after the array
may belong to something else. Growing would require moving everything — which is
what Chapter 17's `ArrayList` does, and it is why growth costs something.

Three properties, one cause. Not arbitrary rules to memorize.

## Why indices start at 0

Now the question that puzzles everyone at first.

Because the index is an **offset** rather than a count. Element 0 is at
`base + 0 × size` — the beginning of the array, no offset at all. Element 1 is one
element along.

If indices started at 1, the formula would be `base + (i-1) × size`, and a
subtraction would happen on every single array access ever performed. That is a
real cost for no benefit, and the languages that made the other choice pay it or
work around it.

Dijkstra's argument from Chapter 9 gives the other half: half-open ranges make
counts subtract cleanly and adjacent ranges join without adjustment. Zero-based
indexing and `0 <= i < n` are the same convention seen from two directions, and
the loop

```java
for (int i = 0; i < a.length; i++)
```

is idiomatic because it fits both.

## What an array variable holds

Chapter 12's point, made concrete:

```java
int[] a = {3, 1, 4, 1, 5};
```

```
a: ┌────────┐          ┌───┬───┬───┬───┬───┐
   │ ref ───┼─────────▶│ 3 │ 1 │ 4 │ 1 │ 5 │
   └────────┘          └───┴───┴───┴───┴───┘
   stack                heap
```

The variable holds a reference. The array is on the heap, because its size is not
known when the method is compiled and a stack frame is fixed-size.

Consequences you already know from Chapter 12, now with a name attached:

```java
int[] b = a;      // copies the reference — one array, two names
b[0] = 99;
System.out.println(a[0]);      // 99
```

And a new one. The array itself stores its length:

```java
a.length      // 5
```

Note there are no parentheses. `length` is a field, not a method — unlike
`String.length()`, which is a method, and unlike `List.size()`, which is another
method. Three collections, three spellings, and the inconsistency is a historical
accident that everyone trips over and nobody can fix.

## What it costs

The honest other side.

**Fixed size.** Deciding the size in advance is frequently impossible, and it is
Chapter 17's whole subject.

**No insertion or removal.** Inserting at position 3 of a ten-element array means
shifting seven elements — linear work, where a linked structure would do it in
constant time. Unit IV's later chapters show what you use when that matters.

**Searching is linear.** Finding *whether* a value is present means looking at each
one. Constant-time access is by *position*, not by content, and confusing the two
is a common source of bad performance decisions. A `Map` — Chapter 17 — is what
you want when the question is about content.

Arrays are the right tool when you know the size, need access by position, and
iterate more than you insert. That is a common situation and far from a universal
one.

Next: writing them.
