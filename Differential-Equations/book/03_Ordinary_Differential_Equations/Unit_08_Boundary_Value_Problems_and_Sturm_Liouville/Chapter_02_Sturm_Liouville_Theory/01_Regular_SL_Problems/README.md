# Regular Sturm-Liouville Problems

A regular Sturm-Liouville (SL) problem consists of a second-order linear eigenvalue equation on a bounded interval, with positive coefficients and separated boundary conditions. The adjective "regular" distinguishes this well-behaved case from singular SL problems, where the leading coefficient vanishes at an endpoint or the interval is unbounded. Regular SL problems have a complete and clean spectral theory, fully analogous to the spectral theorem for finite-dimensional symmetric matrices.

## Standard Form

Any second-order linear ODE $A(x)y'' + B(x)y' + C(x)y + \lambda D(x)y = 0$ can be converted to SL form by multiplying by an appropriate integrating factor. The SL form is:

$$\frac{d}{dx}[p(x)y'(x)] + [-q(x) + \lambda w(x)]y(x) = 0, \qquad a \leq x \leq b,$$

or equivalently:

$$[p(x)y']' = [q(x) - \lambda w(x)]y.$$

The functions satisfy: $p, p', q, w$ are continuous on $[a,b]$; $p(x) > 0$ on $[a,b]$; $w(x) > 0$ on $[a,b]$ ($w$ is the **weight function** or **density**). The function $q(x) \geq 0$ is the **potential** (though this non-negativity is not always assumed). The parameter $\lambda$ is the **eigenvalue**.

The separated **boundary conditions** are:

$$\alpha_0 y(a) + \alpha_1 y'(a) = 0 \qquad (\alpha_0^2 + \alpha_1^2 > 0),$$
$$\beta_0 y(b) + \beta_1 y'(b) = 0 \qquad (\beta_0^2 + \beta_1^2 > 0).$$

Dirichlet BCs correspond to $\alpha_0 = \beta_0 = 1$, $\alpha_1 = \beta_1 = 0$: $y(a) = 0$, $y(b) = 0$. Neumann BCs: $\alpha_0 = \beta_0 = 0$, $\alpha_1 = \beta_1 = 1$: $y'(a) = 0$, $y'(b) = 0$. Robin BCs allow a mix.

A value $\lambda$ is an **eigenvalue** if the BVP has a nontrivial solution (an **eigenfunction**) for that value of $\lambda$.

## Conversion to SL Form

**Example: Legendre's equation.** $(1-x^2)y'' - 2xy' + n(n+1)y = 0$ on $[-1,1]$. This is $([(1-x^2)y']')' + n(n+1)y = 0$, i.e., $[p(x)y']' + \lambda w(x)y = 0$ with $p(x) = 1-x^2$, $w(x) = 1$, $q(x) = 0$, $\lambda = n(n+1)$. Note $p(\pm 1) = 0$: this is actually a singular SL problem. The regular case would require $p > 0$ on a closed interval.

**Example: Regular SL.** $y'' + \lambda y = 0$ on $[0, L]$ with $y(0) = 0$, $y(L) = 0$. This is already in SL form with $p = 1$, $w = 1$, $q = 0$. Eigenvalues: $\lambda_n = (n\pi/L)^2$, eigenfunctions $\phi_n = \sin(n\pi x/L)$. The classical Fourier sine series.

## The Self-Adjoint Framework

The SL operator is $L = -\frac{1}{w(x)}\left[\frac{d}{dx}\left(p(x)\frac{d}{dx}\right) - q(x)\right]$. It acts on functions satisfying the boundary conditions. The **weighted inner product** on $[a,b]$ is:

$$\langle f, g\rangle_w = \int_a^b f(x)\overline{g(x)}w(x)\,dx.$$

**Claim:** $L$ is self-adjoint: $\langle Lf, g\rangle_w = \langle f, Lg\rangle_w$ for all $f, g$ in the domain.

**Proof.** Compute $\langle Lf, g\rangle_w - \langle f, Lg\rangle_w$: using integration by parts twice (the Lagrange identity),

$$\int_a^b \{g[pf']' - f[pg']'\}dx = [p(fg' - gf')]_a^b.$$

The boundary term $[p(fg' - gf')]_a^b$ vanishes when both $f$ and $g$ satisfy the same separated boundary conditions. (Each BC is $\alpha_0 u + \alpha_1 u' = 0$; at each endpoint, $fg' - gf' = 0$ follows from this condition — if $\alpha_1 \neq 0$, then $f' = -(\alpha_0/\alpha_1)f$ and $g' = -(\alpha_0/\alpha_1)g$, so $fg' - gf' = 0$. Similarly for $\alpha_1 = 0$.) Therefore $\langle Lf,g\rangle_w = \langle f, Lg\rangle_w$, confirming self-adjointness.

Self-adjointness is the key algebraic property from which all eigenvalue properties follow.

## Reduction to SL Form

To convert $Ay'' + By' + (C + \lambda D)y = 0$ to SL form, multiply by $\mu(x) = \frac{1}{A(x)}\exp\!\int^x \frac{B(t)}{A(t)}\,dt$. Then:

$$p(x) = A(x)\mu(x) = \exp\!\int^x\!\frac{B}{A}\,dt, \quad w(x) = D(x)\mu(x), \quad q(x) = -C(x)\mu(x).$$

**Example.** The equation $y'' + 2xy' + \lambda y = 0$ (related to Hermite): $A = 1$, $B = 2x$. Integrating factor $\mu = e^{x^2}$. SL form: $(e^{x^2}y')' + \lambda e^{x^2}y = 0$. Weight $w = e^{x^2}$ (on an appropriate interval).

## The Rayleigh Quotient

For the SL problem, the eigenvalues can be characterized variationally. For a function $y$ satisfying the BCs:

$$\lambda = \frac{\langle Ly, y\rangle_w}{\langle y, y\rangle_w} = \frac{\int_a^b [p(y')^2 + qy^2]\,dx - [py y']_a^b}{\int_a^b w y^2\,dx}.$$

For Dirichlet BCs (where $[pyy']_a^b = 0$):

$$\lambda = \frac{\int_a^b [p(y')^2 + qy^2]\,dx}{\int_a^b wy^2\,dx}.$$

This **Rayleigh quotient** shows immediately that all eigenvalues are positive if $p, q, w > 0$ on $[a,b]$. More generally, eigenvalues are bounded below (the minimum value is the smallest eigenvalue, achieved by the first eigenfunction), a fact that can be proved using the theory of compact self-adjoint operators.

The Rayleigh quotient is also the basis for the **Rayleigh-Ritz method**: approximating eigenvalues variationally by minimizing over a finite-dimensional subspace, without solving the ODE explicitly.
