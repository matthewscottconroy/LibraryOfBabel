# From Chemistry to Equations: Mass Action Kinetics

## The Law of Mass Action

Imagine you are watching two molecules collide. How often do they encounter each other? If you double the concentration of one of them, collisions should double — each molecule of the first type now has twice as many potential partners to run into. If you double both, collisions should quadruple. This simple probabilistic reasoning, worked out rigorously by the Norwegian chemists Guldberg and Waage in the 1860s, is the **law of mass action**: the rate of a chemical reaction is proportional to the product of the concentrations of its reactants.

It turns out this 19th-century principle is the foundation of virtually all deterministic biological modeling. Every ODE model of gene regulation, signaling, or metabolism you will ever encounter is ultimately a system of mass-action-derived equations — perhaps simplified by quasi-steady-state approximations, perhaps complicated by cooperative binding or saturation effects, but always grounded in the same principle: reaction rate is proportional to reactant concentrations.

For a bimolecular reaction:

$$A + B \xrightarrow{k} C$$

the rate of product formation is:

$$v = k[A][B]$$

where $k$ is the **rate constant** (units: M$^{-1}$s$^{-1}$ for bimolecular reactions). For a unimolecular reaction $A \xrightarrow{k} B$, the rate is simply $v = k[A]$ (units: s$^{-1}$). The dimensionality of $k$ always adjusts to make the rate have units of concentration per time.

## Writing ODEs from Reaction Schemes

Given a set of reactions, the ODE for each species is constructed by summing all reaction rates that produce or consume it:

$$\frac{d[X]}{dt} = \sum_j \nu_{Xj} \cdot v_j$$

where $\nu_{Xj}$ is the stoichiometric coefficient of species $X$ in reaction $j$ (positive for products, negative for reactants), and $v_j$ is the rate of reaction $j$.

This rule is remarkably mechanical — almost algorithmic. You list your reactions, write down the rate for each, and sum up contributions for each species. The skill lies not in the bookkeeping but in knowing which reactions to include and how to write their rates.

**Simple Gene Expression Example**

Consider the minimal gene expression model with four reactions:

| Reaction | Process | Rate |
|---|---|---|
| $\emptyset \xrightarrow{\alpha} m$ | Transcription | $\alpha$ |
| $m \xrightarrow{\delta_m} \emptyset$ | mRNA degradation | $\delta_m [m]$ |
| $m \xrightarrow{\beta} m + p$ | Translation | $\beta [m]$ |
| $p \xrightarrow{\delta_p} \emptyset$ | Protein degradation | $\delta_p [p]$ |

This yields:

$$\frac{d[m]}{dt} = \alpha - \delta_m [m]$$

$$\frac{d[p]}{dt} = \beta [m] - \delta_p [p]$$

Steady-state analysis ($d/dt = 0$) gives:

$$[m]^* = \frac{\alpha}{\delta_m}, \quad [p]^* = \frac{\beta \alpha}{\delta_m \delta_p}$$

Notice something interesting about the protein steady state: it depends on the ratio $\beta/\delta_p$ (protein production per mRNA per unit time) multiplied by $\alpha/\delta_m$ (the mRNA level). This means cells can tune protein abundance by changing any of four independent parameters — transcription rate, mRNA stability, translation rate, or protein stability — and the steady-state protein level responds in predictable, multiplicative ways. That kind of quantitative intuition is only possible with the equations in hand.

## Worked Example: Lotka-Volterra Predator-Prey System

The Lotka-Volterra system is the canonical example of coupled nonlinear ODEs derived from mass action kinetics. Consider a population of prey (rabbits, $R$) and predators (foxes, $F$):

| Reaction | Process | Rate |
|---|---|---|
| $R \xrightarrow{\alpha} 2R$ | Prey reproduction | $\alpha R$ |
| $R + F \xrightarrow{\beta} F$ | Predation (prey consumed) | $\beta R F$ |
| $R + F \xrightarrow{\gamma} 2F$ | Predator reproduction | $\gamma R F$ |
| $F \xrightarrow{\delta} \emptyset$ | Predator death | $\delta F$ |

Applying the construction rule:

$$\frac{dR}{dt} = \alpha R - \beta R F$$

$$\frac{dF}{dt} = \gamma R F - \delta F$$

You might expect this system to reach a stable equilibrium — predators and prey coexist at fixed numbers. But it turns out the behavior is far richer: the system exhibits **sustained oscillations**. Prey and predator populations cycle indefinitely, with foxes peaking slightly after rabbits (the predator population can only grow after there are many rabbits to eat). The fixed points are $(R^*, F^*) = (\delta/\gamma,\; \alpha/\beta)$ and the origin, and near the interior fixed point, solutions are closed orbits — a prediction confirmed in ecological time series.

This result was surprising when Vito Volterra derived it in 1925 (motivated by fish catch data from the Adriatic Sea during World War I). The oscillations are not driven by any external cycle — they are intrinsic to the interaction structure of the two-species system. Without equations, it would be almost impossible to anticipate this.

```python
import numpy as np
from scipy.integrate import solve_ivp
import matplotlib.pyplot as plt

def lotka_volterra(t, y, alpha, beta, gamma, delta):
    R, F = y
    dR = alpha * R - beta * R * F
    dF = gamma * R * F - delta * F
    return [dR, dF]

# Parameters
params = dict(alpha=1.0, beta=0.1, gamma=0.075, delta=1.5)
y0 = [10.0, 5.0]  # initial populations
t_span = (0, 50)
t_eval = np.linspace(*t_span, 1000)

sol = solve_ivp(lotka_volterra, t_span, y0, args=tuple(params.values()),
                t_eval=t_eval, method='RK45')

plt.plot(sol.t, sol.y[0], label='Prey (R)')
plt.plot(sol.t, sol.y[1], label='Predator (F)')
plt.xlabel('Time'); plt.ylabel('Population'); plt.legend()
plt.title('Lotka-Volterra Oscillations')
```

## Important Subtleties

**Units must be consistent.** If concentrations are in $\mu$M and time is in minutes, rate constants must carry compatible units. A common error is mixing per-second rate constants with per-minute simulation time, which introduces a factor of 60 error that can be hard to detect because everything still looks qualitatively sensible.

**Mass action applies to elementary reactions.** The reaction $A + B \rightarrow C$ must represent a single mechanistic step — two molecules colliding and reacting in one event. Composite reactions (like the overall Michaelis-Menten reaction $S \rightarrow P$) do not follow mass action kinetics unless the rate law is derived from the elementary steps. This is why we need the quasi-steady-state approximation discussed in the next section.

**Volume scaling for stochastic models.** When the number of molecules is small, concentrations become discrete. The transition from concentration-based to molecule-number-based descriptions introduces factors of system volume $\Omega$. The propensity for a bimolecular reaction $A + B \rightarrow$ products in a volume $\Omega$ is $k [A][B] = k \cdot (n_A/\Omega)(n_B/\Omega)$ — but the stochastic rate constant $c$ absorbs a factor of $1/\Omega$. This distinction becomes critical in Section 3 on stochastic modeling: confusing concentration-based and molecule-number-based rate constants is one of the most common errors in converting from ODE to Gillespie-algorithm models.

## Why This Matters

Mass action kinetics is the universal grammar of deterministic biological modeling. Mastering the translation from reaction scheme to ODE is the first essential skill in quantitative biology — not because you will spend your career doing bookkeeping, but because you will be constantly reading models written by others, checking whether their reaction rates make sense, and identifying the assumptions baked into their equations. The mechanistic transparency of mass action kinetics is precisely what makes ODE models scientifically valuable: every term has a direct biological interpretation, and wrong terms can be identified and corrected.
