# Unit 03 Integration in Several Variables

Single-variable integration computes signed areas under curves. The natural extension accumulates function values over two- and three-dimensional regions: the double integral $\iint_D f(x,y)\,dA$ sums up $f$ over a planar region $D$, and the triple integral $\iiint_E f(x,y,z)\,dV$ sums over a solid region $E$. These integrals compute volumes, masses, centers of mass, moments of inertia, and probabilities, and they reduce to iterated single-variable integrals by Fubini's theorem. The change-of-variables formula, whose central ingredient is the Jacobian determinant introduced in Unit 2, makes polar, cylindrical, and spherical coordinates available for integration.

## What This Unit Covers

**Chapter 1 (Double Integrals)** develops integration over planar regions. The integral $\iint_D f(x,y)\,dA$ is defined as a limit of Riemann sums, analogously to the single-variable integral, and Fubini's theorem allows it to be computed as an iterated integral: $\int_a^b\int_{g_1(x)}^{g_2(x)} f(x,y)\,dy\,dx$. The change-of-variables formula for double integrals is derived using the Jacobian, and polar coordinates are treated as the primary example.

**Chapter 2 (Triple Integrals)** extends everything to three-dimensional regions. Fubini's theorem again reduces the triple integral to iterated integrals. Cylindrical coordinates $(r,\theta,z)$ and spherical coordinates $(\rho,\theta,\phi)$ are introduced, with their respective volume elements $r\,dr\,d\theta\,dz$ and $\rho^2\sin\phi\,d\rho\,d\theta\,d\phi$. The general change-of-variables theorem in $\mathbb{R}^3$ uses the Jacobian determinant of the transformation.

**Chapter 3 (Applications)** shows how double and triple integrals compute physically and geometrically meaningful quantities: areas of regions and volumes of solids, masses of laminas and solid bodies with variable density, centers of mass, moments of inertia (needed in mechanics for computing rotational kinetic energy and angular momentum), and probability distributions given by continuous density functions.

## How the Chapters Connect

Chapter 1 builds the foundations (definitions, Fubini, change of variables) in the two-dimensional case. Chapter 2 directly parallels Chapter 1 in three dimensions. Chapter 3 applies the machinery of both to a variety of problems. The most important theoretical connection is that the Jacobian determinant from Unit 2 appears as the distortion factor in the change-of-variables formula, giving the integration theory a natural connection to the differentiation theory developed earlier.

## How This Unit Fits into the Course

Integration over regions is the prerequisite for the vector calculus that follows: line integrals (integration over curves), surface integrals (integration over surfaces), and the theorems of Green, Stokes, and Gauss that connect them. Specifically, the area and volume calculations here are the simplest cases of the more general integration theory. The probability applications connect to the theory of distributions, which is the natural setting for the fundamental solutions of partial differential equations. Students who complete this unit understand the full apparatus of classical integration in $\mathbb{R}^n$, which is both practically indispensable and the foundation for the higher integration theory to come.
