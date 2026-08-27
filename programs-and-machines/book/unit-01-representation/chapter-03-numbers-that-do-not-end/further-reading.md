# Further Reading

## The standard reference

Goldberg, D. (1991). "What Every Computer Scientist Should Know About
Floating-Point Arithmetic." *ACM Computing Surveys*, 23(1), 5–48.

The paper to read after this chapter. Freely available, and it treats
rigorously everything sketched here: rounding error, cancellation, the design
rationale for the standard's more surprising decisions. Do not be discouraged by
the first pass; the first ten pages repay a slow reading even if the error
analysis later on does not yet.

## The standard itself

*IEEE Standard for Floating-Point Arithmetic*, IEEE Std 754-2019. IEEE.

The authority. Not a tutorial, and not cheap, but worth knowing it exists — when
someone asserts what floating point "does", this is the document that settles it.

*The Java Language Specification*, Java SE 17 edition. Oracle. Sections 4.2.3
(floating-point types) and 15.4 (floating-point operations).

Java's guarantees are stated here, including that `float` and `double` are IEEE
754 single and double precision and that arithmetic is correctly rounded.

## Going deeper

Muller, J.-M., et al. (2018). *Handbook of Floating-Point Arithmetic* (2nd ed.).
Birkhäuser.

Comprehensive, and considerably beyond this book. The place to go when you need
an algorithm rather than an understanding.

Higham, N. J. (2002). *Accuracy and Stability of Numerical Algorithms* (2nd ed.).
SIAM.

On what happens to error when it passes through a whole algorithm rather than a
single operation. This is the discipline called numerical analysis, and this is
its standard text.

## Practical

Kahan, W. (1996). "Lecture Notes on the Status of IEEE Standard 754 for Binary
Floating-Point Arithmetic." University of California, Berkeley.

Kahan writing about his own standard, including what he thinks implementers got
wrong. Opinionated and rewarding.

Bloch, J. (2018). *Effective Java* (3rd ed.). Addison-Wesley. Item 60, "Avoid
`float` and `double` if exact answers are required".

Two pages making the argument of Section 3.2.3 in Java-specific terms, with the
`BigDecimal` and integer-cents options laid out. Worth reading now and again
after Unit V.

## For fun

The `0.30000000000000004` website — a single page listing what `0.1 + 0.2`
produces in dozens of programming languages, together with an explanation.

It is the fastest way to convince yourself that this is not a defect of Java.
Nearly every language gives the same answer, because nearly every language uses
the same hardware, which implements the same standard.
