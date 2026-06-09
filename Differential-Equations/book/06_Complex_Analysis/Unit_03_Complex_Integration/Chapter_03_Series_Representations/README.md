# Chapter 03: Series Representations

Every analytic function can be represented locally by a power series — this is Taylor's theorem for analytic functions, and it shows that "analytic" and "locally equal to its power series" are synonymous. Near an isolated singularity, the appropriate representation is the Laurent series, which includes negative powers and whose principal part (the negative-power terms) determines the nature of the singularity.

## Section 01: Taylor Series

**Theorem (Taylor).** If $f$ is analytic on the disk $D(z_0, R)$, then for all $z \in D(z_0, R)$:
$$f(z) = \sum_{n=0}^{\infty} \frac{f^{(n)}(z_0)}{n!}(z - z_0)^n.$$
The series converges absolutely and uniformly on every closed subdisk $|z - z_0| \leq r < R$.

This is proved by writing $f(z_0 + h) = \frac{1}{2\pi i}\oint_C \frac{f(w)}{w - (z_0 + h)}\, dw$ and expanding $\frac{1}{w-z_0-h}$ as a geometric series in $h/(w-z_0)$.

## Section 02: Laurent Series

If $f$ is analytic on an annulus $r < |z - z_0| < R$, it has a Laurent expansion:
$$f(z) = \sum_{n=-\infty}^{\infty} a_n(z - z_0)^n.$$
The coefficients are $a_n = \frac{1}{2\pi i}\oint_C \frac{f(z)}{(z-z_0)^{n+1}}\, dz$ for any circle $C$ in the annulus.

The terms with $n < 0$ form the principal part; those with $n \geq 0$ form the analytic part.

## Section 03: Classification of Singularities

An isolated singularity at $z_0$ is classified by the Laurent series:
- **Removable singularity:** the principal part is $0$ ($a_n = 0$ for $n < 0$). The function extends analytically to $z_0$.
- **Pole of order $m$:** the principal part has finitely many terms, with $a_{-m} \neq 0$ and $a_n = 0$ for $n < -m$.
- **Essential singularity:** the principal part has infinitely many nonzero terms.

## Section 04: Zeros and Poles

A zero of order $m$ at $z_0$ means $f(z) = (z-z_0)^m g(z)$ with $g$ analytic and $g(z_0) \neq 0$. A pole of order $m$ at $z_0$ means $1/f$ has a zero of order $m$ there. Zeros and poles interact: if $f$ has a zero of order $m$ and $g$ has a pole of order $n$ at $z_0$, then $fg$ has a zero of order $m - n$ (if $m > n$) or a pole of order $n - m$ (if $n > m$).

## Learning Objectives

After this chapter, a student should be able to:

- Compute Taylor series for standard functions using the definition, known series, and algebraic manipulation.
- Find the radius of convergence of a Taylor series.
- Compute Laurent series in annular regions for functions with given singularities.
- Classify isolated singularities as removable, pole, or essential using the Laurent expansion.
- Determine the order of a zero or pole from the Taylor or Laurent series.
- Compute residues from the Laurent series coefficient $a_{-1}$.
