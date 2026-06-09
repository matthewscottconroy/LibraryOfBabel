# Gradient as Normal Vector to Level Sets

One of the most important geometric facts in multivariable calculus is that the gradient of a function is perpendicular to its level sets. This single observation underlies the equation of the tangent plane to a surface, the Lagrange multiplier method for constrained optimization, and the geometric interpretation of conservative vector fields. Understanding why the gradient is normal to level sets requires combining the chain rule, the directional derivative formula, and a clear picture of what "level set" and "normal" mean.

## Level Sets

The **level set** of $f:\mathbb{R}^n\to\mathbb{R}$ at value $c$ is

$$L_c = \{f = c\} = \{\mathbf{x}\in\mathbb{R}^n : f(\mathbf{x}) = c\}.$$

In $\mathbb{R}^2$: level sets are curves, called **level curves** or **contour lines**. In $\mathbb{R}^3$: level sets are surfaces, called **level surfaces** or **isosurfaces**.

For example, $f(x,y) = x^2+y^2$: level curves are circles centered at the origin. For $f(x,y,z) = x^2+y^2+z^2$: level surfaces are spheres. For $f(x,y,z) = x+2y-3z$: level surfaces are planes.

## The Gradient is Perpendicular to Level Sets

**Theorem.** Let $f:\mathbb{R}^n\to\mathbb{R}$ be differentiable at $\mathbf{a}$, and let $c = f(\mathbf{a})$. Suppose $\boldsymbol{\gamma}:(-\epsilon,\epsilon)\to\mathbb{R}^n$ is a smooth curve with $\boldsymbol{\gamma}(0) = \mathbf{a}$ and $\boldsymbol{\gamma}(t)\in L_c$ for all $t$ (i.e., $f(\boldsymbol{\gamma}(t)) = c$ for all $t$). Then

$$\nabla f(\mathbf{a})\cdot\boldsymbol{\gamma}'(0) = 0.$$

That is, the gradient is perpendicular to every tangent vector of every curve lying in the level set.

**Proof.** Since $f(\boldsymbol{\gamma}(t)) = c$ is constant, differentiating both sides with respect to $t$ at $t=0$ using the chain rule:

$$\frac{d}{dt}f(\boldsymbol{\gamma}(t))\bigg|_{t=0} = \nabla f(\boldsymbol{\gamma}(0))\cdot\boldsymbol{\gamma}'(0) = \nabla f(\mathbf{a})\cdot\boldsymbol{\gamma}'(0) = \frac{dc}{dt} = 0.$$

The tangent vector $\boldsymbol{\gamma}'(0)$ to the level set at $\mathbf{a}$ is arbitrary (any tangent direction to the level set); therefore $\nabla f(\mathbf{a})$ is perpendicular to all tangent directions, i.e., normal to the level set.

## Tangent Plane to a Level Surface

For $f:\mathbb{R}^3\to\mathbb{R}$ with level surface $\{f = c\}$ passing through $\mathbf{a}$, the **tangent plane** at $\mathbf{a}$ has normal vector $\nabla f(\mathbf{a})$ and equation:

$$\nabla f(\mathbf{a})\cdot(\mathbf{x}-\mathbf{a}) = 0,$$

i.e., $f_x(a,b,c_0)(x-a) + f_y(a,b,c_0)(y-b) + f_z(a,b,c_0)(z-c_0) = 0$ where $(a,b,c_0) = \mathbf{a}$.

**Example.** Find the tangent plane to $x^2+y^2+z^2 = 3$ at $(1,1,1)$.

$f(x,y,z) = x^2+y^2+z^2$, $\nabla f = (2x,2y,2z)$. At $(1,1,1)$: $\nabla f = (2,2,2)$, or simplified $(1,1,1)$.

Tangent plane: $(x-1)+(y-1)+(z-1)=0$, i.e., $x+y+z=3$.

**Example.** Find the tangent plane to $z = \sqrt{x^2+y^2}$ (a cone) at $(3,4,5)$.

Write $F(x,y,z) = \sqrt{x^2+y^2} - z = 0$. Then $F_x = x/\sqrt{x^2+y^2}$, $F_y = y/\sqrt{x^2+y^2}$, $F_z = -1$. At $(3,4,5)$: $\nabla F = (3/5, 4/5, -1)$.

Tangent plane: $(3/5)(x-3) + (4/5)(y-4) - (z-5) = 0$, i.e., $3x+4y-5z = 0$.

## Tangent Line to a Level Curve

For $f(x,y) = c$ in $\mathbb{R}^2$, the tangent line to the level curve at $(a,b)$ has normal $(f_x(a,b), f_y(a,b))$, so its equation is:

$$f_x(a,b)(x-a) + f_y(a,b)(y-b) = 0.$$

The tangent vector to the level curve at $(a,b)$ is any vector perpendicular to $\nabla f(a,b)$: namely $(-f_y(a,b), f_x(a,b))$ or its negative.

## Gradient Direction vs. Level Set Direction

The gradient and the level set are geometrically complementary: the gradient is perpendicular to the level set, and the level set is locally perpendicular to the gradient. This means:
- Moving along the gradient increases $f$ (you move "uphill").
- Moving along the level set keeps $f$ constant (you stay "on the same contour").
- These two motions are perpendicular.

In thermodynamics: the temperature gradient points in the direction of greatest temperature increase, perpendicular to the isothermal surfaces. In potential theory: the gravitational force (gradient of potential energy) is perpendicular to the equipotential surfaces.

## The Case of Critical Points

If $\nabla f(\mathbf{a}) = \mathbf{0}$, then $\mathbf{a}$ is a **critical point** of $f$. At a critical point, the gradient formula gives $D_{\hat{\mathbf{u}}}f(\mathbf{a}) = \mathbf{0}\cdot\hat{\mathbf{u}} = 0$ for every $\hat{\mathbf{u}}$: the rate of change is zero in all directions. The level set through a critical point can fail to be a smooth surface (e.g., a cone's apex, a saddle's central point), and the normal vector interpretation breaks down. The behavior near critical points is the subject of Chapter 6.

## Common Pitfalls

The gradient is a vector in $\mathbb{R}^n$ (the same space as the domain). It is not a vector in $\mathbb{R}^{n+1}$ (the graph space). Students sometimes confuse the normal to the level surface $\{f=c\}$ in $\mathbb{R}^3$ (which is $\nabla f$, a vector in $\mathbb{R}^3$) with the normal to the graph $\{(x,y,f(x,y))\}\subset\mathbb{R}^3$ (which is $(-f_x, -f_y, 1)$, also in $\mathbb{R}^3$ but different). The two normals are related but not equal.
