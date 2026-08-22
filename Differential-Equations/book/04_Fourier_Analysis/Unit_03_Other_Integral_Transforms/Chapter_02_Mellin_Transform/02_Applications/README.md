# Applications of the Mellin Transform

The Mellin transform earns its place in the analyst's toolkit through a diverse range of applications: computing otherwise intractable definite integrals, studying the asymptotic behavior of generating functions and algorithms, and providing one of the most natural pathways into analytic number theory.

## Computing Definite Integrals

The Mellin transform method systematically evaluates integrals of the form $\int_0^\infty f(x)g(x)\,dx$ when both $\mathcal{M}[f]$ and $\mathcal{M}[g]$ are known.

**The Parseval formula for Mellin transforms.** If $F(s) = \mathcal{M}[f](s)$ and $G(s) = \mathcal{M}[g](s)$, and both transforms are defined in overlapping strips, then
$$\int_0^\infty f(x)g(x)\,dx = \frac{1}{2\pi i}\int_{c-i\infty}^{c+i\infty} F(s)G(1-s)\,ds.$$
This is evaluated by closing the contour and using the residue theorem.

**Example: Computing $\int_0^\infty \frac{\sin x}{x}\,dx$.** Use $f(x) = \sin x$ and $g(x) = 1/x$. The Mellin transform of $\sin x$ is $\mathcal{M}[\sin x](s) = \Gamma(s)\sin(\pi s/2)$ for $0 < \text{Re}(s) < 1$ (proved by contour integration or by expanding $\sin x = \text{Im}(e^{ix})$ and using the Gamma function formula). Then $G(1-s) = \mathcal{M}[1/x](1-s) = \mathcal{M}[x^{-1}](1-s)$... this requires care, since $x^{-1}$ is not Mellin-integrable without modification. A cleaner approach uses the known transform pair directly:

$$\int_0^\infty \frac{\sin x}{x}\,dx = \mathcal{M}\left[\frac{\sin x}{x}\right](1) = \Gamma(1)\sin(\pi/2)\cdot\frac{1}{1} \cdot \text{(regularized)} = \frac{\pi}{2}.$$

The standard approach is to write $\int_0^\infty \frac{\sin x}{x}\,dx = \lim_{s\to 0}\int_0^\infty x^{s-1}\sin x\,dx = \lim_{s\to 0}\Gamma(s)\sin(\pi s/2) = \lim_{s\to 0}\frac{\sin(\pi s/2)}{s}\cdot\frac{s}{\Gamma(1-s)\sin(\pi s)} \cdot \pi = \frac{\pi}{2}$.

**Example: The Beta function integral.** $B(a,b) = \int_0^\infty \frac{x^{a-1}}{(1+x)^{a+b}}\,dx = \frac{\Gamma(a)\Gamma(b)}{\Gamma(a+b)}$.

This follows by writing $1/(1+x)^{a+b} =$ a product and applying the Mellin convolution theorem, or by a direct substitution $x = t/(1-t)$ converting to the standard Beta function.

## Asymptotic Analysis of Algorithms

The Mellin transform is particularly effective for analyzing divide-and-conquer algorithms, where the recurrence $f(x) = \alpha f(x/\beta) + g(x)$ appears. Applying the Mellin transform:

If $F(s) = \mathcal{M}[f]$ and $G(s) = \mathcal{M}[g]$, then from $\mathcal{M}[f(x/\beta)](s) = \beta^s F(s)$:
$$F(s) = \alpha\beta^s F(s) + G(s) \implies F(s) = \frac{G(s)}{1 - \alpha\beta^s}.$$

The pole of $F(s)$ at $s = s_0 = \log_\beta(1/\alpha) = \ln(1/\alpha)/\ln\beta$ determines the leading behavior: $f(x) \sim Cx^{s_0}$ as $x \to \infty$ (by the inverse Mellin transform and the residue at $s_0$).

**Worked example: Merge sort.** The runtime satisfies $T(n) = 2T(n/2) + cn$ (for sorting $n$ elements). Here $\alpha = 2$, $\beta = 2$, $g(n) = cn$. The parameter $s_0 = \log_2(1/2) = -1$... but $s_0$ should satisfy $\alpha\beta^{s_0} = 1$, i.e., $2\cdot 2^{s_0} = 1$, so $s_0 = -1$. The pole of $1/(1-2\cdot 2^s)$ at $s = -1$ gives $T(n) \sim n^{-(-1)} = n^1$... times the logarithm from the pole. More carefully: near $s = -1$, $1 - 2\cdot 2^s = 1 - 2^{1+s} = -\ln(2)(s+1) + O((s+1)^2)$, so $F(s) \approx G(s)/(-(s+1)\ln 2)$. The pole of order 1 at $s = -1$ gives $T(n) \sim n \cdot \log n / \ln 2 = n\log_2 n$. This recovers the classic result $T(n) = O(n\log n)$ for merge sort.

## Relation to the Riemann Zeta Function

One of the most profound applications of the Mellin transform is in analytic number theory. The **Riemann zeta function** $\zeta(s) = \sum_{n=1}^\infty n^{-s}$ for $\text{Re}(s) > 1$ is related to the Mellin transform of the **Jacobi theta function** $\vartheta(x) = \sum_{n=-\infty}^\infty e^{-\pi n^2 x}$.

The Mellin transform of $\frac{1}{2}[\vartheta(x) - 1] = \sum_{n=1}^\infty e^{-\pi n^2 x}$ (just the positive-$n$ terms of the theta function, minus the constant) at $s$ gives:
$$\mathcal{M}\!\left[\sum_{n=1}^\infty e^{-\pi n^2 x}\right](s) = \sum_{n=1}^\infty \mathcal{M}[e^{-\pi n^2 x}](s) = \sum_{n=1}^\infty \frac{\Gamma(s)}{(\pi n^2)^s} = \frac{\Gamma(s)}{\pi^s}\zeta(2s).$$

The theta function satisfies the **modular transformation** $\vartheta(1/x) = x^{1/2}\vartheta(x)$. Using this symmetry in the Mellin transform computation, one derives the **functional equation** of the Riemann zeta function:
$$\pi^{-s/2}\Gamma(s/2)\zeta(s) = \pi^{-(1-s)/2}\Gamma((1-s)/2)\zeta(1-s).$$

This functional equation — connecting $\zeta(s)$ to $\zeta(1-s)$ — is the key to extending $\zeta$ analytically to all of $\mathbb{C}$ and to the study of the Riemann hypothesis.

## The Ramanujan-Mellin Formula

Another striking application: for $f$ with Mellin transform $F$ and $g$ with Mellin transform $G$, the identity
$$\int_0^\infty f(x)g(x)\,x^{s-1}\,dx = \frac{1}{2\pi i}\int_{c-i\infty}^{c+i\infty} F(w)G(s-w)\,dw$$
(Mellin-Parseval) is used to evaluate integrals involving products of special functions. For example, integrals involving $J_\nu(x)K_\mu(x)$ (products of Bessel functions of the first and second kind) have closed forms expressible in terms of Gamma functions, derived by this method.
