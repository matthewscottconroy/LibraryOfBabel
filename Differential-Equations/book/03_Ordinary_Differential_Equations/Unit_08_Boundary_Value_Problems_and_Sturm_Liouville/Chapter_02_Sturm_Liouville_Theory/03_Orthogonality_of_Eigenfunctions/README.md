# Orthogonality of Eigenfunctions

Orthogonality of eigenfunctions is the central algebraic property of Sturm-Liouville theory. It is the rigorous foundation for generalized Fourier series: just as an arbitrary vector in $\mathbb{R}^n$ can be expanded in an orthonormal basis, an arbitrary function in $L^2([a,b],w\,dx)$ can be expanded in the orthonormal eigenfunctions of a Sturm-Liouville operator. Orthogonality makes the computation of expansion coefficients straightforward — no system of equations needs to be solved.

## The Orthogonality Theorem

**Theorem.** Let $\phi_m$ and $\phi_n$ be eigenfunctions of a regular Sturm-Liouville problem corresponding to distinct eigenvalues $\lambda_m \neq \lambda_n$. Then $\phi_m$ and $\phi_n$ are orthogonal with respect to the weight function $w(x)$:

$$\langle\phi_m, \phi_n\rangle_w = \int_a^b \phi_m(x)\phi_n(x)w(x)\,dx = 0.$$

**Proof.** Since $L\phi_m = \lambda_m\phi_m$ and $L\phi_n = \lambda_n\phi_n$, and $L$ is self-adjoint:

$$\lambda_m\langle\phi_m,\phi_n\rangle_w = \langle L\phi_m,\phi_n\rangle_w = \langle\phi_m,L\phi_n\rangle_w = \lambda_n\langle\phi_m,\phi_n\rangle_w.$$

Therefore $(\lambda_m - \lambda_n)\langle\phi_m,\phi_n\rangle_w = 0$. Since $\lambda_m \neq \lambda_n$, we conclude $\langle\phi_m,\phi_n\rangle_w = 0$.

This is the same proof as for the orthogonality of eigenvectors of a symmetric matrix. The key ingredient is self-adjointness; the weight function $w$ enters through the inner product $\langle\cdot,\cdot\rangle_w$.

## Normalization

Eigenfunctions can be normalized to have unit norm. Define $\|\phi_n\|_w = \sqrt{\langle\phi_n,\phi_n\rangle_w} = \sqrt{\int_a^b \phi_n^2 w\,dx}$. The normalized eigenfunction is $\hat{\phi}_n = \phi_n/\|\phi_n\|_w$. The normalized eigenfunctions satisfy $\langle\hat{\phi}_m,\hat{\phi}_n\rangle_w = \delta_{mn}$ (the Kronecker delta): they are orthonormal.

Computing $\|\phi_n\|_w^2 = \int_a^b\phi_n(x)^2 w(x)\,dx$ is a standard integration, often facilitated by reduction formulas or recurrence relations.

## Examples of Orthogonality Relations

**Fourier sine series:** Eigenfunctions of $y'' + \lambda y = 0$, $y(0) = y(L) = 0$ are $\phi_n = \sin(n\pi x/L)$, $n = 1, 2, 3, \ldots$, with $w = 1$. Orthogonality:

$$\int_0^L \sin\!\left(\frac{m\pi x}{L}\right)\sin\!\left(\frac{n\pi x}{L}\right)dx = \frac{L}{2}\delta_{mn}.$$

The norm is $\|\phi_n\|^2 = L/2$ for all $n$.

**Fourier cosine series:** Eigenfunctions of $y'' + \lambda y = 0$, $y'(0) = y'(L) = 0$ are $\phi_0 = 1$ (for $\lambda_0 = 0$) and $\phi_n = \cos(n\pi x/L)$ ($n \geq 1$), with $w = 1$. Orthogonality:

$$\int_0^L\cos\!\left(\frac{m\pi x}{L}\right)\cos\!\left(\frac{n\pi x}{L}\right)dx = \begin{cases}L, & m = n = 0, \\ L/2, & m = n \geq 1, \\ 0, & m \neq n.\end{cases}$$

**Legendre polynomials:** $P_n$ and $P_m$ for $m \neq n$ satisfy $\int_{-1}^1 P_m(x)P_n(x)\,dx = 0$, with $\int_{-1}^1[P_n(x)]^2\,dx = 2/(2n+1)$.

**Bessel functions:** $J_\nu(\alpha_{n,\nu}x)$ for different zeros $\alpha_{n,\nu}$ satisfy $\int_0^1 J_\nu(\alpha_{m,\nu}x)J_\nu(\alpha_{n,\nu}x)x\,dx = 0$ for $m \neq n$ (weight $w = x$).

## The Lagrange Identity and Green's Formula

The proof of orthogonality uses the **Lagrange identity**:

$$v[pu']' - u[pv']' = [p(vu' - uv')]'.$$

Integrating:

$$\int_a^b \{v[pu']' - u[pv']'\}\,dx = [p(vu' - uv')]_a^b.$$

This is **Green's formula** for the SL operator. When $u = \phi_m$ (with $L\phi_m = \lambda_m\phi_m$) and $v = \phi_n$ (with $L\phi_n = \lambda_n\phi_n$), the integrand becomes $(\lambda_m - \lambda_n)uw\phi_n\phi_m$ (after dividing by $w$, the SL eigenvalue equations give $[p\phi_m']' = (q - \lambda_m w)\phi_m$). The boundary term vanishes by the separated BCs. The result is orthogonality.

Green's formula is the bilinear form associated with the SL operator and is the integral-operator analogue of the symmetry condition for matrices. It shows that the source of orthogonality is both the self-adjointness of the operator and the boundary conditions — both must hold.

## Weighted Orthogonality and Physical Meaning

The weight function $w(x)$ in the orthogonality relation $\int_a^b\phi_m\phi_n w\,dx = 0$ has a physical meaning. In many applications, $w(x)$ represents a density: the mass per unit length of a string, the charge density in an electromagnetic problem, or the probability density in quantum mechanics. The weighted inner product $\langle f,g\rangle_w$ is then the physically relevant inner product, and orthogonality with respect to $w$ is what matters for expanding physical quantities.

For example, in the heat equation with a spatially varying thermal diffusivity $\kappa(x)$, the relevant SL problem has weight $w(x) = 1/\kappa(x)$, and the orthogonality relation $\int_a^b\phi_m\phi_n/\kappa\,dx = 0$ is what allows the temperature to be expanded in a convergent eigenfunction series.

## Gram-Schmidt and the Necessity of Orthogonality

If two eigenfunctions for the same eigenvalue exist (a degenerate eigenvalue), they are not automatically orthogonal, but they can be orthogonalized by the Gram-Schmidt process. For regular SL problems, eigenvalues are simple (each eigenspace is one-dimensional), so degeneracy does not occur. For singular SL problems or higher-dimensional analogues (the Laplacian on a disk, for instance), degeneracy does occur and Gram-Schmidt orthogonalization is needed.

The orthogonality of eigenfunctions is what makes SL theory applicable: without it, the expansion coefficients would be coupled, and computing the eigenfunction expansion would require solving an infinite system of equations. Orthogonality decouples the problem completely: each coefficient $c_n$ is determined independently by $c_n = \langle f, \phi_n\rangle_w / \langle\phi_n,\phi_n\rangle_w$, a single integration.
