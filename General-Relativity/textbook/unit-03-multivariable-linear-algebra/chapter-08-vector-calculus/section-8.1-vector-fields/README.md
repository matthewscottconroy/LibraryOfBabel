# Section 8.1: Vector Fields

---

## Section Introduction

A **vector field** assigns a vector to each point of a region in space: $\mathbf{F}: \mathbb{R}^n \to \mathbb{R}^n$. The wind velocity at each point in the atmosphere is a vector field; the electric field $\mathbf{E}(\mathbf{x})$ and magnetic field $\mathbf{B}(\mathbf{x})$ are vector fields; the gravitational acceleration $\mathbf{g}(\mathbf{x})$ is a vector field. In differential geometry, a vector field on a manifold assigns a tangent vector at each point — this is the fundamental geometric object from which all differential geometry is built.

Vector fields have a rich algebraic and analytical structure. They can be added, scaled, and composed. More importantly, there are three differential operations — the **gradient** (on scalar fields), the **divergence**, and the **curl** — that together capture all the first-order differential information about vector fields in $\mathbb{R}^3$.

The **gradient** $\nabla f = (\partial f/\partial x, \partial f/\partial y, \partial f/\partial z)$ converts a scalar field to a vector field pointing in the direction of greatest increase. The **divergence** $\nabla\cdot\mathbf{F} = \partial F_x/\partial x + \partial F_y/\partial y + \partial F_z/\partial z$ measures the local expansion of a vector field — the net outflow per unit volume. The **curl** $\nabla\times\mathbf{F}$ measures the local rotation of a vector field — the "swirling" at a point. These operations are the building blocks of Maxwell's equations, the Navier-Stokes equations, and the equations of general relativity.

A vector field is **conservative** (or **irrotational**) if it is the gradient of some scalar function: $\mathbf{F} = \nabla\phi$. Such fields have zero curl and path-independent line integrals. A field is **solenoidal** (or **divergence-free**) if $\nabla\cdot\mathbf{F} = 0$, in which case it can be written as the curl of another vector field: $\mathbf{F} = \nabla\times\mathbf{A}$. These are exactly the conditions that appear in Maxwell's equations — the electric field is conservative in the static case, and the magnetic field is always solenoidal.

---

## Subsections

- [8.1.1: Definition and Examples of Vector Fields](8.1.1-definition.md)
- [8.1.2: The Gradient, Divergence, and Curl](8.1.2-grad-div-curl.md)
- [8.1.3: Conservative and Irrotational Fields](8.1.3-conservative.md)
- [8.1.4: Solenoidal Fields and Vector Potentials](8.1.4-solenoidal.md)
- [8.1.5: The Laplacian](8.1.5-laplacian.md)
