# Zeros and Poles

Zeros and poles are the structural features that determine the global behavior of meromorphic functions. Their orders, their locations, and the relationships between them govern integration via residues, provide information about the function's growth and decay, and appear in the argument principle that counts zeros and poles inside a contour. This section studies zeros and poles precisely, establishes their relationship, and develops computational techniques.

## Zeros

**Definition.** A point $z_0$ is a zero of order $m$ (or a zero of multiplicity $m$) of a function $f$ analytic near $z_0$ if $f^{(k)}(z_0) = 0$ for $k = 0, 1, \ldots, m-1$ and $f^{(m)}(z_0) \neq 0$.

Equivalently, the Taylor series of $f$ at $z_0$ has the form:
$$f(z) = a_m(z-z_0)^m + a_{m+1}(z-z_0)^{m+1} + \cdots = (z-z_0)^m g(z),$$
where $g$ is analytic near $z_0$ and $g(z_0) = a_m \neq 0$.

A zero of order $1$ is called a simple zero.

**Isolation of zeros.** If $f$ is analytic and nonconstant on a domain $D$, and $f(z_0) = 0$, then $z_0$ is an isolated zero: there exists $r > 0$ such that $f(z) \neq 0$ for $0 < |z - z_0| < r$.

**Proof.** By the factorization $f(z) = (z - z_0)^m g(z)$ with $g(z_0) \neq 0$, continuity of $g$ gives $g(z) \neq 0$ in some neighborhood of $z_0$. Hence $(z - z_0)^m g(z) = 0$ only at $z = z_0$ in this neighborhood. $\square$

This isolation fails for non-analytic functions: $g(x) = x\sin(1/x)$ (extended by $0$ at $x = 0$) has zeros accumulating at $0$ and is smooth there.

## Poles

**Definition.** A function $f$ has a pole of order $m$ at $z_0$ if $f$ is analytic in $0 < |z - z_0| < R$ and the Laurent expansion takes the form:
$$f(z) = \frac{a_{-m}}{(z-z_0)^m} + \cdots + \frac{a_{-1}}{z-z_0} + a_0 + a_1(z-z_0) + \cdots,$$
with $a_{-m} \neq 0$.

**Theorem.** $f$ has a pole of order $m$ at $z_0$ if and only if $g(z) = 1/f(z)$ has a zero of order $m$ at $z_0$.

**Proof.** If $f(z) = \frac{a_{-m}}{(z-z_0)^m}(1 + \text{higher order terms})$, then $1/f(z) = \frac{(z-z_0)^m}{a_{-m}}(1 + \cdots)$, which has a zero of order $m$ at $z_0$. The converse is identical. $\square$

## Relationship Between Zeros and Poles of Products and Quotients

**Theorem.** Let $f$ have a zero of order $m$ at $z_0$ and $g$ have a zero of order $n$ at $z_0$ (or a pole of order $n$ if $n < 0$). Then:
- $fg$ has a zero of order $m + n$ (or pole of order $|m+n|$ if negative) at $z_0$.
- $f/g$ has a zero of order $m - n$ at $z_0$ (if $m > n$), a pole of order $n - m$ (if $n > m$), or a removable singularity with nonzero limit (if $m = n$).

**Worked examples.**
- $f(z) = \sin z$, $g(z) = z$: $f$ has a zero of order $1$ at $0$, $g$ has a zero of order $1$. $f/g = (\sin z)/z$ has a removable singularity at $0$ with limit $1$.
- $f(z) = z^2$, $g(z) = \sin z$: $f/g$ has a zero of order $2 - 1 = 1$ at $0$.
- $f(z) = z$, $g(z) = z^3$: $f/g = 1/z^2$ has a pole of order $2$ at $0$.

## Computing Residues from the Order of a Pole

For a simple pole ($m = 1$):
$$\mathrm{Res}(f; z_0) = \lim_{z \to z_0}(z - z_0)f(z).$$

For a quotient $f = p/q$ with $p(z_0) \neq 0$ and $q$ having a simple zero at $z_0$:
$$\mathrm{Res}(f; z_0) = \frac{p(z_0)}{q'(z_0)}.$$

For a pole of order $m$:
$$\mathrm{Res}(f; z_0) = \frac{1}{(m-1)!}\lim_{z \to z_0}\frac{d^{m-1}}{dz^{m-1}}\left[(z-z_0)^m f(z)\right].$$

**Worked example.** Find the residue of $f(z) = \frac{z^2 + 1}{z^3(z+1)^2}$ at $z = 0$ and $z = -1$.

At $z = 0$ (pole of order $3$): $(z-0)^3 f(z) = \frac{z^2+1}{(z+1)^2}$. Then:
$$\mathrm{Res}(f;0) = \frac{1}{2!}\frac{d^2}{dz^2}\frac{z^2+1}{(z+1)^2}\bigg|_{z=0}.$$
Let $h(z) = (z^2+1)/(z+1)^2$. $h'(z) = \frac{2z(z+1)^2 - 2(z+1)(z^2+1)}{(z+1)^4} = \frac{2z(z+1) - 2(z^2+1)}{(z+1)^3} = \frac{2z - 2}{(z+1)^3}$.

$h''(z) = \frac{2(z+1)^3 - 3(z+1)^2(2z-2)}{(z+1)^6} = \frac{2(z+1) - 3(2z-2)}{(z+1)^4} = \frac{2z + 2 - 6z + 6}{(z+1)^4} = \frac{-4z + 8}{(z+1)^4}$.

At $z = 0$: $h''(0) = 8$. So $\mathrm{Res}(f;0) = 8/2 = 4$.

At $z = -1$ (pole of order $2$): $(z+1)^2 f(z) = \frac{z^2+1}{z^3}$. Then:
$\mathrm{Res}(f;-1) = \frac{d}{dz}\frac{z^2+1}{z^3}\bigg|_{z=-1} = \frac{2z \cdot z^3 - 3z^2(z^2+1)}{z^6}\bigg|_{z=-1} = \frac{2z^2 - 3z^2 - 3}{z^4}\bigg|_{z=-1} = \frac{-z^2 - 3}{z^4}\bigg|_{z=-1} = \frac{-1-3}{1} = -4$. $\square$

## The Argument Principle

**Theorem (Argument Principle).** Let $f$ be meromorphic on and inside a simple closed contour $C$, with no zeros or poles on $C$. If $f$ has $Z$ zeros and $P$ poles inside $C$ (counting multiplicity), then:
$$\frac{1}{2\pi i}\oint_C \frac{f'(z)}{f(z)}\, dz = Z - P.$$

**Proof.** Near a zero of order $m$: $f = (z-z_0)^m g$ with $g(z_0) \neq 0$, so $f'/f = m/(z-z_0) + g'/g$, and the $g'/g$ term is analytic near $z_0$. Its residue is $m$. Near a pole of order $m$: $f = (z-z_0)^{-m}h$ with $h$ analytic and nonzero, so $f'/f = -m/(z-z_0) + h'/h$ with residue $-m$. By the residue theorem, $\frac{1}{2\pi i}\oint_C f'/f\, dz = \sum (\text{zeros, with order}) - \sum(\text{poles, with order}) = Z - P$. $\square$

**Rouche's theorem.** If $|f(z) - g(z)| < |f(z)|$ on $C$, then $f$ and $g$ have the same number of zeros inside $C$. This is proved using the argument principle applied to $g/f$ and is useful for locating zeros of polynomials or establishing that a function has a prescribed number of zeros in a region.
