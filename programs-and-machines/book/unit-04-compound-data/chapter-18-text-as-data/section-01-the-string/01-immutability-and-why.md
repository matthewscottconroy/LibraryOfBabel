# Immutability, and Why

```java
String s = "hello";
s.toUpperCase();
System.out.println(s);      // hello
```

Nothing happened, because nothing could. `toUpperCase` does not modify `s`; it
returns a new string, which was discarded.

```java
s = s.toUpperCase();
System.out.println(s);      // HELLO
```

Now `s` refers to the new string. The original still exists until nothing refers to
it, at which point it becomes garbage.

**Every `String` method returns a new string.** `substring`, `replace`, `trim`,
`concat`, `strip` — all of them. None modifies anything.

## Why Java chose this

It looks like a restriction. It is four separate advantages, and they are worth
separating because different ones matter in different situations.

**Sharing is safe.** If a string cannot change, any number of variables may refer
to it with no risk. Chapter 12's aliasing — two names for one object, where a
change through one is visible through the other — cannot cause a problem, because
there are no changes.

That means Java can intern string literals: every occurrence of `"hello"` in your
source refers to **one** object, shared across the whole program. Millions of
duplicate strings collapse into one, which is a substantial memory saving and is
only safe because of immutability.

**Passing is safe.** Chapter 12 warned that a method receiving an object can modify
it, and advised saying in the contract whether it does. For strings the question
does not arise. Hand a string to a method you did not write and you know exactly
what it will be afterwards.

Compare arrays, where `process(myArray)` may return with your array scrambled and
the only protection is a defensive copy.

**Hashing works.** Chapter 17 said a hash key must not change after insertion, or
the map can no longer find it. Strings cannot change, so they are always safe keys
— which is why `Map<String, ...>` is the most common map type in Java by a wide
margin.

It also means the hash code can be computed once and cached inside the string,
which `String` does. Repeated map lookups with the same key do the hashing work
once.

**Threads are safe.** This is the subject of Chapter 31, and immutable objects need
no coordination at all: if nothing writes, no reader can see a half-finished state.

## What it costs

Modification means allocation.

```java
String s = "hello";
s = s + " world";      // a new string; the old one is garbage
```

For occasional changes this is nothing. In a loop it is a disaster, and the next
lesson measures how much of one.

## The literal pool

One consequence worth seeing, because it explains a confusing result:

```java
String a = "hello";
String b = "hello";
String c = new String("hello");

a == b       // true
a == c       // false
a.equals(c)  // true
```

`a` and `b` are the same object, because the compiler puts identical literals in a
shared pool. `c` is a new object with the same contents, so `==` — which compares
references, per Chapter 12 — is false.

**This is why `==` on strings is unreliable in exactly the worst way.** It works
for literals, which is what beginners test with, and fails for strings built at
run time, which is what programs actually handle. A comparison that passes every
test and fails on real input.

The rule is absolute: **compare strings with `equals`, never with `==`.**

## Immutability as a general idea

Worth flagging, because the chapter is a special case of something larger.

An immutable object is one you can stop thinking about. Its value at any point in
the program is the value it was created with, so understanding it requires finding
one line rather than tracing every path. Chapter 7 made that argument for `final`;
this is the same argument at the scale of a whole object.

Java applies this to `String`, to the wrapper types of Chapter 16, and to the
`List.of` collections of Chapter 17. Chapter 20 argues you should apply it to your
own types wherever you can, and the records of Chapter 22 exist to make it
convenient.

The cost is allocation. The benefit is that an entire category of reasoning —
*could this have changed since I last looked?* — disappears.

Next: what to do when the cost matters.
