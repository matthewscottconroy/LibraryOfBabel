# Exercises

Exercises marked **[carries forward]** introduce something a later chapter
assumes.

## Warm-up

**3.1.** Convert to decimal: `0.1` (binary), `0.011`, `0.1101`, `101.01`.

**3.2.** Write these in binary, exactly: 0.5, 0.75, 0.375, 2.25, 0.0625.

**3.3.** Which of these terminate in binary? 1/2, 1/3, 1/5, 1/16, 3/8, 7/10, 9/32.
State the rule you used rather than converting each one.

**3.4.** One third does not terminate in base ten. Name a base in which it does,
and say why.

## Working through

**3.5. [carries forward]** Convert 0.3 to binary using the multiply-by-two
method, going far enough to see the repeat begin. What is the repeating group?

**3.6.** A `double` has 52 fraction bits plus one hidden bit, giving 53
significant binary digits. Roughly how many significant decimal digits is that?
(Hint: how many decimal digits does one binary digit buy?)

**3.7.** The gap between adjacent doubles near 1.0 is about 2.22 × $10^{-16}$. Using
the fact that the gap doubles each time the exponent increases by one, estimate
the gap near 1000.0. Then near $10^{16}$, and check your answer against the claim in
the text that it is 2.0.

**3.8. [carries forward]** Explain, in terms of the 53-bit significand, why
$2^{53}$ and $2^{53}$ + 1 are the same `double`. What is the largest integer *n* such that
every whole number from 0 to *n* is exactly representable?

**3.9.** `0.1 + 0.2` landed exactly halfway between two doubles and was resolved
by round-half-to-even. Explain why "always round halves up" would be a worse rule
for a long computation.

## Reasoning

**3.10.** A programmer writes `if (Math.abs(a - b) < 0.0001)` to compare two
doubles. Give a pair of values for which this wrongly reports "equal", and a pair
for which it wrongly reports "different". Then write a comparison that handles
both.

**3.11.** Why is `x == Double.NaN` always false? What is it about NaN's intended
role that makes this the right design rather than a mistake?

**3.12.** Adding a million 1.0s to $10^{16}$ changed nothing, but adding them to each
other first and then to $10^{16}$ gave the right answer. Explain both outcomes in
terms of the gap between adjacent doubles.

**3.13.** Floating-point addition is not associative. Construct three doubles
*a*, *b*, *c* for which `(a + b) + c` differs from `a + (b + c)`, and show both
results.

## Going further

**3.14.** You are asked to store the results of a stopwatch, to the millisecond,
for events lasting up to a year. Would you use `double`, `long` milliseconds, or
`BigDecimal`? Defend the choice using the test from Section 3.2.3, and say what
the other two would cost.

**3.15.** `new BigDecimal(0.10)` gives 0.1000000000000000055511…, while
`new BigDecimal("0.10")` gives 0.10. Explain precisely where the difference
enters, and why no amount of care inside `BigDecimal` could repair the first case.

**3.16.** Kahan summation recovers the lost low-order bits with the line
`c = (u - sum) - y`, which in exact arithmetic would be zero. Work through the
ten-tenths example by hand for the first three iterations and show what `c`
holds after each.

**3.17.** IEEE 754 has two zeros, `+0.0` and `−0.0`, which compare as equal.
Find out what `1.0 / 0.0` and `1.0 / -0.0` produce, and construct a situation in
which the distinction between the two zeros changes a program's answer.

**3.18.** The chapter claimed that a base-ten machine would represent 1/10
exactly but fail on 1/3, while base two does the reverse. Design a numeric format
that represents *both* exactly, then say what it costs — there is a reason
nobody uses your format for general arithmetic.
