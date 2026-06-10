# Section 4.2: The Lorenz System

## Physical Origin

In 1963, meteorologist Edward Norton Lorenz published a paper titled "Deterministic Nonperiodic Flow" [Lorenz1963] that would become one of the most cited papers in the history of science. He was not, at the time, trying to invent chaos theory. He was trying to understand convection in the atmosphere.

The physical setting is a horizontal layer of fluid — like the atmosphere — heated from below and cooled from above. When the temperature difference is large enough, the fluid spontaneously develops convective rolls: the hotter fluid near the bottom rises, carries heat to the top, cools, and descends. These rolls are the large-scale circular motions visible in cumulus clouds, ocean currents, and the swirling patterns on Jupiter.

Barry Saltzman [Saltzman1962] had derived a simplified model of this convective flow by truncating an infinite Fourier series expansion to just a few modes. Lorenz took Saltzman's equations, simplified them further, and arrived at a system of three ODEs that captured the essence of the instability.

---

## The Equations

The Lorenz system is:

$$\dot{x} = \sigma(y - x) \tag{4.4a}$$
$$\dot{y} = x(\rho - z) - y \tag{4.4b}$$
$$\dot{z} = xy - \beta z \tag{4.4c}$$

The three variables have physical interpretations:
- $x$: the rate of convective overturning (proportional to the velocity of the convective roll)
- $y$: the horizontal temperature gradient between rising and descending fluid
- $z$: the deviation of the vertical temperature profile from linearity

The three parameters are:
- $\sigma > 0$: the **Prandtl number** (ratio of viscous diffusivity to thermal diffusivity). For water, $\sigma \approx 10$.
- $\rho > 0$: the **Rayleigh number** (normalized by its critical value for the onset of convection). The value $\rho = 28$ is Lorenz's canonical choice.
- $\beta > 0$: a geometric factor related to the aspect ratio of the convective rolls. Lorenz used $\beta = 8/3$.

The **canonical parameters** are $\sigma = 10$, $\rho = 28$, $\beta = 8/3$.

---

## Symmetry, Boundedness, and Volume Contraction

The Lorenz system has a $\mathbb{Z}_2$ symmetry: it is invariant under the transformation $(x, y, z) \mapsto (-x, -y, z)$. If $(x(t), y(t), z(t))$ is a solution, so is $(-x(t), -y(t), z(t))$. Geometrically, the attractor must be symmetric about the $z$-axis.

The system is also **dissipative**: the phase space volume contracts. The divergence of the vector field is:

$$\nabla \cdot F = \frac{\partial \dot{x}}{\partial x} + \frac{\partial \dot{y}}{\partial y} + \frac{\partial \dot{z}}{\partial z} = -\sigma - 1 - \beta$$

For the canonical parameters, this is $-\sigma - 1 - \beta = -10 - 1 - 8/3 \approx -13.67$. By Liouville's theorem, volumes in phase space contract at rate $e^{(\nabla \cdot F) t}$, i.e., exponentially fast. Any initial volume shrinks to zero at rate $e^{-(\sigma + 1 + \beta)t}$. This means the system must have an attractor — an invariant set with zero volume that trajectories converge to.

Lorenz showed that there exists an ellipsoid $E$ in phase space such that all trajectories outside $E$ eventually enter $E$ and stay there. So the attractor is bounded and compact.

---

## Fixed Points and Their Stability

The Lorenz system has three fixed points:

**1. The origin $C_0 = (0, 0, 0)$.**

This corresponds to no convection: the fluid layer is stationary. Setting all equations to zero: $\sigma(y-x) = 0 \Rightarrow y = x$; $x(\rho - z) - y = 0$; $xy - \beta z = 0$.

From $y = x$ and $xy = \beta z$: $x^2 = \beta z$. From the $\dot{y}$ equation: $x(\rho - z) - x = 0 \Rightarrow x(\rho - z - 1) = 0$. So $x = 0$ (giving the origin) or $z = \rho - 1$.

**2 and 3. The symmetric pair $C_\pm = (\pm\sqrt{\beta(\rho-1)},\ \pm\sqrt{\beta(\rho-1)},\ \rho - 1)$.**

These exist only when $\rho > 1$. Physically, they represent steady convective rolling — one direction or the other. For $\rho = 28$, these are at $(\pm\sqrt{8/3 \cdot 27}, \pm\sqrt{72}, 27) \approx (\pm 6\sqrt{2}, \pm 6\sqrt{2}, 27)$.

**Stability of the origin:** The Jacobian at $(0,0,0)$ is:

$$J_0 = \begin{pmatrix} -\sigma & \sigma & 0 \\ \rho & -1 & 0 \\ 0 & 0 & -\beta \end{pmatrix}$$

The eigenvalues are $\lambda = -\beta$ (one) and the eigenvalues of the upper-left $2\times 2$ block:

$$\lambda^2 + (\sigma + 1)\lambda + \sigma(1 - \rho) = 0$$

For $\rho < 1$: both solutions have negative real part — origin is stable. For $\rho > 1$: one eigenvalue becomes positive — origin loses stability and the convective fixed points $C_\pm$ are born. This is a **pitchfork bifurcation** at $\rho = 1$.

**Stability of $C_\pm$:** For the canonical $\rho = 28$, the Jacobian at $C_+$ has three eigenvalues: one negative real, and a complex conjugate pair with *positive* real part. The equilibria $C_\pm$ are **unstable spirals**: trajectories spiral away from them. This is crucial: both possible steady convective states are unstable, so the system can never settle into steady convection. Instead, it wanders chaotically between the neighborhoods of $C_+$ and $C_-$.

**Onset of chaos:** As $\rho$ increases from 1, the system undergoes a subcritical Hopf bifurcation at $\rho = \rho_H = \sigma(\sigma + \beta + 3)/(\sigma - \beta - 1)$. For $\sigma = 10$, $\beta = 8/3$: $\rho_H \approx 24.74$. Above $\rho_H$, $C_\pm$ are unstable and there is no stable periodic orbit — only the strange attractor.

---

## The Strange Attractor

For $\rho = 28$, $\sigma = 10$, $\beta = 8/3$, numerical integration of the Lorenz system reveals a remarkable attractor: a set that looks like a double-winged butterfly or owl face. The trajectory spirals outward from one wing's center ($C_+$ or $C_-$), then crosses over to the other wing, spirals there, crosses back, and so on indefinitely — but *never exactly repeating*.

The Lorenz attractor is a **strange attractor**: it is an attractor (trajectories from a large initial region converge to it) but it is also **fractal** (it has non-integer dimension). The trajectory on the attractor never closes up into a periodic orbit; it is dense in the attractor but aperiodic.

The fractal dimension of the Lorenz attractor has been estimated numerically at approximately $d \approx 2.06$ [Grassberger1983] — just barely above 2. It is almost a surface, but not quite. The transverse "thickness" is a Cantor set. This fractional dimension is a signature of the strange attractor's complex geometric structure.

**Quantitative characterization:** The three Lyapunov exponents of the Lorenz system (at canonical parameters) are approximately [Sparrow1982]:

$$\lambda_1 \approx +0.906, \quad \lambda_2 \approx 0, \quad \lambda_3 \approx -14.57$$

The positive $\lambda_1$ confirms chaos: nearby trajectories diverge. The zero $\lambda_2$ corresponds to the direction along the flow. The large negative $\lambda_3$ corresponds to the strong contraction perpendicular to the attractor surface. The Kaplan-Yorke dimension (Section 4.3) gives $d_{KY} = 2 + \lambda_1/|\lambda_3| \approx 2.062$, consistent with numerical estimates.

---

## Numerical Simulation

The Lorenz system is typically integrated numerically using a fourth-order Runge-Kutta scheme. The following Python code generates the Lorenz attractor:

```python
import numpy as np
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D

def lorenz_rhs(state, sigma=10.0, rho=28.0, beta=8/3):
    x, y, z = state
    dxdt = sigma * (y - x)
    dydt = x * (rho - z) - y
    dzdt = x * y - beta * z
    return np.array([dxdt, dydt, dzdt])

def rk4_step(f, state, dt):
    k1 = f(state)
    k2 = f(state + 0.5 * dt * k1)
    k3 = f(state + 0.5 * dt * k2)
    k4 = f(state + dt * k3)
    return state + (dt / 6.0) * (k1 + 2*k2 + 2*k3 + k4)

# Integrate
dt = 0.01
T = 50.0
N = int(T / dt)
trajectory = np.zeros((N, 3))
trajectory[0] = [0.1, 0.0, 0.0]  # initial condition near origin

for i in range(1, N):
    trajectory[i] = rk4_step(lorenz_rhs, trajectory[i-1], dt)

# Plot
fig = plt.figure(figsize=(10, 8))
ax = fig.add_subplot(111, projection='3d')
ax.plot(trajectory[:, 0], trajectory[:, 1], trajectory[:, 2],
        lw=0.4, alpha=0.8, color='navy')
ax.set_xlabel('x')
ax.set_ylabel('y')
ax.set_zlabel('z')
ax.set_title('Lorenz Attractor ($\\sigma=10$, $\\rho=28$, $\\beta=8/3$)')
plt.tight_layout()
plt.savefig('lorenz_attractor.png', dpi=150)
plt.show()
```

To demonstrate sensitive dependence, run two trajectories from initial conditions that differ by $10^{-8}$:

```python
x0a = np.array([0.1, 0.0, 0.0])
x0b = x0a + np.array([1e-8, 0.0, 0.0])

traj_a = np.zeros((N, 3))
traj_b = np.zeros((N, 3))
traj_a[0], traj_b[0] = x0a, x0b

for i in range(1, N):
    traj_a[i] = rk4_step(lorenz_rhs, traj_a[i-1], dt)
    traj_b[i] = rk4_step(lorenz_rhs, traj_b[i-1], dt)

separation = np.linalg.norm(traj_a - traj_b, axis=1)
time = np.arange(N) * dt

plt.figure(figsize=(8, 4))
plt.semilogy(time, separation)
plt.xlabel('Time')
plt.ylabel('Separation $|\\delta x(t)|$')
plt.title('Exponential divergence in the Lorenz system')
plt.axvline(x=1/0.906, color='r', linestyle='--', label='1 Lyapunov time')
plt.legend()
plt.tight_layout()
plt.savefig('lorenz_divergence.png', dpi=150)
plt.show()
```

The separation grows approximately as $e^{0.906 t}$ until it saturates at the attractor diameter (around $t \approx 30$).

---

## Why the Lorenz System Became the Benchmark for Temporal ML

The Lorenz system has a special status in the machine learning literature on time series. It is chaotic enough to be nontrivial but low-dimensional enough to be analyzable. Its attractor has known properties (dimension, Lyapunov exponents) that allow quantitative evaluation of learned models. And it is deterministic, so there is a "ground truth" to compare against.

The task of **Lorenz forecasting** — predicting $x(t+\tau)$ given $x(t), x(t-\Delta), x(t-2\Delta), \ldots$ — has been used as a benchmark since at least the 1990s [Gershenfeld1994] and remains in use today [Pathak2018]. Reservoir networks [Jaeger2004, Pathak2017] have been shown to forecast the Lorenz system accurately for up to $\approx 8$ Lyapunov times (compared to a theoretical maximum of, say, 20–30 Lyapunov times from a perfect model), with purely data-driven learning.

The Lorenz system is also the central example for **attractor reconstruction** via Takens embedding theorem (Chapter 3), which forms the theoretical foundation for why reservoir readouts can, in principle, recover the attractor geometry from univariate observations.

---

## Summary

The Lorenz system — a three-variable ODE derived from fluid convection physics — is the paradigmatic example of a chaotic dynamical system. Its fixed points ($C_0$ and $C_\pm$) are all unstable at canonical parameters, forcing trajectories into perpetual aperiodic wandering on the strange attractor. The attractor is fractal (dimension $\approx 2.06$), bounded, and has a positive Lyapunov exponent ($\lambda_1 \approx 0.906$), giving a predictability horizon of roughly one Lyapunov time $\approx 1.1$ time units. Its role as a benchmark for temporal machine learning makes it indispensable in the reservoir computing literature.
