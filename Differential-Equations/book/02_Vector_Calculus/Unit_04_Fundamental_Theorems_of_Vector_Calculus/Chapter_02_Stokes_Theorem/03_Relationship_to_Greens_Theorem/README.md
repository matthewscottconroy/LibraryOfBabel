# Relationship of Stokes' Theorem to Green's Theorem

Green's Theorem and Stokes' Theorem appear to be different theorems at first glance: one is about planar regions and line integrals, the other about surfaces in three-dimensional space. But Green's Theorem is exactly the special case of Stokes' Theorem where the surface $S$ is a flat region in the $xy$-plane. Making this reduction explicit clarifies both theorems and previews the further unification via differential forms in Chapter 4.

## The Reduction

Let $D$ be a bounded region in the $xy$-plane, and let $S = D$ (regarded as a flat surface in $\mathbb{R}^3$ with $z=0$). Orient $S$ with the upward normal $\hat{\mathbf{n}} = \mathbf{k}$.

By Stokes' Theorem:

$$\oint_{\partial D}\mathbf{F}\cdot d\mathbf{r} = \iint_D(\nabla\times\mathbf{F})\cdot\mathbf{k}\,dA.$$

Now, $(\nabla\times\mathbf{F})\cdot\mathbf{k}$ is the $z$-component of the curl:

$$(\nabla\times\mathbf{F})\cdot\mathbf{k} = \frac{\partial Q}{\partial x} - \frac{\partial P}{\partial y}.$$

Substituting:

$$\oint_{\partial D} P\,dx + Q\,dy = \iint_D\left(\frac{\partial Q}{\partial x} - \frac{\partial P}{\partial y}\right)dA.$$

This is exactly Green's Theorem (tangential form).

## What This Shows

Green's Theorem is Stokes' Theorem for the flat surface $z=0$ with upward normal. The $z$-component of the curl — $\partial Q/\partial x - \partial P/\partial y$ — is the only component that contributes to the flux through a horizontal surface, since $d\mathbf{S} = \mathbf{k}\,dA$ picks out only the $z$-component of the curl.

The boundary orientation compatibility is also inherited: counterclockwise traversal of $\partial D$ is the right-hand rule orientation for the upward normal $\hat{\mathbf{n}} = \mathbf{k}$.

## The Flux Form

The flux form of Green's Theorem (Section 3 of Chapter 1) is similarly a special case: it is the two-dimensional Divergence Theorem, which is the reduction of the three-dimensional Divergence Theorem to planar domains.

## Chains of Special Cases

The fundamental theorems form a chain:

$$\text{FTC for Line Integrals} \subset \text{Green's Theorem} \subset \text{Stokes' Theorem} \subset \text{Generalized Stokes' Theorem}.$$

Each theorem in the chain is a special case of the next. The Generalized Stokes' Theorem (Chapter 4), expressed in differential forms as $\int_M d\omega = \int_{\partial M}\omega$, encompasses all of them.

Similarly:

$$\text{Flux form of Green's} \subset \text{Divergence Theorem} \subset \text{Generalized Stokes' Theorem.}$$

This nested structure is not an accident — it reflects the fact that all these theorems encode the same abstract principle (boundary of a boundary is zero, or $\partial^2 = 0$) in different dimensional settings.

## Summary

Green's Theorem is Stokes' Theorem applied to a flat surface in the $xy$-plane. The $z$-component of the curl is the relevant quantity for flat horizontal surfaces, and the upward normal gives the counterclockwise boundary orientation. This reduction shows that all the fundamental theorems share a common algebraic structure, which will be fully revealed in Chapter 4 through the exterior derivative and Generalized Stokes' Theorem.
