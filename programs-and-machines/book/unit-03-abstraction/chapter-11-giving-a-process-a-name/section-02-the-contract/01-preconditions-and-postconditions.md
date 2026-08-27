# Preconditions and Postconditions

```java
static int largest(int[] values)
```

What does this promise?

The obvious answer — it returns the largest value — is incomplete in a way that
matters. What if `values` is empty? What if it is `null`? Does it modify the
array? Is the result guaranteed to be an element of the array, or could it be
some computed thing?

A **contract** answers these. It has two halves.

## The two halves

A **precondition** is what the method requires of its caller. What must be true
for the method to work.

A **postcondition** is what the method guarantees in return, provided the
precondition held.

For `largest`:

> **Requires:** `values` is not null and has at least one element.
> **Ensures:** returns the largest element of `values`; `values` is unmodified.

That is the contract, and now the method is usable without reading its body. A
caller knows exactly what to supply and exactly what they get.

The word "contract" is apt. **Each side has obligations, and each side may rely on
the other's.** The caller must supply a non-empty array; in exchange the method
must return the largest element. If the caller supplies an empty array, the
method's guarantee is void — it has promised nothing about that case, so anything
it does is contractually acceptable.

That last point is the one people find uncomfortable, and it is the source of the
technique's power. **A precondition is a way of not handling a case.** By
declaring that `values` must be non-empty, the method is excused from deciding
what "the largest of nothing" means — a question with no good answer.

## Writing them down

In Java, contracts live in Javadoc comments:

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

The `@param`, `@return`, and `@throws` tags are the standard vocabulary. Appendix
C covers the tool that turns these into documentation; the point here is the
discipline rather than the syntax.

You will not write this for every method, and you should not — a three-line
private helper with an obvious name does not need a paragraph. Write contracts
for methods that others will call, methods whose preconditions are not obvious,
and methods where you had to think about an edge case. **If you had to think about
it, write down what you decided.** The alternative is thinking about it again in
six months with less context.

## Strong and weak preconditions

There is a real design choice in how much you demand.

A **strong** precondition demands a lot, and makes the method simpler:

> Requires: the array is sorted ascending and non-empty.

A **weak** precondition demands little, and pushes the work inside:

> Requires: nothing.

Neither is right in general, and the trade is this. A strong precondition makes
the method simpler and faster, and moves the burden to every caller. A weak one
makes the method more robust and moves the burden inside, once.

The considerations that decide it:

**How many callers?** One caller that plainly satisfies the condition argues for
strong. Fifty callers, or a public API used by strangers, argues for weak.

**How expensive to check?** Verifying that an array is sorted costs a full pass —
possibly more than the operation itself. Binary search demands sortedness rather
than checking it for exactly this reason.

**How bad is the failure?** If violating the precondition produces a wrong answer
rather than a crash, lean towards checking. Silent wrong answers are the most
expensive kind of bug, and Unit I's whole argument was about how easily they
happen.

## Invariants, again

Chapter 9 had loop invariants — claims true at every iteration. Contracts are the
same idea at method scale, and the resemblance is not accidental.

A loop invariant is true before and after each iteration. A method's contract
relates what is true before the call to what is true after. In both cases you
are replacing "trace the execution" with "check that a claim holds", and in both
cases that is what makes reasoning about the code possible at all.

Unit IV extends the idea once more to data — a **representation invariant** is a
claim about an object's fields that every method must preserve. Three scales, one
technique: **state what stays true, and check that each step preserves it.**

That is arguably the central intellectual tool in the whole subject, and you now
have it in two of its three forms.

## What the compiler does not check

An uncomfortable truth: Java does not enforce any of this.

The compiler checks types. It does not check that `values` is non-empty, that the
result is really the largest, or that the array was not modified. The contract is
a claim in a comment, and comments can be wrong — and a wrong comment is worse
than none, because it is believed.

Some languages do better. There are systems that verify contracts at compile time,
and Java tools that check them at run time. In practice you rely on three things:
writing contracts carefully, checking preconditions explicitly where it is cheap,
and tests that verify postconditions — Chapter 14's subject.

Which is a good reason to keep contracts short and precise. A contract you cannot
state clearly is usually a method that does not have a clear job.

Next: what happens when the caller does not hold up their end.
