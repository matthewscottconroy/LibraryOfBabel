# Unit 3: Functional Analysis for PDEs

Partial differential equations are naturally posed on infinite-dimensional function spaces: the solution is an element of a space of functions, and the PDE is a linear (or nonlinear) operator on that space. Classical real analysis, with its focus on finite-dimensional linear algebra and pointwise properties of functions, is inadequate for the systematic study of such problems. Functional analysis—linear algebra in infinite dimensions—provides the correct framework.

## The Central Challenge

In finite dimensions, every linear operator on $\mathbb{R}^n$ is bounded, every bounded sequence has a convergent subsequence, and every symmetric matrix has a complete orthonormal basis of eigenvectors. In infinite dimensions, none of these statements are true without additional hypotheses. Bounded versus unbounded operators must be distinguished. Compactness (a strengthening of boundedness that restores the convergent subsequence property) must be imposed where needed. The spectral theorem for symmetric matrices generalizes to self-adjoint operators, but the proof requires entirely new machinery.

## The Program of This Unit

**Chapter 1: Banach and Hilbert Spaces** establishes the foundational framework. Normed vector spaces generalize $\mathbb{R}^n$ with its Euclidean norm. The completeness condition (Cauchy sequences converge) gives Banach spaces, which are the natural setting for operator theory. Hilbert spaces add an inner product, enabling orthogonality and projection arguments analogous to those in finite dimensions. Bounded linear operators between Banach spaces generalize matrices.

**Chapter 2: Sobolev Spaces** constructs the specific function spaces needed for PDE theory. The Sobolev space $W^{k,p}(\Omega)$ consists of functions in $L^p(\Omega)$ all of whose weak derivatives up to order $k$ are also in $L^p$. These spaces are exactly the right setting for elliptic PDEs: the energy norm for a second-order elliptic PDE is the $H^1 = W^{1,2}$ norm. The Sobolev embedding theorems translate $W^{k,p}$ regularity into pointwise or higher-norm regularity, and the trace theorem defines the boundary values of Sobolev functions.

**Chapter 3: Spectral Theory** develops eigenvalue theory for operators on Hilbert spaces. Compact operators generalize finite-rank matrices and enjoy a spectral theorem: their nonzero eigenvalues form a sequence tending to zero, with finite-dimensional eigenspaces. Self-adjoint operators (bounded or unbounded) have real spectra and admit a spectral decomposition generalizing the eigenvalue decomposition of symmetric matrices. The application to the Sturm-Liouville problem shows how boundary value problems for ODEs (and the Laplacian on domains) are analyzed via spectral theory.

## Connections to PDEs

The Lax-Milgram theorem (a consequence of the Riesz representation theorem for Hilbert spaces) is the abstract existence and uniqueness theorem for elliptic PDEs in variational form: if $a(u,v)$ is a coercive bounded bilinear form on $H^1_0(\Omega)$, then for every $f \in H^{-1}(\Omega)$, there exists a unique $u \in H^1_0(\Omega)$ with $a(u,v) = \langle f, v \rangle$ for all $v$. This abstract theorem, proved in Chapter 1, encompasses Poisson's equation, the Helmholtz equation, and many other elliptic problems in a single framework.

The spectral theory of Chapter 3, applied to the Laplace-Beltrami operator on a Riemannian manifold (or the Laplacian on a bounded domain), produces the eigenfunction expansions used in Fourier analysis on domains and in the study of heat and wave equations.
