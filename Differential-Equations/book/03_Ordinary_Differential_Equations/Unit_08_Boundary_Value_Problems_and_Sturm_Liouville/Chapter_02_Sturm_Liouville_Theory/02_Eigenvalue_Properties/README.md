# Eigenvalue Properties

The spectral theorem for regular Sturm-Liouville problems establishes that the eigenvalues form an infinite discrete set with beautiful algebraic and analytic properties. These properties — reality, simplicity, countability, and the interlacing of zeros of eigenfunctions — are the ODE analogues of familiar properties of symmetric matrices, and they are the foundation for eigenfunction expansions and their applications.

## The Spectrum is Discrete, Real, and Unbounded

**Theorem.** A regular Sturm-Liouville problem has infinitely many eigenvalues $\lambda_1 < \lambda_2 < \lambda_3 < \cdots$ (the spectrum), with $\lambda_n \to +\infty$ as $n \to \infty$. All eigenvalues are real.

**Reality.** Suppose $\lambda = \alpha + i\beta$ is an eigenvalue with eigenfunction $\phi$ (possibly complex-valued). From self-adjointness:

$$\lambda\langle\phi,\phi\rangle_w = \langle L\phi,\phi\rangle_w = \langle\phi,L\phi\rangle_w = \bar{\lambda}\langle\phi,\phi\rangle_w.$$

Since $\langle\phi,\phi\rangle_w = \int_a^b|\phi|^2 w\,dx > 0$ (the eigenfunction is nontrivial), we conclude $\lambda = \bar{\lambda}$: the eigenvalue is real.

**Boundedness below.** The Rayleigh quotient $\lambda = \int_a^b[p|\phi'|^2 + q|\phi|^2]\,dx / \int_a^b w|\phi|^2\,dx$ is bounded below (the infimum over all admissible $\phi$ exists as a finite number, achieved by the first eigenfunction). Hence all eigenvalues are bounded below.

**Discreteness and accumulation at $+\infty$.** The SL operator has compact resolvent (the inverse of $L - \mu$ for $\mu$ not an eigenvalue is a compact operator on $L^2([a,b],w\,dx)$). By the spectral theorem for compact self-adjoint operators, the spectrum is discrete (a sequence of isolated points) accumulating only at $+\infty$.

## Simplicity of Eigenvalues

**Theorem.** Each eigenvalue of a regular SL problem is simple (has geometric multiplicity one): the eigenspace corresponding to each $\lambda_n$ is one-dimensional.

**Proof.** Suppose $\phi$ and $\psi$ are two eigenfunctions for the same $\lambda_n$. They both satisfy the same second-order ODE $[py']' + (\lambda_n w - q)y = 0$. Their Wronskian satisfies Abel's formula: $[p(\phi\psi' - \phi'\psi)]' = 0$, so $p(x)(\phi\psi' - \phi'\psi) = C$ (constant). Evaluating at $x = a$ using the boundary condition $\alpha_0\phi(a) + \alpha_1\phi'(a) = 0$ (and the same for $\psi$): if $\alpha_1 \neq 0$, then $\phi'(a) = -(\alpha_0/\alpha_1)\phi(a)$ and $\psi'(a) = -(\alpha_0/\alpha_1)\psi(a)$, so $\phi(a)\psi'(a) - \phi'(a)\psi(a) = 0$. Hence $C = p(a) \cdot 0 = 0$. Thus $p(x)W[\phi,\psi] = 0$ for all $x$, and since $p > 0$, $W[\phi,\psi] = 0$ everywhere: $\phi$ and $\psi$ are linearly dependent. This means the eigenspace is at most one-dimensional; since the eigenvalue exists, it is exactly one-dimensional.

## Ordering of Eigenvalues and Oscillation Theory

Not only is the spectrum real and simple; the eigenfunctions have a beautiful oscillatory structure: the $n$-th eigenfunction $\phi_n$ has exactly $n - 1$ interior zeros in $(a,b)$.

This is Sturm's oscillation theorem. A related result is the **Sturm comparison theorem**: if $q_1(x) \geq q_2(x)$ everywhere, then the $n$-th eigenvalue of the SL problem with potential $q_1$ is at least as large as the $n$-th eigenvalue with potential $q_2$ (larger potential "stiffens" the operator and raises eigenvalues).

The oscillation theorem connects the index of the eigenvalue to the number of zeros: $\lambda_1$ is the smallest eigenvalue (associated with the eigenfunction with no interior zeros), $\lambda_2$ is the next (one interior zero), and so on. For the simplest case $y'' + \lambda y = 0$ on $[0,\pi]$ with Dirichlet BCs: $\lambda_n = n^2$ and $\phi_n = \sin(nx)$ has $n-1$ zeros in $(0,\pi)$ — precisely as predicted.

## Worked Example: Computing Eigenvalues

Find the eigenvalues and eigenfunctions of $y'' + \lambda y = 0$, $y(0) = 0$, $y'(L) = 0$.

Characteristic equation: $r^2 + \lambda = 0$. For $\lambda > 0$: $r = \pm i\sqrt{\lambda}$, general solution $y = A\cos(\sqrt{\lambda}x) + B\sin(\sqrt{\lambda}x)$.

$y(0) = 0$: $A = 0$. So $y = B\sin(\sqrt{\lambda}x)$.

$y'(L) = 0$: $B\sqrt{\lambda}\cos(\sqrt{\lambda}L) = 0$. Since $\lambda > 0$ and $B \neq 0$: $\cos(\sqrt{\lambda}L) = 0$, so $\sqrt{\lambda}L = (2n-1)\pi/2$ for $n = 1, 2, 3, \ldots$. Thus:

$$\lambda_n = \left(\frac{(2n-1)\pi}{2L}\right)^2, \qquad \phi_n(x) = \sin\!\left(\frac{(2n-1)\pi x}{2L}\right), \qquad n = 1, 2, 3, \ldots$$

The eigenfunction $\phi_n$ has $n-1$ zeros in $(0,L)$ (the zeros of $\sin((2n-1)\pi x/(2L))$ in $(0,L)$ are at $x = 2kL/(2n-1)$ for $k = 1, \ldots, n-1$), confirming Sturm's theorem.

For $\lambda = 0$: $y = A + Bx$, $y(0) = 0$ gives $A = 0$, $y'(L) = B = 0$: only the trivial solution. Not an eigenvalue.

For $\lambda < 0$: $y = Ae^{\mu x} + Be^{-\mu x}$ with $\mu = \sqrt{-\lambda} > 0$. $y(0) = A + B = 0$, so $B = -A$, $y = A(e^{\mu x} - e^{-\mu x}) = 2A\sinh(\mu x)$. $y'(L) = 2A\mu\cosh(\mu L) = 0$: since $\cosh(\mu L) > 0$ and $\mu > 0$, $A = 0$ — only the trivial solution. No negative eigenvalues.

## Asymptotics of Eigenvalues

For large $n$, the eigenvalues of a regular SL problem on $[a,b]$ with weight $w$ and leading coefficient $p$ satisfy the Weyl asymptotic formula:

$$\lambda_n \sim \frac{n^2\pi^2}{\left(\int_a^b\sqrt{w(x)/p(x)}\,dx\right)^2} \qquad \text{as } n \to \infty.$$

For constant $p = w = 1$: $\lambda_n \sim n^2\pi^2/(b-a)^2$, consistent with the Fourier case.

This asymptotic shows that the eigenvalues grow like $n^2$ for regular SL problems — a much slower growth than, say, the eigenvalues of the Laplacian in multiple dimensions ($\lambda_n \sim n^{2/d}$ in $d$ dimensions by Weyl's law).
