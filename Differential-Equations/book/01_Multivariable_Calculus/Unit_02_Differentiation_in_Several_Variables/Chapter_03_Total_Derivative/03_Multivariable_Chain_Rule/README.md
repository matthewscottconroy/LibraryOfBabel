# Multivariable Chain Rule

The single-variable chain rule states that if $h(t) = g(f(t))$, then $h'(t) = g'(f(t))\cdot f'(t)$: the derivative of a composition is the product of the outer derivative and the inner derivative. In several variables, the same principle applies — the derivative of a composition is the composition of the derivatives — but "product of derivatives" becomes "matrix product of Jacobians." The multivariable chain rule is one of the most frequently used tools in the subject, appearing whenever quantities depend on other quantities that themselves depend on yet other quantities.

## Statement of the Chain Rule

**Theorem.** Let $f: D\subseteq\mathbb{R}^n\to\mathbb{R}^m$ be differentiable at $\mathbf{a}\in\text{int}(D)$, and let $g: E\subseteq\mathbb{R}^m\to\mathbb{R}^k$ be differentiable at $f(\mathbf{a})\in\text{int}(E)$. Then $h = g\circ f: D\to\mathbb{R}^k$ is differentiable at $\mathbf{a}$, and

$$Dh(\mathbf{a}) = Dg(f(\mathbf{a}))\circ Df(\mathbf{a}).$$

In terms of Jacobian matrices:

$$J_h(\mathbf{a}) = J_g(f(\mathbf{a}))\cdot J_f(\mathbf{a}),$$

where $\cdot$ denotes matrix multiplication. The dimensions: $J_f$ is $m\times n$, $J_g$ is $k\times m$, so $J_h$ is $k\times n$, consistent with $h:\mathbb{R}^n\to\mathbb{R}^k$.

**Proof sketch.** Write $f(\mathbf{a}+\mathbf{h}) = f(\mathbf{a}) + Df(\mathbf{a})\mathbf{h} + \mathbf{r}(\mathbf{h})$ with $\mathbf{r} = o(\|\mathbf{h}\|)$. Let $\mathbf{k} = Df(\mathbf{a})\mathbf{h}+\mathbf{r}(\mathbf{h})$, so $f(\mathbf{a}+\mathbf{h}) = f(\mathbf{a}) + \mathbf{k}$. Then

$g(f(\mathbf{a}+\mathbf{h})) = g(f(\mathbf{a}) + \mathbf{k}) = g(f(\mathbf{a})) + Dg(f(\mathbf{a}))\mathbf{k} + \mathbf{s}(\mathbf{k})$

where $\mathbf{s} = o(\|\mathbf{k}\|)$. Substituting $\mathbf{k}$:

$h(\mathbf{a}+\mathbf{h}) = h(\mathbf{a}) + Dg(f(\mathbf{a}))[Df(\mathbf{a})\mathbf{h}+\mathbf{r}(\mathbf{h})] + \mathbf{s}(\mathbf{k})$
$= h(\mathbf{a}) + [Dg(f(\mathbf{a}))\circ Df(\mathbf{a})]\mathbf{h} + \underbrace{Dg(f(\mathbf{a}))\mathbf{r}(\mathbf{h}) + \mathbf{s}(\mathbf{k})}_{\text{remainder}}$.

One verifies the remainder is $o(\|\mathbf{h}\|)$ using that $Dg$ is a bounded linear map and $\|\mathbf{k}\| = O(\|\mathbf{h}\|)$.

## Component Form

For the common case $h = g\circ f$ with $f:\mathbb{R}^n\to\mathbb{R}^m$ and $g:\mathbb{R}^m\to\mathbb{R}^k$, the chain rule in component form is:

$$\frac{\partial h_i}{\partial x_j} = \sum_{l=1}^m \frac{\partial g_i}{\partial y_l}\cdot\frac{\partial f_l}{\partial x_j},$$

the $(i,j)$ entry of the matrix product $J_g\cdot J_f$.

**Important special case:** $f:\mathbb{R}^n\to\mathbb{R}^m$ and $g:\mathbb{R}^m\to\mathbb{R}$ (scalar). Then $\nabla h(\mathbf{a}) = J_f(\mathbf{a})^T\cdot\nabla g(f(\mathbf{a}))$. In components: $\frac{\partial h}{\partial x_j} = \sum_l \frac{\partial g}{\partial y_l}\frac{\partial f_l}{\partial x_j}$.

## Worked Examples

**Example 1: Function of one variable composed with a function of two variables.**

Let $g: \mathbb{R}^2\to\mathbb{R}$, $g(x,y) = x^2+y^2$. Let $f:\mathbb{R}\to\mathbb{R}^2$, $f(t) = (\cos t, \sin t)$. Then $h(t) = g(f(t)) = \cos^2 t + \sin^2 t = 1$.

Chain rule: $h'(t) = \nabla g(f(t))\cdot f'(t) = (2\cos t, 2\sin t)\cdot(-\sin t, \cos t) = -2\cos t\sin t + 2\sin t\cos t = 0$.

Consistent with $h(t) = 1$ (constant).

**Example 2: Two-variable composition.**

Let $z = f(x,y) = e^{x+y^2}$, and let $x = u^2v$, $y = u\sin v$ (so $z$ is a function of $u$ and $v$ via composition).

$\frac{\partial z}{\partial u} = \frac{\partial z}{\partial x}\frac{\partial x}{\partial u} + \frac{\partial z}{\partial y}\frac{\partial y}{\partial u} = e^{x+y^2}\cdot 2uv + e^{x+y^2}\cdot 2y\cdot\sin v = e^{x+y^2}(2uv + 2y\sin v)$.

At $(u,v) = (1,0)$: $x = 0$, $y = 0$, $z = e^0 = 1$, and $\partial z/\partial u = 1\cdot(0+0) = 0$.

$\frac{\partial z}{\partial v} = e^{x+y^2}(u^2 + 2y\cdot u\cos v)$. At $(1,0)$: $\partial z/\partial v = 1\cdot(1 + 0) = 1$.

**Example 3: Tree diagram interpretation.**

For $z = f(x,y)$ with $x = x(s,t)$ and $y = y(s,t)$:

$$\frac{\partial z}{\partial s} = \frac{\partial z}{\partial x}\frac{\partial x}{\partial s} + \frac{\partial z}{\partial y}\frac{\partial y}{\partial s}, \qquad \frac{\partial z}{\partial t} = \frac{\partial z}{\partial x}\frac{\partial x}{\partial t} + \frac{\partial z}{\partial y}\frac{\partial y}{\partial t}.$$

The "tree diagram" mnemonic: draw $z$ at the top; branch to $x$ and $y$; from $x$, branch to $s$ and $t$; similarly from $y$. The partial derivative of $z$ with respect to $s$ is the sum of products along all paths from $z$ to $s$.

## Implicit Differentiation via the Chain Rule

Suppose $F(x, y) = 0$ defines $y$ as a function of $x$ near a point. Differentiating both sides with respect to $x$ using the chain rule:

$$\frac{\partial F}{\partial x} + \frac{\partial F}{\partial y}\frac{dy}{dx} = 0, \quad\text{so}\quad \frac{dy}{dx} = -\frac{F_x}{F_y},$$

provided $F_y \neq 0$. This is the one-variable version of the implicit function theorem; the full generalization (Chapter 7) uses the Jacobian.

## Connection to Differential Equations

The chain rule is the foundation for the method of characteristics in first-order PDEs, where one converts a PDE for $u(x,t)$ into an ODE along special curves (the characteristics). It also underlies the substitution $u = u(r,\theta)$ in converting PDEs from Cartesian to polar coordinates: computing $u_{xx}+u_{yy}$ in polar form requires applying the chain rule to $u_x$ and $u_y$ to express them in terms of $u_r$ and $u_\theta$.
