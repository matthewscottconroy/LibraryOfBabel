# Statement and Proof of the Divergence Theorem

The Divergence Theorem relates the outward flux of a vector field through a closed surface to the integral of its divergence over the enclosed volume. The proof follows the same strategy as Green's Theorem: reduce to the one-variable Fundamental Theorem of Calculus applied in each coordinate direction, then assemble the result.

## Statement

**Divergence Theorem.** Let $V$ be a bounded region in $\mathbb{R}^3$ whose boundary $\partial V$ is a piecewise smooth closed surface, oriented with the outward unit normal $\hat{\mathbf{n}}$. Let $\mathbf{F} = (P, Q, R)$ be $C^1$ on an open set containing $\overline{V}$. Then

$$\oiint_{\partial V}\mathbf{F}\cdot d\mathbf{S} = \iiint_V\nabla\cdot\mathbf{F}\,dV = \iiint_V\left(\frac{\partial P}{\partial x} + \frac{\partial Q}{\partial y} + \frac{\partial R}{\partial z}\right)dV.$$

## Proof for a Box

Let $V = [a_1,b_1]\times[a_2,b_2]\times[a_3,b_3]$.

It suffices to prove the three separate equalities:

$$\oiint_{\partial V} P\,dydz = \iiint_V\frac{\partial P}{\partial x}\,dV, \quad \oiint_{\partial V} Q\,dzdx = \iiint_V\frac{\partial Q}{\partial y}\,dV, \quad \oiint_{\partial V} R\,dxdy = \iiint_V\frac{\partial R}{\partial z}\,dV.$$

We prove the first; the others are analogous.

**Proof of $\oiint_{\partial V} P\,dydz = \iiint_V\partial P/\partial x\,dV$:**

The six faces of the box: the faces $x = b_1$ (right) and $x = a_1$ (left) contribute to the $yz$-integral; the four other faces have $dydz = 0$ (the outward normal on those faces is in the $\pm y$ or $\pm z$ direction, not $x$).

On the right face ($x = b_1$, outward normal $+\mathbf{i}$): $\mathbf{F}\cdot d\mathbf{S} = P(b_1, y, z)\,dy\,dz$.

On the left face ($x = a_1$, outward normal $-\mathbf{i}$): $\mathbf{F}\cdot d\mathbf{S} = -P(a_1, y, z)\,dy\,dz$.

Sum:

$$\oiint_{\partial V} P\,dydz = \int_{a_2}^{b_2}\int_{a_3}^{b_3}[P(b_1,y,z) - P(a_1,y,z)]\,dydz.$$

By the one-variable FTC:

$$P(b_1,y,z) - P(a_1,y,z) = \int_{a_1}^{b_1}\frac{\partial P}{\partial x}(x,y,z)\,dx.$$

Therefore:

$$\oiint_{\partial V} P\,dydz = \int_{a_2}^{b_2}\int_{a_3}^{b_3}\int_{a_1}^{b_1}\frac{\partial P}{\partial x}\,dx\,dy\,dz = \iiint_V\frac{\partial P}{\partial x}\,dV.$$

Adding the three equalities completes the proof for the box.

## Proof for General Regions

A general smooth region $V$ can be approximated by unions of small boxes (in the limit). For a "simple" region (bounded by functions of two coordinates), the same argument works directly. For general regions, use a partition of unity to cover $\partial V$ with finitely many patches, each expressible as a graph over one of the coordinate planes, and apply the box proof to each patch. Interior interface contributions cancel in pairs, leaving only $\partial V$.

## Worked Examples

**Example 1.** Compute $\oiint_S\mathbf{F}\cdot d\mathbf{S}$ for $\mathbf{F} = (x^3, y^3, z^3)$ over the sphere $S: x^2+y^2+z^2 = 1$ with outward orientation.

Direct computation would require parametrizing the sphere and integrating — tedious. By the Divergence Theorem:

$\nabla\cdot\mathbf{F} = 3x^2 + 3y^2 + 3z^2 = 3(x^2+y^2+z^2) = 3r^2$.

$$\oiint_S\mathbf{F}\cdot d\mathbf{S} = \iiint_V 3r^2\,dV = 3\int_0^{2\pi}\int_0^\pi\int_0^1 r^2\cdot r^2\sin\phi\,dr\,d\phi\,d\theta = 3\cdot 2\pi\cdot 2\cdot\frac{1}{5} = \frac{12\pi}{5}.$$

**Example 2.** Compute the flux of $\mathbf{F} = (x,y,z)$ through the closed surface $\partial V$ where $V$ is the unit cube $[0,1]^3$.

$\nabla\cdot\mathbf{F} = 3$.

$$\oiint_{\partial V}\mathbf{F}\cdot d\mathbf{S} = \iiint_V 3\,dV = 3\cdot 1 = 3.$$

Direct verification: the six faces each have $\mathbf{F}\cdot\hat{\mathbf{n}} = $ one coordinate evaluated on that face. On the face $x=1$: $\hat{\mathbf{n}} = \mathbf{i}$, flux $= \iint 1\,dydz = 1$. On $x=0$: $\hat{\mathbf{n}} = -\mathbf{i}$, flux $= \iint(-0)\,dydz = 0$. Similarly for $y$ and $z$ faces. Total: $1+0+1+0+1+0 = 3$. Verified.

**Example 3: Using a Simpler Surface.** Compute $\oiint_S\mathbf{F}\cdot d\mathbf{S}$ for $\mathbf{F} = (e^{yz}, \sin(xz), \cos(xy))$ over the cube $[0,2]^3$.

$\nabla\cdot\mathbf{F} = 0 + 0 + 0 = 0$. So $\oiint_S\mathbf{F}\cdot d\mathbf{S} = 0$.

This illustrates the theorem's power: even though computing the flux directly would require integrating a complicated function over six faces, the result is immediately zero because the divergence vanishes.

## The Divergence Theorem and Gauss's Law

The electric field of a point charge $q$ at the origin, $\mathbf{E} = \frac{q}{4\pi\varepsilon_0}\frac{\hat{\mathbf{r}}}{r^2}$, has $\nabla\cdot\mathbf{E} = 0$ for $\mathbf{r}\neq\mathbf{0}$ (as computed earlier). For any closed surface $S$ not enclosing the origin:

$$\oiint_S\mathbf{E}\cdot d\mathbf{S} = \iiint_V\nabla\cdot\mathbf{E}\,dV = 0.$$

For any closed surface $S$ enclosing the origin: use the Divergence Theorem on the region between $S$ and a small sphere $S_\varepsilon$ around the origin (where $\nabla\cdot\mathbf{E}=0$):

$$\oiint_S\mathbf{E}\cdot d\mathbf{S} = \oiint_{S_\varepsilon}\mathbf{E}\cdot d\mathbf{S} = \frac{q}{\varepsilon_0}.$$

This is Gauss's law: $\oiint_S\mathbf{E}\cdot d\mathbf{S} = Q_{\text{enc}}/\varepsilon_0$.

## Summary

The Divergence Theorem converts outward flux through a closed surface to the volume integral of divergence. The proof reduces to three applications of the one-variable FTC. It is a powerful computational tool, often converting difficult surface integrals (six faces of a cube, complicated parametric surface) into much simpler volume integrals. Its physical interpretation — total outflow equals total source strength — is the mathematical form of conservation laws throughout physics.
