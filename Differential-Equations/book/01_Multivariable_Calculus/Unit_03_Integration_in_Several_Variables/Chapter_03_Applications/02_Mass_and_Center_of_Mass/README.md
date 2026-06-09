# Mass and Center of Mass

A planar lamina made of a material with varying composition has a mass density $\rho(x,y)$ (mass per unit area) that differs from point to point. The total mass and the point at which the mass is effectively concentrated (the center of mass) are computed by integrating $\rho$ against the appropriate functions over the lamina. These computations, while elementary in concept, require the full power of double integrals when the density is not constant, and they generalize directly to solid bodies using triple integrals.

## Mass of a Lamina

A **lamina** (thin flat body) occupies a region $D\subseteq\mathbb{R}^2$ with density function $\rho: D\to[0,\infty)$ (mass per unit area, in kg/m$^2$ say). The **total mass** is:

$$m = \iint_D\rho(x,y)\,dA.$$

For a uniform lamina ($\rho = \text{const}$), $m = \rho\cdot A(D)$, as expected.

**Example.** A triangular lamina with vertices $(0,0)$, $(1,0)$, $(0,1)$ has density $\rho(x,y) = x+y$. Find its mass.

$D: x\geq 0$, $y\geq 0$, $x+y\leq 1$.

$m = \int_0^1\int_0^{1-x}(x+y)\,dy\,dx = \int_0^1\left[xy+\frac{y^2}{2}\right]_0^{1-x}dx = \int_0^1\left[x(1-x)+\frac{(1-x)^2}{2}\right]dx$

$= \int_0^1\left[x-x^2+\frac{1-2x+x^2}{2}\right]dx = \int_0^1\left[\frac{1}{2}+\frac{x^2-2x^2+2x-2x}{2}+\cdots\right]$...

Let me compute directly: $x(1-x)+\frac{(1-x)^2}{2} = x-x^2+\frac{1-2x+x^2}{2} = x-x^2+\frac{1}{2}-x+\frac{x^2}{2} = \frac{1}{2}-\frac{x^2}{2}$.

$m = \int_0^1\left(\frac{1}{2}-\frac{x^2}{2}\right)dx = \left[\frac{x}{2}-\frac{x^3}{6}\right]_0^1 = \frac{1}{2}-\frac{1}{6} = \frac{1}{3}$.

## Center of Mass: First Moments

The **first moments** of the lamina about the $x$- and $y$-axes are:

$$M_x = \iint_D y\,\rho(x,y)\,dA, \quad M_y = \iint_D x\,\rho(x,y)\,dA.$$

(Note: $M_x$ involves $y$ and $M_y$ involves $x$ — the notation refers to the axis about which the moment is computed, not the variable in the integrand.)

The **center of mass** $(\bar{x},\bar{y})$ is:

$$\bar{x} = \frac{M_y}{m} = \frac{1}{m}\iint_D x\,\rho\,dA, \quad \bar{y} = \frac{M_x}{m} = \frac{1}{m}\iint_D y\,\rho\,dA.$$

Geometrically, the center of mass is the point at which the lamina, if balanced on a knife edge, would be in equilibrium.

**Continuing the triangular lamina example:**

$M_y = \int_0^1\int_0^{1-x}x(x+y)\,dy\,dx = \int_0^1 x\left[xy+\frac{y^2}{2}\right]_0^{1-x}dx = \int_0^1 x\left(\frac{1}{2}-\frac{x^2}{2}\right)dx$

(using the computation from above) $= \int_0^1\left(\frac{x}{2}-\frac{x^3}{2}\right)dx = \left[\frac{x^2}{4}-\frac{x^4}{8}\right]_0^1 = \frac{1}{4}-\frac{1}{8} = \frac{1}{8}$.

$\bar{x} = M_y/m = (1/8)/(1/3) = 3/8$.

By the symmetry of $\rho(x,y) = x+y$ under swapping $x$ and $y$ (and the triangle is symmetric in $x$ and $y$), $\bar{y} = 3/8$ as well.

## Centroid

For a uniform lamina ($\rho = 1$), the center of mass reduces to the **centroid**:

$$\bar{x} = \frac{1}{A(D)}\iint_D x\,dA, \quad \bar{y} = \frac{1}{A(D)}\iint_D y\,dA.$$

The centroid is the geometric center of the region, independent of any physical density.

**Example.** Centroid of a semicircle $x^2+y^2\leq R^2$, $y\geq 0$.

$A = \pi R^2/2$. $\bar{x} = 0$ by symmetry.

$\bar{y} = \frac{1}{\pi R^2/2}\iint y\,dA = \frac{2}{\pi R^2}\int_0^{\pi}\int_0^R r\sin\theta\cdot r\,dr\,d\theta = \frac{2}{\pi R^2}\int_0^{\pi}\sin\theta\,d\theta\cdot\frac{R^3}{3} = \frac{2}{\pi R^2}\cdot 2\cdot\frac{R^3}{3} = \frac{4R}{3\pi}$.

## Mass and Center of Mass in Three Dimensions

For a solid body occupying region $E\subseteq\mathbb{R}^3$ with volume density $\rho(x,y,z)$ (mass per unit volume):

$$m = \iiint_E\rho\,dV, \quad \bar{x} = \frac{1}{m}\iiint_E x\rho\,dV, \quad \bar{y} = \frac{1}{m}\iiint_E y\rho\,dV, \quad \bar{z} = \frac{1}{m}\iiint_E z\rho\,dV.$$

**Example.** A solid hemisphere $x^2+y^2+z^2\leq R^2$, $z\geq 0$, with uniform density $\rho=1$. Find the center of mass.

$m = \frac{2\pi R^3}{3}$ (half the sphere volume).

$\bar{z} = \frac{1}{m}\iiint_E z\,dV$. In spherical: $z=\rho\cos\phi$, $dV=\rho^2\sin\phi\,d\rho\,d\phi\,d\theta$, $0\leq\phi\leq\pi/2$.

$\iiint z\,dV = \int_0^{2\pi}\int_0^{\pi/2}\int_0^R\rho\cos\phi\cdot\rho^2\sin\phi\,d\rho\,d\phi\,d\theta = 2\pi\cdot\frac{R^4}{4}\int_0^{\pi/2}\sin\phi\cos\phi\,d\phi = 2\pi\cdot\frac{R^4}{4}\cdot\frac{1}{2} = \frac{\pi R^4}{4}$.

$\bar{z} = \frac{\pi R^4/4}{2\pi R^3/3} = \frac{3R}{8}$.

The centroid of a uniform solid hemisphere is $3R/8$ above the flat face, closer to the flat face than to the curved top.

## Physical Interpretation: Balance and Equilibrium

The center of mass is the balance point of the body: a rigid body in a uniform gravitational field can be balanced at the center of mass. For a two-dimensional lamina, the body balances on a knife edge through $(\bar{x},\bar{y})$ parallel to either coordinate axis. This is why the center of mass is also called the **centroid** (in the uniform case) and the center of gravity (in a uniform gravitational field).

In differential equations and mechanics, the motion of the center of mass of a system of particles obeys Newton's laws as if all the mass were concentrated at that point and all external forces acted there. This is the fundamental simplification that makes the theory of rigid body motion tractable.
