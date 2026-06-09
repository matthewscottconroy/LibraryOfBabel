# Method of Separation of Variables

Separation of variables is the oldest and most direct technique for solving first-order ODEs. Its power lies in reducing a differential equation in two variables to two independent single-variable integrals, each of which can be evaluated (or at least expressed as a definite integral) independently of the other.

## Separable Equations: Definition

A first-order ODE $y' = f(x, y)$ is **separable** if $f(x, y)$ can be written as a product of a function of $x$ alone and a function of $y$ alone:

$$\frac{dy}{dx} = g(x)\,h(y).$$

Not every equation is separable; the test is whether $f(x, y)$ factors in this way. The equation $y' = x^2 y$ separates as $g(x) = x^2$, $h(y) = y$. The equation $y' = x + y$ does not separate (it is linear but not separable). The equation $y' = (x^2 + 1)(y^2 + 1)$ separates with $g(x) = x^2 + 1$ and $h(y) = y^2 + 1$.

## The Algorithm

Given $dy/dx = g(x)h(y)$ with $h(y) \neq 0$:

1. Rewrite as $\frac{dy}{h(y)} = g(x)\,dx$.
2. Integrate both sides: $\int \frac{dy}{h(y)} = \int g(x)\,dx + C$.
3. Solve for $y$ if possible (explicit solution), or leave in implicit form.
4. Check separately for constant solutions where $h(y) = 0$.

**Justification.** The manipulation is not just formal. If $y = \phi(x)$ is a solution, then $\phi'(x) = g(x)h(\phi(x))$, so dividing by $h(\phi(x))$ gives $\phi'(x)/h(\phi(x)) = g(x)$. Integrating both sides from $x_0$ to $x$:

$$\int_{x_0}^x \frac{\phi'(t)}{h(\phi(t))}\,dt = \int_{x_0}^x g(t)\,dt.$$

The left side, by the substitution $u = \phi(t)$, becomes $\int_{y_0}^{y} du/h(u)$, where $y_0 = \phi(x_0)$ and $y = \phi(x)$. So the separation step is exactly the change of variables $u = \phi(t)$ in the integral.

## Worked Example 1: Explicit Solution

Solve $dy/dx = xy$, $y(0) = 2$.

Separating: $dy/y = x\,dx$. Integrating: $\ln|y| = x^2/2 + C_1$. Exponentiating: $|y| = e^{C_1}e^{x^2/2}$, so $y = Ce^{x^2/2}$ where $C = \pm e^{C_1}$ is an arbitrary nonzero constant (and $y = 0$ is the constant solution corresponding to $C = 0$). Imposing $y(0) = 2$: $C = 2$. Solution: $y = 2e^{x^2/2}$, defined on $(-\infty, \infty)$.

## Worked Example 2: Implicit Solution

Solve $dy/dx = (1 + y^2)/(1 + x^2)$.

Separating: $dy/(1 + y^2) = dx/(1 + x^2)$. Integrating: $\arctan(y) = \arctan(x) + C$. Taking tangent of both sides:

$$y = \tan(\arctan(x) + C).$$

Using the addition formula $\tan(\alpha + \beta) = (\tan\alpha + \tan\beta)/(1 - \tan\alpha\tan\beta)$:

$$y = \frac{x + \tan C}{1 - x\tan C}.$$

Writing $A = \tan C$ (which ranges over all real numbers as $C$ varies):

$$y = \frac{x + A}{1 - Ax}.$$

This is the general solution, which can be verified by differentiation. Each member of the family is a Mobius transformation of $x$.

## Worked Example 3: Constant Solutions Must Be Checked Separately

Solve $dy/dx = y^2 - y$.

The right side factors as $h(y) = y^2 - y = y(y-1)$, so the equation is autonomous and separable.

Constant solutions: $h(y^*) = 0$ gives $y^* = 0$ and $y^* = 1$.

For non-constant solutions, separate: $dy/[y(y-1)] = dx$. Partial fractions: $1/(y(y-1)) = -1/y + 1/(y-1)$. So

$$\int \left(\frac{-1}{y} + \frac{1}{y-1}\right)dy = \int dx.$$

$-\ln|y| + \ln|y-1| = x + C$. Combining: $\ln|{(y-1)/y}| = x + C$, so $(y-1)/y = Ae^x$ where $A = \pm e^C$. Solving:

$$1 - 1/y = Ae^x \implies 1/y = 1 - Ae^x \implies y = \frac{1}{1 - Ae^x}.$$

For $A = 0$: $y = 1$, recovering the equilibrium $y^* = 1$ (which is thus included in the formula). For the equilibrium $y^* = 0$: setting $y = 1/(1 - Ae^x) = 0$ has no solution for any finite $A$, so $y = 0$ is a genuinely separate singular-looking solution not captured by the family with $A \neq 0$. It corresponds formally to $A \to \infty$.

The correct general solution is: $y = 0$, $y = 1$, and $y = 1/(1 - Ae^x)$ for $A \neq 0$.

## Domains and Implicit Functions

The implicit equation $F(x, y) = C$ obtained by separating variables defines $y$ as a function of $x$ only when the implicit function theorem applies: when $\partial F/\partial y \neq 0$. At points where $\partial F/\partial y = 0$, the implicit equation may define a curve with a vertical tangent, and the solution breaks down or transitions to another branch.

This issue appears in the equation $dy/dx = y^{1/2}$. Separating: $y^{-1/2}\,dy = dx$, so $2y^{1/2} = x + C$, giving $y = (x+C)^2/4$. But $y = 0$ is also a solution (since $f(0) = 0^{1/2} = 0$). The general formula gives $y = 0$ only when $x = -C$, a single point, not on the whole line. So $y = 0$ is a singular solution, and more generally, solutions can be patched: one can set $y = 0$ for $x \leq c$ and $y = (x - c)^2/4$ for $x > c$, for any $c$. These are all valid solutions of the IVP $y(x_0) = 0$ for $x_0 \leq c$, showing uniqueness fails at $y = 0$ (consistent with the non-Lipschitz behavior of $f(y) = y^{1/2}$ at $y = 0$).

## When Separation Fails

The equation $y' = x + y$ is first-order linear but not separable; no factoring $g(x)h(y)$ is possible. The equation $y' = \sin(xy)$ is not separable because $\sin(xy)$ cannot be factored into a product of a function of $x$ alone and a function of $y$ alone. For such equations, other methods (integrating factors, series, numerics) are required.
