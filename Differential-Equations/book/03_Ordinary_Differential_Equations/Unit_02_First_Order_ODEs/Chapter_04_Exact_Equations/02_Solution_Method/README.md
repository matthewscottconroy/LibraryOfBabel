# Solution Method for Exact Equations

Once an equation $M\,dx + N\,dy = 0$ is confirmed exact, finding the solution reduces to finding the potential function $F(x,y)$ such that $F_x = M$ and $F_y = N$. The general solution is then $F(x, y) = C$.

## The Algorithm

**Step 1.** Integrate $M$ with respect to $x$, treating $y$ as a parameter:

$$F(x, y) = \int M(x, y)\,dx + g(y),$$

where $g(y)$ is an arbitrary function of $y$ (the "constant" of integration, which may depend on $y$).

**Step 2.** Differentiate with respect to $y$ and set equal to $N$:

$$\frac{\partial F}{\partial y} = \frac{\partial}{\partial y}\int M\,dx + g'(y) = N(x, y).$$

This gives an equation for $g'(y)$:

$$g'(y) = N(x, y) - \frac{\partial}{\partial y}\int M(x, y)\,dx.$$

**Step 3.** The right side must depend on $y$ only (otherwise the equation is not exact). Integrate to find $g(y)$.

**Step 4.** The general solution is $F(x, y) = C$, or equivalently $\int M\,dx + g(y) = C$.

## Worked Example 1

Solve $(2xy + 3)\,dx + (x^2 + 4y)\,dy = 0$.

We verified exactness: $M_y = 2x = N_x$. Integrate $M$ w.r.t. $x$:

$$F = \int (2xy + 3)\,dx = x^2 y + 3x + g(y).$$

Differentiate w.r.t. $y$: $F_y = x^2 + g'(y)$. Set equal to $N = x^2 + 4y$:

$$x^2 + g'(y) = x^2 + 4y \implies g'(y) = 4y \implies g(y) = 2y^2.$$

General solution: $x^2 y + 3x + 2y^2 = C$.

**Verification.** Implicit differentiation of $x^2 y + 3x + 2y^2 = C$ gives $(2xy + 3)dx + (x^2 + 4y)dy = 0$. Correct.

## Worked Example 2

Solve $(e^x \sin y + 2x)\,dx + (e^x \cos y + 2y)\,dy = 0$.

Check: $M_y = e^x\cos y$ and $N_x = e^x\cos y$. Exact. Integrate $M$ w.r.t. $x$:

$$F = e^x\sin y + x^2 + g(y).$$

Then $F_y = e^x\cos y + g'(y) = N = e^x\cos y + 2y$, so $g'(y) = 2y$, $g(y) = y^2$. General solution:

$$e^x\sin y + x^2 + y^2 = C.$$

## Alternative: Integrating from Both Variables

One can also integrate $N$ w.r.t. $y$ first and then determine the function of $x$ by differentiating and comparing with $M$. Both approaches give the same result; the choice between them is made for computational convenience.

## IVP Solution

To solve the IVP $M\,dx + N\,dy = 0$, $y(x_0) = y_0$, first find $F$ by the algorithm above, then determine $C$ from $F(x_0, y_0) = C$. The solution is the implicitly defined curve $F(x, y) = F(x_0, y_0)$ passing through $(x_0, y_0)$.

**Example.** For the solution $x^2 y + 3x + 2y^2 = C$ with $y(1) = 2$: $C = 1\cdot 2 + 3\cdot 1 + 2\cdot 4 = 2 + 3 + 8 = 13$. The particular solution is $x^2 y + 3x + 2y^2 = 13$.

## Explicit Form and Singularities

The implicit solution $F(x,y) = C$ can be solved for $y$ explicitly near a point $(x_0, y_0)$ when $F_y(x_0, y_0) = N(x_0, y_0) \neq 0$ (by the implicit function theorem). When $N = 0$ at a point on the solution curve, the curve has a vertical tangent there, and the explicit form $y(x)$ may break down.

This is not a failure of the method; it is a geometric feature of the solution. The implicit form $F(x,y) = C$ remains valid on the entire solution curve; it is the explicit representation $y = y(x)$ that breaks down at vertical tangents.
