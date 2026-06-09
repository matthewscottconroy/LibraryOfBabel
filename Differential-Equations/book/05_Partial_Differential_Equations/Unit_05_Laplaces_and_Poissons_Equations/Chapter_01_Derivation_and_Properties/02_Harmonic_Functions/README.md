# Harmonic Functions

A function $u \in C^2(\Omega)$ is **harmonic** in an open set $\Omega \subset \mathbb{R}^n$ if it satisfies Laplace's equation:

$$\Delta u(\mathbf{x}) = \sum_{i=1}^n \frac{\partial^2 u}{\partial x_i^2}(\mathbf{x}) = 0, \qquad \mathbf{x} \in \Omega.$$

Harmonic functions are among the most regular objects in analysis: they are infinitely differentiable (in fact, real-analytic) wherever they are merely twice continuously differentiable. This dramatic improvement in regularity — from $C^2$ to $C^\omega$ — is a consequence of the elliptic structure of the Laplacian and distinguishes harmonic functions from general smooth functions.

## Basic Examples

**Linear functions:** Any polynomial $u = a_1 x_1 + \cdots + a_n x_n + b$ is harmonic (all second derivatives vanish).

**Quadratic harmonic polynomials:** In 2D, $u = x^2 - y^2$ is harmonic: $u_{xx} + u_{yy} = 2 - 2 = 0$. In 3D, $u = x^2 + y^2 - 2z^2$ is harmonic.

**Complex analytic functions (2D):** If $f(z) = u(x,y) + iv(x,y)$ is analytic (holomorphic) in a domain $\Omega \subset \mathbb{C}$, then both $u$ and $v$ are harmonic in $\Omega$ (they satisfy the Cauchy-Riemann equations, which imply the Laplace equation). For example, $u = \text{Re}(z^n) = r^n\cos(n\theta)$ is harmonic.

**The fundamental solution:** The most important non-polynomial harmonic function is the fundamental solution:

$$\Phi(\mathbf{x}) = \begin{cases} -\dfrac{1}{2\pi}\log|\mathbf{x}| & n = 2 \\ \dfrac{1}{n(n-2)\omega_n}|\mathbf{x}|^{2-n} & n \geq 3 \end{cases}$$

where $\omega_n = \pi^{n/2}/\Gamma(n/2+1)$ is the volume of the unit ball in $\mathbb{R}^n$. The fundamental solution is harmonic in $\mathbb{R}^n\setminus\{0\}$ and satisfies $-\Delta\Phi = \delta(\mathbf{x})$ in the distributional sense (with appropriate normalization).

In 2D: $\Phi = -\log r/(2\pi)$ (logarithmic potential). In 3D: $\Phi = 1/(4\pi r)$ — the Coulomb potential of a unit point charge.

## Spherical Harmonics as Harmonic Polynomials

In polar coordinates $(r,\theta)$ in 2D, harmonic functions separate as $u = r^n\cos(n\theta)$ and $u = r^n\sin(n\theta)$ for $n = 0, 1, 2, \ldots$ These are the real and imaginary parts of $(re^{i\theta})^n = z^n$. The "harmonic polynomials" of degree $n$ in 2D form a 2-dimensional space (for $n \geq 1$).

In 3D, in spherical coordinates $(r,\theta,\phi)$, harmonic polynomials of degree $\ell$ have the form $u = r^\ell Y_\ell^m(\theta,\phi)$ where $Y_\ell^m$ are spherical harmonics (developed in Chapter 5 of this unit). There are $2\ell+1$ linearly independent spherical harmonics of each degree $\ell$.

## Invariance Properties

**Translations:** If $u$ is harmonic in $\Omega$, then $u(\mathbf{x}-\mathbf{a})$ is harmonic in $\Omega+\mathbf{a}$.

**Rotations:** If $u$ is harmonic in $\Omega$, then $u(R\mathbf{x})$ is harmonic in $R^{-1}\Omega$ for any rotation $R$.

**Scaling (Kelvin transform):** For $n \geq 3$, the Kelvin transform $\tilde{u}(\mathbf{x}) = |\mathbf{x}|^{2-n}u(\mathbf{x}/|\mathbf{x}|^2)$ is harmonic in the inverted domain. This is a key tool for studying harmonic functions near infinity.

**Complex conjugate (2D):** The conjugate harmonic function — the harmonic conjugate of $u$ — is the function $v$ such that $f = u + iv$ is analytic. It exists locally always (by the Cauchy-Riemann equations) and globally when $\Omega$ is simply connected.

## Liouville's Theorem

**Theorem (Liouville for harmonic functions).** A bounded harmonic function on all of $\mathbb{R}^n$ is constant.

This is the PDE analogue of Liouville's theorem in complex analysis (a bounded entire function is constant). The proof uses the mean value property (Section 3): if $u$ is harmonic and bounded in $\mathbb{R}^n$, then for any two points $\mathbf{x}_1, \mathbf{x}_2$ and large $r$, the mean value property gives $u(\mathbf{x}_1) = \text{avg over }B_r(\mathbf{x}_1)$ and $u(\mathbf{x}_2) = \text{avg over }B_r(\mathbf{x}_2)$. As $r\to\infty$, both averages converge to the same limit (since $u$ is bounded and the two balls $B_r(\mathbf{x}_1)$ and $B_r(\mathbf{x}_2)$ become indistinguishable in proportion), giving $u(\mathbf{x}_1) = u(\mathbf{x}_2)$.

## Analyticity of Harmonic Functions

**Theorem (Real-Analyticity).** If $\Delta u = 0$ in $\Omega$, then $u$ is real-analytic in $\Omega$: at every point $\mathbf{x}_0 \in \Omega$, $u$ equals its Taylor series in some ball around $\mathbf{x}_0$.

This is proved using the mean value property to show that all derivatives of $u$ are bounded by $\|u\|_{L^\infty}$ with factorial-improving constants, establishing convergence of the Taylor series. The result shows that harmonic functions are vastly more regular than merely smooth: a function can be in $C^\infty$ without being real-analytic (the bump function $e^{-1/x^2}$ is $C^\infty$ but not analytic at $x=0$), but a harmonic function is always analytic.

## Green's Identities

Two integration-by-parts formulas are central to the theory of harmonic functions.

**Green's first identity:**
$$\int_\Omega u\Delta v\,d\mathbf{x} = \oint_{\partial\Omega}u\frac{\partial v}{\partial\nu}\,dS - \int_\Omega\nabla u\cdot\nabla v\,d\mathbf{x}.$$

**Green's second identity:**
$$\int_\Omega(u\Delta v - v\Delta u)\,d\mathbf{x} = \oint_{\partial\Omega}\left(u\frac{\partial v}{\partial\nu} - v\frac{\partial u}{\partial\nu}\right)dS.$$

These identities (derived from the divergence theorem) are the primary computational tools for proving uniqueness, deriving the mean value property, and constructing Green's functions.

If $u$ is harmonic in $\Omega$ and $v = 1$ in Green's first identity: $0 = \oint_{\partial\Omega}\frac{\partial u}{\partial\nu}\,dS$. This is the compatibility condition for the Neumann problem (the total flux must be zero for a harmonic function).

If $u$ and $v$ are both harmonic, Green's second identity gives $\oint_{\partial\Omega}(u\partial v/\partial\nu - v\partial u/\partial\nu)\,dS = 0$ — a reciprocity relation.
