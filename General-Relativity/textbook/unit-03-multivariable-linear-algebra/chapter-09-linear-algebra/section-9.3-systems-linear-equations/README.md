# Section 9.3: Systems of Linear Equations

---

## Section Introduction

The problem of solving $n$ linear equations in $n$ unknowns — or more generally, $m$ equations in $n$ unknowns — is among the most ancient and practically important in mathematics. Gaussian elimination, developed in antiquity and systematized in the modern era, provides a complete algorithmic solution. But the theory of linear systems is more than an algorithm: it is a window into the structure of linear maps.

The system $A\mathbf{x} = \mathbf{b}$ (where $A$ is an $m\times n$ matrix, $\mathbf{x}\in\mathbb{R}^n$, $\mathbf{b}\in\mathbb{R}^m$) is:
- **Uniquely solvable** iff $A$ has full column rank and $\mathbf{b}\in\text{col}(A)$
- **Inconsistent** (no solution) iff $\mathbf{b}\notin\text{col}(A)$  
- **Underdetermined** (infinitely many solutions) iff $\ker A$ is nontrivial and $\mathbf{b}\in\text{col}(A)$

The **row echelon form** and **reduced row echelon form** produced by Gaussian elimination reveal this structure directly. The number of nonzero rows in echelon form is the **rank** of $A$; the rank equals both the column rank and the row rank (a nontrivial theorem).

For square systems ($m = n$), the existence of a unique solution is equivalent to $\det A\neq 0$ — the determinant is nonzero. This connects the solvability of linear systems to the eigenvalue theory: $A$ has a nontrivial kernel precisely when $0$ is an eigenvalue.

In physics, linear systems appear as the linearization of nonlinear equations near equilibrium (perturbation theory), as the discretized versions of differential equations (finite element methods), and as the equations of constraint in Lagrangian mechanics (Lagrange multipliers for holonomic constraints). In GR, the linearized Einstein equations are a system of linear PDEs for the metric perturbation $h_{\mu\nu}$; their solutions give gravitational waves.

---

## Subsections

- [9.3.1: Gaussian Elimination and Row Operations](9.3.1-gaussian.md)
- [9.3.2: Row Echelon Form and Rank](9.3.2-echelon.md)
- [9.3.3: The Solution Set: Consistency and Uniqueness](9.3.3-solution-set.md)
- [9.3.4: LU Decomposition](9.3.4-lu.md)
- [9.3.5: Applications and Numerical Methods](9.3.5-applications.md)
