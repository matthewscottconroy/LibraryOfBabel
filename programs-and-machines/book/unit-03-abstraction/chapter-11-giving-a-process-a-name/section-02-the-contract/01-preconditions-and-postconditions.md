# Preconditions and Postconditions

```java
static int largest(int[] values)
```

What does that promise you?

"It returns the largest value" is the answer everyone gives, and it is incomplete
in ways that will eventually cost somebody an afternoon. What if `values` is
empty? What if it is null? Does it hand the array back rearranged? Is the answer
guaranteed to be *one of the elements*, or could it be something computed?

None of that is in the signature, and all of it is part of what the method means.

A **contract** is where the rest of it lives, and it comes in two halves.

## The two halves

A **precondition** is what the method requires of you. What has to be true before
it will work.

A **postcondition** is what the method guarantees back, provided you held up your
end.

For `largest`:

> **Requires:** `values` is not null and has at least one element.
> **Ensures:** returns the largest element of `values`; `values` is unmodified.

Read those two lines and you can now use this method without ever opening it. You
know exactly what to bring and exactly what you will leave with.

"Contract" is a well-chosen word, because it captures the part people miss: **both
sides have obligations, and both sides get to rely on the other's.** You must bring
a non-empty array. In exchange, the method must return the largest element. And
if you bring an empty array, the method's promise is void — it never made one
about that case, so whatever it does next is, strictly speaking, fine.

That is the part that feels wrong the first time, and it is where the whole power
of the technique is hiding. **A precondition is a way of refusing to handle a
case.** By declaring that `values` must be non-empty, the method has excused itself
from answering "what is the largest of nothing?" — a question with no good answer,
which somebody would otherwise have had to invent a bad one for.

## Writing them down

In Java these live in Javadoc:

```java
/**
 * Returns the largest value in the given array.
 *
 * @param values a non-empty array; must not be null
 * @return the largest element of {@code values}
 * @throws IllegalArgumentException if {@code values} is empty
 */
static int largest(int[] values) { ... }
```

`@param`, `@return` and `@throws` are the standard vocabulary. Appendix C covers
the tool that turns these into browsable documentation; what matters here is the
habit rather than the tags.

You will not write this for every method and you should not. A three-line private
helper with an honest name does not need a paragraph about itself. Write contracts
for the methods other people will call, for the ones whose requirements are not
obvious from the outside, and — this is the useful rule — **for any method where
you had to stop and think about an edge case.**

If you had to think about it, write down what you decided. Otherwise you will
think about it again in March, with less context and less patience.

## How much to demand

There is a genuine design decision in how greedy your precondition is.

A **strong** precondition demands a lot and buys a simpler method:

> Requires: the array is sorted ascending and non-empty.

A **weak** one demands little and does the work itself:

> Requires: nothing.

Neither wins in general. What you are trading is who carries the burden: a strong
precondition makes the method simpler and faster and hands the problem to every
caller; a weak one makes the method robust and absorbs the problem once, inside.

Three things decide it in practice.

**How many callers are there?** One caller, in the same file, that plainly meets
the condition — go strong. A public method that strangers will find on the
internet — go weak, because you cannot make them read anything.

**How expensive is checking?** Verifying that an array is sorted takes a full pass,
which can cost more than the search you were about to do. That is exactly why
binary search *demands* sortedness instead of confirming it.

**How bad is the failure?** If breaking the precondition produces a crash, you
will find out. If it produces a plausible wrong answer, you may not — and silent
wrong answers are the most expensive bugs there are.

## You have seen this shape before

Chapter 9 asked you to find a claim that stayed true through every turn of a loop.
This is the same move, one level up.

A loop invariant holds before and after each iteration. A contract relates what is
true before a call to what is true after it. In both cases you have swapped
"trace the execution and see" for "check that a claim survives" — and that swap is
the only reason reasoning about a program of any size is possible at all.

There is a third scale coming in Unit IV, where the claim will be about an
object's fields and every method will be obliged to preserve it. Three scales, one
technique: **say what stays true, then check that each step keeps it true.**

If you take one intellectual tool from this book, it is likely to be that one, and
you now have it in two of its three forms.

## Nobody is checking

Now the uncomfortable part. Java enforces none of this.

The compiler checks types and stops. It does not check that `values` is non-empty,
that your answer really is the largest, or that you left the array alone. The
contract is a claim in a comment — and comments can be wrong, and a wrong comment
is worse than no comment, because somebody will believe it.

Other languages do better; there are systems that verify contracts before the
program runs, and Java tools that check them while it runs. What you have in
practice is three things: write contracts carefully, check preconditions in code
where it is cheap, and write tests that exercise the postconditions — which is
Chapter 14, and closer than it sounds.

All of which is an argument for keeping contracts short and exact. A contract you
cannot state cleanly is usually a method that does not have a clean job.

Next: what happens when the caller does not hold up their end.
