# The equals/hashCode Contract

You have written `equals`. It works — you tested it, two points with the same
coordinates come back equal, and you moved on to something else.

Here is a set. A set holds no duplicates; that is the one thing a set is for.

```java
Set<GoodPoint> t = new HashSet<>();
t.add(new GoodPoint(1, 2));
t.add(new GoodPoint(1, 2));
System.out.println(t.size());
```

Say the number out loud before you read on.

It is 2.

The two objects are equal. Your `equals` says so, and it is right. The set added
both of them anyway, and it did not hesitate, and it will not tell you.

## Follow the key

To see why, walk in behind a key and watch where it goes.

The map takes the object you handed it and asks for a number — not a comparison,
a single integer, from `hashCode()`. It uses that number to choose a shelf. Then,
and only then, does it walk along that one shelf, comparing with `equals`, until
it finds a match or runs out of shelf.

Now put two equal objects through that. They hand back two different numbers.
They are filed on two different shelves. And the comparison that would have caught
them — the `equals` you wrote so carefully — is never reached, because nothing ever
looks on both shelves at once.

The set is not confused. It looked exactly where the number told it to look, found
nothing, and did as it was asked.

Which means the method you inherited and ignored was not decoration. It was the
filing system.

The inherited `hashCode` derives its number from the object's memory address, so
two distinct objects get two distinct numbers almost every time. That is a perfect
match for the inherited `equals`, which also asks about addresses. The moment you
replace one of them and not the other, you have a filing clerk and a librarian who
disagree about what counts as the same book.

## The five promises

The specification asks four things of `equals`. None will surprise you, which is
the point of reading them once.

**Reflexive** — `a.equals(a)` is true. A thing is itself.

**Symmetric** — if `a.equals(b)` then `b.equals(a)`. Order is not a factor.

**Transitive** — if `a.equals(b)` and `b.equals(c)` then `a.equals(c)`.

**Consistent** — ask twice, get the same answer, as long as nothing changed.

And `a.equals(null)` is false, always.

Then two of `hashCode`, and only one of them can hurt you:

**Equal objects must have equal hash codes.** This is the promise the set above
was relying on when you broke it.

**Unequal objects are allowed to collide.** Two different things may land on the
same shelf. That is fine — the map walks the shelf and sorts it out with `equals`.

Read those two together and you will find they do not mirror each other. Equal
forces the hashes to match; matching hashes force nothing at all. The arrow points
one way only, and every strange-looking rule in this lesson is that asymmetry
having consequences.

## Writing it

```java
@Override
public int hashCode() {
    return Objects.hash(x, y);
}
```

`Objects.hash` takes your fields and stirs them into one number. There is one
discipline and it fits in a sentence: **use the same fields you used in `equals`.**

Which is also the argument for writing the two methods next to each other, touching.
Six months from now you will add a field, and the only thing standing between you
and a very quiet bug is that you cannot edit one of these without your eye falling
on the other.

You may be tempted, at some point, by this:

```java
@Override
public int hashCode() { return 0; }
```

It is legal. It satisfies the contract exactly — equal objects certainly do get
equal hash codes this way. It also puts every object you own on a single shelf, so
every lookup walks the whole thing, and a `HashMap` that was finding your key
instantly is now reading the entire map to find it. Nothing breaks. It just
quietly stops being a hash map.

## A thing I would like you to predict

Read this, and before the last two lines, decide what they print.

```java
List<Integer> key = new ArrayList<>(List.of(1, 2));
Map<List<Integer>, String> m = new HashMap<>();
m.put(key, "value");

System.out.println(m.get(key));      // value

key.add(3);
System.out.println(m.get(key));      // ?
System.out.println(m.size());        // ?
```

`null`, and `1`.

Sit with that pair for a second, because it is stranger than either line alone.
The map contains one entry. You are holding the exact object you filed it under —
not a copy, the same object — and the map tells you there is nothing there.

Nothing is broken. The key was filed on the shelf its hash chose at insertion.
Then you changed the key, so the hash changed, so `get` walks confidently to a
different shelf and finds it empty. The entry is still on the old shelf. It will
be there forever. You cannot read it and you cannot remove it, because every
operation you might use to reach it starts by asking for a number that no longer
points there.

You have lost data inside a live collection, in four lines, with no exception and
no warning.

So: **never mutate an object while it is serving as a key.** The reliable way to
obey that is not vigilance, it is choosing keys that cannot be mutated in the
first place. `String` and the wrapper types are safe by construction, and that is
a large part of why `Map<String, ...>` is the shape you see everywhere.

Hold on to that feeling. In a few pages it becomes an argument.

## Or let the compiler do it

There is a way out of this entire lesson, and it arrives properly in two chapters:

```java
record Point(int x, int y) { }
```

That one line generates `equals`, `hashCode` and `toString` from the components,
consistent with each other by construction, and it cannot forget a field because
it never had to remember one.

When your class is a value — when two of them with the same contents would be
interchangeable, which is exactly the situation that made you want value equality
— a record is the answer, and writing the two methods by hand is a decision you
should be able to justify.

## Before you move on

**Write them together, touching.** Adding a field means editing both.

**Use the same fields in each.** The rest of the contract follows.

**Reach for a record** when the class is a value.

**Let your IDE generate them** when it is not. It will do it correctly, which is
more than most of us manage by hand at four in the afternoon.

**Always write `@Override`.** A wrong parameter type is then a compile error
instead of a method that never runs.

There is a way to make almost all of this stop mattering, and it is the next
lesson.
