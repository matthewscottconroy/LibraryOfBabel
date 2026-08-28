# Sharing by Extension

Some things really are special cases of other things. A circle is a shape. A
savings account is an account. Saying so in code lets you write a method that
works on shapes and hand it a circle.

Java gives you a keyword for that claim. What makes this chapter more than syntax
is that the keyword is a *promise*, the compiler checks only the smaller half of
it, and the half it does not check is where the trouble lives.

```java
class Shape {
    double area() { return 0; }
}

class Circle extends Shape {
    final double r;
    Circle(double r) { this.r = r; }
    @Override double area() { return Math.PI * r * r; }
}
```

`Circle extends Shape` says: a circle **is a** shape. Everything a `Shape` can do,
a `Circle` can do — and `Circle` adds a radius and supplies its own `area`.

## What a subclass gets

All the superclass's fields and methods, subject to access:

- `public` and `protected` members are available
- package-private members are available only in the same package
- `private` members exist in the object and are **not** reachable by name

That last point causes confusion. A `Circle` object contains any private fields
`Shape` declares — they are part of the object — but `Circle`'s code cannot touch
them. It must go through whatever public or protected methods `Shape` provides.

That is Chapter 19's boundary holding even against subclasses, and it is correct:
a subclass is a client too, and the invariant belongs to the class that declared
the fields.

## Constructors are not inherited

```java
class Circle extends Shape {
    Circle(double r) {
        super();            // implicit if you write nothing
        this.r = r;
    }
}
```

Every constructor calls a superclass constructor first, either explicitly with
`super(...)` or implicitly with `super()`. The superclass must be fully
constructed before the subclass adds to it — its invariant established before
yours depends on it.

If the superclass has no no-argument constructor, the implicit `super()` will not
compile and you must call one explicitly. That error — *constructor Shape in class
Shape cannot be applied to given types* — is common and means exactly this.

`super(...)` and `this(...)` must both be the first statement, so you can use one
or the other, not both.

## What extension is for

The honest list, because it is shorter than people expect.

**Shared behavior.** Several types that genuinely do the same thing in the same
way, where the common code should live once.

**A common type for callers.** So that a method can take a `Shape` and be given a
`Circle`. This is polymorphism and it is usually the real reason.

**Extending something you do not own.** Adding behavior to a library class.

Notice what is not on the list: **code reuse for its own sake**. If two classes
share some code but one is not a kind of the other, inheritance is the wrong tool
— it commits you to a relationship you do not mean, and Chapter 23 will show what
composition does instead.

The test is the phrase: **is a `Circle` a `Shape`?** Yes, in every context where a
shape is expected. If the answer needs qualification — "a `Stack` is a `List`,
except you should not use the middle of it" — the answer is no.

## final classes and methods

You can forbid extension:

```java
public final class Money { ... }        // cannot be subclassed
public final int balance() { ... }      // cannot be overridden
```

This looks unfriendly and is frequently right. A class that was not designed to be
extended usually cannot be extended safely — a subclass can override a method the
class calls internally and break an invariant the class was maintaining, without
either author doing anything wrong.

`String` is final for exactly this reason. If it were not, anyone could write a
subclass whose `length()` lied, and every security check that measured a string
would be defeated.

The guidance: **design for extension explicitly, or forbid it.** A class that is
neither is a class whose subclasses will break in ways nobody anticipated. Chapter
23 returns to this with Bloch's version of the rule.

## The depth problem

One practical warning. Inheritance hierarchies grow, and a class four levels down
is genuinely hard to understand: to know what a method does you may need to read
four files, and to know what a field means you may need all four.

Two levels is usually comfortable. Three should make you ask a question. Beyond
that, the hierarchy has almost certainly become a way of organizing code rather
than a statement about kinds of thing, and Chapter 23's alternative is waiting.

Next: replacing a behavior rather than inheriting it.
