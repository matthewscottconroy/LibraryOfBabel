# Exercises

Exercises marked **[carries forward]** introduce something a later chapter
assumes.

## ArrayList

**17.1.** Write a program that reads ten numbers into a `List<Integer>` and
prints their total, without using an array.

**17.2.** Explain each part of `List<String> x = new ArrayList<>();` — the
declared type, the created type, the diamond — and say why the declared type is
the interface.

**17.3.** Predict and explain:
```java
List<Integer> list = new ArrayList<>(List.of(10, 20, 30));
list.remove(1);
System.out.println(list);
```
Then make it remove the *value* 10.

**17.4.** Predict and explain:
```java
List<String> l = List.of("a", "b");
l.add("c");
```

**17.5. [carries forward]** `size()`, `length`, `length()`. Which belongs to
which, and why can the inconsistency not be repaired?

## Growth

**17.6.** An `ArrayList` grows by doubling. Adding *n* items, how many element
copies happen in total? Show the arithmetic.

**17.7.** Suppose it grew by adding 1 each time. How many copies for *n* items?
For *n* = 1,000,000, compare the two.

**17.8. [carries forward]** Define amortized constant time in your own words, and
say why an individual `add` can be expensive without contradicting it.

**17.9.** Write the representation invariant for `ArrayList`'s two fields.

**17.10.** Why does `clear()` not release the memory? Give the argument for that
decision and say when you would override it.

## Choosing

**17.11.** For each, name the collection type and the implementation:
- unique visitor IDs, order irrelevant
- lines of a file, in order
- word frequencies
- students sorted by surname
- the last 10 commands typed, in order
- checking whether a username is taken

**17.12. [carries forward]** Rewrite so it is not quadratic, and say what the cost
becomes:
```java
for (String c : candidates)
    if (names.contains(c)) hits++;
```

**17.13.** Reading 100,000 elements by index took 1 ms from an `ArrayList` and
2,589 ms from a `LinkedList`. Explain the difference in terms of Chapter 15.

**17.14.** Give a situation where `LinkedList` genuinely wins, and say why it is
rarer than it sounds.

**17.15.** Why does `TreeMap` cost more per lookup than `HashMap`? What do you get
for it?

## Generics and iteration

**17.16.** What does `List<String>` promise, and who enforces it? What error do
you get without generics, and when?

**17.17.** Explain type erasure, and give the three things it makes impossible.

**17.18.** Predict, then fix three ways:
```java
for (String n : names)
    if (n.equals("Ada")) names.remove(n);
```

**17.19.** Why does the exception in 17.18 mention concurrency when there is one
thread?

**17.20.** Write a loop over a `Map<String,Integer>` printing each key and value,
using `entrySet`. Say why that is better than iterating keys and calling `get`.

**17.21. [carries forward]** `HashMap` iteration order is unspecified. Give a bug
this can cause that passes every test on your machine.

## Going further

**17.22.** Implement your own growable list with `add`, `get`, and `size`, backed
by an array that doubles. Write the invariant first. Then test it with enough adds
to force several growths.

**17.23.** Section 17.2.2 says `Iterator.next()` violates command–query separation
and is nonetheless universal. Argue that the design is right anyway.

**17.24.** Measure it: time a million `add` calls on an `ArrayList` created with
default capacity, and on one created with the capacity given in advance. Explain
the difference.
