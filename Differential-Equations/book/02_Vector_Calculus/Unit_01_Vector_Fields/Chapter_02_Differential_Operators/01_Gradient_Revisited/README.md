# The Gradient Revisited

The gradient was likely your first encounter with a differential operator in several variables: given a function $f(x,y)$ or $f(x,y,z)$, form the vector of its partial derivatives. But this definition — while correct — understates what the gradient is and why it is the right definition. This section develops the gradient from a more fundamental perspective: as the vector that encodes all directional information about how $f$ changes at a point, pointing precisely in the direction of steepest ascent.

## Review: Definition and Formula

For a scalar function $f: D \subseteq \mathbb{R}^3 \to \mathbb{R}$ that is differentiable at a point $\mathbf{p}$, the **gradient** of $f$ at $\mathbf{p}$ is the vector

$$\nabla f(\mathbf{p}) = \frac{\partial f}{\partial x}(\mathbf{p})\,\mathbf{i} + \frac{\partial f}{\partial y}(\mathbf{p})\,\mathbf{j} + \frac{\partial f}{\partial z}(\mathbf{p})\,\mathbf{k}.$$

The notation $\nabla f$ (read "del $f$" or "grad $f$") assembles the partial derivatives into a single vector. Regarded as a function of $\mathbf{p}$, the gradient $\nabla f: D \to \mathbb{R}^3$ is itself a vector field.

## The Directional Derivative

To understand the gradient's full significance, we must first understand directional derivatives. Partial derivatives measure the rate of change of $f$ along the coordinate axes — special directions. But $f$ can be differentiated in any direction.

**Definition.** Let $\hat{\mathbf{u}}$ be a unit vector. The **directional derivative** of $f$ at $\mathbf{p}$ in the direction $\hat{\mathbf{u}}$ is

$$D_{\hat{\mathbf{u}}} f(\mathbf{p}) = \lim_{h \to 0} \frac{f(\mathbf{p} + h\hat{\mathbf{u}}) - f(\mathbf{p})}{h}.$$

This is the instantaneous rate of change of $f$ as you move from $\mathbf{p}$ in the direction $\hat{\mathbf{u}}$.

**Theorem.** If $f$ is differentiable at $\mathbf{p}$, then for any unit vector $\hat{\mathbf{u}}$,

$$D_{\hat{\mathbf{u}}} f(\mathbf{p}) = \nabla f(\mathbf{p}) \cdot \hat{\mathbf{u}}.$$

**Proof sketch.** Apply the chain rule to $g(t) = f(\mathbf{p} + t\hat{\mathbf{u}})$ at $t = 0$:

$$g'(0) = \nabla f(\mathbf{p} + 0\cdot\hat{\mathbf{u}}) \cdot \hat{\mathbf{u}} = \nabla f(\mathbf{p}) \cdot \hat{\mathbf{u}}.$$

But $g'(0)$ is exactly $D_{\hat{\mathbf{u}}}f(\mathbf{p})$ by definition.

This theorem is the key: the gradient encodes all directional information about how $f$ changes. Once you know $\nabla f(\mathbf{p})$, you know $D_{\hat{\mathbf{u}}}f(\mathbf{p})$ for every direction $\hat{\mathbf{u}}$ via a single dot product.

## Direction of Maximum Increase

Since $D_{\hat{\mathbf{u}}}f = \nabla f \cdot \hat{\mathbf{u}} = |\nabla f||\hat{\mathbf{u}}|\cos\theta = |\nabla f|\cos\theta$, where $\theta$ is the angle between $\nabla f$ and $\hat{\mathbf{u}}$:

- The directional derivative is **maximized** when $\cos\theta = 1$, i.e., when $\hat{\mathbf{u}} = \nabla f / |\nabla f|$. The maximum rate of increase is $|\nabla f|$.
- The directional derivative is **minimized** (most negative) when $\hat{\mathbf{u}} = -\nabla f/|\nabla f|$. The maximum rate of decrease is $-|\nabla f|$.
- The directional derivative is **zero** when $\hat{\mathbf{u}}$ is perpendicular to $\nabla f$ — that is, when you move along a level set of $f$.

**Interpretation.** The gradient $\nabla f(\mathbf{p})$ points in the direction of steepest ascent of $f$ at $\mathbf{p}$, with magnitude equal to the slope in that direction. Moving perpendicular to $\nabla f$ keeps $f$ constant (to first order).

## Gradient as Normal to Level Sets

**Theorem.** Let $f: \mathbb{R}^3 \to \mathbb{R}$ be $C^1$, and let $S = \{(x,y,z) : f(x,y,z) = c\}$ be a level surface through $\mathbf{p}$. Then $\nabla f(\mathbf{p})$ is perpendicular to $S$ at $\mathbf{p}$.

**Proof.** Let $\mathbf{r}(t)$ be any smooth curve on $S$ passing through $\mathbf{p}$ at $t = 0$, so $f(\mathbf{r}(t)) = c$ for all $t$. Differentiating: $\nabla f(\mathbf{r}(t)) \cdot \mathbf{r}'(t) = 0$. At $t=0$, this gives $\nabla f(\mathbf{p}) \cdot \mathbf{r}'(0) = 0$. Since $\mathbf{r}'(0)$ is an arbitrary tangent vector to $S$ at $\mathbf{p}$, the gradient is perpendicular to all tangent vectors, hence normal to $S$.

This theorem has immediate practical consequence: the equation of the tangent plane to the surface $f(x,y,z) = c$ at the point $\mathbf{p} = (x_0, y_0, z_0)$ is

$$\nabla f(\mathbf{p}) \cdot (\mathbf{r} - \mathbf{p}) = 0,$$

or in components,

$$f_x(x_0,y_0,z_0)(x-x_0) + f_y(x_0,y_0,z_0)(y-y_0) + f_z(x_0,y_0,z_0)(z-z_0) = 0.$$

**Example.** For the sphere $f(x,y,z) = x^2 + y^2 + z^2 = 1$, the gradient is $\nabla f = (2x, 2y, 2z)$. At the point $(1/\sqrt{3}, 1/\sqrt{3}, 1/\sqrt{3})$, the gradient is proportional to $(1,1,1)$, pointing radially outward — as one expects geometrically for the normal to a sphere.

## Worked Examples

**Example 1.** Let $f(x,y) = \sin(xy)$. Find $\nabla f$ and compute $D_{\hat{\mathbf{u}}}f$ at the point $(\pi/2, 1)$ in the direction $\hat{\mathbf{u}} = (3/5, 4/5)$.

$\nabla f = (y\cos(xy), x\cos(xy))$. At $(\pi/2, 1)$: $\nabla f = (1 \cdot \cos(\pi/2), (\pi/2)\cos(\pi/2)) = (0, 0)$.

The gradient is zero at this point (it is a critical point of $f$), so $D_{\hat{\mathbf{u}}}f = 0$ in every direction. The function is locally flat here.

**Example 2.** Let $T(x,y,z) = 100 - x^2 - 2y^2 - 3z^2$ represent temperature in a solid. A heat-seeking particle at $(1,1,1)$ should move in the direction of increasing $T$.

$\nabla T = (-2x, -4y, -6z)$. At $(1,1,1)$: $\nabla T = (-2, -4, -6)$. The temperature decreases most rapidly in the direction $(-2,-4,-6)/\sqrt{56}$. To move toward higher temperature, the particle should move in the direction $-\nabla T = (2,4,6)/\sqrt{56}$, i.e., toward the origin.

The maximum rate of temperature decrease at $(1,1,1)$ is $|\nabla T| = \sqrt{4 + 16 + 36} = \sqrt{56} = 2\sqrt{14}$ degrees per unit distance.

## The Gradient in Other Coordinate Systems

In cylindrical coordinates $(r, \theta, z)$:

$$\nabla f = \frac{\partial f}{\partial r}\,\hat{\mathbf{r}} + \frac{1}{r}\frac{\partial f}{\partial \theta}\,\hat{\boldsymbol{\theta}} + \frac{\partial f}{\partial z}\,\hat{\mathbf{k}}.$$

In spherical coordinates $(\rho, \theta, \phi)$ (with $\rho$ the radial distance, $\theta$ the polar angle, $\phi$ the azimuthal angle):

$$\nabla f = \frac{\partial f}{\partial \rho}\,\hat{\boldsymbol{\rho}} + \frac{1}{\rho}\frac{\partial f}{\partial \theta}\,\hat{\boldsymbol{\theta}} + \frac{1}{\rho\sin\theta}\frac{\partial f}{\partial \phi}\,\hat{\boldsymbol{\phi}}.$$

The extra scale factors $(1/r$ and $1/\rho\sin\theta)$ account for the fact that equal angular displacements correspond to different arc lengths at different radii.

## Summary

The gradient $\nabla f$ is the vector that encodes the infinitesimal behavior of $f$ in all directions simultaneously: $D_{\hat{\mathbf{u}}}f = \nabla f \cdot \hat{\mathbf{u}}$. It points in the direction of steepest ascent of $f$ and is everywhere normal to the level sets of $f$. These two properties — directional derivative formula and normality to level sets — are the gradient's most important geometric attributes, and they underlie everything from the computation of tangent planes to the Lagrange multiplier method of constrained optimization.
