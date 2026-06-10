# Section 12.1: Complex Functions and the Cauchy-Riemann Equations

---

## Section Introduction

A function f: ℂ → ℂ is a map from the complex plane to itself. Writing z = x + iy and f(z) = u(x, y) + iv(x, y), we have two real functions u (the real part) and v (the imaginary part). The question is: what does it mean for f to be "differentiable" in the complex sense?

The answer turns out to be far more restrictive than real differentiability. A complex-differentiable (analytic or holomorphic) function is not merely continuous or differentiable in the real sense — it satisfies the **Cauchy-Riemann equations**, which impose a rigid coupling between u and v. This rigidity has profound consequences.

---

## 12.1.1 Complex Differentiation

**Definition**: f: ℂ → ℂ is **complex differentiable** (holomorphic) at z₀ if the limit:

$$f'(z_0) = \lim_{h \to 0} \frac{f(z_0 + h) - f(z_0)}{h}$$

exists, where h ∈ ℂ and the limit is taken as |h| → 0 from any direction in ℂ.

The critical word is "any direction." The limit must be the same whether h approaches 0 along the real axis, the imaginary axis, or any other path.

**Cauchy-Riemann Equations**: f = u + iv is holomorphic at z₀ = x₀ + iy₀ iff:

$$\frac{\partial u}{\partial x} = \frac{\partial v}{\partial y}, \quad \frac{\partial u}{\partial y} = -\frac{\partial v}{\partial x}$$

at (x₀, y₀) (and the partial derivatives are continuous there).

*Derivation*: Along the real direction h = Δx: f'(z₀) = ∂u/∂x + i∂v/∂x. Along the imaginary direction h = iΔy: f'(z₀) = (1/i)(∂u/∂y + i∂v/∂y) = ∂v/∂y − i∂u/∂y. Equating real and imaginary parts gives the Cauchy-Riemann equations.

**Examples**:
- f(z) = z²: u = x² − y², v = 2xy. Check: ∂u/∂x = 2x = ∂v/∂y; ∂u/∂y = −2y = −∂v/∂x. ✓ Holomorphic.
- f(z) = z̄ = x − iy: u = x, v = −y. ∂u/∂x = 1 ≠ −1 = ∂v/∂y. Not holomorphic (except nowhere).
- f(z) = |z|²: u = x² + y², v = 0. Cauchy-Riemann: 2x = 0 and 2y = 0. Holomorphic only at z = 0.
- f(z) = eˢ: Using the Taylor series e^{x+iy} = eˣ(cos y + i sin y). u = eˣ cos y, v = eˣ sin y. Cauchy-Riemann: ∂u/∂x = eˣ cos y = ∂v/∂y ✓; ∂u/∂y = −eˣ sin y = −∂v/∂x ✓. Holomorphic everywhere.

---

## 12.1.2 Harmonic Functions

**Theorem**: If f = u + iv is holomorphic, then both u and v are **harmonic**: ∇²u = 0 and ∇²v = 0.

*Proof*: Differentiate the first Cauchy-Riemann equation with respect to x and the second with respect to y (assuming C²): ∂²u/∂x² = ∂²v/∂x∂y = ∂²v/∂y∂x = −∂²u/∂y². So ∂²u/∂x² + ∂²u/∂y² = 0. Similarly for v. □

**Significance**: The real and imaginary parts of any holomorphic function satisfy Laplace's equation. This means:
- Complex analysis is a powerful tool for 2D problems in electrostatics and fluid mechanics.
- Level curves of u (equipotentials) are orthogonal to level curves of v (field lines).
- The gravitational potential in 2D problems can be found by finding the real part of a holomorphic function.

**Connection to GR**: In the Newman-Penrose formalism and twistor theory, spacetime itself is treated as (part of) a complex manifold. The holomorphic functions on this space encode the geometry of the gravitational field. The Penrose transform maps solutions of massless free field equations to cohomology classes on twistor space — a purely holomorphic construction.

---

## 12.1.3 Conformal Maps

A holomorphic function with f'(z) ≠ 0 is a **conformal map** — it preserves angles. This follows from the fact that multiplication by f'(z₀) (the local linearization) is a rotation-and-scaling in ℂ.

**Riemann mapping theorem** (Riemann, 1851): Any simply connected open subset of ℂ (other than ℂ itself) is conformally equivalent to the open unit disk. This shows the extraordinary flexibility of conformal maps.

**Applications in GR**: Conformal transformations (rescaling the metric by a positive function, g → Ω²g) are used extensively in GR:
- Conformal compactification produces Penrose diagrams (infinite spacetime regions mapped to finite ones while preserving causal structure).
- Conformal invariance of massless wave equations.
- The conformal boundary of AdS space (in AdS/CFT) is the boundary "at infinity" in the conformal compactification.

---

## References

- Ahlfors, L.V. (1979). *Complex Analysis*, 3rd ed. McGraw-Hill. [The classic rigorous treatment; definitive.]
- Conway, J.B. (1978). *Functions of One Complex Variable*, 2nd ed. Springer. [Excellent standard graduate text.]
- Stein, E.M. and Shakarchi, R. (2003). *Complex Analysis*. Princeton University Press. [Volume 2 of the Princeton Lectures in Analysis; beautifully written with physical applications.]
