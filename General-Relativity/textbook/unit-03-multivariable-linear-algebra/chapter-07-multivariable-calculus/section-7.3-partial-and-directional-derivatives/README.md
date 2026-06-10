# Section 7.3: Partial and Directional Derivatives

---

## Section Introduction

The derivative of a single-variable function $f: \mathbb{R}\to\mathbb{R}$ at a point $a$ is the limit $f'(a) = \lim_{h\to 0}(f(a+h)-f(a))/h$. For a function $f: \mathbb{R}^n\to\mathbb{R}$, we must specify a *direction* in which to differentiate, because there are infinitely many directions in $\mathbb{R}^n$. The simplest directional derivatives are the **partial derivatives**: differentiation along the coordinate axes.

The **partial derivative** $\partial f/\partial x^i$ at a point $\mathbf{a}$ holds all other variables fixed and differentiates with respect to $x^i$ alone:
$$\frac{\partial f}{\partial x^i}(\mathbf{a}) = \lim_{h\to 0}\frac{f(a^1, \ldots, a^i + h, \ldots, a^n) - f(\mathbf{a})}{h}$$
This is single-variable differentiation in disguise — all the single-variable differentiation rules (chain rule, product rule, etc.) apply directly.

The **directional derivative** in the direction of a unit vector $\hat{\mathbf{u}}$ generalizes this: $D_{\hat{\mathbf{u}}}f(\mathbf{a}) = \lim_{h\to 0}(f(\mathbf{a}+h\hat{\mathbf{u}})-f(\mathbf{a}))/h$. If $f$ is differentiable, then $D_{\hat{\mathbf{u}}}f = \nabla f\cdot\hat{\mathbf{u}}$ where $\nabla f = (\partial f/\partial x^1, \ldots, \partial f/\partial x^n)$ is the **gradient vector**. The gradient points in the direction of steepest ascent and has magnitude equal to the maximum directional derivative.

A cautionary point: the existence of all partial derivatives at a point does not guarantee continuity or differentiability there. There are functions with all partial derivatives existing at a point but that are not continuous — a pathology with no single-variable analogue. The correct multivariable generalization of differentiability is the total derivative (Section 7.4), which requires a good linear approximation in *all* directions, not just the coordinate ones.

---

## Subsections

- [7.3.1: Partial Derivatives and Notation](7.3.1-partial-derivatives.md)
- [7.3.2: The Gradient Vector](7.3.2-gradient.md)
- [7.3.3: Directional Derivatives](7.3.3-directional.md)
- [7.3.4: Higher-Order Partial Derivatives and Clairaut's Theorem](7.3.4-higher-order.md)
- [7.3.5: Geometric Interpretation and Level Sets](7.3.5-geometry.md)
