# Exercises

Exercises marked **[carries forward]** introduce something a later chapter
assumes.

## Abstract data types

**16.1.** For a collection supporting `add`, `contains`, and `size`, list three
different representations. For each, say what `contains` would cost.

**16.2. [carries forward]** Separate interface from representation for a stack.
What operations does a user need? What could the storage be?

**16.3.** Fifty places in a program use a collection. Explain concretely why
using only its three operations makes changing the storage cheap, and reaching
into the array makes it expensive.

**16.4.** Name four ways a representation can leak. Which is hardest to detect?

**16.5.** `String` changed its internal storage in Java 9 and no source code
needed changing. What property made that possible?

## Representation invariants

**16.6.** Write the representation invariant for:
```java
private String[] names = new String[100];
private int count = 0;
```

**16.7. [carries forward]** Write the invariant for a fixed-capacity circular
queue with fields `Object[] items`, `int head`, `int size`. What must be true of
`head` and `size`?

**16.8.** For the `add` in Section 16.1.2, identify the exact point at which the
invariant is temporarily false. Explain why that is acceptable.

**16.9.** Show that this breaks the invariant, and fix it:
```java
String[] getNames() { return names; }
```

**16.10.** An operation is added to `NameSet` to remove a name by shifting later
elements down. Write it, then check it against your invariant from 16.6. Did you
get it right the first time?

## Wrappers and autoboxing

**16.11.** Predict and explain:
```java
Integer a = 127, b = 127;
Integer c = 128, d = 128;
System.out.println(a == b);
System.out.println(c == d);
System.out.println(c.equals(d));
```

**16.12.** Why does this throw, and what is the exception?
```java
Map<String, Integer> m = new HashMap<>();
int n = m.get("absent");
```

**16.13. [carries forward]** Explain why `list.remove(1)` and
`list.remove(Integer.valueOf(1))` differ, naming the overload-resolution rule
involved.

**16.14.** `Integer a = 1; Long b = 1L; a.equals(b)` is false. Explain why this
is correct behavior, and why it looks wrong.

**16.15.** Rewrite so it does not allocate three million objects, and say how you
would notice the problem in code you did not write:
```java
Long sum = 0L;
for (int i = 0; i < 3_000_000; i++) sum += i;
```

## null

**16.16.** Give three different things `null` might mean as a return value.
Explain why the caller cannot tell them apart.

**16.17.** Rewrite to avoid returning null, and say what each caller no longer
has to do:
```java
List<Order> findOrders(String id) { if (none) return null; ... }
```

**16.18.** When is `Optional` appropriate, and when is it not? Give an example of
each.

**16.19.** Section 16.2.3 says `null` "defeats the type system". Explain what a
type is supposed to promise and what `null` does to that promise.

## Going further

**16.20.** Section 16.2.3 claims autoboxing and `null` fail in the same way — an
abstraction hiding something you must nevertheless know. Argue this, using one
trap from each.

**16.21.** Kotlin distinguishes `String` from `String?` at compile time. Explain
what that buys, and why Java cannot adopt it now.

**16.22.** Write a small class with two fields whose relationship is an invariant.
Write the invariant as a comment, then write three operations and check each
against it. Report anything you got wrong.
