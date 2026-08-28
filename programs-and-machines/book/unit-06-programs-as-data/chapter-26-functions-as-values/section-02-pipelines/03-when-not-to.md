# When Not To

A new tool arrives and gets used for everything for about two years. Streams
arrived in Java 8 and the two years were memorable.

So here is the counterweight, argued with measurements rather than taste — because
the honest answer turns out to be more interesting than either "always" or
"never", and the reason streams are sometimes wrong is not the reason people
usually give.

Streams arrived in Java 8 and were immediately overused. This lesson is the
counterweight, and the argument is made with measurements where measurements are
available.

## The performance question

Ten million `int`s, filtered to the evens, squared, summed. Three
implementations, timed after warm-up on this machine:

```
loop    3 ms   IntStream    3 ms   Stream<Integer>   29 ms
loop    3 ms   IntStream    3 ms   Stream<Integer>   28 ms
loop    3 ms   IntStream    3 ms   Stream<Integer>   36 ms
```

Two conclusions, and they point in opposite directions.

**`IntStream` costs nothing.** Three milliseconds against three. The JIT inlines
the lambdas, fuses the stages, and produces essentially the loop. The stream
abstraction is free here, and "streams are slow" is false as a general claim.

**`Stream<Integer>` costs ten times as much.** Twenty-eight to thirty-six
milliseconds against three. The difference is not the stream; it is the boxing —
ten million `Integer` objects allocated, dereferenced, and collected.

That is Chapter 16's measurement again, in a new place, and it gives a precise
rule: **use the primitive streams for numbers.** `mapToInt` is one word and it is
the difference between the two rows.

So performance is rarely the reason to avoid a stream. Readability usually is.

## When a loop is clearer

**When the loop is not map, filter, or reduce.** Section 26.2.1's test: name what
each stage does. If a stage does a bit of everything, the decomposition failed.

**When you need the index.** Streams have no natural index. `IntStream.range(0,
list.size()).mapToObj(i -> ...)` works and is worse than a `for` loop with an `i`
in it.

**When you are iterating two things in step.** Java has no `zip`. Pairing two
lists element by element is a loop, and pretending otherwise produces something
nobody enjoys reading.

**When there is an early exit with a condition on accumulated state.** `findFirst`
and `anyMatch` cover simple cases; "stop when the running total exceeds a
threshold" does not fit, and `takeWhile` only fits when the condition depends on
the element alone.

**When you are mutating.** A pipeline whose stages modify things is a loop written
with more punctuation, and it has given up every property that made the pipeline
worth having.

**When the loop body is long.** A ten-line lambda inside a pipeline is worse than
a ten-line loop body, because the loop's braces are load-bearing and the
pipeline's are decoration. Extract the body to a named method and then decide
again.

## Debugging

A real and under-discussed cost.

Setting a breakpoint inside a lambda works, but stepping through a pipeline moves
in an order that does not match the source: laziness means the elements go through
all the stages one at a time, so the debugger appears to jump between lines. Stack
traces show synthetic frames with names like `lambda$main$0`, and a deep pipeline
produces a trace that is mostly stream internals.

`peek` is the tool for this — it lets you observe elements passing a point without
changing them, and Section 26.2.2 used it to count what `findFirst` examined. It
is for diagnosis, not production.

If a pipeline is hard to debug, that is evidence it is too long. Three or four
stages is comfortable. Beyond that, name an intermediate result.

## The style question

Two versions of the same computation:

```java
// loop
int total = 0;
for (Order o : orders)
    if (o.isShipped()) total += o.total();

// stream
int total = orders.stream()
    .filter(Order::isShipped)
    .mapToInt(Order::total)
    .sum();
```

The stream version is not clearly better. It is one line longer, it names each
step, and it will read better to someone who is fluent and worse to someone who
is not.

That last point is a real engineering consideration rather than a concession. Code
is read more than written, and the relevant fluency is that of the people who will
read it. On a team where streams are the house style, the stream version is
clearer. On a team where they are not, it is a small tax on every reader.

The honest position is that both are fine, that neither is worth arguing about,
and that consistency within a codebase is worth more than either.

## Where streams clearly win

To be fair in the other direction:

**Grouping and partitioning.** `groupingBy` in one expression against six lines of
`computeIfAbsent`. This is the single strongest case.

**Multi-stage transformations.** Four operations chained read better than four
nested loops or four intermediate lists.

**Reading files.** `Files.lines(path).filter(...).map(...)` is genuinely nicer than
the reader loop, and it closes the file if you use try-with-resources.

**Anything with `flatMap`.** Flattening nested structures is awkward as a loop and
natural as a pipeline.

## Functional style beyond streams

The most valuable idea in this chapter is not the syntax. It is the discipline
Section 26.1.3 named:

**Write pure functions where you can.** Read the arguments, return a value, touch
nothing else. Such a method is testable without setup, safe to call twice, safe to
move, and safe on several threads.

**Separate the pure part from the effectful part.** A program that reads input,
computes, and writes output should have the computation in pure methods and the
reading and writing at the edges. That structure is testable in the middle, which
is where the logic is, and it is Chapter 23's "do not read the clock inside a
class" generalized.

**Prefer immutable data.** Chapter 20's argument, and it is what makes purity
achievable — a function cannot accidentally modify what it cannot modify.

None of these require a lambda. They are available in the Java of Chapter 11, and
this chapter's real contribution is that lambdas make the pure parts small enough
to pass around, which turns a discipline into a convenience.

Backus argued for exactly this in his 1977 Turing Award lecture, which Chapter 24
recommended: that assignment and iteration make programs hard to reason about, and
that composing functions is the better foundation. He overstated it — the language
he proposed found no users — but the diagnosis was right, and thirty years later
every mainstream language grew the features in this chapter.

Chapter 27 closes the unit by turning the mirror the other way: a program that
examines not its data but itself.
