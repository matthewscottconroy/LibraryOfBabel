# Immutability as a Strategy

Everything difficult in this chapter has one cause: objects change.

Aliasing is a problem because a change through one name surprises the holder of
the other. Defensive copying exists because a caller might modify what you gave
them. A mutated hash key is lost because its hash changed after filing.

Make the object unable to change and all three disappear. Not mitigated —
**gone**.

## What immutable means

An object whose observable state cannot change after construction. Chapter 18
established that `String` is one; the wrapper types of Chapter 16 are; so are
`LocalDate` and the collections from `List.of`.

Writing one:

```java
public final class Money {
    private final long cents;
    private final String currency;

    public Money(long cents, String currency) {
        if (currency == null) throw new IllegalArgumentException("currency required");
        this.cents = cents;
        this.currency = currency;
    }

    public long cents()      { return cents; }
    public String currency() { return currency; }

    public Money plus(Money other) {
        if (!currency.equals(other.currency))
            throw new IllegalArgumentException("currency mismatch");
        return new Money(cents + other.cents, currency);
    }
}
```

Note `plus` returns a **new** `Money`. There is no `add` that modifies. That is
what immutability means in practice: operations produce new values rather than
changing existing ones, exactly as `String.toUpperCase` does.

## The rules

There are five, and they are worth having as a checklist, because between them they
account for most of the difficulty in this chapter:

**No mutators.** No method changes observable state.

**All fields `private final`.**

**The class is `final`**, or its constructors are private. Otherwise a subclass can
add mutable state or override a method to lie about the values.

**Mutable components are defensively copied** on the way in *and* on the way out.
A `final` field holding an `ArrayList` is not immutable — the reference cannot be
reassigned and the list can still be modified.

**No reference to a mutable internal escapes**, including through the
constructor.

The fourth is the one people miss. `private final List<Item> items` looks safe and
is not, unless the constructor copies what it is given and the accessor returns a
copy or an unmodifiable view.

## What it buys

**Aliasing is harmless.** Share it as widely as you like; nobody can change it, so
nobody can surprise anybody. This is why `String` literals can be pooled.

**No defensive copying.** There is nothing to protect against, so passing and
returning are free.

**Safe as a hash key**, permanently. The hash cannot change because nothing can.

**Thread safe with no effort at all.** Chapter 31's hardest problems are about
coordinating writes; an object that is never written needs no coordination. This
is the largest benefit and it will not be visible to you until that chapter.

**Easier to reason about.** An immutable object's value is what it was constructed
with, so understanding it means finding one line rather than tracing every path —
Chapter 7's argument for `final`, at the scale of a whole object.

**`equals` and `hashCode` are straightforward**, because there is no window in
which the fields change.

## What it costs

Allocation. Every operation that would have modified now creates.

For `Money.plus` that is one small object and it does not matter. For a large
structure modified in a loop it can matter a great deal — Chapter 18's
`StringBuilder` exists precisely because immutable strings make concatenation
quadratic.

So the pattern that resolves it: **an immutable type with a mutable builder for
construction.** Build it up in a builder where changing is cheap, then produce an
immutable result. `StringBuilder` and `String` are exactly this, and it is worth
recognizing as a design rather than a quirk of strings.

## The recommendation

Bloch's rule, which I think is right:

> Classes should be immutable unless there is a very good reason to make them
> mutable.

Stronger than most people expect. The default should be immutable, and mutability
should be justified — usually by size, by cost, or because the thing genuinely has
a lifetime and an identity that changes over time. An account's balance changes;
that is what an account is. A point's coordinates do not; a moved point is a
different point.

The distinction is roughly **entities against values**. Entities have identity and
change over time. Values are defined entirely by what they hold. Values should be
immutable, and most of the classes you write are values.

## Closing the chapter

Two questions that English confuses. **Identity** — one object or two, answered by
`==`. **Equality** — do these count as the same for our purposes, answered by
`equals`, which by default *is* identity because `Object.equals` compares
references.

A variable of object type holds a reference, assignment copies the reference, and
so two names can denote one object. That is aliasing, it is usually what you want,
and the trouble is the unintended case — which defensive copying at both
boundaries prevents, at the cost of the copying.

Every copying facility in Java is shallow, because deep copying is not well
defined in general. If you need a deep copy you write it, and the fields whose
types are immutable need not be copied at all.

`equals` and `hashCode` must agree, or hash collections silently fail — a set with
two equal elements, a map that cannot find a key it contains. Mutating a key after
insertion loses the entry permanently.

And immutability makes most of this stop being a problem, which is why the
recommendation is to reach for it by default and justify mutability rather than
the reverse.

Next: what happens when one type is a special case of another.
