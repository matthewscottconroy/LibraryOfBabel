# Composition over Inheritance

The argument this book has been deferring since Chapter 21.

Two ways for class `A` to use class `B`:

```java
class A extends B { ... }              // inheritance: A is a B
class A { private B b; ... }           // composition: A has a B
```

The claim, from the 1994 design patterns book and repeated by everyone since:
**prefer the second.**

That sounds like taste. It is not. Here is the demonstration.

## A set that counts

Suppose you want a `Set` that also tracks how many elements have been added over
its lifetime. The obvious approach:

```java
class CountingHashSet<E> extends HashSet<E> {
    int added = 0;

    @Override public boolean add(E e) {
        added++;
        return super.add(e);
    }

    @Override public boolean addAll(Collection<? extends E> c) {
        added += c.size();
        return super.addAll(c);
    }
}
```

Both methods are overridden. Both increment correctly. Both delegate to `super`.
Read it as carefully as you like; there is no mistake visible.

Add three elements with `addAll` and the verified output is:

```
inheritance : added=6 size=3
```

Six. Three elements were added and the counter says six.

## Why

`HashSet.addAll` is implemented by calling `add` for each element.

So `addAll(["a","b","c"])` adds 3, calls `super.addAll`, which calls `this.add`
three times — and `this.add` is the override, which adds 1 each time. Three plus
three.

Nobody made a mistake. `HashSet`'s implementation is reasonable. The override is
reasonable. The bug lives in the space between them, and it exists because
**inheritance exposed an implementation detail of the superclass** — that
`addAll` is built on `add` — that no documentation was obliged to mention and that
a future version may change.

That is the point, and it is worth stating sharply. When you extend a class, its
*internal calls become part of your contract*. You are coupled not to what the
superclass promises but to how it is built.

Note the second-order horror: fixing this by not overriding `addAll` works today
and breaks if a future JDK reimplements `addAll` without calling `add`. There is
no version of this class that is correct against `HashSet` as specified.

## The composition version

```java
class CountingSet<E> {
    private final Set<E> inner = new HashSet<>();
    int added = 0;

    boolean add(E e) { added++; return inner.add(e); }

    boolean addAll(Collection<? extends E> c) {
        boolean changed = false;
        for (E e : c) changed |= add(e);
        return changed;
    }

    int size() { return inner.size(); }
}
```

Verified:

```
composition : added=3 size=3
```

The difference is that `inner.add` is a call to another object, and that object's
`addAll` — whatever it does internally — cannot reach back into this class.
Whether `HashSet.addAll` calls `add` is now none of our business, which is what it
should have been all along.

## What composition costs

Honesty requires the other column.

**You must forward the methods you want.** `CountingSet` had to write `size()`.
A full `Set` implementation would need every method, and that is real typing.
Modern IDEs generate it, and Java has no `delegate` keyword, which is a genuine
gap.

**You do not automatically get the type.** `CountingSet` is not a `Set`, so it
cannot be passed where one is expected. The fix is to implement the interface —
`class CountingSet<E> implements Set<E>` — and forward. You get the type from the
*interface* and the behavior from the *field*, which is the pattern in full and
is what the standard library's wrapper classes do.

That combination is the actual recommendation, and it is worth stating as one
sentence: **inherit the type from an interface, and get the implementation from a
field.**

## When inheritance is right

The rule is *prefer*, not *never*. Inheritance is correct when:

**The relationship is genuinely is-a and passes Chapter 21's substitution test.**
Every property callers could rely on holds for the subclass.

**You control both classes.** The fragility above is worst across a library
boundary, where the superclass can change beneath you.

**The superclass was designed for extension and says so.** Documented extension
points, a stated contract about which methods call which. Chapter 22's `Account`
qualifies — the abstract `fee` is an advertised hole.

**There is no alternative.** Some frameworks require you to extend their class.

Bloch's summary is the one to carry: **design and document for inheritance, or
prohibit it.** A class that is neither is a class whose subclasses break for
reasons nobody chose.

## The general form

Chapter 21 said an inheritance hierarchy four levels deep is hard to read.
Composition has no such problem, because a composed object's parts are visible in
its fields, and each part is a separate thing you can understand alone.

There is a deeper reason too. Inheritance is decided when the class is written and
cannot change. Composition is decided at run time — you can swap the inner object,
choose it in a constructor, or change it later. The `CountingSet` could take a
`Set` parameter and count for a `TreeSet` just as happily.

Flexibility deferred to run time is nearly always worth more than structure fixed
at compile time. Chapter 26 pushes the same idea one step further, when the thing
you compose in stops being an object and becomes a function.

Next: the vocabulary for talking about all this.
