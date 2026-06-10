# Section 8.2: Line Integrals and Surface Integrals

---

## Section Introduction

The integral of a function over a region generalizes to the integral of a vector field along a curve (a **line integral**) or over a surface (a **surface integral**). These objects measure: how much work a force field does along a path; how much fluid flows through a surface per unit time; how much "circulation" a magnetic field has around a loop. They are the building blocks of Maxwell's equations, fluid mechanics, and the integral forms of conservation laws in GR.

---

## 8.2.1 Line Integrals

**Setup**: A smooth curve C in ℝ³, parameterized by **r**(t) = (x(t), y(t), z(t)) for t ∈ [a, b].

**Line integral of a scalar field** f along C (arc-length integral):

$$\int_C f \, ds = \int_a^b f(\mathbf{r}(t)) |\mathbf{r}'(t)| \, dt$$

This is the integral of f with respect to arc length — it gives the "mass" of a wire with density f.

**Line integral of a vector field** **F** along C (work integral):

$$\int_C \mathbf{F} \cdot d\mathbf{r} = \int_a^b \mathbf{F}(\mathbf{r}(t)) \cdot \mathbf{r}'(t) \, dt$$

This gives the work done by **F** in moving a particle along C.

**Independence of parameterization**: Both integrals are independent of the specific parameterization of C (the integral changes sign if the orientation is reversed, which is expected — work done against a force is negative).

**The Fundamental Theorem for Line Integrals**: If **F** = ∇φ (conservative), then:

$$\int_C \mathbf{F} \cdot d\mathbf{r} = \phi(\mathbf{r}(b)) - \phi(\mathbf{r}(a))$$

The integral depends only on the endpoints, not the path. This is the multivariable FTC.

*Proof*: ∫ᵢ₊ **F** · d**r** = ∫ₐᵇ ∇φ(**r**(t)) · **r**'(t) dt = ∫ₐᵇ d/dt[φ(**r**(t))] dt = φ(**r**(b)) − φ(**r**(a)). □

**When is a field conservative?**: In a simply connected domain (no holes), **F** is conservative iff ∇ × **F** = **0**. This is the integrability condition — the path-independence condition.

---

## 8.2.2 Surface Integrals

**Surface parameterization**: A smooth surface S parameterized by **r**(u, v) = (x(u,v), y(u,v), z(u,v)) for (u,v) ∈ D ⊂ ℝ².

**Normal vector**: The cross product **r**_u × **r**_v (where **r**_u = ∂**r**/∂u) is perpendicular to S with magnitude equal to the area element. The **surface area element** is dS = |**r**_u × **r**_v| du dv.

**Surface integral of a scalar field** f:

$$\iint_S f \, dS = \iint_D f(\mathbf{r}(u, v)) |\mathbf{r}_u \times \mathbf{r}_v| \, du \, dv$$

This gives the "total mass" of a shell with density f.

**Flux integral** (surface integral of a vector field):

$$\iint_S \mathbf{F} \cdot d\mathbf{S} = \iint_D \mathbf{F}(\mathbf{r}(u,v)) \cdot (\mathbf{r}_u \times \mathbf{r}_v) \, du \, dv$$

Here **dS** = (**r**_u × **r**_v) du dv is the **vector area element** — it points in the normal direction and has magnitude equal to the area element. The flux integral measures the rate at which **F** flows through S.

**Orientation**: The sign of the flux integral depends on the choice of normal direction (which side of the surface is "outward"). A surface is **orientable** if a consistent choice of normal can be made globally. The Möbius strip is the standard non-orientable surface — walking along it continuously, the normal flips. Non-orientable surfaces cannot be used as bounding surfaces for flux integrals. [For the non-orientability of the Möbius strip, see Munkres (1991), §27.]

---

## 8.2.3 Connection to Physics

**Work-energy theorem**: For a force **F** = m**a**, the work done along a trajectory is W = ∫ **F** · d**r** = ΔKE.

**Faraday's law** (integral form): $\oint_{\partial S} \mathbf{E} \cdot d\mathbf{l} = -\frac{d}{dt} \iint_S \mathbf{B} \cdot d\mathbf{S}$ — the EMF around a loop equals the rate of change of magnetic flux through any surface bounded by the loop. This connects line and surface integrals.

**Proper time in GR**: The proper time elapsed along a worldline x^μ(τ) in a spacetime with metric gᵤᵥ is:

$$\tau = \int \sqrt{-g_{\mu\nu} \frac{dx^\mu}{d\lambda} \frac{dx^\nu}{d\lambda}} \, d\lambda$$

This is a line integral — the "arc length" of the worldline in spacetime with the Lorentzian metric. For a massive particle, it is the time measured by a clock carried along that worldline. This is both the physical observable (what clocks measure) and the action whose extremization yields the geodesic equation.

---

## References

- Griffiths, D.J. (2017). *Introduction to Electrodynamics*, 4th ed. Cambridge University Press. [§1.3 on line, surface, and volume integrals in the context of electrostatics.]
- Marsden, J.E. and Tromba, A.J. (2012). *Vector Calculus*, 6th ed. W.H. Freeman. [Chapters 7–8 on vector integration.]
- Misner, C.W., Thorne, K.S., and Wheeler, J.A. (1973). *Gravitation*. W.H. Freeman. [§1.6 on proper time as a line integral in spacetime.]
