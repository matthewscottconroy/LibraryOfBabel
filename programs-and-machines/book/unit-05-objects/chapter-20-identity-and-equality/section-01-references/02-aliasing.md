# Aliasing

Two names for one object.

```java
int[] p = {1, 2, 3};
int[] q = p;
q[0] = 99;
System.out.println(p[0]);      // 99
```

`p` was not touched, and `p[0]` changed. There is one array with two names, and
modifying it through either is visible through both.

That is **aliasing**, and it is the single most common source of confusion for
people moving from primitives to objects — because the effect appears in code that
has no visible connection to the code that caused it.

## Why it looks impossible

Consider a bug report: *the discount list is being modified and nothing in the
discount module touches it.*

```java
List<Item> items = order.items();     // returns the internal list
items.clear();                        // "my" list
```

The caller believes they were handed a list to work with. They were handed *the*
list. Clearing it empties the order.

Nothing in the discount module mentions `Order`. The stack trace at the point of
damage names only the discount code. The cause is a method in `Order` that
returned a reference instead of a copy — Chapter 19's warning about not returning
internals, and this is the failure it prevents.

## Where aliases come from

Four places, and being able to name them is most of the defence.

**Assignment.** `b = a`.

**Argument passing.** Calling `record(account)` gives the method an alias. Chapter
12's demonstrations were exactly this.

**Returning an internal.** `return items;` — the case above.

**Storing in a collection.** `list.add(account)` stores a reference. Modifying the
account afterwards changes what the list contains, because the list does not hold
a copy.

## When aliasing is what you want

Most of the time, and it is worth saying so before the warnings.

An account passed to three different subsystems should be the *same* account —
otherwise a deposit recorded by one is invisible to the others. Object identity is
how you model a thing that exists once and is referred to from several places,
which is most things in most programs.

The trouble is not aliasing. It is **unintended** aliasing: a caller who believes
they have their own copy and does not.

## Defending against it

**Return copies of mutable internals.** Chapter 19's rule, and this is why.

**Copy on the way in as well.** If a constructor stores a list the caller supplied,
the caller still has a reference to it:

```java
public Order(List<Item> items) {
    this.items = items;                    // caller keeps an alias
}

public Order(List<Item> items) {
    this.items = List.copyOf(items);       // defensive copy
}
```

The second is a **defensive copy**, and the general rule is to make one at both
boundaries: when a mutable object comes in, and when one goes out.

**Or make it immutable**, which removes the problem rather than defending against
it. If nothing can change, an alias is harmless — which is Section 20.2.3's
argument and the reason `String` is designed the way it is.

## The cost of defending

Defensive copying is not free. Copying a large list on every accessor call is
real work, and a class that does it constantly will show up in a profile.

The judgment: copy when the object is mutable, when it is genuinely part of your
representation, and when a caller could plausibly hold on to it. Do not copy
immutable objects — there is nothing to protect — and do not copy things you have
already established the caller owns.

When the copying starts to hurt, that is usually a sign that the type should have
been immutable in the first place.

Next: what "copy" even means when objects contain objects.
