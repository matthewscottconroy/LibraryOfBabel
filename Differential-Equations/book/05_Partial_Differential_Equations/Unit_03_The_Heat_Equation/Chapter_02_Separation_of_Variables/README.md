# Chapter 2: Separation of Variables for the Heat Equation

Separation of variables is the first and most important method for solving the heat equation on bounded domains. The idea is to seek solutions of the form $u(x,t) = X(x)T(t)$ — a product of a function of space alone and a function of time alone. Substituting into the PDE decouples the equation into two ODEs: one for $X$ (a Sturm-Liouville eigenvalue problem) and one for $T$ (a simple first-order linear ODE). The eigenfunctions of the spatial problem form a complete orthogonal basis, and the general solution is an infinite series whose coefficients are determined by the initial condition via Fourier's formula.

## Why Separation Works

The heat equation $u_t = \kappa u_{xx}$ is linear, which means any superposition of solutions is a solution. If we can find a complete family of simple solutions and express the initial data in terms of that family, the PDE is solved. Separation of variables finds exactly such a family.

The key equation after substituting $u = X(x)T(t)$:

$$\frac{T'(t)}{\kappa T(t)} = \frac{X''(x)}{X(x)} = -\lambda,$$

where the separation constant $-\lambda$ must be the same constant since the left side depends only on $t$ and the right side only on $x$. The spatial equation $X'' = -\lambda X$ with boundary conditions is a Sturm-Liouville eigenvalue problem, and the temporal equation $T' = -\kappa\lambda T$ gives $T(t) = e^{-\kappa\lambda t}$.

## Structure of This Chapter

**Section 1: Homogeneous Boundary Conditions on a Slab** solves the heat equation on $[0,L]$ with Dirichlet boundary conditions $u(0,t) = u(L,t) = 0$. The eigenfunctions are $\sin(n\pi x/L)$ and the solution is a Fourier sine series:

$$u(x,t) = \sum_{n=1}^\infty b_n e^{-\kappa(n\pi/L)^2 t}\sin\!\left(\frac{n\pi x}{L}\right).$$

The exponential decay factors $e^{-\kappa(n\pi/L)^2 t}$ show that high-frequency modes ($n$ large) decay much faster than low-frequency modes. This is the mathematical statement of the smoothing property: initial roughness (high-frequency content) is progressively erased.

**Section 2: Eigenfunction Expansion** develops the theory more broadly: general Sturm-Liouville problems, orthogonality of eigenfunctions, completeness, and the formula for eigenfunction expansion coefficients. The connection to the general theory of self-adjoint operators in Hilbert space is made explicit.

**Section 3: Nonhomogeneous Boundary Conditions and Sources** handles the case where boundary data is nonzero or a source term is present. The strategy is to subtract a steady-state (or quasi-static) solution to reduce to a homogeneous problem, then apply the method of eigenfunction expansion.

## Central Results Previewed

The fundamental theorem underlying all of Chapter 2 is the completeness of the eigenfunctions of the Sturm-Liouville problem:

**Theorem.** Let $\{\phi_n\}_{n=1}^\infty$ be the normalized eigenfunctions of $-d^2/dx^2$ on $[0,L]$ with Dirichlet boundary conditions. Then $\{\phi_n\}$ is a complete orthonormal basis for $L^2(0,L)$: every $f \in L^2(0,L)$ can be written as $f = \sum_{n=1}^\infty \langle f,\phi_n\rangle \phi_n$ with convergence in $L^2$.

This theorem — a special case of the spectral theorem for compact self-adjoint operators — is what allows the separation of variables method to solve the initial value problem for arbitrary initial data, not just for data that happens to be a finite sum of eigenfunctions.
