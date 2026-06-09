# Chapter 5: Laplace's Equation in 3D Spherical Coordinates

The separation of variables method for Laplace's equation in 3D spherical coordinates produces two types of special functions: the radial functions $r^\ell$ and $r^{-\ell-1}$ (from the Euler equation in $r$), and the spherical harmonics $Y_\ell^m(\theta,\phi)$ (from the angular eigenvalue problem on the sphere $S^2$). Spherical harmonics are the eigenfunctions of the Laplace-Beltrami operator on $S^2$, and they form a complete orthonormal basis for $L^2(S^2)$. Every solution of Laplace's equation on a ball or the exterior of a ball can be expressed as an infinite series of these functions.

## Separation of Variables in 3D

The Laplacian in spherical coordinates $(r,\theta,\phi)$ is:

$$\Delta u = \frac{1}{r^2}\frac{\partial}{\partial r}\!\left(r^2\frac{\partial u}{\partial r}\right) + \frac{1}{r^2\sin\theta}\frac{\partial}{\partial\theta}\!\left(\sin\theta\frac{\partial u}{\partial\theta}\right) + \frac{1}{r^2\sin^2\theta}\frac{\partial^2 u}{\partial\phi^2} = 0.$$

Seeking $u = R(r)\Theta(\theta)\Phi(\phi)$ and separating: the radial equation gives $R = Ar^\ell + Br^{-\ell-1}$; the azimuthal equation gives $\Phi = e^{im\phi}$ with $m \in \mathbb{Z}$; the polar equation gives the associated Legendre equation for $\Theta(\theta) = P_\ell^m(\cos\theta)$.

## Structure of This Chapter

**Section 1: Spherical Harmonics** defines and develops the spherical harmonics $Y_\ell^m(\theta,\phi) = C_\ell^m P_\ell^m(\cos\theta)e^{im\phi}$ — the eigenfunctions of the Laplace-Beltrami operator on $S^2$ with eigenvalue $-\ell(\ell+1)$. They are orthonormal: $\int_{S^2}Y_\ell^m\overline{Y_{\ell'}^{m'}}\,dS = \delta_{\ell\ell'}\delta_{mm'}$.

**Section 2: Legendre Polynomials in PDE Applications** reviews the properties of $P_\ell^m$ needed for PDE applications: Rodrigues' formula, the generating function, orthogonality, and the recurrence relations. The addition theorem $P_\ell(\cos\gamma) = \frac{4\pi}{2\ell+1}\sum_{m=-\ell}^\ell Y_\ell^m(\hat{\mathbf{x}})Y_\ell^{m*}(\hat{\mathbf{y}})$ (where $\gamma$ is the angle between $\hat{\mathbf{x}}$ and $\hat{\mathbf{y}}$) is the key formula connecting spherical harmonics to the Green's function.

**Section 3: Applications to Potential Theory** solves the Dirichlet problem for Laplace's equation on a ball using spherical harmonic expansions. The solution is an infinite series in $r^\ell Y_\ell^m(\hat{\mathbf{x}})$ (interior) or $r^{-\ell-1}Y_\ell^m(\hat{\mathbf{x}})$ (exterior), with coefficients determined by the boundary data. The Poisson formula for the ball is recovered.

## Physical Significance

Spherical harmonics are ubiquitous in physics:
- **Atomic orbitals:** The angular part of hydrogen atom wave functions are spherical harmonics $Y_\ell^m$, labeled by quantum numbers $\ell$ (angular momentum) and $m$ (magnetic quantum number).
- **Multipole expansion:** The potential of a charge distribution outside a sphere is expanded as $\sum_{\ell,m}q_{\ell m}r^{-\ell-1}Y_\ell^m$ — the multipole expansion. The $\ell=0$ term is the monopole, $\ell=1$ is the dipole, $\ell=2$ is the quadrupole.
- **Earth's geoid:** The shape of the earth's gravitational potential is expressed in spherical harmonics with thousands of terms.
