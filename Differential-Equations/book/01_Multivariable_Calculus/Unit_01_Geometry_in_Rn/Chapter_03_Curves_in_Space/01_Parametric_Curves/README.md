# Parametric Curves

Every motion in space traces a curve. A ball thrown through the air follows a parabolic arc; a planet orbits the sun in an ellipse; a marble rolling along a twisted ramp traces a helix. In each case, the position of the object at time $t$ is a point in $\mathbb{R}^3$, and as $t$ varies, that point sweeps out a one-dimensional curve embedded in three-dimensional space. The parametric approach encodes this directly: a curve is a function of a single real parameter, not a level set of equations.

## Definitions

A **parametric curve** (or **vector-valued function**) in $\mathbb{R}^n$ is a function $\mathbf{r}: I \to \mathbb{R}^n$ defined on an interval $I \subseteq \mathbb{R}$. In $\mathbb{R}^3$, it is written

$$\mathbf{r}(t) = (x(t), y(t), z(t)) = x(t)\,\mathbf{i} + y(t)\,\mathbf{j} + z(t)\,\mathbf{k}.$$

The variable $t$ is the **parameter** (often thought of as time). The **image** or **trace** of $\mathbf{r}$ is the set of all points $\{\mathbf{r}(t) : t \in I\}$ in $\mathbb{R}^n$, which is the curve itself as a geometric object.

The curve is **continuous** at $t_0$ if $\lim_{t\to t_0}\mathbf{r}(t) = \mathbf{r}(t_0)$, i.e., if each component function is continuous. It is **differentiable** at $t_0$ if each component is differentiable, with derivative

$$\mathbf{r}'(t_0) = (x'(t_0), y'(t_0), z'(t_0)) = \lim_{h\to 0}\frac{\mathbf{r}(t_0+h) - \mathbf{r}(t_0)}{h}.$$

The vector $\mathbf{r}'(t)$ is the **velocity vector** or **tangent vector** at the point $\mathbf{r}(t)$. It points in the direction of motion along the curve at time $t$.

## Regularity

A curve is **regular** at $t_0$ if $\mathbf{r}'(t_0) \neq \mathbf{0}$. At a regular point, the tangent direction is well defined and the curve has no cusps or corners. Throughout most of this chapter, we assume curves are smooth (infinitely differentiable) and regular.

At an irregular point (where $\mathbf{r}'(t_0) = \mathbf{0}$), interesting behavior can occur: the curve might have a cusp, or it might double back on itself. The cycloid $\mathbf{r}(t) = (t - \sin t, 1 - \cos t)$ has cusps at $t = 0, \pm 2\pi, \pm 4\pi, \ldots$, where both component derivatives vanish.

## Standard Examples

**Circle of radius $R$ in the $xy$-plane:**

$$\mathbf{r}(t) = (R\cos t, R\sin t, 0), \quad t \in [0, 2\pi].$$

$\mathbf{r}'(t) = (-R\sin t, R\cos t, 0)$, which has magnitude $R$ and is always perpendicular to $\mathbf{r}(t)$. (Verify: $\mathbf{r}\cdot\mathbf{r}' = -R^2\cos t\sin t + R^2\sin t\cos t = 0$.)

**Circular helix:**

$$\mathbf{r}(t) = (a\cos t, a\sin t, bt), \quad t \in \mathbb{R}.$$

This is a circle of radius $a$ in the $xy$-plane that rises at rate $b$ per unit parameter. $\mathbf{r}'(t) = (-a\sin t, a\cos t, b)$, which has constant magnitude $\sqrt{a^2+b^2}$, so the helix is traversed at constant speed. The helix is the simplest genuinely three-dimensional curve and provides the canonical example for curvature and torsion computations.

**Line segment from $\mathbf{p}$ to $\mathbf{q}$:**

$$\mathbf{r}(t) = (1-t)\mathbf{p} + t\mathbf{q}, \quad t \in [0,1].$$

At $t=0$, $\mathbf{r}(0) = \mathbf{p}$; at $t=1$, $\mathbf{r}(1) = \mathbf{q}$. The velocity $\mathbf{r}'(t) = \mathbf{q} - \mathbf{p}$ is constant.

## Differentiation Rules

Derivatives of vector-valued functions obey rules analogous to those for scalar functions. If $\mathbf{r}(t)$ and $\mathbf{s}(t)$ are differentiable vector-valued functions and $f(t)$ is a differentiable scalar function, then:

- $(\mathbf{r} + \mathbf{s})' = \mathbf{r}' + \mathbf{s}'$
- $(f\mathbf{r})' = f'\mathbf{r} + f\mathbf{r}'$ (product rule)
- $(\mathbf{r}\cdot\mathbf{s})' = \mathbf{r}'\cdot\mathbf{s} + \mathbf{r}\cdot\mathbf{s}'$ (dot product rule)
- $(\mathbf{r}\times\mathbf{s})' = \mathbf{r}'\times\mathbf{s} + \mathbf{r}\times\mathbf{s}'$ (cross product rule; note order matters)
- $(\mathbf{r}(f(t)))' = f'(t)\mathbf{r}'(f(t))$ (chain rule)

The dot product and cross product rules follow from the product rule applied componentwise.

**Important consequence:** If $\|\mathbf{r}(t)\|$ is constant (the curve lies on a sphere), then $\mathbf{r}(t)\cdot\mathbf{r}(t) = \text{const}$, so differentiating: $2\mathbf{r}(t)\cdot\mathbf{r}'(t) = 0$. Thus $\mathbf{r}(t) \perp \mathbf{r}'(t)$ whenever the curve has constant distance from the origin. This is a vector-valued generalization of a fact from single-variable calculus: if $f^2$ is constant then $ff' = 0$.

## Worked Example: Tangent Line to a Helix

Find the tangent line to $\mathbf{r}(t) = (\cos t, \sin t, t)$ at $t = \pi/2$.

Point on curve: $\mathbf{r}(\pi/2) = (0, 1, \pi/2)$.

Velocity: $\mathbf{r}'(t) = (-\sin t, \cos t, 1)$, so $\mathbf{r}'(\pi/2) = (-1, 0, 1)$.

Tangent line: $\mathbf{L}(s) = (0, 1, \pi/2) + s(-1, 0, 1) = (-s, 1, \pi/2 + s)$.

## Reparameterization

Two parametric curves $\mathbf{r}(t)$ and $\mathbf{s}(u)$ have the same image if there is a differentiable bijection $t = \phi(u)$ such that $\mathbf{s}(u) = \mathbf{r}(\phi(u))$. In this case, $\mathbf{s}'(u) = \phi'(u)\mathbf{r}'(\phi(u))$ by the chain rule: the tangent vectors differ by the scalar factor $\phi'(u)$, which changes the speed of traversal but not the direction. Geometric properties of the curve (like curvature, to be defined in Section 3) must therefore be independent of parameterization.

## Common Pitfalls

The trace (image) of a curve and the curve (as a function) are different objects. Two functions $\mathbf{r}$ and $\mathbf{s}$ with the same image may traverse that image at different speeds, in different directions, or multiple times. For example, $\mathbf{r}(t) = (\cos t, \sin t)$ and $\mathbf{s}(t) = (\cos 2t, \sin 2t)$ have the same image (the unit circle), but $\mathbf{s}$ goes around twice as fast.

Another common error is computing the tangent vector at a point by substituting the point coordinates into $\mathbf{r}'(t)$, rather than finding the parameter value $t_0$ such that $\mathbf{r}(t_0)$ equals the given point. There may be multiple parameter values giving the same point (if the curve self-intersects), in which case the tangent direction depends on which passage through the point is desired.
