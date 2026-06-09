# Moments of Inertia

When a body rotates about an axis, its resistance to changes in angular velocity is measured by its **moment of inertia** — the rotational analogue of mass. Just as a larger mass resists linear acceleration more, a larger moment of inertia resists angular acceleration more. For a continuous body with density $\rho$, the moment of inertia about an axis is computed as a double or triple integral, weighting the density by the square of the distance from the axis. This integral is the bridge between the geometry of the body and its rotational dynamics.

## Definition

The **moment of inertia** of a lamina with density $\rho(x,y)$ about an axis $\ell$ is:

$$I_\ell = \iint_D d^2(x,y)\,\rho(x,y)\,dA,$$

where $d(x,y)$ is the perpendicular distance from the point $(x,y)$ to the axis $\ell$.

**About the $z$-axis** (perpendicular to the $xy$-plane at the origin): $d^2 = x^2+y^2$.

$$I_z = \iint_D(x^2+y^2)\rho(x,y)\,dA.$$

**About the $x$-axis** (the horizontal axis in the $xy$-plane): $d^2 = y^2$.

$$I_x = \iint_D y^2\,\rho(x,y)\,dA.$$

**About the $y$-axis:** $d^2 = x^2$.

$$I_y = \iint_D x^2\,\rho(x,y)\,dA.$$

Note that $I_z = I_x + I_y$ (the **perpendicular axis theorem** for laminas).

## Physical Significance

The rotational kinetic energy of a rigid body spinning about axis $\ell$ with angular velocity $\omega$ is:

$$T = \frac{1}{2}I_\ell\omega^2.$$

Newton's second law for rotation says: $\tau = I_\ell\alpha$, where $\tau$ is the net torque and $\alpha = d\omega/dt$ is the angular acceleration. A flywheel with large $I$ requires more torque to spin up but also stores more kinetic energy, which is why large flywheels are used as energy storage devices.

## Worked Example 1: Disk of Uniform Density

A circular disk of radius $R$ and uniform density $\rho = 1$ (mass per unit area). Find $I_z$.

$$I_z = \iint_{x^2+y^2\leq R^2}(x^2+y^2)\,dA = \int_0^{2\pi}\int_0^R r^2\cdot r\,dr\,d\theta = 2\pi\cdot\frac{R^4}{4} = \frac{\pi R^4}{2}.$$

The mass is $m = \pi R^2$, so $I_z = \frac{mR^2}{2}$. This is the standard formula: the moment of inertia of a solid disk about its center is $mR^2/2$.

## Worked Example 2: Rectangular Lamina

A rectangle $[0,a]\times[0,b]$ with uniform density $\rho = 1$. Find $I_x$ and $I_y$.

$I_x = \int_0^a\int_0^b y^2\,dy\,dx = a\cdot\frac{b^3}{3}$. Mass $m = ab$, so $I_x = \frac{mb^2}{3}$.

$I_y = \int_0^a\int_0^b x^2\,dy\,dx = b\cdot\frac{a^3}{3} = \frac{ma^2}{3}$.

$I_z = I_x+I_y = m\cdot\frac{a^2+b^2}{3}$.

## The Parallel Axis Theorem

If the moment of inertia about an axis through the center of mass is $I_{\text{cm}}$, and one wants the moment of inertia about a parallel axis at distance $d$ from the first, the result is:

$$I = I_{\text{cm}} + md^2,$$

where $m$ is the total mass. This theorem avoids recomputing $I$ for every axis: compute it once about the center of mass, then shift.

**Derivation.** Let the new axis be at distance $d$ from the center of mass axis, so $x' = x - \bar{x}$ (shifted coordinates). Then $I = \iint(x^2+y^2)\rho\,dA$ in original coords = $\iint((x'+\bar{x})^2+(y'+\bar{y})^2)\rho\,dA$. Expanding: $= I_{\text{cm}}+2\bar{x}\iint x'\rho\,dA+2\bar{y}\iint y'\rho\,dA+(\bar{x}^2+\bar{y}^2)\cdot m$. The cross terms $\iint x'\rho\,dA = \iint(x-\bar{x})\rho\,dA = m\bar{x}-m\bar{x}=0$ (by definition of center of mass). So $I = I_{\text{cm}}+md^2$ where $d^2=\bar{x}^2+\bar{y}^2$.

## Moments of Inertia for Solid Bodies

For a solid body $E$ with density $\rho(x,y,z)$:

$$I_z = \iiint_E(x^2+y^2)\rho\,dV, \quad I_x = \iiint_E(y^2+z^2)\rho\,dV, \quad I_y = \iiint_E(x^2+z^2)\rho\,dV.$$

**Example.** Solid ball of radius $R$ and uniform density $\rho$. Find $I_z$.

By symmetry, $I_x = I_y = I_z$ (the ball is spherically symmetric).

$I_z = \rho\iiint_{x^2+y^2+z^2\leq R^2}(x^2+y^2)\,dV = \rho\iiint(r^2)\,dV$ where $r^2=x^2+y^2=\rho_{\text{sph}}^2\sin^2\phi$.

In spherical: $\rho_{\text{sph}}^2\sin^2\phi\cdot\rho_{\text{sph}}^2\sin\phi\,d\rho_{\text{sph}}\,d\phi\,d\theta$:

$I_z = \rho\int_0^{2\pi}\int_0^\pi\int_0^R\rho_s^4\sin^3\phi\,d\rho_s\,d\phi\,d\theta = \rho\cdot 2\pi\cdot\frac{R^5}{5}\cdot\frac{4}{3} = \frac{8\pi\rho R^5}{15}$.

Mass: $m = \rho\cdot\frac{4\pi R^3}{3}$. So $I_z = \frac{8\pi\rho R^5}{15} = \frac{2mR^2}{5}$.

The standard result: the moment of inertia of a solid sphere about a diameter is $\frac{2}{5}mR^2$.

## The Inertia Tensor

For a fully general rigid body, the rotational dynamics about an arbitrary axis is described not by a single scalar moment of inertia but by the **inertia tensor**, a symmetric $3\times3$ matrix:

$$I_{ij} = \iiint_E\left(\delta_{ij}\sum_k x_k^2 - x_ix_j\right)\rho\,dV,$$

where $\delta_{ij}$ is the Kronecker delta. The diagonal entries $I_{11}, I_{22}, I_{33}$ are the moments of inertia about the coordinate axes; the off-diagonal entries are the **products of inertia**. The inertia tensor is diagonalizable (being symmetric), and its eigenvectors are the **principal axes** of inertia about which the body rotates most naturally.

This is the most general formulation of rotational dynamics, and it connects multiple integration with linear algebra in a beautiful way.
