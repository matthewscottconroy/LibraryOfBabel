# The Logistic Equation

Exponential growth is sustainable only in an unlimited environment. When resources are finite, population growth slows as the population approaches the environment's **carrying capacity**. The logistic equation, introduced by Pierre-Francois Verhulst in 1838, is the simplest model capturing this saturation and remains one of the most important differential equations in mathematical biology and ecology.

## The Model

The logistic equation is

$$\frac{dP}{dt} = rP\left(1 - \frac{P}{K}\right),$$

where $P(t)$ is the population at time $t$, $r > 0$ is the intrinsic growth rate (the per capita growth rate when population is far below carrying capacity), and $K > 0$ is the **carrying capacity** (the maximum sustainable population). The factor $(1 - P/K)$ is a crowding term that reduces the effective growth rate as $P$ increases. At $P = K$, growth stops; above $K$, the term is negative and the population declines.

Rewriting: $dP/dt = rP - (r/K)P^2$. The linear term $rP$ is the Malthusian (unrestricted) growth, and the quadratic term $-(r/K)P^2$ represents the drag from intraspecific competition.

## Equilibria and Phase Line

Setting $rP(1 - P/K) = 0$ gives equilibria $P^* = 0$ and $P^* = K$. The linearization test: $f(P) = rP(1 - P/K)$, $f'(P) = r - 2rP/K$.

At $P^* = 0$: $f'(0) = r > 0$, so $P^* = 0$ is unstable.
At $P^* = K$: $f'(K) = r - 2r = -r < 0$, so $P^* = K$ is asymptotically stable.

For $0 < P < K$: $f(P) > 0$, population increases. For $P > K$: $f(P) < 0$, population decreases. All solutions starting with $P(0) > 0$ converge to $K$.

## Exact Solution by Separation

The logistic equation is separable. With $P(0) = P_0$:

$$\int_{P_0}^{P} \frac{dP'}{P'(1 - P'/K)} = rt.$$

Partial fractions: $\frac{1}{P(1-P/K)} = \frac{K}{P(K-P)} = \frac{1}{P} + \frac{1}{K - P}$.

Integrating: $\ln P - \ln(K - P) = rt + C_1$, so $\ln\frac{P}{K-P} = rt + C_1$.

Exponentiating: $\frac{P}{K-P} = Ae^{rt}$ where $A = e^{C_1}$.

Imposing $P(0) = P_0$: $A = P_0/(K - P_0)$. Solving for $P$:

$$P = \frac{AKe^{rt}}{1 + Ae^{rt}} = \frac{K}{1 + \frac{K - P_0}{P_0}e^{-rt}}.$$

This is the **logistic function**, a sigmoid (S-shaped) curve. Writing it as $P(t) = K/(1 + Be^{-rt})$ with $B = (K - P_0)/P_0$:

- As $t \to -\infty$: $P \to 0$ (the zero equilibrium is the past state when $P_0 < K$).
- As $t \to +\infty$: $P \to K$ (the carrying capacity is the asymptotic future state).
- The inflection point occurs at $P = K/2$, the population at which growth rate $dP/dt$ is maximized.

**Maximum growth rate.** Differentiating $r = dP/dt = rP(1-P/K)$ with respect to $P$ and setting to zero: $r(1 - 2P/K) = 0$, giving $P = K/2$. The maximum growth rate is $rK/4$, achieved at the inflection point.

## The Inflection Point as a Prediction Tool

Because the logistic curve is symmetric about its inflection point $(t^*, K/2)$, one can use observations of the population accelerating (below $K/2$) and decelerating (above $K/2$) to identify the inflection and thereby estimate $K$. This is used in epidemiology (estimating the final size of an outbreak) and in population ecology (estimating carrying capacity from census data).

The inflection time $t^*$ satisfies $P(t^*) = K/2$:

$$\frac{K/2}{K - K/2} = \frac{K-P_0}{P_0}e^{rt^*} \implies 1 = \frac{K - P_0}{P_0}e^{rt^*} \implies t^* = \frac{1}{r}\ln\frac{K - P_0}{P_0}.$$

For $P_0 \ll K$, $t^* \approx (\ln K - \ln P_0)/r$, showing that the inflection is delayed when the initial population is very small.

## The Harvesting Model

Adding constant harvesting $h$ to the logistic model gives

$$\frac{dP}{dt} = rP\left(1 - \frac{P}{K}\right) - h.$$

The equilibria are found from $rP(1 - P/K) = h$: this is a quadratic $rP - (r/K)P^2 = h$, or $(r/K)P^2 - rP + h = 0$, with solutions

$$P^* = \frac{K}{2}\left(1 \pm \sqrt{1 - \frac{4h}{rK}}\right).$$

For $h < rK/4$ (harvesting below the maximum sustainable yield), two equilibria exist. The larger is stable, the smaller is unstable. For $h = rK/4$ (harvesting at maximum sustainable yield), the two equilibria merge into a single semi-stable one at $P = K/2$. For $h > rK/4$, no equilibria exist and all populations collapse to zero. This is a saddle-node bifurcation: the model predicts population collapse if harvesting exceeds the critical threshold $h_{\max} = rK/4$.

## Generalizations

The logistic equation is the $n=1$ case of the Lotka-Volterra predator-prey system (for $n=2$ species) and the replicator equation (in evolutionary game theory). It also appears in neural activation functions (the logistic sigmoid), in physics as the equation for a saturating damped system, and in epidemiology as the underlying structure of the SIS model. Its mathematical simplicity, exact solvability, rich qualitative behavior, and broad applicability make it the most important autonomous nonlinear ODE in the first-order theory.
