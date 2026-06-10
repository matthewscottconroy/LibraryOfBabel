# Section 8.5: Curvilinear Coordinate Systems

---

## Section Introduction

Cartesian coordinates are natural for problems with rectangular symmetry, but many physical systems have spherical or cylindrical symmetry. Working in **curvilinear coordinates** — polar, cylindrical, spherical — simplifies both the equations and the intuition enormously. The price is that the gradient, divergence, curl, and Laplacian take more complex forms than their Cartesian counterparts.

**Cylindrical coordinates** $(r, \phi, z)$ are related to Cartesian by $x = r\cos\phi$, $y = r\sin\phi$, $z = z$. They are adapted to problems with azimuthal symmetry: the charge distribution of a wire, the magnetic field of a solenoid, the flow in a pipe. In these coordinates, the Laplacian becomes $\nabla^2 f = \frac{1}{r}\frac{\partial}{\partial r}\left(r\frac{\partial f}{\partial r}\right) + \frac{1}{r^2}\frac{\partial^2 f}{\partial\phi^2} + \frac{\partial^2 f}{\partial z^2}$.

**Spherical coordinates** $(r, \theta, \phi)$ are related to Cartesian by $x = r\sin\theta\cos\phi$, $y = r\sin\theta\sin\phi$, $z = r\cos\theta$. They are adapted to problems with spherical symmetry: the hydrogen atom, the gravitational field of a spherical mass, the Schwarzschild black hole. The Laplacian in spherical coordinates is $\nabla^2 f = \frac{1}{r^2}\frac{\partial}{\partial r}\left(r^2\frac{\partial f}{\partial r}\right) + \frac{1}{r^2\sin\theta}\frac{\partial}{\partial\theta}\left(\sin\theta\frac{\partial f}{\partial\theta}\right) + \frac{1}{r^2\sin^2\theta}\frac{\partial^2 f}{\partial\phi^2}$.

The general framework is that of **orthogonal curvilinear coordinates**: coordinate systems $(q^1, q^2, q^3)$ in which the coordinate curves are mutually perpendicular everywhere, characterized by scale factors $h_i = |\partial\mathbf{r}/\partial q^i|$. The gradient, divergence, curl, and Laplacian all take standard forms in terms of these scale factors. This framework is the precursor to Riemannian geometry, where the metric tensor $g_{ij}$ replaces the scale factors and the coordinate curves need not be orthogonal.

---

## Subsections

- [8.5.1: Polar and Cylindrical Coordinates](8.5.1-cylindrical.md)
- [8.5.2: Spherical Coordinates](8.5.2-spherical.md)
- [8.5.3: Orthogonal Curvilinear Coordinates and Scale Factors](8.5.3-curvilinear.md)
- [8.5.4: Differential Operators in Curvilinear Coordinates](8.5.4-operators.md)
- [8.5.5: Connection to the Metric Tensor](8.5.5-metric.md)
