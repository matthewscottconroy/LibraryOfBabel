# Chapter 04: Eigentheory

Eigenvalues and eigenvectors are the key to understanding the behavior of linear maps and, consequently, linear differential equations. An eigenvector is a special direction in the vector space that is preserved by a linear map — the map simply scales the vector. The scaling factor is the corresponding eigenvalue. When a linear system $\mathbf{x}' = A\mathbf{x}$ is expressed in a basis of eigenvectors, it decouples into $n$ independent scalar ODEs, each trivially solved. This is the central insight of linear ODE theory: find the eigenstructure of $A$, and the system unravels.

## Chapter Overview

**Section 1: Eigenvalues and Eigenvectors** defines these objects and establishes the fundamental facts: eigenvectors form subspaces (eigenspaces), distinct eigenvalues correspond to linearly independent eigenvectors, and the eigenvalues of real matrices may be complex.

**Section 2: The Characteristic Polynomial** shows that eigenvalues are roots of $p(\lambda) = \det(\lambda I - A)$, a polynomial of degree $n$. The characteristic polynomial is the computational gateway to eigenvalues, and its structure (trace, determinant as coefficients) encodes key information about the matrix.

**Section 3: Diagonalization** addresses when a matrix is similar to a diagonal matrix. A matrix is diagonalizable iff it has $n$ linearly independent eigenvectors. For diagonalizable $A = PDP^{-1}$, the ODE system $\mathbf{x}' = A\mathbf{x}$ is solved by the change of variables $\mathbf{y} = P^{-1}\mathbf{x}$, which decouples it into $\mathbf{y}' = D\mathbf{y}$.

**Section 4: Jordan Normal Form** handles the non-diagonalizable case. Every matrix over $\mathbb{C}$ is similar to a Jordan matrix — a block-diagonal matrix with Jordan blocks. Solutions of $\mathbf{x}' = A\mathbf{x}$ for the non-diagonalizable case involve polynomial factors multiplying exponentials.

**Section 5: The Matrix Exponential** defines $e^{At}$ via power series and shows it is the solution to $\mathbf{x}' = A\mathbf{x}$ with $\mathbf{x}(0) = \mathbf{x}_0$. Properties: $e^{At}e^{As} = e^{A(t+s)}$, $(e^{At})' = Ae^{At}$, $\det(e^{At}) = e^{(\text{tr}\, A)t}$.

## Connection to Differential Equations

Eigentheory converts a matrix system of ODEs into a collection of scalar ODEs. The solution $e^{\lambda t}$ for a scalar ODE $y' = \lambda y$ becomes the building block: for a diagonalizable system, the solution is a superposition $\mathbf{x}(t) = \sum_i c_i e^{\lambda_i t} v_i$ where $v_i$ are eigenvectors. The stability of the zero solution (does it attract nearby solutions?) is determined by the sign of $\text{Re}(\lambda_i)$: stable if all $\text{Re}(\lambda_i) < 0$, unstable if any $\text{Re}(\lambda_i) > 0$.
