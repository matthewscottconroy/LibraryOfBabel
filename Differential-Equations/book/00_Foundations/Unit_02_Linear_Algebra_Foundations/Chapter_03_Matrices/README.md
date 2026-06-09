# Chapter 03: Matrices

Matrices are the computational face of linear algebra. Every linear map between finite-dimensional vector spaces, after bases are chosen, corresponds to a matrix, and the algebraic operations on maps — composition, inversion — correspond to matrix multiplication and matrix inversion. This chapter develops the matrix calculus in full, from basic arithmetic through the systematic techniques (row reduction, LU and QR decomposition) that underlie all numerical linear algebra.

## What the Chapter Covers

**Section 1: Matrix Arithmetic** defines addition, scalar multiplication, and the non-commutative operation of matrix multiplication. The definition of matrix multiplication ($C = AB$ has $C_{ij} = \sum_k A_{ik}B_{kj}$) is motivated by the requirement that it represent composition of linear maps.

**Section 2: Matrix Multiplication and Composition** makes the correspondence between matrix multiplication and composition of linear maps explicit. It establishes that $(AB)C = A(BC)$ (associativity, reflecting the associativity of function composition) and $A(B+C) = AB + AC$ (distributivity).

**Section 3: Determinants** develops the determinant, a single number associated to a square matrix that captures whether the matrix is invertible, the signed volume scaling factor of the corresponding linear map, and (via the characteristic polynomial) the eigenvalues.

**Section 4: Inverse Matrices** treats invertibility: when does $A^{-1}$ exist, how is it computed, and what does invertibility mean for the solution of linear systems?

**Section 5: Row Reduction and Echelon Forms** presents Gaussian elimination as the systematic algorithm for solving $Ax = b$, computing rank, and finding bases for the fundamental subspaces. Row echelon form and reduced row echelon form are defined and used.

**Section 6: LU and QR Decomposition** develops the two most important matrix factorizations for computation: $A = LU$ (lower-upper triangular) for solving linear systems, and $A = QR$ (orthogonal-upper triangular) for least squares problems and eigenvalue algorithms.

## Connection to Differential Equations

Matrix exponentials ($e^{At}$, treated in Chapter 4), eigenvalue computation (essential for linear ODE systems), and the numerical solution of ODE boundary value problems (which reduces to solving large linear systems) all depend on the computational machinery of this chapter. Row reduction is the algorithm behind solving initial-value problems numerically by implicit methods. QR decomposition underlies the QR algorithm for computing eigenvalues numerically. LU decomposition makes the matrix operations in each step of an implicit ODE solver efficient.
