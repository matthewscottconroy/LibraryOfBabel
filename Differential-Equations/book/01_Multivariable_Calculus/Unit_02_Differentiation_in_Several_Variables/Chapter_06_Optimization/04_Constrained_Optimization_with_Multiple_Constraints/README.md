# Constrained Optimization with Multiple Constraints

A single constraint reduces the $n$-dimensional search space to an $(n-1)$-dimensional surface. Two constraints reduce it to an $(n-2)$-dimensional intersection, and so on. The method of Lagrange multipliers extends naturally to multiple constraints: the gradient of the objective function must lie in the span of the gradients of all the constraints, with one Lagrange multiplier per constraint.

## Setup with $k$ Constraints

Optimize $f(\mathbf{x})$ subject to $k$ constraints $g_1(\mathbf{x}) = c_1, \, g_2(\mathbf{x}) = c_2, \, \ldots, \, g_k(\mathbf{x}) = c_k$, where $f, g_1, \ldots, g_k: \mathbb{R}^n\to\mathbb{R}$ are smooth and $k < n$.

The constraint set is $S = \{g_1=c_1\}\cap\{g_2=c_2\}\cap\cdots\cap\{g_k=c_k\}$, which generically has dimension $n-k$.

## The Lagrange Condition

**Theorem.** Let $\mathbf{a}$ be a local extremum of $f$ on $S$, and suppose the **regularity condition** holds: the gradients $\nabla g_1(\mathbf{a}), \ldots, \nabla g_k(\mathbf{a})$ are linearly independent. Then there exist scalars $\lambda_1, \ldots, \lambda_k$ such that

$$\nabla f(\mathbf{a}) = \lambda_1\nabla g_1(\mathbf{a}) + \lambda_2\nabla g_2(\mathbf{a}) + \cdots + \lambda_k\nabla g_k(\mathbf{a}).$$

**Proof sketch.** The tangent space to $S$ at $\mathbf{a}$ is $T = \bigcap_{i=1}^k\ker(\nabla g_i(\mathbf{a})^T)$, the set of directions perpendicular to all $\nabla g_i$. At a constrained extremum, $\nabla f(\mathbf{a})\perp T$ (same argument as the one-constraint case). The vectors perpendicular to $T$ are exactly the span of $\{\nabla g_1(\mathbf{a}),\ldots,\nabla g_k(\mathbf{a})\}$ (by linear algebra: $T^\perp = \text{span}\{\nabla g_i\}$ when the $\nabla g_i$ are linearly independent). Hence $\nabla f(\mathbf{a})\in\text{span}\{\nabla g_i(\mathbf{a})\}$.

## The Lagrange System

The conditions give $n$ equations from $\nabla f = \sum_i\lambda_i\nabla g_i$, plus $k$ equations from $g_i = c_i$, for a total of $n+k$ equations in $n+k$ unknowns $(x_1,\ldots,x_n,\lambda_1,\ldots,\lambda_k)$.

Equivalently, define the **Lagrangian function**:

$$\mathcal{L}(\mathbf{x},\boldsymbol{\lambda}) = f(\mathbf{x}) - \sum_{i=1}^k\lambda_i(g_i(\mathbf{x})-c_i).$$

Setting all partial derivatives of $\mathcal{L}$ to zero:
- $\frac{\partial\mathcal{L}}{\partial x_j} = \frac{\partial f}{\partial x_j} - \sum_i\lambda_i\frac{\partial g_i}{\partial x_j} = 0$ (the Lagrange condition).
- $\frac{\partial\mathcal{L}}{\partial\lambda_i} = -(g_i(\mathbf{x})-c_i) = 0$ (the constraints).

The critical points of $\mathcal{L}$ are exactly the constrained critical points plus the Lagrange multipliers.

## Worked Example: Optimization on a Curve

Minimize $f(x,y,z) = x^2+y^2+z^2$ (distance from origin squared) subject to $g_1(x,y,z)=x+y+z=1$ and $g_2(x,y,z) = x-y=0$.

The constraints define the intersection of the plane $x+y+z=1$ and the plane $x=y$: a line in $\mathbb{R}^3$.

$\nabla f = (2x,2y,2z)$, $\nabla g_1 = (1,1,1)$, $\nabla g_2 = (1,-1,0)$.

Lagrange conditions: $(2x,2y,2z) = \lambda_1(1,1,1)+\lambda_2(1,-1,0)$.

$2x = \lambda_1+\lambda_2$, $2y = \lambda_1-\lambda_2$, $2z = \lambda_1$.

From constraint 2: $x=y$, so $\lambda_1+\lambda_2 = \lambda_1-\lambda_2 \Rightarrow 2\lambda_2=0 \Rightarrow \lambda_2=0$.

So $2x=2y=2z=\lambda_1$, giving $x=y=z$.

Constraint 1: $3x=1 \Rightarrow x=y=z=1/3$.

Minimum distance: $f(1/3,1/3,1/3) = 3(1/9) = 1/3$.

## Isoperimetric Problems

A classical example with two constraints: maximize the area $A$ of a rectangle with given perimeter $P$ and fixed diagonal $d$.

Let the sides be $a$ and $b$. Constraints: $2(a+b) = P$ and $a^2+b^2 = d^2$.

By symmetry, the maximum area is achieved at $a=b$ (a square), but this can be verified via Lagrange multipliers.

More generally, the **isoperimetric problem** — maximize area enclosed by a curve of fixed length — is a classical problem in the calculus of variations, the infinite-dimensional generalization of Lagrange multipliers.

## The Regularity Condition and Its Failure

The regularity condition that $\nabla g_1,\ldots,\nabla g_k$ be linearly independent at $\mathbf{a}$ is called a **constraint qualification**. When it fails, the constraint set may have a singular point at $\mathbf{a}$ (e.g., two curves meeting at a cusp), and the Lagrange conditions are no longer necessary. In such cases, one must analyze the constraint set more carefully.

**Example of failure.** Minimize $f(x,y)=x+y$ subject to $g_1(x,y)=x^2=0$ and $g_2(x,y)=y^2=0$. The constraint set is just the origin $\{(0,0)\}$, which is a single point — trivially both the maximum and the minimum. But $\nabla g_1(0,0) = (0,0)$ and $\nabla g_2(0,0) = (0,0)$ are linearly dependent (both zero), so the regularity condition fails.

## Connection to KKT Conditions

In constrained optimization with both equality and inequality constraints — the setting of modern optimization theory — the Lagrange conditions generalize to the **Karush-Kuhn-Tucker (KKT) conditions**. For an inequality constraint $h(\mathbf{x}) \leq 0$, the corresponding Lagrange multiplier must be non-negative, and the complementary slackness condition $\lambda\cdot h(\mathbf{x}) = 0$ must hold. This is the foundation of convex optimization, linear programming duality, and support vector machine training.
