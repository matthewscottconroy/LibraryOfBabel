# null and the Billion-Dollar Mistake

Every reference type in Java can hold `null`, meaning *refers to nothing*.

```java
String s = null;
int[] a = null;
Integer n = null;
```

And following one fails:

```java
s.length();      // NullPointerException
```

This is the most common exception in Java by a wide margin, and the man who
introduced the idea has apologized for it.

## The apology

Tony Hoare — Chapter 9's loop invariants, Chapter 15's bounds checking —
introduced the null reference into ALGOL W in 1965. In a 2009 talk he said:

> I call it my billion-dollar mistake. It was the invention of the null reference
> in 1965. At that time, I was designing the first comprehensive type system for
> references in an object oriented language. My goal was to ensure that all use of
> references should be absolutely safe, with checking performed automatically by
> the compiler. But I couldn't resist the temptation to put in a null reference,
> simply because it was so easy to implement. This has led to innumerable errors,
> vulnerabilities, and system crashes, which have probably caused a billion
> dollars of pain and damage in the last forty years.

The estimate is rhetorical. The diagnosis is exact, and it is worth understanding
rather than merely quoting.

## What is wrong with it

**It defeats the type system.** When a method declares that it returns a `String`,
the type is supposed to be a promise, as Chapter 7 argued. But it may return
`null`, which is not a `String` and supports none of a `String`'s operations. So
the promise is really "a String, or nothing", and the type does not say which.

Every reference type is secretly a union of itself and nothing, and the compiler
will not tell you which you have.

**The failure is remote.** A `null` can be stored, passed through several layers,
and put in a collection without complaint. It fails when someone finally uses it,
which may be far from where it entered — the distance between mistake and symptom
that Chapter 10 warned about, maximized — and the stack trace shows you the use, not the origin.

**It has no meaning of its own.** `null` might mean not found, not yet
initialized, not applicable, an error occurred, or genuinely absent. The value is
the same in every case, so the caller cannot distinguish and the documentation
must.

## Living with it

Java cannot remove it. Some practices genuinely help.

**Do not return `null` for a collection.** Return an empty one:

```java
// bad
List<Order> findOrders(...) { if (none) return null; ... }

// good
List<Order> findOrders(...) { if (none) return List.of(); ... }
```

An empty list works in a loop, has a size, and can be passed onward. `null` breaks
all three, and every caller must remember to check.

**Use `Optional` for a value that may be absent.**

```java
Optional<Customer> findCustomer(String id)
```

The type now says the value may be missing, and the compiler makes you address it:

```java
findCustomer(id).map(Customer::name).orElse("unknown");
```

This is the modern answer, added in Java 8. Use it for **return values** that may
legitimately be absent. It is not intended for fields or parameters, and using it
everywhere is its own mistake.

**Fail fast on `null` parameters.** Chapter 11 made the case:

```java
Objects.requireNonNull(name, "name must not be null");
```

One line, and it converts a remote failure into an immediate one that names the
parameter.

**State it in the contract.** If a method may return `null`, say so, and say what
it means.

**Prefer objects that cannot be null.** An empty string, an empty list, a
zero-valued object. This is sometimes called the null object pattern, and it
removes the check rather than remembering it.

## What better languages do

Worth knowing, because it shows the problem is solvable rather than inherent.

Several modern languages make nullability part of the type. Kotlin distinguishes
`String` from `String?`, and will not let you call a method on the second without
handling the absent case — checked at compile time, which is Chapter 5's principle
applied to exactly the right problem. Rust and Swift do something similar with
option types and no null at all.

Java is unable to follow, because thirty years of libraries assume the current
behavior. There are annotations — `@Nullable`, `@NonNull` — that tools can check,
and they help, and they are not enforced by the language.

## What this chapter was about

Pull the two halves together, because they are one argument.

The first half said: a collection of values means whatever we have agreed it
means, the agreement is the **representation invariant**, and the value of putting
a boundary around it is that the code which could break it becomes small enough to
check.

The second half showed what happens when a boundary is drawn badly. Autoboxing
hides a distinction that still matters, so the distinction reappears as five
unrelated-looking traps. `null` is a value that belongs to every type and satisfies
none of their contracts, so every type's promise is weaker than it appears.

Both are failures of the same kind: **an abstraction that hides something you must
nevertheless know.** Chapter 11 warned that a wrong abstraction is worse than
none, and these are the two largest instances in the language.

Chapter 19 gives you the tools to do better in your own code. Before that, Chapter
17 takes up the thing arrays could not do, which is grow.
