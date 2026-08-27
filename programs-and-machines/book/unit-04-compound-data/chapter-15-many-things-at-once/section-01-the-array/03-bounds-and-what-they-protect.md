# Bounds and What They Protect

```java
int[] a = {3, 1, 4, 1, 5};
System.out.println(a[5]);
```

```
Exception in thread "main" java.lang.ArrayIndexOutOfBoundsException: Index 5 out of bounds for length 5
```

Java checks every array access. Every single one, at run time, comparing the index
against the length before doing the arithmetic.

That is a cost, and it is worth understanding what it buys.

## What would happen without the check

The address formula does not care whether the index is in range:

```
address = base + 5 × 4 = base + 20
```

For a five-element array occupying bytes 0–19, that address is the byte
*immediately after* the array — which belongs to something else. Another variable,
another object, or part of the machinery of the program.

Without a check, reading it returns whatever is there, and writing it **modifies
something else**. Not an error; a silent corruption of unrelated data, which then
misbehaves somewhere far away. Chapter 10's distance between mistake and symptom,
maximized.

Worse, the something else can be chosen deliberately. Chapter 6 described the
stored-program idea and noted its cost: anything that can write data can write
instructions. A **buffer overflow** attack supplies input long enough to write past
the end of an array and into the region holding a return address, so that when the
method returns, control transfers wherever the attacker chose.

That single technique accounts for an enormous share of the security
vulnerabilities of the past four decades. It is possible in C, and it is
impossible in Java, and this check is why.

## The cost

A comparison and a branch on every access. In a tight loop over a large array,
that is real.

It is also less than you would expect, for two reasons. Modern processors predict
the branch correctly nearly every time, so the cost is close to nothing in
practice. And the JIT compiler of Chapter 5 can often prove the check unnecessary
— in

```java
for (int i = 0; i < a.length; i++) {
    total += a[i];
}
```

the loop condition already guarantees `i < a.length`, so the compiler removes the
per-access check entirely. This is called **bounds check elimination**, and it
means idiomatic loops usually pay nothing.

Which is a nice illustration of a general point: writing the ordinary, obvious
form of a loop lets the optimizer help you, while a clever hand-optimized version
frequently defeats it.

## The trade, stated plainly

Java made a choice: **spend a little speed to eliminate a category of failure
entirely.**

C made the other one, and it is not a foolish choice — it was made when processors
were far slower and the cost mattered more, and C's purpose is to be usable where
nothing else is.

But the consequences are on record. Analyses of the vulnerability histories of
large C and C++ codebases — including Microsoft's and Google's Chromium — have
repeatedly attributed something in the region of two thirds of serious security
defects to memory safety errors, of which out-of-bounds access is the largest
category. Java has essentially none of these.

This is Chapter 1's pattern once more. A fixed-size region with an enforced
boundary buys safety and costs flexibility. The novelty here is that the price is
now known and the industry has largely decided it was worth paying — newer
systems languages are designed for memory safety from the start.

## Reading the error

Chapter 10 covered this and it is worth repeating because you will see it often:

```
Index 5 out of bounds for length 5
```

Both numbers are given. Index 5, length 5, so valid indices are 0 to 4 — the
index is exactly one too large, which is the signature of `<=` where `<` was
meant, or of `a.length` where `a.length - 1` was meant.

An index of −1 usually means a search returned "not found" and the result was used
without checking. A wildly wrong index usually means the wrong variable.

## Defensive habits

**Use `a.length`, never a literal.** Writing `for (int i = 0; i < 5; i++)` breaks
silently when the array changes size.

**Prefer the enhanced `for`** when you do not need the index — no index, no index
error.

**Check before indexing** when the index comes from outside:

```java
if (i >= 0 && i < a.length) {
    use(a[i]);
}
```

Note the order, which is Chapter 8's short-circuit rule: check the range before
using it.

**Remember that arrays of objects start full of `null`.** Bounds checking protects
you from indices, not from unfilled elements.

Next: arrays containing arrays.
