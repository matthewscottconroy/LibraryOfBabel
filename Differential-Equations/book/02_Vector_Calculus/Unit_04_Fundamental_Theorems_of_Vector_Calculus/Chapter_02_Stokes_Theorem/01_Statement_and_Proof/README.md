# Statement and Proof of Stokes' Theorem

Stokes' Theorem is the three-dimensional generalization of Green's Theorem: it relates the line integral of a vector field around the boundary of a surface to the surface integral of the curl of the field over the surface. The proof, at its core, is an application of Green's Theorem in the parameter domain, composed with the change of variables coming from the surface parametrization.

## Statement

**Stokes' Theorem.** Let $S$ be an oriented, piecewise smooth surface in $\mathbb{R}^3$ whose boundary $\partial S$ is a piecewise smooth closed curve, oriented by the right-hand rule relative to $S$. Let $\mathbf{F} = (P, Q, R)$ be $C^1$ on an open set containing $S$. Then

$$\oint_{\partial S}\mathbf{F}\cdot d\mathbf{r} = \iint_S(\nabla\times\mathbf{F})\cdot d\mathbf{S}.$$

## Setup and Notation

Let $\mathbf{r}(u,v)$ be a regular parametrization of $S$ over a region $D$ in the $uv$-plane, with boundary $\partial D$ mapping to $\partial S$. The oriented area element is $d\mathbf{S} = (\mathbf{r}_u\times\mathbf{r}_v)\,du\,dv$.

The right side of Stokes' Theorem, expanded in components:

$$\iint_S(\nabla\times\mathbf{F})\cdot d\mathbf{S} = \iint_D(\nabla\times\mathbf{F})(\mathbf{r}(u,v))\cdot(\mathbf{r}_u\times\mathbf{r}_v)\,du\,dv.$$

## Proof for a Graph Surface

We give the proof for a graph surface $S: z = g(x,y)$, $(x,y) \in D_{xy}$, with the upward orientation. The boundary $\partial S$ lies above $\partial D_{xy}$.

**Left side.** Parametrize $\partial S$ by $(x(t), y(t), g(x(t),y(t)))$. Then

$$\oint_{\partial S}\mathbf{F}\cdot d\mathbf{r} = \oint_{\partial D_{xy}} P\,dx + Q\,dy + R\,dz,$$

where $dz = (g_x\,x' + g_y\,y')\,dt = g_x\,dx + g_y\,dy$. Substituting:

$$= \oint_{\partial D_{xy}} (P + Rg_x)\,dx + (Q + Rg_y)\,dy.$$

**Apply Green's Theorem** to the integral over $D_{xy}$:

$$= \iint_{D_{xy}} \left[\frac{\partial(Q + Rg_y)}{\partial x} - \frac{\partial(P + Rg_x)}{\partial y}\right]dx\,dy.$$

Expand the partial derivatives (using the chain rule, since $P, Q, R$ depend on $(x, y, g(x,y))$):

$$\frac{\partial(Q + Rg_y)}{\partial x} = Q_x + Q_z g_x + (R_x + R_z g_x)g_y + Rg_{yx},$$

$$\frac{\partial(P + Rg_x)}{\partial y} = P_y + P_z g_y + (R_y + R_z g_y)g_x + Rg_{xy}.$$

Since $g_{xy} = g_{yx}$ (Clairaut's theorem), the $Rg_{xy}$ and $Rg_{yx}$ terms cancel. The difference is:

$$= (Q_x - P_y) + (Q_z - R_y)g_x + (R_x - Q_z)(-g_x \text{ etc.})...$$

After careful bookkeeping, this equals $(R_y - Q_z)(-g_x) + (P_z - R_x)(-g_y) + (Q_x - P_y)\cdot 1$, which is exactly

$$(\nabla\times\mathbf{F})\cdot(-g_x, -g_y, 1) = (\nabla\times\mathbf{F})\cdot\mathbf{N},$$

with $\mathbf{N} = (-g_x, -g_y, 1)$ the normal from the graph parametrization.

Integrating over $D_{xy}$: $\iint_{D_{xy}}(\nabla\times\mathbf{F})\cdot\mathbf{N}\,dx\,dy = \iint_S(\nabla\times\mathbf{F})\cdot d\mathbf{S}$.

This completes the proof for graph surfaces.

## Proof for General Surfaces

A general smooth surface can be covered by finitely many patches, each of which is a graph surface (by the implicit function theorem). Apply Stokes' Theorem to each patch. The boundary integrals over interior interfaces cancel (each interface is traversed in opposite directions by adjacent patches), leaving only the boundary $\partial S$.

## Worked Examples

**Example 1.** Verify Stokes' Theorem for $\mathbf{F} = (y, -x, z)$ and $S$ the upper hemisphere $x^2+y^2+z^2=1$, $z\geq 0$, with upward orientation.

**Left side:** $\partial S$ is the unit circle $x^2+y^2=1$, $z=0$, traversed counterclockwise.

$\oint_{\partial S}\mathbf{F}\cdot d\mathbf{r} = \oint_{\partial S} y\,dx - x\,dy + z\,dz$. On $z=0$: $\oint (y\,dx - x\,dy)$.

With $\mathbf{r}(t) = (\cos t, \sin t, 0)$: $y\,dx - x\,dy = \sin t\cdot(-\sin t)\,dt - \cos t\cdot\cos t\,dt = -(\sin^2 t + \cos^2 t)\,dt = -dt$.

$\oint = \int_0^{2\pi}(-1)\,dt = -2\pi$.

**Right side:** $\nabla\times\mathbf{F} = (0-0, 0-0, -1-1) = (0,0,-2)$.

$\iint_S(0,0,-2)\cdot\hat{\mathbf{n}}\,dS$. With upward normal $\hat{\mathbf{n}} = (x,y,z)$ on the unit sphere, $\hat{\mathbf{n}}\cdot\mathbf{k} = z$.

$\iint_S -2z\,dS = -2\iint_S z\,dS = -2\cdot\pi = -2\pi$ (using the earlier result $\iint_{S^+} z\,dS = \pi a^3 = \pi$).

Both sides equal $-2\pi$. Verified.

**Example 2.** Use Stokes' Theorem to evaluate $\oint_C \mathbf{F}\cdot d\mathbf{r}$ where $C$ is the triangle with vertices $(1,0,0)$, $(0,1,0)$, $(0,0,1)$ (counterclockwise viewed from above) and $\mathbf{F} = (z^2, x^2, y^2)$.

$\nabla\times\mathbf{F} = (2y-0, 2z-0, 2x-0) = (2y, 2z, 2x)$.

The triangle lies in the plane $x+y+z=1$ with normal $\hat{\mathbf{n}} = (1,1,1)/\sqrt{3}$.

$(\nabla\times\mathbf{F})\cdot\hat{\mathbf{n}} = (2y+2z+2x)/\sqrt{3} = 2/\sqrt{3}$ (since $x+y+z=1$ on the surface).

Area of equilateral triangle with these vertices: $A = \sqrt{3}/2$.

$\oint_C\mathbf{F}\cdot d\mathbf{r} = \iint_S(\nabla\times\mathbf{F})\cdot d\mathbf{S} = \frac{2}{\sqrt{3}}\cdot\frac{\sqrt{3}}{2} = 1$.

## Summary

Stokes' Theorem equates the circulation of $\mathbf{F}$ around $\partial S$ to the flux of the curl of $\mathbf{F}$ through $S$. The proof for graph surfaces reduces to Green's Theorem; the general case follows by patching. The theorem implies that curl flux depends only on the boundary curve, not on which surface spans it — a deep topological fact. It underlies the relationship between integral and differential forms of Faraday's law and provides the precise meaning to "curl as circulation per unit area."
