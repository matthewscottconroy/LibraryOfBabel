# Singular Endpoints

A Sturm-Liouville endpoint is **singular** if the leading coefficient $p(x)$ vanishes there, the weight function $w(x)$ vanishes or is unbounded, the interval is infinite (semi-infinite or doubly infinite), or the coefficient functions are not continuous at the endpoint. The theory of singular endpoints is substantially more complex than the regular theory, because the standard separated boundary conditions may not be well-defined (if $p(a) = 0$, then $p(a)y'(a)$ is always zero and gives no information), and the appropriate replacement conditions require understanding the asymptotic behavior of solutions near the singular point.

## Weyl's Classification: Limit-Point and Limit-Circle

The fundamental classification of singular endpoints was given by Hermann Weyl in 1910. Consider the SL equation $[py']' + (\lambda w - q)y = 0$ on $(a,b]$ where $x = a$ is a singular endpoint (e.g., $p(a) = 0$ or $a = -\infty$).

For any $\lambda \in \mathbb{C}$ with $\text{Im}(\lambda) \neq 0$, the SL equation has two linearly independent solutions. Weyl proved:

**Limit-circle (LC) case:** Both linearly independent solutions belong to $L^2([a,c], w\,dx)$ for every $c \in (a,b)$. In this case, boundary conditions must be imposed at the singular endpoint $x = a$ to make the operator self-adjoint.

**Limit-point (LP) case:** Exactly one linearly independent solution belongs to $L^2([a,c], w\,dx)$. In this case, no boundary condition is needed (or possible) at $x = a$: the $L^2$ condition itself selects the unique square-integrable solution.

The classification is independent of $\lambda$ (as long as $\text{Im}(\lambda) \neq 0$): the LC or LP character is a property of the equation at the endpoint, not of the specific eigenvalue.

**Physical meaning.** The limit-point condition means the equation is "strong enough" at the singular endpoint to automatically select the physically meaningful solution (the one that is square-integrable, and hence has finite energy). No additional boundary condition is needed or can be imposed. The limit-circle case is more degenerate: both solutions are square-integrable and additional specification is needed.

## Criteria for Limit-Point Behavior

Several sufficient conditions for the limit-point case at $x = a$ are known:

If $\int_a^c [1/p(x)]\,dx = +\infty$ (the endpoint is far away in the metric of the ODE): limit-point. This includes $p(a) = 0$ with $1/p$ non-integrable.

If $q(x) \geq c_1/x^2$ for some $c_1 > 1/4$ (strong potential): limit-point.

For the standard singular SL problems of mathematical physics, the limit-point condition typically holds at singular endpoints, and the boundary condition is simply square-integrability (or equivalently, boundedness, which is equivalent to square-integrability at a regular singular point where the solutions behave as powers of $x - a$).

## Regular Singular Points and Frobenius Solutions

When $x = a$ is a regular singular point (in the sense of the Frobenius theory), the two Frobenius solutions behave like $(x-a)^{r_1}$ and $(x-a)^{r_2}$ (or with a logarithm, if $r_1 - r_2$ is an integer). The square-integrability condition with weight $w$ selects the solution with the larger exponent (the one that goes to zero faster, or at least doesn't blow up):

For weight $w = (x-a)^\alpha$ (typical near a regular singular point): $\phi_1 = (x-a)^{r_1}$ is in $L^2([a,c], w\,dx)$ iff $2r_1 + \alpha + 1 > 0$, i.e., $r_1 > -(1+\alpha)/2$.

If both solutions satisfy the square-integrability condition (both exponents $r_1, r_2$ give $L^2$ functions), we are in the limit-circle case. If only one does, we are in the limit-point case, and that solution is automatically selected.

## Discrete and Continuous Spectra

For regular SL problems, the spectrum is entirely discrete (a sequence of eigenvalues accumulating at $+\infty$). For singular SL problems, the spectrum may have a continuous component. The nature of the spectrum depends on the behavior at the singular endpoints:

If the interval is finite and both endpoints are limit-circle (or one is regular and one is limit-circle): typically discrete spectrum.

If the interval is semi-infinite or doubly infinite: typically a combination of discrete spectrum (at most countably many eigenvalues) and continuous spectrum. The eigenfunctions for the continuous spectrum are not square-integrable (they don't live in $L^2$) but appear in the expansion as improper integrals rather than sums (the Fourier transform is the canonical example).

## Examples

**Legendre equation on $(-1,1)$.** The SL form is $[(1-x^2)y']' + \lambda y = 0$, with $p(x) = 1-x^2$, $w = 1$, on $(-1,1)$. Both endpoints $x = \pm 1$ are regular singular points: $p(\pm 1) = 0$. The indicial exponents at $x = 1$ are $r = 0$ (bounded solution) and $r = 1$ (another bounded solution — but the Legendre function of the second kind $Q_n$ actually has a logarithmic singularity). The regular solution (bounded at $\pm 1$) is the Legendre polynomial $P_n$. The spectrum is $\lambda_n = n(n+1)$, $n = 0,1,2,\ldots$ — entirely discrete.

**Bessel equation on $(0,R)$.** $(xy')' + (\lambda x - \nu^2/x)y = 0$, with $p(x) = x$, $w(x) = x$, singular at $x = 0$. The indicial exponents are $\pm\nu$. For $\nu \geq 0$, the solution $J_\nu(\sqrt{\lambda}x)$ is bounded at $x=0$; the other solution $Y_\nu(\sqrt{\lambda}x)$ blows up. Square-integrability selects $J_\nu$. With a Dirichlet condition at $x = R$, the spectrum is the set of $\lambda$ such that $J_\nu(\sqrt{\lambda}R) = 0$ — discrete.

**Hermite equation on $(-\infty,+\infty)$.** $y'' - 2xy' + \lambda y = 0$, with SL weight $w = e^{-x^2}$, on $\mathbb{R}$. The square-integrability condition with weight $e^{-x^2}$ selects, for each integer $n \geq 0$, the Hermite polynomial $H_n(x)$ with $\lambda = 2n$. All other solutions grow faster than any Gaussian and are excluded. The spectrum $\{0, 2, 4, 6, \ldots\}$ is discrete; the eigenfunctions $H_n e^{-x^2/2}$ form a complete orthonormal set in $L^2(\mathbb{R})$.

## The Weyl-Titchmarsh Theory

For more general singular SL problems, the Weyl-Titchmarsh $m$-function provides a comprehensive framework for the spectral theory. The $m$-function is a Herglotz (Nevanlinna) function encoding the spectral measure (both discrete and continuous parts) of the SL operator. The spectral expansion theorem in the general case takes the form of an integral transform (generalized Fourier transform) whose measure is the spectral measure of the operator. For discrete spectrum, this reduces to the eigenfunction expansion; for continuous spectrum, it reduces to an integral (such as the Fourier integral for the SL problem on the whole line with $p = w = 1$, $q = 0$).

This framework, though beyond the scope of an introductory ODE course, is the foundation of the spectral theory of Schrödinger operators in quantum mechanics and of inverse scattering theory.
