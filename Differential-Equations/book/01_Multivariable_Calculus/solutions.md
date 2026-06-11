# Solutions: Multivariable Calculus

## Problem 1: Partial Derivatives and the Chain Rule

**Problem.** Let $f(x,y) = x^2e^{xy}$. Compute $\partial f/\partial x$, $\partial f/\partial y$, and all second-order partial derivatives. Verify Clairaut's theorem.

**Solution.**
$f_x = 2xe^{xy} + x^2 y e^{xy} = xe^{xy}(2 + xy)$.
$f_y = x^3 e^{xy}$.

Second-order:
$f_{xx} = e^{xy}(2+xy) + xe^{xy}(y + y) = e^{xy}(2 + xy) + xye^{xy}(2+xy)/\ldots$

More carefully: $f_x = xe^{xy}(2+xy)$. Differentiate with respect to $x$:
$f_{xx} = e^{xy}(2+xy) + x[ye^{xy}(2+xy) + e^{xy}\cdot y] = e^{xy}[(2+xy) + xy(2+xy) + xy] = e^{xy}[2+xy+2xy+x^2y^2+xy] = e^{xy}[2+4xy+x^2y^2]$.

$f_{xy} = \frac{\partial}{\partial y}[xe^{xy}(2+xy)] = x[xe^{xy}(2+xy) + e^{xy}\cdot x] = x^2e^{xy}(2+xy) + x^2e^{xy} = x^2e^{xy}(3+xy)$.

$f_{yy} = \frac{\partial}{\partial y}[x^3e^{xy}] = x^4e^{xy}$.

$f_{yx} = \frac{\partial}{\partial x}[x^3e^{xy}] = 3x^2e^{xy} + x^3ye^{xy} = x^2e^{xy}(3+xy)$.

Clairaut verification: $f_{xy} = x^2e^{xy}(3+xy) = f_{yx}$. Confirmed.

---

## Problem 2: Gradient, Level Sets, and Directional Derivatives

**Problem.** For $f(x,y,z) = x^2 + y^2 - z$, find the gradient at $(1,1,2)$, the unit normal to the level surface through that point, and the directional derivative in the direction $\mathbf{v} = (1,1,1)/\sqrt{3}$.

**Solution.**
$\nabla f = (2x, 2y, -1)$. At $(1,1,2)$: $\nabla f = (2,2,-1)$.

Level surface: $x^2 + y^2 - z = 0$ (paraboloid $z = x^2+y^2$, passing through $(1,1,2)$ since $1+1 = 2$).

Unit normal: $\hat{n} = (2,2,-1)/\|(2,2,-1)\| = (2,2,-1)/3$.

Directional derivative in direction $\hat{\mathbf{v}} = (1,1,1)/\sqrt{3}$:
$D_{\hat{\mathbf{v}}}f = \nabla f \cdot \hat{\mathbf{v}} = (2,2,-1)\cdot(1,1,1)/\sqrt{3} = (2+2-1)/\sqrt{3} = 3/\sqrt{3} = \sqrt{3}$.

The gradient points in the direction of steepest ascent; $D_{\hat{\mathbf{v}}}f = \sqrt{3}$ gives the rate of increase in the direction $\hat{\mathbf{v}}$.

---

## Problem 3: Critical Points and Classification

**Problem.** Find and classify all critical points of $f(x,y) = x^3 - 3xy^2$.

**Solution.**
$f_x = 3x^2 - 3y^2 = 0 \Rightarrow x^2 = y^2 \Rightarrow y = \pm x$.
$f_y = -6xy = 0 \Rightarrow x = 0$ or $y = 0$.

Case $y = x$: $-6xy = -6x^2 = 0 \Rightarrow x = 0$. Critical point $(0,0)$.
Case $y = -x$: $-6xy = 6x^2 = 0 \Rightarrow x = 0$. Critical point $(0,0)$ again.

Only critical point: $(0,0)$.

Hessian at $(0,0)$: $f_{xx} = 6x = 0$, $f_{xy} = -6y = 0$, $f_{yy} = -6x = 0$. So $H = \begin{pmatrix}0&0\\0&0\end{pmatrix}$, $\det H = 0$. Test inconclusive.

Note $f(x,y) = x(x^2 - 3y^2)$. On the $x$-axis ($y=0$): $f = x^3$, which changes sign through $(0,0)$. The origin is a saddle point (neither local max nor min), but the Hessian test fails.

This function is the real part of $z^3$ (where $z = x + iy$) — it is a harmonic polynomial with a monkey saddle at the origin.

---

## Problem 4: Lagrange Multipliers

**Problem.** Find the maximum and minimum distances from the origin to the ellipse $4x^2 + y^2 = 4$.

**Solution.** Minimize/maximize $f(x,y) = x^2 + y^2$ subject to $g(x,y) = 4x^2 + y^2 - 4 = 0$.

Lagrange: $\nabla f = \lambda\nabla g$:
$(2x, 2y) = \lambda(8x, 2y)$.

From $2x = 8\lambda x$: either $x = 0$ or $\lambda = 1/4$.
From $2y = 2\lambda y$: either $y = 0$ or $\lambda = 1$.

If $\lambda = 1/4$: $2y = 2(1/4)y = y/2 \Rightarrow y = 0$. Constraint: $4x^2 = 4$, $x = \pm 1$. Points $(\pm 1, 0)$, $f = 1$.

If $\lambda = 1$: $2x = 8x \Rightarrow x = 0$. Constraint: $y^2 = 4$, $y = \pm 2$. Points $(0, \pm 2)$, $f = 4$.

Minimum distance: $\sqrt{1} = 1$ at $(\pm 1, 0)$. Maximum distance: $\sqrt{4} = 2$ at $(0, \pm 2)$.

**Geometric check.** The ellipse $4x^2 + y^2 = 4$ has semi-axes $a = 1$ (in $x$) and $b = 2$ (in $y$), confirming nearest and farthest points.

---

## Problem 5: Change of Variables in a Double Integral

**Problem.** Evaluate $\iint_D e^{(x-y)/(x+y)}\,dA$ where $D$ is the triangle with vertices $(0,0)$, $(1,0)$, $(0,1)$.

**Solution.** Let $u = x - y$, $v = x + y$. Then $x = (u+v)/2$, $y = (v-u)/2$.

Jacobian: $\partial(x,y)/\partial(u,v) = \begin{vmatrix}1/2&1/2\\-1/2&1/2\end{vmatrix} = 1/4 - (-1/4) = 1/2$. So $dA = |J|\,du\,dv = \frac{1}{2}\,du\,dv$.

The triangle $D$: $x \geq 0$, $y \geq 0$, $x+y \leq 1$ transforms to: $x = (u+v)/2 \geq 0 \Rightarrow u \geq -v$; $y = (v-u)/2 \geq 0 \Rightarrow u \leq v$; $x+y = v \leq 1$; $x+y = v \geq 0$.

New region: $0 \leq v \leq 1$, $-v \leq u \leq v$.

$\iint_D e^{u/v}\,dA = \int_0^1\int_{-v}^v e^{u/v}\frac{1}{2}\,du\,dv = \frac{1}{2}\int_0^1\left[ve^{u/v}\right]_{u=-v}^{u=v}\,dv = \frac{1}{2}\int_0^1 v(e^1-e^{-1})\,dv = \frac{e - e^{-1}}{2}\cdot\frac{1}{2} = \frac{e-1/e}{4}$.

---

## Problem 6: Implicit Function Theorem

**Problem.** Near the point $(x_0, y_0) = (1, 1)$, the equation $F(x,y) = x^3 + y^3 - 2xy = 0$ defines $y$ as a function of $x$. Find $dy/dx$ at $(1,1)$.

**Solution.** By IFT: $dy/dx = -F_x/F_y$.

$F_x = 3x^2 - 2y$, $F_y = 3y^2 - 2x$.

At $(1,1)$: $F_x = 3-2 = 1$, $F_y = 3-2 = 1$. $F_y \neq 0$, so IFT applies.

$dy/dx|_{(1,1)} = -F_x/F_y = -1/1 = -1$.

**Verification.** Differentiating $F(x,y(x)) = 0$ implicitly: $3x^2 + 3y^2 y' - 2y - 2xy' = 0$, so $y'(3y^2-2x) = 2y-3x^2$, $y' = (2y-3x^2)/(3y^2-2x)$. At $(1,1)$: $y' = (2-3)/(3-2) = -1$. Agrees.
