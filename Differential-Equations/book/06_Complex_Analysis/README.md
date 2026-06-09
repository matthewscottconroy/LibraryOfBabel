# Complex Analysis

Complex analysis is the study of functions of a complex variable, and it stands as one of the most beautiful and internally coherent branches of mathematics. Where real analysis often requires delicate hypotheses and careful counterexamples, complex analysis rewards you with theorems of extraordinary power from surprisingly modest assumptions. A function that is merely once differentiable in the complex sense turns out to be infinitely differentiable, representable by convergent power series, and subject to global constraints that have no real-variable analogue.

## Why Complex Analysis Matters

The subject begins with a simple algebraic extension: adjoin a square root of $-1$ to the real numbers to obtain $\mathbb{C}$. But this small step opens a vast landscape. The geometry of the plane becomes algebraically active, multiplication encodes rotation and scaling, and differentiation acquires a two-dimensional character that fuses analysis with geometry in a deep way.

Complex analysis is not merely an elegant theory. It is an indispensable computational tool across mathematics, physics, and engineering. Fourier and Laplace transforms, the evaluation of definite integrals that resist elementary methods, the analysis of fluid flow and electrostatics, the distribution of prime numbers via the Riemann zeta function — all draw on the core ideas developed here.

## Structure of This Module

This module is organized into four units, progressing from foundational algebra through the deepest applications.

**Unit 01: Complex Numbers and Functions** establishes the number system $\mathbb{C}$, its algebra, geometry, and topology. The polar form of a complex number leads immediately to de Moivre's theorem and the elegant theory of roots of unity. The unit closes with a careful treatment of complex-valued functions, including the subtle issue of multivalued functions and branch cuts — a phenomenon with no real-variable counterpart.

**Unit 02: Complex Differentiation** introduces the central concept of analyticity. The Cauchy-Riemann equations translate complex differentiability into a coupled system of real partial differential equations, revealing that analytic functions have real and imaginary parts that are both harmonic. The elementary functions — exponential, logarithm, trigonometric, and power functions — are extended to the complex plane, where their behavior is richer and their interrelations clearer.

**Unit 03: Complex Integration** develops contour integration and arrives at the theorems of Cauchy, which are the engine of the entire subject. Cauchy's theorem states that the integral of an analytic function around a closed curve in a simply connected domain is zero. From this fact flows the Cauchy integral formula, the representation of derivatives as integrals, Liouville's theorem, and a proof of the Fundamental Theorem of Algebra. The unit concludes with Taylor and Laurent series, which classify the local behavior of analytic functions and introduce the concept of a singularity.

**Unit 04: Residue Theory and Conformal Mapping** harvests the full computational power of complex analysis. The residue theorem converts contour integrals into a sum over isolated singularities, yielding a machinery for evaluating a remarkable range of real integrals and infinite series. The module closes with conformal mapping: angle-preserving maps between domains, the classification of Mobius transformations, the Schwarz-Christoffel formula for mapping polygonal domains, and the Riemann Mapping Theorem, which asserts that any simply connected proper subdomain of $\mathbb{C}$ is conformally equivalent to the unit disk.

## Prerequisites

A student entering this module should be comfortable with multivariable calculus (partial derivatives, line integrals, Green's theorem) and the core ideas of real analysis (sequences, series, uniform convergence, continuity, and differentiability on $\mathbb{R}$). Familiarity with linear algebra is helpful but not strictly required.

## A Note on Rigor

These notes aim for full mathematical precision. Theorems are stated with complete hypotheses, and proofs are either given in full or sketched with enough detail that the interested reader can supply the missing steps. The goal is not merely to teach techniques but to convey why they work — the geometric intuition behind analytic continuation, the topological content of simply connectedness, and the surprising global consequences of local differentiability.
