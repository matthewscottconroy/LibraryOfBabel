# Map, Filter, Reduce

Almost every loop you have written does one of three things, or a combination.

**Map** — produce a new collection by transforming each element. Same size,
different values.

**Filter** — produce a new collection containing the elements that satisfy a
test. Same values, fewer of them.

**Reduce** — combine all the elements into one value. A sum, a maximum, a
concatenation, a count.

Three operations. They were named in Lisp in the 1960s, they are the core of
functional programming, and once you can see them you will notice that a large
proportion of the loops in any codebase are one of the three with the wrong
clothes on.

## Map

```java
static <T, R> List<R> map(List<T> in, Function<T, R> f) {
    List<R> out = new ArrayList<>(in.size());
    for (T x : in) out.add(f.apply(x));
    return out;
}
```

Every element goes through `f`. The result is the same length, and the input is
untouched.

That the input is untouched is the discipline. A loop that modifies in place is
faster and prevents the caller from having both versions. Map's version is safer
and Chapter 20's immutability argument applies unchanged.

## Filter

```java
static <T> List<T> filter(List<T> in, Predicate<T> p) {
    List<T> out = new ArrayList<>();
    for (T x : in) if (p.test(x)) out.add(x);
    return out;
}
```

Same elements, kept or dropped. The result is shorter or equal.

## Reduce

The one that takes a moment:

```java
static <T> T reduce(List<T> in, T identity, BinaryOperator<T> op) {
    T acc = identity;
    for (T x : in) acc = op.apply(acc, x);
    return acc;
}
```

An accumulator, an initial value, and a rule for combining. Every aggregate
computation is this with different arguments:

| computation | identity | operator |
|---|---|---|
| sum | 0 | `a + b` |
| product | 1 | `a * b` |
| maximum | `MIN_VALUE` | `Math.max` |
| count | 0 | `a + 1` |
| concatenation | `""` | `a + b` |

This is the shape Chapter 13 called **accumulator passing** and said you would
meet again — carry the answer forward rather than leaving work pending. Here is
the promised meeting, and note that the accumulator is now the *only* state, which
is why the pattern parallelizes at all.

The identity matters. Reducing an empty list gives the identity, so the value must
be genuinely neutral: 0 for addition, 1 for multiplication. Choose wrong and empty
inputs give wrong answers, which is a bug that survives every test with non-empty
data.

## Composing the three

The loop:

```java
int total = 0;
for (int n : ns) if (n % 2 == 0) total += n * n;
```

Verified for 1 through 10: `220`.

Decomposed: filter to the evens, map to squares, reduce by addition.

```
[1..10]  --filter even-->  [2,4,6,8,10]
         --map square-->   [4,16,36,64,100]
         --reduce sum-->   220
```

Same answer, and the difference is what you can see. The loop mixes three
decisions into one body — a condition, a transformation, and an accumulation — and
you separate them by reading. The pipeline names them.

The other difference is that each stage is independently testable, replaceable,
and reusable. Change the filter and nothing else moves.

## Why the decomposition matters

Three payoffs beyond legibility.

**Each stage is a pure function**, so each can be reasoned about alone. Section
26.1.3's argument, applied.

**The traversal is not your business.** The loop specifies *how* to iterate —
index, order, one at a time. The pipeline says only what to do with each element,
which leaves the library free to iterate differently: lazily, in parallel, or by
fusing the stages so no intermediate collection is built. Java's streams do all
three.

**Fewer places to make an off-by-one error.** The loop has an index or an iterator;
the pipeline has neither.

## The counterweight, in advance

Not every loop is one of these three, and forcing it is a mistake.

A loop with an early exit that depends on accumulated state, a loop over two
collections in step, a loop that builds a result whose shape depends on what it has
seen — these are loops, and writing them as pipelines produces something longer and
harder to read.

The test: **can you name what each stage does?** If the answer for some stage is
"it does a bit of everything", the decomposition has failed and the loop was
right. Section 26.2.3 develops this.

Next: Java's version, which adds three things worth knowing.
