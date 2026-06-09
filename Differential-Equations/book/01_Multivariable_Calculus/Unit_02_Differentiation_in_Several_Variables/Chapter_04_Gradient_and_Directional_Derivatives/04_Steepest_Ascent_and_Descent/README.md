# Steepest Ascent and Descent

The directional derivative formula $D_{\hat{\mathbf{u}}}f(\mathbf{a}) = \nabla f(\mathbf{a})\cdot\hat{\mathbf{u}}$ expresses the rate of change of $f$ in direction $\hat{\mathbf{u}}$ as a dot product. The Cauchy-Schwarz inequality then immediately tells us which direction produces the greatest rate of increase, which produces the greatest rate of decrease, and which directions leave $f$ unchanged. The answers — gradient, negative gradient, and perpendicular to gradient — are among the most practically important facts in all of multivariable calculus.

## The Maximum of the Directional Derivative

By the Cauchy-Schwarz inequality, for any unit vector $\hat{\mathbf{u}}$:

$$D_{\hat{\mathbf{u}}}f(\mathbf{a}) = \nabla f(\mathbf{a})\cdot\hat{\mathbf{u}} \leq \|\nabla f(\mathbf{a})\|\cdot\|\hat{\mathbf{u}}\| = \|\nabla f(\mathbf{a})\|,$$

with equality when $\hat{\mathbf{u}} = \nabla f(\mathbf{a})/\|\nabla f(\mathbf{a})\|$ (assuming $\nabla f(\mathbf{a}) \neq \mathbf{0}$).

Similarly, $D_{\hat{\mathbf{u}}}f(\mathbf{a}) \geq -\|\nabla f(\mathbf{a})\|$, with equality when $\hat{\mathbf{u}} = -\nabla f(\mathbf{a})/\|\nabla f(\mathbf{a})\|$.

And $D_{\hat{\mathbf{u}}}f(\mathbf{a}) = 0$ when $\hat{\mathbf{u}}\perp\nabla f(\mathbf{a})$, i.e., when one moves tangentially along the level set.

**Summary:**
- The direction of **steepest ascent** (maximum directional derivative) is $\hat{\mathbf{u}} = \nabla f/\|\nabla f\|$.
- The direction of **steepest descent** (most negative directional derivative) is $\hat{\mathbf{u}} = -\nabla f/\|\nabla f\|$.
- The directions of **no change** (zero directional derivative) are those perpendicular to $\nabla f$, i.e., tangent to the level set.
- The **rate of steepest ascent** equals $\|\nabla f(\mathbf{a})\|$.

## Geometric Picture

The gradient field and level sets are complementary: the gradient vectors are everywhere perpendicular to the level sets, pointing toward larger values of $f$. The steepest path from a starting point to a local maximum of $f$ follows the gradient: at each point, move in the direction $\nabla f$. This trajectory — an **integral curve** of the gradient vector field — crosses level sets perpendicularly and climbs as steeply as possible.

The steepest path in the downward direction follows $-\nabla f$. On a smooth surface (the graph of $f$), water flows along the steepest descent path, which is why rivers follow gradient lines of the topographic elevation function (approximately, in the absence of other forces).

## Worked Example

$f(x,y) = 100 - x^2 - 2y^2$. The level curves are ellipses $x^2 + 2y^2 = c$.

At the point $(3, 2)$: $\nabla f = (-2x, -4y) = (-6, -8)$.

Direction of steepest ascent: $\hat{\mathbf{u}} = (-6,-8)/10 = (-3/5, -4/5)$ (toward the origin, i.e., toward the maximum at the origin).

Maximum rate of increase: $\|\nabla f(3,2)\| = 10$.

Direction of no change: any vector perpendicular to $(-6,-8)$, e.g., $(8,-6)/10$ or $(-8,6)/10$ (tangent to the ellipse $x^2+2y^2 = 17$ at $(3,2)$).

## Gradient Descent in Optimization

The practical importance of the steepest descent direction lies in numerical optimization. Given a function $f$ to minimize (think: a loss function in machine learning), the **gradient descent** algorithm iterates:

$$\mathbf{x}_{k+1} = \mathbf{x}_k - \alpha_k \nabla f(\mathbf{x}_k),$$

where $\alpha_k > 0$ is the step size (learning rate). Each step moves in the direction of steepest decrease. If $f$ has a unique global minimum and the step sizes are chosen appropriately, the sequence $\mathbf{x}_k$ converges to the minimizer.

Gradient descent is the backbone of training neural networks (via backpropagation, which is just the chain rule for computing $\nabla f$ efficiently). Understanding why it works — it decreases $f$ at the fastest possible rate — requires exactly the material of this section.

**Convergence analysis.** For a strongly convex function $f$ (meaning $f(\mathbf{y}) \geq f(\mathbf{x}) + \nabla f(\mathbf{x})\cdot(\mathbf{y}-\mathbf{x}) + \frac{\mu}{2}\|\mathbf{y}-\mathbf{x}\|^2$ for some $\mu > 0$), gradient descent with constant step size $\alpha < 2/L$ (where $L$ is the Lipschitz constant of $\nabla f$) converges geometrically to the minimum.

## Ascent and Descent Along Curves

The steepest ascent and descent directions are instantaneous: they are the directions that maximize or minimize the derivative at a single point. Following the gradient or negative gradient over time traces a curve in $\mathbb{R}^n$, the **gradient flow**:

$$\frac{d\mathbf{x}}{dt} = -\nabla f(\mathbf{x}(t)).$$

This ODE always has solutions (at least locally, by the Picard-Lindelöf theorem), and solutions flow toward critical points of $f$ where $\nabla f = \mathbf{0}$. The stability of a critical point under gradient flow is determined by the Hessian (Chapter 5): if the Hessian is positive definite at a critical point, that point is a stable equilibrium of the gradient flow (a local minimum of $f$).

## Connection to Conservative Fields

In physics, a force field $\mathbf{F}$ is **conservative** if $\mathbf{F} = -\nabla V$ for some potential energy function $V$. The force then always points in the direction of steepest descent of the potential — particles move toward lower potential energy. The level sets of $V$ are the equipotential surfaces, and the force is always perpendicular to them. This is precisely the gradient-perpendicular-to-level-sets geometry of the previous section.

## Common Pitfalls

The gradient points in the direction of steepest ascent of $f$, not of the graph of $f$ as a surface. These are different things. The gradient of $f:\mathbb{R}^2\to\mathbb{R}$ is a vector in $\mathbb{R}^2$ (the domain), not in $\mathbb{R}^3$ (where the graph lives).

Also, "steepest ascent" is a local notion: the gradient direction gives the steepest increase at the given point, not a direction that remains steepest along the entire path. Following the gradient continuously traces a curve that is everywhere locally steepest, but this curve need not be the straight-line path to the maximum.
