# Gradient Vector

The derivative of a scalar function $f: \mathbb{R}^n \to \mathbb{R}$ packages all partial derivatives into a single geometric object: the gradient vector. Far from being merely a compact notation, the gradient has a rich geometric meaning — it points in the direction of steepest increase and is perpendicular to the level sets of $f$ — and it is the central tool in optimization, differential geometry, and the study of partial differential equations.

## Definition

The **gradient** of a differentiable function $f: D\subseteq\mathbb{R}^n\to\mathbb{R}$ at a point $\mathbf{a} = (a_1, \ldots, a_n) \in D$ is the vector

$$\nabla f(\mathbf{a}) = \left(\frac{\partial f}{\partial x_1}(\mathbf{a}),\; \frac{\partial f}{\partial x_2}(\mathbf{a}),\; \ldots,\; \frac{\partial f}{\partial x_n}(\mathbf{a})\right) \in \mathbb{R}^n.$$

In $\mathbb{R}^2$: $\nabla f = (f_x, f_y)$. In $\mathbb{R}^3$: $\nabla f = (f_x, f_y, f_z)$.

The gradient is not a scalar; it is a vector whose components are the partial derivatives. It should be thought of as a function $\nabla f: D \to \mathbb{R}^n$ that assigns to each point $\mathbf{a}$ a vector $\nabla f(\mathbf{a})$.

## Relationship to the Total Derivative

For a differentiable scalar function $f:\mathbb{R}^n\to\mathbb{R}$, the total derivative $Df(\mathbf{a})$ is a linear functional on $\mathbb{R}^n$ — a $1\times n$ row vector (the Jacobian). The gradient is the corresponding column vector: $\nabla f(\mathbf{a}) = [Df(\mathbf{a})]^T$. The action of the total derivative on a displacement $\mathbf{h}$ is

$$Df(\mathbf{a})(\mathbf{h}) = \nabla f(\mathbf{a})\cdot\mathbf{h},$$

the dot product of the gradient with $\mathbf{h}$.

## Algebraic Properties

The gradient satisfies the same rules as the derivative:

- **Linearity:** $\nabla(af+bg) = a\nabla f + b\nabla g$ for constants $a,b$.
- **Product rule:** $\nabla(fg) = g\nabla f + f\nabla g$.
- **Quotient rule:** $\nabla(f/g) = (g\nabla f - f\nabla g)/g^2$ where $g\neq 0$.
- **Chain rule:** if $h(t) = f(\mathbf{r}(t))$, then $h'(t) = \nabla f(\mathbf{r}(t))\cdot\mathbf{r}'(t)$.
- **Chain rule (scalar composition):** if $g:\mathbb{R}\to\mathbb{R}$, then $\nabla(g\circ f) = g'(f)\nabla f$.

## Worked Examples

**Example 1.** $f(x,y) = x^2e^{y}$. $\nabla f = (2xe^y, x^2e^y)$. At $(1, 0)$: $\nabla f(1,0) = (2, 1)$.

**Example 2.** $f(x,y,z) = \frac{1}{\sqrt{x^2+y^2+z^2}} = (x^2+y^2+z^2)^{-1/2}$. By the chain rule:

$f_x = -\frac{x}{(x^2+y^2+z^2)^{3/2}}$, and similarly for $f_y$, $f_z$.

$\nabla f = -\frac{1}{r^3}(x,y,z) = -\frac{\mathbf{r}}{r^3} = -\frac{\hat{\mathbf{r}}}{r^2}$,

where $r = \|\mathbf{r}\|$. This is the Coulomb/gravitational field: the gradient of the potential $1/r$ is the radially-inward force field proportional to $1/r^2$.

**Example 3.** If $f(\mathbf{x}) = \mathbf{a}\cdot\mathbf{x}$ (a linear function), then $f(x_1,\ldots,x_n) = a_1x_1+\cdots+a_nx_n$, so $\nabla f = (a_1,\ldots,a_n) = \mathbf{a}$ (constant). The gradient of a linear function is the vector defining it.

## The Gradient as a Vector Field

Because $\nabla f$ assigns a vector to each point in $D$, it is a **vector field** — the **gradient field** of $f$. Geometrically, one can imagine drawing the gradient vector at every point of the domain; the collection of these arrows is the gradient field. Gradient fields are conservative vector fields (the fundamental theorem of calculus for line integrals), and they arise naturally in physics as force fields derived from potentials.

## The Del Operator

The symbol $\nabla$ (nabla or del) is a formal vector of partial derivative operators:

$$\nabla = \left(\frac{\partial}{\partial x_1}, \frac{\partial}{\partial x_2}, \ldots, \frac{\partial}{\partial x_n}\right).$$

When applied to a scalar $f$, it gives the gradient $\nabla f$. When dotted with a vector field $\mathbf{F}$, it gives the divergence $\nabla\cdot\mathbf{F} = \sum_i \partial F_i/\partial x_i$. When crossed with a vector field in $\mathbb{R}^3$, it gives the curl $\nabla\times\mathbf{F}$. Applied twice: $\nabla\cdot(\nabla f) = \Delta f$, the Laplacian. The del notation provides a compact and suggestive syntax for all these operations.

## Connection to Differential Equations

The gradient connects to differential equations in multiple ways. The heat equation $\partial u/\partial t = k\Delta u = k\nabla\cdot(\nabla u)$ is the divergence of the gradient of $u$ — it models heat flowing from regions of high temperature to regions of low temperature, i.e., against the gradient. Laplace's equation $\Delta u = \nabla\cdot(\nabla u) = 0$ says the gradient field of $u$ is divergence-free. The method of steepest descent, used in numerical methods for differential equations, iterates in the direction of $-\nabla f$ to minimize a functional.
