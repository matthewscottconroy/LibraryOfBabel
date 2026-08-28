# Scope and Lifetime

Two questions that sound like one and are not.

*Where can I write this name?* And: *when does the thing it names actually exist?*

For everything you have written so far the answers coincide, which is why the
difference has not mattered yet. It stops coinciding the moment objects arrive, so
it is worth separating them now while the examples are small.

The first is **scope**, and Java's rule is one sentence: a local variable is in
scope from its declaration to the closing brace of the block containing it.

```java
public static void main(String[] args) {
    int outer = 1;
    {
        int inner = 2;
        System.out.println(outer);   // fine: outer is in scope
        System.out.println(inner);   // fine
    }
    System.out.println(inner);       // error: cannot find symbol
}
```

The block ended, so `inner` is gone. Not "gone to waste" — the name genuinely no
longer exists, and the compiler will tell you so.

The same applies to a `for` loop's counter:

```java
for (int i = 0; i < 3; i++) { }
System.out.println(i);      // error: cannot find symbol
```

`i` was declared in the loop header, so its scope is the loop.

## Why restrict visibility?

The mechanical answer is that the compiler needs to know which cell a name refers
to. The useful answer is about you.

A name in scope is a name you must consider. If every variable in a
thousand-line program were visible everywhere, then reading any ten lines would
require knowing about all of them — because any of them might be involved, and
any might have been changed by code you have not read.

Restricting scope is **restricting how much you have to think about**. A variable
declared inside a loop cannot possibly be affecting anything after the loop,
and knowing that costs you nothing to establish.

This is the same argument the rest of the book keeps making. Chapter 6 said
language constructs exist to extend what a person can hold in their head; scope
is one of the plainest examples.

The practical rule that follows: **declare each variable in the smallest scope
that works.** If it is only needed inside a loop, declare it inside the loop. The
compiler does not care. Your reader does.

## Shadowing

A name in an inner scope may reuse an outer name, hiding it:

```java
int value = 1;
{
    int value = 2;      // error in Java for locals
}
```

Java forbids this for local variables — a genuine kindness, since it is almost
always a mistake. But it *does* permit a local variable to shadow a field, which
is a real source of confusion and which Unit V will return to. For now, note the
rule and prefer distinct names.

## Lifetime

Scope is about visibility in the source. **Lifetime** is about when the cell
exists at run time. Usually they coincide; the distinction becomes important
later.

A local variable's cell lives in the method's **call frame** — Chapter 12's
subject. The frame is created when the method is called and destroyed when it
returns, so a local variable's lifetime is exactly one execution of its method.

That has a consequence worth stating now:

```java
int counter = 0;
counter++;
```

If those two lines are inside a method, then every call gets a *fresh* `counter`
starting at 0. The variable does not remember anything between calls. Beginners
frequently expect otherwise, and write a method they expect to count how many
times it has run.

To remember something across calls you need state whose lifetime is longer than
one call — a field, which is Unit V.

The distinction between scope and lifetime becomes sharpest with objects. An
object created inside a block may outlive the block, if a reference to it escapes.
The *name* went out of scope; the *object* did not, because something else still
points at it. Java's garbage collector frees an object when nothing refers to it
any more, which is a lifetime rule with no relation to scope at all.

For primitives, none of this arises. Box, name, block, done.

## Blocks

A block is a group of statements in braces, and it is the unit scope is measured
in:

```java
if (x > 0) {
    int doubled = x * 2;
    System.out.println(doubled);
}
```

`doubled` exists only inside the `if`. This is usually what you want, and when it
is not, the fix is to declare the variable before the block:

```java
int doubled = 0;
if (x > 0) {
    doubled = x * 2;
}
System.out.println(doubled);
```

Note that the second version requires an initial value, because the compiler
cannot prove the assignment inside the `if` will happen. That is the
definite-assignment check from earlier in the chapter, doing its job.

Next: what it means for the value to change at all.
