# Section 2.2: Linearization and the Jacobian Matrix

## The Core Idea

When you stand very close to a fixed point $\mathbf{x}^*$, the dynamics you see is approximately linear. The nonlinear vector field $f$, when restricted to a small neighborhood of $\mathbf{x}^*$, looks like its own best linear approximation — the tangent map. The matrix that encodes this linear approximation is the **Jacobian** of $f$ evaluated at $\mathbf{x}^*$, and it is the central tool for local stability analysis.

This is the dynamical systems version of a familiar idea: near any smooth curve, you can approximate it by its tangent line. Near any smooth surface, by its tangent plane. Near any smooth vector field at a fixed point, by its linear (matrix) approximation.

---

## Taylor Expansion of the Vector Field

Let $\mathbf{x}^*$ be a fixed point of $\dot{\mathbf{x}} = f(\mathbf{x})$, so $f(\mathbf{x}^*) = \mathbf{0}$. Define the **displacement** from the fixed point:

$$\mathbf{u}(t) = \mathbf{x}(t) - \mathbf{x}^*$$

We want to find an equation for how $\mathbf{u}(t)$ evolves. Differentiating:

$$\dot{\mathbf{u}} = \dot{\mathbf{x}} = f(\mathbf{x}) = f(\mathbf{x}^* + \mathbf{u})$$

Now expand $f$ in a Taylor series around $\mathbf{x}^*$. For a function $f: \mathbb{R}^n \to \mathbb{R}^n$, the Taylor expansion is:

$$f(\mathbf{x}^* + \mathbf{u}) = f(\mathbf{x}^*) + Df(\mathbf{x}^*)\, \mathbf{u} + O(\|\mathbf{u}\|^2)$$

Here $Df(\mathbf{x}^*)$ is the **Jacobian matrix** of $f$ at $\mathbf{x}^*$: the $n \times n$ matrix whose $(i,j)$ entry is

$$[Df(\mathbf{x}^*)]_{ij} = \frac{\partial f_i}{\partial x_j}\bigg|_{\mathbf{x} = \mathbf{x}^*} \tag{2.3}$$

Since $f(\mathbf{x}^*) = \mathbf{0}$ (because $\mathbf{x}^*$ is a fixed point), the Taylor expansion gives:

$$\dot{\mathbf{u}} = Df(\mathbf{x}^*)\, \mathbf{u} + O(\|\mathbf{u}\|^2) \tag{2.4}$$

When $\|\mathbf{u}\|$ is small, the higher-order terms $O(\|\mathbf{u}\|^2)$ are negligible compared to the linear term, and we obtain the **linearized system**:

$$\dot{\mathbf{u}} \approx J\, \mathbf{u}, \qquad J = Df(\mathbf{x}^*) \tag{2.5}$$

This is a linear ODE with constant coefficient matrix $J$. Its solution is:

$$\mathbf{u}(t) = e^{Jt}\, \mathbf{u}(0) \tag{2.6}$$

where $e^{Jt}$ is the **matrix exponential**. The behavior of $\mathbf{u}(t)$ — whether it grows or shrinks — is entirely determined by the eigenvalues of $J$.

---

## The Hartman-Grobman Theorem

The linearization (2.5) is valid for small $\|\mathbf{u}\|$, but does it tell us the true behavior of the *nonlinear* system near $\mathbf{x}^*$?

The answer, for **hyperbolic** fixed points, is yes:

**Theorem (Hartman-Grobman).** *If $J = Df(\mathbf{x}^*)$ has no eigenvalue with zero real part (the fixed point is hyperbolic), then there is a homeomorphism $h$ of a neighborhood of $\mathbf{x}^*$ that conjugates the nonlinear flow $\Phi^t$ to the linear flow $e^{Jt}$:*

$$h(\Phi^t(\mathbf{x})) = e^{Jt} h(\mathbf{x}) \quad \text{for all } \mathbf{x} \text{ near } \mathbf{x}^*$$

In other words, the qualitative behavior of the nonlinear system near a hyperbolic fixed point is topologically the same as the behavior of the linearization. If the linearization says trajectories converge to $\mathbf{x}^*$, so does the nonlinear system. If it says they diverge, so does the nonlinear system.

The catch is "homeomorphism" rather than "diffeomorphism": the conjugacy may distort distances, so the spiraling speed or angle may change. But the topology — spiraling in vs. out, stable vs. unstable manifold structure — is preserved.

For **non-hyperbolic** fixed points (eigenvalues with zero real part), the linearization can be misleading. The nonlinear terms decide the stability, and we must work harder.

---

## Computing the Jacobian: Examples

### Example 1: The Pendulum

Recall the damped pendulum system:

$$\dot{\theta} = \omega, \qquad \dot{\omega} = -\alpha \sin\theta - \gamma \omega$$

The Jacobian is:

$$J = \begin{pmatrix} \partial f_1/\partial \theta & \partial f_1/\partial \omega \\ \partial f_2/\partial \theta & \partial f_2/\partial \omega \end{pmatrix} = \begin{pmatrix} 0 & 1 \\ -\alpha \cos\theta & -\gamma \end{pmatrix}$$

At the downward equilibrium $(\theta^*, \omega^*) = (0, 0)$:

$$J_{\text{down}} = \begin{pmatrix} 0 & 1 \\ -\alpha & -\gamma \end{pmatrix}$$

At the upward equilibrium $(\theta^*, \omega^*) = (\pi, 0)$:

$$J_{\text{up}} = \begin{pmatrix} 0 & 1 \\ \alpha & -\gamma \end{pmatrix}$$

Note the sign flip in the lower-left entry: $-\alpha \cos(0) = -\alpha$ vs. $-\alpha \cos(\pi) = +\alpha$. This sign flip is what makes the downward equilibrium stable and the upward one unstable (as we analyze in Section 2.3).

### Example 2: The Lotka-Volterra System

$$\dot{x} = \alpha x - \beta xy, \qquad \dot{y} = \delta xy - \gamma y$$

$$J = \begin{pmatrix} \alpha - \beta y & -\beta x \\ \delta y & \delta x - \gamma \end{pmatrix}$$

At the coexistence equilibrium $(\gamma/\delta,\ \alpha/\beta)$:

$$J^* = \begin{pmatrix} \alpha - \beta(\alpha/\beta) & -\beta(\gamma/\delta) \\ \delta(\alpha/\beta) & \delta(\gamma/\delta) - \gamma \end{pmatrix} = \begin{pmatrix} 0 & -\beta\gamma/\delta \\ \delta\alpha/\beta & 0 \end{pmatrix}$$

The trace is $\text{tr}(J^*) = 0$ and the determinant is $\det(J^*) = (\beta\gamma/\delta)(\delta\alpha/\beta) = \alpha\gamma > 0$. This means the eigenvalues are purely imaginary: $\lambda = \pm i\sqrt{\alpha\gamma}$. The linearization predicts a **center** — periodic orbits near the fixed point with frequency $\sqrt{\alpha\gamma}$.

For the Lotka-Volterra system, this is exact: the system has a conserved quantity and the orbits really are closed curves. But centers are non-hyperbolic (eigenvalues on the imaginary axis), so we cannot apply Hartman-Grobman directly. A small perturbation to the equations could convert the center to a stable or unstable spiral.

### Example 3: The Logistic Map

For the 1D discrete map $f(x) = rx(1-x)$, the Jacobian reduces to the scalar derivative:

$$f'(x) = r(1 - 2x)$$

At the fixed point $x^* = 1 - 1/r$:

$$f'(x^*) = r\left(1 - 2\left(1 - \frac{1}{r}\right)\right) = r\left(\frac{2}{r} - 1\right) = 2 - r$$

The stability of this fixed point depends on whether $|f'(x^*)| = |2 - r| < 1$, i.e., whether $1 < r < 3$. This confirms what we observed: the fixed point is stable for $1 < r < 3$, and at $r = 3$ the derivative equals $-1$ exactly — a special non-hyperbolic case that marks the onset of period doubling.

---

## Linearization for Discrete-Time Maps

The same Taylor expansion applies to maps. For $x_{t+1} = f(x_t)$ with fixed point $f(\mathbf{x}^*) = \mathbf{x}^*$, let $\mathbf{u}_t = \mathbf{x}_t - \mathbf{x}^*$. Then:

$$\mathbf{u}_{t+1} = \mathbf{x}_{t+1} - \mathbf{x}^* = f(\mathbf{x}^* + \mathbf{u}_t) - f(\mathbf{x}^*)$$

$$= Df(\mathbf{x}^*)\, \mathbf{u}_t + O(\|\mathbf{u}_t\|^2)$$

The linearized map is:

$$\mathbf{u}_{t+1} \approx J\, \mathbf{u}_t, \qquad J = Df(\mathbf{x}^*) \tag{2.7}$$

Iterating:

$$\mathbf{u}_t \approx J^t\, \mathbf{u}_0 \tag{2.8}$$

The question is: does $J^t \mathbf{u}_0 \to \mathbf{0}$ as $t \to \infty$? This is the discrete-time stability question, and it depends on the **eigenvalues of $J$** — specifically, on whether their magnitudes are less than or greater than 1.

---

## Geometric Meaning of the Jacobian

The Jacobian $J$ is a linear map that approximates $f$ near $\mathbf{x}^*$. Geometrically, it tells you how small volumes, areas, and lengths near $\mathbf{x}^*$ are stretched, compressed, and rotated by one step of the dynamics.

The **determinant** $\det(J)$ measures the volume scaling: after one time step, a small ball of volume $V$ near $\mathbf{x}^*$ becomes a region of volume approximately $|\det(J)| \cdot V$. For the continuous-time system, this is related to **Liouville's theorem**: the rate of change of volume is $\text{tr}(J)$, the divergence of the vector field.

The **eigenvalues** of $J$ measure the stretching and compression along specific directions. An eigenvalue with $|\lambda| < 1$ (discrete time) or $\text{Re}(\lambda) < 0$ (continuous time) corresponds to a direction in which the flow *contracts*: perturbations along this direction shrink. An eigenvalue with $|\lambda| > 1$ or $\text{Re}(\lambda) > 0$ corresponds to a direction where perturbations *grow*.

The **stable manifold** of $\mathbf{x}^*$ is the set of all initial conditions that converge to $\mathbf{x}^*$ as $t \to +\infty$. Near $\mathbf{x}^*$, it is tangent to the **stable eigenspace** of $J$ (the span of eigenvectors with $|\lambda| < 1$ in discrete time, or $\text{Re}(\lambda) < 0$ in continuous time).

The **unstable manifold** is the set of conditions that converge to $\mathbf{x}^*$ as $t \to -\infty$ (i.e., would escape $\mathbf{x}^*$ forward in time). Near $\mathbf{x}^*$, it is tangent to the **unstable eigenspace** of $J$.

A **saddle point** is a fixed point with both stable and unstable manifolds present: some directions contract, others expand. Trajectories generically do not converge to saddles (the stable manifold has lower dimension than the full state space), but they matter enormously for organizing the phase portrait.

---

## The Linearized System and Reservoir Design

For reservoir computing, the Jacobian matrix of the reservoir's state update function evaluated at its rest state is a direct design target. The **spectral radius** of this Jacobian — the largest eigenvalue magnitude — controls how fast perturbations grow or shrink near the rest state. A spectral radius less than 1 means the reservoir contracts: it forgets initial conditions exponentially. A spectral radius greater than 1 means the reservoir expands: small differences in initial conditions grow, and the rest state is unstable.

The "edge of stability" — a spectral radius near 1 — is often cited as an optimal operating regime for reservoirs, and the intuition comes directly from linearization. Near the edge, neither forgetting (too fast contraction) nor instability (expansion) dominates, and the reservoir retains sensitivity to input over long timescales [Jaeger2001, Legenstein2007].

This connection is made precise in Section 7 and Chapter 4.

---

## Summary

Linearization replaces the nonlinear system $\dot{\mathbf{x}} = f(\mathbf{x})$ near a fixed point $\mathbf{x}^*$ with the linear system $\dot{\mathbf{u}} = J\mathbf{u}$, where $J = Df(\mathbf{x}^*)$ is the Jacobian. The Hartman-Grobman theorem guarantees that this linear approximation correctly captures the qualitative behavior near hyperbolic fixed points. In discrete time, the linearized map is $\mathbf{u}_{t+1} = J\mathbf{u}_t$, iterated as $\mathbf{u}_t = J^t \mathbf{u}_0$. The eigenvalues of $J$ determine stability. We now turn, in Section 2.3, to the full eigenvalue classification.
