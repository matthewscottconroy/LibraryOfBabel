# Records as Plain Data

Chapter 20 asked a lot of you. A class holding two coordinates needed private
fields, a constructor, two getters, `equals` obeying five clauses, `hashCode`
consistent with it, and `toString`. Forty lines, all of it mechanical, all of it
derivable from the fact that a point is an `x` and a `y`.

```java
record Point(int x, int y) { }
```

That is the same class. The compiler generates the fields, the constructor, the
accessors, `equals`, `hashCode`, and `toString`, from the components.

Verified:

```
Point[x=3, y=4]
x=3 dist=5.0
equals: true
hash equal: true
set size: 1
```

`toString` is readable without being written. `equals` compares components, so two
separately constructed `Point(3, 4)` objects are equal. `hashCode` agrees, so the
`HashSet` holds one element rather than two.

Chapter 20's entire discipline, and the compiler cannot forget a field.

## What a record is

A record declares that a type is **exactly its components and nothing else**. That
declaration is what generates the code — everything follows from it, including the
things you are giving up.

The fields are `private final`. There are no setters, and there is no way to add
one; a record is immutable, and Chapter 20's whole argument for immutability
arrives for free.

The accessors are named `x()` and `y()`, not `getX()`. This is deliberate — a
record is not a JavaBean and the convention marks the difference.

The class is implicitly `final`, so it cannot be extended, and it cannot extend
anything. Records may implement interfaces.

## What you may add

A record is a class, so it may have methods:

```java
record Point(int x, int y) {
    double dist() { return Math.sqrt(x * x + y * y); }
    static Point origin() { return new Point(0, 0); }
}
```

`dist()` returned 5.0 for `Point(3, 4)`, as it should.

You may also validate, using a **compact constructor**:

```java
record Point(int x, int y) {
    Point {
        if (x < 0 || y < 0) throw new IllegalArgumentException("negative");
    }
}
```

No parameter list, no assignments — the compiler adds them after your body runs.
Verified: `new Point(-1, 0)` produced `caught: negative`.

This matters because it means a record is not restricted to invariant-free data,
which is the usual first assumption. It can enforce a constraint on construction,
and since it is immutable, a constraint checked once is a constraint that holds
forever. That is a stronger guarantee than an ordinary class with setters can
offer.

You may override any generated method. That is rarely wise — the generated
versions are correct, and a hand-written `equals` on a record is a good way to
violate the contract quietly.

## The promises kept

Records were mentioned five times before this chapter, and here is each.

**Chapter 11 — returning two values.**

```java
record MinMax(int min, int max) { }

static MinMax range(int[] a) { ... return new MinMax(lo, hi); }
```

Verified: `MinMax[min=2, max=9]`. Compare the alternatives Chapter 11 listed —
an array whose indices you must remember, two calls doing the same work twice, an
out-parameter. This is one line, it is named, and printing it explains itself.

**Chapter 17 — map keys.** A record has correct `equals` and `hashCode`, and is
immutable, so it cannot be mutated after insertion and lost. It is the safest
possible key.

**Chapters 19 and 20 — the boilerplate objection.** Chapter 19 admitted that
encapsulation's ceremony buys nothing for a class with no invariant. A record is
the answer: full encapsulation, no ceremony, and no public fields.

## When to use one

**Use a record when the type is its data.** A coordinate, a range, a parsed
configuration line, a result carrying two values, a key. Records are especially
good as small local types — the friction of declaring one is now low enough that
you can name a concept instead of passing a `Map` or an `Object[]` around.

**Do not use one when the type has identity or mutable state.** An `Account` has a
balance that changes and an identity independent of its values — two accounts with
the same balance are not the same account. That is a class.

The test is Chapter 20's question: **would two of these with equal contents be
interchangeable?** If yes, a record. If no, the type has identity and a record's
generated `equals` would be wrong.

## Restriction as a feature

Records and enums make the same trade, and it is the chapter's real point.

You give up openness. An enum cannot gain a fourth constant at run time; a record
cannot be extended, cannot be mutated, cannot have hidden state.

You get back guarantees the compiler enforces: exhaustive switches, safe `==`,
correct `equals` and `hashCode`, thread safety, and safety as a map key.

This is the opposite move from the interfaces in Section 22.1, which get their
power by promising less. Both are ways of making the compiler do work for you, and
knowing which you want is most of the skill.

Java 21 combines them. A `sealed` interface names its permitted implementations,
records implement them, and `switch` pattern-matches over the result with
exhaustiveness checked — a closed set of shapes, each carrying its own data. That
is an **algebraic data type**, arriving in Java forty years after ML had it, and
Chapter 24 will show why the idea belongs to how languages are described.

That completes the material of Unit V. Chapter 23 asks the design question: given
classes, interfaces, inheritance, composition, enums and records, how do you
decide what to build?
