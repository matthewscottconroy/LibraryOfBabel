# Section 7.6: Multiple Integrals

---

## Section Introduction

Integration in several variables extends the one-dimensional Riemann integral to functions of two or more variables. The double integral $\iint_D f(x,y)\,dA$ represents the volume under the surface $z = f(x,y)$ above the region $D$, or more generally the "total amount" of a quantity distributed with density $f$ over $D$. Triple integrals integrate over a 3D region. The definition — via Riemann sums with infinitesimal area/volume elements — generalizes the one-variable definition directly.

The key computational tool is **Fubini's theorem**: for a continuous function on a rectangle, $\iint_R f\,dA = \int_a^b\left(\int_c^d f(x,y)\,dy\right)dx$. The double integral equals iterated single integrals. This makes double and triple integrals computable in practice, reducing them to repeated applications of one-variable techniques. Fubini's theorem requires some regularity (the function must be integrable and the region must have a suitable structure), but these conditions are satisfied in virtually all applications.

**Change of variables** in multiple integrals generalizes substitution in one variable. If $\mathbf{x} = \mathbf{g}(\mathbf{u})$ is a smooth bijection with invertible Jacobian, then $\int_{\mathbf{g}(D)} f(\mathbf{x})\,d^n x = \int_D f(\mathbf{g}(\mathbf{u}))\,|\det J_{\mathbf{g}}|\,d^n u$ where $|\det J_{\mathbf{g}}|$ is the absolute value of the Jacobian determinant. This formula converts integrals in one coordinate system to integrals in another — a technique used constantly in physics when changing from Cartesian to polar, cylindrical, or spherical coordinates.

In GR, integrals over spacetime must be coordinate-independent. The volume element is not $d^4x$ (which depends on coordinates) but $\sqrt{-g}\,d^4x$ where $g = \det(g_{\mu\nu})$. The factor $\sqrt{-g}$ transforms in precisely the way needed to cancel the Jacobian from the coordinate change, making the integral a true scalar. This is why the Einstein-Hilbert action is $\int R\sqrt{-g}\,d^4x$.

---

## Subsections

- [7.6.1: The Double Integral](7.6.1-double-integral.md)
- [7.6.2: Iterated Integrals and Fubini's Theorem](7.6.2-fubini.md)
- [7.6.3: Triple Integrals](7.6.3-triple-integral.md)
- [7.6.4: Change of Variables and the Jacobian](7.6.4-change-of-variables.md)
- [7.6.5: Polar, Cylindrical, and Spherical Coordinates](7.6.5-coordinate-systems.md)
