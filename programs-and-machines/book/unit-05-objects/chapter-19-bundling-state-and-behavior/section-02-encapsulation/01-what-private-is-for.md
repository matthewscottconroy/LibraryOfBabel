# What private Is For

Here is a keyword that looks like bureaucracy. It adds a word to every field
declaration and appears to buy nothing: the data is still there, and you have only
made it harder to reach.

The argument of this lesson is that "harder to reach" is the entire point, and that
it buys one specific thing — it makes the set of code that could have broken a rule
small enough to read.

```java
public class Account {
    private long cents;
    ...
}
```

`private` means the field is reachable only from inside this class. Code
elsewhere cannot read it and cannot write it:

```java
account.cents = -500;      // error: cents has private access in Account
```

That is the mechanism Chapter 16 deferred, and the whole of it.

## The argument

It is tempting to read `private` as protection against malice, and that is the
wrong frame. Java's access control is not a security boundary — reflection can
reach around it, as Chapter 27 will show, and anyone editing the source can delete
the keyword.

What it is for is **reasoning**, and the argument is worth following slowly
because it is the argument for most of Unit V.

Chapter 16 said an invariant is worth having when the set of code that could break
it is small enough to check. Ask yourself what that set is for a public field: it
is every line in the program. Every file, including the ones written next year by
somebody you have not met.

Now make it private. The set becomes the code inside `Account.java` — a hundred
lines, which you can sit down and read this afternoon.

And notice what has happened to the question. You have gone from *could anything
anywhere have made this negative?*, which you cannot answer and never will, to *do
these six methods each preserve the invariant?*, which you can answer in ten
minutes by reading them.

Chapter 16's preservation check just became finite. That is the whole trade, and it
is a very good one.

So the sentence to hold is:

> `private` does not stop a determined person. It stops an ordinary reader from
> having to consider the possibility.

## The four levels

Java has four, and you will use two.

| modifier | visible from |
|---|---|
| `private` | this class only |
| *(none)* | this class and others in the same package |
| `protected` | this class, subclasses, and the same package |
| `public` | anywhere |

The unmarked level is called **package-private** and is the default when you write
no modifier at all. It is more useful than its obscurity suggests: classes that
work together in one package can see each other's internals while remaining hidden
from the rest of the program.

`protected` is entangled with inheritance and Chapter 21 returns to it. It is
weaker than people expect — it also grants package access — and it exposes your
internals to every subclass anyone ever writes, which is a larger commitment than
it looks.

## The rule

**Make everything as private as it can be, and widen only when you have a reason.**

Concretely: fields `private`, almost always. Methods that exist to support other
methods in the class, `private`. Only the operations that callers genuinely need,
`public`.

The direction matters. Making something more visible later is easy and harmless.
Making it less visible later means breaking everyone who used it — and once a
thing is public and other people depend on it, you have promised it, whether you
meant to or not. Chapter 16's warning: whatever people can observe, they come to
depend on.

## What private fields cost

Now the honest part, because there is a real cost here and pretending there is not
is exactly how people end up generating getters and setters by the hundred without
ever asking what they are for.

Some things become more work. Printing an object's internals for debugging needs a
`toString`. Comparing two objects needs `equals`. Serializing one needs
cooperation. Every one of those is a method you must write.

That work is the point rather than an accident — each of those methods is a place
to decide what the class means by printing, comparing, or storing. But it is real,
and when a class is a genuine bag of values with no invariant, the ceremony buys
nothing. Chapter 22's `record` exists for exactly that case, and it is the right
answer far more often than people realize.

## An honest exception

There is a case where a public field is fine: a small class with no invariant,
used locally, whose fields are `public final` and whose values are set once.

```java
class Point {
    public final int x;
    public final int y;
    Point(int x, int y) { this.x = x; this.y = y; }
}
```

Nothing can break here. The fields cannot be reassigned, there is no relationship
between them to violate, and a getter would add a line and no information.

Java's own library does this in places. But note the conditions — final, no
invariant, immutable types — and note that Chapter 22's `record` does the same
thing in one line with `equals`, `hashCode`, and `toString` supplied. When you
find yourself reaching for the public-field form, a record is usually what you
want.

Next: deciding what to expose.
