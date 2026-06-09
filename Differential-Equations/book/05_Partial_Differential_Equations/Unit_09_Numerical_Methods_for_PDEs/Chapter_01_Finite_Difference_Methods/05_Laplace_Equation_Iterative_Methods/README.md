# Laplace Equation: Iterative Methods

The five-point stencil discretization of $-\Delta u = f$ on a rectangle $[0,1]^2$ with Dirichlet boundary conditions produces a sparse linear system $Au = b$ of dimension $(M-1)^2 \times (M-1)^2$. Unlike the tridiagonal systems arising from the heat equation in one spatial dimension, this system is banded with bandwidth $M-1$ — far from tridiagonal, yet too sparse to invert directly without enormous cost. The remedy is **iterative methods**: Jacobi, Gauss-Seidel, and SOR (successive overrelaxation). These methods exploit the structure of $A$ to converge to the solution through repeated local averaging operations whose cost per iteration is $O(M^2)$ — equal to the number of unknowns.

## The Five-Point Stencil System

On the uniform grid $x_j = jh$, $y_k = kh$, $h = 1/M$, the discrete Laplacian at interior point $(j,k)$ is:

$$-\Delta_h U_{jk} = \frac{-U_{j+1,k} - U_{j-1,k} - U_{j,k+1} - U_{j,k-1} + 4U_{jk}}{h^2} = f_{jk}.$$

Rewriting:

$$U_{jk} = \frac{1}{4}\left(U_{j+1,k} + U_{j-1,k} + U_{j,k+1} + U_{j,k-1} - h^2 f_{jk}\right). \tag{Stencil}$$

This is the **mean value property** for the discrete harmonic function: each interior value equals the average of its four neighbors (plus a source correction). The equation (Stencil) is the basis for all three iterative methods.

**Structure of $A$.** Ordering the unknowns row by row: $U_{1,1}, U_{2,1}, \ldots, U_{M-1,1}, U_{1,2}, \ldots$ gives a block tridiagonal matrix

$$A = \frac{1}{h^2}\begin{pmatrix} T & -I & & \\ -I & T & -I & \\ & \ddots & \ddots & \ddots \end{pmatrix},$$

where $T$ is the $(M-1)\times(M-1)$ tridiagonal matrix $\text{tridiag}(-1, 4, -1)$ and $I$ is the identity. $A$ is symmetric positive definite with eigenvalues $\lambda_{jk} = \frac{2}{h^2}(2 - \cos(j\pi h) - \cos(k\pi h))$ for $j,k = 1,\ldots,M-1$.

The **condition number** $\kappa(A) = \lambda_{\max}/\lambda_{\min} \sim 4/(\pi h)^2 = 4M^2/\pi^2$ — direct Gaussian elimination costs $O(M^4)$ for the banded system, which is prohibitive for large $M$. Iterative methods converge in $O(M^2 \log M)$ operations for Gauss-Seidel and $O(M \log M)$ for optimal SOR.

## Jacobi Method

The **Jacobi iteration** updates all unknowns simultaneously using values from the previous iteration:

$$U_{jk}^{(m+1)} = \frac{1}{4}\left(U_{j+1,k}^{(m)} + U_{j-1,k}^{(m)} + U_{j,k+1}^{(m)} + U_{j,k-1}^{(m)}\right) + \frac{h^2}{4}f_{jk}. \tag{Jacobi}$$

In matrix form: $U^{(m+1)} = D^{-1}(L+U)U^{(m)} + D^{-1}b = B_J U^{(m)} + c_J$, where $A = D - L - U$ is the splitting into diagonal, strictly lower, and strictly upper triangular parts.

**Convergence.** The error $e^{(m)} = U^{(m)} - U^*$ satisfies $e^{(m+1)} = B_J e^{(m)}$, so $\|e^{(m)}\| \leq \rho(B_J)^m \|e^{(0)}\|$ where $\rho(B_J)$ is the spectral radius. For the five-point stencil on the square:

$$\rho(B_J) = \cos(\pi h) = 1 - \frac{\pi^2 h^2}{2} + O(h^4) \approx 1 - \frac{\pi^2}{2M^2}.$$

To reduce the error by $1/e$, one needs $m \approx 2M^2/\pi^2$ iterations — the convergence factor is $1 - O(h^2)$, so the number of iterations grows as $M^2$ as the grid is refined. Jacobi is convergent but slow.

## Gauss-Seidel Method

The **Gauss-Seidel iteration** uses the most recently computed values immediately — as each $U_{jk}$ is updated, the new value is used for all subsequent updates in the same sweep:

$$U_{jk}^{(m+1)} = \frac{1}{4}\left(U_{j+1,k}^{(m)} + U_{j-1,k}^{(m+1)} + U_{j,k+1}^{(m)} + U_{j,k-1}^{(m+1)}\right) + \frac{h^2}{4}f_{jk}, \tag{GS}$$

where $(j-1,k)$ and $(j,k-1)$ have already been updated in the current sweep (row-by-row ordering). In matrix form: $U^{(m+1)} = (D-L)^{-1}U\,U^{(m)} + (D-L)^{-1}b$.

**Convergence.** For the five-point stencil: $\rho(B_{GS}) = \rho(B_J)^2 = \cos^2(\pi h) \approx 1 - \pi^2 h^2$. Gauss-Seidel converges exactly twice as fast as Jacobi (in terms of number of iterations), but the number of iterations still grows as $M^2$. The improvement factor of 2 comes from the general result: for consistently ordered matrices (which arise from the natural ordering of the 5-point stencil), $\rho(B_{GS}) = \rho(B_J)^2$.

**Red-black ordering.** For the five-point stencil, coloring interior points alternately like a checkerboard ("red" and "black") means red points depend only on black neighbors and vice versa. One Gauss-Seidel sweep updates all red points (in parallel, since they are independent given the black points), then all black points. This **red-black Gauss-Seidel** is algorithmically parallelizable and has the same convergence rate as standard GS.

## SOR: Successive Overrelaxation

**Idea.** Gauss-Seidel gives an updated value $\tilde U_{jk}^{(m+1)}$ at each step. Instead of accepting this directly, take a weighted step:

$$U_{jk}^{(m+1)} = (1-\omega)U_{jk}^{(m)} + \omega\tilde U_{jk}^{(m+1)}, \tag{SOR}$$

where $\omega \in (1,2)$ is the **overrelaxation parameter**. For $\omega = 1$: Gauss-Seidel. For $\omega \in (1,2)$: take a longer step in the direction of the GS correction, "overshooting" to accelerate convergence.

**Convergence.** For the five-point stencil on the square:

$$\rho(B_{SOR}) = \frac{\omega - 1 + \sqrt{(\omega-1)^2 + \omega^2\rho(B_J)^2/4}}{...}$$

The optimal parameter is:

$$\omega^* = \frac{2}{1 + \sqrt{1 - \rho(B_J)^2}} = \frac{2}{1 + \sin(\pi h)}. \tag{Optimal SOR}$$

With $\omega^*$:

$$\rho(B_{SOR}^*) = \omega^* - 1 = \frac{1 - \sin(\pi h)}{1 + \sin(\pi h)} \approx 1 - 2\sin(\pi h) \approx 1 - 2\pi h = 1 - \frac{2\pi}{M}.$$

The convergence factor is now $1 - O(h)$ rather than $1 - O(h^2)$. To reduce the error by $1/e$: $m \approx M/(2\pi)$ iterations instead of $M^2/(2\pi^2)$ — a reduction by a factor of $M/\pi$. On a grid with $M = 100$: Jacobi/GS require thousands of iterations; optimal SOR requires only about 16 iterations.

**Derivation of $\omega^*$.** The spectral radius of $B_{SOR}(\omega)$ as a function of $\omega$ achieves its minimum at $\omega^*$. For consistently ordered matrices, the eigenvalues $\mu$ of $B_{SOR}(\omega)$ satisfy $(\mu + \omega - 1)^2 = \omega^2\lambda^2\mu$, where $\lambda$ are eigenvalues of $B_J$. Minimizing over $\omega$ gives (Theorem of Young, 1954):

$$\omega^* = \frac{2}{1 + \sqrt{1-\bar\lambda^2}}, \qquad \bar\lambda = \rho(B_J) = \cos(\pi h),$$

which yields (Optimal SOR) exactly.

## Worked Example: Poisson Equation on the Unit Square

**Problem.** $-\Delta u = 2\pi^2\sin(\pi x)\sin(\pi y)$ on $[0,1]^2$ with homogeneous Dirichlet conditions. Exact solution: $u(x,y) = \sin(\pi x)\sin(\pi y)$.

**Grid.** $M = 10$, $h = 0.1$, $(M-1)^2 = 81$ unknowns.

**Optimal parameter.** $\rho(B_J) = \cos(\pi/10) \approx 0.951$. Optimal $\omega^* = 2/(1+\sqrt{1-0.951^2}) = 2/(1+\sqrt{1-0.904}) = 2/(1+0.310) = 2/1.310 \approx 1.527$.

**Convergence factor.** $\rho(B_{SOR}^*) = \omega^* - 1 \approx 0.527$. To reduce the error by a factor of $10^{-6}$: need $m \geq 6\log(10)/\log(1/0.527) \approx 6\cdot 2.303/0.640 \approx 22$ iterations.

Compare: Gauss-Seidel has $\rho \approx 0.905$, requiring $6\log(10)/\log(1/0.905) \approx 140$ iterations. Jacobi needs 281. SOR with $\omega^* = 1.527$ is approximately $6\times$ faster than Gauss-Seidel on this grid.

**SOR update in code:**
```
for each interior (j,k):
    U_gs = 0.25 * (U[j+1,k] + U[j-1,k] + U[j,k+1] + U[j,k-1] + h²*f[j,k])
    U[j,k] = (1 - omega)*U[j,k] + omega*U_gs
```

The update is performed in-place (Gauss-Seidel style), so updated values are used immediately. Boundary values are fixed throughout.

## Convergence Theory

**Theorem (convergence of SOR).** For a symmetric positive definite matrix $A$: SOR converges for all $0 < \omega < 2$.

**Proof sketch.** Write $A = D - L - L^T$ (symmetric). The SOR iteration matrix is $B_{SOR} = (D-\omega L)^{-1}[(1-\omega)D + \omega L^T]$. Computing $\det(B_{SOR}) = (1-\omega)^{M^2}$ and using the fact that all eigenvalues of $B_{SOR}$ satisfy $|\mu| \leq 1$ when $A$ is SPD and $0 < \omega < 2$ (Ostrowski, 1954). For $\omega \geq 2$: $|\det(B_{SOR})| = |\omega - 1|^{M^2} > 1$, so some eigenvalue has $|\mu| > 1$.

**Stopping criterion.** Iterate until the residual $r^{(m)} = b - AU^{(m)}$ satisfies $\|r^{(m)}\|_2 / \|b\|_2 < \varepsilon$ (relative residual). The residual norm decays geometrically: $\|r^{(m)}\|_2 \leq \rho^m \|r^{(0)}\|_2$.

## Comparison of Methods

| Method | Spectral radius | Iterations for $10^{-6}$ (M=100) | Cost per iter |
|---|---|---|---|
| Jacobi | $1 - \pi^2/(2M^2) \approx 0.9995$ | $\sim 2800$ | $O(M^2)$ |
| Gauss-Seidel | $1 - \pi^2/M^2 \approx 0.9990$ | $\sim 1400$ | $O(M^2)$ |
| Optimal SOR | $1 - 2\pi/M \approx 0.937$ | $\sim 200$ | $O(M^2)$ |
| Multigrid | $\approx 0.25$ (mesh-independent) | $\sim 20$ | $O(M^2)$ |

Optimal SOR achieves $O(M \log(1/\varepsilon))$ total work — much better than $O(M^2 \log(1/\varepsilon))$ for Jacobi and GS, but still requires knowing $\omega^*$ analytically (available only for simple geometries). **Multigrid methods** (Chapter 3, Unit 8) achieve mesh-independent convergence rates with total work $O(M^2)$ per order of magnitude reduction in error — the gold standard for elliptic problems.

For general domains and variable-coefficient problems, $\omega^*$ is not available in closed form, and SOR is replaced by preconditioned Krylov methods (conjugate gradient with incomplete Cholesky or multigrid preconditioner).
