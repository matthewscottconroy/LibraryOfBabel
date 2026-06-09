# Statement and Proof of Green's Theorem

Green's Theorem is a precise formula that converts a line integral around a closed curve into a double integral over the region it encloses, or vice versa. Its proof is not difficult — it is essentially the Fundamental Theorem of Calculus applied twice, once in each coordinate direction — but the geometric setup requires care.

## Full Statement

**Green's Theorem.** Let $D$ be a bounded region in $\mathbb{R}^2$ whose boundary $\partial D$ consists of one or more piecewise smooth simple closed curves. Let $P, Q: \overline{D} \to \mathbb{R}$ be $C^1$ functions on an open set containing $\overline{D}$. Then

$$\oint_{\partial D} P\,dx + Q\,dy = \iint_D\left(\frac{\partial Q}{\partial x} - \frac{\partial P}{\partial y}\right)dA,$$

where $\partial D$ is traversed so that $D$ is on the left (counterclockwise for the outer boundary, clockwise for any inner holes).

If $D$ has holes (is not simply connected), the boundary $\partial D$ consists of the outer boundary (counterclockwise) plus the inner boundaries (clockwise). The same formula holds.

## Proof for a Rectangle

First, prove the theorem for a closed rectangle $D = [a,b]\times[c,d]$.

**Step 1: Prove $\oint_{\partial D} P\,dx = -\iint_D \frac{\partial P}{\partial y}\,dA$.**

The left side: $\partial D$ consists of four sides. On the top ($y = d$, left to right) and bottom ($y = c$, right to left), $x$ varies; on the left ($x = a$) and right ($x = b$) sides, $dx = 0$, so these contribute nothing.

$$\oint_{\partial D} P\,dx = \int_a^b P(x,c)\,dx - \int_a^b P(x,d)\,dx = -\int_a^b [P(x,d) - P(x,c)]\,dx.$$

By the FTC in $y$: $P(x,d) - P(x,c) = \int_c^d \frac{\partial P}{\partial y}(x,y)\,dy$.

Substituting: $\oint_{\partial D} P\,dx = -\int_a^b\int_c^d\frac{\partial P}{\partial y}\,dy\,dx = -\iint_D\frac{\partial P}{\partial y}\,dA$.

**Step 2: Prove $\oint_{\partial D} Q\,dy = \iint_D\frac{\partial Q}{\partial x}\,dA$.**

On $\partial D$, $dy \neq 0$ only on the left and right sides. The right side ($x = b$, $y$ increasing from $c$ to $d$) and the left side ($x = a$, $y$ decreasing from $d$ to $c$):

$$\oint_{\partial D} Q\,dy = \int_c^d Q(b,y)\,dy - \int_c^d Q(a,y)\,dy = \int_c^d [Q(b,y) - Q(a,y)]\,dy.$$

By the FTC in $x$: $Q(b,y) - Q(a,y) = \int_a^b\frac{\partial Q}{\partial x}\,dx$.

$$\oint_{\partial D} Q\,dy = \int_c^d\int_a^b\frac{\partial Q}{\partial x}\,dx\,dy = \iint_D\frac{\partial Q}{\partial x}\,dA.$$

**Step 3: Add the two results:**

$$\oint_{\partial D} P\,dx + Q\,dy = \iint_D\left(\frac{\partial Q}{\partial x} - \frac{\partial P}{\partial y}\right)dA.$$

This completes the proof for a rectangle.

## Proof for Simple Regions

A **Type I region** (vertically simple) is of the form $D = \{(x,y): a \leq x \leq b,\, g_1(x) \leq y \leq g_2(x)\}$. The same two-step proof works: integrate $\partial P/\partial y$ in $y$ using the FTC, and integrate $\partial Q/\partial x$ in $x$ using the FTC. The boundary terms exactly match the four arcs of the boundary $\partial D$.

A **Type II region** (horizontally simple) is handled symmetrically.

For general regions: subdivide into finitely many Type I or Type II regions. Apply the theorem to each piece. The line integrals over interior interfaces cancel in pairs (each interface is traversed in opposite directions for its two adjacent pieces). What remains is the integral over the exterior boundary $\partial D$.

## Worked Examples

**Example 1.** Let $\mathbf{F} = xy\,\mathbf{i} + (x^2-y^2)\,\mathbf{j}$ and $D$ the triangular region with vertices $(0,0)$, $(2,0)$, $(1,2)$. Compute $\oint_{\partial D}\mathbf{F}\cdot d\mathbf{r}$ using Green's Theorem.

$\frac{\partial Q}{\partial x} - \frac{\partial P}{\partial y} = 2x - x = x$.

$$\oint_{\partial D}\mathbf{F}\cdot d\mathbf{r} = \iint_D x\,dA.$$

The triangle has vertices at $(0,0)$, $(2,0)$, $(1,2)$. Set up the integral by integrating $x$ over the triangle. One approach: $x$ ranges from 0 to 2; for fixed $x$, $y$ ranges from 0 to the appropriate boundary. Alternatively, use the centroid: $\bar{x} = (0+2+1)/3 = 1$, Area $= \frac{1}{2}|2\cdot 2 - 0| = 2$. So $\iint_D x\,dA = \bar{x}\cdot A = 1\cdot 2 = 2$.

**Example 2.** Verify Green's Theorem for $P = -y$, $Q = x$, $D$ the unit disk.

$\frac{\partial Q}{\partial x} - \frac{\partial P}{\partial y} = 1 - (-1) = 2$. $\iint_D 2\,dA = 2\pi$.

Direct computation: $\oint_{\partial D} -y\,dx + x\,dy$ on the unit circle with $\mathbf{r}(t) = (\cos t, \sin t)$:

$= \int_0^{2\pi}(-\sin t)(-\sin t) + (\cos t)(\cos t)\,dt = \int_0^{2\pi} 1\,dt = 2\pi$. Verified.

## Extensions: Multiply Connected Regions

If $D$ has an inner hole bounded by a curve $C_{\text{in}}$, then $\partial D = C_{\text{out}} \cup (-C_{\text{in}})$ (outer counterclockwise plus inner clockwise). Green's Theorem still holds:

$$\oint_{C_{\text{out}}} + \oint_{-C_{\text{in}}} = \iint_D\left(\frac{\partial Q}{\partial x} - \frac{\partial P}{\partial y}\right)dA.$$

This can be used to relate integrals over different loops in the same domain: if $\partial Q/\partial x = \partial P/\partial y$ throughout $D$, then the integral around the outer curve equals the integral around the inner curve (in the same orientation).

## Summary

Green's Theorem converts between a circulation integral around a closed curve and a double integral of the curl ($\partial Q/\partial x - \partial P/\partial y$) over the enclosed region. The proof reduces to two applications of the one-variable FTC. The theorem holds for any region whose boundary is piecewise smooth and for which the component functions are $C^1$. It generalizes Stokes' Theorem to surfaces and the Divergence Theorem to volumes, and it is the foundation for computing area via boundary integrals.
