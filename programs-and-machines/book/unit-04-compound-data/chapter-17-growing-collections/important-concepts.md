# Key Concepts

**Why arrays are not enough.** Their size is fixed at creation, and the count is
usually data rather than something known when the program is written.

**Declare by interface.** `List<String> x = new ArrayList<>()`, not
`ArrayList<String> x = ...`. Changing the implementation then touches one line —
Chapter 16's argument, available free.

**The diamond.** `<>` means "the same type parameter as on the left".

**Collections hold objects.** `List<int>` does not compile. Everything about
autoboxing from Chapter 16 applies constantly: `list.remove(1)` removes by index,
unboxing a null throws, and a `Long` accumulator allocates.

**`List.of(...)` is immutable.** Convenient, and it throws on `add`. Wrap in
`new ArrayList<>(...)` for a working list.

**How `ArrayList` grows.** It replaces its array with a larger one. Two fields —
`elements` and `size` — with the invariant `0 <= size <= elements.length`; the gap
between *size* and *capacity* is the mechanism.

**Growth by a constant factor gives amortized constant time.** Doubling means the
total copying over *n* additions is 1 + 2 + 4 + … + n/2, which is less than *n*.
Growing by a constant *amount* gives quadratic behavior instead — a million items
would cost billions of copies.

**Amortized constant.** Any single `add` may be expensive; a sequence of *n* is
cheap per add, because the expensive ones are rare and get rarer.

**Costs of growth.** Up to a third or half the array unused; occasional pauses when
a large list copies; garbage for the collector. Supply an initial capacity when you
know the eventual size.

**Removing does not shrink.** `clear()` leaves the capacity; `trimToSize()` exists
and requires declaring the variable as `ArrayList`.

**Three questions, three shapes.** Position and order → `List`. Presence →
`Set`. Association by key → `Map`. Stating the question chooses the type.

**A list plus a search loop is usually a set or a map.** `contains` on a list is
linear; on a hash set it is constant. Inside a loop that is the difference between
*m* × *n* and *m* + *n*.

**Implementations.** `ArrayList` by default; `LinkedList` is fast where you already
are and slow to reach anywhere — 1 ms versus 2,589 ms for 100,000 indexed reads —
and its poor cache locality often loses it even its theoretical wins. `HashSet`
and `HashMap` by default; `LinkedHash…` for insertion order; `Tree…` for sorted
order at logarithmic cost.

**Constant by position, constant by content — pick one.** Arrays and `ArrayList`
give the first; hashing gives the second.

**Hash keys must be immutable and must implement `equals` and `hashCode`
consistently.** A mutable key modified after insertion is lost inside the map.

**Generics are a promise about contents**, enforced by the compiler: wrong things
cannot go in, and right things come out without a cast. Before Java 5 every read
required a cast checked at run time.

**Erasure.** Type parameters are checked then discarded, so `List<String>` and
`List<Integer>` are one class at run time. Hence no `instanceof List<String>`, no
`new T[10]`, and no overloads differing only by parameter. The cause is
compatibility with pre-generic code.

**Raw types** still compile and lose every check. Do not.

**The iterator.** The enhanced `for` compiles to `hasNext`/`next`. `next` both
returns and advances, which violates command–query separation and is universal
anyway.

**`ConcurrentModificationException`** means the collection changed during
iteration, not that threads were involved. The iterator's position invariant was
broken, and it fails fast rather than skipping elements. Use `removeIf`, the
iterator's own `remove`, or collect-then-remove.

**Map iteration.** `keySet`, `values`, `entrySet`. Use `entrySet` when you need
both, rather than iterating keys and calling `get`.

**Hash iteration order is unspecified** and not stable across versions or runs.
Code depending on it is broken and may pass every test. Use `LinkedHashMap` or
`TreeMap` when order matters.

**Immutable collections.** `List.of`, `Set.of`, `Map.of`. Prefer them for anything
that should not change; a thing that cannot change is a thing you need not track.
