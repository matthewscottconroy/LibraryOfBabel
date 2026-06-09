# Initial Value Problems

The general solution of an $n$-th order ODE contains $n$ arbitrary constants, and infinitely many functions satisfy the equation. In virtually every application, one wants the unique solution consistent with a specific physical state at a specific moment. The mathematical formulation of this requirement is an **initial value problem** (IVP).

## Definition

An **initial value problem** of order $n$ consists of an $n$-th order ODE together with $n$ **initial conditions** specifying the value of the solution and its first $n-1$ derivatives at a single point $x_0$:

$$y^{(n)} = f\!\left(x, y, y', \ldots, y^{(n-1)}\right), \qquad y(x_0) = y_0,\; y'(x_0) = y_1,\; \ldots,\; y^{(n-1)}(x_0) = y_{n-1}.$$

The point $x_0$ is the **initial point** and the prescribed values $y_0, y_1, \ldots, y_{n-1}$ are the **initial data** or **initial state**. For a first-order equation, one initial condition suffices; for a second-order equation, two are required; and so on.

The term "initial" reflects the physical origin: in mechanics, $x_0$ is typically the initial time $t_0$, and the data specify the initial position and velocity. In other contexts, $x_0$ might be a spatial coordinate, but the mathematical structure is the same.

## Physical Motivation

Newton's second law $m\ddot{x} = F(t, x, \dot{x})$ is a second-order ODE. Knowing a trajectory in Newtonian mechanics means knowing both position and velocity at the initial time: specifying $x(t_0) = x_0$ and $\dot{x}(t_0) = v_0$ determines the entire future (and past) motion, subject to conditions on $F$. This is the principle of determinism encoded mathematically.

For an RC circuit with voltage input $V(t)$, the charge $q$ satisfies the first-order ODE $\dot{q} + q/(RC) = V(t)/R$. Knowing the initial charge $q(0) = q_0$ determines $q(t)$ for all subsequent times. The single initial condition matches the single degree of freedom in the general solution.

## Worked Examples

**Example 1.** Solve the IVP

$$y' = 2y,\qquad y(0) = 3.$$

The general solution of $y' = 2y$ is $y = Ce^{2x}$. Imposing $y(0) = 3$ gives $Ce^0 = 3$, so $C = 3$. The solution is $y = 3e^{2x}$, defined for all $x \in (-\infty, \infty)$.

**Example 2.** Solve the IVP

$$y'' + y = 0,\qquad y(0) = 1,\; y'(0) = -2.$$

The general solution is $y = C_1\cos x + C_2\sin x$. Imposing $y(0) = 1$: $C_1 = 1$. Differentiating: $y' = -C_1\sin x + C_2\cos x$. Imposing $y'(0) = -2$: $C_2 = -2$. The solution is $y = \cos x - 2\sin x$, which can also be written as $y = \sqrt{5}\cos(x + \phi)$ where $\tan\phi = 2$.

**Example 3.** Solve the IVP

$$y' = y^2,\qquad y(0) = 1.$$

Separating: $dy/y^2 = dx$, so $-1/y = x + C$, giving $y = -1/(x+C)$. Imposing $y(0) = 1$: $-1/C = 1$, so $C = -1$. The solution is

$$y = \frac{1}{1 - x},$$

defined on $(-\infty, 1)$. Note that the solution blows up as $x \to 1^-$. Even though the ODE $y' = y^2$ is smooth everywhere, the particular solution with initial condition $y(0) = 1$ cannot be extended past $x = 1$. This illustrates the crucial point that the interval of existence depends on the initial condition.

## The Role of the Initial Point

For linear equations, the point $x_0$ can be any point where the coefficient functions are continuous, and the solution exists on the entire interval of continuity. For nonlinear equations, $x_0$ can be any point in the domain of $f$, but the solution may only exist on a smaller interval, as Example 3 shows.

The choice of initial point matters geometrically: it determines which solution curve in the general family passes through the specified point $(x_0, y_0)$ in the $xy$-plane, along with the required slope $y'(x_0) = y_1$, second derivative $y''(x_0) = y_2$, and so on.

## Well-Posedness

A problem is **well-posed** (in the sense of Hadamard) if it has a solution, the solution is unique, and the solution depends continuously on the data. For IVPs, the question of well-posedness is answered by Picard's theorem: under a Lipschitz condition on $f$, the IVP for $y' = f(x, y)$ with $y(x_0) = y_0$ has a unique solution, and this solution varies continuously with $(x_0, y_0)$.

Continuous dependence on initial data is physically essential. It says that small errors in measurement of the initial state lead to small errors in the predicted state at later times, at least on a bounded time interval. (Long-time behavior is a different matter; sensitive dependence on initial conditions over long times is the hallmark of chaos.) A problem that lacks continuous dependence is numerically and physically meaningless, because no physical measurement is exact.

## Comparison with Boundary Value Problems

Initial value problems specify all conditions at one point. **Boundary value problems** (BVPs) specify conditions at two or more distinct points. The IVP $y'' + y = 0$, $y(0) = 1$, $y'(0) = 0$ has the unique solution $y = \cos x$. The BVP $y'' + y = 0$, $y(0) = 0$, $y(\pi) = 0$ has infinitely many solutions $y = C\sin x$. The BVP $y'' + y = 0$, $y(0) = 0$, $y(\pi) = 1$ has no solution, because $C\sin(\pi) = 0 \neq 1$ for any $C$.

This contrast shows that the theory of BVPs is fundamentally different from the theory of IVPs. Existence and uniqueness for BVPs are more subtle and depend on global properties of the equation on the entire interval, not just local smoothness at a single point.
