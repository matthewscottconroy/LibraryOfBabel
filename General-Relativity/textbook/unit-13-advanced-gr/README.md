# Unit XIII: Advanced Formulations of GR

---

## Unit Introduction

Having developed GR in its standard 4D covariant form, this unit explores alternative formulations that reveal additional structure and enable powerful applications.

**The ADM formalism** (Arnowitt-Deser-Misner, 1959) decomposes spacetime into a foliation by spacelike hypersurfaces, giving GR a Hamiltonian formulation. The 4D metric splits into a 3-metric on spatial slices plus a lapse function and shift vector encoding how the slicing evolves. This formulation is the basis for numerical relativity — the numerical solution of Einstein's equations for binary merger events.

**Tetrad and vierbein formalism** introduces a local orthonormal frame at each spacetime point, making the local Lorentz symmetry of GR manifest. The metric is expressed as gᵤᵥ = η_{ab} e^a_μ e^b_ν, where e^a_μ are the "vierbein" (German: "four legs"). This formulation is essential for coupling GR to spinors (Dirac fermions) — the standard metric formulation cannot do this, because spinors transform under the Lorentz group, not the diffeomorphism group.

**The Newman-Penrose formalism** uses a null tetrad (two real null vectors and two complex null vectors) to decompose curvature components into complex scalars (the Newman-Penrose scalars Ψ₀, ..., Ψ₄). The gravitational wave degree of freedom is encoded in Ψ₄. The Petrov classification of spacetimes (their algebraic curvature type) is natural in this language.

**The Regge calculus** is a discrete approximation to GR: spacetime is approximated by a piecewise-flat simplicial complex, and curvature is concentrated in the hinges (lower-dimensional faces). Regge calculus is a stepping stone to loop quantum gravity and is used in numerical approaches to quantum gravity.

---

## Chapters in This Unit

- [Chapter 42: The 3+1 (ADM) Formalism](chapter-42-adm/README.md)
- [Chapter 43: Tetrad Formalism and Spinors in GR](chapter-43-tetrad-spinors/README.md)
- [Chapter 44: The Newman-Penrose Formalism](chapter-44-newman-penrose/README.md)
- [Chapter 45: Regge Calculus and Discrete Gravity](chapter-45-regge/README.md)
