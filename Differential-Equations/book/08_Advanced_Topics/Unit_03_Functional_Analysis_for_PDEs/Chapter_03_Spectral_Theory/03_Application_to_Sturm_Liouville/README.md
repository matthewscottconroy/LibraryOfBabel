# Application to Sturm-Liouville Theory

The Sturm-Liouville problem is the canonical second-order eigenvalue problem in classical analysis:

$$-(p(x)u')' + q(x)u = \lambda w(x)u, \quad x \in (a,b),$$

with boundary conditions at $x = a$ and $x = b$. It arises in separation of variables for PDEs on domains with symmetry (spherical harmonics, Bessel functions, Legendre polynomials are all solutions of Sturm-Liouville problems), and it is the prototypical example of a self-adjoint unbounded operator with discrete spectrum. The abstract spectral theory of the preceding section, applied to the Sturm-Liouville operator, yields the eigenfunction expansion theorems that underlie much of classical mathematical physics.

## The Sturm-Liouville Operator

Let $p, p', q, w$ be continuous on $[a,b]$ with $p(x) > 0$ and $w(x) > 0$. Define the **Sturm-Liouville operator**:

$$Lf = \frac{1}{w(x)}\left[-(p(x)f')' + q(x)f\right].$$

The eigenvalue problem is $Lu = \lambda u$, i.e., $-(pu')' + qu = \lambda wu$.

## Boundary Conditions and Self-Adjointness

Impose **separated boundary conditions**:

$$\alpha_1 u(a) + \alpha_2 u'(a) = 0, \quad \beta_1 u(b) + \beta_2 u'(b) = 0,$$

with $\alpha_1^2 + \alpha_2^2 > 0$ and $\beta_1^2 + \beta_2^2 > 0$. On the Hilbert space $H = L^2_w((a,b))$ with weighted inner product $\langle f, g\rangle_w = \int_a^b f(x)g(x)w(x) \, dx$, the operator $L$ with the above boundary conditions is:

**Claim:** $L$ is symmetric. For $f, g \in \mathcal{D}(L) = \{f \in C^2 : f \text{ satisfies the BCs}\}$:

$$\langle Lf, g\rangle_w - \langle f, Lg\rangle_w = \int_a^b \left[-(pf')'g + q fg + (pg')'f - qfg\right] dx = \int_a^b \left[-(pf')'g + (pg')'f\right] dx$$
$$= [p(g f' - f g')]_a^b.$$

The boundary term $[p(gf' - fg')]_a^b$ vanishes for functions satisfying the separated boundary conditions (by direct verification using $\alpha_1 u(a) = -\alpha_2 u'(a)$ etc.). So $L$ is symmetric. With appropriate domain extensions, $L$ is self-adjoint.

## Spectral Theorem for Sturm-Liouville

**Theorem (Sturm-Liouville).** Under the above conditions, the operator $L$ on $L^2_w((a,b))$ with separated boundary conditions has the following properties:

1. **Discrete spectrum.** There are countably many eigenvalues $\lambda_1 < \lambda_2 < \cdots \to +\infty$, all real and simple.

2. **Orthogonal eigenfunctions.** The eigenfunctions $\phi_n$ (where $L\phi_n = \lambda_n\phi_n$) are orthogonal in $L^2_w$: $\langle \phi_n, \phi_m\rangle_w = \delta_{nm}$ (normalizing appropriately).

3. **Completeness.** The eigenfunctions form a complete orthonormal basis for $L^2_w((a,b))$: every $f \in L^2_w$ has an expansion

$$f = \sum_{n=1}^\infty c_n \phi_n, \quad c_n = \langle f, \phi_n\rangle_w, \quad \|f\|_{L^2_w}^2 = \sum_{n=1}^\infty c_n^2.$$

4. **Oscillation theorem.** The $n$-th eigenfunction $\phi_n$ has exactly $n-1$ zeros in $(a,b)$.

**Proof via the compact resolvent.** The Green's function $G(x,t)$ of $L$ (with homogeneous boundary conditions) gives the resolvent operator $(L - \mu)^{-1}f(x) = \int_a^b G(x,t)f(t)w(t) \, dt$. The kernel $G$ is symmetric and square-integrable on $(a,b) \times (a,b)$ (a Hilbert-Schmidt kernel), so the resolvent is a compact self-adjoint operator on $L^2_w$. By the spectral theorem for compact self-adjoint operators, it has discrete eigenvalues and complete orthonormal eigenfunctions. $\square$

## Examples

**The trigonometric case.** $-(u'') = \lambda u$ on $[0,L]$ with $u(0) = u(L) = 0$: eigenvalues $\lambda_n = (n\pi/L)^2$, eigenfunctions $\phi_n(x) = \sqrt{2/L}\sin(n\pi x/L)$. The eigenfunction expansion is the Fourier sine series.

**Legendre's equation.** $(-(1-x^2)u')' = \lambda u$ on $[-1,1]$ with boundedness at $\pm 1$ (no explicit boundary condition needed, since $p(x) = 1-x^2$ vanishes at the endpoints—a "singular" Sturm-Liouville problem): eigenvalues $\lambda_n = n(n+1)$, eigenfunctions are the Legendre polynomials $P_n(x)$. The expansion is in Legendre series.

**Bessel's equation (singular).** $-(xu')'/x + m^2u/x = \lambda u$ on $[0,1]$ with $u(0)$ bounded and $u(1) = 0$ (or other BC): eigenvalues are squares of zeros of Bessel functions $J_m$, eigenfunctions are $J_m(\sqrt{\lambda_n} x)$. The expansion is in Fourier-Bessel series.

## Connection to Separation of Variables

Sturm-Liouville problems arise naturally when separating variables in PDEs on domains with symmetry. For example, the Laplacian in polar coordinates on a disc $\{r \leq R\}$ leads (via $u = R(r)\Theta(\theta)$) to:

- For $\Theta$: the periodic eigenvalue problem on $[0, 2\pi]$, with eigenfunctions $\{1, \cos n\theta, \sin n\theta\}$.
- For $R$: Bessel's equation, with eigenfunctions $J_n(\alpha_{n,k} r/R)$ where $\alpha_{n,k}$ are zeros of $J_n$.

The Sturm-Liouville theorem guarantees completeness in each factor, and the product completeness gives the full eigenfunction expansion for the Laplacian on the disc.

## Eigenfunction Expansion for the Heat Equation

Given the Sturm-Liouville expansion with eigenfunctions $\{\phi_n\}$ and eigenvalues $\{\lambda_n\}$, the heat equation $\partial_t u = Lu$ on $(a,b)$ (with the same boundary conditions) is solved by:

$$u(x,t) = \sum_{n=1}^\infty c_n e^{-\lambda_n t} \phi_n(x), \quad c_n = \langle u_0, \phi_n\rangle_w,$$

where $u_0(x) = u(x,0)$ is the initial condition. Convergence is guaranteed by the $e^{-\lambda_n t}$ decay for $t > 0$: even if $u_0 \in L^2_w$ only (not smooth), $u(\cdot,t)$ is smooth for every $t > 0$ (since $\sum c_n e^{-\lambda_n t}\phi_n$ converges in $C^\infty$ for $t > 0$, as the $e^{-\lambda_n t}$ decay dominates any polynomial growth of $\|\phi_n\|_{C^k}$). This is the smoothing effect of the heat equation, recovered here via the spectral theorem.

## Physical Interpretation

The eigenvalues $\lambda_n$ are the **normal mode frequencies** (or their squares) of the system governed by the Sturm-Liouville operator. The eigenfunctions $\phi_n$ are the **normal modes** or **standing waves**. The general solution is a superposition of normal modes, each decaying (for the heat equation) or oscillating (for the wave equation) at its characteristic rate $\lambda_n$. The completeness of the eigenfunctions guarantees that any initial configuration can be decomposed into normal modes, and the orthogonality allows the coefficients $c_n$ to be computed by projection.

This spectral picture—decomposing dynamics into independent normal modes—is the conceptual foundation of Fourier analysis, quantum mechanics (energy eigenstates), and the theory of vibrations of continuous media.
