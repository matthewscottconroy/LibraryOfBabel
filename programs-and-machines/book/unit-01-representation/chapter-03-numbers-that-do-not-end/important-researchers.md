# People

## William Kahan (born 1933)

A Canadian mathematician and computer scientist at Berkeley, and the person most
responsible for the fact that floating-point arithmetic behaves the same way on
every machine you will ever use.

Before IEEE 754, every manufacturer had its own floating-point format, with its
own range, its own rounding rules, and its own handling of overflow and division
by zero. A numerical program that worked on one machine could give different
answers — or fail — on another, and there was no way to reason portably about
error. Kahan was the principal architect of the standard that ended this,
adopted in 1985, and received the Turing Award in 1989 for it.

The details he fought for are the ones this chapter relies on: correctly rounded
results for every basic operation, round-half-to-even as the default tie-breaker,
gradual underflow through subnormal numbers, and NaN as a value that propagates
rather than a condition that halts. Kahan summation is also his.

He has spent much of his career arguing, with some asperity, that programming
languages and their implementers routinely undermine the guarantees the standard
provides. Reading him is a good corrective to the idea that a standard settles
anything by itself.

## Konrad Zuse (1910–1995)

A German civil engineer who built the Z1 in his parents' apartment in Berlin
between 1936 and 1938, and the relay-based Z3 in 1941.

Zuse's machines were binary and used floating-point arithmetic — decades before
the American designers who are usually credited with the idea, and largely
independent of them. The Z3's format had a sign bit, a 7-bit exponent, and a
14-bit mantissa, with a hidden leading bit, and it handled exceptional values by
detecting them explicitly. He arrived at floating point because he was an
engineer doing structural calculations, and engineers had been using scientific
notation for a century.

He is included here as a reminder that the choices in this chapter were not
discovered by a committee. They were arrived at repeatedly, by people whose
arithmetic had to work.

## John von Neumann (1903–1957)

Von Neumann appeared in Chapter 2 as an advocate of binary. He appears here as an
opponent of floating point.

His view, argued in the EDVAC-era design discussions, was that floating point
cost hardware that could better be spent elsewhere, and that a competent
numerical analyst could scale a problem by hand to keep it within fixed-point
range. Given the cost of vacuum tubes in 1946, this was a serious position rather
than a blind spot.

He was right about the hardware and wrong about the people. As machines spread to
users who were not numerical analysts, the labour of manual scaling became the
dominant cost, and floating point won. It is a useful case of a technical
judgment that was correct on its own terms and overturned by a change in who was
doing the work.

## David Goldberg

A computer scientist whose 1991 survey *What Every Computer Scientist Should Know
About Floating-Point Arithmetic*, published in *ACM Computing Surveys*, remains
the standard reference for the material in this chapter treated properly.

It is more mathematical than what you have read here — it derives error bounds
rather than demonstrating them — and it is the natural next step once this
chapter's ideas are comfortable. The title is not a boast; the paper is widely
regarded as required reading, and the number of production bugs it would have
prevented is not small.
