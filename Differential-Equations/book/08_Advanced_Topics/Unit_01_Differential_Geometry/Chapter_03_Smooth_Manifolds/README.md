# Chapter 3: Smooth Manifolds

The theory of curves and surfaces in $\mathbb{R}^3$ is rich and beautiful, but it is limited by its dependence on a particular ambient space. Many geometric objects of interest—the configuration space of a robot arm, the space of probability distributions, the parameter space of a family of differential equations—are not naturally subsets of any Euclidean space, or at least are better understood without reference to one. The concept of a smooth manifold generalizes surfaces to arbitrary dimensions while freeing geometry from any dependence on embedding.

## The Abstract Setting

A smooth manifold is a topological space that locally looks like $\mathbb{R}^n$ and on which smooth functions are well-defined. The local coordinates (chart maps) are tied together by smooth transition functions. This abstract framework captures both the local structure (charts) and the global topology (how the charts are assembled) of geometric objects.

Examples: the $n$-sphere $S^n$, real projective space $\mathbb{RP}^n$, the $n$-torus $T^n$, Lie groups such as $SO(3)$ and $SU(2)$, the space of positive-definite matrices, and every regular surface in $\mathbb{R}^3$.

## Tangent Spaces

At each point $p$ of a smooth manifold $M$, the **tangent space** $T_pM$ is the vector space of all velocity vectors of smooth curves passing through $p$. In local coordinates $(x^1, \ldots, x^n)$, a basis for $T_pM$ is $\{\partial/\partial x^1|_p, \ldots, \partial/\partial x^n|_p\}$. The **tangent bundle** $TM = \bigsqcup_{p \in M} T_pM$ is a $2n$-dimensional manifold that encodes all tangent spaces simultaneously.

Vector fields are smooth sections of $TM$: they assign to each point $p$ a tangent vector $X(p) \in T_pM$ smoothly. Differential equations on manifolds are precisely the study of integral curves of vector fields.

## Differential Forms

A **differential $k$-form** on $M$ is a smooth assignment of an alternating $k$-linear form to each tangent space. The exterior algebra of forms supports the **exterior derivative** $d: \Omega^k(M) \to \Omega^{k+1}(M)$, satisfying $d^2 = 0$. The sequence $\Omega^0 \to \Omega^1 \to \cdots \to \Omega^n$ forms the **de Rham complex**, whose cohomology captures topological information about $M$.

**Stokes' theorem** in its full generality:

$$\int_M d\omega = \int_{\partial M} \omega$$

unifies all classical integral theorems. The Gauss-Bonnet theorem is a consequence.

## Riemannian Metrics

A **Riemannian metric** on $M$ is a smooth assignment of an inner product to each tangent space. It allows one to measure lengths of curves, angles between tangent vectors, volumes of regions, and curvature—generalizing all the intrinsic geometry of surfaces to arbitrary dimensions. Every manifold admits a Riemannian metric (by a partition of unity argument), but different metrics yield different geometries.

With a Riemannian metric, one can define geodesics, sectional curvature, Ricci curvature, and scalar curvature. The Einstein field equations of general relativity are equations for the Ricci curvature of a four-dimensional Lorentzian manifold (a slight generalization of Riemannian).

## Chapter Structure

**Section 1** provides the precise definition of a smooth manifold and gives a catalog of examples. **Section 2** develops tangent spaces, the tangent bundle, and vector fields. **Section 3** treats differential forms, the exterior derivative, and Stokes' theorem. **Section 4** introduces Riemannian metrics and sketches the program of Riemannian geometry.
