# Bounds and What They Protect

```java
int[] a = {3, 1, 4, 1, 5};
System.out.println(a[5]);
```

Five elements, and we have asked for the sixth. Decide what happens before you read
on — and in particular, decide whether you think anything *should* happen.

```
Exception in thread "main" java.lang.ArrayIndexOutOfBoundsException: Index 5 out of bounds for length 5
```

Java checked. It compared the index against the length before doing any arithmetic,
and it does that on **every single array access** in every program you have ever
run.

That is not free. So the question worth asking is what you are getting for it, and
the answer is considerably larger than the question suggests.

## What happens on a machine that does not check

Go back to the address formula. Notice that it has no opinion about whether your
index is sensible:

```
address = base + 5 × 4 = base + 20
```

For a five-element array living in bytes 0 through 19, that address is the byte
*immediately after* the array. Which is not nothing. It belongs to something —
another variable, another object, or a piece of the machinery that is running your
program.

So without a check, reading `a[5]` hands you whatever happens to be sitting there.
And writing to `a[5]` **modifies something else entirely**.

Sit with that for a second. Not an error. Not a crash. A silent corruption of
unrelated data, which then goes on to misbehave somewhere else in the program,
possibly much later, in code that is completely innocent. It is the distance
between mistake and symptom from Chapter 10, stretched about as far as it will go.

And now the part that turns a bug into a catastrophe: **the something else can be
chosen deliberately.**

Chapter 6 described the stored-program idea, and noted the price it carries —
anything that can write data can write instructions. A **buffer overflow** attack
is that price being collected. Supply input long enough to run off the end of an
array and into the region holding a return address, and when the method returns,
control goes wherever the attacker put it. Their data has become your program's
next instruction.

That one technique accounts for an enormous share of the security vulnerabilities
of the past forty years. It is possible in C. It is impossible in Java. This check
is the entire reason.

## So what does the check cost?

A comparison and a branch, on every access. In a tight loop over a large array that
sounds expensive, and your instinct is probably that it is.

It is much cheaper than you would think, for two separate reasons.

First, modern processors predict that branch correctly nearly every time — the
index is almost always in range, the predictor learns this immediately, and the
cost collapses to approximately nothing.

Second, and more satisfying, the JIT compiler from Chapter 5 can often prove the
check is unnecessary and delete it. Look at this loop and you can make the same
argument the compiler makes:

```java
for (int i = 0; i < a.length; i++) {
    total += a[i];
}
```

The loop condition *already guarantees* `i < a.length`. Checking it again inside is
redundant, the compiler can see that it is redundant, and so it removes the
per-access check completely. This is called **bounds check elimination**, and it
means that ordinary idiomatic loops usually pay nothing at all.

There is a general lesson hiding in there, and it is one of the more useful things
to know about optimizers: **writing the plain, ordinary form of a loop lets the
optimizer help you.** A clever hand-tuned version frequently defeats it, because
the compiler can no longer prove the thing it needed to prove.

## The trade, stated plainly

Java made a choice here: **spend a little speed to eliminate an entire category of
failure.**

C made the opposite choice, and it was not a foolish one. It was made when
processors were far slower and that comparison genuinely mattered, and C exists to
be usable in places where nothing else is.

But the consequences are now a matter of public record. Analyses of the
vulnerability histories of large C and C++ codebases — Microsoft's and Google's
Chromium among them — have repeatedly attributed something in the region of two
thirds of serious security defects to memory safety errors, with out-of-bounds
access the largest single category. Java has essentially none of these.

Which is Chapter 1's pattern again, for the third time in this book. A fixed region
with an enforced boundary buys safety and costs flexibility. What is new here is
that the price has now been measured, in public, at scale — and the industry has
largely concluded it was worth paying. Newer systems languages are designed for
memory safety from the first day.

## Reading the error when it happens to you

You will meet this message often enough that it repays thirty seconds of attention:

```
Index 5 out of bounds for length 5
```

Both numbers are right there. Index 5, length 5, so the valid indices were 0
through 4. The index is exactly one too large — which is the fingerprint of a `<=`
where you meant `<`, or an `a.length` where you meant `a.length - 1`.

Two other patterns worth recognizing on sight. An index of −1 almost always means a
search returned "not found" and somebody used the result without checking it. A
wildly wrong index — 4,000,000 for a length of 10 — almost always means you indexed
with the wrong variable.

## Habits that keep you out of it

**Use `a.length`, never a literal.** `for (int i = 0; i < 5; i++)` works today and
breaks silently the day the array changes size.

**Prefer the enhanced `for`** whenever you do not actually need the index. No
index, no index error, and nothing to get wrong.

**Check before indexing** when the index came from outside your program:

```java
if (i >= 0 && i < a.length) {
    use(a[i]);
}
```

The order of those two conditions is not arbitrary — it is Chapter 8's
short-circuit rule doing real work. Check that the index is in range *before* you
use it.

**And remember that an array of objects starts out full of `null`.** Bounds
checking protects you from bad indices. It has nothing at all to say about elements
you never filled in.

Next: arrays containing arrays.
