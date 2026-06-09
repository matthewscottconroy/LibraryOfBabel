# Bistability Analysis: Nullclines, Stability, and Parameter Sensitivity

Two stable states. That is the claim. But how do you know, before building a circuit, whether mutual repression between two proteins will actually produce bistability, or whether the system will just settle to a single intermediate fixed point? How do you know which parameter combinations produce two wells of stability, and which produce only one? These are questions the biology alone cannot answer — you need the mathematics. The **nullcline analysis** of the toggle switch ODE model is one of the most instructive examples of how dynamical systems theory earns its keep in biological circuit design. It transforms the question "will this work?" from a matter of intuition into a matter of calculation.

Understanding *why* a toggle switch is bistable — and under what conditions bistability is lost — requires mathematical analysis. This analysis provides quantitative predictions about which parameter combinations produce bistability and what external perturbations are needed to switch states.

## The Toggle Switch ODE Model

Let $u$ represent the concentration of Repressor 1 and $v$ represent the concentration of Repressor 2. Assuming Hill-function regulation and first-order protein degradation:

$$\frac{du}{dt} = \frac{\alpha_1}{1 + v^\beta} - u$$

$$\frac{dv}{dt} = \frac{\alpha_2}{1 + u^\gamma} - v$$

Where:
- $\alpha_1, \alpha_2$: maximum expression rates of each repressor (promoter strength)
- $\beta, \gamma$: Hill coefficients of repression (cooperativity of each repressor)
- The $-u$ and $-v$ terms represent first-order protein degradation (all rates have been normalized so the degradation rate = 1)

This model has been **non-dimensionalized**: protein concentrations are in units of $K$ (the repressor concentration for half-maximal repression), and time is in units of $1/\delta$ (where $\delta$ is the degradation rate). This normalization reduces the parameter space.

## Nullcline Analysis

A **nullcline** is the set of points where one of the time derivatives equals zero:

**$u$-nullcline**: set $\frac{du}{dt} = 0$:
$$u = \frac{\alpha_1}{1 + v^\beta} \implies v = \left(\frac{\alpha_1}{u} - 1\right)^{1/\beta}$$

This is a decreasing curve in the $(u, v)$ phase plane: as $u$ increases, the $v$ required for the $u$-nullcline to be satisfied decreases.

**$v$-nullcline**: set $\frac{dv}{dt} = 0$:
$$v = \frac{\alpha_2}{1 + u^\gamma} \implies v = \frac{\alpha_2}{1 + u^\gamma}$$

This is also a decreasing curve in the $(u, v)$ phase plane.

## Graphical Stability Analysis

Fixed points of the system are where both nullclines intersect: $\dot{u} = \dot{v} = 0$ simultaneously. The number and nature of these intersections determines the dynamics:

**Case 1: Single intersection (monostable)**
- Occurs when $\alpha_1$ or $\alpha_2$ is too large (one repressor dominates) or Hill coefficients are too low
- One stable fixed point: system settles to a unique state regardless of initial conditions
- No memory: the system always returns to the same state

**Case 2: Three intersections (bistable)**
- Two stable fixed points (outer intersections): stable nodes
- One unstable fixed point (middle intersection): saddle point — acts as the separatrix between basins of attraction

The condition for three intersections: the nullclines must cross each other, which requires that the slope of one nullcline at the intersection point differs from the slope of the other. Mathematically, this requires **sufficient cooperativity**.

```python
import numpy as np
import matplotlib.pyplot as plt

def toggle_nullclines(alpha1=3, alpha2=3, beta=2, gamma=2):
    u_range = np.linspace(0.01, alpha1 + 0.5, 500)
    v_range = np.linspace(0.01, alpha2 + 0.5, 500)
    
    # u-nullcline: v as function of u
    # From: u = alpha1/(1 + v^beta) -> v = ((alpha1/u) - 1)^(1/beta)
    u_null_u = u_range
    # Avoid invalid values where alpha1/u - 1 < 0
    mask = u_range < alpha1
    u_null_v = np.where(mask, (alpha1/u_range[mask] - 1)**(1/beta), np.nan)
    
    # v-nullcline: v as function of u
    v_null_v = alpha2 / (1 + u_range**gamma)
    
    plt.figure(figsize=(6, 6))
    plt.plot(u_range[mask], u_null_v, 'b-', label='u-nullcline (du/dt=0)')
    plt.plot(u_range, v_null_v, 'r-', label='v-nullcline (dv/dt=0)')
    plt.xlabel('u (Repressor 1)'); plt.ylabel('v (Repressor 2)')
    plt.legend(); plt.title(f'Toggle Switch Nullclines\nα₁={alpha1}, α₂={alpha2}, β={beta}, γ={gamma}')
    plt.show()

toggle_nullclines(alpha1=3, alpha2=3, beta=2, gamma=2)   # bistable
toggle_nullclines(alpha1=3, alpha2=3, beta=1, gamma=1)   # monostable (no cooperativity)
```

## Bistability Condition: A Practical Rule

For the symmetric case ($\alpha_1 = \alpha_2 = \alpha$ and $\beta = \gamma = n$), bistability requires:

$$\alpha > \alpha_{critical} \approx \frac{(n+1)^{(n+1)/n}}{n}$$

For $n = 2$ (cooperativity): $\alpha_{critical} \approx 2.25$. This means both repressor maximum concentrations must exceed 2.25× their respective $K$ values for bistability.

For $n = 1$ (no cooperativity): $\alpha_{critical} \to \infty$ — bistability is impossible without cooperativity regardless of expression level.

## Switching Dynamics: Inducer as a Perturbation

When IPTG is added to a cell in State 1 (u = LacI high, v = TetR low):
- IPTG effectively reduces $\alpha_1$ (it sequesters LacI, reducing its effective repressor activity)
- As $\alpha_1$ decreases below $\alpha_{critical}$, the system becomes monostable with State 2 as the only stable state
- The system moves to State 2
- When IPTG is removed, $\alpha_1$ returns to its original value; the system is again bistable, but now in State 2

This analysis predicts a **minimum switching dose**: the inducer must reduce $\alpha_1$ (or equivalently, increase the effective $K_1$) enough to cross the monostability threshold. Too little inducer → incomplete switching → cells return to State 1 when inducer is removed.

## Noise-Driven Switching (Spontaneous State Flipping)

At finite protein copy numbers, stochastic fluctuations can cause spontaneous switching between states. The rate of spontaneous switching scales as:

$$k_{switch} \propto e^{-\Delta U / D}$$

where $\Delta U$ is the energy barrier between states (related to the depth of the potential wells) and $D$ is the noise strength (related to protein copy number). Lower protein expression → higher noise → faster spontaneous switching.

For engineered toggle switches with typical protein levels (~100–1000 molecules/cell), spontaneous switching occurs on timescales of hours to days. For applications requiring stable memory (e.g., therapeutic cell engineering), this can be problematic and motivates designs with higher protein expression or additional positive feedback.

## Why This Matters

The nullcline analysis of the toggle switch is a template for analyzing any bistable system in biology. The same mathematical framework applies to:
- The lysis-lysogeny decision in phage lambda
- Stem cell differentiation commitment
- Cell fate decisions in development
- Epigenetic memory

More immediately practical: the stability analysis tells you *before building* whether a proposed toggle switch design will be bistable, and if not, which parameters to change. The cooperativity requirement (n > 1) is a hard constraint that eliminates many candidate repressor pairs; only those with demonstrated cooperativity (through operator looping, dimerization, or other mechanisms) can support bistability in a two-repressor design.
