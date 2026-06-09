# Ordinary Differential Equations

If multivariable calculus is the grammar of multi-component models, then ordinary differential equations are their native sentences. Every mechanistic model in systems biology is, at its core, a system of ODEs. Every pathway in the BioModels database, every gene circuit analyzed in a synthetic biology paper, every pharmacokinetic model used in drug development — all of them amount to a set of statements of the form $dx/dt = f(x)$. The protein level changes because it is being produced at rate $\beta$ and degraded at rate $\delta [P]$. The kinase becomes phosphorylated when it binds its activating signal. The morphogen diffuses down its gradient and is degraded by target cells.

What makes ODEs so powerful is that writing them down is the act of stating your biological assumptions explicitly. The equation *is* the hypothesis. And what makes this section important is that ODEs have a rich mathematical structure — steady states, stability, oscillations, bifurcations — that turns out to correspond directly to real biological phenomena: set points, homeostasis, clocks, and cell fate decisions.

## First-Order ODEs

A **first-order ODE** relates a function $x(t)$ to its first derivative:

$$\frac{dx}{dt} = f(x, t)$$

**Separable ODEs** can be solved by separating variables and integrating:

$$\frac{dx}{g(x)} = h(t)\, dt \implies \int \frac{dx}{g(x)} = \int h(t)\, dt$$

**Example:** Protein production with constitutive synthesis and first-order degradation:

$$\frac{d[P]}{dt} = \beta - \delta [P]$$

This is a **linear first-order ODE** with constant coefficients. Separating and solving:

$$[P](t) = \frac{\beta}{\delta} + \left([P]_0 - \frac{\beta}{\delta}\right) e^{-\delta t}$$

The steady state is $[P]^* = \beta/\delta$, reached with time constant $\tau = 1/\delta$. This "1/delta rule" is fundamental: the steady-state protein level is proportional to the production rate divided by the degradation rate, and perturbations decay with characteristic time $1/\delta$. It is not an exaggeration to say that this single equation, and its consequences, underlies most of what Uri Alon's textbook calls the "design principles" of gene circuits. The faster the degradation, the faster the response — but also the lower the steady-state level.

**Linear first-order ODEs** with non-constant coefficients use the **integrating factor** method. Given $\dot{x} + p(t) x = q(t)$, multiply both sides by $\mu(t) = e^{\int p(t)\, dt}$, and the left side becomes an exact derivative.

## Systems of ODEs

In systems biology, the state of a cell or network is described by a vector $\mathbf{x}(t) = (x_1, x_2, \ldots, x_n)^T$, and its dynamics by a system:

$$\frac{d\mathbf{x}}{dt} = \mathbf{f}(\mathbf{x},\ \mathbf{p})$$

where $\mathbf{p}$ is a vector of parameters (rate constants, binding affinities, etc.). The canonical example is the **Lotka-Volterra predator-prey system**:

$$\frac{dR}{dt} = \alpha R - \beta R P$$

$$\frac{dP}{dt} = \delta R P - \gamma P$$

where $R$ is prey (e.g., bacteria), $P$ is predator (e.g., phage), and $\alpha, \beta, \delta, \gamma > 0$ are rate constants. This system has a nontrivial fixed point at $R^* = \gamma/\delta$, $P^* = \alpha/\beta$ and exhibits periodic oscillations — a classic example of ecological cycling with a direct analogy to predator-prey dynamics in microbial communities. The oscillations are neutrally stable, which turns out to be a fragile property — a fact we will return to when discussing limit cycles.

## Phase Portraits and Fixed Points

A **fixed point** (steady state, equilibrium) satisfies $\mathbf{f}(\mathbf{x}^*) = 0$. The **phase portrait** is a plot of the vector field $d\mathbf{x}/dt$ across state space, showing the trajectories a system can take from any initial condition.

**Nullclines** are curves where individual derivatives vanish: the $x$-nullcline satisfies $\dot{x} = 0$, the $y$-nullcline satisfies $\dot{y} = 0$. Fixed points occur at intersections of nullclines. The shape and arrangement of nullclines determines whether the system has one, two, or three fixed points — and this directly governs biological phenomena like bistability and cell fate decisions. Before you ever compute a single eigenvalue, drawing the nullclines of a two-dimensional system will already reveal whether the system has multiple stable states and which direction trajectories flow in each region of state space.

## Stability Analysis via Linearization

To determine the stability of a fixed point $\mathbf{x}^*$, linearize the system around it. Write $\mathbf{x}(t) = \mathbf{x}^* + \boldsymbol{\xi}(t)$ where $\boldsymbol{\xi}$ is a small perturbation. Then:

$$\frac{d\boldsymbol{\xi}}{dt} = J(\mathbf{x}^*) \boldsymbol{\xi} + O(|\boldsymbol{\xi}|^2)$$

where $J = \partial \mathbf{f}/\partial \mathbf{x}|_{\mathbf{x}^*}$ is the Jacobian. The solution of the linearized system is:

$$\boldsymbol{\xi}(t) = \sum_i c_i \mathbf{v}_i e^{\lambda_i t}$$

where $\lambda_i$ are eigenvalues and $\mathbf{v}_i$ are eigenvectors of $J$. **Stability criterion:** the fixed point is **asymptotically stable** if and only if all eigenvalues have negative real parts ($\text{Re}(\lambda_i) < 0$ for all $i$).

This is the single most important result in the chapter for practical modeling work. Any time you build a model and want to know whether it will settle to a steady state or run away, you compute the Jacobian eigenvalues. The biology is in the ODEs; the stability is in the eigenvalues.

## Limit Cycles and Oscillatory Behavior

A **limit cycle** is an isolated closed trajectory in phase space — the attractor of an oscillating system. Unlike the Lotka-Volterra neutrally stable orbits, a limit cycle is robust: perturbed trajectories return to it. This distinction matters enormously for biology. Circadian clocks, cell cycle oscillations, and NF-$\kappa$B pulses are limit cycles — they keep oscillating even after perturbations, because the limit cycle is a stable attractor. A Lotka-Volterra orbit, by contrast, would be shifted permanently by any perturbation.

The **Poincaré-Bendixson theorem** guarantees a limit cycle exists in a 2D system if: (1) there is a bounded, positively invariant region, and (2) there are no fixed points inside it (or all fixed points are unstable). This theorem cannot be applied in dimensions $\geq 3$, where chaotic attractors can exist. This is why two-dimensional phase plane analysis is such a powerful tool — a 2D system either goes to a fixed point or oscillates on a limit cycle, and nothing else is possible.

## Bifurcation Theory

A **bifurcation** occurs when a qualitative change in the system's dynamics arises as a parameter varies continuously. Three types are essential for biological modeling:

**Saddle-node bifurcation:** Two fixed points (one stable, one unstable) collide and annihilate as a parameter is varied. This underlies bistability — below a critical parameter value, two stable steady states coexist; above it, only one remains. Cell fate decisions often depend on this bifurcation structure. The famous lysis/lysogeny switch in phage $\lambda$, the bistable MAPK cascade, and the Cdc42 polarity switch are all governed by saddle-node bifurcations.

**Pitchfork bifurcation:** A symmetric system transitions from one stable fixed point to two stable fixed points as a parameter crosses a threshold. Appears in symmetry-breaking during development and cell polarization. When a round cell breaks symmetry to form a front and a back, you are watching a pitchfork bifurcation.

**Hopf bifurcation:** A stable fixed point loses stability and gives birth to a limit cycle as a parameter crosses a critical value. This is how sustained oscillations emerge from a stable steady state — the mechanism underlying the emergence of circadian rhythms, somitogenesis oscillations, and p53 pulsing. The remarkable thing about the Hopf bifurcation is that it converts a qualitatively stable resting state into a qualitatively oscillating state simply by tuning one parameter past a threshold. Biology uses this trick constantly.

## Why This Matters for Computational Biology

ODE systems are the workhorse model in systems and synthetic biology. Every pathway model in SBML is a system of ODEs. Every gene circuit designed in synthetic biology is analyzed by writing and solving ODEs. Stability analysis tells you whether a circuit will reach a steady state or oscillate. Bifurcation analysis tells you how robust a biological switch is and what perturbations can flip it. Numerical ODE solvers (like `scipy.integrate.solve_ivp`) are tools you will use daily — but understanding the underlying mathematics is what lets you know when to trust the numerics and when something has gone wrong.

```python
import numpy as np
from scipy.integrate import solve_ivp
import matplotlib.pyplot as plt

def lotka_volterra(t, state, alpha=1.0, beta=0.1, delta=0.075, gamma=1.5):
    R, P = state
    dR = alpha * R - beta * R * P
    dP = delta * R * P - gamma * P
    return [dR, dP]

sol = solve_ivp(lotka_volterra, [0, 80], [10, 5],
                t_eval=np.linspace(0, 80, 2000), rtol=1e-8)

fig, axes = plt.subplots(1, 2, figsize=(12, 4))
axes[0].plot(sol.t, sol.y[0], label='Prey (R)')
axes[0].plot(sol.t, sol.y[1], label='Predator (P)')
axes[0].set_xlabel('Time'); axes[0].legend()
axes[0].set_title('Lotka-Volterra Dynamics')

axes[1].plot(sol.y[0], sol.y[1])
axes[1].set_xlabel('Prey (R)'); axes[1].set_ylabel('Predator (P)')
axes[1].set_title('Phase Portrait')

# Fixed point
alpha, beta, delta, gamma = 1.0, 0.1, 0.075, 1.5
R_star = gamma / delta
P_star = alpha / beta
axes[1].plot(R_star, P_star, 'r*', markersize=12, label='Fixed point')
axes[1].legend()
plt.tight_layout()
```
