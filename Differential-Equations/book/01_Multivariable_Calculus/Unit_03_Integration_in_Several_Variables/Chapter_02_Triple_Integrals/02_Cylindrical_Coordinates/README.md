# Cylindrical Coordinates

Cylindrical coordinates extend polar coordinates from the plane to space by adding the Cartesian $z$-coordinate unchanged. They are natural for problems with axial symmetry: cylinders, cones, paraboloids, and any surface of revolution about the $z$-axis. When the domain or integrand has this symmetry, cylindrical coordinates can dramatically simplify a triple integral that would be unwieldy in Cartesian.

## Coordinate Definitions

A point in $\mathbb{R}^3$ is described by $(r, \theta, z)$ where:
- $r = \sqrt{x^2+y^2} \geq 0$ is the distance from the $z$-axis.
- $\theta \in [0, 2\pi)$ is the azimuthal angle (same as in polar coordinates).
- $z \in \mathbb{R}$ is the signed height.

The conversion to Cartesian is:

$$x = r\cos\theta, \quad y = r\sin\theta, \quad z = z.$$

The Jacobian of this transformation is:

$$\det J = \det\begin{pmatrix}\cos\theta & -r\sin\theta & 0 \\ \sin\theta & r\cos\theta & 0 \\ 0 & 0 & 1\end{pmatrix} = r.$$

Therefore the volume element is $dV = r\,dr\,d\theta\,dz$.

## Setting Up Integrals in Cylindrical Coordinates

The iterated integral in cylindrical coordinates typically takes the form:

$$\iiint_E f\,dV = \int_{\alpha}^{\beta}\int_{r_1(\theta)}^{r_2(\theta)}\int_{h_1(r,\theta)}^{h_2(r,\theta)} f(r\cos\theta, r\sin\theta, z)\,r\,dz\,dr\,d\theta.$$

(The ordering of $dz\,dr\,d\theta$ can vary.)

**Typical descriptions in cylindrical:**
- Cylinder $x^2+y^2 = a^2$: $r = a$.
- Cone $z = \sqrt{x^2+y^2}$: $z = r$ (for $z\geq 0$).
- Paraboloid $z = x^2+y^2$: $z = r^2$.
- Sphere $x^2+y^2+z^2 = R^2$: $r^2+z^2=R^2$, i.e., $z = \pm\sqrt{R^2-r^2}$.

## Worked Example 1: Volume of a Cone

Find the volume of the solid bounded above by $z=1$ and below by the cone $z=\sqrt{x^2+y^2}$.

The cone and plane intersect at $r=1$ (when $z=r=1$).

$$V = \int_0^{2\pi}\int_0^1\int_r^1 r\,dz\,dr\,d\theta = 2\pi\int_0^1 r(1-r)\,dr = 2\pi\left[\frac{r^2}{2}-\frac{r^3}{3}\right]_0^1 = 2\pi\cdot\frac{1}{6} = \frac{\pi}{3}.$$

## Worked Example 2: Region Between Two Paraboloids

Evaluate $\iiint_E z\,dV$ where $E$ is bounded above by $z=4-r^2$ and below by $z=r^2$ (from the previous section's example).

Intersection: $r^2 = 4-r^2 \Rightarrow r=\sqrt{2}$.

$$\int_0^{2\pi}\int_0^{\sqrt{2}}\int_{r^2}^{4-r^2}z\cdot r\,dz\,dr\,d\theta = 2\pi\int_0^{\sqrt{2}}r\cdot\frac{(4-r^2)^2-r^4}{2}\,dr.$$

$(4-r^2)^2-r^4 = 16-8r^2+r^4-r^4 = 16-8r^2$.

$= 2\pi\int_0^{\sqrt{2}}r\cdot\frac{16-8r^2}{2}\,dr = 2\pi\int_0^{\sqrt{2}}(8r-4r^3)\,dr = 2\pi\left[4r^2-r^4\right]_0^{\sqrt{2}} = 2\pi(8-4) = 8\pi$.

## Worked Example 3: Triple Integral Over a Cylinder

Evaluate $\iiint_E (x^2+y^2)\,dV$ where $E$ is the cylinder $x^2+y^2\leq 1$, $0\leq z\leq 2$.

$x^2+y^2 = r^2$.

$\int_0^{2\pi}\int_0^1\int_0^2 r^2\cdot r\,dz\,dr\,d\theta = 2\pi\int_0^1 r^3\cdot 2\,dr = 4\pi\cdot\frac{1}{4} = \pi$.

## When to Use Cylindrical Coordinates

Use cylindrical coordinates when:
- The domain is bounded by a cylinder ($r = a$), cone ($z=r$), or paraboloid ($z=r^2$).
- The integrand involves $x^2+y^2$.
- The domain has azimuthal symmetry (the region looks the same for all $\theta$).

## Relation to Polar Coordinates

The inner two variables $(r,\theta)$ are just polar coordinates in the $xy$-plane. Setting up the $(r,\theta)$ part of a cylindrical integral is exactly the same as setting up a polar double integral over the projection of the region onto the $xy$-plane. The $z$ bounds then describe how high the region extends above (and how low below) each point $(r,\theta)$ in the projection.

## Common Pitfalls

The volume element is $r\,dr\,d\theta\,dz$, never $dr\,d\theta\,dz$. Forgetting the factor $r$ is the standard error.

Also, the outer limits (in $\theta$ and $r$) should not depend on $z$, or if they do (for non-$z$-simple regions), the order must be adjusted accordingly. For example, if the region is $r$-simple but not $z$-simple, one should integrate $z$ and $\theta$ first, then $r$.
