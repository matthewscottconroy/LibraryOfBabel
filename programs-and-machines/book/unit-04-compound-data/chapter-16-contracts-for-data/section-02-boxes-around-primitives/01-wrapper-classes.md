# Wrapper Classes

You have a list of names and it works beautifully. So you want a list of numbers,
and you write the line that ought to work:

```java
List<int> numbers = new ArrayList<>();
```

It does not compile.

That is a genuinely strange refusal. Numbers are the most ordinary data there is,
and a collection of numbers is about the most ordinary thing anyone could want. And
the refusal is not a small wart you can step around — it reaches all the way into
the stream performance of Chapter 26, the erasure rules of Chapter 27, and the
forty near-identical interfaces sitting in `java.util.function`.

So it is worth finding out where it comes from. The answer is a decision made in
1995, and it is this: **Java has a split at the bottom of its type system.**

On one side, **primitives** — `int`, `double`, `char`, `boolean`, and the rest.
These are the fixed-width values of Unit I, and they are precisely what they appear
to be. The value lives in the variable. Assignment copies it. There is nothing else
there: no methods to call, no possibility of `null`. Small, fast, and not objects.

On the other side, **objects**, which live on the heap and are reached by
reference, exactly as Chapter 12 described.

The split exists for speed, and the numbers behind it are not subtle. An `int` in a
local variable is four bytes on the stack — no header, no allocation, no
indirection. Make everything an object, as some languages do, and you pay all three
on every number in your program. In 1995, on the hardware of 1995, that was not
acceptable.

The price of the decision is the line that would not compile. Primitives cannot go
anywhere an object is required, and that includes every single collection in the
library.

## The wrappers

The workaround is a class for each primitive, whose instances hold one value:

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

Two of those are spelled out — `Integer` and `Character` — while the rest are the
primitive's name with a capital letter. There is no reason for it. It is an
inconsistency from 1995 that is now far too late to fix, and you will mistype it
about four more times before it sticks.

## They are more useful than they look

A wrapper is not only a box. Each of these classes is *where the operations on that
primitive live*, for the good reason that a primitive has no methods of its own to
put them on.

```java
Integer.MAX_VALUE            // 2147483647
Integer.MIN_VALUE            // -2147483648
Integer.parseInt("42")       // String to int
Integer.toBinaryString(214)  // "11010110"
Integer.toHexString(214)     // "d6"
Integer.compare(a, b)
```

Your instrument from Chapter 5 used two of those, and several of the ranges quoted
back in Unit I came from here.

`Double` is where the Chapter 3 material lives:

```java
Double.MAX_VALUE
Double.isNaN(x)              // the test that == cannot do
Double.toHexString(0.1)      // "0x1.999999999999ap-4"
```

And `Character` holds Chapter 4's:

```java
Character.isDigit(c)
Character.isLetter(c)
Character.toUpperCase(c)
```

So these classes are worth knowing even if you never deliberately store a wrapper
in your life. They are the library's filing cabinet for everything to do with
primitives.

## What wrapping costs you

An `Integer` is an object, and being an object has a bill attached.

**Allocation.** Creating one means asking the heap for space, which is a great deal
more work than putting four bytes into a stack slot.

**Indirection.** Reading the value means following a reference — and the objects
may be scattered anywhere, which is the cache problem from Section 15.2.3.

**Overhead.** Every object carries a header. An `Integer` typically occupies
sixteen bytes in order to hold four bytes of information.

**Garbage.** Every one you make has to be collected eventually.

Now put a number on it. Summing three million values into a `long` against summing
them into a `Long` — how much worse do you expect the second to be? Twice? Three
times?

About 1 ms against about 17 ms. Roughly **twenty-seven times slower**, because the
second version quietly allocated three million objects.

That is not an argument against wrappers, which you cannot avoid and should not try
to. It is an argument for knowing when you are using one — which turns out to be
much harder than it sounds, for reasons that are entirely the next lesson's fault.

## One implementation detail you will need in ten minutes

`Integer.valueOf` caches its instances for small values — by default −128 through
127 — and hands back the very same object every time you ask for one in that range.
Outside it, you get a new object each time.

The reasoning is sound. Small integers are overwhelmingly the common case, so
caching them avoids an enormous amount of pointless allocation. It is a sensible
optimization.

It also makes object identity *observable*, in a way that catches people out
badly — and the next lesson opens by showing you exactly how.

Next: the conversion that happens without your asking.
