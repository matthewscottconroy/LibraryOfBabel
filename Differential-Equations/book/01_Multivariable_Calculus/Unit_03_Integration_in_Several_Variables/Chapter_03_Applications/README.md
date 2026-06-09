# Chapter 03 Applications

The double and triple integral machinery developed in the previous two chapters finds its justification in a wide range of concrete applications. A function $f(x,y)$ can represent a temperature, a charge density, a height, a mass density, or a probability density — and integrating it over a region computes the total temperature (weighted by area), total charge, volume, total mass, or total probability. This chapter develops four families of applications: areas and volumes, mass and center of mass, moments of inertia, and probability density functions.

## What This Chapter Covers

**Section 1 (Area and Volume)** uses double and triple integrals to compute areas of planar regions ($A = \iint_D\,dA$) and volumes of solids ($V = \iiint_E\,dV$). The integral of $f(x,y)$ over a planar region also computes the volume of the solid between the $xy$-plane and the surface $z=f(x,y)$.

**Section 2 (Mass and Center of Mass)** computes the total mass of a lamina (planar body) with area density $\rho(x,y)$ as $m = \iint_D\rho\,dA$, and the center of mass at $(\bar{x},\bar{y})$ where $\bar{x} = \frac{1}{m}\iint_D x\rho\,dA$ and $\bar{y} = \frac{1}{m}\iint_D y\rho\,dA$. The corresponding formulas for three-dimensional solid bodies use triple integrals.

**Section 3 (Moments of Inertia)** computes the moment of inertia of a body about an axis. For a lamina with density $\rho$, the moment of inertia about the $z$-axis (perpendicular to the lamina) is $I_z = \iint_D(x^2+y^2)\rho\,dA$. The moment of inertia governs rotational dynamics: the kinetic energy of a rotating body is $\frac{1}{2}I\omega^2$.

**Section 4 (Probability Density Functions)** interprets a non-negative function $f(x,y)$ with $\iint_{\mathbb{R}^2}f\,dA = 1$ as the joint probability density of two random variables $(X,Y)$. The probability that $(X,Y)$ falls in a region $D$ is $\int\int_D f(x,y)\,dA$. The expected values and variances of $X$ and $Y$ are computed as integrals against $f$.

## How the Sections Connect

All four sections use the same basic machinery: the double or triple integral. What varies is the interpretation of the integrand ($\rho$ for mass, $(x^2+y^2)\rho$ for moment of inertia, $f$ for probability) and the quantity being computed. The center-of-mass formulas can be viewed as computing the expected values of $X$ and $Y$ under the "probability distribution" $\rho/m$, so mass and probability are formally the same — a normalized measure. The parallel becomes explicit in Section 4.

## How This Chapter Fits into the Unit

Applications are the reason the integration machinery was developed. The physical applications — mass, center of mass, moments of inertia — connect multivariable calculus to classical mechanics. The probability applications connect to statistics and to the theory of partial differential equations, where solutions are often expressed as convolutions with probability densities (fundamental solutions / Green's functions). Students who work through this chapter gain both computational fluency and an appreciation for why multiple integration is indispensable in science and engineering.
