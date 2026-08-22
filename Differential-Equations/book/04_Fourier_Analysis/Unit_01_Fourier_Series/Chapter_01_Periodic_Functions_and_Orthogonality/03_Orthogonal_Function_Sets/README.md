# Orthogonal Function Sets

The trigonometric system $\{1, \cos x, \sin x, \cos 2x, \sin 2x, \ldots\}$ is not the only family of orthogonal functions in $L^2$. Many important problems in mathematical physics give rise to their own natural families of orthogonal functions, each adapted to the geometry and boundary conditions of the problem at hand. Understanding orthogonal function sets in general — what makes them useful, when they form complete bases, and how to use them — provides a unifying framework that applies far beyond Fourier series.

## Orthogonal Sets: Definitions

Let $\{f_n\}_{n=0}^\infty$ be a sequence of nonzero functions in $L^2([a,b], w)$, where $w > 0$ is a weight function. The sequence is **orthogonal** (with respect to the weighted inner product $\langle f, g \rangle_w = \int_a^b fg\,w\,dx$) if
$$\langle f_m, f_n \rangle_w = 0 \quad \text{whenever } m \neq n.$$
It is **orthonormal** if additionally $\langle f_n, f_n \rangle_w = 1$ for all $n$. Any orthogonal set can be normalized to an orthonormal set by dividing each function by its norm: $\hat{f}_n = f_n / \|f_n\|_w$.

## The Trigonometric System

On $[-\pi, \pi]$ with $w \equiv 1$, the functions
$$1,\; \cos x,\; \sin x,\; \cos 2x,\; \sin 2x,\; \ldots$$
form an orthogonal set. With the inner product $\langle f, g \rangle = \int_{-\pi}^\pi f(x)g(x)\,dx$, the norms are:
$$\|1\|^2 = 2\pi, \quad \|\cos(nx)\|^2 = \pi, \quad \|\sin(nx)\|^2 = \pi \quad (n \geq 1).$$
The orthonormal version uses $\frac{1}{\sqrt{2\pi}}$, $\frac{1}{\sqrt{\pi}}\cos(nx)$, $\frac{1}{\sqrt{\pi}}\sin(nx)$.

The orthogonality relations can be verified by direct integration. For $m \neq n$:
$$\int_{-\pi}^\pi \cos(mx)\cos(nx)\,dx = \frac{1}{2}\int_{-\pi}^\pi [\cos((m-n)x) + \cos((m+n)x)]\,dx = 0,$$
since $\int_{-\pi}^\pi \cos(kx)\,dx = 0$ for any nonzero integer $k$.

## Completeness

An orthogonal set $\{f_n\}$ in a Hilbert space $H$ is **complete** (also called a **basis** or **total** set) if the only element of $H$ orthogonal to every $f_n$ is the zero element: $\langle h, f_n \rangle = 0$ for all $n$ implies $h = 0$.

**Equivalently**, $\{f_n\}$ is complete if and only if every $f \in H$ can be approximated arbitrarily well in norm by finite linear combinations of the $f_n$. That is, for any $\epsilon > 0$ there exists $N$ and coefficients $c_1, \ldots, c_N$ such that $\|f - \sum_{n=1}^N c_n f_n\| < \epsilon$.

**Theorem (Completeness of the Trigonometric System).** The trigonometric system is complete in $L^2([-\pi, \pi])$. That is, if $f \in L^2([-\pi, \pi])$ satisfies $\int_{-\pi}^\pi f(x)\cos(nx)\,dx = 0$ and $\int_{-\pi}^\pi f(x)\sin(nx)\,dx = 0$ for all $n \geq 0$, then $f = 0$ a.e.

This theorem is typically proved via the Weierstrass approximation theorem (continuous functions on $[-\pi, \pi]$ can be uniformly approximated by trigonometric polynomials) combined with a density argument. The completeness is what allows us to write $f = \sum c_n f_n$ (in $L^2$) rather than merely $f = \langle f, f_n\rangle / \|f_n\|^2 +$ some unexplained remainder.

## Classical Families of Orthogonal Polynomials

Different weight functions and intervals give rise to different orthogonal families. Here are the most important:

**Legendre polynomials** $P_n(x)$ are orthogonal on $[-1, 1]$ with weight $w \equiv 1$:
$$\int_{-1}^1 P_m(x) P_n(x)\,dx = \frac{2}{2n+1}\delta_{mn}.$$
They arise in solving Laplace's equation in spherical coordinates and in approximation theory.

**Chebyshev polynomials** $T_n(x) = \cos(n \arccos x)$ are orthogonal on $[-1,1]$ with weight $w(x) = 1/\sqrt{1-x^2}$:
$$\int_{-1}^1 T_m(x)T_n(x)\frac{dx}{\sqrt{1-x^2}} = \begin{cases} 0 & m \neq n \\ \pi/2 & m = n \geq 1 \\ \pi & m = n = 0. \end{cases}$$
They minimize the maximum error among all polynomials of given degree, which makes them essential in numerical approximation.

**Hermite polynomials** $H_n(x)$ are orthogonal on $(-\infty, \infty)$ with weight $w(x) = e^{-x^2}$:
$$\int_{-\infty}^\infty H_m(x) H_n(x) e^{-x^2}\,dx = \sqrt{\pi}\,2^n\,n!\,\delta_{mn}.$$
They appear in the quantum harmonic oscillator and in probability theory (the Hermite functions are eigenfunctions of the Fourier transform).

**Laguerre polynomials** $L_n(x)$ are orthogonal on $[0, \infty)$ with weight $w(x) = e^{-x}$. They arise in the radial part of the hydrogen atom wave functions.

## Sturm-Liouville Theory

The most systematic source of orthogonal function families is **Sturm-Liouville theory**. A Sturm-Liouville problem has the form
$$-(p(x)y')' + q(x)y = \lambda w(x) y, \quad x \in [a,b],$$
with boundary conditions at $x = a$ and $x = b$. Under appropriate hypotheses on $p, q, w$, the following hold:

1. The eigenvalues $\lambda_n$ are real and form an increasing sequence $\lambda_1 < \lambda_2 < \lambda_3 < \cdots$ with $\lambda_n \to \infty$.
2. The eigenfunctions $y_n$ corresponding to distinct eigenvalues are orthogonal with respect to the weight $w$: $\int_a^b y_m y_n\,w\,dx = 0$ for $m \neq n$.
3. The eigenfunctions are complete in $L^2([a,b], w)$.

The equation $-y'' = \lambda y$ on $[0, \pi]$ with $y(0) = y(\pi) = 0$ is a Sturm-Liouville problem. The eigenvalues are $\lambda_n = n^2$, the eigenfunctions are $y_n = \sin(nx)$, and completeness gives the sine series expansion. The trigonometric system on $[-\pi, \pi]$ with periodic boundary conditions is another instance.

## Using an Orthogonal Basis: The Expansion Theorem

**Theorem.** Let $\{f_n\}$ be a complete orthogonal set in $L^2([a,b], w)$ with $\|f_n\|_w \neq 0$. Then every $f \in L^2([a,b], w)$ satisfies
$$f = \sum_{n=0}^\infty c_n f_n \quad \text{in } L^2,$$
where the **generalized Fourier coefficients** are
$$c_n = \frac{\langle f, f_n \rangle_w}{\|f_n\|_w^2} = \frac{\int_a^b f(x) f_n(x) w(x)\,dx}{\int_a^b f_n(x)^2 w(x)\,dx}.$$
Convergence is in the $L^2$ sense: $\left\|f - \sum_{n=0}^N c_n f_n\right\|_w \to 0$ as $N \to \infty$.

**Worked Example.** Expand $f(x) = x$ on $[0, \pi]$ in Legendre polynomials shifted to $[0,\pi]$ — or more simply, expand $f(x) = x$ in eigenfunctions of $-y'' = \lambda y$ on $[0, \pi]$ with $y(0) = y(\pi) = 0$. The eigenfunctions are $\sin(nx)$ with $\|sin(nx)\|^2 = \pi/2$. So
$$c_n = \frac{2}{\pi}\int_0^\pi x\sin(nx)\,dx.$$
Integrate by parts: $\int_0^\pi x\sin(nx)\,dx = [-x\cos(nx)/n]_0^\pi + \int_0^\pi \cos(nx)/n\,dx = -\pi(-1)^n/n + [\sin(nx)/n^2]_0^\pi = (-1)^{n+1}\pi/n$.

So $c_n = \frac{2}{\pi} \cdot \frac{(-1)^{n+1}\pi}{n} = \frac{2(-1)^{n+1}}{n}$, giving the expansion
$$x = 2\sum_{n=1}^\infty \frac{(-1)^{n+1}}{n}\sin(nx), \quad x \in (0, \pi),$$
which is the sine series for $f(x) = x$. This converges in $L^2[0,\pi]$ and pointwise on $(0,\pi)$ by Dirichlet's theorem.
