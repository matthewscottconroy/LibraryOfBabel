# The Representation Invariant

Take the name collection again:

```java
String[] names = new String[100];
int count = 0;
```

Some things are true of these two variables together, and the program only works
because they are true:

- `count` is between 0 and `names.length`
- elements `names[0]` through `names[count-1]` are the names, and none is `null`
- elements from `names[count]` onwards are unused and their contents are
  meaningless
- no name appears twice

That is the **representation invariant** — the claim that must hold whenever
anyone outside can look.

## What it is for

Without it, `names` and `count` are two variables that happen to sit near each
other. With it, they are a *set of names*, and the invariant is precisely the
difference.

Notice that the invariant is what allows the operations to be written at all.
`size()` can return `count` because the invariant says `count` is the number of
names. `contains` can stop at `count` because the invariant says nothing beyond
that is meaningful. Every operation is written *assuming* the invariant and would
be wrong without it.

That is the same bargain as Chapter 11's precondition. **An invariant is a way of
not handling cases** — `contains` need not consider what to do about the garbage
past `count`, because the invariant promises nobody will care.

## The obligation

Each operation may **assume** the invariant on entry and must **restore** it on
exit.

```java
void add(String name) {
    // may assume: count is the number of names, no duplicates, etc.
    if (count == names.length) grow();
    names[count] = name;
    count++;
    // must ensure: all of the above is true again
}
```

Between the two lines the invariant is *broken* — after `names[count] = name` but
before `count++`, there is a name at position `count` that `count` does not
account for. That is fine, and it is worth being explicit about why: **the
invariant must hold when outsiders can observe, not at every instant.** Inside an
operation, the structure is under repair.

Which is exactly the shape of Chapter 9's loop invariant, where the claim holds at
the top of each iteration and may be false halfway through the body.

## The uniqueness problem

The list above claimed no name appears twice, and `add` as written does not
enforce it. Either the claim comes out of the invariant, or `add` must check:

```java
void add(String name) {
    if (contains(name)) return;      // now the invariant is preserved
    ...
}
```

This is the useful discipline: **write the invariant down, then check each
operation against it.** Where an operation fails to preserve it, you have found
either a bug or a claim you did not really mean.

That check is mechanical and finds real defects. It is Chapter 9's preservation
obligation, applied to data.

## Where invariants break

Almost always in one of three places, and they are worth knowing.

**A missed case.** An operation that handles the normal path and forgets removal,
or the empty case, or the case where the item is already present.

**An escaping reference.** The one from the last lesson:

```java
String[] getNames() { return names; }
```

Now anyone can write `names[50] = "x"` and the invariant is broken by code that
is nowhere near the operations. This is the worst kind, because the damage is done
outside the unit that is supposed to guarantee the property, and no amount of care
inside will help.

The fix is to return a copy, or something unmodifiable:

```java
String[] getNames() { return Arrays.copyOf(names, count); }
```

**Concurrency.** Two threads running `add` at once can interleave between
`names[count] = name` and `count++`, and both write to the same slot. Chapter 31
is about this; note now that it is an invariant violation and that it happens
*during* the window where the structure is legitimately broken.

## Making it enforceable

Everything so far has been discipline. Nothing in the code stops a bad write.

Java's mechanism is `private`, and Chapter 19 covers it:

```java
public class NameSet {
    private String[] names = new String[100];
    private int count = 0;

    public void add(String name) { ... }
    public boolean contains(String name) { ... }
    public int size() { return count; }
}
```

Now the fields are unreachable from outside. The only code that can modify them is
the code inside the class — which is the code you checked against the invariant.

That is the whole point of the construct. **A class is a way of putting a boundary
around an invariant so that the set of code which could break it is small enough
to check.**

If you take one sentence from this chapter into Unit V, take that one. It converts
`private` from ceremony into the thing that makes the reasoning possible.

## Writing it down

In practice, as a comment at the top of the class:

```java
public class NameSet {
    // Representation invariant:
    //   0 <= count <= names.length
    //   names[0..count-1] are non-null and distinct
    //   names[count..] are unused
    private String[] names;
    private int count;
```

Worth writing for anything with more than one field, because the moment there are
two fields there is a relationship between them, and that relationship is the
invariant.

The habit pays off in a specific way. When you come back in six months and need to
add an operation, the invariant tells you what you must preserve — which is
information you would otherwise reconstruct by reading every existing method.

Next: a case study, in the form of Java's most-used small classes.
