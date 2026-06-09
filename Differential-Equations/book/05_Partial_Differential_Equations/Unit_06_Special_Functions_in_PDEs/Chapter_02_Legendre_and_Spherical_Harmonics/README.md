# Chapter 2: Legendre Functions and Spherical Harmonics

Legendre polynomials $P_\ell(t)$, associated Legendre functions $P_\ell^m(t)$, and spherical harmonics $Y_\ell^m(\theta,\phi)$ form the complete angular theory for the Laplacian on the sphere $S^2$. While Unit 5 introduced these functions in the context of solving Laplace's equation on a ball, this chapter develops their properties as a self-contained theory — a reference for their generation, orthogonality, recursion, addition theorem, and connections to group theory. Together, these properties make spherical harmonics the natural "Fourier analysis on the sphere," with applications ranging from quantum mechanics to cosmology.

## The Angular Eigenvalue Problem

The angular part of the Laplacian in spherical coordinates is the **Laplace-Beltrami operator**:

$$\Delta_{S^2} = \frac{1}{\sin\theta}\frac{\partial}{\partial\theta}\!\left(\sin\theta\frac{\partial}{\partial\theta}\right) + \frac{1}{\sin^2\theta}\frac{\partial^2}{\partial\phi^2}.$$

Seeking eigenfunctions $\Delta_{S^2}Y = -\ell(\ell+1)Y$ (the eigenvalue is conventionally written as $-\ell(\ell+1)$ for algebraic convenience), separation $Y = \Theta(\theta)\Phi(\phi)$ with $\Phi = e^{im\phi}$ ($m \in \mathbb{Z}$, periodicity in $\phi$) gives the **associated Legendre equation** for $\Theta$:

$$\frac{d}{dt}\!\left[(1-t^2)\frac{d\Theta}{dt}\right] + \left[\ell(\ell+1) - \frac{m^2}{1-t^2}\right]\Theta = 0, \qquad t = \cos\theta \in [-1,1].$$

For solutions bounded at $t = \pm 1$ (the poles $\theta = 0,\pi$), the parameter $\ell$ must be a non-negative integer with $|m| \leq \ell$. The bounded solutions are $\Theta = P_\ell^m(\cos\theta)$.

## Structure of This Chapter

**Section 1: Legendre Polynomials — Properties** provides a comprehensive reference for the $m=0$ case: the power series solution, the Rodrigues formula $P_\ell(t) = \frac{1}{2^\ell\ell!}(d/dt)^\ell(t^2-1)^\ell$, the generating function $\sum_\ell P_\ell(t)s^\ell = (1-2ts+s^2)^{-1/2}$, the three-term recursion $(n+1)P_{n+1} = (2n+1)tP_n - nP_{n-1}$, the orthogonality $\int_{-1}^1 P_\ell P_k\,dt = \frac{2}{2\ell+1}\delta_{\ell k}$, and special values. The Legendre polynomials are the orthogonal polynomials for the weight $w=1$ on $[-1,1]$.

**Section 2: Associated Legendre Functions** develops $P_\ell^m(t) = (-1)^m(1-t^2)^{m/2}(d/dt)^m P_\ell(t)$ for $m \geq 0$ and the extension to $m < 0$ via $P_\ell^{-m} = (-1)^m\frac{(\ell-m)!}{(\ell+m)!}P_\ell^m$. The orthogonality relation $\int_{-1}^1 P_\ell^m P_k^m\,dt = \frac{2}{2\ell+1}\frac{(\ell+m)!}{(\ell-m)!}\delta_{\ell k}$ with its asymmetric normalization is derived, and the Condon-Shortley phase convention is explained.

**Section 3: Spherical Harmonics** presents $Y_\ell^m$ as normalized joint eigenfunctions of $\Delta_{S^2}$ and $\partial/\partial\phi$. The orthonormality $\int_{S^2}Y_\ell^m\overline{Y_{\ell'}^{m'}}\,dS = \delta_{\ell\ell'}\delta_{mm'}$, completeness (every $L^2$ function on $S^2$ expands uniquely in $Y_\ell^m$), and the structure of the $(2\ell+1)$-dimensional eigenspace (irreducible under $SO(3)$) are developed. Real spherical harmonics for computational purposes are also given.

**Section 4: The Addition Theorem** is the central identity connecting spherical harmonics to potential theory: for unit vectors $\hat{\mathbf{x}}, \hat{\mathbf{y}}$ with angle $\gamma$ between them,

$$P_\ell(\cos\gamma) = \frac{4\pi}{2\ell+1}\sum_{m=-\ell}^\ell Y_\ell^m(\hat{\mathbf{x}})\overline{Y_\ell^m(\hat{\mathbf{y}})}.$$

This encodes the rotational invariance of $P_\ell(\cos\gamma)$ (it depends only on the angle between the two directions, not their individual orientations) as a sum over the $2\ell+1$ components of the rotation-invariant combination $\sum_m Y_\ell^m\overline{Y_\ell^m}$. The addition theorem is the key to the multipole expansion of the Coulomb potential and to the convolution theorem on $S^2$.

## Key Theorems to Be Proved

**Theorem (spectral theory on $S^2$).** The eigenvalues of $-\Delta_{S^2}$ are $\lambda_\ell = \ell(\ell+1)$ for $\ell = 0,1,2,\ldots$, each with multiplicity $2\ell+1$. The eigenspace for $\lambda_\ell$ is spanned by $\{Y_\ell^m : -\ell \leq m \leq \ell\}$, and this space is the space of restrictions to $S^2$ of harmonic homogeneous polynomials of degree $\ell$ in $\mathbb{R}^3$.

**Theorem (completeness).** The spherical harmonics $\{Y_\ell^m\}_{\ell \geq 0, |m| \leq \ell}$ form a complete orthonormal system in $L^2(S^2)$.

**Theorem (addition theorem).** Stated above. Proof via the orthogonal invariance of $P_\ell$ and the completeness of $\{Y_\ell^m\}_{|m|\leq\ell}$ as a basis for the $\ell$-th eigenspace.

## Connections and Applications

**Fourier analysis on $S^2$.** The spherical harmonic expansion $f = \sum_{\ell,m}\hat{f}_\ell^m Y_\ell^m$ is the analog of the Fourier series on the circle. The Fourier coefficients $\hat{f}_\ell^m = \int_{S^2}f\overline{Y_\ell^m}\,dS$ satisfy Parseval: $\|f\|_{L^2}^2 = \sum_{\ell,m}|\hat{f}_\ell^m|^2$. Differentiation corresponds to multiplication: $\widehat{\Delta_{S^2}f}_\ell^m = -\ell(\ell+1)\hat{f}_\ell^m$.

**Representation theory.** The space $\{Y_\ell^m : |m| \leq \ell\}$ carries the irreducible $(2\ell+1)$-dimensional representation of $SO(3)$. The Wigner D-matrices $D^{(\ell)}_{m'm}(R)$ describe how $Y_\ell^m$ transforms under rotations. The Peter-Weyl theorem for compact groups says that the matrix elements of all irreducible representations form a complete orthogonal system — for $SO(3)$, these are the $D^{(\ell)}_{m'm}$, with the $Y_\ell^m = D^{(\ell)}_{m0}$ as a special case.

**Harmonic analysis on $S^2$.** The convolution of two functions on $S^2$ (under the group action of $SO(3)$) diagonalizes in the spherical harmonic basis: $\widehat{(f*g)}_\ell = \frac{4\pi}{2\ell+1}\hat{f}_\ell\cdot\hat{g}_\ell$ (a product of $\ell$-th order multipole moments). This is the basis for fast spherical harmonic transforms and is used in CMB data analysis, where the power spectrum $C_\ell = \frac{1}{2\ell+1}\sum_m|\hat{a}_\ell^m|^2$ captures the angular power of temperature fluctuations.
