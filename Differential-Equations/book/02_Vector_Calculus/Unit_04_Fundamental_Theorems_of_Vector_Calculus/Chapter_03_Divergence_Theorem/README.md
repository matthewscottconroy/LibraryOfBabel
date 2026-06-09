# Chapter 3: The Divergence Theorem

The Divergence Theorem — also known as Gauss's Theorem or Ostrogradsky's Theorem — is the three-dimensional version of the flux form of Green's Theorem. It states that the total outward flux of a vector field through a closed surface equals the integral of the divergence of the field over the enclosed volume. This theorem is arguably the most important result in all of vector calculus for applications to physics: it converts global flux measurements (surfaces) into local source information (volume integrals), and vice versa.

## Statement

**Divergence Theorem.** Let $V$ be a bounded, simply connected open region in $\mathbb{R}^3$ with piecewise smooth boundary $\partial V$, oriented with the outward normal. Let $\mathbf{F}$ be a $C^1$ vector field on an open set containing $\overline{V}$. Then

$$\oiint_{\partial V}\mathbf{F}\cdot d\mathbf{S} = \iiint_V\nabla\cdot\mathbf{F}\,dV.$$

## Chapter Overview

**Section 1: Statement and Proof** gives a complete proof of the Divergence Theorem for rectangular boxes (by the Fundamental Theorem of Calculus in each coordinate direction) and explains how to extend to general regions by subdivision and cancellation. Worked examples verify the theorem for spheres and cylinders.

**Section 2: Flux, Sources, and Sinks** develops the physical interpretation in depth. The divergence $\nabla\cdot\mathbf{F}$ at a point measures the local source strength: the rate at which $\mathbf{F}$ is being "created" per unit volume. The Divergence Theorem says the total creation inside a volume equals the net outward flux through the boundary. This is the mathematical form of conservation: if nothing is created inside ($\nabla\cdot\mathbf{F} = 0$), then as much flows in as flows out.

**Section 3: Applications in Physics** applies the theorem to: Gauss's law (converting between differential and integral forms), the continuity equation (mass conservation in fluids), heat flux and Laplace's equation, and the derivation of Green's identities. These applications show the Divergence Theorem operating at the heart of mathematical physics.

## Key Computational Uses

The Divergence Theorem converts hard surface integrals into (sometimes easier) volume integrals, and vice versa:

**Surface to Volume:** If computing $\oiint_{\partial V}\mathbf{F}\cdot d\mathbf{S}$ directly is complicated (awkward surface), compute $\iiint_V\nabla\cdot\mathbf{F}\,dV$ instead (if divergence is simpler).

**Volume to Surface:** If the volume integral is complicated, sometimes the surface integral is simpler.

**Closed surfaces with holes:** If $V$ lies between two surfaces $S_1$ (outer) and $S_2$ (inner), then $\oiint_{S_1}\mathbf{F}\cdot d\mathbf{S} - \oiint_{S_2}\mathbf{F}\cdot d\mathbf{S} = \iiint_V\nabla\cdot\mathbf{F}\,dV$. If $\nabla\cdot\mathbf{F} = 0$ in $V$, the flux through $S_1$ equals the flux through $S_2$ — the flux is "conserved" as you move the surface outward through a divergence-free region.

## Green's Identities

As a direct consequence of the Divergence Theorem applied to $f\nabla g$:

$$\iiint_V f\nabla^2 g\,dV = \oiint_{\partial V} f\nabla g\cdot d\mathbf{S} - \iiint_V\nabla f\cdot\nabla g\,dV.$$

This is **Green's first identity**. Subtracting the same formula with $f$ and $g$ exchanged gives **Green's second identity**:

$$\iiint_V(f\nabla^2 g - g\nabla^2 f)\,dV = \oiint_{\partial V}(f\nabla g - g\nabla f)\cdot d\mathbf{S}.$$

These identities are fundamental to the theory of harmonic functions, potential theory, and the analysis of boundary value problems for Laplace's and Poisson's equations.
