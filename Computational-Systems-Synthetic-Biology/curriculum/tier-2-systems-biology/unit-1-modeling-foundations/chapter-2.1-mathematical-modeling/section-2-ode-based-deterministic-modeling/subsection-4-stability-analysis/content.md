# Stability Analysis of Biological ODEs

## Fixed Points and Their Biological Meaning

In 2000, Tim Gardner, Charles Cantor, and Jim Collins published a paper in *Nature* that changed synthetic biology. They built a genetic toggle switch — a two-gene circuit that could exist in either of two stable states and switch between them on demand. The concept was simple: gene A represses gene B, and gene B represses gene A. But would mutual repression actually produce two stable states? Could a gene circuit remember which state it was in?

The answer was yes — but understanding why requires stability analysis. The question of whether a system will stay near a particular state, or drift away from it, or oscillate around it, is answered by the eigenvalues of the Jacobian matrix. This is not abstract mathematics; it is the precise tool for determining whether a gene circuit has memory, whether a signaling cascade will ring, and whether a metabolic pathway will reach a stable operating point.

A **fixed point** (or steady state, or equilibrium) of a dynamical system $\dot{\mathbf{x}} = \mathbf{f}(\mathbf{x})$ is a state $\mathbf{x}^*$ where all time derivatives vanish: $\mathbf{f}(\mathbf{x}^*) = 0$. Biologically, fixed points correspond to stable cell states (low or high gene expression), metabolic steady states, or resting potential in neurons.

The stability of a fixed point determines what the system does after a small perturbation. A **stable fixed point** (attractor) is one to which nearby trajectories converge. An **unstable fixed point** is one from which they diverge. Understanding stability is essential for understanding biological robustness and switching behavior.

## Linearization and the Jacobian Matrix

Near a fixed point $\mathbf{x}^*$, we can write $\mathbf{x}(t) = \mathbf{x}^* + \boldsymbol{\xi}(t)$ where $\boldsymbol{\xi}$ is a small perturbation. Taylor expanding $\mathbf{f}$ to first order:

$$\dot{\boldsymbol{\xi}} \approx J \boldsymbol{\xi}$$

where $J$ is the **Jacobian matrix** evaluated at $\mathbf{x}^*$:

$$J_{ij} = \left.\frac{\partial f_i}{\partial x_j}\right|_{\mathbf{x}^*}$$

The solution to the linearized system is:

$$\boldsymbol{\xi}(t) = \sum_k c_k \mathbf{v}_k e^{\lambda_k t}$$

where $\lambda_k$ are the **eigenvalues** of $J$ and $\mathbf{v}_k$ are the corresponding eigenvectors. The stability of the fixed point is determined entirely by the signs of the eigenvalue real parts. If all real parts are negative, every perturbation decays — the fixed point is an attractor. If any real part is positive, perturbations in that direction grow — the fixed point is unstable.

## Stability Criteria from Eigenvalues

For a general $n$-dimensional system:
- **All Re($\lambda_k$) < 0**: stable node or spiral — perturbations decay, fixed point is an attractor.
- **Any Re($\lambda_k$) > 0**: unstable — perturbations grow in at least one direction.
- **Some Re($\lambda_k$) = 0**: marginal stability — the linear analysis is inconclusive; higher-order terms determine behavior (bifurcation point).

For a **2D system** with state variables $(x, y)$, the Jacobian has two eigenvalues related to its trace $\tau = \lambda_1 + \lambda_2$ and determinant $\Delta = \lambda_1 \lambda_2$:

$$\lambda_{1,2} = \frac{\tau \pm \sqrt{\tau^2 - 4\Delta}}{2}$$

The stability classification follows from the $(\tau, \Delta)$ plane:

| Condition | Classification |
|---|---|
| $\Delta < 0$ | Saddle point (always unstable) |
| $\Delta > 0$, $\tau < 0$, $\tau^2 > 4\Delta$ | Stable node |
| $\Delta > 0$, $\tau > 0$, $\tau^2 > 4\Delta$ | Unstable node |
| $\Delta > 0$, $\tau < 0$, $\tau^2 < 4\Delta$ | Stable spiral |
| $\Delta > 0$, $\tau > 0$, $\tau^2 < 4\Delta$ | Unstable spiral |
| $\tau = 0$, $\Delta > 0$ | Center (Hopf bifurcation condition) |

**Key result**: A 2D system's fixed point is stable if and only if $\tau < 0$ and $\Delta > 0$. The trace must be negative (net damping) and the determinant positive (the two eigenvalues have the same sign, both negative). This simple criterion is the workhorse of 2D biological stability analysis.

## Worked Example: The Genetic Toggle Switch

The toggle switch (Gardner et al. 2000, *Nature*) consists of two mutually repressing genes:

$$\frac{du}{dt} = \frac{\alpha_1}{1 + v^n} - u$$

$$\frac{dv}{dt} = \frac{\alpha_2}{1 + u^n} - v$$

(using dimensionless time $t$ scaled by degradation rate). For $\alpha_1 = \alpha_2 = \alpha$ and $n = 2$, the system is symmetric. The fixed points satisfy:

$$u^* = \frac{\alpha}{1 + (v^*)^2}, \quad v^* = \frac{\alpha}{1 + (u^*)^2}$$

Numerically (for $\alpha = 4, n = 2$), the symmetric solution $u^* = v^* \approx 1.56$ is an **unstable saddle point**. Two additional asymmetric solutions exist:

- State 1: $u^* \approx 3.8$, $v^* \approx 0.3$ (gene 1 ON, gene 2 OFF)
- State 2: $u^* \approx 0.3$, $v^* \approx 3.8$ (gene 1 OFF, gene 2 ON)

The Jacobian at State 1 (using $f_1 = \alpha/(1+v^n) - u$):

$$J = \begin{pmatrix} -1 & \frac{-\alpha n (v^*)^{n-1}}{(1+(v^*)^n)^2} \\ \frac{-\alpha n (u^*)^{n-1}}{(1+(u^*)^n)^2} & -1 \end{pmatrix}$$

At State 1, the off-diagonal terms are small (since $v^* \approx 0.3$ means the repressor is at low concentration, so $\partial f_1/\partial v \approx 0$). This gives $\tau \approx -2 < 0$ and $\Delta \approx 1 > 0$ — a stable node. The toggle switch is **bistable**: two stable states exist simultaneously.

This analysis explains why the toggle switch works. When gene 1 is highly expressed, it produces a large amount of the repressor for gene 2, keeping gene 2 firmly off. The off-diagonal Jacobian entries are small because the repressor is already so effective that small perturbations in its concentration barely change the repression level. The system is locked in State 1.

```python
import numpy as np
from scipy.optimize import fsolve

def toggle_rhs(state, alpha, n):
    u, v = state
    f1 = alpha / (1 + v**n) - u
    f2 = alpha / (1 + u**n) - v
    return [f1, f2]

alpha, n = 4.0, 2.0

# Find all fixed points by scanning initial conditions
fixed_points = []
for u0 in np.linspace(0.1, alpha, 20):
    for v0 in np.linspace(0.1, alpha, 20):
        sol = fsolve(toggle_rhs, [u0, v0], args=(alpha, n), full_output=True)
        fp = sol[0]
        residual = np.max(np.abs(toggle_rhs(fp, alpha, n)))
        if residual < 1e-10:
            fixed_points.append(tuple(np.round(fp, 4)))

unique_fps = list(set(fixed_points))
print("Fixed points (u*, v*):", unique_fps)
```

## Nullcline Analysis

Nullclines provide geometric insight into the phase plane that eigenvalue calculations alone cannot give. The **$u$-nullcline** is the curve where $\dot{u} = 0$; the **$v$-nullcline** is where $\dot{v} = 0$. Fixed points are intersections of the two nullclines.

For the toggle switch, the $u$-nullcline is $u = \alpha/(1 + v^n)$ (a decreasing function of $v$) and the $v$-nullcline is $v = \alpha/(1 + u^n)$. When $n > 1$ and $\alpha$ is large enough, these curves intersect three times — one unstable middle intersection and two stable outer intersections. This geometric picture immediately reveals bistability.

The power of this approach is that you can see the bistability directly, without computing eigenvalues. The shape of the nullclines tells you: if they cross once, the system is monostable; if they cross three times (with the alternating stable-unstable-stable pattern), it is bistable. Changing a parameter — say, increasing $\alpha$ (stronger production) — reshapes the nullclines and can change the number of crossings. This is a bifurcation, the topic of the next section.

## Why This Matters

Stability analysis is the foundation of understanding cellular decision-making. Every biological circuit that exhibits a stable state — from differentiated cell identity to metabolic homeostasis to resting membrane potential — corresponds to a stable fixed point of its underlying ODE system. The Jacobian-based criterion tells us whether that state is robust to perturbations and what perturbation magnitudes are needed to switch between states.

For synthetic biology, this analysis is a design tool. Before Gardner et al. built their toggle switch, they analyzed the Jacobian and showed analytically that mutual repression with sufficient nonlinearity ($n \geq 1$ with appropriate $\alpha$) would produce bistability. The mathematics predicted the biology. That predictive power — knowing from equations alone that a circuit will have memory — is what stability analysis gives you.
