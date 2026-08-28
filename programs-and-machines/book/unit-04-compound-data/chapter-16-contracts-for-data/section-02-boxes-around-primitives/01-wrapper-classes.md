# Wrapper Classes

You have a list of names and it works. You want a list of numbers, so you write
`List<int>` — and it does not compile.

That is a strange thing for a language to refuse. Numbers are the most ordinary
data there is, and a collection of them is the most ordinary thing to want. The
refusal is not arbitrary, and it is not a small wart either: it reaches into
Chapter 26's stream performance, Chapter 27's erasure, and the forty near-identical
interfaces in `java.util.function`.

It comes from a decision made in 1995, and the decision was this. Java has a split
at the bottom of its type system.

**Primitives** — `int`, `double`, `char`, `boolean` and the rest — are the
fixed-width values of Unit I. They live in variables directly, they are copied on
assignment, and they are not objects. They have no methods and cannot be `null`.

**Objects** live on the heap and are reached by reference, as Chapter 12
described.

The split exists for speed. An `int` in a local variable is four bytes on the
stack with no header, no allocation, and no indirection. Making everything an
object, as some languages do, costs all three, and in 1995 that cost was not
acceptable.

The price is that primitives cannot go anywhere an object is required — which
includes every collection in the library.

## The wrappers

So each primitive has a matching class whose instances hold one value:

| primitive | wrapper |
|---|---|
| `byte` | `Byte` |
| `short` | `Short` |
| `int` | `Integer` |
| `long` | `Long` |
| `float` | `Float` |
| `double` | `Double` |
| `char` | `Character` |
| `boolean` | `Boolean` |

Note `Integer` and `Character` are spelled out while the rest match. An
inconsistency from 1995 that will never be fixed.

## What they carry

A wrapper is not only a container. Each class is where the useful operations on
that primitive live, since a primitive has no methods of its own.

```java
Integer.MAX_VALUE            // 2147483647
Integer.MIN_VALUE            // -2147483648
Integer.parseInt("42")       // String to int
Integer.toBinaryString(214)  // "11010110"
Integer.toHexString(214)     // "d6"
Integer.compare(a, b)
```

Chapter 5's instrument used two of these. Unit I quoted several ranges that come
from here.

`Double` carries the Chapter 3 material:

```java
Double.MAX_VALUE
Double.isNaN(x)              // the test that == cannot do
Double.toHexString(0.1)      // "0x1.999999999999ap-4"
```

And `Character` carries Chapter 4's:

```java
Character.isDigit(c)
Character.isLetter(c)
Character.toUpperCase(c)
```

So the wrappers are worth knowing even when you never store one, because they are
where the library keeps everything about primitives.

## The cost of wrapping

An `Integer` is an object. That means:

**Allocation.** Creating one asks the heap for space, which is far more work than
putting four bytes in a stack slot.

**Indirection.** Reading the value means following a reference — Section 15.2.3's
cache problem, since the objects may be scattered.

**Overhead.** An object has a header. An `Integer` typically occupies sixteen
bytes to hold four bytes of information.

**Garbage.** Each one must eventually be collected.

Measured on the machine this book was written on, summing three million values
into a `long` took about 1 ms; into a `Long`, about 17 ms. Roughly **twenty-seven
times slower**, because the second version allocates three million objects.

That is not an argument against wrappers. It is an argument for knowing when you
are using one, which is harder than it sounds because of the next lesson.

## The caching wrinkle

One implementation detail with visible consequences, which the next lesson needs.

`Integer.valueOf` caches instances for small values — by default −128 to 127 — and
returns the same object each time. Outside that range it creates a new one.

The reason is that small integers are overwhelmingly common, so caching them
avoids a great deal of allocation. It is a sensible optimization and it makes
object identity observable in a way that surprises people, which Section 16.2.2
demonstrates.

Next: the conversion that happens without your asking.
