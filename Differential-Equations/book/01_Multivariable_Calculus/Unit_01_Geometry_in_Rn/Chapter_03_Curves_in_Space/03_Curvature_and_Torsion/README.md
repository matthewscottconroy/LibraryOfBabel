# Curvature and Torsion

A straight line bends nowhere. A circle bends uniformly. A figure-eight bends more sharply in some places than others. The curvature of a curve is the precise measure of how rapidly the curve is changing direction. But in three dimensions, a curve can do something a plane curve cannot: it can twist out of any fixed plane. The torsion is the measure of this twisting. Together, curvature and torsion are the two intrinsic quantities that completely determine the shape of a smooth space curve, up to rigid motion.

## Curvature

Let $\mathbf{r}(s)$ be a unit-speed curve (parameterized by arc length, so $\|\mathbf{r}'(s)\| = 1$). The **unit tangent vector** is $\mathbf{T}(s) = \mathbf{r}'(s)$.

Since $\|\mathbf{T}(s)\| = 1$ is constant, the vector $\mathbf{T}'(s)$ is perpendicular to $\mathbf{T}(s)$ (by the constant-norm rule). The **curvature** is defined as the magnitude of the rate of change of the unit tangent:

$$\kappa(s) = \|\mathbf{T}'(s)\| = \|\mathbf{r}''(s)\|.$$

Geometrically, $\kappa$ measures how fast the tangent direction is turning. A straight line has $\mathbf{T}' = \mathbf{0}$ and $\kappa = 0$. A circle of radius $R$ has curvature $\kappa = 1/R$: small circles are highly curved (large $\kappa$) and large circles are nearly flat (small $\kappa$).

The **radius of curvature** is $R = 1/\kappa$, the radius of the **osculating circle** — the circle that best approximates the curve at a given point, lying in the plane of $\mathbf{T}$ and $\mathbf{T}'$.

## Curvature in a General Parameterization

Computing the arc length parameterization explicitly is often infeasible, so one needs a formula for curvature in terms of any regular parameterization $\mathbf{r}(t)$.

**Theorem.** For a regular curve $\mathbf{r}(t)$ in $\mathbb{R}^3$,

$$\kappa(t) = \frac{\|\mathbf{r}'(t)\times\mathbf{r}''(t)\|}{\|\mathbf{r}'(t)\|^3}.$$

**Derivation.** Let $v = \|\mathbf{r}'(t)\|$ be the speed. Then $\mathbf{T} = \mathbf{r}'/v$, so $\mathbf{r}' = v\mathbf{T}$. Differentiating: $\mathbf{r}'' = v'\mathbf{T} + v\mathbf{T}'$. Taking the cross product with $\mathbf{r}' = v\mathbf{T}$:

$$\mathbf{r}'\times\mathbf{r}'' = v\mathbf{T}\times(v'\mathbf{T} + v\mathbf{T}') = v\mathbf{T}\times v\mathbf{T}' = v^2(\mathbf{T}\times\mathbf{T}'),$$

since $\mathbf{T}\times\mathbf{T} = \mathbf{0}$. Thus $\|\mathbf{r}'\times\mathbf{r}''\| = v^2\|\mathbf{T}\times\mathbf{T}'\| = v^2\|\mathbf{T}'\|$ (using $\|\mathbf{T}\| = 1$ and $\mathbf{T}\perp\mathbf{T}'$). Since $\kappa = \|\mathbf{T}'\|/v$ (by the chain rule), we get $\|\mathbf{r}'\times\mathbf{r}''\| = v^3\kappa$, and the formula follows.

For a plane curve $\mathbf{r}(t) = (x(t), y(t))$, the cross product formula simplifies (using the $z$-component of the cross product): $\kappa = |x'y'' - y'x''|/(x'^2+y'^2)^{3/2}$.

## Worked Example: Curvature of the Helix

$\mathbf{r}(t) = (a\cos t, a\sin t, bt)$. Then $\mathbf{r}'(t) = (-a\sin t, a\cos t, b)$ and $\mathbf{r}''(t) = (-a\cos t, -a\sin t, 0)$.

$$\mathbf{r}'\times\mathbf{r}'' = \begin{vmatrix}\mathbf{i}&\mathbf{j}&\mathbf{k}\\-a\sin t&a\cos t&b\\-a\cos t&-a\sin t&0\end{vmatrix} = (0\cdot a\cos t - b(-a\sin t),\; b(-a\cos t)-0,\; (-a\sin t)(-a\sin t)-(a\cos t)(-a\cos t))$$

$$= (ab\sin t, -ab\cos t, a^2\sin^2 t + a^2\cos^2 t) = (ab\sin t, -ab\cos t, a^2).$$

$\|\mathbf{r}'\times\mathbf{r}''\| = \sqrt{a^2b^2\sin^2 t + a^2b^2\cos^2 t + a^4} = \sqrt{a^2b^2 + a^4} = a\sqrt{b^2+a^2}$.

$\|\mathbf{r}'\|^3 = (a^2+b^2)^{3/2}$.

$$\kappa = \frac{a\sqrt{a^2+b^2}}{(a^2+b^2)^{3/2}} = \frac{a}{a^2+b^2}.$$

The curvature is constant, as expected by symmetry. When $b = 0$ (a circle), $\kappa = 1/a$, confirming the formula for a circle.

## Torsion

While curvature measures bending in the osculating plane, **torsion** measures how much the curve twists out of that plane. A curve lies in a fixed plane if and only if its torsion is identically zero.

The **principal normal** $\mathbf{N}(s) = \mathbf{T}'(s)/\kappa(s)$ is the unit vector in the direction of $\mathbf{T}'(s)$, pointing toward the center of curvature. The **binormal** is $\mathbf{B}(s) = \mathbf{T}(s)\times\mathbf{N}(s)$, a unit vector perpendicular to the osculating plane.

The torsion is defined by $\mathbf{B}'(s) = -\tau(s)\mathbf{N}(s)$, or equivalently

$$\tau = -\mathbf{B}'\cdot\mathbf{N}.$$

The sign convention is chosen so that a right-handed helix has positive torsion.

**Theorem.** In a general parameterization,

$$\tau(t) = \frac{(\mathbf{r}'\times\mathbf{r}'')\cdot\mathbf{r}'''}{\|\mathbf{r}'\times\mathbf{r}''\|^2}.$$

This formula involves the **scalar triple product** of $\mathbf{r}'$, $\mathbf{r}''$, and $\mathbf{r}'''$ (the third derivative), and is the most practical formula for computation.

## Torsion of the Helix

Continuing the helix example: $\mathbf{r}'''(t) = (a\sin t, -a\cos t, 0)$.

From above, $\mathbf{r}'\times\mathbf{r}'' = (ab\sin t, -ab\cos t, a^2)$.

$(\mathbf{r}'\times\mathbf{r}'')\cdot\mathbf{r}''' = ab\sin t\cdot a\sin t + (-ab\cos t)(-a\cos t) + a^2\cdot 0 = a^2b\sin^2 t + a^2b\cos^2 t = a^2b$.

$\|\mathbf{r}'\times\mathbf{r}''\|^2 = a^2(a^2+b^2)$.

$$\tau = \frac{a^2b}{a^2(a^2+b^2)} = \frac{b}{a^2+b^2}.$$

The torsion is constant. A right-handed helix ($b > 0$) has positive torsion; a left-handed helix ($b < 0$) has negative torsion. A planar circle ($b = 0$) has zero torsion, as expected.

## The Fundamental Theorem of Curves

**Theorem.** Given continuous functions $\kappa(s) > 0$ and $\tau(s)$ on an interval $[0, L]$, there exists a smooth unit-speed curve $\mathbf{r}: [0, L] \to \mathbb{R}^3$ with curvature $\kappa$ and torsion $\tau$, and this curve is unique up to a rigid motion of $\mathbb{R}^3$ (rotation and translation).

This theorem, proved using the Frenet-Serret equations in the next section, says that curvature and torsion together completely determine the shape of a curve. It is the curve analogue of the fundamental theorem of ordinary differential equations — both assert that a system of equations has a unique solution given initial data.

## Common Pitfalls

Curvature is always non-negative ($\kappa \geq 0$), while torsion can be positive, negative, or zero. A common mistake is defining curvature as $\mathbf{r}''(s)$ rather than $\|\mathbf{r}''(s)\|$; the curvature is the magnitude of the second derivative, not the derivative itself.

The formula $\kappa = \|\mathbf{r}'\times\mathbf{r}''\|/\|\mathbf{r}'\|^3$ requires the cross product and is specific to $\mathbb{R}^3$. In $\mathbb{R}^2$, the analogous formula is $\kappa = |x'y''-y'x''|/(x'^2+y'^2)^{3/2}$.
