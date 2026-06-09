# Unit 1: Differential Geometry

Differential geometry is the study of smooth geometric objects using the tools of calculus. The field ranges from the classical theory of curves and surfaces in $\mathbb{R}^3$—tractable through explicit computations with coordinates—to the abstract theory of smooth manifolds and Riemannian metrics, where the geometry is intrinsic (independent of any ambient space). For analysts, differential geometry provides the language for formulating equations on curved spaces; for topologists, it provides invariants like curvature and the Euler characteristic; for physicists, it is the mathematical foundation of general relativity and gauge theories.

## Classical Geometry: Curves and Surfaces

The simplest geometric objects are curves in $\mathbb{R}^3$. A regular curve is a smooth map $\gamma: I \to \mathbb{R}^3$ with nonvanishing derivative. The **Frenet frame** $\{T, N, B\}$ (tangent, normal, binormal) attached to the curve captures its shape through two scalar functions: curvature $\kappa$ (measuring bending) and torsion $\tau$ (measuring twisting out of a plane). The fundamental theorem of curves states that a curve is determined up to rigid motion by its curvature and torsion functions.

Surfaces in $\mathbb{R}^3$ are more complex. A **regular surface** locally looks like a piece of $\mathbb{R}^2$, embedded smoothly in $\mathbb{R}^3$. The **first fundamental form** is the restriction of the Euclidean inner product to the tangent plane: it measures lengths and areas intrinsically. The **second fundamental form** measures how the surface bends within $\mathbb{R}^3$.

## Curvature

The most important scalar invariant of a surface is the **Gaussian curvature** $K = \kappa_1 \kappa_2$, the product of the principal curvatures. Positive Gaussian curvature characterizes elliptic surfaces (like spheres); negative curvature characterizes hyperbolic surfaces (like saddle shapes); zero curvature characterizes developable surfaces (like cylinders and cones, which can be unrolled flat).

Gauss's **Theorema Egregium** ("remarkable theorem") establishes that Gaussian curvature is an intrinsic invariant: it can be computed entirely from the first fundamental form, without reference to the embedding in $\mathbb{R}^3$. This means that Gaussian curvature is preserved by isometries (distance-preserving maps). The theorem is remarkable because the Gaussian curvature is defined extrinsically (using the second fundamental form), yet turns out to be intrinsic. It implies, for instance, that a flat map of a spherical earth must distort distances.

## Intrinsic Geometry and the Gauss-Bonnet Theorem

The **Gauss-Bonnet theorem** is the crowning result of classical surface theory. For a compact surface $S$ without boundary:

$$\iint_S K \, dA = 2\pi \chi(S),$$

where $\chi(S) = 2 - 2g$ is the Euler characteristic and $g$ is the genus (number of handles). This extraordinary identity equates a differential-geometric quantity (the integral of curvature) to a purely topological invariant (the Euler characteristic). It shows that, however a surface is bent in space, the total Gaussian curvature is determined by its topology alone.

## Smooth Manifolds

The abstract generalization of surfaces is the concept of a smooth manifold: a topological space locally homeomorphic to $\mathbb{R}^n$, with compatible smooth coordinate charts. Manifolds free geometry from dependence on an ambient Euclidean space: the $n$-sphere $S^n$, projective spaces, Lie groups, and configuration spaces of mechanical systems are all manifolds.

On a manifold, one defines tangent vectors (derivations of smooth functions at a point), the tangent bundle (the disjoint union of all tangent spaces), differential forms (alternating multilinear functionals on tangent vectors), and the exterior derivative. Stokes' theorem—$\int_M d\omega = \int_{\partial M} \omega$—unifies all the classical theorems of vector calculus (Green's theorem, the divergence theorem, the classical Stokes' theorem) in a single identity.

A **Riemannian metric** on a manifold is a smoothly varying inner product on each tangent space, allowing one to define lengths, angles, volumes, and curvature intrinsically. Riemannian geometry is the natural generalization of the classical surface theory studied in Chapters 1 and 2.

## Unit Structure

**Chapter 1** (Curves and Surfaces Revisited) reviews and deepens the classical theory: the Frenet-Serret formulas, the fundamental theorems for curves and surfaces, the first and second fundamental forms.

**Chapter 2** (Intrinsic Geometry) develops curvature theory: Gaussian and mean curvature, the Gauss-Bonnet theorem, and Gauss's Theorema Egregium.

**Chapter 3** (Smooth Manifolds) builds the abstract framework: manifold definitions and examples, tangent spaces and tangent bundles, differential forms and the exterior derivative, and Riemannian metrics.
