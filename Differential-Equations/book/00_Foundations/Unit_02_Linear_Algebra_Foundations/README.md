# Unit 02: Linear Algebra Foundations

Linear algebra is the mathematics of linearity: of operations that preserve addition and scaling. Its subject matter — vector spaces, linear maps, matrices, eigenvalues — is deceptively simple in its axioms but extraordinarily rich in its consequences. For differential equations, linear algebra is not merely a computational tool; it provides the structural framework within which solutions live. The solution set of a homogeneous linear ODE is a vector space, every solution method exploits linearity, and the long-term behavior of a linear system is encoded entirely in its eigenvalues.

## Why Linear Algebra?

A first-order linear ODE $y' = ay$ has a one-dimensional solution space spanned by $e^{at}$. A second-order linear ODE $y'' + py' + qy = 0$ has a two-dimensional solution space, meaning the general solution is a linear combination $c_1 y_1 + c_2 y_2$ of any two linearly independent particular solutions. The $n$-th order case has an $n$-dimensional solution space. This structure — "the solution set is a vector space of dimension $n$" — is the central organizing principle of linear ODE theory, and it is a linear algebra statement.

Systems of linear ODEs $\mathbf{x}' = A\mathbf{x}$ are solved using the eigenstructure of the matrix $A$. If $A$ is diagonalizable with eigenvalues $\lambda_1, \ldots, \lambda_n$, the system decouples into $n$ independent scalar equations, each with solution $e^{\lambda_i t}$. If $A$ is not diagonalizable, Jordan normal form takes over. The long-term behavior — does the solution grow, decay, or oscillate? — is determined by the sign of the real parts of the eigenvalues, a fact whose proof is linear algebra. Inner product spaces and the Spectral Theorem, covered in Chapter 5, underlie Sturm-Liouville theory and the theory of Fourier series as eigenfunction expansions.

## Structure of the Unit

**Chapter 1: Vector Spaces** develops the abstract theory. A vector space is a set with addition and scalar multiplication satisfying eight axioms. The solution space of a linear ODE is the prototype. The chapter covers subspaces, linear independence, bases, and dimension — the foundational language for all that follows.

**Chapter 2: Linear Maps** studies functions between vector spaces that preserve the algebraic structure. The kernel and image of a linear map are subspaces, and the Rank-Nullity Theorem (dimension of kernel plus dimension of image equals dimension of domain) is the fundamental structural result. Differential operators — $D(f) = f'$, $L(y) = y'' + py' + q$ — are linear maps on function spaces, and the theory developed here applies to them directly.

**Chapter 3: Matrices** connects the abstract theory to computation. Matrices represent linear maps between finite-dimensional vector spaces, and matrix multiplication corresponds to composition of maps. The chapter covers determinants, inverses, row reduction, and the LU and QR decompositions. These are the computational tools for solving linear systems and for numerical methods.

**Chapter 4: Eigentheory** is the heart of the unit and of all of linear ODE theory. Eigenvalues and eigenvectors are introduced, the characteristic polynomial is derived, and diagonalization is developed for matrices with $n$ independent eigenvectors. Jordan normal form handles the non-diagonalizable case, and the matrix exponential $e^{At}$ provides the solution formula for the system $\mathbf{x}' = A\mathbf{x}$.

**Chapter 5: Inner Product Spaces** adds geometry to the algebraic structure. The dot product, norms, orthogonality, the Gram-Schmidt process, and orthogonal projections are developed. The Spectral Theorem — symmetric (or self-adjoint) matrices are always diagonalizable with real eigenvalues and orthogonal eigenvectors — is the culminating result, with immediate application to systems with symmetric coefficient matrices and to the theory of self-adjoint boundary value problems.

## How the Chapters Connect

The chapters form a logical chain. Bases and dimension (Chapter 1) are required to define the matrix of a linear map (Chapter 2–3). Matrices are needed to state the characteristic polynomial and compute eigenvalues (Chapter 4). Eigenvalues are needed to state the Spectral Theorem (Chapter 5). Each chapter also enriches its predecessors: the Rank-Nullity Theorem (Chapter 2) answers "how many independent solutions does a linear ODE have?"; the Spectral Theorem (Chapter 5) tells you when the eigenvectors of a real matrix form an orthonormal basis. A student who reads through the unit in order will find each chapter preparing exactly what the next one needs.
