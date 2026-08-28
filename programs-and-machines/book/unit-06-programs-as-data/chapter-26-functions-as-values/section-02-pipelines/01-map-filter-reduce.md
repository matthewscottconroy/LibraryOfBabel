# Map, Filter, Reduce

Here is a claim you are entitled to be skeptical about.

Almost every loop you have ever written does one of three things, or some
combination of them. Three. Not thirty, not a family of patterns with exceptions —
three operations, and everything else is those three wearing different clothes.

The useful part of that claim is not the taxonomy, and if it were only a taxonomy
it would not be worth a lesson. The useful part is what it lets you *see*: once you
can tell which of the three a loop is doing, you can also tell when a loop is doing
two of them at once and would be clearer as two loops, or as none.

**Map** — build a new collection by transforming each element. Same size, different
values.

**Filter** — build a new collection of the elements that pass a test. Same values,
fewer of them.

**Reduce** — combine every element into a single value. A sum, a maximum, a
concatenation, a count.

They were named in Lisp in the 1960s and they are the spine of functional
programming. Once you can see them, you will start noticing how much of any
codebase is these three in disguise, and the noticing does not switch off again.

## Map

```java
static <T, R> List<R> map(List<T> in, Function<T, R> f) {
    List<R> out = new ArrayList<>(in.size());
    for (T x : in) out.add(f.apply(x));
    return out;
}
```

Every element goes through `f`. The result is the same length, and — this is the
part that is a discipline rather than an implementation detail — **the input is
untouched.**

You could write this to modify in place. It would be faster and it would allocate
nothing. What it would also do is destroy the caller's ability to have both
versions, and Chapter 20's argument for immutability applies here without a single
word changed.

## Filter

```java
static <T> List<T> filter(List<T> in, Predicate<T> p) {
    List<T> out = new ArrayList<>();
    for (T x : in) if (p.test(x)) out.add(x);
    return out;
}
```

Same elements, each one kept or dropped. The result is shorter, or the same length,
never longer.

## Reduce

This is the one that takes a moment, so give it one:

```java
static <T> T reduce(List<T> in, T identity, BinaryOperator<T> op) {
    T acc = identity;
    for (T x : in) acc = op.apply(acc, x);
    return acc;
}
```

An accumulator, a starting value, and a rule for combining two things into one.
That is all there is, and yet every aggregate computation you have ever written is
this with different arguments filled in:

| computation | identity | operator |
|---|---|---|
| sum | 0 | `a + b` |
| product | 1 | `a * b` |
| maximum | `MIN_VALUE` | `Math.max` |
| count | 0 | `a + 1` |
| concatenation | `""` | `a + b` |

Read down that table and see how little the shape changes. The loop was never the
interesting part. The two arguments were.

This is also the pattern Chapter 13 called **accumulator passing**, and told you
that you would meet again — carry the answer forward with you rather than leaving
work stacked up behind you. Here is the promised meeting. And notice something that
was not true there: the accumulator is now the *only* state in the entire
operation. That is precisely why this pattern parallelizes and an ordinary loop
does not.

One warning about the identity, because it produces a bug that hides well.
Reducing an empty list gives you the identity back, so the value has to be
genuinely neutral — 0 for addition, 1 for multiplication, not whatever seemed
convenient. Choose it wrong and empty inputs give wrong answers, quietly, while
every test you wrote with real data passes.

## Putting the three together

Here is an ordinary loop:

```java
int total = 0;
for (int n : ns) if (n % 2 == 0) total += n * n;
```

Verified for 1 through 10: `220`.

Now count the decisions inside that one-line body. There are three: a condition, a
transformation, and an accumulation. They are all in there, interleaved, and you
separate them by reading carefully.

Pull them apart and they are exactly our three operations:

```
[1..10]  --filter even-->  [2,4,6,8,10]
         --map square-->   [4,16,36,64,100]
         --reduce sum-->   220
```

Same answer. The difference is not correctness, and it is not length. The
difference is that the loop *contains* three decisions and the pipeline *names*
them.

And once they are named they come apart. Each stage can be tested on its own,
replaced on its own, reused somewhere else. Change the filter and nothing else in
the pipeline moves.

## Three payoffs beyond legibility

**Each stage is a pure function**, so each can be reasoned about entirely by
itself. That is Section 26.1.3's argument collecting its dividend.

**The traversal stops being your business.** Look again at the loop: it specifies
*how* to iterate — one at a time, in order, from the front. The pipeline says only
what to do with each element and leaves the how to the library. Which frees the
library to iterate lazily, or in parallel, or to fuse the stages together so that
no intermediate collection is ever built. Java's streams do all three, and they can
only do it because you did not insist on the order.

**Fewer opportunities for an off-by-one.** The loop has an index or an iterator.
The pipeline has neither, and you cannot get wrong what you did not write.

## The counterweight, offered in advance

Not every loop is one of these three, and forcing one into the shape is a mistake
that produces worse code than you started with.

A loop with an early exit that depends on accumulated state. A loop walking two
collections in step. A loop building a result whose shape depends on what it has
already seen. Those are loops. Write them as pipelines and you will get something
longer and harder to read, and you will have paid for the privilege.

The test is short: **can you name what each stage does?** If your answer for some
stage is "well, it does a bit of everything", the decomposition has failed and the
loop was the right answer all along. Section 26.2.3 makes this argument properly.

Next: Java's version, which adds three things worth knowing.
