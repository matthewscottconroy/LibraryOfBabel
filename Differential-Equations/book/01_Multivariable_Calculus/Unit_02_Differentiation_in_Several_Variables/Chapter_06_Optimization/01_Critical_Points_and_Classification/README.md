# Critical Points and Classification

The fundamental strategy in optimization is: scan the domain for points where the function could have an extremum, then check which candidates are actually extrema and which are not. For differentiable functions, the first scan is decisive: at any interior local extremum, the gradient must vanish. This reduces the search to the **critical points** — the zeros of the gradient — plus any boundary points. The classification of critical points into local minima, local maxima, and saddle points then requires the Hessian (the next section), but understanding what critical points are, how to find them, and why they matter is the first task.

## Definition

A point $\mathbf{a}\in\text{int}(D)$ is a **critical point** of $f:D\subseteq\mathbb{R}^n\to\mathbb{R}$ if $\nabla f(\mathbf{a}) = \mathbf{0}$, i.e., all partial derivatives vanish at $\mathbf{a}$:

$$\frac{\partial f}{\partial x_1}(\mathbf{a}) = \frac{\partial f}{\partial x_2}(\mathbf{a}) = \cdots = \frac{\partial f}{\partial x_n}(\mathbf{a}) = 0.$$

A value $c = f(\mathbf{a})$ at a critical point is a **critical value**.

## Necessary Condition for Interior Extrema

**Theorem.** If $f$ has a local maximum or minimum at an interior point $\mathbf{a}$ of its domain, and $f$ is differentiable at $\mathbf{a}$, then $\mathbf{a}$ is a critical point.

**Proof.** Fix any direction $\mathbf{e}_i$ and consider $g(t) = f(\mathbf{a}+t\mathbf{e}_i)$. If $f$ has a local maximum at $\mathbf{a}$, then $g$ has a local maximum at $t=0$, so $g'(0) = \partial f/\partial x_i(\mathbf{a}) = 0$. Since this holds for each $i$, $\nabla f(\mathbf{a}) = \mathbf{0}$.

**Remark.** This is necessary but not sufficient. Not every critical point is a local extremum; saddle points are critical points that are neither maxima nor minima.

## Types of Critical Points

**Local minimum:** $f(\mathbf{x}) \geq f(\mathbf{a})$ for all $\mathbf{x}$ near $\mathbf{a}$. Near a local minimum, the graph of $f$ looks like the inside of a bowl.

**Local maximum:** $f(\mathbf{x}) \leq f(\mathbf{a})$ for all $\mathbf{x}$ near $\mathbf{a}$. The graph looks like an upside-down bowl.

**Saddle point:** $\mathbf{a}$ is a critical point that is neither a local minimum nor a local maximum. Near a saddle, the function increases in some directions and decreases in others. In $\mathbb{R}^2$, the classic example is $f(x,y) = x^2-y^2$: at the origin, $f_x = f_y = 0$, but $f(h,0) = h^2 > 0$ and $f(0,k) = -k^2 < 0$, so the origin is neither a min nor a max.

## Finding Critical Points

To find the critical points of $f$, solve the system $\nabla f = \mathbf{0}$, i.e., $\frac{\partial f}{\partial x_i} = 0$ for all $i = 1, \ldots, n$. This is a system of $n$ equations in $n$ unknowns; it may have 0, finitely many, or infinitely many solutions.

**Example 1.** $f(x,y) = x^3 - 3x + y^2 - 4y$.

$f_x = 3x^2-3 = 0 \Rightarrow x^2=1 \Rightarrow x=\pm 1$.

$f_y = 2y-4 = 0 \Rightarrow y=2$.

Critical points: $(1,2)$ and $(-1,2)$.

$f(1,2) = 1-3+4-8 = -6$.
$f(-1,2) = -1+3+4-8 = -2$.

The nature of these critical points (min, max, or saddle) requires the second derivative test.

**Example 2.** $f(x,y) = e^{-(x^2+y^2)}$.

$f_x = -2xe^{-(x^2+y^2)} = 0 \Rightarrow x=0$.

$f_y = -2ye^{-(x^2+y^2)} = 0 \Rightarrow y=0$.

Unique critical point: $(0,0)$. Since $f(0,0) = 1$ and $f(x,y) = e^{-(x^2+y^2)} \leq 1$ for all $(x,y)$, with equality only at the origin, this is a global maximum.

## Global vs. Local Extrema

A **global (absolute) maximum** is a point where $f(\mathbf{a}) \geq f(\mathbf{x})$ for all $\mathbf{x}$ in the domain. A **local maximum** only requires this in a neighborhood of $\mathbf{a}$. Every global extremum is a local extremum, but not conversely.

For continuous $f$ on a compact domain $K$ (closed and bounded), the extreme value theorem guarantees the existence of global extrema. To find them:
1. Find all critical points in the interior of $K$.
2. Find all extrema on the boundary of $K$ (which is a lower-dimensional optimization problem).
3. Compare all candidate values.

**Example.** Find the global extrema of $f(x,y) = 4xy - x^2 - y^2$ on the square $[0,2]\times[0,2]$.

$\nabla f = (4y-2x, 4x-2y) = (0,0) \Rightarrow 4y=2x$ and $4x=2y \Rightarrow x=y=0$.

Critical point $(0,0)$ is a corner of the square, not interior.

On the boundary: four edges.
- $x=0$: $f(0,y) = -y^2$, max at $y=0$, min at $y=2$: $f(0,2)=-4$.
- $x=2$: $f(2,y) = 8y-4-y^2 = -(y-4)^2+12$, max at $y=2$: $f(2,2)=12$. Min at $y=0$: $f(2,0)=-4$.
- Similarly by symmetry for $y=0$ and $y=2$.

Global maximum: $f(2,2) = 16-4-4=8$. Wait — $f(2,2) = 4(4) - 4 - 4 = 16-8 = 8$. Global minimum: $f(0,2) = f(2,0) = -4$.

## Degenerate Critical Points

When the Hessian at a critical point is singular (has a zero eigenvalue), the second derivative test is inconclusive. Such points are called **degenerate critical points**. Their nature depends on higher-order terms. For example, $f(x,y) = x^4+y^4$ has a degenerate minimum at the origin; $f(x,y) = x^3$ has a degenerate non-extremum at $x=0$ (it is a point of inflection in the $x$-direction, but the zero is due to the odd power).

## Connection to Differential Equations

In the theory of differential equations, the critical points of the function $f$ (not to be confused with equilibrium points of the ODE, though the two concepts are related) play a role in the study of gradient flows $\dot{\mathbf{x}} = -\nabla f(\mathbf{x})$. The stable equilibria of this ODE are precisely the local minima of $f$. More broadly, in Morse theory, the critical points of a function on a manifold encode topological information about the manifold itself.
