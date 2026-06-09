# Advanced Topics in Differential Equations

The modules preceding this one have developed the classical theory of ordinary differential equations—existence, uniqueness, linear systems, power series methods, Laplace transforms, and the geometric theory of dynamical systems. This module ventures into territory that is both deeper and broader: the mathematical infrastructure that underlies modern analysis of partial differential equations, and the geometric language needed to formulate equations on curved spaces.

## Overview of the Module

Three units form the core of this module, each addressing a distinct foundational area.

**Unit 1: Differential Geometry** provides the geometric language for formulating differential equations on curved spaces. Starting from curves and surfaces in $\mathbb{R}^3$, the unit develops the intrinsic geometry of surfaces (curvature, geodesics, the Gauss-Bonnet theorem) and then builds toward the abstract framework of smooth manifolds, tangent bundles, differential forms, and Riemannian metrics. Differential geometry is not merely background material: many important PDEs (the heat equation, the wave equation, the Einstein field equations) are most naturally formulated on Riemannian manifolds, and the geometry shapes the analysis of their solutions.

**Unit 2: Distributions and Generalized Functions** addresses a fundamental limitation of classical analysis: many naturally occurring "functions"—the Dirac delta function, derivatives of discontinuous functions, fundamental solutions of PDEs—do not fit within the framework of ordinary functions. The theory of distributions (Schwartz, 1945) resolves this by extending the dual space of smooth functions. This framework makes rigorous the physicists' manipulations with delta functions, provides a coherent theory of differentiation for non-smooth objects, and is indispensable for the study of fundamental solutions and Green's functions.

**Unit 3: Functional Analysis for PDEs** develops the infinite-dimensional linear algebra needed for modern PDE theory. Banach spaces, Hilbert spaces, bounded linear operators, Sobolev spaces, and spectral theory are the key ingredients. Sobolev spaces are function spaces tailored to PDEs: they measure regularity in terms of $L^2$ norms of derivatives, and the Sobolev embedding and trace theorems translate this regularity into pointwise or boundary behavior. The Lax-Milgram theorem provides existence and uniqueness for elliptic PDEs in weak (variational) form, and the spectral theorem for self-adjoint operators generalizes the eigenvalue decomposition of symmetric matrices to infinite dimensions.

## Connections Between Units

The three units are deeply interconnected. Differential forms on manifolds are precisely the right objects for formulating Stokes' theorem, which in turn is the foundation for integration by parts in PDE theory. Distributions generalize functions and are the natural coefficients for equations on manifolds. Sobolev spaces are built on $L^2$ inner products, which connect to the Hilbert space theory of Unit 3, and the trace theorem in Sobolev theory is proved using differential geometry (the Riemannian measure on the boundary). Spectral theory of self-adjoint operators applies directly to the Laplacian on a Riemannian manifold, connecting Units 1, 2, and 3.

## Prerequisites

This module assumes fluency with multivariable calculus, linear algebra, and the ODE theory developed in the preceding modules. Unit 2 requires familiarity with metric spaces and the basic definitions of Lebesgue integration. Unit 3 requires comfort with abstract vector spaces and inner product geometry. The differential geometry in Unit 1 builds on multivariable calculus and linear algebra and is largely self-contained.

Students who complete this module will have access to the foundational tools of modern analysis, enabling them to read advanced texts in PDE theory, Riemannian geometry, and mathematical physics.
