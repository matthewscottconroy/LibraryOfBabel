# The equals/hashCode Contract

Define `equals` and forget `hashCode`, and this happens:

```java
Set<GoodPoint> t = new HashSet<>();
t.add(new GoodPoint(1, 2));
t.add(new GoodPoint(1, 2));
System.out.println(t.size());
```

With both methods defined, the size is 1 — the second is recognized as a
duplicate. With only `equals`, it is 2. A set containing two equal elements, which
is not what a set is.

## Why hashing needs the pair

Chapter 17 said a `HashMap` converts a key to a number indicating where to look.
That number is `hashCode()`.

The lookup is: compute the hash, go to that bucket, and compare with `equals`
against what is there.

So if two equal objects produce **different** hash codes, they go to different
buckets, and the comparison that would have found them equal never happens. The
set never notices the duplicate; the map never finds the key.

`Object.hashCode` returns something derived from the object's address, so two
distinct objects almost always get different codes — which is consistent with
`Object.equals`, and inconsistent with any `equals` you write.

## The contract

Four rules for `equals`, from the specification:

**Reflexive** — `a.equals(a)` is true.
**Symmetric** — if `a.equals(b)` then `b.equals(a)`.
**Transitive** — if `a.equals(b)` and `b.equals(c)` then `a.equals(c)`.
**Consistent** — repeated calls give the same answer while nothing changes.

And `a.equals(null)` is always false.

Two for `hashCode`:

**Equal objects must have equal hash codes.** This is the one that breaks
collections.

**Unequal objects need not differ** — but should, as often as is practical, or
everything lands in one bucket and lookup degrades to a linear scan.

Note the asymmetry: equal implies same hash, and same hash does not imply equal.
A collision is legitimate and the map handles it by comparing with `equals`.

## Writing hashCode

```java
@Override
public int hashCode() {
    return Objects.hash(x, y);
}
```

`Objects.hash` takes the fields and combines them. **Use the same fields you used
in `equals`** — that is the whole discipline, and writing the two methods adjacent
makes it hard to forget one when you add a field.

Do not write `return 0;`. It is technically legal — equal objects certainly have
equal codes — and it puts every object in one bucket, turning constant-time lookup
into a linear scan.

## The trap that surprises everyone

```java
List<Integer> key = new ArrayList<>(List.of(1, 2));
Map<List<Integer>, String> m = new HashMap<>();
m.put(key, "value");

System.out.println(m.get(key));      // value

key.add(3);
System.out.println(m.get(key));      // null
System.out.println(m.size());        // 1
```

The map still contains the entry. Looking it up with the very object used as the
key returns null.

The key was filed in the bucket for its hash at insertion time. Mutating it
changed its hash, so the lookup now goes to a different bucket. The entry is
there, in the old bucket, and nothing will ever find it — you cannot retrieve it,
and you cannot remove it either.

**Never mutate an object that is being used as a hash key.** In practice: use
immutable types as keys. `String` and the wrapper types are safe by construction,
which is a large part of why `Map<String, ...>` is so common.

This is also the strongest practical argument in the chapter for the next lesson.

## Records do it for you

Chapter 22's preview, and it is the reason most of this discipline can be avoided:

```java
record Point(int x, int y) { }
```

That generates `equals`, `hashCode`, and `toString` using all the components,
correctly and consistently. For a value object — which is exactly the case where
you wanted value equality — a record is the right answer, and writing the two
methods by hand is a choice you should have a reason for.

## Getting it right

**Write them together.** Adding a field means updating both.

**Use the same fields in each.**

**Prefer a record** when the class is a value object.

**Let your IDE generate them** when it is not. Every IDE will, correctly, and the
generated code is better than most handwritten attempts.

**Always write `@Override`**, so that a wrong signature is a compile error rather
than a silent inherited method.

Next: the strategy that makes most of this unnecessary.
