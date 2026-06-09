# Lines in Rn

A line in the plane is a one-dimensional geometric object through which exactly one direction of travel is possible. In $\mathbb{R}^2$, students learn to describe lines by their slope and intercept: $y = mx + b$. This works well in the plane, but in $\mathbb{R}^3$ a single equation of the form $ax + by + cz = d$ describes a plane, not a line. To describe a line in three dimensions — or in any number of dimensions — one needs a different approach, and the parametric description using vectors is the right one.

## Parametric Equations of a Line

The idea is simple: a line is determined by a point and a direction. Fix a point $\mathbf{p}_0 \in \mathbb{R}^n$ on the line and a nonzero direction vector $\mathbf{d} \in \mathbb{R}^n$. Any other point on the line is obtained by starting at $\mathbf{p}_0$ and moving some distance in the direction $\mathbf{d}$. As the parameter $t$ ranges over all real numbers, the expression

$$\mathbf{r}(t) = \mathbf{p}_0 + t\mathbf{d}$$

traces out the entire line. This is the **parametric vector equation** of the line.

In $\mathbb{R}^3$ with $\mathbf{p}_0 = (x_0, y_0, z_0)$ and $\mathbf{d} = (a, b, c)$, this gives the **parametric scalar equations**:

$$x = x_0 + at, \quad y = y_0 + bt, \quad z = z_0 + ct.$$

If all of $a$, $b$, $c$ are nonzero, eliminating $t$ from these three equations gives the **symmetric equations**:

$$\frac{x - x_0}{a} = \frac{y - y_0}{b} = \frac{z - z_0}{c}.$$

When one component, say $c = 0$, the line lies in a plane parallel to the $xy$-plane, and one writes $z = z_0$ separately alongside $\frac{x-x_0}{a} = \frac{y-y_0}{b}$.

## Non-Uniqueness of Parametrization

A given line has infinitely many valid parametrizations: any point on the line can serve as $\mathbf{p}_0$, and any nonzero scalar multiple of $\mathbf{d}$ gives the same line traversed at a different speed. Two parametrizations $\mathbf{p}_0 + t\mathbf{d}$ and $\mathbf{q}_0 + t\mathbf{e}$ describe the same line if and only if $\mathbf{d}$ and $\mathbf{e}$ are parallel (one is a nonzero scalar multiple of the other) and $\mathbf{q}_0 - \mathbf{p}_0$ is parallel to $\mathbf{d}$.

## Worked Example: Line Through Two Points

Find the parametric equations of the line through $P = (1, 2, -1)$ and $Q = (3, -1, 4)$.

The direction vector is $\mathbf{d} = Q - P = (2, -3, 5)$. Using $P$ as the base point:

$$x = 1 + 2t, \quad y = 2 - 3t, \quad z = -1 + 5t.$$

Check: at $t = 0$, the point is $P = (1,2,-1)$; at $t = 1$, the point is $(3,-1,4) = Q$. The symmetric equations are $\frac{x-1}{2} = \frac{y-2}{-3} = \frac{z+1}{5}$.

## Intersection of Two Lines

Two lines in $\mathbb{R}^3$ may intersect in a single point, be parallel (same direction, no common point), or be **skew** (neither parallel nor intersecting). This last possibility — skew lines — does not exist in $\mathbb{R}^2$, where any two non-parallel lines must meet.

To test whether lines $\mathbf{r}_1(t) = \mathbf{p}_1 + t\mathbf{d}_1$ and $\mathbf{r}_2(s) = \mathbf{p}_2 + s\mathbf{d}_2$ intersect, set them equal and solve the system $\mathbf{p}_1 + t\mathbf{d}_1 = \mathbf{p}_2 + s\mathbf{d}_2$ for $t$ and $s$. This is a system of $n$ equations in 2 unknowns; in $\mathbb{R}^3$ (3 equations, 2 unknowns) it is generically overdetermined, so most pairs of non-parallel lines in $\mathbb{R}^3$ are skew.

**Example.** Check whether $\mathbf{r}_1(t) = (0,0,0) + t(1,1,0)$ and $\mathbf{r}_2(s) = (0,1,1) + s(1,0,1)$ intersect.

Setting equal: $t = s$, $t = 1$, $0 = 1 + s$. From the second equation $t = 1$; from the third $s = -1$; but then the first equation gives $1 = -1$, a contradiction. The lines are skew.

## Distance from a Point to a Line

The distance from a point $Q$ to the line through $P$ with direction $\mathbf{d}$ is computed using the projection of $\overrightarrow{PQ}$ perpendicular to $\mathbf{d}$.

Let $\mathbf{u} = Q - P$. Then $\mathbf{u}_\perp = \mathbf{u} - \text{proj}_\mathbf{d}\mathbf{u}$, and the distance is $d = \|\mathbf{u}_\perp\|$.

In $\mathbb{R}^3$, an equivalent formula is:

$$d = \frac{\|\mathbf{u} \times \mathbf{d}\|}{\|\mathbf{d}\|},$$

since $\|\mathbf{u}\times\mathbf{d}\| = \|\mathbf{u}\|\|\mathbf{d}\|\sin\theta$ and $\|\mathbf{u}\|\sin\theta = \|\mathbf{u}_\perp\|$ (the component of $\mathbf{u}$ perpendicular to $\mathbf{d}$).

**Example.** Find the distance from $Q = (1, 2, 3)$ to the line through $P = (0, 0, 0)$ with direction $\mathbf{d} = (1, 0, 0)$ (the $x$-axis).

$\mathbf{u} = (1, 2, 3)$. $\mathbf{u} \times \mathbf{d} = (1,2,3)\times(1,0,0) = (2\cdot0-3\cdot0, 3\cdot1-1\cdot0, 1\cdot0-2\cdot1) = (0, 3, -2)$. Distance $= \|(0,3,-2)\|/\|(1,0,0)\| = \sqrt{9+4}/1 = \sqrt{13}$. This makes sense geometrically: the distance from $(1,2,3)$ to the $x$-axis is $\sqrt{y^2+z^2} = \sqrt{4+9} = \sqrt{13}$.

## Distance Between Two Skew Lines

The minimum distance between skew lines $\mathbf{p}_1 + t\mathbf{d}_1$ and $\mathbf{p}_2 + s\mathbf{d}_2$ is achieved along the common perpendicular. The direction of this perpendicular is $\mathbf{n} = \mathbf{d}_1 \times \mathbf{d}_2$. The distance is

$$d = \frac{|(\mathbf{p}_2 - \mathbf{p}_1)\cdot\mathbf{n}|}{\|\mathbf{n}\|},$$

the scalar projection of $\mathbf{p}_2 - \mathbf{p}_1$ onto the unit vector in the direction of $\mathbf{n}$.

## Lines in Higher Dimensions

In $\mathbb{R}^n$ for $n > 3$, the parametric form $\mathbf{r}(t) = \mathbf{p}_0 + t\mathbf{d}$ works identically. The symmetric form does not generalize as cleanly (since one needs $n-1$ equations to specify a line in $\mathbb{R}^n$), but the parametric approach remains the standard. This is one reason why vector methods are preferred over coordinate-based methods in higher-dimensional geometry.

## Common Pitfalls

Students sometimes confuse the direction vector with the normal vector to a plane. For a line, the direction vector $\mathbf{d}$ points along the line; for a plane, the normal vector points perpendicular to the plane. These roles are reversed, and mixing them up leads to incorrect equations.

When checking whether two lines intersect, one must use separate parameters ($t$ for one line and $s$ for another), not the same parameter. Using the same parameter amounts to asking whether the lines are the same line, not whether they meet.
