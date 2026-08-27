# Binary Fractions

In Chapter 2 we said that in base ten the positions to the left of the decimal
point are worth 1, 10, 100, and so on. We stopped there. Now go right.

```
    1   2   .   3   7   5
    │   │       │   │   └── 5 × 1/1000  = 0.005
    │   │       │   └────── 7 × 1/100   = 0.07
    │   │       └────────── 3 × 1/10    = 0.3
    │   └────────────────── 2 × 1       = 2
    └────────────────────── 1 × 10      = 10
                                          ──────
                                          12.375
```

The rule did not change. Each position is worth one tenth of the position to its
left, and that continues indefinitely in both directions. The decimal point is
not an operator; it is a marker showing where the ones place is.

## The same rule in base two

Set the base to 2 and the positions to the right of the point are worth 1/2, 1/4,
1/8, 1/16 — the negative powers of two.

So `0.11` in binary is 1/2 + 1/4 = 0.75.

And `1100.011` in binary is 8 + 4 + 1/4 + 1/8 = 12.375. The same number we just
wrote in decimal, in a different numeral.

Try one yourself before reading on. What is `0.101` in binary?

It is 1/2 + 1/8 = 0.625.

That is all a binary fraction is. There is no new machinery — it is the
positional rule from Chapter 2, run in the other direction.

## The uncomfortable question

Now the question the chapter turns on.

Write one third in decimal. You get 0.3333… and it never stops. You have known
this since primary school and you probably filed it under "some fractions are
like that". But *which* fractions are like that, and why?

Here is the answer, and it is worth deriving because it explains everything that
follows.

A fraction written in base *b* terminates exactly when, after reducing it to
lowest terms, **every prime factor of its denominator is also a factor of *b***.

Check it against what you know. Base ten is 2 × 5, so the only denominators that
terminate are those built from 2s and 5s:

- 1/2 = 0.5 — terminates. Denominator is 2. **yes**
- 1/4 = 0.25 — terminates. 4 = 2 × 2. **yes**
- 1/5 = 0.2 — terminates. **yes**
- 1/8 = 0.125 — terminates. 8 = $2^{3}$. **yes**
- 1/10 = 0.1 — terminates. 10 = 2 × 5. **yes**
- 1/20 = 0.05 — terminates. 20 = $2^{2}$ × 5. **yes**
- 1/3 = 0.333… — does not. 3 is not a factor of 10. **no**
- 1/7 = 0.142857142857… — does not. **no**
- 1/6 = 0.1666… — does not. 6 = 2 × 3, and the 3 spoils it. **no**

The rule holds. And notice it is a fact about the *numeral*, not the *number*.
One third is a perfectly definite quantity; it is our base-ten notation that
cannot write it down in finitely many symbols.

## Now run the rule in base two

Base two has exactly one prime factor: 2. So a fraction terminates in binary only
when its reduced denominator is a power of two.

- 1/2 = `0.1` **yes**
- 1/4 = `0.01` **yes**
- 3/4 = `0.11` **yes**
- 1/8 = `0.001` **yes**
- 5/16 = `0.0101` **yes**
- **1/10 — denominator 10 = 2 × 5, and 5 is not a factor of 2.** **no**

One tenth does not terminate in binary.

Sit with that for a moment, because it is the whole chapter. The number you write
as `0.1` — the most ordinary decimal fraction there is, the one on every price
tag — is a repeating fraction in base two, in exactly the way one third is a
repeating fraction in base ten.

## Seeing it happen

Let us actually compute it, using the mirror of the repeated-division method from
Chapter 2. To convert a fraction to base two, repeatedly multiply by 2 and record
the whole part:

```
0.1 × 2 = 0.2   → 0
0.2 × 2 = 0.4   → 0
0.4 × 2 = 0.8   → 0
0.8 × 2 = 1.6   → 1   (keep 0.6)
0.6 × 2 = 1.2   → 1   (keep 0.2)
0.2 × 2 = 0.4   → 0
0.4 × 2 = 0.8   → 0
0.8 × 2 = 1.6   → 1   (keep 0.6)
0.6 × 2 = 1.2   → 1   (keep 0.2)
```

Look at what happened at the sixth row: we are back to 0.2, which we already saw
at row two. From here the sequence must repeat forever, because the procedure is
deterministic and the state has recurred.

So one tenth in binary is

```
0.0001100110011001100110011...
```

with `0011` repeating without end. In the notation used for repeating decimals,
0.0001100 with the last four digits overlined.

## What this means for a machine

A machine has finitely many bits. An unending expansion cannot be stored. So
something must be discarded, and whatever is stored is *not* one tenth — it is
the nearest value the format can represent.

This is not a defect in the machine, and it is not a defect in binary. It is the
same limitation base ten has with one third, moved to a place where it surprises
us because the affected fractions are ones we use constantly.

Notice which way the surprise runs. A base-ten machine would store 1/10 exactly
and fail on 1/3. Our base-two machines store 1/4 and 1/8 exactly and fail on
1/10. Neither is more accurate than the other; they fail on different sets. It is
only because money and measurement are written in tenths that base two's failures
are the ones we trip over.

Next: how the format decides which values to keep.
