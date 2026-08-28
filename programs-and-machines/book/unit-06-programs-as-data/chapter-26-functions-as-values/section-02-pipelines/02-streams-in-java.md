# Streams in Java

Now the syntax, and one warning that prevents most of the confusion.

```java
int total = ns.stream()
              .filter(n -> n % 2 == 0)
              .map(n -> n * n)
              .reduce(0, Integer::sum);
```

Verified: `220`, the same as the loop.

There are the previous lesson's three operations, in Java's own notation, and
nothing in that pipeline runs until the last line of it.

A **stream** holds no elements. It is not a collection that has been made
fashionable; it is a *description of a computation that has not happened yet*.

Every surprising thing about streams falls out of that one fact, so it is worth
fixing before the syntax arrives.

## The three parts of a pipeline

```java
ns.stream()                        // source
  .filter(n -> n % 2 == 0)         // intermediate operation
  .map(n -> n * n)                 // intermediate operation
  .reduce(0, Integer::sum);        // terminal operation
```

**A source** — `collection.stream()`, `Arrays.stream(array)`, `Stream.of(...)`,
`IntStream.range(0, n)`, or a file's lines.

**Intermediate operations** return another stream. `filter`, `map`, `sorted`,
`distinct`, `limit`, `skip`, `flatMap`, `peek`. Any number of them, in any order.

**A terminal operation** produces a result and ends the stream. `reduce`, `sum`,
`count`, `collect`, `toList`, `forEach`, `findFirst`, `anyMatch`, `min`, `max`.

From which: **nothing runs until the terminal operation does.**

Write a pipeline and forget the last line, and it does not fail, and it does not
warn you. It does nothing at all, in complete silence. Most people make that
mistake exactly once.

## Laziness

Streams do not process stage by stage. They pull elements through the whole
pipeline one at a time, and stop as soon as the answer is known.

Verified, over the list 1 to 10:

```java
List<Integer> seen = new ArrayList<>();
Optional<Integer> first = ns.stream()
    .peek(seen::add)
    .filter(n -> n > 3)
    .findFirst();
```

```
found 4 after examining [1, 2, 3, 4]
```

Four elements were examined out of ten. `findFirst` stopped the moment it had an
answer, and the remaining six were never touched.

The hand-written `filter` from the last lesson could not do this. It builds the
whole filtered list and then `findFirst` looks at element zero, having done ten
times the necessary work. On a list of ten that is nothing; on a million-element
list where the answer is early, it is the difference between instant and slow.

Laziness also means **no intermediate collections**. A four-stage pipeline over a
million elements allocates nothing per stage; each element passes through all four
before the next begins. That fusion is why streams are competitive with loops at
all.

## Short-circuiting

The same mechanism, named:

```java
ns.stream().anyMatch(n -> n > 3)      // stops at the first match
ns.stream().allMatch(n -> n > 0)      // stops at the first failure
ns.stream().limit(3)                  // stops after three
ns.stream().findFirst()               // stops at one
```

These make infinite streams usable:

```java
Stream.iterate(1, n -> n * 2).limit(10).toList()
```

A stream of every power of two, cut to ten. The source is infinite and the
pipeline terminates, because nothing is computed until something asks.

## Collectors

`reduce` handles combining into one value. `collect` handles building a
collection, and it is where most real stream code ends.

Verified, on four people:

```java
people.stream().map(Person::name).collect(Collectors.joining(", "))
```
```
Ada, Grace, Alan, Katherine
```

```java
people.stream().collect(Collectors.groupingBy(Person::city, Collectors.counting()))
```
```
{New York=1, London=2, Hampton=1}
```

That second one is worth staring at. It builds a `Map<String, Long>` grouped by
city, counting each group, in one expression. Written as a loop it is six lines
with a `computeIfAbsent` or a `getOrDefault`, and the six lines are all bookkeeping.

More:

```java
people.stream().mapToInt(Person::age).average().getAsDouble()   ->  43.5

people.stream().filter(p -> p.age() > 40).map(Person::name).toList()
                                                 ->  [Grace, Alan, Katherine]
```

`toList()` since Java 16 replaces `collect(Collectors.toList())` and returns an
immutable list, which is the better default. The older form returns a mutable
`ArrayList` and is still needed when you want one.

The `Collectors` class has about forty factory methods. The ones worth knowing:
`toList`, `toSet`, `toMap`, `joining`, `groupingBy`, `partitioningBy`, `counting`,
`summingInt`, `averagingInt`. The rest are compositions of those.

## Optional

`findFirst` returned an `Optional<Integer>`, not an `Integer`, because the stream
might have been empty.

```java
Optional<Integer> first = ...;
first.isPresent()             // is there one?
first.get()                   // the value, throwing if absent
first.orElse(0)               // the value or a default
first.map(n -> n * 2)         // transform if present
first.ifPresent(System.out::println)
```

`Optional` is a container holding zero or one values, and its purpose is to make
"there might not be an answer" visible in the type rather than represented by
`null`. A method returning `Optional<Person>` cannot be used without acknowledging
the empty case; a method returning `Person` can return `null` and nothing warns
you.

Two pieces of guidance, both widely agreed and widely ignored:

**Use it as a return type, not as a field or a parameter.** A field of type
`Optional` adds a wrapper object per instance and a second way of being absent. A
parameter of type `Optional` forces every caller to wrap.

**Avoid `get()` without checking.** It throws `NoSuchElementException`, which is
`NullPointerException` with a longer name. `orElse`, `orElseThrow` with a message,
or `ifPresent` are all better.

## Primitive streams

```java
ns.stream().mapToInt(n -> n * n).sum()
```

`IntStream`, `LongStream` and `DoubleStream` exist separately from `Stream<T>`,
because `Stream<Integer>` boxes every element. `mapToInt` converts, `boxed()`
converts back, and `sum`, `average`, `max` and `summaryStatistics` are available
only on the primitive versions.

Use them whenever the elements are numbers. Section 26.2.3 measures why.

## Parallel streams

```java
list.parallelStream().filter(...).map(...).reduce(...)
```

One word, and the pipeline runs across all available cores.

That it is one word is the danger. It works only if every function in the pipeline
is pure and the reduction is associative, and if either fails you get a wrong
answer rather than an error — as Section 26.1.2's counter demonstrated, three
times, with three different wrong answers.

Chapter 31 gives this properly. Until then, the safe rule: **do not write
`parallelStream` yet.** The cases where it helps are narrower than they look —
large data, expensive per-element work, no shared state — and the cases where it
silently corrupts are easy to reach.

Next: when to use none of this.
