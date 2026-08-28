# When Not To

A new tool arrives and gets used for everything for about two years. Streams landed
in Java 8 and those two years were memorable.

So here is the counterweight. It is argued with measurements rather than taste,
because the honest answer turns out to be more interesting than either "always" or
"never" — and because the real reason to avoid a stream is not the reason people
usually give.

## Start with the accusation everyone makes

"Streams are slow." Let us find out.

Ten million `int`s, filtered down to the even ones, squared, summed. Three
implementations, timed after warm-up on this machine. Before you look, put money on
the ordering.

```
loop    3 ms   IntStream    3 ms   Stream<Integer>   29 ms
loop    3 ms   IntStream    3 ms   Stream<Integer>   28 ms
loop    3 ms   IntStream    3 ms   Stream<Integer>   36 ms
```

Two conclusions, pointing in opposite directions.

**`IntStream` costs nothing whatsoever.** Three milliseconds against the loop's
three. The JIT inlines the lambdas, fuses the stages together, and emits
essentially the loop you would have written by hand. The abstraction is free. So
"streams are slow" is false as a general claim, and you can stop worrying about it.

**`Stream<Integer>` costs ten times as much.** Twenty-eight to thirty-six
milliseconds against three.

And notice what the difference is not. It is not the stream — the row above proves
that. It is the boxing: ten million `Integer` objects allocated, chased through a
pointer, and collected. That is the Chapter 16 measurement turning up again in a
new costume.

Which gives you a precise rule rather than a vague suspicion: **use the primitive
streams for numbers.** `mapToInt` is one word, and it is the entire distance
between those two rows.

So performance is hardly ever the reason to avoid a stream. Readability usually
is — and that is a harder argument, so it gets the rest of the lesson.

## Six times a loop is the clearer choice

**When what you are doing is not map, filter, or reduce.** Apply the test from
Section 26.2.1: name what each stage does. If a stage does a bit of everything, the
decomposition has failed and the pipeline is a disguise.

**When you need the index.** Streams have no natural notion of position.
`IntStream.range(0, list.size()).mapToObj(i -> ...)` does work, and it is worse
than a `for` loop with an `i` in it. Everyone agrees about this and people write it
anyway.

**When you are walking two things in step.** Java has no `zip`. Pairing two lists
element by element is a loop, and the alternatives produce something nobody enjoys
meeting in a review.

**When there is an early exit that depends on accumulated state.** `findFirst` and
`anyMatch` handle the easy cases. "Stop once the running total crosses a threshold"
does not fit, and `takeWhile` only helps when the condition depends on the element
alone.

**When you are mutating something.** A pipeline whose stages modify state is a loop
wearing more punctuation, and it has surrendered every property that made a
pipeline worth having in the first place.

**When the body is long.** A ten-line lambda buried in a pipeline is worse than a
ten-line loop body, because the loop's braces are structural and the pipeline's are
decorative. Extract the body into a named method, then come back and decide again —
you will often find the pipeline is fine once the body has a name.

## Debugging, which nobody warns you about

This is a real cost and it goes largely undiscussed.

Setting a breakpoint inside a lambda works. Stepping through a pipeline, however,
moves in an order that has nothing to do with the order of the source: laziness
means each element travels through *all* the stages before the next element starts,
so the debugger appears to leap around between lines at random. Stack traces show
synthetic frames with names like `lambda$main$0`, and a deep pipeline gives you a
trace that is mostly stream internals with your code somewhere inside it.

`peek` is the instrument for this. It lets you watch elements pass a point without
disturbing them — Section 26.2.2 used it to count what `findFirst` actually looked
at. Use it to diagnose, not in production.

And take the difficulty itself as evidence. **If a pipeline is hard to debug, it is
probably too long.** Three or four stages is comfortable. Past that, give an
intermediate result a name and let the reader breathe.

## The style question, answered honestly

Two versions of one computation:

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

The stream version is not clearly better, and you should be suspicious of anyone
who tells you it is. It is a line longer. It names each step, which is a genuine
gain. And it will read beautifully to somebody fluent in streams and poorly to
somebody who is not.

That last point is engineering, not a concession. Code is read far more often than
it is written, and the fluency that matters is the fluency of the people who will
read *this* code. On a team where streams are the house style, the stream version
is clearer, full stop. On a team where they are not, it is a small tax collected
from every future reader.

The honest position: both are fine, neither is worth an argument, and consistency
within a codebase is worth more than whichever one you personally prefer.

## And where streams win outright

Fairness in the other direction, because the list is real:

**Grouping and partitioning.** `groupingBy` in a single expression against six
lines of `computeIfAbsent`. This is the strongest case there is.

**Multi-stage transformations.** Four chained operations read better than four
nested loops or four intermediate lists, and it is not close.

**Reading files.** `Files.lines(path).filter(...).map(...)` is genuinely nicer than
the reader loop, and with try-with-resources it closes the file for you.

**Anything needing `flatMap`.** Flattening a nested structure is awkward as a loop
and natural as a pipeline.

## The part of this chapter that outlives the syntax

If you remember one thing from these lessons, do not let it be the syntax. Let it
be the discipline named in Section 26.1.3.

**Write pure functions wherever you can.** Read the arguments, return a value,
touch nothing else. A method like that is testable with no setup, safe to call
twice, safe to move, and safe on several threads at once.

**Keep the pure part away from the effectful part.** A program that reads input,
computes, and writes output should hold the computation in pure methods with the
reading and writing pushed out to the edges. That shape is testable in the middle,
which is where all the logic lives — and it is Chapter 23's "do not read the clock
inside a class", generalized to the whole program.

**Prefer immutable data.** This is Chapter 20's argument, and it is what makes
purity reachable rather than aspirational: a function cannot accidentally modify
what it is not able to modify.

Here is the thing worth noticing about all three. **None of them needs a lambda.**
Every one was available in the Java of Chapter 11. What this chapter actually
contributed is that lambdas make the pure parts small enough to pass around
comfortably — which quietly turns a discipline into a convenience, and that is why
the ideas spread.

John Backus argued for exactly this in his 1977 Turing Award lecture, the one
Chapter 24 recommended: that assignment and iteration make programs hard to reason
about, and that composing functions is a better foundation to build on. He
overstated the case, and the language he proposed found no users at all. But the
diagnosis was right, and thirty years later every mainstream language had grown the
features in this chapter.

Chapter 27 closes the unit by turning the mirror around: a program that examines
not its data, but itself.
