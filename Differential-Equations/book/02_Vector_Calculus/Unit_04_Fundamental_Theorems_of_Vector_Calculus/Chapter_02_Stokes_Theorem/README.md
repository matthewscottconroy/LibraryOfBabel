# Chapter 2: Stokes' Theorem

Stokes' Theorem generalizes Green's Theorem from flat regions in the plane to curved surfaces in three-dimensional space. Where Green's Theorem relates a line integral around a planar curve to a double integral over the enclosed region, Stokes' Theorem relates a line integral around the boundary of a surface to a surface integral of the curl over the surface itself. The theorem is named after Sir George Gabriel Stokes (1819–1903), who posed it as an examination problem at Cambridge in 1854, though the result was known to Lord Kelvin somewhat earlier.

## Statement

**Stokes' Theorem.** Let $S$ be an oriented, piecewise smooth surface in $\mathbb{R}^3$ with boundary $\partial S$ (a piecewise smooth closed curve), oriented consistently with $S$ via the right-hand rule. Let $\mathbf{F}$ be a $C^1$ vector field on an open set containing $S$. Then

$$\oint_{\partial S}\mathbf{F}\cdot d\mathbf{r} = \iint_S(\nabla\times\mathbf{F})\cdot d\mathbf{S}.$$

The left side is the **circulation** of $\mathbf{F}$ around the boundary curve $\partial S$. The right side is the **flux of the curl** of $\mathbf{F}$ through the surface $S$.

## Chapter Overview

**Section 1: Statement and Proof** gives a full proof of Stokes' Theorem, first for a graph surface and then extended to general parametric surfaces. The proof reduces to Green's Theorem applied in the parameter domain.

**Section 2: Curl and Circulation** develops the relationship between local rotation (curl) and global circulation more deeply. The curl at a point is defined as the limit of circulation per unit area of infinitesimal loops — and Stokes' Theorem makes this precise and global.

**Section 3: Relationship to Green's Theorem** shows explicitly that Green's Theorem is the special case of Stokes' Theorem where $S$ is a flat region in the $xy$-plane. This unification clarifies both theorems and prepares the ground for the further unification through differential forms in Chapter 4.

## Key Consequences

**Conservative fields.** A field $\mathbf{F}$ with $\nabla\times\mathbf{F} = \mathbf{0}$ has $\iint_S(\nabla\times\mathbf{F})\cdot d\mathbf{S} = 0$ for any surface $S$, so the circulation around any boundary curve is zero. On simply connected domains, this implies $\mathbf{F}$ is conservative.

**Circulation depends only on the boundary.** For any two surfaces $S_1$ and $S_2$ with the same oriented boundary $\partial S = \partial S_1 = \partial S_2$ (and assuming $\nabla\times\mathbf{F}$ is defined on the region between them), the circulations are equal:

$$\iint_{S_1}(\nabla\times\mathbf{F})\cdot d\mathbf{S} = \iint_{S_2}(\nabla\times\mathbf{F})\cdot d\mathbf{S}.$$

This is the topological content of Stokes' Theorem: the curl flux is a function of the boundary alone, not of the specific surface spanning it.

**Maxwell's equations.** The differential form of Faraday's law, $\nabla\times\mathbf{E} = -\partial\mathbf{B}/\partial t$, follows from its integral form $\oint_C\mathbf{E}\cdot d\mathbf{r} = -d/dt\iint_S\mathbf{B}\cdot d\mathbf{S}$ via Stokes' Theorem (since the left side equals $\iint_S(\nabla\times\mathbf{E})\cdot d\mathbf{S}$).
