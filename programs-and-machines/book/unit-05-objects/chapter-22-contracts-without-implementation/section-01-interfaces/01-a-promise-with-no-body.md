# A Promise with No Body

Here is a construct that sounds useless when described: a type with no fields, no
constructor, and no method bodies. It cannot do anything. You cannot even make one.

It is the most useful thing in the chapter, and the reason is that it is *nothing
but a promise* — which turns out to be exactly what a caller needs and nothing more.

```java
interface Drawable {
    void draw();
    String name();
}
```

Two method signatures, no bodies, no fields. Any class may declare that it
satisfies this:

```java
class Dot implements Drawable {
    public void draw() { System.out.println("."); }
    public String name() { return "dot"; }
}
```

and from then on a `Dot` may be used wherever a `Drawable` is expected. The
compiler enforces it: leave out `name()` and the class does not compile.

That is the whole idea. `Drawable` supplies no code — it is a type and a promise
and nothing else.

## Why this is not just a weaker class

The obvious question is why you would want a superclass that cannot help you. Two
answers, and both are structural rather than cosmetic.

**A class may implement many interfaces and extend only one class.**

```java
class Dot implements Drawable, Comparable<Dot>, Serializable { ... }
```

A `Dot` is drawable, comparable and serializable, and those three facts are
independent. There is no hierarchy that could express all three, because a class
has one superclass and these are not kinds of each other.

Java allows this precisely because interfaces carry no state. Multiple
inheritance of *implementation* — C++ permits it — raises the question of what
happens when two superclasses both define a field or a method. Multiple
inheritance of *contract* has no such problem: two interfaces both demanding
`draw()` is not a conflict, it is the same demand twice.

**Unrelated classes can share a type.** `String`, `Integer` and `LocalDate` all
implement `Comparable`. They have no common ancestor but `Object` and nothing
whatever in common in their implementations, and yet `Collections.sort` works on
lists of any of them. It was written against the interface.

That is the pattern: **an interface is a type invented for the benefit of
callers,** not a description of what something is made of.

## The vocabulary

Everything in an interface is `public` — that is the point of it, so you need not
write the word. Methods with no body are implicitly `abstract`. And any field
you declare is implicitly `public static final`, which is why interfaces should
generally not declare fields at all: a constant belongs to a class.

## Default methods

Before Java 8, adding a method to an interface broke every class that implemented
it — including, for `java.util.List`, code the JDK authors had never seen. This is
the commitment problem Chapter 21 raised, and it had frozen the collection
interfaces for over a decade.

Java 8's answer:

```java
interface Drawable {
    void draw();
    String name();

    default String label() { return "shape: " + name(); }
}
```

A `default` method has a body. Implementing classes inherit it and may override
it, and a class that says nothing about `label()` still compiles. Verified:

```
.
shape: dot
```

`Dot` never mentioned `label`, and calling it produced `shape: dot` — the default
body, calling `Dot`'s `name()`. Note what that means: a default method may call
the abstract ones, so it can supply real behavior built out of the contract.

This is how `List` acquired `sort`, `forEach` and `removeIf` in Java 8 without
breaking a single existing implementation, and it is what made the streams of the
standard library possible.

The honest caveat: default methods put implementation into interfaces, which
blurs the line this chapter opened with. Use them for extending an existing
interface, and for small conveniences derived from the abstract methods. Do not
use them to avoid writing an abstract class — if you want shared state, you want
a class.

## Static methods and factories

Interfaces may also hold `static` methods, which is convenient for factories:

```java
static Drawable nothing() {
    return new Drawable() {
        public void draw() { System.out.println("(nothing)"); }
        public String name() { return "nothing"; }
    };
}
```

Verified output: `(nothing)`.

That inner construction is an **anonymous class** — a class with no name,
declared and instantiated in one expression. It is how you supply a small
one-off implementation, and Chapter 26 shows the much shorter form that replaced
it for interfaces with a single method.

## How the call works

Chapter 21 left `invokeinterface` unexplained. Here is why it is separate.

For a class, the method table's layout is fixed by the superclass, so `area` is
slot 0 in every `Shape` subclass and the compiler can emit a slot number. For an
interface, this fails: `Dot` implements `Drawable` and `Comparable2`, and both
cannot claim slot 0. There is no single layout that works, because a class may
implement any combination of interfaces.

So `invokeinterface` must search rather than index — each implementing class keeps
a per-interface table, and the JVM finds the right one. Modern JVMs cache the
result at the call site, so a monomorphic interface call ends up about as fast as
a virtual one. The 28 ms figure from Section 21.2.2 was measured on interface
calls, and it is what a devirtualized interface call costs: about 1.4 nanoseconds.

The mechanism is more work; the measured cost is not enough to influence a design.

Next: what to do when you want a contract *and* some shared code.
