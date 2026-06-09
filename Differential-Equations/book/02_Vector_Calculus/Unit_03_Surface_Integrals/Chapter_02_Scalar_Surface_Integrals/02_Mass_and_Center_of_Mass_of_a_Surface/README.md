# Mass and Center of Mass of a Surface

A thin curved shell — a bowl, a dome, a soap bubble — has mass distributed over a two-dimensional surface. If the surface mass density (mass per unit area) $\rho(\mathbf{p})$ is known at each point $\mathbf{p}$ on the surface $S$, then the total mass and center of mass are determined by scalar surface integrals. This section develops these applications and connects them to the analogous formulas for wires (line integrals) and solid bodies (triple integrals).

## Total Mass

The **total mass** of a thin shell with surface $S$ and surface mass density $\rho: S \to \mathbb{R}$ (in units of mass per area) is

$$M = \iint_S \rho\,dS.$$

**Uniform density.** If $\rho$ is constant, $M = \rho \cdot A(S)$ — mass equals density times area. The surface integral reduces to the surface area computation.

**Non-uniform density.** For varying $\rho$, the integral must be evaluated by parametrizing the surface: $M = \iint_D \rho(\mathbf{r}(u,v))\,|\mathbf{r}_u\times\mathbf{r}_v|\,du\,dv$.

## Center of Mass

The **center of mass** (centroid weighted by density) has coordinates

$$\bar{x} = \frac{1}{M}\iint_S x\,\rho\,dS, \quad \bar{y} = \frac{1}{M}\iint_S y\,\rho\,dS, \quad \bar{z} = \frac{1}{M}\iint_S z\,\rho\,dS.$$

For a surface with uniform density, the center of mass is the **centroid** of the surface:

$$\bar{x} = \frac{1}{A(S)}\iint_S x\,dS, \quad \bar{y} = \frac{1}{A(S)}\iint_S y\,dS, \quad \bar{z} = \frac{1}{A(S)}\iint_S z\,dS.$$

## Worked Example: Centroid of a Hemisphere

Find the centroid of the upper hemisphere $S: x^2+y^2+z^2 = a^2$, $z \geq 0$, with uniform density.

**By symmetry,** $\bar{x} = \bar{y} = 0$ (the hemisphere is symmetric about the $z$-axis).

**Area:** $A = 2\pi a^2$ (half the sphere).

**Compute $\iint_S z\,dS$:** From the previous section, $\iint_S z\,dS = \pi a^3$.

$$\bar{z} = \frac{\pi a^3}{2\pi a^2} = \frac{a}{2}.$$

The centroid of the hemisphere is at height $a/2$ above the equatorial plane — halfway between the equator and the pole.

## Worked Example: Mass with Non-Uniform Density

Find the mass of the spherical shell $x^2+y^2+z^2 = a^2$ with density $\rho(x,y,z) = |z|$.

$$M = \iint_S |z|\,dS.$$

By symmetry (upper and lower hemispheres contribute equally):

$$M = 2\iint_{S^+} z\,dS = 2\pi a^3,$$

using $\iint_{S^+} z\,dS = \pi a^3$ from the previous section.

## Moments of Inertia

The **moment of inertia** of the shell about the $z$-axis is

$$I_z = \iint_S (x^2+y^2)\,\rho\,dS.$$

For the uniform-density sphere ($\rho = 1$, area $4\pi a^2$):

$$I_z = \iint_S (x^2+y^2)\,dS.$$

By symmetry, $\iint_S x^2\,dS = \iint_S y^2\,dS = \iint_S z^2\,dS = \frac{1}{3}\iint_S(x^2+y^2+z^2)\,dS = \frac{a^2}{3}\cdot 4\pi a^2 = \frac{4\pi a^4}{3}$.

So $I_z = \iint_S x^2\,dS + \iint_S y^2\,dS = \frac{8\pi a^4}{3}$.

For a shell of total mass $M = 4\pi a^2\rho$, the moment of inertia is $I_z = \frac{2}{3}Ma^2$ — a formula used in rigid body mechanics for the rotation of a spherical shell.

## Analogy with Other Dimensions

| Setting | Mass | Center of mass |
|---|---|---|
| Curve $C$ (wire) | $M = \int_C\rho\,ds$ | $\bar{x} = \frac{1}{M}\int_C x\rho\,ds$ |
| Surface $S$ (shell) | $M = \iint_S\rho\,dS$ | $\bar{x} = \frac{1}{M}\iint_S x\rho\,dS$ |
| Solid $E$ (body) | $M = \iiint_E\rho\,dV$ | $\bar{x} = \frac{1}{M}\iiint_E x\rho\,dV$ |

The pattern is uniform across dimensions: integrate the density against the appropriate measure (arc length, area, volume).

## Summary

The mass of a thin curved shell is $M = \iint_S\rho\,dS$, and its center of mass is given by the weighted average of position. For uniform density, these reduce to purely geometric quantities (centroid). The moment of inertia about an axis is the integral of $(\text{distance to axis})^2\cdot\rho$. All these computations reduce to ordinary double integrals via surface parametrization and are independent of the orientation of the surface.
