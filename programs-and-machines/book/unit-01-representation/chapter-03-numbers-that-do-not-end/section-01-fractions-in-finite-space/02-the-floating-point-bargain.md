# The Floating-Point Bargain

We have 64 bits and we want to cover a range from the size of an atom to the size
of a galaxy. How would you spend them?

## The obvious plan, and why it is not enough

The straightforward approach is called **fixed point**: pick a spot for the
binary point and stick to it. Say 32 bits for the whole part and 32 for the
fraction. Every value then has 32 bits of fractional precision, and arithmetic is
just integer arithmetic with a bookkeeping convention about where the point sits.

Fixed point is fast, exact within its grid, and still used — in audio processing,
in embedded controllers, in some financial systems. But look at what it commits
you to. The largest value is about 4 billion, and the smallest step is about
2 × $10^{-10}$. Every number gets the same absolute precision.

That is the flaw. If you are measuring interstellar distances you have no use for
ten decimal places of a meter, and 4 billion is nowhere near enough range. If you
are measuring the mass of an electron, a step size of 2 × $10^{-10}$ is uselessly
coarse. The precision you need is almost always *relative* to the size of the
thing you are measuring — you want a fixed number of significant figures, not a
fixed number of decimal places.

## Scientific notation, made binary

Which is exactly what scientists worked out long before computers. Write a number
as a fixed number of significant digits, times a power of the base:

```
6.022 × 10²³        Avogadro's number
9.109 × 10⁻³¹       electron mass, kg
```

Four significant figures in both cases, spanning fifty-four orders of magnitude.
The exponent supplies the range; the digits supply the precision; the two are
budgeted separately.

Floating point is this, in base two. A number is stored as

$$ \text{value} = \pm\, m \times 2^{e} $$

where *m* is the **mantissa** (or significand) and *e* is the **exponent**. The
point "floats" — its position is carried in the exponent rather than fixed by the
format. Hence the name.

## Normalization and the free bit

In decimal scientific notation we insist on exactly one non-zero digit before the
point: 6.022 × $10^{23}$, not 60.22 × $10^{22}$ or 0.6022 × $10^{24}$. This is called
**normalizing**, and it makes the representation unique.

Do the same in binary and something pleasant happens. The leading digit must be
non-zero — and in base two the only non-zero digit is 1. So the leading digit is
*always* 1, in every normalized number, without exception.

A bit that is always 1 carries no information. So we do not store it. The format
stores only the digits after the point and reconstructs the leading 1 when
reading. This is called the **hidden bit** or implicit leading bit, and it is
worth one extra bit of precision for free — which is the kind of trick that gets
a design adopted.

## The budget

IEEE 754, the standard everyone follows, divides a 64-bit `double` like this:

| Field | Bits | Purpose |
|---|---:|---|
| sign | 1 | 0 positive, 1 negative |
| exponent | 11 | the power of two, offset by a bias of 1023 |
| fraction | 52 | the digits after the implied leading 1 |

The exponent is stored **biased**: rather than using a signed encoding, the
stored value is the true exponent plus 1023. So a stored 1023 means an exponent
of 0, a stored 1019 means −4, and so on. The reason is comparison — biasing makes
the bit pattern of a positive float sort in the same order as the number it
represents, so hardware can compare floats with integer comparison circuitry.

The 32-bit `float` uses the same scheme with 8 exponent bits and 23 fraction
bits, and a bias of 127. That is the format we decoded in Chapter 1 without
explaining it; you can go back now and see that `0 10000010 1001…` was sign
positive, stored exponent 130 meaning 3, mantissa 1.5625, giving 12.5.

## Decoding one-tenth

Now we can see exactly what happens to `0.1`. Its stored bits are:

```
0 01111111011 1001100110011001100110011001100110011001100110011010
│ └─ 1019 ──┘ └────────────── 52 fraction bits ──────────────────┘
sign
```

The stored exponent 1019 means a true exponent of 1019 − 1023 = −4. The fraction
bits, with the hidden leading 1 restored, give a mantissa of exactly 1.6. So the
value is 1.6 × $2^{-4}$ = 0.1.

Except that it is not, because the mantissa is not exactly 1.6 either — those 52
bits are the truncation of the repeating pattern we computed in the last lesson,
rounded at the end. Look at the fraction bits and you can see the `1001 1001
1001…` repeating, and then the final group ends `1010` rather than `1001`. That
is the rounding: the discarded tail was more than half, so the last bit was
rounded up.

The number actually stored is

```
0.1000000000000000055511151231257827021181583404541015625
```

That is not an approximation of what is stored — it is what is stored, exactly.
A binary fraction with 52 fraction bits is a precise rational number, and this is
it. When you write `0.1` in a program, this is the number you get.

## What the bargain bought

The range is enormous. A `double` reaches to about 1.798 × $10^{308}$ and down to
about 2.225 × $10^{-308}$ for normalized values — vastly more than the roughly
4 × $10^{9}$ of our fixed-point scheme.

The precision is about 15 to 17 significant decimal digits, everywhere. Not a
fixed step size: a fixed *proportion*. Near 1.0 the gap between adjacent
representable doubles is about 2.22 × $10^{-16}$. Near $10^{16}$ the gap is 2.0 — at that
magnitude, consecutive representable values are two apart, and there is nothing
in between.

That last fact has a consequence worth stating now. Since integers above $2^{53}$ are
spaced more than 1 apart in the `double` grid, not all of them are representable.
$2^{53}$ is 9,007,199,254,740,992, and

```
(double) 9007199254740992  ==  (double) 9007199254740993   →  true
```

Two different integers, one `double`. If you have ever seen large identifiers get
mangled by a system that stored them as floating point — this is why.

## The corners of the format

Three exponent patterns are reserved, and they buy useful behavior.

**Zero** has an all-zeros exponent and fraction. Note that the sign bit is
independent, so there are two zeros: `+0.0` and `−0.0`. They compare as equal but
are distinguishable, which matters when a computation approaches zero from a
direction you care about.

**Infinity** has an all-ones exponent and zero fraction. Dividing a positive
number by zero yields `Infinity` rather than raising an error, which lets a long
computation continue and report a sensible result at the end.

**NaN** — Not a Number — has an all-ones exponent and non-zero fraction. It is
what `0.0/0.0` produces. NaN has the striking property that it is not equal to
anything, including itself: `Double.NaN == Double.NaN` is `false`. That is
deliberate, and it means a NaN cannot hide in your data pretending to be a
value — but it also means the obvious way to test for one does not work.

**Subnormal** numbers use an all-zeros exponent with a non-zero fraction to
represent values below the smallest normalized number, trading precision for the
ability to approach zero gradually rather than falling off a cliff.

Next: what this costs you in practice, starting with the reason `0.1` is not 0.1.
