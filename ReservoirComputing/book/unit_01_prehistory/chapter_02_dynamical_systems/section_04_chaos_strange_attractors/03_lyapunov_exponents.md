# Section 4.3: Lyapunov Exponents

## Motivation

In Section 4.1 we identified the key signature of chaos: nearby trajectories diverge *exponentially*. But how fast, exactly? And in how many directions does the divergence occur? The answer is provided by **Lyapunov exponents** — real numbers that measure the average exponential rates at which infinitesimally close trajectories separate (or converge) in different directions.

Lyapunov exponents are one of the most powerful diagnostic tools in nonlinear dynamics. They quantify chaos (a positive Lyapunov exponent is essentially the defining condition), measure predictability horizons (the inverse of the largest Lyapunov exponent is the e-folding time for error growth), and characterize the geometry of strange attractors via the Kaplan-Yorke conjecture. For reservoir computing, they are the theoretical foundation for analyzing when a reservoir is in a useful operating regime.

---

## Definition: The Maximum Lyapunov Exponent

Consider a trajectory $\mathbf{x}(t)$ and a nearby trajectory $\mathbf{x}(t) + \boldsymbol{\delta}(t)$, where $\boldsymbol{\delta}(t)$ is an infinitesimal displacement vector. The evolution of $\boldsymbol{\delta}(t)$ is governed by the linearized equation (the variational equation):

$$\dot{\boldsymbol{\delta}} = J(\mathbf{x}(t))\, \boldsymbol{\delta} \tag{4.5}$$

where $J(\mathbf{x}(t)) = Df(\mathbf{x}(t))$ is the Jacobian of $f$ evaluated along the trajectory. This is a time-varying linear ODE — the coefficient matrix $J(\mathbf{x}(t))$ changes as $\mathbf{x}(t)$ moves along the attractor.

The **maximum Lyapunov exponent** $\lambda_{\max}$ is defined as:

$$\lambda_{\max} = \lim_{t \to \infty} \frac{1}{t} \ln \frac{\|\boldsymbol{\delta}(t)\|}{\|\boldsymbol{\delta}(0)\|} \tag{4.6}$$

provided this limit exists. By Oseledets' multiplicative ergodic theorem [Oseledets1968], for almost all initial conditions on an ergodic attractor, this limit exists and takes a definite value (independent of the initial condition and, for almost all initial displacement directions $\boldsymbol{\delta}(0)/\|\boldsymbol{\delta}(0)\|$).

A more transparent equivalent form: if we define the growth factor $R(t) = \|\boldsymbol{\delta}(t)\| / \|\boldsymbol{\delta}(0)\|$, then

$$\lambda_{\max} = \lim_{t \to \infty} \frac{\ln R(t)}{t}$$

So $\lambda_{\max}$ is the *time-averaged* logarithmic growth rate of an infinitesimal perturbation. Positive $\lambda_{\max}$ means average exponential growth — chaos. Negative means average exponential contraction — stability.

---

## The Full Lyapunov Spectrum

An $n$-dimensional dynamical system has $n$ Lyapunov exponents $\lambda_1 \geq \lambda_2 \geq \cdots \geq \lambda_n$.

These arise because different directions in tangent space grow or shrink at different rates. The Oseledets theorem guarantees the existence of a **Lyapunov decomposition**: for almost every initial condition, the tangent space $T_{\mathbf{x}_0}\mathbb{R}^n$ decomposes into a nested sequence of subspaces $V_1 \supset V_2 \supset \cdots \supset V_n$ such that if $\boldsymbol{\delta}(0) \in V_k \setminus V_{k+1}$, then

$$\lim_{t \to \infty} \frac{1}{t} \ln \|\boldsymbol{\delta}(t)\| = \lambda_k$$

The exponent $\lambda_k$ measures the growth rate in the "most stable" direction in $V_k$.

The Lyapunov exponents are ordered and satisfy several properties:

1. **Sum formula.** For continuous-time systems, the sum of all Lyapunov exponents equals the time-averaged divergence of the vector field:

$$\sum_{k=1}^n \lambda_k = \langle \nabla \cdot f \rangle_{\text{time avg}} = \int (\nabla \cdot f) d\mu \tag{4.7}$$

where $\mu$ is the natural invariant measure on the attractor. For the Lorenz system, $\nabla \cdot f = -(\sigma + 1 + \beta) \approx -13.67$, which must equal $\lambda_1 + \lambda_2 + \lambda_3 \approx 0.906 + 0 + (-14.57) = -13.66$. ✓

2. **Flow direction.** For a smooth continuous-time flow, the direction *along* the trajectory is neither expanding nor contracting on average, so one Lyapunov exponent is exactly zero: $\lambda_j = 0$ for some $j$.

3. **Dissipative systems.** If the system is dissipative ($\nabla \cdot f < 0$), then $\sum \lambda_k < 0$: the sum is negative. This is consistent with the attractor having zero volume.

---

## Derivation and Geometric Interpretation

Let us work through the definition more carefully. At time $t$, the growth of a small displacement $\boldsymbol{\delta}(0)$ is given by the evolution of the variational equation (4.5). The formal solution is:

$$\boldsymbol{\delta}(t) = M(t)\, \boldsymbol{\delta}(0)$$

where $M(t)$ is the **fundamental matrix** (or monodromy matrix) satisfying $\dot{M} = J(\mathbf{x}(t)) M$, $M(0) = I$.

The growth rate of $\|\boldsymbol{\delta}(t)\|$ depends on the direction of $\boldsymbol{\delta}(0)$. For generic directions, the growth is dominated by the largest eigenvalue. The Lyapunov exponents are the logarithms of the singular values of $M(t)$, normalized by $t$:

$$\lambda_k = \lim_{t \to \infty} \frac{1}{t} \ln \sigma_k(M(t)) \tag{4.8}$$

where $\sigma_1(M(t)) \geq \sigma_2(M(t)) \geq \cdots$ are the singular values of $M(t)$.

Why singular values rather than eigenvalues? Because the eigenvalues of $M(t)$ can be complex and the growth of *lengths* (measured by the Euclidean norm) is governed by singular values. However, for the maximum exponent and for normal matrices, both give the same answer.

The geometric picture: at time $t$, a small sphere of initial conditions $\{\boldsymbol{\delta}(0) : \|\boldsymbol{\delta}(0)\| = \varepsilon\}$ has been mapped to an ellipsoid. The principal axes of this ellipsoid have lengths $\varepsilon \sigma_1(M(t)), \ldots, \varepsilon \sigma_n(M(t))$. The Lyapunov exponents measure how fast these principal axes grow (or shrink) on average.

The longest axis grows at rate $e^{\lambda_1 t}$: it corresponds to the most unstable direction. The shortest axis shrinks at rate $e^{\lambda_n t}$ (with $\lambda_n < 0$ for dissipative systems). The shape of the ellipsoid encodes all $n$ Lyapunov exponents.

---

## Numerical Computation: QR Method

Computing Lyapunov exponents numerically faces a practical obstacle: the matrix $M(t)$ tends to become ill-conditioned because the singular values grow and shrink at exponential rates, eventually causing numerical overflow/underflow and loss of orthogonality in the basis vectors.

The standard solution is the **QR algorithm** for Lyapunov exponents, due to Benettin et al. [Benettin1980]:

1. Initialize $n$ orthonormal vectors $\{\mathbf{q}_1^{(0)}, \ldots, \mathbf{q}_n^{(0)}\}$.
2. Integrate the system and the variational equation forward for a time interval $\tau$.
3. After each interval, the evolved vectors $\{\mathbf{q}_k^{(t)}\}$ have been stretched and tilted. Apply **Gram-Schmidt orthonormalization** (QR decomposition): write the matrix of evolved vectors as $Q R$, where $Q$ is orthogonal and $R$ is upper triangular. Record $\ln R_{kk}$ (the logarithm of each diagonal entry of $R$, which measures how much the $k$-th direction has been stretched).
4. Reset the vectors to $Q$ (the orthonormal basis) and repeat.
5. The $k$-th Lyapunov exponent is the long-time average of $(\ln R_{kk})/\tau$.

The re-orthonormalization at step 3 prevents numerical collapse: by periodically resetting to an orthonormal basis, we maintain numerical stability while accumulating the growth factors.

```python
def lyapunov_exponents_lorenz(sigma=10., rho=28., beta=8/3,
                               T=1000., dt=0.01, tau=1.0,
                               x0=None):
    """
    Estimate Lyapunov exponents of the Lorenz system using QR method.
    T: total integration time
    tau: re-orthonormalization interval
    """
    import numpy as np
    
    def lorenz(state):
        x, y, z = state
        return np.array([sigma*(y-x), x*(rho-z)-y, x*y-beta*z])
    
    def lorenz_jacobian(state):
        x, y, z = state
        return np.array([[-sigma, sigma, 0],
                         [rho-z, -1, -x],
                         [y, x, -beta]])
    
    def rk4(f, s, h):
        k1=f(s); k2=f(s+.5*h*k1); k3=f(s+.5*h*k2); k4=f(s+h*k3)
        return s + h*(k1+2*k2+2*k3+k4)/6
    
    n = 3
    steps_per_tau = int(tau / dt)
    num_tau = int(T / tau)
    
    if x0 is None:
        x0 = np.array([1., 1., 1.])
    
    state = x0.copy()
    Q = np.eye(n)          # orthonormal tangent vectors
    exponents = np.zeros(n)
    
    for _ in range(num_tau):
        # Integrate for time tau, simultaneously evolving state and Q
        for _ in range(steps_per_tau):
            J = lorenz_jacobian(state)
            state = rk4(lorenz, state, dt)
            # Evolve each column of Q under the linearized equations
            Q = Q + dt * (J @ Q)  # Euler step for tangent vectors
            # (in practice, use RK4 for the full system + tangent space)
        
        # QR decomposition to re-orthonormalize
        Q, R = np.linalg.qr(Q)
        # Ensure sign convention: R diagonal positive
        signs = np.sign(np.diag(R))
        Q = Q * signs
        R = (R.T * signs).T
        
        # Accumulate log growth rates
        exponents += np.log(np.abs(np.diag(R)))
    
    exponents /= T
    return np.sort(exponents)[::-1]  # sorted descending

# Usage:
# les = lyapunov_exponents_lorenz()
# print(f"Lyapunov exponents: {les}")
# Expected: approximately [0.906, 0.0, -14.57]
```

---

## The Kaplan-Yorke Conjecture

The Kaplan-Yorke conjecture [Kaplan1979] relates the Lyapunov exponents to the **fractal dimension** of the attractor.

Order the Lyapunov exponents $\lambda_1 \geq \lambda_2 \geq \cdots \geq \lambda_n$. Let $j$ be the largest index such that

$$\sum_{k=1}^j \lambda_k \geq 0 \tag{4.9}$$

That is, $j$ is the number of exponents you can "add up" before the sum goes negative. Then the **Kaplan-Yorke dimension** (also called the Lyapunov dimension) is:

$$d_{KY} = j + \frac{\lambda_1 + \lambda_2 + \cdots + \lambda_j}{|\lambda_{j+1}|} \tag{4.10}$$

**Geometric interpretation.** The formula interpolates between integer dimensions. $j$ is the largest integer dimension for which the sum of expansion rates is still non-negative — meaning an infinitesimal $j$-dimensional volume element on the attractor does not contract. The fractional part $(\sum_{k=1}^j \lambda_k)/|\lambda_{j+1}|$ measures "how close" the $(j+1)$-th direction is to contributing net expansion. It is a measure of the "thickness" in the $(j+1)$-th direction.

**For the Lorenz system:** $\lambda_1 \approx 0.906$, $\lambda_2 \approx 0$, $\lambda_3 \approx -14.57$.

We have $j = 2$ because $\lambda_1 + \lambda_2 \approx 0.906 > 0$ but $\lambda_1 + \lambda_2 + \lambda_3 \approx -13.66 < 0$.

$$d_{KY} = 2 + \frac{\lambda_1 + \lambda_2}{|\lambda_3|} = 2 + \frac{0.906 + 0}{14.57} \approx 2.062$$

This agrees well with numerical estimates of the Hausdorff dimension of the Lorenz attractor ($d_H \approx 2.06$), supporting the conjecture.

The Kaplan-Yorke conjecture is exactly that — a conjecture. It has been proven for some classes of systems and for some types of attractors, but a general proof remains open. Nevertheless, it is empirically accurate for a wide range of chaotic systems and provides an elegant formula connecting the dynamical (Lyapunov) and geometric (dimension) properties of strange attractors.

---

## Lyapunov Exponents and Predictability

The connection between the maximum Lyapunov exponent and predictability is made precise by equation (4.3) from Section 4.1:

$$T_{\text{predict}} \approx \frac{1}{\lambda_{\max}} \ln \frac{\delta}{\varepsilon}$$

where $\varepsilon$ is the initial measurement uncertainty and $\delta$ is the acceptable prediction error. This formula has a striking implication: *the predictability horizon grows only logarithmically with measurement precision*.

If you reduce your measurement error by a factor of $10^6$ (six orders of magnitude improvement), the predictability horizon grows by only $6 \ln(10) / \lambda_{\max} \approx 13.8 / \lambda_{\max}$. For the atmosphere ($\lambda_{\max}^{-1} \approx 5$ days), this means 69 extra days of predictability — from about 2 weeks to about 3 months. And this is the *theoretical maximum*; achieving it requires a perfect model.

This is the deep reason weather forecasting cannot be extended indefinitely by better data.

---

## Lyapunov Exponents for Reservoir Computing

For a reservoir computer, the Lyapunov exponents of the driven reservoir — the combined dynamical system of reservoir + input — are a key diagnostic.

Let $\mathbf{x}_{t+1} = F(\mathbf{x}_t, u_t)$ be the driven reservoir update. For a fixed input $u_t = 0$ (or constant), the Lyapunov spectrum characterizes the autonomous dynamics. As the reservoir operates, the **conditional Lyapunov exponents** (defined for the driven system with fixed input) determine whether the reservoir response is stable. The echo state property is equivalent to the condition that all conditional Lyapunov exponents are negative [Jaeger2001].

In practice, monitoring the maximum Lyapunov exponent of the reservoir — computed from the matrix of linearized state updates along the trajectory — gives a real-time diagnostic: if it approaches zero from below, the reservoir is operating at the "edge of stability" where memory is maximal; if it becomes positive, the reservoir is chaotic and the echo state property is lost.

This connection between Lyapunov exponents and reservoir performance is developed quantitatively in Chapter 4.

---

## Summary

The Lyapunov exponent spectrum $\lambda_1 \geq \cdots \geq \lambda_n$ measures the average exponential growth rates of perturbations in different directions. The maximum exponent $\lambda_{\max}$ characterizes sensitivity to initial conditions and sets the predictability horizon via $T_{\text{predict}} \approx \lambda_{\max}^{-1} \ln(\delta/\varepsilon)$. The sum of all exponents equals the time-averaged phase space contraction rate. The Kaplan-Yorke formula relates the Lyapunov spectrum to the fractal dimension of the attractor. Numerically, the QR algorithm computes all exponents stably. For reservoirs, the conditional Lyapunov spectrum determines the echo state property and the operating regime.
