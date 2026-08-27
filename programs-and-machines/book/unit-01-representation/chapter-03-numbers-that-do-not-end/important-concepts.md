# Key Concepts

**Positional fractions.** The digits right of the point continue the same rule as
those left of it: each position is worth one base-th of the position to its left.
In base two they are worth 1/2, 1/4, 1/8, and so on.

**The termination rule.** A reduced fraction terminates in base *b* exactly when
every prime factor of its denominator divides *b*. Base ten is 2 × 5, so 1/3 does
not terminate. Base two has only the factor 2, so 1/10 does not terminate.

**One tenth is a repeating binary fraction.** `0.0001100110011…` with `0011`
repeating forever, in exactly the way 1/3 repeats in decimal. This single fact
causes most floating-point surprise, because tenths are the fractions humans use
most.

**Fixed point.** A format with the binary point at a fixed position. Exact within
its grid and fast, but gives every value the same absolute precision, which is
wrong for quantities spanning many orders of magnitude.

**Floating point.** Scientific notation in base two: a signed mantissa times a
power of two, with the point's position carried in the exponent. Buys enormous
range and constant *relative* precision.

**Normalization and the hidden bit.** Requiring exactly one non-zero digit before
the point makes the representation unique; in base two that digit is always 1, so
it is not stored. Worth one extra bit of precision at no cost.

**IEEE 754 double.** 1 sign bit, 11 exponent bits, 52 fraction bits. The exponent
is stored biased by 1023, so that the bit patterns of positive floats sort in
numeric order and can be compared with integer hardware.

**Range and precision.** About ±1.798 × $10^{308}$, with roughly 15–17 significant
decimal digits at every magnitude. The gap between adjacent doubles is about
2.22 × $10^{-16}$ near 1.0 and 2.0 near $10^{16}$.

**The $2^{53}$ boundary.** Above 9,007,199,254,740,992 the spacing of doubles exceeds
1, so consecutive integers are no longer distinguishable. $2^{53}$ and $2^{53}$ + 1 are the
same `double`.

**Correct rounding.** Every IEEE 754 arithmetic operation returns the
representable value nearest the exact mathematical result. Individual operations
are never sloppy; loss accumulates across many of them.

**Round half to even.** Ties are resolved toward the candidate whose last
mantissa bit is 0, rather than always upward, so that rounding errors tend to
cancel rather than drift. `0.1 + 0.2` is decided by exactly this rule.

**Zero, infinity, NaN, subnormals.** Reserved exponent patterns. There are two
zeros; division by zero yields infinity rather than an error; NaN is unequal to
everything including itself, so `x == Double.NaN` is always false and
`Double.isNaN(x)` is required.

**Shortest round-tripping output.** Printing a `double` shows the shortest decimal
string that maps back to the same bit pattern, which is why `0.1` displays as
`0.1` despite not being one tenth. The discrepancy is hidden until an operation
exposes it.

**Absolute versus relative tolerance.** A fixed epsilon is meaningless without
knowing the magnitudes involved. Scaling the tolerance to the size of the
operands makes a comparison mean the same thing at every scale.

**Non-associativity.** `(a + b) + c` may differ from `a + (b + c)`. When summing
values of widely different magnitudes, add the small ones first; Kahan summation
does better by feeding the rounding error of each step back into the next.

**When to refuse.** If the quantity has a natural smallest unit and correctness
at that unit matters — money, counts, identifiers — use integers of that unit or
a decimal type. `BigDecimal` must be constructed from a *string*; constructing it
from a `double` preserves the error already present.
