# == vs. equals

Two operators, two questions.

**`a == b`** asks: are these the same object? One thing, or two?

**`a.equals(b)`** asks: do these count as the same for our purposes?

English uses "the same" for both, which is why this is hard.

## The demonstration

```java
class Point {
    final int x, y;
    Point(int x, int y) { this.x = x; this.y = y; }
}

Point a = new Point(1, 2);
Point b = new Point(1, 2);
```

```
a == b        false
a.equals(b)   false
```

Both false. `a == b` is false because they are two objects — `new` ran twice, so
there are two things on the heap.

`a.equals(b)` is *also* false, and that is the surprise. `Point` does not define
`equals`, so it inherits the one from `Object`, and **`Object.equals` compares
references** — it is `==` with a longer name.

So by default, equality *is* identity. If you want anything else you must say so.

## Saying so

```java
@Override
public boolean equals(Object o) {
    if (this == o) return true;
    if (!(o instanceof GoodPoint p)) return false;
    return x == p.x && y == p.y;
}
```

```
c == d        false
c.equals(d)   true
```

Now two points with the same coordinates count as equal, and they remain two
distinct objects.

Reading the method:

**`if (this == o) return true;`** — an object is equal to itself, and this is a
cheap early exit.

**`if (!(o instanceof GoodPoint p)) return false;`** — the parameter is `Object`,
because that is the signature being overridden. Anything not of our type is not
equal, including `null`, since `null instanceof Anything` is false. The `p` is
pattern matching, added in Java 16, which tests and casts in one step.

**`return x == p.x && y == p.y;`** — compare the fields that matter. Which fields
those are is a decision about your domain.

The parameter type is worth dwelling on. Writing `equals(GoodPoint o)` compiles and
does **not** override anything — it is an overload, per Chapter 12, and the
collections will call the inherited `Object` version instead. This is why
`@Override` should always be written: it turns a silent wrong-behaviour bug into a
compile error.

## Which fields count

The interesting question, and the language cannot help.

For a `Point`, both coordinates. For a `Person`, probably a national insurance
number and not their current address. For an `Account`, almost certainly the
account number and *not* the balance — two accounts are not equal because they
happen to hold the same amount today.

The test: **would replacing one with the other be acceptable everywhere in my
program?** If two objects with these fields equal are interchangeable for your
purposes, include those fields. If not, they are not equal.

And notice a class where the answer is "never": an entity with a lifetime and an
identity — an account, a user, an open file — is usually best left with identity
equality, because two of them are two, whatever their fields say.

## When to leave equals alone

Not every class should define it.

**Value objects** — points, money, dates, ranges — should. Their whole purpose is
the values they hold.

**Entities** with identity — accounts, users, sessions — usually should not, or
should compare only the identifier.

**Objects with no meaningful equality** — a thread, a window, a connection — should
not. Two connections are never the same connection.

The default is identity equality, and it is the right answer more often than
people expect.

## The == trap, restated

Chapter 18 gave the string case and it generalizes:

```java
if (name == "admin")            // compares references — unreliable
if (name.equals("admin"))       // compares contents
if ("admin".equals(name))       // also safe when name might be null
```

The third form puts the literal first so that a null `name` returns false instead
of throwing. It reads slightly oddly and is a common defensive habit;
`Objects.equals(a, b)` handles nulls on both sides and is clearer.

Two places where `==` is right: comparing primitives, and deliberately asking
whether two references are the same object — checking for identity, or comparing
against `null`, or comparing enum constants, which Chapter 22 explains are
singletons.

Next: the contract that makes hash collections work, and what breaks without it.
