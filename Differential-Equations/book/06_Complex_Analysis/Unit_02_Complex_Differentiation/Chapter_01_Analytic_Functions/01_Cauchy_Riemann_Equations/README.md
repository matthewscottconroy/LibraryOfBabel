# The Cauchy-Riemann Equations

The Cauchy-Riemann equations are the algebraic heart of complex analysis. They express precisely what it means for a function of a complex variable to be complex differentiable: the difference quotient $(f(z+h)-f(z))/h$ must approach the same limit regardless of the direction from which $h \to 0$. The requirement that limits along the real and imaginary axes agree imposes a pair of first-order PDE constraints on the real and imaginary parts of $f$. These constraints — simple in form but deep in consequence — are the gateway to the entire theory of analytic functions.

## The Complex Derivative

**Definition.** Let $f : D \to \mathbb{C}$ be defined on an open set $D$. The complex derivative of $f$ at $z_0 \in D$ is
$$f'(z_0) = \lim_{h \to 0} \frac{f(z_0 + h) - f(z_0)}{h}, \qquad h \in \mathbb{C} \setminus \{0\},$$
provided this limit exists. If the limit exists, $f$ is said to be complex differentiable at $z_0$.

The formal definition is identical to the real derivative. The crucial difference is that $h$ is complex: the denominator $h$ divides in $\mathbb{C}$, and the limit must be the same for all paths along which $h \to 0$.

## Derivation of the Cauchy-Riemann Equations

Write $f(z) = u(x,y) + iv(x,y)$ where $z = x + iy$. Suppose $f'(z_0)$ exists. We compute the difference quotient along two special paths:

**Path 1: $h = \Delta x$ (real).** Then $f(z_0 + h) = u(x_0 + \Delta x, y_0) + iv(x_0 + \Delta x, y_0)$, and
$$\frac{f(z_0 + h) - f(z_0)}{h} \to u_x(x_0, y_0) + iv_x(x_0, y_0) \quad \text{as } \Delta x \to 0.$$

**Path 2: $h = i\Delta y$ (imaginary).** Then $f(z_0 + h) = u(x_0, y_0 + \Delta y) + iv(x_0, y_0 + \Delta y)$, and
$$\frac{f(z_0 + h) - f(z_0)}{i\Delta y} \to \frac{u_y + iv_y}{i} = v_y - iu_y \quad \text{as } \Delta y \to 0.$$

Since both limits must equal $f'(z_0)$, we equate real and imaginary parts:
$$u_x = v_y \qquad \text{and} \qquad u_y = -v_x.$$

These are the **Cauchy-Riemann equations**. In vector form: if we think of $(u,v)$ as a vector field, the Jacobian $\begin{pmatrix} u_x & u_y \\ v_x & v_y \end{pmatrix}$ must equal $\begin{pmatrix} a & -b \\ b & a \end{pmatrix}$ for some real $a, b$, which is a scalar multiple of a rotation matrix. This is the geometric statement: multiplication by $f'(z_0) = a + ib$ is rotation by $\arg(f'(z_0))$ composed with scaling by $|f'(z_0)|$.

**Theorem (Necessary condition).** If $f'(z_0)$ exists, then the Cauchy-Riemann equations hold at $z_0$, and
$$f'(z_0) = u_x(z_0) + iv_x(z_0) = v_y(z_0) - iu_y(z_0).$$

## The Converse: Sufficient Conditions

The Cauchy-Riemann equations are necessary but not sufficient for differentiability: a function can satisfy them at a point without being complex differentiable there (if the partial derivatives are not continuous). The standard sufficient condition is:

**Theorem (Sufficient condition).** If the partial derivatives $u_x, u_y, v_x, v_y$ exist in a neighborhood of $z_0$, are continuous at $z_0$, and satisfy the Cauchy-Riemann equations at $z_0$, then $f'(z_0)$ exists.

**Proof sketch.** By the total differentiability of $u$ and $v$ (which follows from continuity of their partial derivatives):
$$u(x_0 + \Delta x, y_0 + \Delta y) = u(x_0, y_0) + u_x \Delta x + u_y \Delta y + o(|\Delta z|),$$
$$v(x_0 + \Delta x, y_0 + \Delta y) = v(x_0, y_0) + v_x \Delta x + v_y \Delta y + o(|\Delta z|).$$
Therefore:
$$\frac{f(z_0 + h) - f(z_0)}{h} = \frac{(u_x + iv_x)\Delta x + (u_y + iv_y)\Delta y + o(|h|)}{h}.$$
Using the Cauchy-Riemann equations to replace $u_y = -v_x$ and $v_y = u_x$:
$$= \frac{(u_x + iv_x)(\Delta x + i\Delta y) + o(|h|)}{h} = (u_x + iv_x) + \frac{o(|h|)}{h} \to u_x + iv_x. \quad \square$$

## Worked Examples

**Example 1.** Show that $f(z) = z^2$ is entire and compute $f'$.

Write $z = x + iy$. Then $z^2 = x^2 - y^2 + 2ixy$, so $u = x^2 - y^2$, $v = 2xy$.
$$u_x = 2x = v_y, \qquad u_y = -2y = -v_x = -(2y). \quad \checkmark$$
The Cauchy-Riemann equations hold everywhere, and all partial derivatives are continuous, so $f$ is entire. The derivative is $f'(z) = u_x + iv_x = 2x + 2iy = 2(x+iy) = 2z$.

**Example 2.** Show that $g(z) = \bar{z}$ is nowhere complex differentiable.

Here $u = x$, $v = -y$. The Cauchy-Riemann equations require $u_x = v_y$, but $u_x = 1$ and $v_y = -1$. They are never satisfied. So $\bar{z}$ has no complex derivative anywhere.

**Example 3.** Show that $h(z) = |z|^2$ is complex differentiable only at $z = 0$.

Here $u = x^2 + y^2$, $v = 0$. The equations require $u_x = v_y = 0$ and $u_y = -v_x = 0$, i.e., $2x = 0$ and $2y = 0$. These hold only at the origin. So $h$ is differentiable only at $z = 0$, with $h'(0) = 0$. But $h$ is not analytic at any point (analyticity requires differentiability in a neighborhood).

## Polar Form of the Cauchy-Riemann Equations

In polar coordinates $z = re^{i\theta}$, writing $f(re^{i\theta}) = U(r,\theta) + iV(r,\theta)$, the Cauchy-Riemann equations become:
$$U_r = \frac{1}{r} V_\theta, \qquad V_r = -\frac{1}{r} U_\theta.$$
And the derivative is:
$$f'(z) = e^{-i\theta}\left(U_r + iV_r\right) = \frac{e^{-i\theta}}{r}\left(-iU_\theta + V_\theta\right).$$

These are useful when $f$ is naturally expressed in polar form.

**Example.** Verify the Cauchy-Riemann equations in polar form for $f(z) = z^n$.

In polar form, $f = r^n e^{in\theta}$, so $U = r^n\cos(n\theta)$ and $V = r^n\sin(n\theta)$.
$$U_r = nr^{n-1}\cos(n\theta), \quad \frac{1}{r}V_\theta = \frac{n}{r}r^n\cos(n\theta) = nr^{n-1}\cos(n\theta). \quad \checkmark$$
$$V_r = nr^{n-1}\sin(n\theta), \quad -\frac{1}{r}U_\theta = \frac{n}{r}r^n\sin(n\theta) = nr^{n-1}\sin(n\theta). \quad \checkmark$$

## The Jacobian Interpretation

The Jacobian of $F = (u, v) : \mathbb{R}^2 \to \mathbb{R}^2$ at a point where the Cauchy-Riemann equations hold is:
$$JF = \begin{pmatrix} u_x & u_y \\ v_x & v_y \end{pmatrix} = \begin{pmatrix} u_x & -v_x \\ v_x & u_x \end{pmatrix}.$$
This matrix has determinant $u_x^2 + v_x^2 = |f'(z)|^2 \geq 0$. When $f'(z) \neq 0$, the Jacobian is invertible, and by the real inverse function theorem, $F$ is locally a diffeomorphism. Moreover, the Jacobian is a scalar multiple of an orthogonal matrix (rotation), so the map preserves angles between curves. This is the geometric meaning of conformality, which will be developed fully in Unit 04.

## Connection to Physics

The Cauchy-Riemann equations state that the vector field $(u, v)$ simultaneously satisfies:
$$\frac{\partial u}{\partial x} - \frac{\partial v}{\partial y} = 0 \quad \text{and} \quad \frac{\partial u}{\partial y} + \frac{\partial v}{\partial x} = 0,$$
which in vector calculus notation says $\nabla \cdot (u, -v) = 0$ (zero divergence) and $\nabla \times (u, v) = 0$ (zero curl). Thus the real and imaginary parts of an analytic function describe a two-dimensional, irrotational, incompressible fluid flow: $u$ is the velocity potential and $v$ is the stream function. This is one of the foundational facts connecting complex analysis to two-dimensional fluid mechanics and electrostatics.
