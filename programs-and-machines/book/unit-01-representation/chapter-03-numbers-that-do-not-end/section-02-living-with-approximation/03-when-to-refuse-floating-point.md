# When to Refuse Floating Point

Everything so far has taught you to use floating point carefully. This lesson
teaches you to recognize when the right decision is not to use it.

That is a different skill, and in my experience a rarer one. Beginning
programmers reach for the tool they know and then work around its problems.
Experienced ones notice when the problem is telling them to pick a different
tool.

## The test

Ask one question: **does this quantity have a natural smallest unit, and does
correctness at that unit matter?**

If yes, floating point is probably wrong.

Money has a natural smallest unit — the cent, or whatever your currency's
smallest denomination is — and correctness at that unit is enforced by law and by
auditors. Counted things have a natural smallest unit: you cannot have 2.3
customers. Array indices, database keys, and version numbers are exact by
construction.

Measured quantities are the opposite. A temperature, a distance, a probability, a
physical simulation — these have no smallest unit, the input already carries
measurement error, and a relative precision of fifteen significant digits is far
beyond what the data justifies. Floating point is exactly right for these, and
using something slower and exact would be a waste.

## Money, concretely

Watch what floating point does to a shopping cart:

```
0.10 × 3          →  0.30000000000000004
1.10 − 1.00       →  0.10000000000000009
```

Add one hundred one-cent items:

```
100 additions of 0.01  →  1.0000000000000007
```

Which is not one dollar. It is one dollar and a small fraction of a
quadrillionth, and if your code checks whether the customer has paid in full, it
will decide they have not.

You cannot fix this with a bigger type. `float` to `double` buys more digits, not
exactness — one tenth is a repeating binary fraction regardless of how many bits
you throw at it, in the same way that no number of decimal digits will ever write
one third exactly.

## Two things that work

**Integer cents.** Store money as a whole number of the smallest unit. £12.34
becomes the integer 1234. Every value is exact, addition and subtraction are
exact, comparison is exact.

```
10 × 3 cents  →  30 cents
```

The cost is that you must handle the decimal point yourself when displaying,
and you must think carefully about division — splitting 100 cents three ways
leaves a remainder that has to go somewhere, and *deciding where* is a business
question rather than an arithmetic one. Floating point would have silently
invented a third of a cent and hidden the decision from you. Being forced to make
it is an advantage.

A `long` holds about 9.2 quintillion, so in cents that is about 92 quadrillion
currency units. Sufficient for most purposes, and you should still ask.

**Decimal types.** Java provides `BigDecimal`, which stores an arbitrary-precision
integer together with a decimal scale. Because the scale is a power of ten, values
like 0.10 are exact:

```
BigDecimal("0.10") × BigDecimal("3")   →  0.30
BigDecimal("1.10") − BigDecimal("1.00") →  0.10
```

Exactly right, both times, with the trailing zero preserved because `BigDecimal`
tracks scale as part of the value.

There is a trap in it that catches nearly everyone, and it is worth showing:

```
new BigDecimal(0.10)
  →  0.1000000000000000055511151231257827021181583404541015625
```

Constructing a `BigDecimal` from a `double` faithfully preserves the error that
was already in the `double`. The damage happened at the literal, before
`BigDecimal` was involved. **Always construct from a string**, `new
BigDecimal("0.10")`, so that the decimal text is parsed exactly and never passes
through binary floating point at all.

`BigDecimal` is slower than `double` — often by a large factor — and its API is
verbose, since Java has no operator overloading and you write `.add()` and
`.multiply()` instead of `+` and `*`. For a payroll system this is an excellent
trade. For a physics simulation it would be foolish.

## A wider principle

Step back, because this generalizes past money.

Floating point encodes a decision: *approximate, with high relative precision,
across an enormous range.* That decision is exactly right when your quantity is
approximate anyway, and exactly wrong when your quantity is a count of discrete
things.

The mistake is not using floating point. The mistake is using it without noticing
that a decision was made. Every type you choose is an agreement about what can be
represented and what will be silently rounded away — which is where this unit
started, and it is not a coincidence that we have arrived back at it.

When you declare a variable, you are choosing a set of representable values and a
policy for everything outside that set. Chapter 2 showed you the policy for
integers: wrap. This chapter showed you the policy for reals: round to nearest,
ties to even. Neither policy is wrong. Both are silent.

## Closing the chapter

We asked what a finite machine can do with numbers that do not end, and the
answer turned out to be a bargain with legible terms.

Keep a fixed number of significant binary digits and a separate exponent. Get
enormous range and about sixteen significant decimal digits everywhere. Pay for
it by accepting that most decimal fractions — including nearly every price you
will ever handle — are not on the grid, and that what you store is the nearest
value that is.

Every arithmetic result is correctly rounded, which is the best any format could
do. The errors that surprise you entered at the boundary, when your decimal text
became binary, and became visible later when a comparison or an accumulation
magnified them.

Two chapters have now been about numbers. The next is about the other thing
machines mostly hold, which turns out to be harder: text, and the long, unfinished
argument about what a letter is.
