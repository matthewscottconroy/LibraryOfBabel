# Section 7.4: Multiple Integrals

---

## Section Introduction

The integral of a function over a region in ℝⁿ generalizes the single-variable integral in a natural way: instead of partitioning an interval into subintervals, we partition a region into small cells and form a Riemann sum. For n = 2, this gives the double integral — the volume under a surface. For n = 3, the triple integral — the total "mass" of a region with varying density. The key tools are Fubini's theorem (reducing multiple integrals to iterated single integrals) and change of variables (computing the integral in whatever coordinate system is most convenient).

---

## 7.4.1 Double Integrals

**Definition**: For f: ℝ² → ℝ bounded on a closed rectangle R = [a,b]×[c,d], partition R into subrectangles, form the Riemann sum, and take the limit. The **double integral** is:

$$\iint_R f(x, y) \, dA = \lim \sum_{i,j} f(x_i^*, y_j^*) \Delta x_i \Delta y_j$$

when this limit exists.

**Fubini's Theorem**: If f is continuous on the closed rectangle R = [a,b]×[c,d], then:

$$\iint_R f(x, y) \, dA = \int_a^b \left[\int_c^d f(x, y) \, dy\right] dx = \int_c^d \left[\int_a^b f(x, y) \, dx\right] dy$$

The double integral can be computed as an **iterated integral** — integrate over y first (with x held fixed), then integrate the result over x. The order can be reversed if f is continuous.

*Proof*: The result follows from applying the single-variable FTC twice. The key hypothesis (continuous f) ensures both iterated integrals are equal to the double integral.

**Integration over non-rectangular regions**: For a general region D ⊂ ℝ², write D = {(x,y): a ≤ x ≤ b, g₁(x) ≤ y ≤ g₂(x)} for continuous g₁, g₂. Then:

$$\iint_D f \, dA = \int_a^b \int_{g_1(x)}^{g_2(x)} f(x, y) \, dy \, dx$$

---

## 7.4.2 Change of Variables

The change-of-variables theorem for multiple integrals is the multivariable analogue of substitution:

**Theorem** (Change of Variables): Let T: ℝⁿ → ℝⁿ be a C¹ bijection from region U to region V, with T(**u**) = **x**. Then:

$$\int_V f(\mathbf{x}) \, d^n\mathbf{x} = \int_U f(T(\mathbf{u})) |\det(DT(\mathbf{u}))| \, d^n\mathbf{u}$$

The factor |det(DT)| is the **Jacobian determinant** — it measures how volumes are scaled by T.

**Intuition**: A small box of side lengths du¹ × ⋯ × duⁿ centered at **u** maps to a small parallelepiped of volume |det(DT)| du¹ ⋯ duⁿ at T(**u**). The integral must account for this volume scaling.

**Polar coordinates** (2D): x = r cos θ, y = r sin θ. Jacobian: det(∂(x,y)/∂(r,θ)) = r. So dx dy = r dr dθ.

$$\iint f(x,y) \, dx \, dy = \int_0^\infty \int_0^{2\pi} f(r\cos\theta, r\sin\theta) \, r \, dr \, d\theta$$

**Spherical coordinates** (3D): x = r sin φ cos θ, y = r sin φ sin θ, z = r cos φ. Jacobian determinant = r² sin φ. So dx dy dz = r² sin φ dr dθ dφ.

**In GR**: The volume element in curved spacetime is not d⁴x but √(−g) d⁴x, where g = det(gᵤᵥ) < 0. The factor √(−g) is the Jacobian determinant between curved and flat coordinates — it ensures the integral ∫ f √(−g) d⁴x is coordinate-invariant (a scalar). This is the GR version of change of variables, and it is essential for writing covariant integral formulas. [Misner, Thorne, Wheeler (1973), §4.4.]

---

## 7.4.3 Stokes' Theorem (Preview)

The fundamental theorem of calculus says: ∫ₐᵇ f'(x) dx = f(b) − f(a). The right side is an integral over the boundary {a, b} of the interval [a, b]. The generalization to multiple dimensions:

**Green's Theorem** (2D): For a smooth region D ⊂ ℝ² with boundary ∂D:

$$\iint_D \left(\frac{\partial Q}{\partial x} - \frac{\partial P}{\partial y}\right) dA = \oint_{\partial D} P \, dx + Q \, dy$$

**Divergence Theorem** (3D): For a smooth region V ⊂ ℝ³ with boundary ∂V:

$$\iiint_V (\nabla \cdot \mathbf{F}) \, dV = \oiint_{\partial V} \mathbf{F} \cdot d\mathbf{S}$$

**Stokes' Theorem** (surfaces): For a smooth surface S ⊂ ℝ³ with boundary ∂S:

$$\iint_S (\nabla \times \mathbf{F}) \cdot d\mathbf{S} = \oint_{\partial S} \mathbf{F} \cdot d\mathbf{l}$$

These three theorems are all special cases of a single generalized Stokes' theorem for differential forms:

$$\int_M d\omega = \int_{\partial M} \omega$$

where ω is a differential (n−1)-form on an n-dimensional manifold M with boundary ∂M, and d is the exterior derivative. This is developed in Chapter 28.

**Physical application**: The divergence theorem underlies Maxwell's equations in integral form: Gauss's law ∮ **E** · dA = Q_enc/ε₀ follows from the differential form ∇ · **E** = ρ/ε₀ by integrating over a volume and applying the divergence theorem. The GR analogue: the contracted Bianchi identity ∇_μ G^{μν} = 0 and the conservation law ∇_μ T^{μν} = 0 are differential laws; their integral forms (conservation of energy-momentum in a spacetime region) follow by applying the covariant divergence theorem.

---

## References

- Apostol, T.M. (1974). *Mathematical Analysis*, 2nd ed. Addison-Wesley. [Chapter 11 on multiple integrals and Stokes' theorem.]
- Misner, C.W., Thorne, K.S., and Wheeler, J.A. (1973). *Gravitation*. W.H. Freeman. [§4.4 on the invariant volume element √(−g) d⁴x; §3.5 on Stokes' theorem.]
- Spivak, M. (1965). *Calculus on Manifolds*. W.A. Benjamin. [Chapters 3–5: integration on manifolds and the generalized Stokes' theorem. The core of the book.]
- Rudin, W. (1976). *Principles of Mathematical Analysis*, 3rd ed. McGraw-Hill. [Chapter 10 on integration of differential forms.]
