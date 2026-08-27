# Separating Use from Implementation

Suppose you need to keep a collection of names and answer three questions: add a
name, check whether a name is present, and report how many there are.

Here is one way:

```java
String[] names = new String[100];
int count = 0;
```

And here is another:

```java
String[] names = new String[100];
int count = 0;
// but kept in sorted order at all times
```

And a third, using Chapter 17's `ArrayList`, and a fourth using a hash table, and
a fifth using a tree.

All five support the three operations. They differ enormously in how they store
things and in what each operation costs.

## The two questions

That situation has a shape worth naming.

**What can I do with this?** Add, contains, size. Three operations, each with a
contract.

**How is it stored?** An array, sorted or not; a linked structure; a hash table.

These are independent. Every implementation answers the first question the same
way and the second differently, which means **a user who only knows the answer to
the first question can be given any implementation**.

That separation is the **abstract data type**: a set of operations with contracts,
considered apart from any particular storage.

## Why it matters

Concretely: suppose fifty places in your program use the collection, all through
the three operations. You discover the linear search in `contains` is too slow and
switch to a hash table.

If the fifty places only ever called `add`, `contains`, and `size`, you change one
file.

If they reached into the array — `names[i]`, `names.length`, a loop over the
slots — you change fifty places, and each is a chance to introduce a bug.

**The abstraction is what makes the change affordable.** Not making the code
prettier; making a future change cost one file instead of fifty.

Parnas's paper, from Chapter 14's profiles, is the argument in full: draw module
boundaries around *decisions likely to change*, so that a change is contained. The
decision here is "how the names are stored", and it is exactly the sort of thing
that changes.

## The interface and the representation

Two words for the two halves.

The **interface** is the set of operations and their contracts. What a user needs
to know.

The **representation** is the actual storage — the array, the count, the sortedness.
What a user must *not* need to know.

The rule that follows: **if a user can observe the representation, you cannot
change it.** Not "should not" — cannot, without breaking their code. Every detail
that leaks becomes part of the contract whether you intended it or not.

This is why Chapter 19's `private` matters, and why the answer to "why not just
make the field public and save writing a method" is not a matter of style. A
public field is a promise that the field exists, is called that, has that type,
and can be assigned anything of that type. You have promised all of it by
accident.

## Leaking

Some ways a representation escapes, in increasing order of subtlety.

**Public fields.** The most direct.

**Returning the internal array.**

```java
String[] getNames() { return names; }      // hands out the actual array
```

The caller can now modify your storage. Chapter 12 established that returning a
reference does not copy — so this method gives away the representation while
looking like it reports on it.

**Documented behavior that is really an accident.** If `contains` happens to be
faster for names added recently, and a user notices and depends on it, you have
lost the freedom to change the ordering. Behavior people rely on becomes contract
regardless of what the documentation says.

**Exposing the size limit.** If the array is 100 long and adding the 101st fails
in a particular way, that is now the observable behavior.

The general principle, and it is worth remembering when you design anything:
**whatever users can observe, they will come to depend on.**

## An example worth having

Consider `String`, which Chapter 18 covers. You use it constantly and you almost
certainly do not know how it stores characters.

You do not need to. The interface — `length()`, `charAt()`, `substring()`,
`indexOf()` — is all you use.

And the representation *has changed*. Java stored strings as an array of `char`
(two bytes each) for twenty years. In Java 9 it changed to a `byte` array with an
encoding flag, so that strings of Latin-1 characters take half the memory. This
change affected essentially every Java program ever written, produced a
substantial memory reduction across the ecosystem, and required no source changes
from anybody.

That is what the separation buys, at a scale worth taking seriously.

Next: the claim that makes a representation into a structure.
