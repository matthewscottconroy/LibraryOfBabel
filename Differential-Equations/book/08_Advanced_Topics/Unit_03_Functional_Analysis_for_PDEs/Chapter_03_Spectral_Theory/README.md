# Chapter 3: Spectral Theory

The spectral theory of linear operators is the generalization of the eigenvalue theory of matrices to infinite-dimensional spaces. For a symmetric matrix, the spectral theorem guarantees a complete orthonormal basis of eigenvectors with real eigenvalues. For infinite-dimensional operators, the appropriate generalization depends on the type of operator: compact operators have discrete spectra with accumulating eigenvalues; bounded self-adjoint operators have more general spectral decompositions involving projection-valued measures; unbounded self-adjoint operators (like the Laplacian) have spectra that can be discrete, continuous, or mixed.

## Why Spectral Theory for PDEs

The connection between spectral theory and PDEs is deep and multidirectional:

**Eigenvalue problems.** The equation $-\Delta u = \lambda u$ in $\Omega$ with $u|_{\partial\Omega} = 0$ is an eigenvalue problem for the Dirichlet Laplacian. Its solutions (eigenfunctions) form a basis for $L^2(\Omega)$, enabling eigenfunction expansions analogous to Fourier series—but adapted to the geometry of $\Omega$.

**Solution by eigenfunction expansion.** The heat equation $\partial_t u = \Delta u$ in $\Omega$ with homogeneous Dirichlet conditions is solved by expanding $u$ in eigenfunctions: $u(x,t) = \sum_n c_n e^{-\lambda_n t}\phi_n(x)$, where $\lambda_n$ and $\phi_n$ are the Dirichlet eigenvalues and eigenfunctions of $-\Delta$.

**Spectral information encodes geometry.** The eigenvalues of the Laplacian on a domain $\Omega$ (or Riemannian manifold) encode geometric information: the volume, the diameter, and (in some cases) the shape of $\Omega$. The question "Can you hear the shape of a drum?" (Kac, 1966)—whether the Laplacian spectrum determines the domain—drove decades of research in spectral geometry.

## Chapter Structure

**Section 1: Compact Operators.** Compact operators have discrete spectra with eigenvalues accumulating only at zero. The Fredholm alternative—either $Tu = f$ has a unique solution, or the homogeneous equation $Tu = 0$ has nontrivial solutions—governs the solvability of compact perturbations of the identity. For PDEs, the resolvent $(-\Delta - \lambda)^{-1}$ (when it exists) is compact on bounded domains, giving the Laplacian a discrete spectrum.

**Section 2: Spectral Theorem for Self-Adjoint Operators.** Bounded self-adjoint operators on Hilbert spaces have a spectral decomposition into projections. Unbounded self-adjoint operators (like $-\Delta$) require the theory of spectral measures. The Sturm-Liouville operator is the primary example: it is self-adjoint and has a discrete spectrum (countably many eigenvalues $\lambda_n \to \infty$) with eigenfunctions forming a complete orthonormal basis for $L^2$.

**Section 3: Application to Sturm-Liouville.** The Sturm-Liouville operator $Lf = -(p(x)f')' + q(x)f$ with appropriate boundary conditions is the paradigm of a self-adjoint unbounded operator with discrete spectrum. Its eigenvalue theory is the foundation of the classical theory of ODEs (separation of variables, eigenfunction expansions) and generalizes to the Laplace-Beltrami operator on Riemannian manifolds.
