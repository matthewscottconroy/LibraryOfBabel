# Phase Lines and Equilibria

For autonomous first-order equations $y' = f(y)$, the right-hand side depends only on $y$, not on the independent variable $x$. This independence means that the entire dynamics of the equation can be captured on a single number line, the **phase line**, making qualitative analysis especially clean and powerful.

## Autonomous Equations and Translation Invariance

An equation is **autonomous** when $f(x, y) = f(y)$, so that the slope at any point depends only on the height $y$ and not on the horizontal position $x$. Equivalently, the equation is unchanged by replacing $x$ with $x + c$ for any constant $c$. As a consequence, if $\phi(x)$ is a solution, so is $\phi(x - c)$: solutions can be translated horizontally without disturbing the equation.

This symmetry makes the two-dimensional direction field redundant. All the essential information is captured by the one-dimensional graph of $f(y)$ versus $y$.

## Equilibrium Points

An **equilibrium** (or **equilibrium point**, or **critical point**) of $y' = f(y)$ is a value $y^*$ such that $f(y^*) = 0$. The constant function $y(x) = y^*$ is then a solution, since $y'(x) = 0 = f(y^*)$. These constant solutions are the equilibria, also called **stationary states** or **fixed points**.

**Example.** For $y' = y(y-1)(y+2)$, the equilibria are $y^* = 0, 1, -2$. The polynomial $f(y) = y(y-1)(y+2)$ can be analyzed by sign: it is positive for $y \in (-2, 0)$, negative for $y \in (0, 1)$, positive for $y \in (1, \infty)$, and negative for $y < -2$.

## The Phase Line

The **phase line** is a copy of the real $y$-axis annotated with:
- dots at each equilibrium $y^*$ (where $f(y^*) = 0$);
- upward arrows in intervals where $f(y) > 0$ (solutions are increasing);
- downward arrows in intervals where $f(y) < 0$ (solutions are decreasing).

For the example $y' = y(y-1)(y+2)$:
- $y < -2$: $f < 0$, downward arrow
- $-2 < y < 0$: $f > 0$, upward arrow
- $0 < y < 1$: $f < 0$, downward arrow
- $y > 1$: $f > 0$, upward arrow

The phase line immediately reveals the long-term behavior of every initial condition.

## Stability Classification

**Asymptotically stable equilibria** (stable nodes or sinks) are those that attract nearby solutions as $x \to +\infty$. In the phase line, this occurs when arrows on both sides of $y^*$ point toward $y^*$: $f(y) > 0$ for $y$ slightly less than $y^*$ and $f(y) < 0$ for $y$ slightly greater than $y^*$.

**Unstable equilibria** (sources) repel nearby solutions. The arrows on both sides point away from $y^*$.

**Semi-stable equilibria** attract from one side and repel from the other, occurring when $f$ changes sign in only one direction at $y^*$ (i.e., $f$ is of one sign on both sides of $y^*$, or $f$ touches zero but remains on one side).

For the example: the equilibrium $y^* = -2$ has arrows pointing away on both sides (downward below, upward above going toward 0), so it is unstable. The equilibrium $y^* = 0$ has arrows pointing toward it from below and away from it above... actually $f > 0$ for $-2 < y < 0$ means solutions increase toward 0 from below, and $f < 0$ for $0 < y < 1$ means solutions decrease toward 0 from above. So $y^* = 0$ is asymptotically stable. The equilibrium $y^* = 1$ has arrows pointing away from it on both sides (toward 0 below, upward above), so it is unstable.

## The Linearization Test

The stability of an equilibrium can be determined analytically by the sign of $f'(y^*)$:
- If $f'(y^*) < 0$, then $y^*$ is asymptotically stable.
- If $f'(y^*) > 0$, then $y^*$ is unstable.
- If $f'(y^*) = 0$, the test is inconclusive; higher derivatives determine stability.

This test comes from linearizing $f(y)$ near $y^*$. Writing $y = y^* + u$ where $u$ is small:
$$y' = f(y^* + u) \approx f(y^*) + f'(y^*)u = f'(y^*)u.$$

So $u' \approx f'(y^*) u$, giving $u(x) \approx u(0)e^{f'(y^*)x}$. If $f'(y^*) < 0$, perturbations decay exponentially; if $f'(y^*) > 0$, they grow.

## Worked Example: The Logistic Equation

The logistic equation $y' = ry(1 - y/K)$ has equilibria at $y^* = 0$ and $y^* = K$. Here $f(y) = ry - ry^2/K$, so $f'(y) = r - 2ry/K$. At $y^* = 0$: $f'(0) = r > 0$, so $y^* = 0$ is unstable. At $y^* = K$: $f'(K) = r - 2r = -r < 0$, so $y^* = K$ is asymptotically stable.

The phase line confirms this: for $0 < y < K$, $f(y) > 0$, so solutions increase toward $K$. For $y > K$, $f(y) < 0$, so solutions decrease toward $K$. All solutions starting with $y(0) > 0$ approach $K$ as $x \to \infty$, which is the carrying capacity of the environment.

## Monotonicity of Non-Constant Solutions

A fundamental theorem for autonomous equations states that every non-constant solution is strictly monotone. This follows from uniqueness: if $y'(x_0) = f(\phi(x_0)) = 0$ for some $x_0$, then $\phi(x) = \phi(x_0)$ is the unique solution with that value, so $\phi$ is the constant function $y^* = \phi(x_0)$. If a solution is not identically constant, then $f(\phi(x)) \neq 0$ for all $x$, so $\phi'(x) \neq 0$ for all $x$, making $\phi$ strictly monotone.

This monotonicity rules out oscillatory behavior for one-dimensional autonomous systems. Oscillation requires two-dimensional (or higher) phase space: the simple harmonic oscillator $y'' + y = 0$, when written as a system $y' = v$, $v' = -y$, lives in the $yv$-plane and can orbit the origin without contradicting monotonicity in the individual components.
