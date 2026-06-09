# Coordinate Systems

Cartesian coordinates — the familiar $(x, y)$ in the plane and $(x, y, z)$ in space — are not always the most efficient way to describe a geometric configuration or evaluate an integral. When a problem has circular or spherical symmetry, the natural coordinate system matches that symmetry, and the resulting description is far simpler. This section develops the three most important coordinate systems beyond Cartesian: polar coordinates in the plane, and cylindrical and spherical coordinates in space.

## Polar Coordinates in the Plane

In polar coordinates, a point $P$ in the plane is described by its distance $r$ from the origin and the angle $\theta$ that the segment $OP$ makes with the positive $x$-axis. The conversion formulas are:

$$x = r\cos\theta, \quad y = r\sin\theta, \quad r = \sqrt{x^2 + y^2}, \quad \tan\theta = \frac{y}{x}.$$

Here $r \geq 0$ and $\theta \in [0, 2\pi)$ (or $(-\pi, \pi]$ depending on convention). Note that every point except the origin has a unique polar representation with $r > 0$ and $\theta \in [0, 2\pi)$; the origin corresponds to $r = 0$ for any $\theta$.

Circles centered at the origin have particularly simple polar equations: $r = a$ is a circle of radius $a$. The equation $r = 2\cos\theta$ describes a circle of radius 1 centered at $(1, 0)$, which takes many more symbols to write in Cartesian form: $(x-1)^2 + y^2 = 1$. Spirals, cardioids, rose curves, and limaçons all have elegant polar equations.

Polar coordinates are essential for evaluating double integrals over circular regions. The **area element** in polar coordinates is $dA = r\,dr\,d\theta$ — the extra factor of $r$ is the Jacobian of the transformation from polar to Cartesian, which will be derived formally in Unit 3.

## Cylindrical Coordinates

Cylindrical coordinates $(r, \theta, z)$ extend polar coordinates to $\mathbb{R}^3$ by simply appending the Cartesian $z$-coordinate. The conversion formulas are:

$$x = r\cos\theta, \quad y = r\sin\theta, \quad z = z, \quad r = \sqrt{x^2+y^2}, \quad \tan\theta = y/x.$$

The coordinate surface $r = a$ is a cylinder of radius $a$ with axis along the $z$-axis (hence the name). The surface $\theta = \theta_0$ is a half-plane bounded by the $z$-axis. The surface $z = c$ is a horizontal plane, same as in Cartesian.

Cylindrical coordinates are natural for problems involving cylinders, cones, and any surface of revolution about the $z$-axis. The **volume element** is $dV = r\,dr\,d\theta\,dz$.

**Example.** In cylindrical coordinates, the sphere $x^2 + y^2 + z^2 = R^2$ becomes $r^2 + z^2 = R^2$. The cone $z = \sqrt{x^2 + y^2}$ becomes simply $z = r$ (for $z \geq 0$).

## Spherical Coordinates

Spherical coordinates $(\rho, \theta, \phi)$ describe a point by its distance from the origin, the azimuthal angle around the $z$-axis, and the polar angle from the positive $z$-axis:

- $\rho = \sqrt{x^2 + y^2 + z^2} \geq 0$ (distance from origin)
- $\theta \in [0, 2\pi)$ (same azimuthal angle as in cylindrical)
- $\phi \in [0, \pi]$ (polar angle; $\phi = 0$ is the positive $z$-axis, $\phi = \pi/2$ is the $xy$-plane, $\phi = \pi$ is the negative $z$-axis)

The conversion to Cartesian is:

$$x = \rho\sin\phi\cos\theta, \quad y = \rho\sin\phi\sin\theta, \quad z = \rho\cos\phi.$$

Note the relationship to cylindrical: $r = \rho\sin\phi$ and $z = \rho\cos\phi$.

The coordinate surface $\rho = a$ is a sphere of radius $a$ centered at the origin. The surface $\phi = \phi_0$ (constant polar angle) is a cone with apex at the origin and axis along the $z$-axis. The surface $\theta = \theta_0$ is a half-plane (same as in cylindrical).

The **volume element** in spherical coordinates is $dV = \rho^2\sin\phi\,d\rho\,d\theta\,d\phi$. The factor $\rho^2\sin\phi$ is the Jacobian of the transformation; it reflects the fact that small boxes in spherical coordinates have volume proportional to $\rho^2$ (because shells at large $\rho$ subtend larger physical volumes) and proportional to $\sin\phi$ (because slices near the poles are smaller).

## Warning on Conventions

Different textbooks and disciplines use different conventions for spherical coordinates. The physics convention often swaps $\theta$ and $\phi$: $\theta$ is the polar angle from the $z$-axis and $\phi$ is the azimuthal angle in the $xy$-plane. Always check which convention is in use. The mathematics convention adopted here uses $\phi$ for the polar angle; the physics convention is the opposite.

## Worked Examples

**Example 1.** Convert the point $(x, y, z) = (1, 1, \sqrt{2})$ to spherical coordinates.

$\rho = \sqrt{1+1+2} = 2$. $\cos\phi = z/\rho = \sqrt{2}/2$, so $\phi = \pi/4$. $\tan\theta = y/x = 1$, so $\theta = \pi/4$.

**Example 2.** The equation $\rho = 2\cos\phi$ in spherical coordinates describes the surface $\rho^2 = 2\rho\cos\phi$, i.e., $x^2+y^2+z^2 = 2z$, i.e., $x^2+y^2+(z-1)^2=1$. This is a sphere of radius 1 centered at $(0,0,1)$.

**Example 3.** Describe the region $1 \leq \rho \leq 2$, $0 \leq \phi \leq \pi/4$, $0 \leq \theta \leq 2\pi$ in Cartesian terms.

This is the region between two spheres of radii 1 and 2, lying above the cone $z = \sqrt{x^2+y^2}$ (which corresponds to $\phi = \pi/4$). It is a spherical shell with a cone removed from the bottom — the shape of an ice cream scoop in a cone.

## Choosing the Right Coordinate System

The guiding principle is to match the coordinate system to the symmetry of the problem. A region bounded by concentric spheres and cones calls for spherical coordinates. A region bounded by a cylinder and two horizontal planes calls for cylindrical. Regions with no symmetry are typically easiest in Cartesian.

This matching principle becomes particularly powerful in Unit 3 when evaluating triple integrals. The correct choice of coordinates can reduce a computation that would require several pages in Cartesian to a few lines.

## Connection to Differential Equations

Spherical and cylindrical coordinates appear naturally in the separation of variables technique for partial differential equations. Laplace's equation $\Delta u = 0$ in spherical coordinates separates into ordinary differential equations in $\rho$, $\theta$, and $\phi$, leading to spherical harmonics. Understanding the coordinate geometry here is a prerequisite for that later work.
