# Quadric Surfaces

Just as the conic sections — ellipses, hyperbolas, and parabolas — are the curves described by second-degree polynomial equations in two variables, the **quadric surfaces** are the surfaces described by second-degree polynomial equations in three variables. They are the simplest nonlinear surfaces in three-dimensional geometry, and they appear ubiquitously throughout mathematics and physics: the gravitational potential well near a stable equilibrium is a paraboloid, wave fronts in anisotropic media are ellipsoids, and the level surfaces of quadratic forms are quadrics. Understanding their shapes is essential for visualizing the mathematics that follows in this course.

## General Form and Classification

The general second-degree equation in $x$, $y$, $z$ is

$$Ax^2 + By^2 + Cz^2 + Dxy + Exz + Fyz + Gx + Hy + Iz + J = 0.$$

By a rotation of coordinates, the cross terms ($xy$, $xz$, $yz$) can be eliminated; by a translation, the linear terms can often be eliminated too. The resulting equation takes one of a small number of standard forms. The six non-degenerate types are described below.

## The Six Standard Quadric Surfaces

**Ellipsoid:** $\frac{x^2}{a^2} + \frac{y^2}{b^2} + \frac{z^2}{c^2} = 1$. This is a surface symmetric about all three coordinate planes. Its traces in the coordinate planes are ellipses. When $a = b = c$, it degenerates to a sphere of radius $a$. Every point on the ellipsoid satisfies $-a \leq x \leq a$, $-b \leq y \leq b$, $-c \leq z \leq c$, so it is a bounded surface.

**Hyperboloid of one sheet:** $\frac{x^2}{a^2} + \frac{y^2}{b^2} - \frac{z^2}{c^2} = 1$. This surface is connected — one can travel from any point to any other without leaving the surface. Its traces in horizontal planes ($z = k$) are ellipses that grow as $|k|$ increases. Its traces in the $xz$- and $yz$-planes are hyperbolas. The surface has a "waist" (minimum circular cross-section) at $z = 0$.

**Hyperboloid of two sheets:** $-\frac{x^2}{a^2} - \frac{y^2}{b^2} + \frac{z^2}{c^2} = 1$. This surface is disconnected, consisting of two separate sheets: one where $z \geq c$ and one where $z \leq -c$. Horizontal traces (for $|z| > c$) are ellipses; vertical traces are hyperbolas.

**Elliptic paraboloid:** $z = \frac{x^2}{a^2} + \frac{y^2}{b^2}$. This surface opens upward like a bowl. All horizontal traces are ellipses; vertical traces are parabolas. It has a single minimum at the origin. This surface is particularly important in optimization: any function of two variables with a local minimum looks like an elliptic paraboloid near the minimum point.

**Hyperbolic paraboloid:** $z = \frac{x^2}{a^2} - \frac{y^2}{b^2}$. This surface has the shape of a saddle. In the $x$-direction, $z$ increases; in the $y$-direction, $z$ decreases. Horizontal traces are hyperbolas; vertical traces are parabolas. The origin is a **saddle point** — a critical point that is neither a maximum nor a minimum. The second derivative test for functions of two variables is designed precisely to distinguish between local minima (elliptic paraboloid shape) and saddle points (hyperbolic paraboloid shape).

**Elliptic cone:** $\frac{x^2}{a^2} + \frac{y^2}{b^2} = \frac{z^2}{c^2}$. The cone consists of two nappes meeting at the origin. Horizontal traces are ellipses (except at $z = 0$ where the trace is a point); vertical traces are hyperbolas or lines. The cone is a degenerate quadric in the sense that it is the limiting surface between the hyperboloid of one sheet and the hyperboloid of two sheets.

## Recognizing Quadrics from Their Equations

The key technique for identifying a quadric surface is to examine its **traces** — the intersections of the surface with the coordinate planes $x = 0$, $y = 0$, $z = 0$ — and with horizontal planes $z = k$ for various values of $k$.

For example, consider $x^2 + \frac{y^2}{4} - z^2 = 1$.
- Set $z = 0$: $x^2 + y^2/4 = 1$, an ellipse.
- Set $x = 0$: $y^2/4 - z^2 = 1$, a hyperbola opening in the $y$-direction.
- Set $y = 0$: $x^2 - z^2 = 1$, a hyperbola opening in the $x$-direction.

This pattern — elliptic cross-sections horizontally and hyperbolic cross-sections vertically — identifies a hyperboloid of one sheet.

## Completing the Square

When a quadric surface is given in non-standard form, completing the square converts it to standard form. For instance,

$$x^2 + 2y^2 - 4y + z^2 = 3$$

can be rewritten by completing the square in $y$: $2(y-1)^2 - 2$, giving

$$x^2 + 2(y-1)^2 + z^2 = 5, \quad \text{i.e.,} \quad \frac{x^2}{5} + \frac{(y-1)^2}{5/2} + \frac{z^2}{5} = 1.$$

This is an ellipsoid centered at $(0, 1, 0)$.

## Worked Example

Identify and sketch the surface $4x^2 - y^2 + z^2 = 0$.

Rewrite as $y^2 = 4x^2 + z^2$. Every horizontal trace ($y = k$) gives $4x^2 + z^2 = k^2$, an ellipse (for $k \neq 0$) or the origin (for $k = 0$). The $xy$-trace ($z = 0$) gives $y^2 = 4x^2$, i.e., $y = \pm 2x$, two lines. This is an **elliptic cone** with axis along the $y$-axis.

## Connection to Later Material

Quadric surfaces appear as level surfaces of functions $f(x,y,z) = ax^2 + by^2 + cz^2$. The gradient of $f$ is $\nabla f = (2ax, 2by, 2cz)$, which is the normal to the level surface at each point. The second-order behavior of any smooth function near a critical point is governed by its quadratic approximation — a quadratic form whose level sets are quadrics. The classification of critical points (elliptic paraboloid for minima, hyperbolic paraboloid for saddles) is therefore a direct application of the geometry studied here.
