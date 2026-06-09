# Chapter 02 Triple Integrals

The triple integral $\iiint_E f(x,y,z)\,dV$ accumulates the values of $f$ over a three-dimensional solid region $E$. It is the natural extension of the double integral to one higher dimension, and the same tools apply: Fubini's theorem reduces it to iterated single-variable integrals, and changes of variables (cylindrical or spherical coordinates) simplify the computation when the region or integrand has appropriate symmetry.

## What This Chapter Covers

**Section 1 (Iterated Triple Integrals)** reduces $\iiint_E f\,dV$ to an iterated integral $\int\int\int f\,dz\,dy\,dx$ (or any other ordering of the three variables). For a simple region $E$ described by $a\leq x\leq b$, $g_1(x)\leq y\leq g_2(x)$, $h_1(x,y)\leq z\leq h_2(x,y)$, the triple integral is $\int_a^b\int_{g_1(x)}^{g_2(x)}\int_{h_1(x,y)}^{h_2(x,y)}f\,dz\,dy\,dx$. Setting up the limits correctly requires carefully identifying the bounds on each variable in terms of those integrated after it.

**Section 2 (Cylindrical Coordinates)** applies the coordinate system $(r,\theta,z)$ where $x=r\cos\theta$, $y=r\sin\theta$, $z=z$. The volume element is $dV = r\,dr\,d\theta\,dz$. Cylindrical coordinates are effective for regions with circular cross-sections in the $xy$-plane, such as cylinders, cones (described by $z=r$), and paraboloids (described by $z=r^2$).

**Section 3 (Spherical Coordinates)** applies $(\rho,\theta,\phi)$ where $x=\rho\sin\phi\cos\theta$, $y=\rho\sin\phi\sin\theta$, $z=\rho\cos\phi$. The volume element is $dV = \rho^2\sin\phi\,d\rho\,d\theta\,d\phi$. Spherical coordinates simplify integration over balls, spherical shells, and cones, and over integrands that depend only on the distance from the origin.

**Section 4 (General Change of Variables)** states the change-of-variables formula in $\mathbb{R}^3$: $\iiint_E f(\mathbf{x})\,dV = \iiint_{E^*}f(\mathbf{g}(\mathbf{u}))|\det J_\mathbf{g}(\mathbf{u})|\,dV^*$. Both cylindrical and spherical coordinate transformations are special cases, with Jacobian determinants $r$ and $\rho^2\sin\phi$ respectively.

## How the Sections Build on Each Other

The chapter parallels Chapter 1 exactly: Section 1 (iterated integrals) corresponds to the double-integral iterated integral, Sections 2 and 3 are the three-dimensional analogues of polar coordinates, and Section 4 is the three-dimensional change-of-variables theorem. The progression is: set up the integral in Cartesian, then use coordinate transformations to simplify.

## How This Chapter Fits into the Unit

Triple integrals are the direct extension of double integrals and are needed for all three-dimensional applications in Chapter 3. The coordinate systems introduced here (cylindrical and spherical) appear repeatedly in physics: the heat equation, wave equation, and Laplace equation are all most naturally written and solved in the coordinate system that matches the geometry of the problem. Students who are fluent in setting up and evaluating triple integrals in all three coordinate systems will find the transition to partial differential equations much more natural.
