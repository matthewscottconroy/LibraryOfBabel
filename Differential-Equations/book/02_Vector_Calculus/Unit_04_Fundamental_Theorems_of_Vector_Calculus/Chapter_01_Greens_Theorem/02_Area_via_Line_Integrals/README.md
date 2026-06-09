# Area via Line Integrals

One of the most elegant consequences of Green's Theorem is that the area of a region can be computed as a line integral around its boundary — no interior information required. This converts a two-dimensional measurement (area) into a one-dimensional computation (a line integral), inverting the usual direction of integration.

## Derivation

Green's Theorem states $\oint_{\partial D} P\,dx + Q\,dy = \iint_D (\partial Q/\partial x - \partial P/\partial y)\,dA$. To compute area, we want the right side to equal $\iint_D 1\,dA$. This requires $\partial Q/\partial x - \partial P/\partial y = 1$. Three natural choices:

- $Q = x$, $P = 0$: $\partial Q/\partial x - \partial P/\partial y = 1 - 0 = 1$. Gives $A = \oint_{\partial D} x\,dy$.
- $Q = 0$, $P = -y$: $\partial Q/\partial x - \partial P/\partial y = 0 - (-1) = 1$. Gives $A = -\oint_{\partial D} y\,dx$.
- $Q = x/2$, $P = -y/2$: Gives the symmetric formula $A = \frac{1}{2}\oint_{\partial D} (x\,dy - y\,dx)$.

## The Shoelace Formula

For a polygon with vertices $(x_1,y_1), (x_2,y_2), \ldots, (x_n,y_n)$ listed counterclockwise, applying $A = \frac{1}{2}\oint_{\partial D}(x\,dy - y\,dx)$ to each straight edge and summing:

$$A = \frac{1}{2}\left|\sum_{i=1}^n (x_i y_{i+1} - x_{i+1} y_i)\right|,$$

where indices are taken modulo $n$. This is the **shoelace formula**, widely used in computational geometry. It computes the area of any polygon from its vertices without subdividing into triangles.

**Example.** For the triangle $(0,0)$, $(2,0)$, $(1,3)$:

$A = \frac{1}{2}|(0\cdot 0 - 2\cdot 0) + (2\cdot 3 - 1\cdot 0) + (1\cdot 0 - 0\cdot 3)| = \frac{1}{2}|0 + 6 + 0| = 3$.

Verify: $A = \frac{1}{2}|base\times height| = \frac{1}{2}\cdot 2\cdot 3 = 3$. Correct.

## Area of an Ellipse

For the ellipse $x = a\cos t$, $y = b\sin t$, $t \in [0,2\pi]$:

$$A = \frac{1}{2}\oint (x\,dy - y\,dx) = \frac{1}{2}\int_0^{2\pi}(a\cos t\cdot b\cos t - b\sin t\cdot(-a\sin t))\,dt = \frac{ab}{2}\int_0^{2\pi}1\,dt = \pi ab.$$

This recovers the classical formula for ellipse area.

## Practical Use

This formula is especially useful when the boundary of $D$ is parametrically described. If the domain $D$ has a complicated shape but a simple boundary parametrization, the area formula converts the area computation into a one-dimensional integral — often much simpler than the double integral $\iint_D 1\,dA$.

## Summary

By choosing $P$ and $Q$ so that $\partial Q/\partial x - \partial P/\partial y = 1$, Green's Theorem gives $A(D) = \oint_{\partial D} x\,dy = -\oint_{\partial D} y\,dx = \frac{1}{2}\oint_{\partial D}(x\,dy - y\,dx)$. Applied to polygons, this yields the shoelace formula. The principle — computing an interior quantity from boundary data — is the essence of all the fundamental theorems.
