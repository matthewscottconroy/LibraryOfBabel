# Laurent Series

When a function has an isolated singularity at $z_0$, it cannot have a Taylor series centered there, but it can be expanded in a generalized power series that includes negative powers of $(z - z_0)$. This is the Laurent series, and it is the correct local description of an analytic function on a punctured disk or annulus. The Laurent series is not merely a computational device: its structure — specifically, how many negative-power terms appear and whether they continue indefinitely — completely determines the type of singularity.

## The Laurent Series Theorem

**Theorem (Laurent's Theorem).** Let $f$ be analytic on the annulus $A = \{z : r < |z - z_0| < R\}$ (where $0 \leq r < R \leq \infty$). Then $f$ has a unique representation:
$$f(z) = \sum_{n=-\infty}^{\infty} a_n(z - z_0)^n = \cdots + \frac{a_{-2}}{(z-z_0)^2} + \frac{a_{-1}}{z - z_0} + a_0 + a_1(z-z_0) + \cdots,$$
where the coefficients are:
$$a_n = \frac{1}{2\pi i}\oint_C \frac{f(z)}{(z - z_0)^{n+1}}\, dz$$
for any circle $C : |z - z_0| = \rho$ with $r < \rho < R$. The series converges absolutely and uniformly on any compact subset of $A$.

**Proof sketch.** For $z \in A$, write the Cauchy integral representation as an integral over the outer circle $C_R$ minus an integral over the inner circle $C_r$. The outer integral gives the Taylor (analytic) part via the geometric series $1/(w - z) = \sum_{n \geq 0}(z-z_0)^n/(w-z_0)^{n+1}$. The inner integral gives the principal part via the expansion $-1/(z - w) = \sum_{n \geq 1}(w-z_0)^{n-1}/(z-z_0)^n$. Combining yields the Laurent series.

## Terminology

The **principal part** of the Laurent series is $\sum_{n=-\infty}^{-1} a_n(z-z_0)^n$ (the negative-power terms). The **analytic part** is $\sum_{n=0}^{\infty} a_n(z-z_0)^n$.

The coefficient $a_{-1}$ is called the **residue** of $f$ at $z_0$, written $\mathrm{Res}(f; z_0)$. It is the central quantity in residue theory (Unit 04).

## Computing Laurent Series

Laurent series are typically computed by manipulating known series (geometric, Taylor) rather than by direct integration.

**Worked example 1.** Find the Laurent series of $f(z) = \frac{1}{z(z-1)}$ in the annulus $0 < |z| < 1$.

Partial fractions: $f(z) = \frac{-1}{z} + \frac{1}{z-1} = \frac{-1}{z} - \frac{1}{1-z}$.

For $|z| < 1$: $\frac{1}{1-z} = \sum_{n=0}^\infty z^n$. So:
$$f(z) = -\frac{1}{z} - \sum_{n=0}^{\infty} z^n = -z^{-1} - 1 - z - z^2 - \cdots.$$
Residue at $z = 0$: $a_{-1} = -1$. $\square$

**Worked example 2.** Find the Laurent series of $f(z) = \frac{1}{z(z-1)}$ in the annulus $1 < |z| < \infty$.

For $|z| > 1$: $\frac{1}{z-1} = \frac{1}{z}\cdot\frac{1}{1 - 1/z} = \frac{1}{z}\sum_{n=0}^\infty z^{-n} = \sum_{n=0}^\infty z^{-(n+1)} = \sum_{m=1}^\infty z^{-m}$.

So $f(z) = -z^{-1} + \frac{1}{z-1} = -z^{-1} + z^{-1} + z^{-2} + z^{-3} + \cdots = z^{-2} + z^{-3} + \cdots = \sum_{n=2}^\infty z^{-n}$.

Residue: $a_{-1} = 0$. (The residue depends on the Laurent expansion valid at the point in question; to find the residue at $z = 1$, expand in $0 < |z-1| < 1$.) $\square$

**Worked example 3.** Find the Laurent series of $f(z) = e^{1/z}$ in $0 < |z| < \infty$.

Substitute $1/z$ into the Taylor series of $e^w = \sum w^n/n!$:
$$e^{1/z} = \sum_{n=0}^{\infty} \frac{1}{n! z^n} = 1 + \frac{1}{z} + \frac{1}{2z^2} + \frac{1}{6z^3} + \cdots.$$
The principal part has infinitely many terms: $\sum_{n=1}^\infty \frac{1}{n! z^n}$. Residue: $a_{-1} = 1$. $\square$

## Different Laurent Expansions in Different Annuli

A function can have different Laurent expansions in different annuli centered at the same point. This is not a contradiction: the Laurent theorem gives the unique expansion valid in a specified annulus.

**Example.** For $f(z) = \frac{1}{z(z-1)(z-2)}$, there are three singularities ($0, 1, 2$) and thus four relevant annuli around $z = 0$: $0 < |z| < 1$, $1 < |z| < 2$, and $|z| > 2$. The Laurent expansions in these three annuli are all different.

## Worked Example: The Exponential of $z + 1/z$

**Example.** Find the coefficient of $z^n$ in the Laurent expansion of $e^{(z + 1/z)/2}$ in $0 < |z| < \infty$.

This is related to the generating function for Bessel functions. Write:
$$e^{z/2} \cdot e^{1/(2z)} = \left(\sum_{m=0}^\infty \frac{z^m}{2^m m!}\right)\left(\sum_{k=0}^\infty \frac{z^{-k}}{2^k k!}\right).$$
The coefficient of $z^n$ (for $n \geq 0$) is obtained by taking $m - k = n$, so $k = m - n$:
$$a_n = \sum_{m=n}^{\infty}\frac{1}{2^m m!}\cdot\frac{1}{2^{m-n}(m-n)!} = \frac{1}{2^n}\sum_{j=0}^{\infty}\frac{1}{j!(j+n)!\cdot 4^j} \cdot \frac{1}{1}$$
(substituting $j = m - n$). This is related to the Bessel function $J_n(1) = \sum_{j=0}^\infty \frac{(-1)^j}{j!(j+n)!}(1/2)^{2j+n}$.

## Uniqueness of the Laurent Expansion

**Theorem.** The Laurent expansion of $f$ in a given annulus is unique.

**Proof.** If $\sum a_n(z-z_0)^n = \sum b_n(z-z_0)^n$ on the annulus, multiply both sides by $(z-z_0)^{-(m+1)}$ and integrate over a circle in the annulus. Uniform convergence allows term-by-term integration, and $\oint (z-z_0)^{n-m-1}\, dz = 2\pi i \cdot \mathbf{1}_{n = m}$. So $a_m = b_m$ for all $m$. $\square$
