# Taylor Series for Analytic Functions

Taylor's theorem in real analysis gives a polynomial approximation to a smooth function, with a remainder that can be bounded. In complex analysis, the analogous theorem is dramatically stronger: every analytic function equals its Taylor series exactly, with no remainder, and the series converges on an entire open disk. This equivalence between analyticity and power series representability is the key structural fact of the subject.

## The Taylor Series Theorem

**Theorem (Taylor's Theorem for Analytic Functions).** Let $f$ be analytic on the open disk $D(z_0, R) = \{z : |z - z_0| < R\}$. Then for all $z \in D(z_0, R)$:
$$f(z) = \sum_{n=0}^{\infty} a_n(z - z_0)^n, \qquad a_n = \frac{f^{(n)}(z_0)}{n!} = \frac{1}{2\pi i}\oint_{|w-z_0|=r}\frac{f(w)}{(w-z_0)^{n+1}}\, dw$$
for any $r \in (0, R)$. The series converges absolutely and uniformly on every closed subdisk $|z - z_0| \leq \rho < R$.

**Proof.** Fix $z \in D(z_0, R)$ and choose $r$ with $|z - z_0| < r < R$. By the Cauchy integral formula:
$$f(z) = \frac{1}{2\pi i}\oint_{|w-z_0|=r}\frac{f(w)}{w - z}\, dw.$$
Write $w - z = (w - z_0) - (z - z_0)$ and expand:
$$\frac{1}{w - z} = \frac{1}{(w-z_0)\left(1 - \frac{z-z_0}{w-z_0}\right)} = \frac{1}{w-z_0}\sum_{n=0}^{\infty}\left(\frac{z-z_0}{w-z_0}\right)^n,$$
valid because $|z - z_0|/|w - z_0| = |z - z_0|/r < 1$. Substituting and exchanging sum and integral (justified by uniform convergence on the circle $|w - z_0| = r$):
$$f(z) = \sum_{n=0}^{\infty}\left(\frac{1}{2\pi i}\oint_{|w-z_0|=r}\frac{f(w)}{(w-z_0)^{n+1}}\, dw\right)(z-z_0)^n = \sum_{n=0}^{\infty} a_n(z-z_0)^n. \quad \square$$

## Radius of Convergence

The Taylor series of $f$ centered at $z_0$ converges on the largest open disk centered at $z_0$ on which $f$ is analytic. The radius of convergence equals the distance from $z_0$ to the nearest singularity of $f$.

**Examples:**
- $f(z) = \frac{1}{1-z}$: singularity at $z = 1$. Taylor series centered at $0$ has radius $1$: $\sum_{n=0}^\infty z^n$.
- $f(z) = e^z$: no singularities. Taylor series at any center has infinite radius.
- $f(z) = \frac{1}{1 + z^2}$: singularities at $z = \pm i$. Taylor series centered at $0$ has radius $1$ (distance to $\pm i$): $\sum_{n=0}^\infty (-1)^n z^{2n}$.

## Standard Taylor Series at $z_0 = 0$

These are valid for all $z \in \mathbb{C}$ (entire functions) or on the indicated disk:
$$e^z = \sum_{n=0}^{\infty}\frac{z^n}{n!}, \qquad |z| < \infty.$$
$$\sin z = \sum_{n=0}^{\infty}\frac{(-1)^n z^{2n+1}}{(2n+1)!}, \qquad |z| < \infty.$$
$$\cos z = \sum_{n=0}^{\infty}\frac{(-1)^n z^{2n}}{(2n)!}, \qquad |z| < \infty.$$
$$\frac{1}{1-z} = \sum_{n=0}^{\infty} z^n, \qquad |z| < 1.$$
$$\log(1+z) = \sum_{n=1}^{\infty}\frac{(-1)^{n-1}z^n}{n}, \qquad |z| < 1.$$
$$(1+z)^\alpha = \sum_{n=0}^{\infty}\binom{\alpha}{n}z^n, \qquad |z| < 1,$$
where $\binom{\alpha}{n} = \frac{\alpha(\alpha-1)\cdots(\alpha-n+1)}{n!}$ is the generalized binomial coefficient.

## Computing Taylor Series

**Method 1: Direct differentiation.** Compute $a_n = f^{(n)}(z_0)/n!$.

**Method 2: Manipulation of known series.** Substitute, multiply, divide, differentiate, or integrate known series.

**Worked example.** Find the Taylor series of $f(z) = \frac{1}{(1-z)^2}$ centered at $z_0 = 0$.

Differentiate $\frac{1}{1-z} = \sum_{n=0}^\infty z^n$ with respect to $z$:
$$\frac{1}{(1-z)^2} = \sum_{n=1}^{\infty} nz^{n-1} = \sum_{n=0}^{\infty}(n+1)z^n, \qquad |z| < 1.$$

**Worked example.** Find the first four nonzero terms of the Taylor series of $\tan z$ at $z_0 = 0$.

Write $\tan z = \sin z / \cos z$ and perform long division of power series:
$$\sin z = z - \frac{z^3}{6} + \frac{z^5}{120} - \cdots, \qquad \cos z = 1 - \frac{z^2}{2} + \frac{z^4}{24} - \cdots.$$
Dividing: $\tan z = z + \frac{z^3}{3} + \frac{2z^5}{15} + \frac{17z^7}{315} + \cdots$.

(Radius of convergence is $\pi/2$, the distance from $0$ to the nearest singularity of $\tan$.)

## Zeros and the Taylor Series

If $f(z_0) = 0$, then $a_0 = 0$. If also $f'(z_0) = 0$, then $a_1 = 0$, and so on. The order of the zero at $z_0$ is the smallest $m$ with $a_m \neq 0$:
$$f(z) = a_m(z-z_0)^m + a_{m+1}(z-z_0)^{m+1} + \cdots = (z-z_0)^m g(z),$$
where $g(z_0) = a_m \neq 0$ and $g$ is analytic near $z_0$. The order of the zero is $m$.

**Worked example.** Show that $\sin z$ has a simple zero at $z = 0$.

$\sin z = z - z^3/6 + \cdots$. The first nonzero term has $m = 1$, so the zero at $0$ has order $1$ (simple). $\square$

## Analytic Continuation

The Taylor series of $f$ at $z_0$ converges on the largest disk in which $f$ is analytic. But the coefficients $a_n = f^{(n)}(z_0)/n!$ depend only on the function near $z_0$: given $f$ defined on a small neighborhood of $z_0$, we can compute all $a_n$ and the series gives an analytic function on a larger disk. This process of extending the domain of definition of an analytic function — from a small region to a larger one via overlapping power series — is called analytic continuation.

**Key fact (uniqueness of analytic continuation).** If two analytic functions agree on a set with an accumulation point, they agree on their entire common domain. This means that if a power series at $z_0$ can be analytically continued to a larger domain, the result is unique.
