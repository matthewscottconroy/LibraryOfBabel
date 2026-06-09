# Lyapunov Exponents

Two nearby orbits in a dynamical system may converge, diverge, or do something in between depending on the direction of initial separation. Lyapunov exponents quantify the long-time average rate of exponential growth or decay of small perturbations. A positive Lyapunov exponent is the defining characteristic of chaos: it means that nearby orbits diverge exponentially on average, making long-term prediction impossible in practice despite the deterministic character of the equations.

## Definition

Let $\phi_t$ be the flow of $\dot{x} = F(x)$ on $\mathbb{R}^n$. The **Lyapunov exponent** in the direction of a tangent vector $v \in T_{x_0}\mathbb{R}^n \cong \mathbb{R}^n$ is

$$\lambda(x_0, v) = \lim_{t \to \infty} \frac{1}{t} \log \|D\phi_t(x_0) \cdot v\|,$$

when this limit exists. Here $D\phi_t(x_0)$ is the Jacobian of the flow (the fundamental matrix $Y(t)$ satisfying $\dot{Y} = DF(\phi_t(x_0))Y$, $Y(0) = I$).

Intuitively, if $x_0 + \varepsilon v$ is a perturbed initial condition, then $\|\phi_t(x_0 + \varepsilon v) - \phi_t(x_0)\| \approx \varepsilon e^{\lambda(x_0,v) t}$ for small $\varepsilon$. If $\lambda > 0$, perturbations in the direction $v$ grow exponentially; if $\lambda < 0$, they shrink.

## The Oseledets Multiplicative Ergodic Theorem

The existence of Lyapunov exponents for almost all initial conditions is far from obvious—the limit may not exist for all $v$ and $x_0$. The fundamental theorem is:

**Theorem (Oseledets, 1968).** Let $\phi_t$ be a $C^1$ flow preserving an ergodic probability measure $\mu$. For $\mu$-almost every $x_0$, there exist real numbers $\lambda_1 > \lambda_2 > \cdots > \lambda_k$ (the **Lyapunov spectrum**) and a filtration

$$\mathbb{R}^n = V_1(x_0) \supset V_2(x_0) \supset \cdots \supset V_k(x_0) \supset \{0\}$$

such that for every $v \in V_i(x_0) \setminus V_{i+1}(x_0)$,

$$\lim_{t \to \infty} \frac{1}{t} \log \|D\phi_t(x_0) \cdot v\| = \lambda_i.$$

The subspaces $V_i(x_0)$ are measurable in $x_0$ and invariant in the sense that $D\phi_t(x_0) V_i(x_0) = V_i(\phi_t(x_0))$.

The key insight of the Oseledets theorem is that the Lyapunov exponents are defined $\mu$-almost everywhere and are constant (equal to $\lambda_1, \ldots, \lambda_k$) as $x_0$ varies, because $\mu$ is ergodic. Different ergodic measures can give different Lyapunov spectra.

## Computation via the Variational Equation

In practice, Lyapunov exponents are computed numerically using the **QR method** (or Gram-Schmidt orthogonalization). The idea is that directly exponentiating $D\phi_t$ leads to overflow/underflow because some directions grow and others shrink; periodically orthonormalizing the columns of the fundamental matrix separates the growth rates.

**Algorithm:**
1. Initialize $Y_0 = I$ (the $n \times n$ identity matrix).
2. Integrate $\dot{Y} = DF(\phi_t(x_0)) Y$ for a time interval $[0, \tau]$.
3. Perform QR decomposition: $Y(\tau) = Q_1 R_1$ where $Q_1$ is orthogonal and $R_1$ upper triangular.
4. Record $\log|{(R_1)_{ii}}|$ for each $i$ as the contribution to $\lambda_i$.
5. Restart with $Y = Q_1$ and repeat.

After $N$ steps, estimate $\lambda_i \approx \frac{1}{N\tau} \sum_{j=1}^N \log|(R_j)_{ii}|$.

This algorithm is numerically stable because it prevents any single direction from dominating. It converges to the true Lyapunov exponents by the Oseledets theorem.

## Lyapunov Exponents of the Lorenz System

For the Lorenz system at $\sigma = 10$, $b = 8/3$, $r = 28$, numerical computation gives:

$$\lambda_1 \approx 0.906, \quad \lambda_2 = 0, \quad \lambda_3 \approx -14.572.$$

Interpretation:
- $\lambda_1 > 0$: chaos, exponential divergence in one direction.
- $\lambda_2 = 0$: the flow direction (perturbations along the orbit neither grow nor shrink).
- $\lambda_3 < 0$: strong contraction transverse to the attractor.

The sum $\lambda_1 + \lambda_2 + \lambda_3 \approx -13.67 \approx -(\sigma + 1 + b) = -41/3 \approx -13.67$, consistent with Liouville's formula: the average rate of volume contraction equals $\text{div}\, F = -(\sigma + 1 + b)$.

## The Kaplan-Yorke Formula

The **Kaplan-Yorke conjecture** (now established as a theorem in many cases) relates the Lyapunov exponents to the fractal dimension of the attractor. Define $j$ as the largest integer such that

$$\lambda_1 + \lambda_2 + \cdots + \lambda_j \geq 0.$$

The **Lyapunov dimension** (Kaplan-Yorke dimension) is

$$d_{KY} = j + \frac{\lambda_1 + \cdots + \lambda_j}{|\lambda_{j+1}|}.$$

For the Lorenz attractor: $j = 2$ (since $\lambda_1 + \lambda_2 \approx 0.906 > 0$ but $\lambda_1 + \lambda_2 + \lambda_3 \approx -13.67 < 0$), so

$$d_{KY} = 2 + \frac{\lambda_1 + \lambda_2}{|\lambda_3|} = 2 + \frac{0.906}{14.572} \approx 2.062.$$

This matches the independently estimated Hausdorff dimension of the Lorenz attractor, supporting the Kaplan-Yorke conjecture.

## Lyapunov Exponents and Predictability

The inverse of the largest Lyapunov exponent $\lambda_1^{-1}$ gives a characteristic time scale for the loss of predictability: the **Lyapunov time**. For the Lorenz system, $\lambda_1^{-1} \approx 1.1$ dimensionless time units. In meteorological applications with typical parameters, this corresponds to a predictability horizon of the order of a few days, consistent with the practical experience that weather forecasts become unreliable beyond a week.

More precisely, if the initial condition is known with uncertainty $\varepsilon$, then after time $t$ the uncertainty has grown to approximately $\varepsilon e^{\lambda_1 t}$. The forecast becomes useless when this exceeds the size of the attractor $L$, i.e., when $t \geq \lambda_1^{-1} \log(L/\varepsilon)$. Improving measurements by an order of magnitude ($\varepsilon \to \varepsilon/10$) extends the predictability horizon by only $\lambda_1^{-1} \log 10 \approx 2.5$ additional time units—a logarithmic gain, which is vanishingly small for large $t$.

## Connection to Entropy

The **metric entropy** (Kolmogorov-Sinai entropy) of a measure-preserving system is related to Lyapunov exponents by Pesin's formula: for an ergodic measure $\mu$ and a $C^{1+\alpha}$ system,

$$h_\mu = \sum_{\lambda_i > 0} \lambda_i,$$

where the sum is over all positive Lyapunov exponents. For the Lorenz attractor, $h \approx \lambda_1 \approx 0.906$ bits per unit time. The entropy measures the rate at which the orbit generates information: each unit of time produces about $\lambda_1 / \log 2 \approx 1.3$ bits of new information about the trajectory.
