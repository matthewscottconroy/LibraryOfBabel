# Chapter 4: The Heat Equation in Other Geometries

The heat equation on a rectangular slab is solved by separation of variables with Fourier series in Cartesian coordinates. But many physically important domains — cylinders, spheres, annuli, spherical shells — have geometries that call for other coordinate systems. When the domain and boundary conditions have cylindrical or spherical symmetry, separation of variables in the appropriate coordinate system reduces the problem to ODEs involving Bessel functions (cylindrical case) or Legendre polynomials and spherical harmonics (spherical case).

## Cylindrical Geometry

Consider the heat equation in three dimensions for a temperature $u(r,\theta,z,t)$ in a cylindrical region. In cylindrical coordinates $(r,\theta,z)$ with $r = \sqrt{x^2+y^2}$, the Laplacian is

$$\Delta u = u_{rr} + \frac{1}{r}u_r + \frac{1}{r^2}u_{\theta\theta} + u_{zz} = \frac{1}{r}\frac{\partial}{\partial r}\!\left(r\frac{\partial u}{\partial r}\right) + \frac{1}{r^2}u_{\theta\theta} + u_{zz}.$$

For a radially symmetric problem (independent of $\theta$ and $z$), this reduces to

$$u_t = \kappa\!\left(u_{rr} + \frac{1}{r}u_r\right) = \frac{\kappa}{r}\frac{\partial}{\partial r}\!\left(r\frac{\partial u}{\partial r}\right).$$

Separation of variables $u = R(r)T(t)$ gives $T'/(\kappa T) = (R'' + R'/r)/R = -\lambda^2$. The radial equation is Bessel's equation of order zero: $R'' + R'/r + \lambda^2 R = 0$, with bounded solutions $R = J_0(\lambda r)$ where $J_0$ is the Bessel function of the first kind of order zero. The boundary condition at the cylinder wall $r = a$ determines the eigenvalues: $J_0(\lambda_{0n}a) = 0$ where $\lambda_{0n} = j_{0n}/a$ are determined by the zeros $j_{0n}$ of $J_0$.

## Spherical Geometry

In spherical coordinates $(r,\theta,\phi)$, the Laplacian is

$$\Delta u = \frac{1}{r^2}\frac{\partial}{\partial r}\!\left(r^2\frac{\partial u}{\partial r}\right) + \frac{1}{r^2\sin\theta}\frac{\partial}{\partial\theta}\!\left(\sin\theta\frac{\partial u}{\partial\theta}\right) + \frac{1}{r^2\sin^2\theta}\frac{\partial^2 u}{\partial\phi^2}.$$

For a radially symmetric problem ($u = u(r,t)$ only), this becomes $u_t = \kappa(u_{rr} + 2u_r/r)$. The substitution $u = v/r$ gives $v_t = \kappa v_{rr}$ — the heat equation in one dimension! This dimensional reduction is powerful: it means the theory of the 1D heat equation applies directly to radially symmetric 3D problems.

## Structure of This Chapter

**Section 1: Cylindrical Coordinates** develops the full theory of the heat equation on a disk or cylinder, including the solution in terms of Bessel functions, the Bessel-Fourier series, and explicit heat kernel formulas.

**Section 2: Spherical Coordinates** uses the $v = ur$ substitution for radially symmetric problems and develops the full expansion in spherical harmonics for non-symmetric problems on spherical domains.

Both sections connect to Unit 6, where Bessel functions and spherical harmonics are treated in greater depth as the special functions arising from eigenvalue problems in curved geometries.
