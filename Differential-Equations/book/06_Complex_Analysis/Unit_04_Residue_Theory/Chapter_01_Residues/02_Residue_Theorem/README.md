# The Residue Theorem

The residue theorem is the central computational theorem of complex analysis. It converts the problem of evaluating a contour integral — which might seem to require detailed knowledge of $f$ everywhere on and inside the contour — into a purely algebraic problem: identify the singularities inside the contour, compute their residues, and sum. The theorem subsumes Cauchy's theorem (no singularities, sum is $0$) and the Cauchy integral formula ($f(z)/(z-z_0)$ has exactly one singularity) as special cases.

## Statement

**Theorem (Residue Theorem).** Let $f$ be analytic on and inside a simple closed positively oriented contour $C$, except at finitely many isolated singularities $z_1, z_2, \ldots, z_N$ in the interior of $C$. Then:
$$\oint_C f(z)\, dz = 2\pi i\sum_{k=1}^N \mathrm{Res}(f; z_k).$$

## Proof

For each singularity $z_k$, choose a circle $C_k$ of small radius $\varepsilon_k$ centered at $z_k$, with all $C_k$ disjoint and lying inside $C$. The function $f$ is analytic on the region $D$ bounded by $C$ and the circles $C_k$. By Cauchy's theorem applied to $D$ (a multiply connected region, handled by introducing cuts connecting $C$ to each $C_k$):
$$\oint_C f\, dz - \sum_{k=1}^N \oint_{C_k} f\, dz = 0.$$

(The $C_k$ here are traversed counterclockwise; the minus sign reflects the orientation convention when they serve as inner boundary components.) Therefore:
$$\oint_C f\, dz = \sum_{k=1}^N \oint_{C_k} f\, dz.$$

For each $k$, use the Laurent expansion of $f$ near $z_k$:
$$f(z) = \sum_{n=-\infty}^\infty a_n^{(k)}(z - z_k)^n.$$
Integrating term by term over $C_k$ (justified by uniform convergence):
$$\oint_{C_k} f\, dz = \sum_{n=-\infty}^\infty a_n^{(k)} \oint_{C_k} (z-z_k)^n\, dz = a_{-1}^{(k)} \cdot 2\pi i = 2\pi i\,\mathrm{Res}(f; z_k),$$
since $\oint_{C_k}(z-z_k)^n\, dz = 2\pi i \cdot \mathbf{1}_{n = -1}$. Summing over $k$ gives the result. $\square$

## Connection to Cauchy's Theorem and Integral Formula

- **Cauchy's theorem:** if $f$ is analytic on and inside $C$, there are no singularities and the sum of residues is $0$, giving $\oint_C f\, dz = 0$.
- **Cauchy integral formula:** $\oint_C \frac{f(z)}{z-z_0}\, dz$ where $f$ is analytic and $z_0$ is inside $C$. The function $g(z) = f(z)/(z-z_0)$ has a simple pole at $z_0$ with $\mathrm{Res}(g; z_0) = f(z_0)$. The residue theorem gives $\oint g\, dz = 2\pi i f(z_0)$, recovering the Cauchy formula.

## Worked Examples

**Example 1.** Evaluate $\displaystyle\oint_{|z|=3} \frac{5z - 2}{z(z-1)}\, dz$.

Singularities inside $|z| = 3$: $z = 0$ and $z = 1$.

Partial fractions: $\frac{5z-2}{z(z-1)} = \frac{2}{z} + \frac{3}{z-1}$.

$\mathrm{Res}$ at $0 = 2$. $\mathrm{Res}$ at $1 = 3$.

Integral $= 2\pi i(2 + 3) = 10\pi i$. $\square$

**Example 2.** Evaluate $\displaystyle\oint_{|z|=2} \frac{e^{iz}}{z^2(z^2+9)}\, dz$.

Inside $|z| = 2$: singularities at $z = 0$ (pole of order $2$) and $z = \pm 3i$ (both outside since $|3i| = 3 > 2$).

Only the residue at $z = 0$ contributes. Let $g(z) = z^2 f(z) = e^{iz}/(z^2+9)$:
$$\mathrm{Res}(f;0) = g'(0) = \frac{d}{dz}\frac{e^{iz}}{z^2+9}\bigg|_{z=0} = \frac{ie^{iz}(z^2+9) - 2ze^{iz}}{(z^2+9)^2}\bigg|_{z=0} = \frac{9i}{81} = \frac{i}{9}.$$

Integral $= 2\pi i \cdot \frac{i}{9} = -\frac{2\pi}{9}$. $\square$

**Example 3.** Evaluate $\displaystyle\oint_{|z|=2}\frac{z^2}{(z-1)^2(z+1)}\, dz$.

Singularities inside $|z| = 2$: $z = 1$ (order $2$) and $z = -1$ (order $1$).

At $z = -1$ (simple pole): $\mathrm{Res} = \frac{z^2}{(z-1)^2}\big|_{z=-1} = \frac{1}{4}$.

At $z = 1$ (order $2$): $(z-1)^2 f(z) = z^2/(z+1)$. Derivative: $\frac{d}{dz}\frac{z^2}{z+1}\big|_{z=1} = \frac{2z(z+1) - z^2}{(z+1)^2}\big|_{z=1} = \frac{2+2-1}{4} = \frac{3}{4}$.

Integral $= 2\pi i\left(\frac{1}{4} + \frac{3}{4}\right) = 2\pi i$. $\square$

## The General Residue Theorem for Multiply Connected Regions

If $f$ is analytic on a region bounded by an outer contour $C$ (counterclockwise) and inner contours $C_1, \ldots, C_m$ (clockwise) — with no singularities on any boundary — and has isolated singularities $z_1, \ldots, z_N$ in the interior:
$$\oint_C f\, dz - \sum_{j=1}^m \oint_{C_j} f\, dz = 2\pi i\sum_{k=1}^N \mathrm{Res}(f; z_k).$$

## The Residue at Infinity

For functions that are analytic outside a large disk, the residue at infinity is defined as:
$$\mathrm{Res}(f; \infty) = -\frac{1}{2\pi i}\oint_{|z|=R} f(z)\, dz = -\mathrm{Res}\!\left(\frac{1}{z^2}f(1/z); 0\right).$$

**Theorem.** For a function analytic except at finitely many isolated singularities in $\hat{\mathbb{C}}$ (including possibly $\infty$):
$$\sum_{z_k \in \mathbb{C}} \mathrm{Res}(f; z_k) + \mathrm{Res}(f; \infty) = 0.$$

This is useful when it is easier to compute the residue at infinity than to sum all finite residues.
