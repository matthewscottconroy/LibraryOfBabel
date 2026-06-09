# Arc Length as a Line Integral

The arc length of a curve is the most fundamental scalar line integral: it is the special case in which the function being integrated is identically 1. Writing arc length as a line integral is not just a cosmetic reformulation — it places arc length firmly within the framework of integration, makes its properties (additivity, independence of parametrization) transparent, and enables extensions such as the computation of arc length in curved coordinate systems.

## The Formula

Let $C$ be a smooth curve with parametrization $\mathbf{r}: [a, b] \to \mathbb{R}^n$, where $\mathbf{r}'(t) \neq \mathbf{0}$ on $(a,b)$. The **arc length** of $C$ is

$$L(C) = \int_C ds = \int_a^b |\mathbf{r}'(t)|\,dt.$$

In two dimensions, with $\mathbf{r}(t) = (x(t), y(t))$:

$$L = \int_a^b \sqrt{\left(\frac{dx}{dt}\right)^2 + \left(\frac{dy}{dt}\right)^2}\,dt.$$

In three dimensions, with $\mathbf{r}(t) = (x(t), y(t), z(t))$:

$$L = \int_a^b \sqrt{\left(\frac{dx}{dt}\right)^2 + \left(\frac{dy}{dt}\right)^2 + \left(\frac{dz}{dt}\right)^2}\,dt.$$

For a curve $y = g(x)$ on $[a,b]$, the standard form is $L = \int_a^b \sqrt{1 + [g'(x)]^2}\,dx$.

## Derivation: Why $|\mathbf{r}'(t)|$?

A direct derivation illuminates why the formula takes this form. Subdivide $[a,b]$ into $n$ subintervals with partition points $a = t_0 < t_1 < \cdots < t_n = b$. The arc from $t_{k-1}$ to $t_k$ is approximated by the chord from $\mathbf{r}(t_{k-1})$ to $\mathbf{r}(t_k)$, of length

$$|\mathbf{r}(t_k) - \mathbf{r}(t_{k-1})| \approx |\mathbf{r}'(t_{k-1})| \cdot (t_k - t_{k-1})$$

by the mean value theorem (for small subdivisions). Summing over all chords and taking the limit as the partition is refined:

$$L = \lim_{n\to\infty} \sum_{k=1}^n |\mathbf{r}'(t_{k-1})|\,\Delta t_k = \int_a^b |\mathbf{r}'(t)|\,dt.$$

The approximation by chords becomes exact in the limit because a smooth curve is locally straight.

## Properties of Arc Length

**Additivity.** If $C$ is split at an interior point into $C_1$ and $C_2$, then $L(C) = L(C_1) + L(C_2)$.

**Invariance under reparametrization.** If $\mathbf{r}_1(t)$ and $\mathbf{r}_2(s)$ are two smooth parametrizations of the same curve related by $s = \phi(t)$ with $\phi' > 0$, then

$$\int_a^b |\mathbf{r}_1'(t)|\,dt = \int_c^d |\mathbf{r}_2'(s)|\,ds$$

by the substitution rule. This confirms that arc length is a geometric property of the curve, not of how it is parametrized.

**Non-negativity.** $L(C) \geq 0$ always, with equality only if $C$ is a single point.

## Worked Examples

**Example 1: Circle of radius $r$.** $\mathbf{r}(t) = (r\cos t, r\sin t)$, $t \in [0, 2\pi]$.

$|\mathbf{r}'(t)| = |(-r\sin t, r\cos t)| = r$.

$L = \int_0^{2\pi} r\,dt = 2\pi r$. Confirms the familiar formula.

**Example 2: Helix.** $\mathbf{r}(t) = (a\cos t, a\sin t, bt)$, $t \in [0, 2\pi]$, for constants $a, b > 0$.

$|\mathbf{r}'(t)| = |(-a\sin t, a\cos t, b)| = \sqrt{a^2 + b^2}$.

$L = \int_0^{2\pi}\sqrt{a^2+b^2}\,dt = 2\pi\sqrt{a^2+b^2}$.

Geometrically, if you unroll the cylinder of radius $a$ on which the helix lives, the helix straightens into a line segment of length $2\pi\sqrt{a^2+b^2}$.

**Example 3: Parabola.** $y = x^2/2$ on $[0,1]$.

$L = \int_0^1 \sqrt{1 + x^2}\,dx = \left[\frac{x\sqrt{1+x^2}}{2} + \frac{1}{2}\ln(x + \sqrt{1+x^2})\right]_0^1 = \frac{\sqrt{2}}{2} + \frac{\ln(1+\sqrt{2})}{2}$.

## The Arc Length Function

Define the arc length function from a fixed starting point $\mathbf{r}(a)$:

$$s(t) = \int_a^t |\mathbf{r}'(u)|\,du.$$

By the Fundamental Theorem of Calculus, $s'(t) = |\mathbf{r}'(t)|$. The function $s(t)$ measures how far along the curve one has traveled by time $t$. If $|\mathbf{r}'(t)| > 0$ everywhere, then $s$ is strictly increasing and invertible; the inverse $t(s)$ allows us to reparametrize the curve by arc length. In the arc length parametrization, $|\mathbf{r}'(s)| = 1$ (the curve is traversed at unit speed), which greatly simplifies many formulas in differential geometry.

## Connection to the General Scalar Line Integral

Any scalar line integral $\int_C f\,ds$ can be interpreted as a weighted arc length: $f(\mathbf{p})$ is the weight assigned to the portion of the curve near $\mathbf{p}$, and $ds$ is the infinitesimal arc length element. When $f \equiv 1$, all points are weighted equally and the integral reduces to total length. This connection shows that arc length is not a formula unto itself but an instance of a much more general construction.

## Summary

Arc length $L = \int_C ds = \int_a^b |\mathbf{r}'(t)|\,dt$ is the foundational scalar line integral. It measures the total length of a curve, independently of parametrization and direction. The arc length function $s(t)$ provides a natural reparametrization — by arc length — that simplifies the geometry of curves and underlies the theory of curvature and the Frenet-Serret formulas of differential geometry.
