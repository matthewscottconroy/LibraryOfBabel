# Foundations

The study of differential equations rests on two great pillars: real analysis and linear algebra. Before a differential equation can be solved, it must be understood — and understanding requires knowing what it means for a function to be continuous, for a sequence to converge, for a system of equations to have a unique solution. This unit builds both pillars from the ground up, providing the mathematical infrastructure that every subsequent topic in the course depends on.

## Why Foundations Matter

Differential equations are statements about rates of change, and rates of change are limits. When we write $\frac{dy}{dt} = f(t, y)$, we are asserting something about the behavior of a function near every point in its domain. Whether that equation has a solution, whether that solution is unique, whether numerical approximations to it converge — all of these questions reduce, ultimately, to questions in real analysis and linear algebra. Students who have seen calculus but not real analysis often carry implicit assumptions about continuity and differentiability that break down precisely when they are most needed. This unit makes those assumptions explicit and replaces them with theorems.

## What This Unit Covers

The unit divides into two parts: Real Analysis Essentials and Linear Algebra Foundations.

**Real Analysis Essentials** begins with the logical scaffolding required to read and write mathematical proofs, then builds the real number system from its axioms. The completeness of the real line — the property that distinguishes $\mathbb{R}$ from $\mathbb{Q}$ — is not just a curiosity; it is what guarantees that sequences defined by iterative schemes actually converge, and it underpins the Intermediate Value Theorem, which is in turn used to prove existence of solutions. From completeness, the unit develops convergence of sequences and series, paying particular attention to power series, whose radius of convergence determines where a solution can be expressed as an infinite polynomial. The unit then covers continuity in depth, including uniform continuity (essential for interchanging limits and integrals), followed by differentiation through the Mean Value Theorem and Taylor's theorem with remainder, and closes with Riemann integration and the Fundamental Theorem of Calculus.

**Linear Algebra Foundations** builds the theory of vector spaces and linear maps, then develops matrices as the concrete computational face of that theory. Eigenvalues and eigenvectors are the central objects: the behavior of a linear ODE system $\mathbf{x}' = A\mathbf{x}$ is determined entirely by the eigenstructure of $A$. When $A$ is diagonalizable, the solution decouples into independent scalar equations; when it is not, Jordan normal form and the matrix exponential take over. The unit closes with inner product spaces and the Spectral Theorem, which will be essential when the course turns to boundary value problems and Sturm-Liouville theory.

## Prerequisites

This unit assumes the mathematical maturity that comes from a standard two-semester calculus sequence. Specifically, a student should be comfortable with:

- Computing derivatives and integrals of elementary functions
- Working with sequences and series at the level of a standard calculus course
- Matrix arithmetic and solving linear systems by row reduction
- The basic ideas of vectors in $\mathbb{R}^n$

What this unit does not assume is any prior exposure to proof-based mathematics. Chapter 1 of Unit 1 introduces propositional logic, quantifiers, and the standard proof techniques — direct proof, contrapositive, contradiction, and induction — precisely because everything that follows depends on them. Students who have already taken a course in discrete mathematics or introduction to proofs may move through Chapter 1 quickly, but should not skip it entirely; the language and notation established there are used consistently throughout.

## How the Units Connect

Real analysis and linear algebra are not independent. The solution space of a homogeneous linear ODE is a vector space, and recognizing it as such transforms a collection of formulas into a coherent structure. The theory of series solutions to ODEs — Frobenius method, Bessel equations, and their kin — requires power series from Unit 1 and matrix techniques from Unit 2. The spectral theory of Chapter 4 in Unit 2, combined with the convergence theory of Unit 1, provides the foundation for understanding Fourier series as expansions in orthogonal eigenfunctions.

Students who work through both units carefully will find that later topics in differential equations — existence and uniqueness theorems, stability analysis, series solutions, Laplace transforms, boundary value problems — arrive not as isolated techniques to memorize but as natural consequences of the mathematical framework built here.
