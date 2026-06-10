# Section 2.3: Eigenvalue Stability Analysis

## The Solution of the Linearized System

We established in Section 2.2 that the behavior near a fixed point is governed by the linearized system. In continuous time:

$$\dot{\mathbf{u}} = J\mathbf{u} \tag{3.1}$$

with solution $\mathbf{u}(t) = e^{Jt}\mathbf{u}(0)$. In discrete time:

$$\mathbf{u}_{t+1} = J\mathbf{u}_t \tag{3.2}$$

with solution $\mathbf{u}_t = J^t\mathbf{u}_0$.

In both cases, the long-time behavior is determined by the eigenvalues of $J$. Let us derive this precisely.

Suppose $J$ is diagonalizable (the generic case) with eigenvalues $\lambda_1, \ldots, \lambda_n$ and corresponding eigenvectors $\mathbf{v}_1, \ldots, \mathbf{v}_n$ forming a basis of $\mathbb{R}^n$ (or $\mathbb{C}^n$). Write the initial condition in this basis:

$$\mathbf{u}(0) = \sum_{k=1}^n c_k \mathbf{v}_k$$

Then, in continuous time:

$$e^{Jt}\mathbf{u}(0) = \sum_{k=1}^n c_k e^{\lambda_k t} \mathbf{v}_k \tag{3.3}$$

In discrete time:

$$J^t \mathbf{u}_0 = \sum_{k=1}^n c_k \lambda_k^t \mathbf{v}_k \tag{3.4}$$

The conclusion is immediate:

- **Continuous time:** The magnitude of the $k$-th component grows as $e^{\text{Re}(\lambda_k) t}$. The component shrinks if $\text{Re}(\lambda_k) < 0$, grows if $\text{Re}(\lambda_k) > 0$, and oscillates without growth or decay if $\text{Re}(\lambda_k) = 0$.

- **Discrete time:** The magnitude of the $k$-th component grows as $|\lambda_k|^t$. It shrinks if $|\lambda_k| < 1$, grows if $|\lambda_k| > 1$, and oscillates without growth or decay if $|\lambda_k| = 1$.

A fixed point is **stable** if all components of $\mathbf{u}(t)$ decay to zero — which requires $\text{Re}(\lambda_k) < 0$ for all $k$ (continuous time) or $|\lambda_k| < 1$ for all $k$ (discrete time). If any eigenvalue violates this condition, the fixed point is **unstable**.

---

## Classification of Fixed Points in 2D Continuous-Time Systems

For a 2D system, $J$ is a $2 \times 2$ matrix with two eigenvalues $\lambda_1, \lambda_2 \in \mathbb{C}$. The eigenvalues satisfy the characteristic equation:

$$\lambda^2 - \text{tr}(J)\, \lambda + \det(J) = 0$$

$$\lambda_{1,2} = \frac{\text{tr}(J) \pm \sqrt{\text{tr}(J)^2 - 4\det(J)}}{2} \tag{3.5}$$

Let $\tau = \text{tr}(J)$ and $\Delta = \det(J)$. The discriminant $\tau^2 - 4\Delta$ determines whether the eigenvalues are real or complex.

The full classification is given in the following table. Let $T = \tau$ (trace) and $D = \Delta$ (determinant).

| Region | Eigenvalues | Fixed Point Type | Stability |
|--------|-------------|------------------|-----------|
| $D < 0$ | Real, opposite signs | **Saddle** | Unstable |
| $D > 0$, $T^2 - 4D > 0$, $T < 0$ | Real, both negative | **Stable node** | Stable |
| $D > 0$, $T^2 - 4D > 0$, $T > 0$ | Real, both positive | **Unstable node** | Unstable |
| $D > 0$, $T^2 - 4D < 0$, $T < 0$ | Complex, $\text{Re}(\lambda) < 0$ | **Stable spiral** | Stable |
| $D > 0$, $T^2 - 4D < 0$, $T > 0$ | Complex, $\text{Re}(\lambda) > 0$ | **Unstable spiral** | Unstable |
| $D > 0$, $T = 0$ | Purely imaginary | **Center** | Neutral |
| $T^2 - 4D = 0$, $T < 0$ | Real repeated, negative | **Stable star/degenerate node** | Stable |
| $T^2 - 4D = 0$, $T > 0$ | Real repeated, positive | **Unstable star/degenerate node** | Unstable |

The case $D = 0$ corresponds to a zero eigenvalue and is not a hyperbolic fixed point (the linearization is degenerate).

The dividing curves in the $(T, D)$ plane are:
- The $T$-axis ($D = 0$): one eigenvalue is zero.
- The parabola $D = T^2/4$: repeated eigenvalues.
- The $D$-axis ($T = 0$, $D > 0$): purely imaginary eigenvalues (center).

### Geometric Descriptions

**Stable node:** Both eigenvalues real and negative. Trajectories approach $\mathbf{x}^*$ along straight lines (the eigenvector directions), with the slower decay mode dominating at large $t$. The node looks like a starfish pattern converging to the fixed point.

**Stable spiral:** Complex eigenvalues with negative real part. Trajectories spiral inward like water draining. The imaginary part $\text{Im}(\lambda)$ is the angular frequency of the spiral.

**Center:** Purely imaginary eigenvalues. Trajectories are closed ellipses around the fixed point. This is the borderline case — infinitesimally close to spiraling in or out. The Lotka-Volterra coexistence equilibrium is a center.

**Saddle point:** One positive, one negative eigenvalue. Trajectories approach along the stable eigenvector direction and flee along the unstable eigenvector direction. A saddle looks like a mountain pass: approach from two sides, but departure in two other directions. The stable and unstable manifolds are 1D curves meeting at the saddle.

**Unstable spiral:** Complex eigenvalues with positive real part. Trajectories spiral outward. The van der Pol oscillator's origin is an unstable spiral for $\mu > 0$: the limit cycle is the destination.

---

## Derivation of Stability Conditions for Discrete-Time Maps

For discrete-time maps, the stability criterion $|\lambda_k| < 1$ for all eigenvalues can be expressed in terms of the trace and determinant, but the analysis is slightly different.

For a $2 \times 2$ Jacobian $J$ with eigenvalues $\lambda_{1,2}$:

$$\lambda_1 + \lambda_2 = \tau = \text{tr}(J)$$
$$\lambda_1 \lambda_2 = \Delta = \det(J)$$

We want both $|\lambda_1| < 1$ and $|\lambda_2| < 1$.

**Case 1: Real eigenvalues.** We need $-1 < \lambda_k < 1$ for each $k$. The relevant conditions are:

From $\lambda_1 \lambda_2 = \Delta$ and $\lambda_1 + \lambda_2 = \tau$, the conditions $|\lambda_{1,2}| < 1$ are equivalent to:

- $|\Delta| < 1$ (product of eigenvalues inside unit circle)
- $|\tau| < 1 + \Delta$ (trace condition)

More precisely, the **Schur-Cohn stability conditions** for a $2\times 2$ matrix state that both eigenvalues lie strictly inside the unit disk if and only if:

$$|\tau| < 1 + \Delta \quad \text{and} \quad |\Delta| < 1 \tag{3.6}$$

**Derivation.** For real eigenvalues, we want $-1 < \lambda_k < 1$. Using the eigenvalue expressions:

$$\lambda_k = \frac{\tau \pm \sqrt{\tau^2 - 4\Delta}}{2}$$

The condition $\lambda_k < 1$ means $\tau \pm \sqrt{\tau^2 - 4\Delta} < 2$, i.e., $\tau - 2 < \mp\sqrt{\tau^2 - 4\Delta}$.

Rather than chasing this algebra, use the fact that the characteristic polynomial is $p(\lambda) = \lambda^2 - \tau\lambda + \Delta$. Both roots are inside $(-1, 1)$ if and only if:

- $p(1) = 1 - \tau + \Delta > 0$, i.e., $\tau < 1 + \Delta$
- $p(-1) = 1 + \tau + \Delta > 0$, i.e., $\tau > -(1 + \Delta)$
- $p(0) = \Delta < 1$ (leading coefficient times constant term condition, which here just gives $|\Delta| < 1$)

These three conditions together give (3.6). The geometric picture: the point $(\tau, \Delta)$ must lie inside the triangular region bounded by $\Delta = 1$, $\tau = 1 + \Delta$, and $\tau = -(1 + \Delta)$ in the $(\tau, \Delta)$ plane. This is the discrete-time analogue of the stability region in the trace-determinant plane.

**Case 2: Complex eigenvalues.** We have $\lambda_{1,2} = a \pm bi$ where $a = \tau/2$ and $a^2 + b^2 = \Delta$. The modulus is $|\lambda_k| = \sqrt{a^2 + b^2} = \sqrt{\Delta}$. So the stability condition is simply $\Delta < 1$.

**Summary for discrete time:** The fixed point is stable if and only if all eigenvalues satisfy $|\lambda_k| < 1$, equivalently all eigenvalues lie inside the unit circle in $\mathbb{C}$. For the $2\times 2$ case, this reduces to the Schur-Cohn conditions (3.6).

---

## The Spectral Radius

The **spectral radius** of a matrix $A$ is:

$$\rho(A) = \max_k |\lambda_k| \tag{3.7}$$

the maximum modulus of its eigenvalues. For discrete-time systems, the fixed point is stable if and only if $\rho(J) < 1$.

The spectral radius has a beautiful characterization in terms of matrix norms:

$$\rho(A) = \lim_{t \to \infty} \|A^t\|^{1/t} \tag{3.8}$$

for any matrix norm $\|\cdot\|$. This is Gelfand's formula, and it says: the spectral radius measures the long-run growth rate of iterates of the matrix, regardless of which direction in state space you look.

For reservoir computing, the spectral radius of the reservoir weight matrix $W^{\text{res}}$ is a standard design parameter. The intuition from linearization: at the autonomous fixed point, the Jacobian of the reservoir's state update function is approximately $\sigma'(W^{\text{res}} x^*) \cdot W^{\text{res}}$ (where $\sigma'$ is the derivative of the nonlinearity). When the spectral radius of $W^{\text{res}}$ is less than 1/max($\sigma'$), the fixed point is stable and the reservoir contracts. The threshold $\rho(W^{\text{res}}) = 1$ (with the nonlinearity accounted for) marks the onset of instability.

---

## Example: Stability of the Damped Pendulum's Equilibria

Returning to the pendulum Jacobians from Section 2.2:

**At the downward equilibrium $(\theta, \omega) = (0, 0)$:**

$$J = \begin{pmatrix} 0 & 1 \\ -\alpha & -\gamma \end{pmatrix}, \quad \tau = -\gamma,\ \Delta = \alpha$$

Both $\tau < 0$ (since $\gamma > 0$) and $\Delta = \alpha > 0$. The eigenvalues are:

$$\lambda_{1,2} = \frac{-\gamma \pm \sqrt{\gamma^2 - 4\alpha}}{2}$$

- If $\gamma^2 > 4\alpha$ (overdamped): both eigenvalues real and negative — **stable node**.
- If $\gamma^2 < 4\alpha$ (underdamped): complex eigenvalues with $\text{Re}(\lambda) = -\gamma/2 < 0$ — **stable spiral**.
- If $\gamma^2 = 4\alpha$ (critically damped): repeated real negative eigenvalue — **stable degenerate node**.

In all cases with $\gamma > 0$: **stable**. The pendulum returns to hanging down.

**At the upward equilibrium $(\theta, \omega) = (\pi, 0)$:**

$$J = \begin{pmatrix} 0 & 1 \\ \alpha & -\gamma \end{pmatrix}, \quad \tau = -\gamma,\ \Delta = -\alpha < 0$$

Since $\Delta < 0$, the eigenvalues are real with opposite signs: **saddle point**. Unstable. The pendulum balanced upright is an unstable saddle: almost all trajectories starting near it will ultimately swing away to one side or the other.

---

## Non-Hyperbolic Fixed Points and Higher-Order Analysis

When a fixed point is non-hyperbolic — when some eigenvalue has zero real part (continuous time) or modulus exactly 1 (discrete time) — the linearization is inconclusive. The quadratic and higher-order terms in the Taylor expansion determine stability.

The standard tool is the **center manifold theorem** and normal form theory [Guckenheimer1983]. The idea: the behavior near a non-hyperbolic fixed point is governed by the dynamics on the **center manifold** (the invariant manifold tangent to the center eigenspace). This center manifold has a lower dimension than the full state space and inherits the non-hyperbolic eigenvalues. The dynamics on the center manifold can be analyzed using normal form reductions.

For example, the logistic map at $r = 3$ has the fixed point $x^* = 2/3$ with derivative $f'(x^*) = -1$ — exactly on the unit circle. The next-order analysis (quadratic and cubic terms of $f^2(x)$ near $x^*$) determines the period-doubling bifurcation.

This is the subject of Section 5.

---

## Summary

The eigenvalues of the Jacobian $J = Df(\mathbf{x}^*)$ completely determine local stability:
- **Continuous time:** stable if $\text{Re}(\lambda_k) < 0$ for all $k$.
- **Discrete time:** stable if $|\lambda_k| < 1$ for all $k$.

In 2D, the trace-determinant plane provides a complete map of fixed-point types: stable and unstable nodes, spirals, centers, and saddles. For discrete time, the Schur-Cohn conditions (3.6) replace the trace-determinant classification. The spectral radius is the key quantity for reservoir design: it measures the long-run growth rate of the linearized dynamics and determines whether the reservoir contracts (useful) or diverges (unstable).
