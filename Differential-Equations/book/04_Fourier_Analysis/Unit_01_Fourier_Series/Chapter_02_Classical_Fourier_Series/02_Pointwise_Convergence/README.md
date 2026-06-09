# Pointwise Convergence of Fourier Series

Having defined the Fourier coefficients and written down the formal series $\frac{a_0}{2} + \sum_{n=1}^\infty [a_n\cos(nx) + b_n\sin(nx)]$, we face the central question of classical analysis: does this series converge, and if so, to what? The answer depends on where we look and what properties $f$ has near that point.

## The Dirichlet Kernel

The $N$-th partial sum of the Fourier series of $f$ is
$$S_N f(x) = \frac{a_0}{2} + \sum_{n=1}^N [a_n\cos(nx) + b_n\sin(nx)].$$
Substituting the integral formulas for $a_n$ and $b_n$ and interchanging sum and integral, we obtain
$$S_N f(x) = \frac{1}{\pi}\int_{-\pi}^\pi f(t)\left[\frac{1}{2} + \sum_{n=1}^N \cos(n(t-x))\right]dt = \frac{1}{\pi}\int_{-\pi}^\pi f(t)\,D_N(t-x)\,dt,$$
where the **Dirichlet kernel** is
$$D_N(u) = \frac{1}{2} + \sum_{n=1}^N \cos(nu).$$

Using the geometric series for complex exponentials, one derives the closed form
$$D_N(u) = \frac{\sin\!\left((N + \tfrac{1}{2})u\right)}{2\sin(u/2)}.$$
This follows from writing $\frac{1}{2} + \sum_{n=1}^N \cos(nu) = \text{Re}\!\left[\frac{1}{2} + \sum_{n=1}^N e^{inu}\right]$ and summing the geometric series.

The Dirichlet kernel satisfies $\frac{1}{\pi}\int_{-\pi}^\pi D_N(u)\,du = 1$ for all $N$ (since integrating the sum gives only the constant term). So
$$S_N f(x) - f(x) = \frac{1}{\pi}\int_{-\pi}^\pi [f(t) - f(x)]\,D_N(t-x)\,dt.$$
Convergence of $S_N f(x)$ to $f(x)$ is therefore equivalent to showing that the integral on the right goes to zero.

## Dirichlet's Theorem

**Theorem (Dirichlet, 1829).** Let $f$ be $2\pi$-periodic and piecewise smooth on $[-\pi, \pi]$: that is, $f$ is continuous except at finitely many points, and at every point $x_0 \in [-\pi, \pi]$ the one-sided limits $f(x_0^\pm)$ and one-sided derivatives
$$f'(x_0^\pm) = \lim_{h \to 0^\pm} \frac{f(x_0 + h) - f(x_0^\pm)}{h}$$
exist and are finite. Then for every $x$,
$$S_N f(x) \to \frac{f(x^+) + f(x^-)}{2} \quad \text{as } N \to \infty.$$
In particular, at a point of continuity, $S_N f(x) \to f(x)$.

**Proof sketch.** By the substitution $t = x + u$ and the even symmetry $D_N(-u) = D_N(u)$:
$$S_N f(x) = \frac{1}{\pi}\int_0^\pi \left[f(x+u) + f(x-u)\right] D_N(u)\,du.$$
Since $\frac{1}{\pi}\int_0^\pi D_N(u)\,du = \frac{1}{2}$, we have
$$S_N f(x) - \frac{f(x^+)+f(x^-)}{2} = \frac{1}{\pi}\int_0^\pi \left[\frac{f(x+u)+f(x-u)}{2} - \frac{f(x^+)+f(x^-)}{2}\right] \frac{\sin((N+\frac{1}{2})u)}{2\sin(u/2)}\,du.$$
Defining $g(u) = \frac{f(x+u)+f(x-u) - f(x^+) - f(x^-)}{2} \cdot \frac{u/2}{\sin(u/2)}$, the integrand becomes $g(u)\sin((N+\frac{1}{2})u)/u$, and a further manipulation writes this as an integral of $h(u)\sin((N+\frac{1}{2})u)$ where $h(u) = g(u)/u$. The piecewise smoothness of $f$ ensures $h$ is in $L^1[0,\pi]$, and the **Riemann-Lebesgue Lemma** — which states that $\int_a^b h(u)\sin(\lambda u)\,du \to 0$ as $\lambda \to \infty$ for any $h \in L^1$ — then gives the result.

## The Riemann-Lebesgue Lemma

**Lemma.** If $h \in L^1([a,b])$, then $\int_a^b h(u) e^{i\lambda u}\,du \to 0$ as $|\lambda| \to \infty$.

This says that rapidly oscillating integrands have cancelling positive and negative contributions, so the integral goes to zero. The proof for piecewise continuous $h$ follows from the observation that $\int_a^b \sin(\lambda u)\,du = O(1/\lambda)$, and a piecewise-constant approximation to $h$ reduces the general case to this.

## Worked Example: Square Wave Convergence

For the square wave $f(x) = \text{sgn}(x)$ on $(-\pi, \pi)$, Dirichlet's theorem predicts:
- At $x = \pi/4$ (point of continuity, $f(\pi/4) = 1$): $S_N f(\pi/4) \to 1$.
- At $x = 0$ (jump from $-1$ to $+1$): $S_N f(0) \to \frac{-1+1}{2} = 0$.
- At $x = \pi$ (where the periodic extension has a jump from $1$ to $-1$): $S_N f(\pi) \to \frac{1 + (-1)}{2} = 0$.

The Fourier series $\frac{4}{\pi}\sum_{k=0}^\infty \frac{\sin((2k+1)x)}{2k+1}$ indeed gives $0$ when evaluated at $x = 0$ or $x = \pi$, confirming the theorem. At $x = \pi/2$, the series gives $\frac{4}{\pi}(1 - \frac{1}{3} + \frac{1}{5} - \cdots) = \frac{4}{\pi}\cdot\frac{\pi}{4} = 1$.

## Conditions for Convergence: A Hierarchy

Dirichlet's theorem covers a broad class but not all $L^2$ functions. Here is a rough hierarchy:

1. **Piecewise smooth** (Dirichlet): pointwise convergence everywhere (to the midpoint of jumps).
2. **Lipschitz** ($|f(x) - f(y)| \leq C|x-y|^\alpha$, $\alpha > 0$): same conclusion, proved similarly.
3. **Merely continuous**: convergence is not guaranteed at every point. Continuous functions can have Fourier series diverging on a dense set (although pointwise divergence everywhere is impossible for continuous functions, by a theorem of Carleson and Hunt).
4. **$L^2$**: convergence in $L^2$ norm is always guaranteed (from completeness of the trigonometric system), but pointwise convergence may fail on a set of measure zero.

**Theorem (Carleson, 1966).** If $f \in L^2([-\pi,\pi])$, then the Fourier series of $f$ converges to $f(x)$ for almost every $x$. This theorem, which resolved a decades-old conjecture, is one of the deepest results in harmonic analysis. Its proof goes far beyond this course.

## The Localization Principle

An important corollary of the Dirichlet kernel approach is the **localization principle**: the convergence of the Fourier series of $f$ at a point $x$ depends only on the behavior of $f$ in an arbitrarily small neighborhood of $x$.

More precisely, if $f$ and $g$ agree on some interval $(x-\delta, x+\delta)$, then $S_N f(x) - S_N g(x) \to 0$. This follows because $S_N(f-g)(x) = \frac{1}{\pi}\int_{-\pi}^\pi (f(t)-g(t))D_N(t-x)\,dt$, and the integrand is supported away from zero where $D_N$ oscillates rapidly. The Riemann-Lebesgue lemma then gives the result.

The localization principle means that local pathologies (a jump at one point) do not affect convergence at other points, and that smoothness at a single point suffices for convergence there.
