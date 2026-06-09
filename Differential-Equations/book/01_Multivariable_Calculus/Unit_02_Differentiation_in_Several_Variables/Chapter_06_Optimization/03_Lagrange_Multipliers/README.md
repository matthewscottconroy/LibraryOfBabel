# Lagrange Multipliers

Real optimization problems rarely involve an unconstrained function. A company maximizes profit subject to a budget. A physicist minimizes energy subject to conservation laws. An engineer designs a container of maximum volume subject to a fixed surface area. In each case, the function to be optimized is restricted to a constraint set, and the unconstrained critical points may not satisfy the constraint. The **method of Lagrange multipliers**, developed by Joseph-Louis Lagrange in the 18th century, converts a constrained optimization problem into an enlarged unconstrained system, providing both the optimal values and the rates at which the optimal value changes as the constraint is relaxed.

## The Setup

Consider optimizing $f(\mathbf{x})$ subject to the constraint $g(\mathbf{x}) = c$, where $f,g:\mathbb{R}^n\to\mathbb{R}$ are smooth. The constraint set $S = \{g = c\}$ is typically a smooth $(n-1)$-dimensional surface in $\mathbb{R}^n$ (a curve if $n=2$, a surface if $n=3$).

A constrained extremum on $S$ is a point $\mathbf{a}\in S$ where $f|_S$ (the restriction of $f$ to $S$) has a local extremum. At such a point, $f$ cannot increase or decrease along any direction tangent to $S$: the directional derivative of $f$ in every direction tangent to $S$ is zero.

## The Lagrange Condition

**Theorem (Lagrange Multipliers, one constraint).** Let $f,g:\mathbb{R}^n\to\mathbb{R}$ be $C^1$. Suppose $\mathbf{a}$ is a local extremum of $f$ on the constraint set $\{g = c\}$, and suppose $\nabla g(\mathbf{a})\neq\mathbf{0}$ (a **regularity condition**). Then there exists a scalar $\lambda\in\mathbb{R}$ (the **Lagrange multiplier**) such that

$$\nabla f(\mathbf{a}) = \lambda\,\nabla g(\mathbf{a}).$$

**Proof.** At $\mathbf{a}$, the constraint surface $\{g=c\}$ has tangent space $T = \ker(\nabla g(\mathbf{a})^T) = \{\mathbf{v}: \nabla g(\mathbf{a})\cdot\mathbf{v}=0\}$ (the subspace of directions perpendicular to $\nabla g$). For any curve $\boldsymbol{\gamma}(t)$ on $\{g=c\}$ with $\boldsymbol{\gamma}(0)=\mathbf{a}$, differentiating $f(\boldsymbol{\gamma}(t))$ and evaluating at $t=0$: if $f$ has a local extremum at $\mathbf{a}$ on $S$, then $\nabla f(\mathbf{a})\cdot\boldsymbol{\gamma}'(0)=0$. Since $\boldsymbol{\gamma}'(0)$ ranges over all tangent directions (all of $T$ when $\nabla g\neq\mathbf{0}$ by the implicit function theorem), $\nabla f(\mathbf{a})\perp T$. But the only vectors perpendicular to all of $T = (\nabla g)^\perp$ are the multiples of $\nabla g$. Hence $\nabla f(\mathbf{a}) = \lambda\nabla g(\mathbf{a})$ for some $\lambda$.

## The Lagrange System

The conditions $\nabla f(\mathbf{a}) = \lambda\nabla g(\mathbf{a})$ (giving $n$ equations in the $n+1$ unknowns $\mathbf{a}$ and $\lambda$) plus the constraint $g(\mathbf{a}) = c$ (one more equation) form the **Lagrange system** of $n+1$ equations in $n+1$ unknowns $(x_1,\ldots,x_n,\lambda)$:

$$\begin{cases} \nabla f(\mathbf{x}) = \lambda\,\nabla g(\mathbf{x}) \\ g(\mathbf{x}) = c \end{cases}$$

All constrained extrema satisfying the regularity condition are solutions. Note that not every solution is an extremum; one must compare the values of $f$ at all solutions to determine which are maxima, which are minima, and which are neither.

## Worked Example 1: Maximum on an Ellipse

Maximize $f(x,y) = x+2y$ subject to $g(x,y) = x^2+y^2 = 5$.

$\nabla f = (1,2)$, $\nabla g = (2x,2y)$.

Lagrange conditions: $(1,2) = \lambda(2x,2y)$, so $1=2\lambda x$ and $2 = 2\lambda y$. Thus $x = 1/(2\lambda)$ and $y = 1/\lambda$.

Constraint: $\frac{1}{4\lambda^2} + \frac{1}{\lambda^2} = 5 \Rightarrow \frac{5}{4\lambda^2} = 5 \Rightarrow \lambda^2 = 1/4 \Rightarrow \lambda = \pm 1/2$.

$\lambda = 1/2$: $x = 1, y = 2$. $f(1,2) = 1+4 = 5$.
$\lambda = -1/2$: $x=-1, y=-2$. $f(-1,-2) = -1-4 = -5$.

Maximum is $5$ at $(1,2)$; minimum is $-5$ at $(-1,-2)$.

## Worked Example 2: Closest Point on a Plane

Find the point on the plane $2x+y+3z=6$ closest to the origin.

Minimize $f(x,y,z) = x^2+y^2+z^2$ subject to $g(x,y,z) = 2x+y+3z = 6$.

$\nabla f = (2x,2y,2z)$, $\nabla g = (2,1,3)$.

$(2x,2y,2z) = \lambda(2,1,3)$: $x=\lambda$, $y=\lambda/2$, $z=3\lambda/2$.

Constraint: $2\lambda + \lambda/2 + 9\lambda/2 = 6 \Rightarrow \lambda(2+1/2+9/2) = 6 \Rightarrow 7\lambda = 6 \Rightarrow \lambda = 6/7$.

Closest point: $(6/7, 3/7, 9/7)$. Distance: $\sqrt{\lambda^2+\lambda^2/4+9\lambda^2/4} = \lambda\sqrt{14/4} = (6/7)\sqrt{14}/2 = 3\sqrt{14}/7$. Alternatively, the distance from the origin to the plane is $6/\sqrt{4+1+9} = 6/\sqrt{14}$, confirming the result.

## The Geometric Meaning of $\lambda$

The multiplier $\lambda$ has a concrete interpretation: it is the rate at which the optimal value changes when the constraint level $c$ is relaxed. If $f^*(c)$ is the maximum of $f$ subject to $g=c$, then $\frac{df^*}{dc} = \lambda$ at the solution. This is the "shadow price" in economics: the marginal value of relaxing the constraint.

## The Regularity Condition

The condition $\nabla g(\mathbf{a})\neq\mathbf{0}$ is needed; without it, the constraint surface may not be smooth at $\mathbf{a}$, and the argument breaks down. If $\nabla g(\mathbf{a})=\mathbf{0}$, then $\mathbf{a}$ is a critical point of $g$ and a singular point of the constraint set; it must be checked separately. For example, the constraint $g(x,y) = x^2+y^2 = 0$ is not a smooth curve (it is just the origin), and $\nabla g(0,0) = (0,0)$.

## Connection to the Implicit Function Theorem

The Lagrange multiplier theorem is a consequence of the implicit function theorem (Chapter 7). The implicit function theorem guarantees that near a regular point of the constraint $g = c$ (i.e., where $\nabla g\neq\mathbf{0}$), the constraint surface is locally a smooth manifold, and one can locally parameterize it and express $f$ as a function of $n-1$ free variables. The stationarity condition on that reduced function is equivalent to the Lagrange condition.
