# Completeness and Eigenfunction Expansions

Orthogonality of eigenfunctions allows one to compute expansion coefficients cleanly, but it does not by itself guarantee that an arbitrary function can be recovered from its expansion. **Completeness** is the additional property that the eigenfunctions span the entire space — that the eigenfunction expansion of a function actually converges back to that function. Completeness is the ODE analogue of the statement that an orthonormal basis for $\mathbb{R}^n$ spans all of $\mathbb{R}^n$, and it is what makes Sturm-Liouville theory applicable to solving PDEs by separation of variables.

## Completeness Theorem

**Theorem.** The normalized eigenfunctions $\{\hat{\phi}_n\}_{n=1}^\infty$ of a regular Sturm-Liouville problem form a complete orthonormal set in $L^2([a,b], w\,dx)$: every $f \in L^2([a,b], w\,dx)$ can be expanded as:

$$f(x) = \sum_{n=1}^\infty c_n \hat{\phi}_n(x), \qquad c_n = \langle f, \hat{\phi}_n\rangle_w = \int_a^b f(x)\hat{\phi}_n(x)w(x)\,dx,$$

with convergence in the $L^2$ sense: $\|f - \sum_{n=1}^N c_n\hat{\phi}_n\|_w \to 0$ as $N \to \infty$.

The proof of completeness is deeper than the proof of orthogonality. It uses the theory of compact self-adjoint operators on Hilbert spaces: the SL operator has compact resolvent (its inverse, the Green's function operator, is a compact operator), and by the spectral theorem for compact self-adjoint operators, the eigenfunctions of a compact self-adjoint operator form a complete orthonormal basis. The SL problem satisfies these conditions when $p > 0$ and $w > 0$ on a compact interval $[a,b]$.

Alternatively, one can appeal to the **Parseval identity** (or Bessel's equality), which states that completeness is equivalent to $\sum_{n=1}^\infty |c_n|^2 = \|f\|_w^2$ for all $f \in L^2$. This can be proved for SL eigenfunctions using the theory of Green's functions.

## Computing Expansion Coefficients

The expansion coefficient $c_n$ is:

$$c_n = \langle f, \hat{\phi}_n\rangle_w = \frac{1}{\|\phi_n\|_w^2}\int_a^b f(x)\phi_n(x)w(x)\,dx.$$

These are the **generalized Fourier coefficients** of $f$ with respect to the SL eigenfunctions. They are the weighted projections of $f$ onto each eigenfunction direction. No system of equations needs to be solved — each $c_n$ is determined by a single integral, independently of all others, because of orthogonality.

## Convergence

While the $L^2$ convergence of the eigenfunction expansion always holds, pointwise convergence requires additional smoothness. The standard result is:

**Theorem.** If $f$ is piecewise smooth on $[a,b]$ and satisfies the boundary conditions, then the eigenfunction expansion converges pointwise to $f(x)$ at every continuity point of $f$, and to $[f(x^+)+f(x^-)]/2$ at each jump discontinuity.

This is exactly analogous to the Dirichlet theorem for Fourier series, which is the special case $p = w = 1$, $q = 0$, Dirichlet BCs on $[0,\pi]$.

## The Fourier Sine Series

The eigenfunction expansion for $y'' + \lambda y = 0$, $y(0) = y(L) = 0$ is the Fourier sine series. For $f \in L^2([0,L])$:

$$f(x) = \sum_{n=1}^\infty b_n\sin\!\left(\frac{n\pi x}{L}\right), \qquad b_n = \frac{2}{L}\int_0^L f(x)\sin\!\left(\frac{n\pi x}{L}\right)dx.$$

Completeness of the Fourier sine series (the fact that any $L^2$ function has a convergent Fourier series representation) is a deep result, proved by Parseval's theorem for Fourier series.

## Application to PDEs: Heat Equation

The paradigmatic application of SL eigenfunction expansions is the heat equation $u_t = \kappa u_{xx}$ on $[0,L]$ with $u(0,t) = u(L,t) = 0$ and initial condition $u(x,0) = f(x)$.

The method of separation of variables writes $u(x,t) = X(x)T(t)$, giving $X'' + \lambda X = 0$ (the SL problem with Dirichlet BCs) and $T' = -\kappa\lambda T$. The eigenvalues are $\lambda_n = (n\pi/L)^2$ with eigenfunctions $X_n = \sin(n\pi x/L)$, and $T_n(t) = e^{-\kappa(n\pi/L)^2 t}$.

The general solution is:

$$u(x,t) = \sum_{n=1}^\infty b_n\sin\!\left(\frac{n\pi x}{L}\right)e^{-\kappa(n\pi/L)^2 t}.$$

The initial condition $u(x,0) = f(x)$ requires $\sum b_n\sin(n\pi x/L) = f(x)$ — exactly the Fourier sine series of $f$. Completeness guarantees this is achievable for any $f \in L^2([0,L])$.

The eigenfunction expansion works because: (1) the eigenfunctions solve the boundary conditions automatically; (2) they diagonalize the spatial differential operator; (3) completeness guarantees the initial condition can be satisfied; and (4) the time evolution decouples into independent exponential decay for each mode.

## Nonhomogeneous Problems and the Eigenfunction Method

The eigenfunction expansion also solves nonhomogeneous BVPs. For $Ly = f$ with $Lx\phi_n = \lambda_n\phi_n$, expand $y = \sum c_n\phi_n$ and $f = \sum d_n\phi_n$ (with $d_n = \langle f,\phi_n\rangle_w/\|\phi_n\|_w^2$). Substituting into $Ly = f$:

$$\sum c_n L\phi_n = \sum c_n\lambda_n\phi_n = \sum d_n\phi_n.$$

By orthogonality and linear independence, $c_n = d_n/\lambda_n$ (provided $\lambda_n \neq 0$). The solution is:

$$y(x) = \sum_{n=1}^\infty \frac{d_n}{\lambda_n}\phi_n(x) = \sum_{n=1}^\infty\frac{\langle f,\phi_n\rangle_w}{\lambda_n\|\phi_n\|_w^2}\phi_n(x).$$

Comparing with the Green's function formula $y(x) = \int_a^b G(x,\xi)f(\xi)\,d\xi$ gives the spectral expansion of the Green's function:

$$G(x,\xi) = \sum_{n=1}^\infty\frac{\phi_n(x)\phi_n(\xi)}{\lambda_n\|\phi_n\|_w^2}.$$

If $\lambda_k = 0$ for some $k$ (i.e., $\lambda = 0$ is an eigenvalue), then $c_k$ is undetermined and the nonhomogeneous problem $Ly = f$ has a solution only if $d_k = \langle f,\phi_k\rangle_w = 0$ — the Fredholm alternative, now visible explicitly in the eigenfunction expansion.

## Parseval's Identity

Completeness is equivalent to Parseval's identity:

$$\|f\|_w^2 = \int_a^b |f(x)|^2 w(x)\,dx = \sum_{n=1}^\infty |c_n|^2\|\phi_n\|_w^2 = \sum_{n=1}^\infty |c_n|^2.$$

(In the last equality, using normalized eigenfunctions.) This states that the "energy" (weighted $L^2$ norm) of $f$ equals the sum of squares of the eigenfunction coefficients — analogous to the Pythagorean theorem in infinite dimensions. Completeness is exactly the statement that no "energy" is lost in the expansion: the eigenfunctions capture all of $f$, not just part of it.
