# Liouville's Theorem

Liouville's theorem is a striking illustration of the global rigidity of analytic functions: a function that is both analytic on all of $\mathbb{C}$ (entire) and bounded must be constant. In real analysis, this is wildly false — $\sin x$, $\cos x$, and many other nonconstant functions are bounded and infinitely differentiable on all of $\mathbb{R}$. The complex version holds because the Cauchy integral formula allows a bound on the function's values to propagate into a bound on its derivative, and the derivative bound can be made arbitrarily sharp by enlarging the contour.

## Statement and Proof

**Theorem (Liouville).** If $f$ is entire (analytic on all of $\mathbb{C}$) and bounded ($|f(z)| \leq M$ for all $z \in \mathbb{C}$), then $f$ is constant.

**Proof.** Fix any two points $z_1, z_2 \in \mathbb{C}$. For any $R > \max(|z_1|, |z_2|)$, apply the Cauchy integral formula to express:
$$f(z_1) - f(z_2) = \frac{1}{2\pi i}\oint_{|z|=R}\left(\frac{1}{z-z_1} - \frac{1}{z-z_2}\right)f(z)\, dz = \frac{1}{2\pi i}\oint_{|z|=R} \frac{(z_2 - z_1)f(z)}{(z-z_1)(z-z_2)}\, dz.$$

On $|z| = R$ with $R > 2\max(|z_1|, |z_2|)$: $|z - z_k| \geq R - |z_k| \geq R/2$. So $|(z-z_1)(z-z_2)| \geq R^2/4$. By ML:
$$|f(z_1) - f(z_2)| \leq \frac{1}{2\pi} \cdot \frac{|z_2 - z_1| \cdot M}{R^2/4} \cdot 2\pi R = \frac{4M|z_2 - z_1|}{R}.$$

As $R \to \infty$, the right side $\to 0$. So $f(z_1) = f(z_2)$ for all $z_1, z_2 \in \mathbb{C}$, i.e., $f$ is constant. $\square$

**Alternative proof via $f'$.** By Cauchy's inequality applied to $f'$ on the disk $|z| \leq R$:
$$|f'(z_0)| \leq \frac{M}{R} \quad \text{for all } z_0 \in \mathbb{C}.$$
As $R \to \infty$, $|f'(z_0)| \leq 0$, so $f'(z_0) = 0$ for all $z_0$. An analytic function with identically zero derivative on a connected domain is constant. $\square$

## Extensions and Generalizations

**Polynomial growth implies polynomial.** If $f$ is entire and $|f(z)| \leq M|z|^n$ for all sufficiently large $|z|$, then $f$ is a polynomial of degree at most $n$.

**Proof.** By Cauchy's inequality for the $(n+1)$-th derivative: $|f^{(n+1)}(z_0)| \leq \frac{(n+1)! \cdot M R^n}{R^{n+1}} = \frac{(n+1)! M}{R} \to 0$. So $f^{(n+1)} \equiv 0$, which means $f$ is a polynomial of degree at most $n$. $\square$

This generalization immediately implies the Fundamental Theorem of Algebra (next section): if a nonconstant polynomial $p$ had no root, then $1/p$ would be entire and bounded, hence constant — a contradiction.

**Picard's Great Theorem (advanced).** An entire nonconstant function takes every complex value with at most one exception. The exceptional value can occur (e.g., $e^z \neq 0$), but there can be at most one such value. This is a vastly stronger result than Liouville's theorem.

## The Contrast with Real Analysis

The real analogue of Liouville's theorem is false: $f(x) = \sin x$ is infinitely differentiable, bounded by $1$, and nonconstant. The failure occurs because the real line has no "width" from which to derive integral estimates that improve as the domain grows. In $\mathbb{C}$, the two-dimensional character of the domain means that a contour of radius $R$ has length $2\pi R$, and the integral formula for the derivative involves a factor of $1/R^2$ in the denominator (area of the disk), which forces the derivative to zero as $R \to \infty$.

More precisely, in the Cauchy integral formula for $f'$:
$$f'(z_0) = \frac{1}{2\pi i}\oint_{|z-z_0|=R}\frac{f(z)}{(z-z_0)^2}\, dz,$$
the integrand has size $M/R^2$ and the contour has length $2\pi R$, giving $|f'(z_0)| \leq M/R \to 0$. The key is that the power of $R$ in the denominator exceeds the power in the numerator, which is a consequence of the quadratic nature of the area element in the plane. In real analysis, the corresponding formula would involve a first power in both numerator and denominator, giving no decay.

## Worked Applications

**Application 1.** Show that if $f$ is entire and $\mathrm{Re}(f(z)) \leq M$ for all $z$, then $f$ is constant.

Consider $g(z) = e^{f(z)}$. Then $g$ is entire and $|g(z)| = e^{\mathrm{Re}(f(z))} \leq e^M$. By Liouville, $g$ is constant, so $f$ is constant. $\square$

**Application 2.** Show that if $f$ is entire and $|f(z)| \geq 1$ for all $z$, then $f$ is constant.

$g = 1/f$ is entire (since $f$ never vanishes) and bounded ($|g| \leq 1$). By Liouville, $g$ is constant, so $f$ is constant. $\square$

**Application 3.** Let $f$ be entire with $f(z + 1) = f(z)$ and $f(z + i) = f(z)$ for all $z$ (doubly periodic). Show $f$ is constant.

The fundamental domain $[0,1] \times [0,1]$ is compact, and $f$ restricted to it is continuous hence bounded. Since $f$ is doubly periodic, it is bounded on all of $\mathbb{C}$. By Liouville, $f$ is constant. $\square$

(This is the reason that nonconstant doubly periodic analytic functions — elliptic functions — must have poles in their fundamental domain.)
