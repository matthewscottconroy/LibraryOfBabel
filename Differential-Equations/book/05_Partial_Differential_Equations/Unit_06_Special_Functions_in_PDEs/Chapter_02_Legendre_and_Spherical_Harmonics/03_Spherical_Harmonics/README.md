# Spherical Harmonics

Spherical harmonics $Y_\ell^m(\theta,\phi)$ are the simultaneous eigenfunctions of the Laplace-Beltrami operator $\Delta_{S^2}$ and the azimuthal derivative $-i\partial/\partial\phi$. They form a complete orthonormal basis for $L^2(S^2)$ — the Fourier modes of the sphere — and carry the irreducible representations of $SO(3)$. Every function on the sphere expands uniquely in spherical harmonics, and this expansion diagonalizes both the Laplacian and all rotationally invariant operators. The present section develops the definition, orthonormality, completeness, transformation properties under rotations, and explicit formulas needed for PDE applications.

## Definition and Normalization

**Definition.** The spherical harmonics are:

$$Y_\ell^m(\theta,\phi) = (-1)^m\sqrt{\frac{2\ell+1}{4\pi}\frac{(\ell-|m|)!}{(\ell+|m|)!}}\,P_\ell^{|m|}(\cos\theta)\,e^{im\phi}, \tag{Def}$$

for $\ell = 0,1,2,\ldots$ and $m = -\ell,-\ell+1,\ldots,\ell$. The factor $(-1)^m$ is the Condon-Shortley phase (standard in physics). The normalization constant is chosen so that $\|Y_\ell^m\|_{L^2(S^2)} = 1$.

**Eigenvalue equation:**

$$\Delta_{S^2}Y_\ell^m = -\ell(\ell+1)Y_\ell^m. \tag{Eigenvalue}$$

**Azimuthal eigenvalue:**

$$-i\frac{\partial}{\partial\phi}Y_\ell^m = m\,Y_\ell^m.$$

In quantum mechanics, $L_z = -i\hbar\partial/\partial\phi$ is the $z$-component of angular momentum, and $Y_\ell^m$ is an eigenstate of $L_z$ with eigenvalue $m\hbar$.

## Explicit Formulas

**$\ell = 0$:** $Y_0^0 = \frac{1}{\sqrt{4\pi}}$ (constant function on $S^2$).

**$\ell = 1$:**

$$Y_1^0 = \sqrt{\frac{3}{4\pi}}\cos\theta, \qquad Y_1^{\pm 1} = \mp\sqrt{\frac{3}{8\pi}}\sin\theta\,e^{\pm i\phi}.$$

**$\ell = 2$:**

$$Y_2^0 = \sqrt{\frac{5}{16\pi}}(3\cos^2\theta-1), \qquad Y_2^{\pm 1} = \mp\sqrt{\frac{15}{8\pi}}\sin\theta\cos\theta\,e^{\pm i\phi},$$

$$Y_2^{\pm 2} = \sqrt{\frac{15}{32\pi}}\sin^2\theta\,e^{\pm 2i\phi}.$$

**$\ell = 3$:**

$$Y_3^0 = \sqrt{\frac{7}{16\pi}}(5\cos^3\theta-3\cos\theta), \qquad Y_3^{\pm 1} = \mp\frac{1}{4}\sqrt{\frac{21}{4\pi}}\sin\theta(5\cos^2\theta-1)e^{\pm i\phi},$$

$$Y_3^{\pm 2} = \frac{1}{2}\sqrt{\frac{105}{2\pi}}\sin^2\theta\cos\theta\,e^{\pm 2i\phi}, \qquad Y_3^{\pm 3} = \mp\frac{1}{4}\sqrt{\frac{35}{\pi}}\sin^3\theta\,e^{\pm 3i\phi}.$$

**Complex conjugation:** $\overline{Y_\ell^m(\theta,\phi)} = (-1)^m Y_\ell^{-m}(\theta,\phi)$.

## Orthonormality

**Theorem.** The spherical harmonics satisfy:

$$\int_{S^2} Y_\ell^m(\hat{\mathbf{x}})\overline{Y_{\ell'}^{m'}(\hat{\mathbf{x}})}\,dS = \delta_{\ell\ell'}\delta_{mm'}, \tag{Orthonormality}$$

where $dS = \sin\theta\,d\theta\,d\phi$ is the surface measure on $S^2$.

**Proof.** The integral factors as:

$$\int_0^{2\pi}e^{im\phi}e^{-im'\phi}\,d\phi \cdot \int_0^\pi P_\ell^{|m|}(\cos\theta)P_{\ell'}^{|m'|}(\cos\theta)\sin\theta\,d\theta.$$

The azimuthal factor gives $2\pi\delta_{mm'}$. For $m = m'$, the polar factor (with $t = \cos\theta$) becomes $\int_{-1}^1 P_\ell^m(t)P_{\ell'}^m(t)\,dt = \frac{2}{2\ell+1}\frac{(\ell+|m|)!}{(\ell-|m|)!}\delta_{\ell\ell'}$ (orthogonality of associated Legendre functions). Multiplying by the normalization constants $(2\ell+1)/(4\pi) \cdot (\ell-|m|)!/(\ell+|m|)!$ and the factor $2\pi$ from the $\phi$-integral gives exactly $\delta_{\ell\ell'}$. $\square$

## Completeness

**Theorem.** The spherical harmonics $\{Y_\ell^m : \ell \geq 0, |m| \leq \ell\}$ form a complete orthonormal system in $L^2(S^2)$: every $f \in L^2(S^2)$ has the expansion:

$$f(\theta,\phi) = \sum_{\ell=0}^\infty\sum_{m=-\ell}^\ell \hat{f}_\ell^m Y_\ell^m(\theta,\phi), \qquad \hat{f}_\ell^m = \int_{S^2}f(\hat{\mathbf{x}})\overline{Y_\ell^m(\hat{\mathbf{x}})}\,dS,$$

with $\|f\|_{L^2(S^2)}^2 = \sum_{\ell,m}|\hat{f}_\ell^m|^2$ (Parseval's identity). Convergence is in $L^2(S^2)$; for smooth $f$, the convergence is uniform.

**Proof sketch.** The Laplace-Beltrami operator $\Delta_{S^2}$ is a self-adjoint elliptic operator on $S^2$ (a compact manifold). By the spectral theorem for self-adjoint elliptic operators, its eigenfunctions form a complete orthonormal basis for $L^2(S^2)$. The eigenvalues are $-\ell(\ell+1)$ with multiplicity $2\ell+1$, and the eigenfunctions are the spherical harmonics.

## The Eigenvalue Problem and Harmonic Polynomials

There is a deep connection between $Y_\ell^m$ on the sphere and harmonic homogeneous polynomials in $\mathbb{R}^3$.

**Theorem.** The function $h_\ell(\mathbf{x}) = r^\ell Y_\ell^m(\theta,\phi)$ is a homogeneous polynomial of degree $\ell$ satisfying $\Delta h_\ell = 0$ in $\mathbb{R}^3$.

**Proof.** In spherical coordinates, $\Delta = r^{-2}\partial_r(r^2\partial_r) + r^{-2}\Delta_{S^2}$. For $h = r^\ell Y$: $r^{-2}\partial_r(r^2 \cdot \ell r^{\ell-1})Y = r^{-2}\ell(\ell+1)r^{\ell-1} \cdot r \cdot Y/r = \ell(\ell+1)r^{\ell-2}Y$... More precisely: $\partial_r(r^2\partial_r r^\ell) = \partial_r(\ell r^{\ell+1}) = \ell(\ell+1)r^\ell$, so $r^{-2}\cdot\ell(\ell+1)r^\ell Y + r^{-2}r^\ell(-\ell(\ell+1))Y = 0$. $\square$

The space of harmonic homogeneous polynomials of degree $\ell$ is $(2\ell+1)$-dimensional, spanned by $\{r^\ell Y_\ell^m : |m| \leq \ell\}$. In Cartesian coordinates:

- $\ell=0$: $1$.
- $\ell=1$: $x, y, z$.
- $\ell=2$: $xy, yz, xz, x^2-y^2, 2z^2-x^2-y^2$ (five harmonic quadratics).

## Transformation Under Rotations

**Theorem.** For $R \in SO(3)$: $Y_\ell^m(R^{-1}\hat{\mathbf{x}}) = \sum_{m'=-\ell}^\ell D^{(\ell)}_{m'm}(R)\,Y_\ell^{m'}(\hat{\mathbf{x}})$,

where $D^{(\ell)}$ is the **Wigner D-matrix** — the $(2\ell+1)\times(2\ell+1)$ matrix representing $R$ in the $\ell$-th irreducible representation of $SO(3)$.

**Consequences:**
1. The eigenspace $\{Y_\ell^m : |m| \leq \ell\}$ is invariant under rotations — spherical harmonics of degree $\ell$ mix only among themselves under $SO(3)$.
2. Rotationally invariant quantities involve only $\ell$-sums: $\sum_{m=-\ell}^\ell |Y_\ell^m(\hat{\mathbf{x}})|^2 = (2\ell+1)/(4\pi)$ is independent of $\hat{\mathbf{x}}$ (follows from the addition theorem).
3. The Wigner D-matrices satisfy $\int_{SO(3)} D^{(\ell)}_{mn}(R)\overline{D^{(\ell')}_{m'n'}(R)}\,dR = \frac{8\pi^2}{2\ell+1}\delta_{\ell\ell'}\delta_{mm'}\delta_{nn'}$ (Peter-Weyl theorem for $SO(3)$).

## Real Spherical Harmonics

For applications where real-valued functions are preferred (computer graphics, geophysics), the real spherical harmonics are:

$$Y_{\ell m}^{\text{real}} = \begin{cases}\frac{i}{\sqrt{2}}(Y_\ell^{-|m|} - (-1)^m Y_\ell^{|m|}) & m < 0, \\ Y_\ell^0 & m = 0, \\ \frac{1}{\sqrt{2}}(Y_\ell^{-m} + (-1)^m Y_\ell^m) & m > 0.\end{cases}$$

These are real-valued, form an orthonormal basis for $L^2(S^2;\mathbb{R})$, and are labeled $\{s, p_x, p_y, p_z, d_{xy}, d_{yz}, d_{xz}, d_{x^2-y^2}, d_{z^2}, \ldots\}$ in spectroscopic notation.

**Explicit real harmonics for $\ell=1$:**

$$Y_{10}^{\text{real}} = \sqrt{\frac{3}{4\pi}}\cos\theta = \sqrt{\frac{3}{4\pi}}\frac{z}{r} \propto p_z, \quad Y_{11}^{\text{real}} = \sqrt{\frac{3}{4\pi}}\sin\theta\cos\phi \propto p_x, \quad Y_{1,-1}^{\text{real}} = \sqrt{\frac{3}{4\pi}}\sin\theta\sin\phi \propto p_y.$$

## Worked Example: Expansion of a Characteristic Function

**Problem.** Find the spherical harmonic expansion of $f(\theta,\phi) = \mathbf{1}[\cos\theta > 0]$ (the indicator function of the upper hemisphere).

**Solution.** By azimuthal symmetry, $f$ depends only on $\theta$, so $\hat{f}_\ell^m = 0$ for $m \neq 0$. The coefficients are:

$$\hat{f}_\ell^0 = \int_{S^2}f(\hat{\mathbf{x}})Y_\ell^0(\hat{\mathbf{x}})\,dS = 2\pi\sqrt{\frac{2\ell+1}{4\pi}}\int_0^1 P_\ell(t)\,dt.$$

Using $\int_0^1 P_0\,dt = 1$, $\int_0^1 P_1\,dt = 1/2$, $\int_0^1 P_2\,dt = 0$, $\int_0^1 P_3\,dt = -1/8$, $\int_0^1 P_4\,dt = 0$, $\int_0^1 P_5\,dt = 3/64$ (odd-$\ell$ terms via recursion):

$$f(\theta,\phi) = \frac{1}{2}Y_0^0\cdot\sqrt{4\pi} + \sum_{\ell=1,3,5,\ldots}\hat{f}_\ell^0 Y_\ell^0(\theta,\phi).$$

The leading terms give: $f \approx \frac{1}{2} + \frac{3}{4}\cos\theta + \frac{7}{16}(5\cos^3\theta - 3\cos\theta)\cdot\frac{-1}{8}/|\text{coeff}| + \ldots$ The series converges in $L^2(S^2)$; pointwise convergence at $\theta=\pi/2$ (the boundary) converges to $1/2$ (the average of the limiting values $0$ and $1$).

## Application to the Heat Equation on $S^2$

The heat equation on the unit sphere $u_t = \kappa\Delta_{S^2}u$ with initial data $u(\hat{\mathbf{x}},0) = f(\hat{\mathbf{x}})$ has solution:

$$u(\hat{\mathbf{x}},t) = \sum_{\ell=0}^\infty\sum_{m=-\ell}^\ell \hat{f}_\ell^m\,e^{-\kappa\ell(\ell+1)t}Y_\ell^m(\hat{\mathbf{x}}).$$

Each spherical harmonic mode decays with rate $\kappa\ell(\ell+1)$: the monopole ($\ell=0$) is conserved, the dipole ($\ell=1$) decays with rate $2\kappa$, the quadrupole ($\ell=2$) with rate $6\kappa$, etc. As $t\to\infty$, $u(\hat{\mathbf{x}},t) \to \hat{f}_0^0 Y_0^0 = \frac{1}{4\pi}\int_{S^2}f\,dS$ — the spatial average, consistent with the mean value property for harmonic functions.
