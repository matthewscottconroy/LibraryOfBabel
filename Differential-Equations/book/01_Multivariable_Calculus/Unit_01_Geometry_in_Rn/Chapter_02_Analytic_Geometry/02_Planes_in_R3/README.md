# Planes in R3

A plane is a two-dimensional flat object living inside three-dimensional space. Every student of geometry has an intuitive sense of what a plane is, but translating that intuition into precise algebraic equations requires the dot product and the concept of a normal vector. The resulting description of planes is both elegant and enormously practical — it appears in collision detection in computer graphics, in the formulation of linear programming problems, and as the key geometric interpretation of the gradient in multivariable calculus.

## The Normal Vector and the Plane Equation

A plane in $\mathbb{R}^3$ is determined uniquely by a point and a direction perpendicular to it. Fix a point $\mathbf{p}_0 = (x_0, y_0, z_0)$ on the plane and a nonzero **normal vector** $\mathbf{n} = (a, b, c)$ perpendicular to the plane. A point $\mathbf{p} = (x, y, z)$ lies in the plane if and only if the vector $\mathbf{p} - \mathbf{p}_0$ is perpendicular to $\mathbf{n}$, i.e.,

$$\mathbf{n} \cdot (\mathbf{p} - \mathbf{p}_0) = 0.$$

Expanding: $a(x - x_0) + b(y - y_0) + c(z - z_0) = 0$, or equivalently

$$ax + by + cz = d, \quad \text{where } d = ax_0 + by_0 + cz_0 = \mathbf{n}\cdot\mathbf{p}_0.$$

This is the **standard form** of the equation of a plane. Conversely, any equation of the form $ax + by + cz = d$ with $(a,b,c) \neq (0,0,0)$ describes a plane with normal vector $(a,b,c)$.

## Worked Example: Plane Through Three Points

Find the equation of the plane through $A = (1, 0, 0)$, $B = (0, 2, 0)$, $C = (0, 0, 3)$.

Compute two vectors in the plane: $\mathbf{u} = B - A = (-1, 2, 0)$ and $\mathbf{v} = C - A = (-1, 0, 3)$.

The normal vector is $\mathbf{n} = \mathbf{u}\times\mathbf{v}$:

$$\mathbf{n} = \begin{vmatrix}\mathbf{i} & \mathbf{j} & \mathbf{k} \\ -1 & 2 & 0 \\ -1 & 0 & 3\end{vmatrix} = (6, 3, 2).$$

Using point $A$: $6(x-1) + 3(y-0) + 2(z-0) = 0$, so $6x + 3y + 2z = 6$.

Check: $A$: $6+0+0=6$; $B$: $0+6+0=6$; $C$: $0+0+6=6$. All three points satisfy the equation.

## Angle Between Two Planes

Two planes with normal vectors $\mathbf{n}_1$ and $\mathbf{n}_2$ make a **dihedral angle** $\theta$ satisfying

$$\cos\theta = \frac{|\mathbf{n}_1\cdot\mathbf{n}_2|}{\|\mathbf{n}_1\|\|\mathbf{n}_2\|}.$$

(The absolute value accounts for the ambiguity in the choice of direction of the normal.) Two planes are **parallel** if their normals are parallel, and **perpendicular** if their normals are orthogonal.

**Example.** Find the angle between $2x - y + z = 3$ and $x + y - z = 1$.

$\mathbf{n}_1 = (2,-1,1)$, $\mathbf{n}_2 = (1,1,-1)$. $\mathbf{n}_1\cdot\mathbf{n}_2 = 2-1-1 = 0$. The planes are perpendicular.

## Distance from a Point to a Plane

The distance from a point $Q = (x_1, y_1, z_1)$ to the plane $ax + by + cz = d$ is

$$\text{dist}(Q, \text{plane}) = \frac{|ax_1 + by_1 + cz_1 - d|}{\sqrt{a^2 + b^2 + c^2}}.$$

**Derivation.** Let $P_0 = (x_0, y_0, z_0)$ be any point on the plane (so $ax_0 + by_0 + cz_0 = d$). The distance is the scalar projection of $\overrightarrow{P_0 Q} = Q - P_0$ onto the unit normal $\hat{\mathbf{n}} = \mathbf{n}/\|\mathbf{n}\|$:

$$\text{dist} = |\overrightarrow{P_0 Q}\cdot\hat{\mathbf{n}}| = \frac{|(\mathbf{n})\cdot(Q - P_0)|}{\|\mathbf{n}\|} = \frac{|a(x_1-x_0) + b(y_1-y_0) + c(z_1-z_0)|}{\|\mathbf{n}\|} = \frac{|ax_1+by_1+cz_1 - d|}{\|\mathbf{n}\|}.$$

**Example.** Distance from $(2, 3, 1)$ to $x + 2y - 2z = 5$:

$$\frac{|2 + 6 - 2 - 5|}{\sqrt{1+4+4}} = \frac{|1|}{3} = \frac{1}{3}.$$

## Intersection of Two Planes

Two distinct planes in $\mathbb{R}^3$ either are parallel (and do not intersect) or intersect in a line. To find the line of intersection of $\mathbf{n}_1\cdot\mathbf{p} = d_1$ and $\mathbf{n}_2\cdot\mathbf{p} = d_2$, observe that the direction of the intersection line is perpendicular to both normals, so it is $\mathbf{d} = \mathbf{n}_1\times\mathbf{n}_2$. One then finds a single point satisfying both equations (e.g., set $z = 0$ and solve for $x$ and $y$) to get a base point.

**Example.** Find the line of intersection of $x + y + z = 1$ and $x - y + z = 0$.

Direction: $(1,1,1)\times(1,-1,1) = (1\cdot1-1\cdot(-1),\, 1\cdot1-1\cdot1,\, 1\cdot(-1)-1\cdot1) = (2, 0, -2)$, or simplified, $(1, 0, -1)$.

Set $z = 0$: $x + y = 1$ and $x - y = 0$, so $x = y = 1/2$. Base point: $(1/2, 1/2, 0)$.

Line: $\mathbf{r}(t) = (1/2, 1/2, 0) + t(1, 0, -1)$.

## Connection to the Gradient

The equation of a plane $ax + by + cz = d$ can be written as $\nabla f \cdot (\mathbf{p} - \mathbf{p}_0) = 0$ where $f(x,y,z) = ax + by + cz$ and $\nabla f = (a,b,c) = \mathbf{n}$. This is the prototype for the general fact, proved in Unit 2, that the gradient of any smooth function $f$ at a point $\mathbf{p}_0$ is perpendicular to the level surface $f(\mathbf{p}) = f(\mathbf{p}_0)$, and the tangent plane to that level surface is $\nabla f(\mathbf{p}_0)\cdot(\mathbf{p} - \mathbf{p}_0) = 0$.

## Common Pitfalls

The normal vector to a plane $ax + by + cz = d$ is $(a, b, c)$, regardless of $d$. The value of $d$ shifts the plane parallel to itself but does not change the normal. Students sometimes treat $d$ as a component of the normal, which is incorrect.

When finding the plane through three points, it is essential to form two linearly independent vectors in the plane before taking their cross product. If the three points happen to be collinear, there is no unique plane through them (in fact, infinitely many planes contain any given line), and the cross product will yield the zero vector as a warning.
