# Power Series and Radius of Convergence

A power series is an infinite polynomial: a series of the form
$$\sum_{k=0}^\infty c_k (x - a)^k = c_0 + c_1(x-a) + c_2(x-a)^2 + \cdots,$$
where $a$ is the **center** and $c_k$ are the **coefficients**. For each fixed $x$, this is a series of real numbers, which may or may not converge. The central fact about power series is that the set of $x$ for which it converges is always an interval (possibly degenerate) centered at $a$, and within that interval the series defines a function with remarkable regularity.

## The Radius of Convergence

**Theorem.** For any power series $\sum c_k (x-a)^k$, there exists $R \in [0, \infty]$ — the **radius of convergence** — such that:
- The series converges absolutely for all $x$ with $|x - a| < R$.
- The series diverges for all $x$ with $|x - a| > R$.
- At $|x - a| = R$ (the endpoints), convergence must be checked separately.

The value $R$ is given by the **Cauchy-Hadamard formula**:
$$\frac{1}{R} = \limsup_{k\to\infty} |c_k|^{1/k},$$
with the conventions $1/0 = \infty$ and $1/\infty = 0$.

*Proof sketch.* Apply the root test: $|c_k (x-a)^k|^{1/k} = |c_k|^{1/k} \cdot |x-a|$. The $\limsup$ of this is $|x-a|/R$. The root test gives absolute convergence when $|x-a|/R < 1$ and divergence when $|x-a|/R > 1$. $\square$

**Ratio test version.** If $\lim_{k\to\infty} |c_{k+1}/c_k| = L$ exists, then $R = 1/L$.

**Example.** The geometric series $\sum_{k=0}^\infty x^k$ has $c_k = 1$, so $R = 1/(1)^{1/k} = 1$. It converges to $1/(1-x)$ for $|x| < 1$.

**Example.** The exponential series $\sum_{k=0}^\infty \frac{x^k}{k!}$ has $c_k = 1/k!$. Ratio: $|c_{k+1}/c_k| = 1/(k+1) \to 0$, so $R = \infty$. The series converges for all $x \in \mathbb{R}$.

**Example.** $\sum_{k=0}^\infty k!\, x^k$ has $|c_{k+1}/c_k| = k+1 \to \infty$, so $R = 0$. The series converges only at $x = 0$.

## Analyticity: Differentiation and Integration

Inside the interval of convergence, power series behave like polynomials — they can be differentiated and integrated term by term.

**Theorem.** If $f(x) = \sum_{k=0}^\infty c_k (x-a)^k$ converges for $|x-a| < R$, then $f$ is infinitely differentiable on $(a-R, a+R)$, and:
$$f'(x) = \sum_{k=1}^\infty k c_k (x-a)^{k-1}, \qquad \int_a^x f(t)\,dt = \sum_{k=0}^\infty \frac{c_k}{k+1}(x-a)^{k+1},$$
each with the same radius of convergence $R$.

*Proof idea.* One shows that the series of derivatives converges uniformly on every compact subset $[a-r, a+r]$ with $r < R$ — a consequence of the Weierstrass M-test applied with $M_k = |c_k| r^k$, and $\sum M_k < \infty$ by the definition of $R$. Uniform convergence allows the interchange of differentiation and summation. $\square$

**Corollary.** The coefficients $c_k$ are determined by the function values:
$$c_k = \frac{f^{(k)}(a)}{k!}.$$
This is the Taylor series formula. Power series are Taylor series.

## Examples of Power Series

**Sine and Cosine:**
$$\sin x = \sum_{k=0}^\infty \frac{(-1)^k x^{2k+1}}{(2k+1)!}, \qquad \cos x = \sum_{k=0}^\infty \frac{(-1)^k x^{2k}}{(2k)!}, \quad R = \infty.$$

**Logarithm:**
$$\ln(1+x) = \sum_{k=1}^\infty \frac{(-1)^{k+1} x^k}{k}, \quad R = 1.$$
At $x = 1$: this gives the conditionally convergent alternating harmonic series, summing to $\ln 2$. At $x = -1$: the series diverges.

**Binomial Series:** For $\alpha \in \mathbb{R}$,
$$(1+x)^\alpha = \sum_{k=0}^\infty \binom{\alpha}{k} x^k, \quad R = 1,$$
where $\binom{\alpha}{k} = \frac{\alpha(\alpha-1)\cdots(\alpha-k+1)}{k!}$.

## Power Series Solutions to ODEs

The fundamental application: consider the ODE
$$y'' + P(x)y' + Q(x)y = 0$$
where $P$ and $Q$ have power series expansions centered at an ordinary point $a$ with common radius $R_0$. Then every solution has a power series expansion $y = \sum_{k=0}^\infty c_k(x-a)^k$ converging for $|x-a| < R_0$.

**Example.** The equation $y'' - xy' - y = 0$ near $x = 0$. Substituting $y = \sum c_k x^k$:
$$\sum_{k=2}^\infty k(k-1)c_k x^{k-2} - x \sum_{k=1}^\infty k c_k x^{k-1} - \sum_{k=0}^\infty c_k x^k = 0.$$
Shifting indices and collecting powers of $x^k$:
$$(k+2)(k+1)c_{k+2} - k c_k - c_k = 0 \implies c_{k+2} = \frac{(k+1)c_k}{(k+2)(k+1)} = \frac{c_k}{k+2}.$$
This recurrence determines all coefficients from $c_0$ and $c_1$, giving a two-parameter family of solutions, as expected for a second-order linear ODE.

## Convergence at Endpoints

At the endpoints $x = a \pm R$, the power series may converge absolutely, converge conditionally, or diverge. This must be checked case by case.

**Abel's Theorem.** If $\sum_{k=0}^\infty c_k R^k$ converges to $S$, then $\lim_{x \to R^-} \sum_{k=0}^\infty c_k x^k = S$.

Abel's theorem allows one to evaluate sums of conditionally convergent series via the limit of an absolutely convergent power series. For instance, $\sum_{k=1}^\infty (-1)^{k+1}/k = \ln 2$ follows from evaluating the logarithm series at $x = 1$.

## Common Pitfalls

**Forgetting to check endpoints.** The radius gives the open interval of convergence; endpoints require separate work.

**Confusing radius with interval.** $R$ is the radius, not the interval. The interval of convergence is $(a-R, a+R)$ at minimum.

**Differentiating outside the interval.** Term-by-term differentiation is valid only inside the interval of convergence, where uniform convergence holds.
