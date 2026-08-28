# Who Should Know This?

Almost every design decision you will make, stripped of its particulars, is the
same small question: **which class should be responsible for this?**

It sounds too vague to be useful. It is not, because getting it wrong has a
recognisable shape — facts about one object's insides start showing up in code that
lives somewhere else — and once you can see that shape you can see it everywhere.

Here is the question in its smallest form. A playlist holds items; each item has a
duration. Where does the total go?

```java
// version A — the caller works it out
int total = 0;
for (Item i : playlist.items()) total += i.minutes();
```

```java
// version B — the playlist knows
int total = playlist.totalMinutes();
```

Verified, version B prints `total = 78` for two items of 45 and 33.

Version A is not wrong, and for a one-off it is fine. But notice what it requires:
the caller must be able to get the items, must know that items have durations, and
must know that a total is a sum. Three facts about the playlist's insides, now
living outside it.

Write that loop in four places and the playlist's representation is frozen. Change
`minutes` to `seconds` and you have four sites to find.

## The heuristic

**Behavior belongs with the data it needs.** If a computation reads only one
object's state, it is that object's method.

This is the same argument Chapter 19 made for encapsulation, restated as a design
rule rather than an access modifier. Private fields stop callers from reaching
inside; putting behavior next to the data removes their reason to want to.

The symptom of getting it wrong has a name: a class that is nothing but fields and
getters, with all the logic elsewhere, and callers that reach through it to do
work. It looks harmless — every field is private, every access goes through a
getter — and it has achieved nothing, because the getters exposed the
representation just as thoroughly as public fields would have.

Chapter 19 warned about this and it is worth the repetition: **a getter for every
field is not encapsulation.** It is public fields with extra typing.

## Where the heuristic runs out

Two cases where "put it with the data" gives no answer.

**When the computation needs two objects.** Does a `Route` between two `Stop`s
belong to the route or to the stops? Usually the answer is a third thing — the
computation is its own concept and deserves its own class. Reaching for a third
type feels like overkill and is often exactly right.

**When the operation is not about the object at all.** Rendering an `Account` as
JSON is about the JSON format, not about accounts, and putting `toJson()` on
`Account` couples your domain model to a serialization library forever. That
belongs outside.

The test for the second case: **would this method still make sense if the output
format changed?** If a change in some other system forces a change to this class,
the responsibility is in the wrong place.

## Guarding what you hand out

There is a second half to responsibility that Chapter 20 set up:

```java
List<Item> items() { return List.copyOf(items); }
```

Verified — calling `add` on the returned list throws
`UnsupportedOperationException`.

A getter that returns the internal collection has handed out the ability to modify
it, and the invariant the class was maintaining is no longer the class's to keep.
`List.copyOf` returns an immutable snapshot, so the boundary holds.

This is Chapter 20's defensive copying, and it is the single most commonly skipped
piece of encapsulation in real code. If a class owns a mutable collection, it must
not hand out the original.

Better still, when it fits: do not hand out the collection. Offer the operations
callers actually need — `totalMinutes()`, `contains(item)`, `forEach(...)` — and
the question does not arise.

## Responsibility as a design tool

There is an old technique, from Beck and Cunningham in the late 1980s, that
formalizes exactly this. Take an index card per class. Write the class's name at
the top, its **responsibilities** on the left, and the **collaborators** it needs
on the right.

The cards are small, and that is the point — a class whose responsibilities do not
fit on a card is doing too much. Moving a responsibility means physically moving a
line from one card to another, which makes the alternatives easy to try and cheap
to abandon.

It sounds quaint. It is faster than any tool and it works, and Section 23.2 uses
it on a real problem.

Next: the choice this chapter exists to argue about.
