# Trusting the Recursion

The single most useful thing in this chapter, and it is rarely taught directly.

**Do not trace recursive calls. Assume they work.**

## The technique

To write or read a recursive method, hold exactly two things:

1. **The base case is right.**
2. **The recursive call returns the correct answer for its smaller input** — not
   because you traced it, but because you assume it.

Then check one thing: **given a correct answer for the smaller problem, does my
code build a correct answer for this one?**

If yes, the method is correct. You are finished. You never think about the third
level down, or the fourth, and you certainly never trace to the bottom.

## Applying it

```java
static int sum(int[] a, int from) {
    if (from == a.length) return 0;              // base
    return a[from] + sum(a, from + 1);           // recursive
}
```

Reading this the tracing way — `sum(a,0)` calls `sum(a,1)` which calls `sum(a,2)`
— gets unmanageable immediately.

Reading it the trusting way:

*The base case says the sum of nothing is 0. Correct.*

*For the recursive case, assume `sum(a, from+1)` correctly returns the sum of
elements from `from+1` onwards. Then the sum from `from` onwards is that, plus the
element at `from`. Which is what the code computes. Correct.*

Two sentences. No tracing. And notice that the argument does not get longer for
larger arrays — it is the same two sentences for ten elements or ten million.

Another:

```java
static String reverse(String s) {
    if (s.isEmpty()) return s;
    return reverse(s.substring(1)) + s.charAt(0);
}
```

*Base: the reverse of the empty string is the empty string. Correct.*

*Recursive: assume `reverse` correctly reverses everything after the first
character. Then putting the first character at the end of that gives the reverse
of the whole string. Correct.*

```java
reverse("stressed")     // "desserts"
```

## Why this is legitimate

It feels like cheating. You are assuming the thing you are trying to establish.

It is not cheating, and Section 13.1.3 gives the formal reason: this is
mathematical induction, and assuming the smaller case is exactly what the
inductive step is entitled to do. The assumption is discharged by the base case
plus the requirement that inputs shrink.

The informal version, which is worth holding alongside: **if the method is correct
for the smallest input, and correct-for-*n*−1 implies correct-for-*n*, then it is
correct for everything.** You do not need to check each level because the second
condition covers all of them at once.

## The leap

Most people find this genuinely uncomfortable at first, and I want to name the
discomfort rather than pretend it away.

The discomfort is that the method is not finished when you write the recursive
call. It does not yet work. You are asking it to do something it cannot currently
do, and trusting that it will once you finish writing it.

That is a real leap and it takes a while to become natural. The thing that helped
me, and that I offer for what it is worth: **stop thinking of it as the same
method.** Imagine that `reverse(s.substring(1))` is a call to a *different* method,
written by a competent colleague, which correctly reverses strings. Your job is
only to handle the first character. Under that framing there is nothing strange
happening — you are calling a library.

Then notice that the colleague's method is yours, and that the framing was
accurate all along.

## What this buys

**Short arguments.** Correctness in two sentences regardless of input size.

**Short code.** Recursive solutions to recursive problems are frequently a third
the length of the iterative equivalent, because the bookkeeping the loop version
does explicitly is done by the stack.

**A design procedure.** Faced with a problem, ask: *what is the smallest case, and
if I had the answer for a slightly smaller version, how would I build this one?*
Those two questions produce a recursive method directly, and they are answerable
when "write a loop that does this" is not.

## Where the trust is misplaced

Two failures, and both are failures of the *conditions* rather than of the
technique.

**The input does not shrink.** Trust is only warranted because the argument gets
smaller and the base case is reached. `reverse(s)` calling `reverse(s)` satisfies
nothing.

**The base case is wrong.** Everything is built on it, so an error there is an
error everywhere. If `factorial(0)` returned 0, every factorial would be 0 — and
the recursive step would still be perfectly correct.

Check those two, and the trust is earned. Next: why.
