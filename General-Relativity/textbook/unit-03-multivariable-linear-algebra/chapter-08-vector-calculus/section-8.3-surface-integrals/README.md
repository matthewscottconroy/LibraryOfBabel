# Section 8.3: Surface Integrals

---

## Section Introduction

A **surface integral** generalizes line integrals from curves to surfaces. Given a surface $S$ in $\mathbb{R}^3$ and a vector field $\mathbf{F}$, the surface integral $\iint_S \mathbf{F}\cdot d\mathbf{A}$ computes the **flux** of $\mathbf{F}$ through $S$: the total flow of the field across the surface, with direction. This is the natural concept for Gauss's law in electrostatics (total charge enclosed equals flux of $\mathbf{E}$ through any closed surface) and for the continuity equations of fluid mechanics.

To define the surface integral, one parametrizes the surface $S$ as $\mathbf{r}(u,v)$ for $(u,v)$ in some domain $D$ in the $uv$-plane. The **area element** is $d\mathbf{A} = (\partial\mathbf{r}/\partial u\times\partial\mathbf{r}/\partial v)\,du\,dv$ — the cross product of the tangent vectors gives an outward-pointing normal of magnitude equal to the local area distortion. The flux integral is then $\iint_S\mathbf{F}\cdot d\mathbf{A} = \iint_D \mathbf{F}(\mathbf{r}(u,v))\cdot(\partial_u\mathbf{r}\times\partial_v\mathbf{r})\,du\,dv$.

The parametrization introduces a **choice of orientation** — the normal vector can point in two directions, and the integral changes sign with a change of orientation. For a **closed surface** (one that encloses a region), the conventional orientation is outward-pointing. The **divergence theorem** (Gauss's theorem) then states that the flux through a closed surface equals the total divergence inside: $\oiint_{\partial V}\mathbf{F}\cdot d\mathbf{A} = \iiint_V\nabla\cdot\mathbf{F}\,dV$. This is one of the most important theorems in physics.

In GR, surface integrals appear in the definition of conserved quantities. The total energy and momentum of an isolated system are given by surface integrals of the gravitational "pseudo-tensor" at spatial infinity (the ADM mass). The second law of black hole thermodynamics ($\delta A\geq 0$) concerns the area of the event horizon, itself a two-dimensional surface.

---

## Subsections

- [8.3.1: Parametrized Surfaces and Area Elements](8.3.1-parametrized.md)
- [8.3.2: The Flux Integral](8.3.2-flux.md)
- [8.3.3: Orientation and Closed Surfaces](8.3.3-orientation.md)
- [8.3.4: The Divergence Theorem (Gauss's Theorem)](8.3.4-divergence-theorem.md)
- [8.3.5: Applications in Physics](8.3.5-applications.md)
