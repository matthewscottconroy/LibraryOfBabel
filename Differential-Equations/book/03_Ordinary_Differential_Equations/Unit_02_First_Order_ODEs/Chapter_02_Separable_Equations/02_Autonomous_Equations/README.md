# Autonomous Equations

An autonomous first-order equation $y' = f(y)$ has no explicit dependence on the independent variable $x$. This structural simplicity enables a complete qualitative theory and makes autonomous equations among the most studied in both pure mathematics and applications.

## Definition and Basic Properties

The equation $y' = f(y)$ is **autonomous** because the rate of change of $y$ is determined entirely by the current value $y$, not by $x$. Physically, this models systems in which the rules of evolution do not change over time: a population growing according to a fixed biological law, a chemical reaction under steady conditions, a pendulum in a uniform gravitational field.

The key algebraic fact is that autonomous equations are always separable: writing $dy/f(y) = dx$ and integrating. The key geometric fact is that all solution curves are horizontal translates of each other: if $\phi(x)$ is a solution, so is $\phi(x - c)$ for any constant $c$.

## Reduction to Quadrature

For an autonomous equation, the method of separation gives

$$\int_{y_0}^{y} \frac{du}{f(u)} = x - x_0,$$

provided $f(u) \neq 0$ on the integration path. This implicitly defines $y$ as a function of $x$. In favorable cases the integral can be evaluated in closed form, giving an explicit solution. In other cases, the integral defines a special function (as with the elliptic integrals arising from the pendulum equation $\theta'' = -(g/\ell)\sin\theta$, which can be written as a system and then treated as a first-order autonomous equation for $\theta'$ as a function of $\theta$).

## Examples

**Exponential growth.** $y' = ky$, $k > 0$. Separating: $\int dy/y = kx + C$, giving $y = Ce^{kx}$. The only equilibrium is $y^* = 0$, which is unstable (since $f'(0) = k > 0$). Every solution with $y(0) > 0$ grows without bound; every solution with $y(0) < 0$ decays toward $-\infty$.

**Logistic saturation.** $y' = y(1-y)$. Equilibria at $y^* = 0$ (unstable) and $y^* = 1$ (stable). Solutions between 0 and 1 increase monotonically toward 1. Solutions above 1 decrease toward 1.

**Bistable switch.** $y' = y(1-y)(y-a)$ for $0 < a < 1$. There are three equilibria: 0 (stable), $a$ (unstable), 1 (stable). Solutions starting below $a$ converge to 0; solutions starting above $a$ converge to 1. The unstable equilibrium $y = a$ is the **threshold**: initial conditions on either side lead to opposite long-term outcomes. This model captures bistability in gene regulatory networks, neuron firing, and chemical reactions.

## Time to Reach Equilibrium

For a stable equilibrium, no solution reaches it in finite time (since the solution is a smooth strictly monotone function of $x$, and the equilibrium is a constant function; they can only be asymptotically equal). However, the "time" to get within $\epsilon$ of equilibrium is determined by the integral

$$x - x_0 = \int_{y_0}^{y^* - \epsilon} \frac{du}{f(u)},$$

which diverges logarithmically for generic smooth $f$ vanishing simply at $y^*$: near $y^*$, $f(u) \approx f'(y^*)(u - y^*)$, so

$$\int \frac{du}{f'(y^*)(u - y^*)} = \frac{1}{f'(y^*)}\ln|u - y^*| + \text{const.}$$

This integral to $u = y^*$ diverges, confirming that the equilibrium is approached in infinite time. The rate of approach is exponential: $|y(x) - y^*| \sim Ce^{f'(y^*)x}$ for large $x$.

## Connection to Dynamical Systems

Autonomous ODEs are the simplest example of a **continuous dynamical system**: a flow on the real line $\mathbb{R}$. The flow map $\Phi^t: \mathbb{R} \to \mathbb{R}$ sends each initial condition $y_0$ to the solution value $y(t)$ with $y(0) = y_0$. The one-parameter family $\{\Phi^t\}_{t \in \mathbb{R}}$ satisfies $\Phi^0 = \mathrm{id}$ and $\Phi^{s+t} = \Phi^s \circ \Phi^t$ (the group property), reflecting the translation invariance of autonomous systems.

The phase line analysis is the complete picture of the dynamics of this flow: the basins of attraction of the stable equilibria partition $\mathbb{R}$ into intervals, each converging to a different attracting equilibrium. The separatrices between basins are the unstable equilibria. Understanding this picture, for one-dimensional systems, requires no more than the sign analysis of $f(y)$.
