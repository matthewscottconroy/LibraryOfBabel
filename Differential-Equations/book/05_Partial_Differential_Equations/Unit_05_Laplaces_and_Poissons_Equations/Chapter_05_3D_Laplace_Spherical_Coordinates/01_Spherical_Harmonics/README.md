# Spherical Harmonics

Spherical harmonics are the angular eigenfunctions of the Laplacian on the sphere $S^2 = \{|\mathbf{x}|=1\}\subset\mathbb{R}^3$. They are the natural basis functions for representing functions on a sphere — analogous to the Fourier modes $e^{in\theta}$ on the circle $S^1$. Every square-integrable function on $S^2$ has a unique expansion in spherical harmonics, and this expansion diagonalizes the Laplacian, making it the essential tool for solving Laplace's equation and the wave equation in spherical geometry.

## Definition

The **spherical harmonics** $Y_\ell^m(\theta,\phi)$ are defined for $\ell = 0, 1, 2, \ldots$ and $m = -\ell, -\ell+1, \ldots, \ell$ by:

$$Y_\ell^m(\theta,\phi) = (-1)^m\sqrt{\frac{(2\ell+1)}{4\pi}\frac{(\ell-|m|)!}{(\ell+|m|)!}}\,P_\ell^{|m|}(\cos\theta)\,e^{im\phi},$$

where $P_\ell^m$ are the associated Legendre polynomials. The factor $(-1)^m$ (Condon-Shortley convention) is standard in physics; some mathematics texts omit it.

The first few spherical harmonics:

$$Y_0^0 = \frac{1}{\sqrt{4\pi}}, \qquad Y_1^0 = \sqrt{\frac{3}{4\pi}}\cos\theta, \qquad Y_1^{\pm1} = \mp\sqrt{\frac{3}{8\pi}}\sin\theta\,e^{\pm i\phi},$$

$$Y_2^0 = \sqrt{\frac{5}{16\pi}}(3\cos^2\theta-1), \qquad Y_2^{\pm1} = \mp\sqrt{\frac{15}{8\pi}}\sin\theta\cos\theta\,e^{\pm i\phi}, \qquad Y_2^{\pm2} = \sqrt{\frac{15}{32\pi}}\sin^2\theta\,e^{\pm2i\phi}.$$

## Eigenvalue Equation

The spherical harmonics are eigenfunctions of the Laplace-Beltrami operator on $S^2$:

$$\Delta_{S^2}Y_\ell^m = -\ell(\ell+1)Y_\ell^m,$$

where $\Delta_{S^2} = \frac{1}{\sin\theta}\frac{\partial}{\partial\theta}\!\left(\sin\theta\frac{\partial}{\partial\theta}\right) + \frac{1}{\sin^2\theta}\frac{\partial^2}{\partial\phi^2}$ is the Laplace-Beltrami operator.

This eigenvalue equation is equivalent to: $Y_\ell^m$ restricted to the sphere $|\mathbf{x}|=1$ is the angular part of a harmonic polynomial of degree $\ell$ in $\mathbb{R}^3$.

## Orthonormality and Completeness

$$\int_{S^2}Y_\ell^m(\hat{\mathbf{x}})\overline{Y_{\ell'}^{m'}(\hat{\mathbf{x}})}\,dS = \delta_{\ell\ell'}\delta_{mm'}.$$

**Completeness:** The spherical harmonics form a complete orthonormal basis for $L^2(S^2)$: every $f\in L^2(S^2)$ can be written as

$$f(\theta,\phi) = \sum_{\ell=0}^\infty\sum_{m=-\ell}^\ell \hat{f}_\ell^m Y_\ell^m(\theta,\phi), \qquad \hat{f}_\ell^m = \int_{S^2}f(\hat{\mathbf{x}})\overline{Y_\ell^m(\hat{\mathbf{x}})}\,dS,$$

with convergence in $L^2(S^2)$.

## Symmetry Properties

**Complex conjugate:** $\overline{Y_\ell^m} = (-1)^m Y_\ell^{-m}$.

**Rotation:** For a rotation $R\in SO(3)$, $Y_\ell^m(R^{-1}\hat{\mathbf{x}}) = \sum_{m'=-\ell}^\ell D^{(\ell)}_{m'm}(R)\,Y_\ell^{m'}(\hat{\mathbf{x}})$, where $D^{(\ell)}$ is the Wigner D-matrix (the $(2\ell+1)$-dimensional irreducible representation of $SO(3)$). In particular, spherical harmonics of the same degree $\ell$ transform among themselves under rotations.

**Addition theorem:** For any two unit vectors $\hat{\mathbf{x}}, \hat{\mathbf{y}} \in S^2$ with angle $\gamma$ between them:

$$P_\ell(\cos\gamma) = \frac{4\pi}{2\ell+1}\sum_{m=-\ell}^\ell Y_\ell^m(\hat{\mathbf{x}})\overline{Y_\ell^m(\hat{\mathbf{y}})},$$

where $P_\ell$ is the Legendre polynomial of degree $\ell$.

## Real Spherical Harmonics

For computation, real spherical harmonics $Y_{\ell m}^{\text{real}}$ are often preferred:

$$Y_{\ell m}^{\text{real}} = \begin{cases}\frac{1}{\sqrt{2}}\left(Y_\ell^{-|m|} + (-1)^m Y_\ell^{|m|}\right) & m > 0 \\ Y_\ell^0 & m = 0 \\ \frac{i}{\sqrt{2}}\left(Y_\ell^{-|m|} - (-1)^m Y_\ell^{|m|}\right) & m < 0\end{cases}$$

These are real-valued and form an orthonormal basis for $L^2(S^2;\mathbb{R})$.

## Harmonic Polynomials

In Cartesian coordinates, $r^\ell Y_\ell^m(\hat{\mathbf{x}})$ is a homogeneous polynomial of degree $\ell$ satisfying $\Delta(r^\ell Y_\ell^m)=0$. For example:

- $\ell=0$: $r^0 Y_0^0 = 1/\sqrt{4\pi}$ (constant).
- $\ell=1$: $rY_1^0 \propto z$, $rY_1^{\pm1} \propto x\pm iy$ (linear polynomials).
- $\ell=2$: $r^2Y_2^m$ gives the five harmonic quadratics $\{xy, yz, xz, x^2-y^2, 2z^2-x^2-y^2\}$.

The space of harmonic polynomials of degree $\ell$ in $\mathbb{R}^3$ is $(2\ell+1)$-dimensional, corresponding to the $2\ell+1$ values of $m$. This dimension equals the number of irreducible components of the $\ell$-th symmetric tensor representation minus the trace, a fact from representation theory.

## Connection to Hydrogen Atom

The hydrogen atom wave functions $\psi_{n\ell m}(r,\theta,\phi) = R_{n\ell}(r)Y_\ell^m(\theta,\phi)$ factor into a radial part $R_{n\ell}$ (involving Laguerre polynomials) and an angular part $Y_\ell^m$. The quantum number $\ell$ is the angular momentum quantum number; $m$ is the magnetic quantum number. The energy levels $E_n = -13.6\text{ eV}/n^2$ depend only on $n = \ell + 1, \ell+2, \ldots$, not on $m$, reflecting the degeneracy of spherical harmonics of the same degree.
