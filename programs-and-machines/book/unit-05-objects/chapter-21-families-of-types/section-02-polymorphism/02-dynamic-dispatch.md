# Dynamic Dispatch

The last lesson made a claim that ought to seem impossible on reflection.

Code compiled today calls `area()` on a shape. A class written next year, which
the compiler that produced that code never saw, supplies its own `area()`. And the
old, already-compiled code calls the new method.

Nothing was recompiled. So the decision about *which* method runs cannot have been
made when the call was compiled — and the mechanism that makes this work is worth
knowing, because it explains both the power and the price.

The mechanism has a name — **dynamic dispatch** — and it is worth following in
detail, because the same few instructions explain both why this is powerful and
what it costs.

## The method table

Every object begins with a header, and part of that header is a pointer to its
class. The class holds — among other things — a table of the methods the class
supports, one slot per method, each slot holding the address of the code to run.

Java builds these tables so that **a subclass's table has the same layout as its
superclass's**, with overridden entries replaced and new methods appended:

```
Shape's table            Circle's table
  0: Shape.area            0: Circle.area      <- overridden
  1: Object.toString       1: Object.toString
  2: Object.equals         2: Object.equals
                           3: Circle.radius    <- added
```

So `area` is slot 0 in both. The compiler, seeing `s.area()` where `s` is declared
`Shape`, does not need to know the runtime type — it only needs to know that
`area` lives in slot 0 of whatever table the object points to. It emits a single
instruction:

```
invokevirtual  Shape.area()D
```

and at run time the JVM follows the object's class pointer, reads slot 0, and
jumps there. Two indirections and a jump.

This is why the layout rule matters. Because a subclass never rearranges the
inherited part of the table, the slot number computed at compile time is valid for
every subclass — including ones written years later. That single constraint is
what makes it possible to compile `totalArea` once and have it work on shapes that
did not exist when it was compiled.

## The bytecode instructions

The JVM has four call instructions, and the distinction is exactly the one this
chapter is about:

| instruction | used for | resolved |
|---|---|---|
| `invokestatic` | `static` methods | at compile time |
| `invokespecial` | constructors, `private`, `super.m()` | at compile time |
| `invokevirtual` | ordinary instance methods | table lookup |
| `invokeinterface` | methods called through an interface | search |

The first two are direct calls — the target is fixed, which is precisely why
`private` and `static` methods are safe to call from a constructor and overridable
ones are not.

`invokeinterface` is the awkward one and it is Chapter 22's subject. A class may
implement several interfaces, so an interface method cannot be guaranteed a fixed
slot in every implementing class's table, and the lookup is correspondingly less
direct.

## What it costs

Two indirections is not free, but modern JVMs make it nearly so.

The JIT compiler watches which types actually arrive at each call site. If only
one ever does — the common case, called **monomorphic** — it compiles a direct
call to that method with a cheap type check in front, and often inlines the body
outright. The virtual call disappears.

If two arrive, it can still do this with two branches. Past a handful of types the
site is **megamorphic** and falls back to the table lookup.

Measured on this machine: 20 million interface calls through a single implementing
class took 28 ms — about 1.4 nanoseconds each, including the loop. The same 20
million calls spread across three implementing classes took 42 ms, about 2.1
nanoseconds. Roughly one and a half times slower, and the absolute difference is
0.7 nanoseconds per call.

That is the honest size of the effect. It matters in the innermost loop of a
numerical kernel and it does not matter anywhere else, which puts it firmly in
Chapter 18's territory: measure before you let it change a design.

## Why it is called virtual

The word comes from C++, where methods are *not* dispatched dynamically unless you
mark them `virtual`. In Java the default is reversed — every instance method is
virtual unless it is `private`, `static`, or `final`.

The reversal was deliberate and it is a real design decision. C++ makes you pay
for dynamic dispatch only where you ask for it, so a class with no `virtual`
methods needs no table at all. Java makes extension work by default, on the
argument that a method you cannot override is a method whose author had to
correctly anticipate every future use.

Both positions are defensible. Java's choice is why `final` on a method is
meaningful, and why the JIT's devirtualization work matters so much — the language
made almost every call virtual, so the runtime had to make virtual calls cheap.

## Static methods do not dispatch

A consequence people trip over:

```java
class Shape  { static String name() { return "shape";  } }
class Circle extends Shape { static String name() { return "circle"; } }

Shape s = new Circle();
s.name();        // "shape"
```

`static` methods belong to the class, not the object, so there is nothing to
dispatch on — the declared type decides, exactly as with overloading. A subclass's
same-named static method **hides** rather than overrides.

Calling a static method through a reference is legal and misleading; write
`Shape.name()` and the confusion cannot arise. Most tools warn about it.

Next: the rule that decides whether a hierarchy is honest.
