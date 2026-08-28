# List, Set, and Map

The collections library is enormous, and if you go looking at it as a list of
classes you will drown.

Do not do that. It is organized around **three questions**, and the question you
have chooses your type before you look at a single class name. Learn the three
questions and the library shrinks to something you can hold.

## The three questions

**List — "what is in position *i*, and in what order?"**

An ordered sequence. Duplicates are allowed. You get at things by position.

```java
List<String> names = new ArrayList<>();
names.add("Ada");
names.add("Grace");
names.add("Ada");           // fine
names.get(1);               // "Grace"
// [Ada, Grace, Ada]
```

**Set — "is this thing in here?"**

No duplicates, and usually no meaningful order.

```java
Set<String> unique = new HashSet<>(names);
// [Grace, Ada]  — one Ada, and the order is not the insertion order
```

Look at what happened to the second "Ada". Adding something already present does
nothing at all — no error, no complaint, no duplicate. And there is no `get(i)`,
because in a set there are no positions to get.

**Map — "what goes with this key?"**

Pairs. Keys are unique; values need not be.

```java
Map<String, Integer> counts = new HashMap<>();
counts.put("Ada", 2);
counts.put("Grace", 1);
counts.get("Ada");          // 2
counts.get("Alan");         // null — absent
```

Here is the tell for when you want one. **If you find yourself keeping two lists
side by side, or searching a list to find the element with a matching field, you
wanted a map.**

## Choosing

Say your question out loud. That is usually the whole of the decision:

| your question | type |
|---|---|
| the order matters, or I need positions | `List` |
| I need to know whether something is present | `Set` |
| I need to look something up by a key | `Map` |
| I need to count occurrences | `Map<T, Integer>` |
| I need to remove duplicates | `Set` |

The most common beginner pattern by a wide margin is a `List` plus a loop that
searches it. That is nearly always a `Map` or a `Set` in disguise.

And the difference is not a matter of style. Searching a list of *n* elements costs
*n* comparisons. A hash lookup costs about one. On a list of ten thousand names
that is the difference between a program that responds and a program that thinks
about it.

## Counting things

This is the commonest small task in all of programming, and it is worth seeing done
well:

```java
Map<String, Integer> counts = new HashMap<>();
for (String name : names) {
    counts.merge(name, 1, Integer::sum);
}
// {Grace=1, Ada=2}
```

`merge` says: if the key is absent, store 1; if it is present, combine the existing
value with 1 using addition. One line, and the awkward "is it there yet?" case
disappears entirely.

If `merge` is unfamiliar, this does the same job and reads more plainly:

```java
counts.put(name, counts.getOrDefault(name, 0) + 1);
```

Both are fine. The `Integer::sum` is a method reference, which arrives properly in
Chapter 26.

## The implementations, and what they cost

Each of the three shapes has several implementations, and choosing between them is
always a question about cost.

**`ArrayList`** — a growable array. Fast access by position, fast appending, slow
insertion or removal in the middle, because everything after the gap has to shift.

**`LinkedList`** — each element holds a reference to the next. Fast insertion and
removal *if you are already standing there*, and slow access by position, because
reaching element *i* means following *i* references one at a time.

Now, before you read the next paragraph, guess how much slower. Theory says
`LinkedList` should be bad at indexed access. Put a number on "bad".

Reading 100,000 elements by index took about 1 ms from an `ArrayList` and about
2,589 ms from a `LinkedList`. Over two thousand times slower. The array does
arithmetic; the linked list goes for a walk.

Which supports a piece of advice that reliably surprises people: **use `ArrayList`
almost always.** `LinkedList` wins in a genuinely narrow band of cases, and its
poor locality — the cache argument from Section 15.2.3 — often makes it lose even
where the theory says it should win.

**`HashSet` and `HashMap`** — hashing. The key gets converted into a number that
says where to look, so lookup takes about constant time regardless of how much is
in there. No useful ordering. These are your defaults, and you will reach for them
more than everything else combined.

**`TreeSet` and `TreeMap`** — a balanced tree that keeps the keys sorted. Lookup
costs about $\log_{2} n$ instead of constant, and what you buy with that is the
ability to iterate in order and ask for ranges. Worth it exactly when order
matters.

**`LinkedHashSet` and `LinkedHashMap`** — hashing, plus a record of insertion
order. A little more memory, and iteration hands things back in the order they went
in, which is frequently what a user expects to see.

## One thing about hashing you need before Chapter 20

Chapter 20 gives this the treatment it deserves. You need a piece of it now.

`HashMap` and `HashSet` work by calling `hashCode()` on a key to decide where to
put it. For that to work at all, **equal objects must produce equal hash codes** —
and if a key's contents change after it has been filed, the map can no longer find
it.

Two consequences to carry with you until Chapter 20 explains them properly:

**Use immutable keys.** `String` and the wrapper types are safe by construction. A
mutable object used as a key and then modified is effectively lost inside the map —
still in there, and unreachable.

**A class you wrote needs `equals` and `hashCode`** before it can serve as a key or
live in a set. Chapter 20 shows you how, and Chapter 22's records do it for you
without your having to remember.

## Declare by the interface

Worth repeating, because it costs nothing and pays repeatedly:

```java
List<String> names = new ArrayList<>();      // yes
ArrayList<String> names = new ArrayList<>(); // no
```

Declare the variable as the interface. Then the day you switch to a `LinkedList`,
or return something else entirely from a method, you edit one line. That is Chapter
16's entire argument, handed to you for the price of typing a shorter word.

The exception is when you genuinely need an operation the interface does not offer
— `trimToSize` from the last lesson. That is a real reason, and committing to the
concrete type should feel like a small cost, because it is one.

Next: the angle brackets.
