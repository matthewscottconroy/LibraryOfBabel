# Randomness as Incompressibility

Sixteen coin flips come up all heads. You would say something was wrong with the
coin.

But a fair coin produces `HHHHHHHHHHHHHHHH` with exactly the same probability as
any other particular sequence — one in 65,536, the same as the messy-looking one
you would have accepted without comment. Probability cannot tell them apart, and
yet you are clearly noticing something real.

What does it mean for a string to be random?

The word is used constantly and it resists definition. Consider three sequences of
sixteen coin flips:

```
HHHHHHHHHHHHHHHH
HTHTHTHTHTHTHTHT
HTTHHHTHTTHHTHTH
```

The third looks random and the first two do not. But a fair coin produces each
with probability exactly $2^{-16}$. Probability cannot distinguish them, because
under the intended distribution they are equally likely.

So randomness is not a property of how a string was generated. Something else is
being noticed about the strings themselves.

## The definition

Kolmogorov's answer, and it is the one that works:

> A string is **random** if it has no description shorter than itself — if
> $K(s) \ge |s|$, up to a small constant.

The first sequence compresses to "H sixteen times". The second to "HT eight
times". The third has no shorter description than itself.

That is exactly what people mean by random when they look at the third and not the
first two. **Random means patternless, and patternless means incompressible.**

The definition has properties you would want.

**Almost all strings are random.** Section 34.2.1's counting: fewer than one in
$2^k$ can be shortened by $k$ bits. Randomness is the overwhelming default.

**Random strings pass statistical tests.** A string with too many `H`s could be
described as "mostly H with exceptions at these positions", which is shorter. So
incompressibility implies the frequencies come out right — the statistical
properties follow from the definition rather than being assumed.

**It applies to individual strings**, which probability cannot do. "This particular
sequence is random" is meaningless probabilistically and perfectly meaningful here.

## And no program can find one

Now the sting, and it is the chapter's last result.

Almost every string is random. **No program can exhibit a single one.**

Suppose a program could test whether a string is random. Then Section 34.2.1's
argument applies directly: search strings in order, return the first random one of
length $n$. It exists, so the search terminates — and the program that found it is
short, about $c + \log_2 n$ bits, which is a description of a string that by
construction has no short description.

Contradiction. So randomness is undecidable.

Sit with the pair of statements.

**Almost every string is random.** Pick one at random and it is, with probability
essentially 1.

**No program can verify that any particular string is random.** Not one, ever.

Those are both true and they are not in tension, though they feel as though they
should be. The property is overwhelmingly common and individually unverifiable.
You can know that almost everything has it and never confirm it of anything.

That combination is, in miniature, what this unit has been about: the reachable
part of a space can be vanishingly small and still be the only part you can work
in.

## Practical randomness

Nothing above stops you from needing random numbers, and it does shape what you
can claim.

**Pseudorandom generators** are programs. `Random` in Java produces a sequence from
a seed, and the sequence is entirely determined by it — so its Kolmogorov
complexity is about the size of the generator plus the seed, which is tiny. A
gigabyte from `new Random(42)` has complexity a few hundred bits.

It is *not random* in Kolmogorov's sense, and it passes statistical tests, and for
simulation that is fine. Reproducibility is often a feature: the same seed gives
the same run, which is what makes a stochastic simulation debuggable.

**Cryptographic generators** are pseudorandom generators whose output is
computationally infeasible to distinguish from random without the seed. Still not
Kolmogorov-random, and the security rests on a computational assumption rather
than on incompressibility.

**Hardware sources** — thermal noise, radioactive decay, timing jitter — are the
closest thing to genuine randomness available, and they are used to seed the
generators above.

The distinction that matters in practice: **use `Random` for simulation and
`SecureRandom` for anything security-related.** `Random` is fast, reproducible,
and predictable from a few outputs — its state can be recovered from two
consecutive values, which is a real vulnerability when it is used for tokens or
passwords, and it happens regularly.

## Closing the unit

Three impossibility results, one technique.

**Chapter 32.** No comparison sort beats $n \log n$. Count the orderings, count
what the comparisons distinguish.

**Chapter 33.** No compressor shrinks everything. Count the inputs, count the
shorter outputs.

**Chapter 34.** No program decides halting, computes $K$, or identifies a random
string. Count the programs, observe there are more questions than programs, and
construct one specific contradiction.

Each says something cannot be done, none says how anything is done, and all three
are proved by counting. That technique is the most reliable one in the subject
for establishing a limit, and its power is precisely that it ignores the method.

And each has a practical form.

The sorting bound tells you to stop looking for a faster comparison sort and to
consider not comparing. The compression bound tells you to disbelieve universal
compression claims without reading them. Undecidability tells you why your
compiler warns instead of proving, why your static analyser has false positives,
and why "just write a tool that checks it" is sometimes not available.

**Knowing where the wall is is more useful than believing there is no wall.**

That is the last technical thing this book has to say. Chapter 35 looks back at
the whole of it.
