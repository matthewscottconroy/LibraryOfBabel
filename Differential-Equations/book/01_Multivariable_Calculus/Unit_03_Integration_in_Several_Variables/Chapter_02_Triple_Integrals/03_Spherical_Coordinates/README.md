# Spherical Coordinates

Spherical coordinates are optimal for integrating over regions with three-dimensional spherical symmetry: balls, spherical shells, the region between two concentric spheres, and cones. They are also essential in physics — the hydrogen atom, gravitational potentials, and electromagnetic radiation all use spherical harmonics, which are naturally expressed in spherical coordinates. The distinctive feature of the spherical volume element $\rho^2\sin\phi\,d\rho\,d\theta\,d\phi$ is that it grows quadratically with the radius, reflecting the fact that spherical shells have area proportional to $\rho^2$.

## Coordinate Definitions

A point in $\mathbb{R}^3$ is described by $(\rho, \theta, \phi)$ where (using the mathematics convention):
- $\rho = \sqrt{x^2+y^2+z^2} \geq 0$: distance from the origin.
- $\theta \in [0, 2\pi)$: azimuthal angle in the $xy$-plane (same as in cylindrical and polar).
- $\phi \in [0, \pi]$: polar angle from the positive $z$-axis ($\phi=0$ is the north pole, $\phi=\pi/2$ is the equator, $\phi=\pi$ is the south pole).

Conversion to Cartesian:

$$x = \rho\sin\phi\cos\theta, \quad y = \rho\sin\phi\sin\theta, \quad z = \rho\cos\phi.$$

Note $r = \rho\sin\phi$ (polar radius in the $xy$-plane) and $z = \rho\cos\phi$.

## The Jacobian and Volume Element

$$\det J = \det\begin{pmatrix}\sin\phi\cos\theta & \rho\cos\phi\cos\theta & -\rho\sin\phi\sin\theta \\ \sin\phi\sin\theta & \rho\cos\phi\sin\theta & \rho\sin\phi\cos\theta \\ \cos\phi & -\rho\sin\phi & 0\end{pmatrix} = \rho^2\sin\phi.$$

(This determinant is computed by expanding along the third row or using cofactor expansion.)

Therefore the volume element is:

$$dV = \rho^2\sin\phi\,d\rho\,d\theta\,d\phi.$$

The factor $\rho^2\sin\phi$ is positive for $\rho > 0$ and $\phi\in(0,\pi)$ (i.e., away from the $z$-axis), so the Jacobian is non-negative throughout the physically relevant region.

## Standard Spherical Regions

- **Ball of radius $R$:** $0\leq\rho\leq R$, $0\leq\theta\leq 2\pi$, $0\leq\phi\leq\pi$.
- **Upper hemisphere:** $0\leq\phi\leq\pi/2$ added.
- **Spherical shell between radii $a$ and $b$:** $a\leq\rho\leq b$.
- **Cone $\phi\leq\phi_0$:** $0\leq\phi\leq\phi_0$ (ice-cream cone shape).
- **Sphere $\rho = R$:** the surface, not the solid.

The sphere $x^2+y^2+z^2=R^2$ becomes simply $\rho = R$.

The cone $z^2 = x^2+y^2$ (i.e., $z = \pm\sqrt{x^2+y^2}$) becomes $\cos^2\phi = \sin^2\phi$, i.e., $\phi = \pi/4$ or $\phi = 3\pi/4$.

## Worked Example 1: Volume of a Ball

$V = \int_0^{2\pi}\int_0^{\pi}\int_0^R\rho^2\sin\phi\,d\rho\,d\phi\,d\theta = 2\pi\int_0^{\pi}\sin\phi\,d\phi\cdot\frac{R^3}{3} = 2\pi\cdot 2\cdot\frac{R^3}{3} = \frac{4\pi R^3}{3}.$

## Worked Example 2: Integral Over a Ball

$\iiint_{x^2+y^2+z^2\leq 1}(x^2+y^2+z^2)^{3/2}\,dV$.

In spherical: $x^2+y^2+z^2=\rho^2$, so $(x^2+y^2+z^2)^{3/2}=\rho^3$.

$\int_0^{2\pi}\int_0^{\pi}\int_0^1\rho^3\cdot\rho^2\sin\phi\,d\rho\,d\phi\,d\theta = 2\pi\cdot 2\cdot\frac{1}{6} = \frac{2\pi}{3}$.

## Worked Example 3: Region Between Sphere and Cone

Find the volume of the region bounded above by the sphere $\rho=2$ and below by the cone $\phi=\pi/3$.

The region: $0\leq\rho\leq 2$, $0\leq\theta\leq 2\pi$, $0\leq\phi\leq\pi/3$.

$V = \int_0^{2\pi}\int_0^{\pi/3}\int_0^2\rho^2\sin\phi\,d\rho\,d\phi\,d\theta = 2\pi\cdot\frac{8}{3}\int_0^{\pi/3}\sin\phi\,d\phi = \frac{16\pi}{3}[-\cos\phi]_0^{\pi/3} = \frac{16\pi}{3}(1-1/2) = \frac{8\pi}{3}.$

## Worked Example 4: Gaussian Integral in 3D

$\iiint_{\mathbb{R}^3}e^{-(x^2+y^2+z^2)}\,dV = \int_0^{2\pi}\int_0^{\pi}\int_0^{\infty}e^{-\rho^2}\rho^2\sin\phi\,d\rho\,d\phi\,d\theta = 4\pi\int_0^\infty\rho^2 e^{-\rho^2}\,d\rho$.

Using $\int_0^\infty\rho^2 e^{-\rho^2}\,d\rho = \sqrt{\pi}/4$ (from integration by parts and the Gaussian integral): $4\pi\cdot\sqrt{\pi}/4 = \pi^{3/2}$.

Equivalently, $\left(\int_{-\infty}^{\infty}e^{-x^2}\,dx\right)^3 = \left(\sqrt{\pi}\right)^3 = \pi^{3/2}$.

## Choosing Between Cylindrical and Spherical

Use **cylindrical** when the region or integrand depends on $r = \sqrt{x^2+y^2}$ and $z$ separately: cylinders, paraboloids, cones.

Use **spherical** when the region or integrand depends on $\rho = \sqrt{x^2+y^2+z^2}$: balls, spherical shells, and functions of distance from the origin.

Many problems can be done in either system; the art is choosing the one that gives simpler bounds and simpler integrands.

## Common Pitfalls

The volume element in spherical is $\rho^2\sin\phi\,d\rho\,d\theta\,d\phi$, with both $\rho^2$ and $\sin\phi$. Forgetting either factor is a common error.

The polar angle $\phi$ ranges from $0$ to $\pi$, not $0$ to $2\pi$. The full sphere is covered by letting $\theta$ go from $0$ to $2\pi$ and $\phi$ from $0$ to $\pi$.

Physics texts often swap $\theta$ and $\phi$: $\theta$ is the polar angle (from the $z$-axis) and $\phi$ is the azimuthal angle. Always check which convention is in use before setting up integrals.
