# The Schwarz-Christoffel Transformation

The Schwarz-Christoffel transformation provides an explicit formula for conformal maps from the upper half-plane (or the unit disk) to polygonal regions — interiors of polygons with straight sides. It is the primary tool for solving potential theory and fluid mechanics problems on domains whose boundaries are piecewise linear, and it applies to a wide range of practically important geometries: rectangles, triangles, semi-infinite strips, and L-shaped regions.

## Setup and Motivation

Let $\mathcal{P}$ be a polygon with vertices $w_1, w_2, \ldots, w_n$ (listed counterclockwise) and interior angles $\alpha_1\pi, \alpha_2\pi, \ldots, \alpha_n\pi$ at each vertex, where $0 < \alpha_k \leq 2$ (the exterior angle is $(1-\alpha_k)\pi$). The constraint $\sum_k (1 - \alpha_k) = 2$ (sum of exterior angles equals $2\pi$) gives $\sum_k \alpha_k = n - 2$.

We seek a conformal map $f : H \to \mathcal{P}$ from the upper half-plane $H = \{z : \mathrm{Im}(z) > 0\}$ to the interior of $\mathcal{P}$, extending continuously to a bijection $\bar{H} \to \bar{\mathcal{P}}$.

By the Riemann mapping theorem, such a map exists. Schwarz and Christoffel derived an explicit formula for it.

## The Schwarz-Christoffel Formula

**Theorem.** A conformal map from the upper half-plane to the interior of a polygon with vertices $w_1, \ldots, w_n$ and interior angles $\alpha_k\pi$ is given by:
$$f(z) = A + C\int_{z_0}^z \prod_{k=1}^{n-1} (w - x_k)^{\alpha_k - 1}\, dw,$$
where $x_1 < x_2 < \cdots < x_{n-1}$ are real preimage points (with $x_n = \infty$) and $A, C$ are complex constants.

The parameters $A$ (controlling position), $C$ (controlling scale and rotation), and the $x_k$ (controlling the shape) must be chosen to match the specified vertex positions $w_k = f(x_k)$. Three of the $x_k$ can be fixed freely (by the three-real-parameter freedom of Mobius transformations preserving $H$); the remaining $n - 3$ must be determined numerically.

## Derivation Sketch

Near a vertex preimage $x_k$ on the real axis, the map must create an interior angle $\alpha_k\pi$. The function $(z - x_k)^{\alpha_k}$ maps a neighborhood of $x_k$ in $H$ to a wedge of angle $\alpha_k\pi$ (the argument is multiplied by $\alpha_k$). The derivative of this map is $\alpha_k(z-x_k)^{\alpha_k-1}$. To make $f$ conformal everywhere in $H$ (interior) while achieving the correct angle at each boundary point $x_k$, the derivative $f'(z)$ must have the factor $(z-x_k)^{\alpha_k-1}$ at each $x_k$, giving:
$$f'(z) = C\prod_{k=1}^{n-1}(z-x_k)^{\alpha_k - 1}.$$

Integrating yields the formula.

## Worked Examples

**Example 1: The Upper Half-Plane to a Rectangle.**

A rectangle with vertices $w_1, w_2, w_3, w_4$ has all interior angles $\pi/2$ ($\alpha_k = 1/2$). The Schwarz-Christoffel formula gives:
$$f(z) = C\int_0^z \frac{dw}{\sqrt{(w-x_1)(w-x_2)(w-x_3)(w-x_4)}}.$$

By symmetry, choose $x_1 = -1/k$, $x_2 = -1$, $x_3 = 1$, $x_4 = 1/k$ for $k \in (0,1)$, or equivalently $\{-1/k, -1, 1, 1/k\}$:
$$f(z) = C\int_0^z \frac{dw}{\sqrt{(1-w^2)(1-k^2w^2)}},$$
the elliptic integral of the first kind $F(z, k)$. This maps $H$ to a rectangle whose aspect ratio depends on $k$ (the elliptic modulus). $\square$

**Example 2: The Upper Half-Plane to a Semi-Infinite Strip.**

The strip $\{w : 0 < \mathrm{Re}(w) < \pi, \mathrm{Im}(w) > 0\}$ is a polygon with vertices $w_1 = 0$, $w_2 = \pi$, and $w_3 = \infty$, with angles $\pi/2, \pi/2$ at the finite vertices and $0$ at infinity (a "vertex" at infinity with angle $0$ contributes $\alpha_3 - 1 = -1$).

Formula: $f'(z) = C(z-0)^{-1/2}(z-1)^{-1/2} \cdot (z-\infty)^{-1} \cdot z^{?}$... let me use the standard result: the map $w = \arcsin(z)$ (or equivalently $z = \sin w$) maps the strip $\{0 < \mathrm{Re}(w) < \pi/2\}$ to the upper half-plane. More precisely:
$$w = \int_0^z \frac{dw}{\sqrt{1-w^2}} = \arcsin z$$
maps $H$ (with the boundary interval $(-1,1)$ on the real axis) to the strip $\{0 < \mathrm{Re}(w) < \pi/2, \mathrm{Im}(w) > 0\}$. $\square$

**Example 3: Equilateral Triangle.**

An equilateral triangle has interior angles $\pi/3$ ($\alpha_k = 1/3$, $k = 1, 2, 3$). Choose preimages $x_1 = -1$, $x_2 = 1$, $x_3 = \infty$:
$$f(z) = C\int_0^z (w+1)^{-2/3}(w-1)^{-2/3}\, dw = C\int_0^z (w^2-1)^{-2/3}\, dw.$$
This can be expressed in terms of a hypergeometric function. $\square$

## The Disk Version

Replacing the upper half-plane by the unit disk $\mathbb{D}$, the Schwarz-Christoffel formula becomes:
$$f(z) = A + C\int_{z_0}^z \prod_{k=1}^n (1 - w/\zeta_k)^{\alpha_k - 1}\, dw,$$
where $\zeta_1, \ldots, \zeta_n$ are points on the unit circle $|\zeta_k| = 1$ (preimages of the vertices). The disk version is often more convenient when the domain has internal symmetry.

## Applications to Engineering

**Airfoil design.** The Joukowski transform $f(z) = z + c^2/z$ maps a circle to an airfoil-like profile. More complex airfoil shapes can be designed using iterated Schwarz-Christoffel-type maps.

**Electrical engineering.** The capacitance of capacitors with non-parallel plates, the inductance of waveguides with rectangular cross-sections, and the resistance of conducting sheets with corners are all computed using Schwarz-Christoffel maps.

**Heat conduction.** Temperature distributions in L-shaped or T-shaped domains (with constant-temperature or zero-flux boundary conditions on straight sides) are found by mapping to the upper half-plane with the Schwarz-Christoffel transformation and then using the known solution for the half-plane.

## Numerical Schwarz-Christoffel Mapping

For polygons with more than three finite vertices, the pre-vertex locations $x_k$ are not determined analytically and must be found numerically (the "parameter problem"). The Schwarz-Christoffel Toolbox (written by Driscoll and Trefethen) implements robust numerical algorithms for this purpose and is widely used in applied mathematics and engineering.
