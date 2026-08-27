# Choosing a Collection

The library offers a dozen or more types. Here is how to pick, and it is mostly
one question repeated.

## Start with the question

**"What am I asking of this data?"**

| the question | the answer |
|---|---|
| what is at position *i*? | `List` |
| is this present? | `Set` |
| what is associated with this key? | `Map` |
| what is the next one to handle? | `Deque` or `PriorityQueue` |

Get the shape right first. The implementation is a second, smaller decision.

## Then the implementation

**Lists.** `ArrayList` unless you have measured a reason. `LinkedList` is fast
where you already are and slow to reach anywhere — Section 17.1.3's measurement was
1 ms against 2,589 ms for 100,000 indexed reads. Its poor cache locality frequently
loses it even the cases theory awards it.

**Sets.** `HashSet` by default. `LinkedHashSet` when iteration should follow
insertion order. `TreeSet` when it should be sorted, and you accept
$\log_{2} n$ lookup for it.

**Maps.** The same three: `HashMap`, `LinkedHashMap`, `TreeMap`.

That is nearly the whole decision. Six types cover almost everything, and the
remainder — `ArrayDeque`, `PriorityQueue`, the concurrent collections of Chapter
31 — announce themselves when you need them.

## Costs, roughly

Chapter 32 gives the notation; the shape is useful now.

| operation | `ArrayList` | `LinkedList` | `HashSet`/`HashMap` | `TreeSet`/`TreeMap` |
|---|---|---|---|---|
| access by position | constant | linear | — | — |
| search by value | linear | linear | constant | logarithmic |
| add at end | amortized constant | constant | constant | logarithmic |
| insert at front | linear | constant | — | — |
| remove found item | linear | constant | constant | logarithmic |
| iterate in order | insertion | insertion | unspecified | sorted |

Read the rows rather than memorizing the table. The message is that **`ArrayList`
is fast for position and slow for content, and hashing is the reverse** — which is
Chapter 15's observation that constant-time access is by position and not by
content.

## The mistake to avoid

The single most common performance error at this level:

```java
List<String> names = ...;
for (String candidate : candidates) {
    if (names.contains(candidate)) {      // linear search, every time
        ...
    }
}
```

`contains` on a list examines every element. Inside a loop over *m* candidates and
a list of *n* names, that is *m* × *n* comparisons — Chapter 9's multiplication.

```java
Set<String> nameSet = new HashSet<>(names);
for (String candidate : candidates) {
    if (nameSet.contains(candidate)) {    // constant, every time
        ...
    }
}
```

One extra line, and *m* × *n* becomes *m* + *n*. For ten thousand of each, ten
million comparisons become twenty thousand.

**If you are searching a list inside a loop, you want a set or a map.** That
sentence catches a large fraction of the accidentally-quadratic code beginners
write.

## Immutable collections

```java
List<String> fixed = List.of("Ada", "Grace");
Set<String> s = Set.of(1, 2, 3);
Map<String, Integer> m = Map.of("a", 1, "b", 2);
```

These cannot be modified; every mutating method throws.

Prefer them for anything that should not change — constants, method returns
representing a snapshot, values shared across threads. Chapter 20 argues the case
properly; the short version is that a thing which cannot change is a thing you need
not track.

The trap, from Section 17.1.1: `List.of(...)` looks like a convenient way to make
a list and produces one you cannot add to. Wrap it in `new ArrayList<>(...)` when
you need a working list.

## Closing the chapter

Arrays cannot grow because their storage is consecutive. `ArrayList` grows by
replacing its array with a larger one, and the size it chooses is the interesting
part: **growing by a constant factor gives amortized constant time**, because the
total copying over *n* additions is less than *n*, while growing by a constant
amount gives quadratic behavior.

The library is organized around three questions — position, presence, association
— and answering the question chooses `List`, `Set`, or `Map`. Within each, hashing
is the default, insertion-ordered and sorted variants exist for when order matters,
and `LinkedList` is rarely the answer.

Generics are a promise about contents, enforced at compile time and then erased,
which is why `List<int>` is impossible and why you cannot create an array of a type
parameter.

And iteration has one trap: modifying a collection during an enhanced `for` breaks
the iterator's invariant and throws. Use `removeIf`, or the iterator's own
`remove`.

One kind of data remains, and it is the kind programs handle most. Text has been
appearing since Chapter 4 and we have never treated it as data to be manipulated.
That is the next chapter, and it will turn out to be a worked example of nearly
everything in this unit.
