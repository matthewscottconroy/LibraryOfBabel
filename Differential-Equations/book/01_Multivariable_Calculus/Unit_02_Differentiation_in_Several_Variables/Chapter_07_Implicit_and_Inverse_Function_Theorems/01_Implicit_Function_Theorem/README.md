# Implicit Function Theorem

An equation like $F(x,y) = x^2 + y^2 - 1 = 0$ defines a circle in the plane. Near the point $(0,1)$, the upper semicircle can be written explicitly as $y = \sqrt{1-x^2}$: the equation implicitly defines $y$ as a function of $x$ locally. Near the point $(1,0)$ on the same circle, the equation cannot be solved for $y$ as a function of $x$ (since $dy/dx = -x/y$ has a singularity at $y=0$), but it can be solved for $x$ as a function of $y$. The **implicit function theorem** gives the precise conditions under which an implicit equation $F(\mathbf{x},\mathbf{y})=\mathbf{0}$ can be solved for $\mathbf{y}$ as a smooth function of $\mathbf{x}$, and provides the derivative of the resulting function without requiring the function to be found explicitly.

## Statement

**Theorem (Implicit Function Theorem).** Let $F: D\subseteq\mathbb{R}^{n+m}\to\mathbb{R}^m$ be $C^1$ on an open set $D$, and suppose $F(\mathbf{x}_0, \mathbf{y}_0) = \mathbf{0}$ for some $(\mathbf{x}_0,\mathbf{y}_0)\in D$ (here $\mathbf{x}\in\mathbb{R}^n$ and $\mathbf{y}\in\mathbb{R}^m$). Suppose further that the $m\times m$ **partial Jacobian with respect to $\mathbf{y}$**,

$$\frac{\partial(F_1,\ldots,F_m)}{\partial(y_1,\ldots,y_m)}\bigg|_{(\mathbf{x}_0,\mathbf{y}_0)},$$

is invertible (has nonzero determinant).

Then there exist open neighborhoods $U\ni\mathbf{x}_0$ in $\mathbb{R}^n$ and $V\ni\mathbf{y}_0$ in $\mathbb{R}^m$ and a unique $C^1$ map $\boldsymbol{\phi}: U\to V$ such that:
1. $\boldsymbol{\phi}(\mathbf{x}_0) = \mathbf{y}_0$.
2. $F(\mathbf{x}, \boldsymbol{\phi}(\mathbf{x})) = \mathbf{0}$ for all $\mathbf{x}\in U$.
3. $\{\mathbf{x}\in U, \mathbf{y}\in V: F(\mathbf{x},\mathbf{y})=\mathbf{0}\} = \text{graph of }\boldsymbol{\phi}$.

Moreover, the **derivative of the implicit function** is given by:

$$D\boldsymbol{\phi}(\mathbf{x}) = -\left[\frac{\partial F}{\partial\mathbf{y}}\right]^{-1}\frac{\partial F}{\partial\mathbf{x}}.$$

## The Derivative Formula

The formula $D\boldsymbol{\phi} = -(\partial F/\partial\mathbf{y})^{-1}(\partial F/\partial\mathbf{x})$ follows by differentiating the identity $F(\mathbf{x},\boldsymbol{\phi}(\mathbf{x})) = \mathbf{0}$ with respect to $\mathbf{x}$ using the chain rule:

$$\frac{\partial F}{\partial\mathbf{x}} + \frac{\partial F}{\partial\mathbf{y}}\cdot D\boldsymbol{\phi} = 0 \implies D\boldsymbol{\phi} = -\left(\frac{\partial F}{\partial\mathbf{y}}\right)^{-1}\frac{\partial F}{\partial\mathbf{x}}.$$

For a single equation $F(x,y)=0$ in two variables:

$$\frac{dy}{dx} = -\frac{F_x}{F_y},$$

provided $F_y\neq 0$.

## Worked Example 1: Circle

$F(x,y) = x^2+y^2-1 = 0$. $F_x = 2x$, $F_y = 2y$.

The theorem applies wherever $F_y = 2y\neq 0$, i.e., away from $y=0$ (the points $(\pm 1, 0)$). Near any such point, $y$ is locally a $C^1$ function of $x$, and $dy/dx = -F_x/F_y = -x/y$.

At $(0,1)$: $dy/dx = 0/2 = 0$ (horizontal tangent at the top of the circle, as expected).

At $(1/\sqrt{2}, 1/\sqrt{2})$: $dy/dx = -1$.

## Worked Example 2: System

$F_1(x,y,z) = x^2+y^2-z = 0$ and $F_2(x,y,z) = x+y+z-1 = 0$. Near the point $(0,1,1)$ (check: $0+1-1=0$ and $0+1+1-1=1$, wrong — try $(0,1,1)$: $F_1 = 0+1-1=0$, $F_2=0+1+1-1=1\neq 0$). Use $(0,0,0)$: $F_1=0$, $F_2=0+0+0-1\neq 0$.

Try $(1,0,1)$: $F_1=1+0-1=0$, $F_2=1+0+1-1=1\neq 0$. Try solving the system: from $F_1$: $z=x^2+y^2$; from $F_2$: $x+y+x^2+y^2=1$. At $(0,0)$ in $x,y$: $0+0=0\neq 1$.

Cleaner example: $F_1(x,y,u,v) = u+v-x = 0$ and $F_2 = u-v-y=0$. Solve for $(u,v)$ in terms of $(x,y)$.

$\partial(F_1,F_2)/\partial(u,v) = \begin{pmatrix}1&1\\1&-1\end{pmatrix}$, determinant $-2\neq 0$. So $u = (x+y)/2$, $v=(x-y)/2$ everywhere. Derivative: $D\boldsymbol{\phi} = -\begin{pmatrix}1&1\\1&-1\end{pmatrix}^{-1}\begin{pmatrix}-1&0\\0&-1\end{pmatrix} = \frac{1}{2}\begin{pmatrix}1&-1\\1&1\end{pmatrix}\begin{pmatrix}1&0\\0&1\end{pmatrix}=\begin{pmatrix}1/2&-1/2\\1/2&1/2\end{pmatrix}$.

Check directly: $\partial u/\partial x = 1/2$, $\partial u/\partial y = 1/2$, $\partial v/\partial x = 1/2$, $\partial v/\partial y = -1/2$. Confirmed.

## Proof Sketch

The proof uses the **Banach fixed-point theorem** (contraction mapping principle). One shows that the map $T(\mathbf{y}) = \mathbf{y} - \left[\frac{\partial F}{\partial\mathbf{y}}(\mathbf{x}_0,\mathbf{y}_0)\right]^{-1}F(\mathbf{x},\mathbf{y})$ is a contraction on a small ball around $\mathbf{y}_0$ for $\mathbf{x}$ near $\mathbf{x}_0$, and its unique fixed point is $\boldsymbol{\phi}(\mathbf{x})$.

## Role in Justifying Lagrange Multipliers

The Lagrange multiplier theorem (Chapter 6) implicitly uses the implicit function theorem. The constraint $g(\mathbf{x})=c$ locally defines a manifold near a regular point. Near any such point, one can locally parameterize the constraint surface by $n-1$ free variables, express $f$ in terms of those variables, and set its gradient (in the reduced variables) to zero. This reduced stationarity condition is equivalent to the Lagrange condition $\nabla f = \lambda\nabla g$, with $\lambda$ determined by the implicit function theorem's derivative formula.

## Connection to Differential Equations

The implicit function theorem is the multivariable version of the Picard-Lindelöf existence and uniqueness theorem for ODEs. Consider the ODE $\dot{y} = F(t,y)$ with $y(t_0)=y_0$: the implicit function theorem (in a more general functional form) guarantees that the equation $y'(t) = F(t,y(t))$, viewed as a constraint on the space of smooth functions, has a locally unique solution.
