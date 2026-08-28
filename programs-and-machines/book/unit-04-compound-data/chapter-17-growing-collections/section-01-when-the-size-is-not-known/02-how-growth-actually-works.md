# How Growth Actually Works

There is something faintly impossible about the last lesson.

An `ArrayList` stores its elements in an array. Chapter 15 established that an
array's length is fixed when it is created, and that this is not an oversight but
the price of `base + i * size` — you cannot guarantee contiguity while growing
without moving everything.

So an `ArrayList` is a growable thing built out of an ungrowable thing. How?

It does not. **It replaces the array with a bigger one**, and the interesting part
is how often.

## The representation

Two fields, and Chapter 16's invariant:

```java
Object[] elements;      // the storage, usually larger than needed
int size;               // how many are actually in use
```

> **Representation invariant:** `0 <= size <= elements.length`;
> `elements[0..size-1]` are the elements; `elements[size..]` are unused.

That is exactly the `NameSet` invariant from Section 16.1.2, which was not a
coincidence — it is the standard shape for a growable sequence.

The distinction between `size` (how many you have) and `elements.length` (how many
you have room for, the **capacity**) is the whole mechanism.

## Adding

```java
void add(E item) {
    if (size == elements.length) grow();
    elements[size] = item;
    size++;
}
```

Most calls do nothing but two assignments. Occasionally the array is full and must
be replaced.

## The question that matters

By how much should it grow?

**Add one each time.** Every `add` allocates a new array and copies everything.
Adding *n* items copies 1 + 2 + 3 + … + *n* elements, which is about $n^{2}/2$.
For a million items that is five hundred billion copies. Unusable.

**Add a fixed amount, say 100.** Better — a copy every hundredth add — but still
proportional to $n^{2}$, with a smaller constant. A million items still costs about
five billion copies.

**Double it.** This is what `ArrayList` does, and the result is dramatically
different.

## Why doubling works

Start at capacity 1 and add items. Growth happens at sizes 1, 2, 4, 8, 16, …, and
each growth copies the current contents:

```
grow to 2:    copy 1
grow to 4:    copy 2
grow to 8:    copy 4
grow to 16:   copy 8
...
grow to n:    copy n/2
```

Total copying is 1 + 2 + 4 + … + n/2, which is **less than *n***. Chapter 2's
all-ones fact: a sum of powers of two up to n/2 is n − 1.

So adding *n* items costs fewer than *n* copies **in total**. Averaged over the
adds, each one costs a constant amount, even though individual adds occasionally
copy a great deal.

That is called **amortized constant time**, and it is worth having the phrase. Any
single `add` may be expensive; a sequence of *n* of them is cheap per add, because
expensive ones are rare enough and get rarer as the list grows.

Measured: two million `add` calls took about 30 ms on the machine used for this
book, which is roughly 15 nanoseconds each including the boxing.

## The general principle

**Growing by a constant factor gives amortized constant time; growing by a
constant amount does not.**

The factor need not be 2. Java's `ArrayList` grows by about 1.5×, which wastes
less space at the cost of slightly more copying. Some implementations use 2, some
1.5, some the golden ratio for reasons involving memory reuse. Any factor greater
than 1 gives the amortized result; the choice trades copying against waste.

This is one of the few genuinely important algorithmic ideas that fits in a
lesson, and it recurs — hash tables resize this way, and so do growable buffers in
every language.

## What it costs

**Wasted space.** A list of 1000 elements may have capacity 1500. Up to a third of
the array is unused, and after doubling it can be up to half. Usually irrelevant;
occasionally not, and `trimToSize()` exists for the occasions.

**Occasional pauses.** Most adds are fast and one in a while copies the entire
contents. For a very large list that is a noticeable hitch, which matters in
interactive or real-time contexts.

**Reference churn.** The old array becomes garbage. Growing a large list
repeatedly gives the garbage collector work.

If you know the eventual size, say so:

```java
List<String> names = new ArrayList<>(10000);
```

That is the initial *capacity*, not the size — the list is still empty. It avoids
the growth sequence entirely, and it is worth doing when you are about to load a
known quantity.

## Removing does not shrink

```java
list.add(...);      // a million times
list.clear();
```

The list is now empty and the array is still a million long. `ArrayList` does not
shrink automatically, on the grounds that a list which grew once will probably grow
again and shrinking would cause repeated copying.

Call `trimToSize()` if you genuinely need the memory back — which requires
declaring the variable as `ArrayList` rather than `List`, since `trimToSize` is
not part of the interface.

## Why this lesson exists

You could use `ArrayList` for years without knowing any of this. Three reasons it
is worth an hour.

**It explains a real cost.** Inserting at the front is expensive — everything
shifts — while appending is cheap, and knowing why lets you predict rather than
measure.

**It is a worked invariant.** `size` and `capacity` are two fields with a
relationship, every operation preserves it, and it is unreachable from outside.
Chapter 16 in about twenty lines of real library code.

**The idea generalizes.** Amortized analysis and growth-by-a-factor turn up
everywhere, and meeting them here — where the arithmetic is a sum of powers of two
you already know — is the easiest place to learn them.

Next: the three shapes.
