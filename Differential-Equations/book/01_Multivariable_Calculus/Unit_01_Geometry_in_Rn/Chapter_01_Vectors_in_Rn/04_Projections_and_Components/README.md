# Projections and Components

Given two vectors $\mathbf{u}$ and $\mathbf{v}$, a natural question is: how much of $\mathbf{u}$ lies along the direction of $\mathbf{v}$? This is not asking for the dot product, which mixes information about both magnitudes and angle, but for a pure measurement of overlap in a given direction. The projection operation provides the answer, and it turns out to be one of the most versatile tools in the subject — used to compute distances from points to lines and planes, decompose forces into components, construct orthonormal bases, and understand the geometry of the gradient.

## The Scalar and Vector Projections

Let $\mathbf{v} \neq \mathbf{0}$ be a fixed nonzero vector in $\mathbb{R}^n$, thought of as defining a direction. The **scalar projection** (or **component**) of $\mathbf{u}$ onto $\mathbf{v}$ is the signed length of the shadow of $\mathbf{u}$ in the direction $\hat{\mathbf{v}} = \mathbf{v}/\|\mathbf{v}\|$:

$$\text{comp}_{\mathbf{v}} \mathbf{u} = \mathbf{u} \cdot \hat{\mathbf{v}} = \frac{\mathbf{u} \cdot \mathbf{v}}{\|\mathbf{v}\|}.$$

This is positive when the angle between $\mathbf{u}$ and $\mathbf{v}$ is acute, negative when it is obtuse, and zero when they are orthogonal. It is a scalar — a single real number.

The **vector projection** of $\mathbf{u}$ onto $\mathbf{v}$ is the vector in the direction of $\mathbf{v}$ whose length is $|\text{comp}_{\mathbf{v}}\mathbf{u}|$ (with appropriate sign):

$$\text{proj}_{\mathbf{v}} \mathbf{u} = \left(\frac{\mathbf{u} \cdot \mathbf{v}}{\|\mathbf{v}\|^2}\right)\mathbf{v} = \frac{\mathbf{u} \cdot \mathbf{v}}{\mathbf{v} \cdot \mathbf{v}}\,\mathbf{v}.$$

Note that the coefficient $\frac{\mathbf{u}\cdot\mathbf{v}}{\mathbf{v}\cdot\mathbf{v}}$ is a scalar, and multiplying it by $\mathbf{v}$ gives a vector in the same direction as $\mathbf{v}$ (or opposite, if the scalar is negative).

## Orthogonal Decomposition

The projection immediately yields an orthogonal decomposition of $\mathbf{u}$. Define

$$\mathbf{u}_\parallel = \text{proj}_{\mathbf{v}} \mathbf{u}, \qquad \mathbf{u}_\perp = \mathbf{u} - \text{proj}_{\mathbf{v}} \mathbf{u}.$$

Then $\mathbf{u} = \mathbf{u}_\parallel + \mathbf{u}_\perp$, and $\mathbf{u}_\perp \cdot \mathbf{v} = 0$.

**Proof of perpendicularity:** $\mathbf{u}_\perp \cdot \mathbf{v} = \left(\mathbf{u} - \frac{\mathbf{u}\cdot\mathbf{v}}{\|\mathbf{v}\|^2}\mathbf{v}\right)\cdot\mathbf{v} = \mathbf{u}\cdot\mathbf{v} - \frac{\mathbf{u}\cdot\mathbf{v}}{\|\mathbf{v}\|^2}(\mathbf{v}\cdot\mathbf{v}) = \mathbf{u}\cdot\mathbf{v} - \mathbf{u}\cdot\mathbf{v} = 0.$

This decomposition is unique: there is exactly one way to write $\mathbf{u}$ as a sum of a vector parallel to $\mathbf{v}$ and a vector perpendicular to $\mathbf{v}$.

## Worked Example

Let $\mathbf{u} = (3, 4, 0)$ and $\mathbf{v} = (1, 1, 0)$.

The scalar projection: $\text{comp}_{\mathbf{v}}\mathbf{u} = \frac{(3)(1)+(4)(1)+(0)(0)}{\sqrt{1^2+1^2+0^2}} = \frac{7}{\sqrt{2}}$.

The vector projection: $\text{proj}_{\mathbf{v}}\mathbf{u} = \frac{7}{2}(1,1,0) = (7/2, 7/2, 0)$.

The perpendicular component: $\mathbf{u}_\perp = (3,4,0) - (7/2, 7/2, 0) = (-1/2, 1/2, 0)$.

Check: $(-1/2, 1/2, 0)\cdot(1,1,0) = -1/2 + 1/2 = 0$. Confirmed orthogonal.

## Distance from a Point to a Line

Projection gives a clean formula for the distance from a point to a line. Let the line pass through $\mathbf{p}$ with direction $\mathbf{v}$, and let $\mathbf{q}$ be the external point. The vector $\mathbf{u} = \mathbf{q} - \mathbf{p}$ connects a point on the line to $\mathbf{q}$. The distance is the length of the component of $\mathbf{u}$ perpendicular to $\mathbf{v}$:

$$d = \|\mathbf{u}_\perp\| = \|\mathbf{u} - \text{proj}_{\mathbf{v}}\mathbf{u}\|.$$

In $\mathbb{R}^3$, an equivalent formula is $d = \|\mathbf{u}\times\hat{\mathbf{v}}\|$, using the cross product — since the cross product magnitude equals the area of the parallelogram and dividing by the base $\|\hat{\mathbf{v}}\| = 1$ gives the height.

## Projection onto a Subspace: The Gram-Schmidt Process

The projection formula generalizes to projection onto any subspace, not just a one-dimensional span. If $\{\mathbf{w}_1, \mathbf{w}_2, \ldots, \mathbf{w}_k\}$ is an **orthonormal** set (all unit vectors, mutually perpendicular), then the projection of $\mathbf{u}$ onto their span is

$$\text{proj}_W \mathbf{u} = (\mathbf{u}\cdot\mathbf{w}_1)\mathbf{w}_1 + (\mathbf{u}\cdot\mathbf{w}_2)\mathbf{w}_2 + \cdots + (\mathbf{u}\cdot\mathbf{w}_k)\mathbf{w}_k.$$

The **Gram-Schmidt process** builds an orthonormal basis from any linearly independent set $\{\mathbf{v}_1, \mathbf{v}_2, \ldots, \mathbf{v}_k\}$ by iteratively projecting and subtracting:

$$\mathbf{w}_1 = \frac{\mathbf{v}_1}{\|\mathbf{v}_1\|}, \quad \mathbf{w}_2 = \frac{\mathbf{v}_2 - (\mathbf{v}_2\cdot\mathbf{w}_1)\mathbf{w}_1}{\|\mathbf{v}_2 - (\mathbf{v}_2\cdot\mathbf{w}_1)\mathbf{w}_1\|}, \quad \ldots$$

At each step, one projects the new vector onto the span of the vectors already processed and subtracts, then normalizes. The result is a set of mutually orthogonal unit vectors spanning the same subspace.

## Connection to the Directional Derivative

The projection idea appears directly in multivariable calculus. The directional derivative of a function $f$ at $\mathbf{p}$ in the direction $\hat{\mathbf{u}}$ is $D_{\hat{\mathbf{u}}}f(\mathbf{p}) = \nabla f(\mathbf{p})\cdot\hat{\mathbf{u}}$. This is nothing but the scalar projection of the gradient $\nabla f(\mathbf{p})$ onto the direction $\hat{\mathbf{u}}$. The rate of change of $f$ in direction $\hat{\mathbf{u}}$ is determined entirely by how much of the gradient lies in that direction — which is maximized when $\hat{\mathbf{u}}$ points along the gradient itself, and is zero when $\hat{\mathbf{u}}$ is perpendicular to the gradient (i.e., along a level set).

## Common Pitfalls

Students sometimes confuse the scalar projection with the vector projection. The scalar projection $\text{comp}_{\mathbf{v}}\mathbf{u}$ is a number; the vector projection $\text{proj}_{\mathbf{v}}\mathbf{u}$ is a vector. Mixing them up leads to dimensional errors in applications.

Another error is assuming that $\text{proj}_{\mathbf{v}}\mathbf{u} = \text{proj}_{\mathbf{u}}\mathbf{v}$. These are generally different vectors pointing in different directions. The formula $\text{proj}_{\mathbf{v}}\mathbf{u} = \frac{\mathbf{u}\cdot\mathbf{v}}{\|\mathbf{v}\|^2}\mathbf{v}$ shows clearly that the result always lies along $\mathbf{v}$, not along $\mathbf{u}$.

Finally, the formula $\text{proj}_{\mathbf{v}}\mathbf{u} = \frac{\mathbf{u}\cdot\mathbf{v}}{\|\mathbf{v}\|^2}\mathbf{v}$ uses $\|\mathbf{v}\|^2 = \mathbf{v}\cdot\mathbf{v}$ in the denominator. Using $\|\mathbf{v}\|$ (not squared) in the denominator is a common mistake that gives the wrong formula.
