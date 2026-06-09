# Flux Through a Surface

Water flowing in a river has a velocity at each point. If you place a net across part of the river, some water passes through in the direction the net faces. The rate at which water volume crosses the net is the flux of the velocity field through the net. For a flat net perpendicular to a uniform flow $\mathbf{v}$, the flux is simply $|\mathbf{v}|\cdot A$ (speed times area). For a tilted net, only the component of $\mathbf{v}$ perpendicular to the net contributes: the flux is $(\mathbf{v}\cdot\hat{\mathbf{n}})\cdot A$. For a curved net in a non-uniform flow, we integrate this dot product over the surface.

## Definition

Let $S$ be an oriented surface with unit normal $\hat{\mathbf{n}}$, and $\mathbf{F}: D \to \mathbb{R}^3$ a continuous vector field on a region $D$ containing $S$. The **flux** of $\mathbf{F}$ through $S$ is

$$\Phi = \iint_S\mathbf{F}\cdot d\mathbf{S} = \iint_S (\mathbf{F}\cdot\hat{\mathbf{n}})\,dS.$$

The integrand $\mathbf{F}\cdot\hat{\mathbf{n}}$ is the **normal component** of $\mathbf{F}$: the component in the direction perpendicular to the surface at each point.

## Computational Formula

With parametrization $\mathbf{r}(u,v)$ and $\mathbf{N} = \mathbf{r}_u\times\mathbf{r}_v$:

$$\Phi = \iint_D \mathbf{F}(\mathbf{r}(u,v))\cdot\mathbf{N}(u,v)\,du\,dv.$$

Note: here $\mathbf{N} = \mathbf{r}_u\times\mathbf{r}_v$ (not the unit normal), and the magnitude $|\mathbf{N}|$ is absorbed into the formula because $\hat{\mathbf{n}}\,dS = (\mathbf{N}/|\mathbf{N}|)\cdot|\mathbf{N}|\,du\,dv = \mathbf{N}\,du\,dv$.

**For graph surfaces $z = g(x,y)$** with the upward orientation ($\mathbf{N} = (-g_x, -g_y, 1)$):

$$\Phi = \iint_{D_{xy}}(-P g_x - Q g_y + R)\,dx\,dy,$$

where $\mathbf{F} = (P,Q,R)$ evaluated at $(x, y, g(x,y))$.

## Worked Examples

**Example 1: Flux through a flat horizontal surface.** Let $S$ be the square $[0,1]\times[0,1]$ in the $z=0$ plane with upward normal $\hat{\mathbf{n}} = \mathbf{k}$, and $\mathbf{F}(x,y,z) = x\,\mathbf{i} + y\,\mathbf{j} + z\,\mathbf{k}$.

$\mathbf{F}\cdot\hat{\mathbf{n}} = z = 0$ on $S$ (since $z=0$ on $S$).

$\Phi = \iint_S 0\,dS = 0$.

Interpretation: the field $\mathbf{F}$ is horizontal at $z=0$ (pointing radially outward in the $xy$-plane), so it does not cross the horizontal plane at all.

**Example 2: Flux through a sphere.** Let $\mathbf{F}(x,y,z) = x\,\mathbf{i} + y\,\mathbf{j} + z\,\mathbf{k} = \mathbf{r}$ and $S$ the sphere of radius $a$ with outward orientation.

$\mathbf{N} = a^2(\sin^2\phi\cos\theta, \sin^2\phi\sin\theta, \sin\phi\cos\phi)$ from the spherical parametrization.

$\mathbf{F}(\mathbf{r}(\phi,\theta)) = a(\sin\phi\cos\theta, \sin\phi\sin\theta, \cos\phi)$.

$\mathbf{F}\cdot\mathbf{N} = a\cdot a^2(\sin^3\phi\cos^2\theta + \sin^3\phi\sin^2\theta + \sin\phi\cos^2\phi) = a^3(\sin^3\phi + \sin\phi\cos^2\phi) = a^3\sin\phi(\sin^2\phi+\cos^2\phi) = a^3\sin\phi$.

$\Phi = \int_0^{2\pi}\int_0^\pi a^3\sin\phi\,d\phi\,d\theta = 2\pi a^3\cdot 2 = 4\pi a^3$.

Alternatively: $\mathbf{F}\cdot\hat{\mathbf{n}} = \mathbf{r}\cdot\hat{\mathbf{r}} = a$ (constant!) on the sphere of radius $a$. So $\Phi = a\cdot A(S) = a\cdot 4\pi a^2 = 4\pi a^3$.

**Example 3: Flux through a cylinder.** Let $\mathbf{F}(x,y,z) = x\,\mathbf{i} + y\,\mathbf{j}$ and $S$ the lateral surface of the cylinder $x^2+y^2=1$, $0\leq z\leq 1$, with outward orientation.

Parametrize: $\mathbf{r}(\theta,z) = (\cos\theta, \sin\theta, z)$, $\mathbf{N} = \mathbf{r}_\theta\times\mathbf{r}_z = (\cos\theta, \sin\theta, 0)$.

$\mathbf{F}(\mathbf{r}) = (\cos\theta, \sin\theta, 0)$.

$\mathbf{F}\cdot\mathbf{N} = \cos^2\theta + \sin^2\theta = 1$.

$\Phi = \int_0^{2\pi}\int_0^1 1\,dz\,d\theta = 2\pi$.

## The Normal Component

The geometric content of the flux integral is in the factor $\mathbf{F}\cdot\hat{\mathbf{n}}$:

- When $\mathbf{F}$ is parallel to $\hat{\mathbf{n}}$, $\mathbf{F}\cdot\hat{\mathbf{n}} = |\mathbf{F}|$: maximum flux.
- When $\mathbf{F}$ is perpendicular to $\hat{\mathbf{n}}$ (tangent to the surface), $\mathbf{F}\cdot\hat{\mathbf{n}} = 0$: no flux.
- When $\mathbf{F}$ is anti-parallel to $\hat{\mathbf{n}}$, $\mathbf{F}\cdot\hat{\mathbf{n}} = -|\mathbf{F}|$: negative flux (flow against the orientation).

A field tangent to the surface at every point has zero flux through it. This is the key reason that the magnetic field $\mathbf{B}$ has zero flux through any closed surface ($\nabla\cdot\mathbf{B} = 0$): by the Divergence Theorem, this is equivalent to saying $\mathbf{B}$ has no sources or sinks anywhere.

## Flux of the Inverse-Square Field

Let $\mathbf{F} = \mathbf{r}/|\mathbf{r}|^3$ (the Coulomb/gravitational field). For the sphere $S_a$ of radius $a$ with outward normal:

$\mathbf{F}\cdot\hat{\mathbf{n}} = (\mathbf{r}/a^3)\cdot(\mathbf{r}/a) = |\mathbf{r}|^2/a^4 = a^2/a^4 = 1/a^2$.

$\Phi_a = (1/a^2)\cdot 4\pi a^2 = 4\pi$.

The flux is $4\pi$ regardless of the radius $a$! This remarkable fact reflects the zero divergence of $\mathbf{F}$ away from the origin: there are no sources or sinks in the annular region between two spheres, so no net flux accumulates or dissipates — the same total flux passes through every sphere.

## Summary

The flux integral $\iint_S\mathbf{F}\cdot d\mathbf{S}$ measures the net rate at which $\mathbf{F}$ passes through the oriented surface $S$. It is computed as $\iint_D\mathbf{F}(\mathbf{r}(u,v))\cdot(\mathbf{r}_u\times\mathbf{r}_v)\,du\,dv$. Only the normal component of $\mathbf{F}$ contributes. The sign depends on orientation. The Divergence Theorem, proved in Unit 4, will relate the total outward flux through any closed surface to the integral of divergence inside — converting flux computation from a surface problem to a volume problem.
