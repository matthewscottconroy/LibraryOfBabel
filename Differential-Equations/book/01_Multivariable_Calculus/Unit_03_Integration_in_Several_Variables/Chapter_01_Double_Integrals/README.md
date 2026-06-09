# Chapter 01 Double Integrals

The definite integral $\int_a^b f(x)\,dx$ accumulates the values of $f$ along a one-dimensional interval. When $f$ is a non-negative function, the integral computes the area under the curve. The **double integral** $\iint_D f(x,y)\,dA$ accumulates the values of $f$ over a two-dimensional region $D$. When $f\geq 0$, it computes the volume of the solid lying under the surface $z=f(x,y)$ and above $D$. But its applications extend far beyond geometry: it computes mass, charge, probability, and other physical quantities distributed over a plane region.

## What This Chapter Covers

**Section 1 (Iterated Integrals)** shows how a double integral over a region can be computed as two successive single-variable integrations. For a rectangle $[a,b]\times[c,d]$, the iterated integral $\int_a^b\left[\int_c^d f(x,y)\,dy\right]dx$ fixes $x$ and integrates over $y$ first, then integrates the result over $x$. For more general regions (bounded by curves rather than straight lines), the limits of the inner integral depend on $x$: $\int_a^b\int_{g_1(x)}^{g_2(x)} f(x,y)\,dy\,dx$ for a vertically simple region, or with the order reversed for a horizontally simple region.

**Section 2 (Fubini's Theorem)** establishes that the double integral equals the iterated integral, and that the order of integration can be switched (under appropriate hypotheses). For continuous $f$ on a rectangle, $\iint_R f\,dA = \int_a^b\int_c^d f\,dy\,dx = \int_c^d\int_a^b f\,dx\,dy$. This equality of the two iterated integrals is the multivariable version of the fact that the area under a surface is the same whether you sweep it in the $x$- or $y$-direction.

**Section 3 (Change of Variables and Jacobian)** derives the change-of-variables formula: if $(x,y) = \mathbf{g}(u,v)$ is a smooth bijection, then $\iint_D f(x,y)\,dA = \iint_{D^*}f(\mathbf{g}(u,v))|\det J_\mathbf{g}(u,v)|\,du\,dv$. The Jacobian determinant $|\det J_\mathbf{g}|$ is the area-scaling factor of the transformation.

**Section 4 (Polar Coordinates)** applies the change-of-variables formula with $x=r\cos\theta$, $y=r\sin\theta$. The Jacobian determinant is $r$, giving the area element $dA = r\,dr\,d\theta$. Polar coordinates simplify integration over circles, disks, and sectors, and are essential for many applications.

## How the Sections Build on Each Other

Sections 1 and 2 are the computational core: they establish that double integrals can be evaluated by iterated single-variable integration. Section 3 provides the theoretical tool (the change-of-variables formula) that extends the computational toolkit to non-Cartesian coordinate systems. Section 4 is the primary application of Section 3, demonstrating how dramatically the right coordinate system can simplify an integral.

## How This Chapter Fits into the Unit

Double integrals are the two-dimensional case of the integration theory; Chapter 2 extends everything to three dimensions. Chapter 3 applies both chapters to compute areas, volumes, masses, and probabilities. The change-of-variables formula of Section 3 reappears in Chapter 2 (for cylindrical and spherical coordinates) and is the key computational tool for the entire integration unit.
