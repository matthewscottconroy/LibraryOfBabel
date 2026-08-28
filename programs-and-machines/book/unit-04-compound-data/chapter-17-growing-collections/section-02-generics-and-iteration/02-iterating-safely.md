# Iterating Safely

There are three ways to walk a collection. Two are fine. The third looks like the
most natural thing in the world and has a trap in it that fires only sometimes,
which is the worst way for a trap to behave.

Start with the one you already use:

```java
for (String name : names) {
    System.out.println(name);
}
```

The enhanced `for` works on arrays and on anything implementing `Iterable`, which
every collection does. It is the default, for Chapter 9's reason: no index, no
index error.

## What it compiles to

For a collection, the loop above becomes:

```java
Iterator<String> it = names.iterator();
while (it.hasNext()) {
    String name = it.next();
    System.out.println(name);
}
```

An **iterator** is an object that walks a collection. Two operations: `hasNext()`
asks whether anything remains, `next()` returns the next element and advances.

Note that `next()` does two things — returns a value and moves — which Chapter 14
identified as a command–query violation. It is, and it is deliberate: the interface
predates the advice and changing it now is impossible. It is a fair example of a
design that is universally used and not defensible on first principles.

Knowing the iterator exists matters for one reason: it explains the trap.

## The trap

```java
for (String n : names) {
    if (n.equals("Ada")) names.remove(n);
}
```

```
ConcurrentModificationException
```

Nothing concurrent is happening — there is one thread. The name is misleading and
the meaning is *the collection changed while it was being iterated*.

**Why.** The iterator holds a position. Removing an element shifts everything after
it, so the iterator's position now refers to a different element — it would skip
one, or run past the end. Rather than proceed with a broken position, the
collection detects the modification and throws.

This is Chapter 16's representation invariant being enforced. The iterator's
invariant — *my position is valid* — was broken by something outside it, and
failing loudly is the right response. Chapter 11 called it failing fast.

## Removing while iterating

Three correct approaches.

**Use the iterator's own `remove`:**

```java
Iterator<String> it = names.iterator();
while (it.hasNext()) {
    if (it.next().equals("Ada")) {
        it.remove();
    }
}
```

The iterator knows it removed something and adjusts its position. This is why
`Iterator` has a `remove` method at all.

**Use `removeIf`, which is clearest:**

```java
names.removeIf(n -> n.equals("Ada"));
```

The `n -> ...` is a lambda, Chapter 26. Read it as "for each n, is it Ada?".

**Collect and remove afterwards**, when the condition is complicated:

```java
List<String> doomed = new ArrayList<>();
for (String n : names) {
    if (complicated(n)) doomed.add(n);
}
names.removeAll(doomed);
```

**What does not work** is setting a flag and removing after the loop *ends* if you
also `break` — that is fine — or removing inside and hoping. Hoping does not work.

## Iterating a map

Three ways, for three questions:

```java
for (String key : counts.keySet())        { ... }
for (Integer value : counts.values())     { ... }
for (Map.Entry<String, Integer> e : counts.entrySet()) {
    System.out.println(e.getKey() + " = " + e.getValue());
}
```

Use `entrySet` when you need both. The alternative — iterating keys and calling
`get` for each — does the lookup work twice and is a small, common inefficiency.

## Order

Something worth being explicit about, because it causes real bugs.

**`ArrayList` iterates in insertion order.** Reliable.

**`HashMap` and `HashSet` iterate in an unspecified order.** Not insertion order,
not sorted order, and **not guaranteed to be stable across Java versions or even
across runs**. The `[Grace, Ada]` from Section 17.1.3 came out reversed from the
insertion order, and that is entirely legitimate.

Code that depends on hash iteration order is broken and may pass every test on
your machine. If you need order, say so with `LinkedHashMap` for insertion order
or `TreeMap` for sorted order.

**`TreeMap` and `TreeSet` iterate in sorted order.** Reliable, and the reason to
accept their slower lookup.

## Iterating safely, summarized

**Prefer the enhanced `for`.** No index, no index error.

**Never modify a collection during an enhanced `for` over it.** Use `removeIf`, or
the iterator's `remove`, or collect and remove afterwards.

**Use `entrySet` when you need keys and values.**

**Do not depend on hash ordering.** If order matters, choose a type that promises
one.

Next: choosing.
