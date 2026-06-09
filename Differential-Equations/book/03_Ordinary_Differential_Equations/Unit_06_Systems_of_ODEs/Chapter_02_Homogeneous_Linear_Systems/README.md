# Chapter 2: Homogeneous Linear Systems with Constant Coefficients

For the constant-coefficient homogeneous system $\mathbf{x}' = A\mathbf{x}$ (with $A$ a constant matrix), the theory is complete and explicit. The key observation is that the trial solution $\mathbf{x} = \mathbf{v}e^{\lambda t}$ satisfies the system if and only if $\lambda$ is an eigenvalue of $A$ and $\mathbf{v}$ is a corresponding eigenvector.

## The Eigenvalue Method

Substituting $\mathbf{x} = \mathbf{v}e^{\lambda t}$: $\lambda\mathbf{v}e^{\lambda t} = A\mathbf{v}e^{\lambda t}$, so $(A - \lambda I)\mathbf{v} = \mathbf{0}$. This has a nontrivial solution $\mathbf{v} \neq \mathbf{0}$ if and only if $\det(A - \lambda I) = 0$: the characteristic equation of $A$.

## Chapter Contents

The first section handles the simplest case: $A$ has $n$ distinct real eigenvalues $\lambda_1, \ldots, \lambda_n$ with eigenvectors $\mathbf{v}_1, \ldots, \mathbf{v}_n$. The solutions $\mathbf{v}_k e^{\lambda_k t}$ are linearly independent (verified by the Wronskian), and the general solution is $\mathbf{x} = \sum c_k \mathbf{v}_k e^{\lambda_k t}$.

The second section addresses complex eigenvalues $\lambda = \alpha \pm \beta i$ with complex eigenvectors: the real and imaginary parts of the complex solution $\mathbf{v}e^{(\alpha+\beta i)t}$ give two real-valued linearly independent solutions involving $e^{\alpha t}\cos(\beta t)$ and $e^{\alpha t}\sin(\beta t)$.

The third section handles repeated eigenvalues, where the geometric multiplicity (number of independent eigenvectors) may be less than the algebraic multiplicity (order of the eigenvalue). Jordan blocks require generalized eigenvectors and give solutions with polynomial factors, exactly as in the scalar case.

The fourth section develops the matrix exponential $e^{At}$ as the fundamental matrix and discusses its computation via the Cayley-Hamilton theorem and diagonalization.

## Physical Significance

The eigenvalues of $A$ determine the stability of the zero equilibrium: if all eigenvalues have negative real part, the equilibrium is asymptotically stable; if any eigenvalue has positive real part, it is unstable. The eigenvectors determine the directions of the dominant modes of behavior.
