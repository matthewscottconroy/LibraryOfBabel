# Section 8.2: Line Integrals

---

## Section Introduction

A **line integral** (or **path integral**) integrates a function along a curve. There are two types: the line integral of a scalar field, $\int_C f\,ds$, which sums values of $f$ weighted by arc length; and the line integral of a vector field, $\int_C \mathbf{F}\cdot d\mathbf{r}$, which sums the component of $\mathbf{F}$ tangent to the curve. The second type is the more important for physics: it computes the **work** done by a force $\mathbf{F}$ along a path, the **circulation** of a velocity field around a closed loop, and the **flux** of an electric field through a surface (when combined with surface integrals).

The line integral $\int_C \mathbf{F}\cdot d\mathbf{r}$ depends, in general, on the path taken from start to end, not just on the endpoints. But for **conservative fields** $\mathbf{F} = \nabla\phi$, the integral depends only on the endpoints: $\int_C\nabla\phi\cdot d\mathbf{r} = \phi(\mathbf{b}) - \phi(\mathbf{a})$. This is the **fundamental theorem of calculus for line integrals**, and it means that work done by a conservative force (gravity, electrostatics) is path-independent — a fact of central importance in mechanics and thermodynamics.

The **circulation** of a vector field around a closed loop $C$ — the line integral $\oint_C\mathbf{F}\cdot d\mathbf{r}$ — is zero for conservative fields and nonzero for fields with nonzero curl. **Stokes' theorem** (Section 8.4) will quantify this: the circulation around a closed loop equals the flux of the curl through any surface bounded by the loop. This is the infinitesimal content of the "no work against a curl" fact.

In general relativity, line integrals appear as the definition of geodesic length and proper time: $\tau = \int\sqrt{-g_{\mu\nu}dx^\mu dx^\nu}/c$. The action principle for particle motion extremizes this integral over all paths connecting two events — the geodesic is the stationary path. The mathematical formalism of line integrals is the foundation on which this variational principle rests.

---

## Subsections

- [8.2.1: Line Integrals of Scalar Fields](8.2.1-scalar.md)
- [8.2.2: Line Integrals of Vector Fields and Work](8.2.2-work.md)
- [8.2.3: Path Independence and Conservative Fields](8.2.3-path-independence.md)
- [8.2.4: Fundamental Theorem for Line Integrals](8.2.4-ftc-line.md)
- [8.2.5: Circulation and Closed Loops](8.2.5-circulation.md)
