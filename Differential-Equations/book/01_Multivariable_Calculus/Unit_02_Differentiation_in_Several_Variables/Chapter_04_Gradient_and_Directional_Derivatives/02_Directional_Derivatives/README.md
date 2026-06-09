# Directional Derivatives

The partial derivative $\partial f/\partial x$ measures the rate of change of $f$ as one moves in the $x$-direction. But why privilege the coordinate directions? A hiker on a mountainous terrain wants to know how steeply the elevation changes in the direction they are actually walking, which may be northeast, not due east or due north. The directional derivative generalizes partial derivatives to arbitrary directions, and its connection to the gradient reveals a deep geometric structure.

## Definition

The **directional derivative** of $f: D\subseteq\mathbb{R}^n\to\mathbb{R}$ at $\mathbf{a}\in D$ in the direction of a unit vector $\hat{\mathbf{u}}\in\mathbb{R}^n$ (with $\|\hat{\mathbf{u}}\| = 1$) is

$$D_{\hat{\mathbf{u}}}f(\mathbf{a}) = \lim_{h\to 0} \frac{f(\mathbf{a}+h\hat{\mathbf{u}}) - f(\mathbf{a})}{h},$$

provided this limit exists. This is the rate of change of $f$ along the straight line through $\mathbf{a}$ in the direction $\hat{\mathbf{u}}$.

Taking $\hat{\mathbf{u}} = \mathbf{e}_i$, the $i$-th standard basis vector, gives $D_{\mathbf{e}_i}f(\mathbf{a}) = \partial f/\partial x_i(\mathbf{a})$. Partial derivatives are the special cases of directional derivatives in coordinate directions.

## The Key Formula

**Theorem.** If $f$ is differentiable at $\mathbf{a}$, then for any unit vector $\hat{\mathbf{u}}$,

$$D_{\hat{\mathbf{u}}}f(\mathbf{a}) = \nabla f(\mathbf{a})\cdot\hat{\mathbf{u}}.$$

**Proof.** By the definition of differentiability:

$\frac{f(\mathbf{a}+h\hat{\mathbf{u}}) - f(\mathbf{a})}{h} = \frac{Df(\mathbf{a})(h\hat{\mathbf{u}}) + r(h\hat{\mathbf{u}})}{h} = Df(\mathbf{a})(\hat{\mathbf{u}}) + \frac{r(h\hat{\mathbf{u}})}{h}$.

The first term is $\nabla f(\mathbf{a})\cdot\hat{\mathbf{u}}$ (constant). The second: $|r(h\hat{\mathbf{u}})|/|h| = \|r(h\hat{\mathbf{u}})\|/\|h\hat{\mathbf{u}}\| \to 0$ since $r = o(\|\mathbf{h}\|)$ and $\|h\hat{\mathbf{u}}\| = |h|$. Taking $h\to 0$ gives the result.

## Dependence on the Hypothesis

Without differentiability, the formula $D_{\hat{\mathbf{u}}}f = \nabla f\cdot\hat{\mathbf{u}}$ can fail. Consider $f(x,y) = \begin{cases}\frac{x^2y}{x^4+y^2} & (x,y)\neq(0,0)\\ 0 & (x,y)=(0,0)\end{cases}$. Along the direction $\hat{\mathbf{u}} = (\cos\theta, \sin\theta)$:

$D_{\hat{\mathbf{u}}}f(0,0) = \lim_{h\to 0}\frac{f(h\cos\theta, h\sin\theta)}{h} = \lim_{h\to 0}\frac{h^2\cos^2\theta\cdot h\sin\theta}{h^4\cos^4\theta + h^2\sin^2\theta}\cdot\frac{1}{h}$.

For $\sin\theta\neq 0$: $= \lim_{h\to 0}\frac{h\cos^2\theta\sin\theta}{h^2\cos^4\theta+\sin^2\theta} = 0$ (numerator $\to 0$, denominator $\to\sin^2\theta\neq 0$).

For $\theta = 0$ (the $x$-direction): $D_{\mathbf{e}_1}f(0,0) = f_x(0,0) = 0$.

So all directional derivatives are $0$ at the origin, consistent with $\nabla f(0,0) = (0,0)$. However, $f$ is not continuous at $(0,0)$ (it equals $1/2$ along $y = x^2$), hence not differentiable. The directional derivatives all happen to agree with the formula, but this is coincidental: the function is not differentiable.

## Worked Examples

**Example 1.** $f(x,y) = x^2 - 3y^2$. Find the directional derivative at $(1,1)$ in the direction $(3,4)/5$.

$\nabla f = (2x, -6y)$. At $(1,1)$: $\nabla f = (2,-6)$.

$D_{\hat{\mathbf{u}}}f(1,1) = (2,-6)\cdot(3/5, 4/5) = 6/5 - 24/5 = -18/5$.

Interpretation: at the point $(1,1)$, moving in the direction $(3,4)/5$ (slightly north of east), the function decreases at rate $18/5$.

**Example 2.** $f(x,y,z) = xyz$. Find the directional derivative at $(1,1,1)$ in the direction of $\mathbf{v} = (1,2,2)$.

First, normalize: $\|\mathbf{v}\| = 3$, so $\hat{\mathbf{u}} = (1/3, 2/3, 2/3)$.

$\nabla f = (yz, xz, xy)$. At $(1,1,1)$: $\nabla f = (1,1,1)$.

$D_{\hat{\mathbf{u}}}f = (1,1,1)\cdot(1/3,2/3,2/3) = 1/3+2/3+2/3 = 5/3$.

## Directional Derivatives Without Differentiability

The directional derivative $D_{\hat{\mathbf{u}}}f(\mathbf{a})$ can exist for every $\hat{\mathbf{u}}$ even when $f$ is not differentiable at $\mathbf{a}$. In such cases, the formula $D_{\hat{\mathbf{u}}}f = \nabla f\cdot\hat{\mathbf{u}}$ may fail: the directional derivative in direction $\hat{\mathbf{u}}$ need not equal the dot product of the gradient (if it exists) with $\hat{\mathbf{u}}$.

**Example.** Let $f(x,y) = \sqrt{|xy|}$. At the origin:

$f_x(0,0) = 0$ and $f_y(0,0) = 0$, so $\nabla f(0,0) = (0,0)$ would predict $D_{\hat{\mathbf{u}}}f = 0$ for all $\hat{\mathbf{u}}$.

But $D_{(1,1)/\sqrt{2}}f(0,0) = \lim_{h\to 0}\frac{\sqrt{|h/\sqrt{2}\cdot h/\sqrt{2}|}}{h} = \lim_{h\to 0}\frac{|h|/\sqrt{2}}{h}$, which does not exist (limit from right is $1/\sqrt{2}$, from left is $-1/\sqrt{2}$). So the directional derivative in the diagonal direction fails to exist, even though both partial derivatives exist.

## Non-Unit Directions

Some texts define the directional derivative for arbitrary nonzero vectors $\mathbf{v}$ (not necessarily unit) as $D_\mathbf{v}f(\mathbf{a}) = \nabla f(\mathbf{a})\cdot\mathbf{v}$. This convention makes computation easier but conflates the rate of change with the magnitude of $\mathbf{v}$: $D_{2\hat{\mathbf{u}}}f = 2D_{\hat{\mathbf{u}}}f$ in this convention, even though "moving in direction $\hat{\mathbf{u}}$ twice as fast" should give the same instantaneous rate of change. Using only unit vectors for directional derivatives avoids this issue.
