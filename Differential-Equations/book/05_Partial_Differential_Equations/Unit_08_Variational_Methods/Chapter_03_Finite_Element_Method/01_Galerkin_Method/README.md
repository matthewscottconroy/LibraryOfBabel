# The Galerkin Method

The Galerkin method (Boris Galerkin, 1915) converts the variational (weak) form of a PDE into a finite-dimensional linear system by projecting the problem onto a finite-dimensional subspace. It is the abstract framework underlying the finite element method, spectral methods, the boundary element method, and many other numerical schemes. The key theoretical properties — best approximation (Céa's lemma), Galerkin orthogonality, and the a priori error estimate — flow from the abstract structure and apply regardless of the specific choice of basis.

## The Abstract Galerkin Framework

**Setting.** Let $H$ be a Hilbert space, $a:H\times H\to\mathbb{R}$ a continuous coercive bilinear form with constants $M$ (boundedness) and $\alpha$ (coercivity), and $F\in H^*$ a bounded linear functional. The continuous problem:

$$\text{Find }u\in H: \quad a(u,v) = F(v) \quad \text{for all }v\in H.$$

**Galerkin approximation.** Choose a finite-dimensional subspace $V_N\subset H$ (dim $V_N = N$). The Galerkin problem:

$$\text{Find }u_N\in V_N: \quad a(u_N,v_N) = F(v_N) \quad \text{for all }v_N\in V_N. \tag{Galerkin}$$

By Lax-Milgram applied to the Hilbert space $V_N$ (with inherited inner product and same coercivity constant $\alpha$), a unique $u_N\in V_N$ exists.

## Galerkin Orthogonality

**Proposition.** $a(u-u_N,v_N) = 0$ for all $v_N\in V_N$.

**Proof.** From the continuous problem: $a(u,v_N) = F(v_N)$ for all $v_N\in V_N\subset H$. From the Galerkin problem: $a(u_N,v_N) = F(v_N)$ for all $v_N\in V_N$. Subtracting: $a(u-u_N,v_N) = 0$. $\square$

**Geometric interpretation.** When $a$ is symmetric, it defines an inner product on $H$, and Galerkin orthogonality says $u-u_N \perp_a V_N$ — the error is $a$-orthogonal to the subspace. The Galerkin approximation $u_N$ is the $a$-orthogonal projection of $u$ onto $V_N$:

$$u_N = \text{proj}_{V_N}^a u = \arg\min_{w_N\in V_N}|u-w_N|_a,$$

where $|v|_a^2 = a(v,v)$ is the energy norm. The Galerkin approximation is the **best approximation** in the energy norm, which is better than the best approximation in the $H$ norm by a factor of $M/\alpha$.

## Céa's Lemma (Best Approximation)

**Theorem.** $\|u-u_N\|_H \leq \frac{M}{\alpha}\inf_{w_N\in V_N}\|u-w_N\|_H$.

**Proof.** For any $w_N\in V_N$:

$$\alpha\|u-u_N\|_H^2 \leq a(u-u_N,u-u_N) = a(u-u_N,u-w_N) + a(u-u_N,w_N-u_N).$$

The second term: $w_N-u_N\in V_N$, so $a(u-u_N,w_N-u_N) = 0$ by Galerkin orthogonality. The first term: $|a(u-u_N,u-w_N)| \leq M\|u-u_N\|\|u-w_N\|$ by boundedness. Therefore:

$$\alpha\|u-u_N\|_H \leq M\|u-w_N\|_H \quad \text{for all }w_N\in V_N.$$

Taking infimum over $w_N\in V_N$: $\|u-u_N\|_H \leq (M/\alpha)\inf_{w_N\in V_N}\|u-w_N\|_H$. $\square$

**Remark.** For the Poisson problem, $M = \alpha = 1$ (the bilinear form $a(u,v) = \int\nabla u\cdot\nabla v$ satisfies $|a(u,v)| \leq \|\nabla u\|_{L^2}\|\nabla v\|_{L^2}$ and $a(u,u) = \|\nabla u\|_{L^2}^2$). Céa's lemma gives $\|u-u_N\|_{H^1} \leq \inf_{w_N\in V_N}\|u-w_N\|_{H^1}$ — the Galerkin error equals the best approximation error exactly.

## The Linear System

Choose a basis $\{\phi_1,\ldots,\phi_N\}$ for $V_N$. Write $u_N = \sum_{j=1}^N U_j\phi_j$. The Galerkin equation (Galerkin) with test function $v_N = \phi_i$ becomes:

$$\sum_{j=1}^N U_j a(\phi_j,\phi_i) = F(\phi_i), \quad i=1,\ldots,N.$$

In matrix form: $\mathbf{K}\mathbf{U} = \mathbf{F}$, where:

$$K_{ij} = a(\phi_j,\phi_i), \qquad F_i = F(\phi_i), \qquad \mathbf{U} = (U_1,\ldots,U_N)^T.$$

**$\mathbf{K}$ is positive definite (when $a$ is coercive).** For any $\mathbf{c} = (c_1,\ldots,c_N)^T\neq 0$:

$$\mathbf{c}^T\mathbf{K}\mathbf{c} = \sum_{i,j}c_i K_{ij}c_j = \sum_{i,j}c_i a(\phi_j,\phi_i)c_j = a\!\left(\sum_j c_j\phi_j, \sum_i c_i\phi_i\right) = a(w,w) \geq \alpha\|w\|^2 > 0,$$

where $w = \sum_j c_j\phi_j\in V_N$, $w\neq 0$ (since $\{\phi_j\}$ is a basis). So $\mathbf{K}$ is symmetric positive definite (when $a$ is symmetric), and the linear system $\mathbf{KU} = \mathbf{F}$ has a unique solution.

**Symmetric $a$ implies symmetric $\mathbf{K}$:** $K_{ij} = a(\phi_j,\phi_i) = a(\phi_i,\phi_j) = K_{ji}$. For the Poisson problem: $K_{ij} = \int\nabla\phi_j\cdot\nabla\phi_i\,dx$ — symmetric by construction.

## Spectral Galerkin Method

The simplest Galerkin method is the **spectral method**: choose $V_N = \text{span}\{e_1,\ldots,e_N\}$ where $\{e_k\}$ are the eigenfunctions of the operator $A$ (with $A e_k = \lambda_k e_k$). Then $a(e_k,e_j) = \lambda_k\delta_{kj}$ (diagonal), and the Galerkin system is diagonal: $U_k = F(e_k)/\lambda_k$.

For the Laplacian on $[0,1]$ with $e_k = \sqrt{2}\sin(k\pi x)$ and $\lambda_k = k^2\pi^2$: $U_k = \hat f_k/(k^2\pi^2)$ (Fourier sine coefficients of $f$ divided by $\lambda_k$). This is exactly the classical Fourier series solution of Poisson's equation.

**Error in spectral Galerkin.** For smooth $f$, the Fourier coefficients $\hat f_k$ decay rapidly, and the truncation error $\|u-u_N\|_{H^1}^2 = \sum_{k>N}\lambda_k|\hat u_k|^2 = \sum_{k>N}|\hat f_k|^2/\lambda_k$ decays faster than any power of $1/N$ — **spectral convergence** (exponential for analytic $u$). This is the fundamental advantage of spectral methods over finite element methods for smooth problems on simple geometries.

## Worked Example: 1D Poisson with Piecewise Constant Basis

**Problem.** $-u'' = f$ on $(0,1)$, $u(0) = u(1) = 0$. Use $N=2$ with the (non-conforming) piecewise constant basis $\phi_1 = \mathbf{1}_{(0,1/2)}$, $\phi_2 = \mathbf{1}_{(1/2,1)}$.

**Warning.** $\phi_1, \phi_2$ are not in $H^1_0(0,1)$ (they are discontinuous), so this is a non-conforming Galerkin method. For illustration only.

The Galerkin equation with bilinear form $a(u,v) = \int_0^1 u'v'\,dx$ requires $u_N$ and $v_N$ to have $L^2$ derivatives. Since $\phi_j$ are piecewise constant, their $L^2$ derivatives are Dirac deltas at the discontinuities — not in $L^2$. This example illustrates why the basis functions must be in $H^1_0(\Omega)$ for conforming Galerkin.

**Conforming example.** Instead: $V_2 = \text{span}\{x(1-x), x^2(1-x)\}$ (quadratic polynomials vanishing at endpoints). Then $K_{11} = \int_0^1[(x(1-x))']^2 = \int_0^1(1-2x)^2\,dx = 1/3$, etc. The resulting $2\times 2$ system approximates the solution.

## Petrov-Galerkin and Non-Symmetric Problems

For non-symmetric bilinear forms (e.g., convection-diffusion $a(u,v) = \int\nabla u\cdot\nabla v + \mathbf{b}\cdot\nabla u\cdot v$), the standard Galerkin method can be unstable when convection dominates diffusion (Péclet number $\gg 1$). The **Petrov-Galerkin** method uses different test spaces: $a(u_N,v_N) = F(v_N)$ for all $v_N\in W_N\neq V_N$. The **SUPG (Streamline Upwind Petrov-Galerkin)** method adds a stabilizing term to the test functions, weighted by the direction of the flow. This prevents spurious oscillations (Gibbs-type phenomena) that arise in the standard Galerkin method for convection-dominated problems.
