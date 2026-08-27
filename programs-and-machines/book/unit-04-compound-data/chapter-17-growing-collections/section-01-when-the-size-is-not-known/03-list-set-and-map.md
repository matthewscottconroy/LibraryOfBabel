# List, Set, and Map

The collections library is large and it is organized around three questions.
Knowing which question you have chooses the type.

## The three questions

**List — "what is in position *i*, and in what order?"**

An ordered sequence. Duplicates allowed. Access by position.

```java
List<String> names = new ArrayList<>();
names.add("Ada");
names.add("Grace");
names.add("Ada");           // fine
names.get(1);               // "Grace"
// [Ada, Grace, Ada]
```

**Set — "is this thing present?"**

A collection with no duplicates and, usually, no meaningful order.

```java
Set<String> unique = new HashSet<>(names);
// [Grace, Ada]  — one Ada, and the order is not the insertion order
```

Adding something already present does nothing. There is no `get(i)`, because there
are no positions.

**Map — "what is associated with this key?"**

Pairs of keys and values. Keys are unique; values need not be.

```java
Map<String, Integer> counts = new HashMap<>();
counts.put("Ada", 2);
counts.put("Grace", 1);
counts.get("Ada");          // 2
counts.get("Alan");         // null — absent
```

A map is the answer whenever you find yourself keeping two parallel lists, or
searching a list to find the thing with a matching field.

## Choosing

The question chooses the type, and stating the question is usually enough:

| your question | type |
|---|---|
| the order matters, or I need positions | `List` |
| I need to know whether something is present | `Set` |
| I need to look something up by a key | `Map` |
| I need to count occurrences | `Map<T, Integer>` |
| I need to remove duplicates | `Set` |

A frequent beginner pattern is a `List` plus a loop that searches it. That is
almost always a `Map` or a `Set`, and the difference is not stylistic — searching
a list of *n* elements takes *n* comparisons and a hash lookup takes about one.

## Counting, as an example

The commonest small task in programming, and worth seeing done properly:

```java
Map<String, Integer> counts = new HashMap<>();
for (String name : names) {
    counts.merge(name, 1, Integer::sum);
}
// {Grace=1, Ada=2}
```

`merge` says: if the key is absent, store 1; if present, combine the old value
with 1 using addition. Without it you would write

```java
counts.put(name, counts.getOrDefault(name, 0) + 1);
```

which is also fine and slightly more legible if `merge` is unfamiliar. The
`Integer::sum` is a method reference, which is Chapter 26.

## The implementations

Each shape has several, and the choice is about cost.

**`ArrayList`** — a growable array. Fast access by position, fast appending,
slow insertion or removal in the middle because everything shifts.

**`LinkedList`** — each element holds a reference to the next. Fast insertion and
removal *if you are already there*, and slow access by position, because reaching
element *i* means following *i* references.

That difference is dramatic. Reading 100,000 elements by index took about 1 ms
from an `ArrayList` and about 2,589 ms from a `LinkedList` — over two thousand
times slower — because the array does arithmetic and the linked list walks.

The practical advice, which surprises people: **use `ArrayList` almost always.**
`LinkedList` wins in a narrow band of cases, and its poor locality — Section
15.2.3's cache argument — frequently makes it lose even where theory says it
should win.

**`HashSet` and `HashMap`** — hashing. A key is converted to a number that
indicates where to look, so lookup is about constant time regardless of size. No
useful ordering. These are the defaults.

**`TreeSet` and `TreeMap`** — a balanced tree, keeping keys sorted. Lookup costs
about $\log_{2} n$ rather than constant, and in exchange you can iterate in order
and ask for ranges. Use when order matters.

**`LinkedHashSet` and `LinkedHashMap`** — hashing plus a record of insertion
order. Slightly more memory; iteration returns things in the order added, which is
frequently what a user expects.

## A note on hashing

Chapter 20 covers it properly, and one thing is needed now.

`HashMap` and `HashSet` work by calling `hashCode()` on the key to decide where to
store it. For that to work, **equal objects must have equal hash codes**, and if a
key's contents change after insertion the map can no longer find it.

Two consequences to carry until Chapter 20 explains them:

**Use immutable keys.** `String` and the wrapper types are safe. A mutable object
used as a key, then modified, is effectively lost inside the map.

**A class you write needs `equals` and `hashCode`** before it can be used as a key
or put in a set. Chapter 20 shows how, and Chapter 22's records do it for you.

## Declaring by interface

Repeating the convention because it matters:

```java
List<String> names = new ArrayList<>();      // yes
ArrayList<String> names = new ArrayList<>(); // no
```

Declare the variable as the interface. Then switching to `LinkedList`, or
returning something else entirely from a method, touches one line — Chapter 16's
whole argument, available for free.

The exception is when you need an operation the interface does not have —
`trimToSize` from the last lesson. That is a real reason and it should feel like a
small cost, because it is one.

Next: the angle brackets.
