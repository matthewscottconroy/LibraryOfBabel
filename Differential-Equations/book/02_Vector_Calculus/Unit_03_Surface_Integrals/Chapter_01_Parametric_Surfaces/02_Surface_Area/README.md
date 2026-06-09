# Surface Area

Surface area is the most natural scalar surface integral: the special case in which the function being integrated is the constant 1. Just as arc length integrates the speed $|\mathbf{r}'(t)|$ along a parametric curve, surface area integrates the area element $|\mathbf{r}_u\times\mathbf{r}_v|$ over the parameter domain. The result — the total area of the surface — is a geometric invariant, independent of how the surface is parametrized.

## The Formula

Let $S$ be a smooth parametric surface with parametrization $\mathbf{r}: D \to \mathbb{R}^3$, where $D$ is a bounded region in the $uv$-plane and $\mathbf{r}$ is $C^1$ on $D$ with $\mathbf{r}_u\times\mathbf{r}_v \neq \mathbf{0}$ except possibly on a set of measure zero. The **surface area** of $S$ is

$$A(S) = \iint_S dS = \iint_D |\mathbf{r}_u\times\mathbf{r}_v|\,du\,dv.$$

**Why this formula.** Subdivide $D$ into small rectangles $[u_i, u_i+\Delta u]\times[v_j, v_j+\Delta v]$. The image under $\mathbf{r}$ of such a rectangle is approximately the parallelogram spanned by $\mathbf{r}_u\,\Delta u$ and $\mathbf{r}_v\,\Delta v$, of area $|\mathbf{r}_u\times\mathbf{r}_v|\,\Delta u\,\Delta v$. Summing and passing to the limit gives the formula.

## Case 1: Graph Surfaces $z = g(x,y)$

Using the standard parametrization $\mathbf{r}(x,y) = (x, y, g(x,y))$:

$$A(S) = \iint_{D_{xy}} \sqrt{1 + \left(\frac{\partial g}{\partial x}\right)^2 + \left(\frac{\partial g}{\partial y}\right)^2}\,dx\,dy.$$

For a function $y = h(x)$ in the plane (a curve), the arc length is $\int\sqrt{1+(h')^2}\,dx$. The surface area formula is the natural two-dimensional analogue.

**Example 1.** Find the surface area of the paraboloid $z = x^2 + y^2$ over the disk $D: x^2+y^2 \leq 1$.

$g_x = 2x$, $g_y = 2y$, so $\sqrt{1 + 4x^2 + 4y^2} = \sqrt{1+4r^2}$ in polar coordinates.

$$A = \int_0^{2\pi}\int_0^1 \sqrt{1+4r^2}\,r\,dr\,d\theta = 2\pi\int_0^1 r\sqrt{1+4r^2}\,dr.$$

Substitute $w = 1+4r^2$, $dw = 8r\,dr$:

$$2\pi\int_1^5\sqrt{w}\,\frac{dw}{8} = \frac{\pi}{4}\cdot\frac{2}{3}w^{3/2}\Big|_1^5 = \frac{\pi}{6}(5^{3/2} - 1) = \frac{\pi}{6}(5\sqrt{5}-1).$$

## Case 2: Sphere of Radius $a$

Using the spherical parametrization $\mathbf{r}(\phi,\theta)$ with $dS = a^2\sin\phi\,d\phi\,d\theta$:

$$A = \int_0^{2\pi}\int_0^\pi a^2\sin\phi\,d\phi\,d\theta = 2\pi a^2\int_0^\pi\sin\phi\,d\phi = 2\pi a^2\cdot 2 = 4\pi a^2.$$

This recovers the classical formula for the surface area of a sphere.

**Partial spheres.** The area of the spherical cap $\phi \in [0, \phi_0]$ is $2\pi a^2(1 - \cos\phi_0)$. For $\phi_0 = \pi/2$ (a hemisphere), $A = 2\pi a^2$.

## Case 3: Cylinder

For the lateral surface of a cylinder of radius $a$ and height $h$: $dS = a\,d\theta\,dz$.

$$A = \int_0^{2\pi}\int_0^h a\,dz\,d\theta = 2\pi a h.$$

Again, this recovers the classical formula (circumference times height).

## Case 4: Cone

For the cone $z = \sqrt{x^2+y^2}$ over the disk of radius $R$: $dS = r\sqrt{2}\,dr\,d\theta$.

$$A = \int_0^{2\pi}\int_0^R r\sqrt{2}\,dr\,d\theta = 2\pi\sqrt{2}\cdot\frac{R^2}{2} = \pi R^2\sqrt{2}.$$

The slant height is $l = R\sqrt{2}$ (since the cone has slope 1), and $\pi R l = \pi R^2\sqrt{2}$ matches the formula for lateral surface area of a cone with apex half-angle $45°$.

## The Flat-Surface Special Case

When $g = 0$ (the $xy$-plane), $dS = \sqrt{1+0+0}\,dx\,dy = dx\,dy$, and $A(S) = \iint_D dx\,dy = \text{area of }D$. Surface area reduces to ordinary area for flat surfaces.

## Comparison with Arc Length

The analogy between arc length and surface area is tight:

| Curve | Surface |
|---|---|
| $\mathbf{r}(t) \in \mathbb{R}^3$ | $\mathbf{r}(u,v) \in \mathbb{R}^3$ |
| $\mathbf{r}'(t)$ tangent vector | $\mathbf{r}_u, \mathbf{r}_v$ tangent vectors |
| $|\mathbf{r}'(t)|$ speed | $|\mathbf{r}_u\times\mathbf{r}_v|$ area density |
| $ds = |\mathbf{r}'|\,dt$ | $dS = |\mathbf{r}_u\times\mathbf{r}_v|\,du\,dv$ |
| $L = \int|\mathbf{r}'|\,dt$ | $A = \iint|\mathbf{r}_u\times\mathbf{r}_v|\,du\,dv$ |

## Summary

Surface area is computed by integrating $|\mathbf{r}_u\times\mathbf{r}_v|$ over the parameter domain. For graph surfaces, the formula involves $\sqrt{1+g_x^2+g_y^2}$. For spheres, cylinders, and cones, standard parametrizations give elegant formulas that match classical results. The area element $dS = |\mathbf{r}_u\times\mathbf{r}_v|\,du\,dv$ is the building block for all scalar and vector surface integrals in the sections that follow.
