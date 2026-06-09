# Limits and Continuity of Complex Functions

The concept of a limit of a complex function mirrors the real-variable definition but operates in a richer geometric setting. In $\mathbb{R}$, a limit requires agreement from two directions; in $\mathbb{C}$, the limit point can be approached along any path in the plane, from any direction, and by any manner of winding approach. This multidirectionality, far from being merely a technical complication, is the source of the extraordinary rigidity of complex-analytic functions: a function that possesses a complex limit at every point of a domain is already severely constrained.

## The Limit of a Complex Function

**Definition.** Let $f : D \to \mathbb{C}$ and let $z_0$ be an accumulation point of $D$. We say that $\lim_{z \to z_0} f(z) = L$ if for every $\varepsilon > 0$ there exists $\delta > 0$ such that
$$0 < |z - z_0| < \delta \implies |f(z) - L| < \varepsilon.$$

The definition is formally identical to the real-variable version, with the absolute value replaced by the modulus $|\cdot|$ on $\mathbb{C}$. However, the condition $|z - z_0| < \delta$ now describes a punctured disk in $\mathbb{C}$, and the conclusion $|f(z) - L| < \varepsilon$ must hold for all $z$ in that disk, regardless of the path by which $z$ approaches $z_0$.

## Reduction to Real Limits

**Theorem.** Let $f(z) = u(x,y) + iv(x,y)$ where $z = x + iy$, and let $z_0 = x_0 + iy_0$ and $L = a + ib$. Then $\lim_{z \to z_0} f(z) = L$ if and only if
$$\lim_{(x,y) \to (x_0, y_0)} u(x,y) = a \quad \text{and} \quad \lim_{(x,y) \to (x_0, y_0)} v(x,y) = b.$$

**Proof.** This follows from the inequalities
$$\max(|\mathrm{Re}(w)|, |\mathrm{Im}(w)|) \leq |w| \leq |\mathrm{Re}(w)| + |\mathrm{Im}(w)|,$$
which show that $|f(z) - L| < \varepsilon$ is equivalent to both $|u - a| < \varepsilon$ and $|v - b| < \varepsilon$ (up to the factor of 2). $\square$

This theorem reduces complex limits to pairs of real two-variable limits, allowing the full toolkit of real multivariable analysis to be applied.

## Nonexistence of Limits: Path-Dependence

The most common way to show that $\lim_{z \to z_0} f(z)$ does not exist is to exhibit two paths along which $f(z)$ approaches different values.

**Worked example.** Show that $\lim_{z \to 0} \dfrac{\bar{z}}{z}$ does not exist.

Along the real axis: $z = x \in \mathbb{R}$, $\bar{z} = x$, so $\bar{z}/z = 1$ for $x \neq 0$.

Along the imaginary axis: $z = iy$, $\bar{z} = -iy$, so $\bar{z}/z = -iy/(iy) = -1$.

Since the two paths give different limiting values ($1$ and $-1$), the limit does not exist. $\square$

This example is instructive: $f(z) = \bar{z}/z$ is formed from "simple" operations on $z$, yet it has no limit at the origin. The issue is that $\bar{z}$ is not complex-differentiable, which will be made precise in Unit 02 via the Cauchy-Riemann equations.

## Algebra of Limits

The standard limit laws hold for complex limits:

**Theorem.** Suppose $\lim_{z \to z_0} f(z) = L$ and $\lim_{z \to z_0} g(z) = M$. Then:
1. $\lim_{z \to z_0} (f + g)(z) = L + M$.
2. $\lim_{z \to z_0} (fg)(z) = LM$.
3. $\lim_{z \to z_0} (f/g)(z) = L/M$ provided $M \neq 0$.

These follow from the same proofs as in real analysis, using the triangle inequality and the multiplicativity of the modulus.

## Continuity

**Definition.** A function $f : D \to \mathbb{C}$ is continuous at $z_0 \in D$ if $\lim_{z \to z_0} f(z) = f(z_0)$.  $f$ is continuous on $D$ if it is continuous at every point of $D$.

By the reduction theorem, $f = u + iv$ is continuous at $z_0$ if and only if both $u$ and $v$ are continuous at $(x_0, y_0)$ as functions of two real variables.

**Examples of continuous functions:**
- Every polynomial $p(z) = a_n z^n + \cdots + a_0$ is continuous on $\mathbb{C}$.
- Every rational function $p(z)/q(z)$ is continuous wherever $q(z) \neq 0$.
- The functions $\mathrm{Re}(z)$, $\mathrm{Im}(z)$, $|z|$, and $\bar{z}$ are all continuous on $\mathbb{C}$.

## Uniform Continuity and Compactness

**Theorem.** If $f : K \to \mathbb{C}$ is continuous and $K \subseteq \mathbb{C}$ is compact (closed and bounded), then $f$ is uniformly continuous and bounded on $K$.

This is the complex version of the extreme value theorem. It is used frequently in complex analysis: estimates for contour integrals often require bounding $|f(z)|$ on a compact curve, and the fact that a continuous function achieves its maximum modulus on a compact set is exploited in the proof of the maximum modulus principle.

## Worked Example: Verifying a Limit

**Example.** Show that $\lim_{z \to i} (z^2 + 1)/(z - i) = 2i$.

Factor the numerator: $z^2 + 1 = (z - i)(z + i)$. For $z \neq i$:
$$\frac{z^2 + 1}{z - i} = z + i.$$
As $z \to i$, $z + i \to 2i$. So the limit is $2i$. $\square$

## Limits at Infinity and the Point at Infinity

It is useful to extend the complex plane by a point at infinity $\infty$, obtaining $\hat{\mathbb{C}} = \mathbb{C} \cup \{\infty\}$ (the Riemann sphere). We say $\lim_{z \to z_0} f(z) = \infty$ if for every $M > 0$ there exists $\delta > 0$ such that $0 < |z - z_0| < \delta \implies |f(z)| > M$. Similarly, $\lim_{z \to \infty} f(z) = L$ means $\lim_{|z| \to \infty} f(z) = L$.

For a polynomial of degree $n \geq 1$, $\lim_{z \to \infty} p(z) = \infty$, as expected. For a rational function $R(z) = p(z)/q(z)$ with $\deg p = \deg q$, $\lim_{z \to \infty} R(z) = $ (leading coefficient of $p$)/(leading coefficient of $q$).

## Importance for Differentiation

The concept of a limit is of course the foundation of differentiation. The complex derivative $f'(z_0)$ is defined as $\lim_{h \to 0} \frac{f(z_0 + h) - f(z_0)}{h}$, where $h \in \mathbb{C} \setminus \{0\}$ and the limit is taken in $\mathbb{C}$. The fact that $h$ can approach $0$ along any path in the plane is what makes complex differentiability so much stronger than real differentiability, and it is what forces the Cauchy-Riemann equations, the central topic of Unit 02.
