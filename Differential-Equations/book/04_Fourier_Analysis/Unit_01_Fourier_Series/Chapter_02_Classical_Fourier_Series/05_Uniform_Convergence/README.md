# Uniform Convergence of Fourier Series

Pointwise convergence of a sequence of functions $\{S_N f\}$ to $f$ means that at each fixed $x$, the values converge. Uniform convergence is the stronger requirement that the convergence is equally fast across all $x$ simultaneously: for every $\epsilon > 0$ there exists $N_0$ such that $|S_N f(x) - f(x)| < \epsilon$ for all $N > N_0$ and all $x$ at once. The difference is crucial in practice: uniform convergence allows term-by-term integration and differentiation, and guarantees that the series can be evaluated by truncation with a controlled error.

## Why Pointwise Convergence Is Not Enough

Dirichlet's theorem gives pointwise convergence for piecewise smooth functions, but pointwise convergence alone does not allow passage of limits through integrals or derivatives. For example, a sequence can converge pointwise to one function but have integrals converging to a different value. Uniform convergence prevents this: if $S_N f \to f$ uniformly, then $\int S_N f \to \int f$ (by the standard $\epsilon$-argument), and if the derivatives $S_N' f$ converge uniformly to some $g$, then $f' = g$.

For Fourier series in particular, uniform convergence allows term-by-term differentiation: $\frac{d}{dx}\sum a_n\cos(nx) = -\sum na_n\sin(nx)$, provided this new series converges uniformly. This is essential for using Fourier series to solve differential equations.

## The Uniform Convergence Theorem

**Theorem.** Let $f : \mathbb{R} \to \mathbb{R}$ be $2\pi$-periodic. If $f$ is continuous on all of $\mathbb{R}$ and its derivative $f'$ is piecewise continuous on $[-\pi, \pi]$, then the Fourier series of $f$ converges to $f$ uniformly on $\mathbb{R}$.

**Proof.** Integration by parts on the Fourier coefficient formula gives a relationship between the coefficients of $f$ and those of $f'$. Let $a_n, b_n$ be the Fourier coefficients of $f$ and $\alpha_n, \beta_n$ those of $f'$. Since $f$ is continuous and $2\pi$-periodic:
$$\alpha_n = \frac{1}{\pi}\int_{-\pi}^\pi f'(x)\cos(nx)\,dx = \frac{1}{\pi}\left[f(x)\cos(nx)\right]_{-\pi}^\pi + \frac{n}{\pi}\int_{-\pi}^\pi f(x)\sin(nx)\,dx = n b_n.$$
Similarly $\beta_n = -n a_n$ (the boundary terms vanish because $f$ is continuous and periodic). So $|b_n| = |\alpha_n|/n$ and $|a_n| = |\beta_n|/n$.

Applying Bessel's inequality to $f'$ (which is in $L^2$):
$$\sum_{n=1}^\infty (\alpha_n^2 + \beta_n^2) \leq \|f'\|^2 < \infty.$$
By the Cauchy-Schwarz inequality:
$$\sum_{n=1}^\infty (|a_n| + |b_n|) = \sum_{n=1}^\infty \frac{|\beta_n| + |\alpha_n|}{n} \leq \left(\sum_{n=1}^\infty \frac{1}{n^2}\right)^{1/2}\left(\sum_{n=1}^\infty (\alpha_n^2 + \beta_n^2)\right)^{1/2} < \infty.$$
Since $\sum(|a_n| + |b_n|) < \infty$ and $|a_n\cos(nx) + b_n\sin(nx)| \leq |a_n| + |b_n|$, the Weierstrass M-test guarantees that the Fourier series converges absolutely and uniformly. By Dirichlet's theorem (applicable since $f$ is continuous), the pointwise limit is $f(x)$. Combined with uniform convergence, the series converges uniformly to $f$.

## Decay Rate of Coefficients and Smoothness

The argument above shows that if $f'$ exists and is piecewise continuous, the Fourier coefficients satisfy $|a_n|, |b_n| = O(1/n)$. More generally, $k$ continuous derivatives imply $|a_n|, |b_n| = O(1/n^k)$, and the decay rate determines both the smoothness and the quality of truncated series approximations.

| Smoothness of $f$ | Decay of $a_n, b_n$ | Convergence |
|---|---|---|
| Piecewise smooth, discontinuous | $O(1/n)$ | Pointwise (Gibbs near jumps) |
| Continuous, $f'$ piecewise continuous | $O(1/n^2)$ at least | Uniform |
| $C^k$ ($k$ cont. derivatives) | $O(1/n^k)$ | Uniform, Gibbs-free |
| $C^\infty$ | Faster than any power | Uniform, exponentially fast |
| Real analytic | Exponential: $O(r^n)$ for $r < 1$ | Uniform, exponential rate |

The last two rows reflect the fact that for infinitely differentiable or analytic functions, integrating by parts repeatedly shows the coefficients decay faster than any polynomial, or exponentially, respectively.

## Term-by-Term Differentiation and Integration

**Differentiation.** If $f$ satisfies the conditions of the uniform convergence theorem, the Fourier series of $f'$ is obtained by differentiating term by term:
$$f'(x) = \sum_{n=1}^\infty \left[-na_n\sin(nx) + nb_n\cos(nx)\right].$$
However, the differentiated series has coefficients of size $n \cdot O(1/n^2) = O(1/n)$, which is still summable; so $f'$ has a uniformly convergent Fourier series. In general, differentiating $k$ times and maintaining uniform convergence requires the coefficients to decay fast enough, which is guaranteed by sufficient smoothness.

**Integration.** Term-by-term integration is easier: the integrated series $\int_{-\pi}^x S_N f(t)\,dt$ converges uniformly to $\int_{-\pi}^x f(t)\,dt$ under much weaker conditions — it suffices for $f \in L^1$. The integrated series has coefficients $a_n/n$ and $b_n/n$, which decay faster than those of $f$, improving convergence.

## Uniform Approximation by Trigonometric Polynomials

A related but distinct result is the **Weierstrass approximation theorem for trigonometric polynomials**:

**Theorem.** If $f$ is continuous and $2\pi$-periodic, then for every $\epsilon > 0$ there exists a trigonometric polynomial $T(x) = \frac{A_0}{2} + \sum_{n=1}^N [A_n\cos(nx) + B_n\sin(nx)]$ (for some $N$) such that $|f(x) - T(x)| < \epsilon$ for all $x$.

The Fourier partial sums are specific trigonometric polynomials (using the Fourier coefficients of $f$), but the Weierstrass theorem does not assert that these particular polynomials converge uniformly — just that some trigonometric polynomial of each degree gives a good approximation. Indeed, the Fejér (Cesaro) means of the Fourier series do converge uniformly to $f$ for every continuous periodic $f$.

## Worked Example: Uniform Convergence for $f(x) = x^2 - \pi^2/3$

This function is $2\pi$-periodic (since both endpoints have the same value and derivative vanishes at $\pm\pi$ if we think of it periodically), continuous, with $f' = 2x$ piecewise continuous. Wait — $f'(-\pi) = -2\pi \neq 2\pi = f'(\pi)$, so the derivative has a jump at $\pm\pi$. The second derivative $f'' = 2$ is constant, so continuous.

We computed earlier that $|x| = \frac{\pi}{2} - \frac{4}{\pi}\sum_{k=1}^\infty \frac{\cos((2k-1)x)}{(2k-1)^2}$. Differentiating term by term (valid because the differentiated series converges uniformly for $x \neq \pm\pi$ by the ratio $|n \cdot a_n| = |n \cdot 4/(n^2\pi)| = 4/(n\pi) \to 0$):

For $f(x) = x^2$ on $(-\pi, \pi)$, the Fourier series is $\frac{\pi^2}{3} + 4\sum_{n=1}^\infty \frac{(-1)^n}{n^2}\cos(nx)$, which has coefficients decaying as $1/n^2$. The M-test gives absolute and uniform convergence. This is consistent with $f(x) = x^2$ being continuous with piecewise continuous derivative $f'(x) = 2x$.
