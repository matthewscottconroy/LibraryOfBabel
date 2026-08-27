# The Substitution Principle

The compiler will let you write any hierarchy you like. It checks that the method
signatures line up and stops there. Whether the hierarchy *means* anything — whether
a subclass can actually stand in for its superclass — is not a question the type
system can ask.

Barbara Liskov asked it, in a 1987 keynote and then precisely in a 1994 paper with
Jeannette Wing. The formulation that circulates is:

> Let $\Phi(x)$ be a property provable about objects $x$ of type $T$. Then
> $\Phi(y)$ should be true for objects $y$ of type $S$ where $S$ is a subtype
> of $T$.

Stated in the terms this book has been using: **anything a caller could rely on
from the supertype must still hold for the subtype.** Not the signatures — those
the compiler checks. The behavior.

This is the **Liskov substitution principle**, and it is the test for whether
`extends` is telling the truth.

## A square is not a rectangle

The standard example, because it is short and because everyone's first reaction is
that it must be fine.

A rectangle has a width and a height:

```java
class Rect {
    protected int w, h;
    void setWidth(int w)  { this.w = w; }
    void setHeight(int h) { this.h = h; }
    int area() { return w * h; }
}
```

A square is a rectangle whose sides are equal. That is true in geometry, so:

```java
class Sq extends Rect {
    @Override void setWidth(int x)  { this.w = x; this.h = x; }
    @Override void setHeight(int x) { this.w = x; this.h = x; }
}
```

Each setter keeps the square square. Nothing here looks wrong, and every line
compiles.

Now a method written against `Rect`, by someone who has never heard of `Sq`:

```java
static void resize(Rect r) {
    r.setWidth(4);
    r.setHeight(5);
    System.out.println("expected 20, got " + r.area());
}
```

That method is correct. It follows from `Rect`'s interface: width times height,
4 times 5, 20. Called with a `Rect`, it prints 20.

Called with a `Sq`, the verified output is:

```
--- LSP violation ---
expected 20, got 25
```

`setHeight(5)` also set the width to 5, so the area is 25.

Nobody made a mistake in the ordinary sense. `resize` reasons correctly from
`Rect`. `Sq` maintains its own invariant honestly. The mistake was the word
`extends`, which promised that a `Sq` could stand wherever a `Rect` could, and it
cannot — because `Rect` carries an unwritten guarantee that setting the height
leaves the width alone, and `Sq` cannot keep it.

The lesson is not about squares. It is that **the contract includes things nobody
wrote down**, and inheritance inherits all of it.

## What actually went wrong

It is worth being exact, because "a square is not a rectangle" sounds like a claim
about geometry and is not.

An *immutable* square is a perfectly good subtype of an immutable rectangle. Take
away the setters and nothing breaks: every property provable about an immutable
`Rect` — that `area()` equals `w * h`, that `w` is whatever it was constructed
with — remains true of a `Sq`.

Mutability is what broke it. `setWidth` and `setHeight` are independent in `Rect`
and cannot be in `Sq`, and independence was a property callers could rely on.

Which is one more entry on Chapter 20's list of things immutability makes go away,
and a hint at how often a subtyping problem is really a mutable-state problem.

## The four rules

The principle unpacks into obligations on a subclass, and Java enforces exactly one
of them.

**Preconditions may not be strengthened.** If `Rect.setWidth` accepts any positive
int, `Sq.setWidth` may not demand a value under 100. The caller was entitled to
pass anything the supertype accepted.

**Postconditions may not be weakened.** If `Account.withdraw` guarantees the
balance decreases by exactly the amount, a subclass may not sometimes decrease it
by more. Callers computed on that guarantee.

**Invariants must be preserved.** If `Account` guarantees a non-negative balance,
every subclass must too. An invariant is a promise to everyone who holds a
reference, and the reference does not say which subclass it is.

**History must be respected.** A subclass may not permit state changes the
supertype forbade. This is the square's rule: an immutable supertype cannot have a
mutable subtype, and a supertype with independent setters cannot have a subtype
that couples them.

Java checks none of these. It checks the fifth, mechanical one — that access may
not be narrowed — and leaves the rest to you. This is the same division Chapter 20
described for `equals`: the compiler verifies the shape, and the meaning is your
responsibility.

## The real-world instance

`java.util.Stack extends Vector`. It is in the standard library, it dates from
1.0, and it is wrong.

`Vector` is a list. A `Stack` is meant to be accessed at one end. Because it
inherits from `Vector`, every list operation is available:

```java
Stack<String> s = new Stack<>();
s.push("a"); s.push("b");
s.remove(0);          // legal. removes from the bottom.
s.insertElementAt("x", 1);
```

The stack discipline — the one guarantee the type exists to provide — can be
violated by anyone holding the reference, and the compiler is content.

The failure runs the other way too: a method that takes a `Vector` may be handed a
`Stack` and may reasonably shuffle it. Nothing in the type system objects.

`Stack` is still there because removing it would break code written in 1996. The
current advice is to use `ArrayDeque`, which offers stack operations and *only*
stack operations, because it does not inherit from a list. That is composition
choosing what to expose rather than inheritance exposing everything — Chapter 23's
whole argument, visible in a class you can look up today.

## How to use the principle

In practice it is a question to ask before writing `extends`:

**Is there anything a caller could believe about the supertype that my subclass
would falsify?**

If the honest answer needs the word "except" — "a `Stack` is a `Vector`, except you
should not use the middle"; "a `Sq` is a `Rect`, except the setters interact" — the
answer is no, and `extends` is a lie the compiler will not catch.

Two escape routes exist, and both are better than the lie:

**Make it immutable.** Many violations are about mutation, and removing the setters
removes the violation.

**Use composition.** Hold the other object in a field and expose the operations
that genuinely apply. `Sq` holds a side. `ArrayDeque` holds an array. Nothing is
inherited, so nothing unwanted leaks, and Chapter 23 makes this the default.

## Where the principle came from

Liskov has appeared throughout this book — abstract data types in Chapter 19, the
argument for restricted visibility, and now this. The thread is single: a type is
its behavior, and a type system that checks only signatures has checked the
smaller half.

Her 1987 keynote was titled *Data Abstraction and Hierarchy*, and the argument was
that inheritance had been adopted as an implementation-sharing device when it is
properly a statement about behavior. That distinction is nearly forty years old and
it is still the most common mistake in object-oriented code.

She received the Turing Award in 2008.

That completes Unit V's core. Chapter 22 asks what happens when you want the
substitutability without the inheritance — a common type with no shared
implementation at all — which is what interfaces are for.
