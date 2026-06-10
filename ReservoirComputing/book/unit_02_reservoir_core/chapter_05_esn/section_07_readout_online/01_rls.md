# Section 7: Online Readout Training — Recursive Least Squares

## 7.1 Motivation: When Offline Training is Not Enough

The ridge regression solution of Section 6 is **batch**: it requires storing all $T$ reservoir states simultaneously, forming the $N \times T$ matrix $X$, and solving a linear system. This is perfectly adequate when:
- The task is stationary (the input statistics do not change over time).
- We have access to the full training set in advance.
- Memory is not a constraint.

But many real-world situations violate these assumptions:
- **Non-stationary tasks:** A brain-machine interface must adapt as the neural code drifts over days.
- **Streaming data:** Financial forecasting processes one data point at a time; storing years of history is impractical.
- **Long training sequences:** If $T = 10^6$ and $N = 1000$, the matrix $X \in \mathbb{R}^{1000 \times 10^6}$ requires 8 GB of memory.

For these situations, we need an **online algorithm** that updates the readout weights one time step at a time, processing each sample as it arrives and then discarding it.

The classic algorithm for this is **Recursive Least Squares (RLS)**, which maintains an exact least-squares solution at every step, updated incrementally at cost $O(N^2)$ per step.

---

## 7.2 Setup: The Recursive Least Squares Problem

We want to minimize the **time-discounted least squares** objective:

$$L_t(W^{out}) = \sum_{s=1}^{t} \mu^{t-s} \|y_s^* - W^{out} x_s\|^2 \tag{7.1}$$

where $\mu \in (0, 1]$ is the **forgetting factor**. For $\mu = 1$, this reduces to the ordinary least squares objective. For $\mu < 1$, recent errors are weighted more heavily than past errors, allowing the algorithm to adapt to non-stationary statistics.

For simplicity, consider $L = 1$ (scalar output) and write $w = W^{out\top} \in \mathbb{R}^N$ for the readout weight vector. The objective is:

$$L_t(w) = \sum_{s=1}^{t} \mu^{t-s} (y_s^* - w^\top x_s)^2 \tag{7.2}$$

The solution at time $t$ is:

$$w_t = \left(\sum_{s=1}^{t} \mu^{t-s} x_s x_s^\top\right)^{-1} \left(\sum_{s=1}^{t} \mu^{t-s} y_s^* x_s\right) \tag{7.3}$$

Define:

$$C_t = \sum_{s=1}^{t} \mu^{t-s} x_s x_s^\top \in \mathbb{R}^{N \times N} \qquad \text{(correlation matrix)} \tag{7.4}$$
$$q_t = \sum_{s=1}^{t} \mu^{t-s} y_s^* x_s \in \mathbb{R}^N \qquad \text{(cross-correlation vector)} \tag{7.5}$$

So $w_t = C_t^{-1} q_t$. The question is: can we update $C_t^{-1}$ and $q_t$ incrementally when we receive a new sample $(x_{t+1}, y_{t+1}^*)$?

---

## 7.3 The Recursive Update for $C_t$

The correlation matrix satisfies the recursion:

$$C_{t+1} = \mu C_t + x_{t+1} x_{t+1}^\top \tag{7.6}$$

This follows directly from (7.4): the new correlation matrix is $\mu$ times the old one (discounting past observations) plus the outer product of the new state vector.

We need to compute $C_{t+1}^{-1}$ from $C_t^{-1}$. Define $P_t = C_t^{-1}$ (the **error covariance matrix** in Kalman filter terminology). Then we need to invert $P_{t+1}^{-1} = \mu P_t^{-1} + x_{t+1} x_{t+1}^\top$.

---

## 7.4 The Sherman-Morrison Formula

The key mathematical tool is the **Sherman-Morrison formula** (a special case of the Woodbury identity). For an invertible matrix $A$ and vectors $u, v$:

$$\boxed{(A + uv^\top)^{-1} = A^{-1} - \frac{A^{-1} u v^\top A^{-1}}{1 + v^\top A^{-1} u}} \tag{7.7}$$

**Proof.** Multiply both sides on the left by $A + uv^\top$:

$$(A + uv^\top)\left(A^{-1} - \frac{A^{-1} u v^\top A^{-1}}{1 + v^\top A^{-1} u}\right)$$

$$= I + uv^\top A^{-1} - \frac{uv^\top A^{-1}}{1 + v^\top A^{-1} u} - \frac{uv^\top A^{-1} u v^\top A^{-1}}{1 + v^\top A^{-1} u}$$

$$= I + uv^\top A^{-1} - \frac{u(1 + v^\top A^{-1} u) v^\top A^{-1}}{1 + v^\top A^{-1} u}$$

$$= I + uv^\top A^{-1} - u v^\top A^{-1} = I \quad \checkmark$$

The denominator $1 + v^\top A^{-1} u$ must be nonzero; this is guaranteed when $v^\top A^{-1} u \neq -1$.

---

## 7.5 Deriving the RLS Update

**Apply Sherman-Morrison to the correlation matrix update.**

We have $C_{t+1} = \mu C_t + x_{t+1} x_{t+1}^\top$. We can rewrite this as:

$$C_{t+1} = \mu C_t\left(I + \frac{1}{\mu} C_t^{-1} x_{t+1} x_{t+1}^\top\right)$$

Taking the inverse:

$$C_{t+1}^{-1} = \left(I + \frac{1}{\mu} C_t^{-1} x_{t+1} x_{t+1}^\top\right)^{-1} \frac{1}{\mu} C_t^{-1}$$

Applying Sherman-Morrison to the parenthesized inverse with $A = I$, $u = \frac{1}{\mu} C_t^{-1} x_{t+1}$, $v^\top = x_{t+1}^\top$... Actually, let us apply it more directly.

We want to invert $\mu C_t + x_{t+1} x_{t+1}^\top$. Identify $A = \mu C_t$, $u = x_{t+1}$, $v^\top = x_{t+1}^\top$. By Sherman-Morrison:

$$C_{t+1}^{-1} = (\mu C_t + x_{t+1} x_{t+1}^\top)^{-1} = \frac{1}{\mu}C_t^{-1} - \frac{\frac{1}{\mu}C_t^{-1} x_{t+1} x_{t+1}^\top \frac{1}{\mu} C_t^{-1}}{1 + x_{t+1}^\top \frac{1}{\mu} C_t^{-1} x_{t+1}}$$

$$= \frac{1}{\mu}\left[C_t^{-1} - \frac{C_t^{-1} x_{t+1} x_{t+1}^\top C_t^{-1}}{\mu + x_{t+1}^\top C_t^{-1} x_{t+1}}\right]$$

Now define $P_t = C_t^{-1}$. Then:

$$P_{t+1} = \frac{1}{\mu}\left[P_t - \frac{P_t x_{t+1} x_{t+1}^\top P_t}{\mu + x_{t+1}^\top P_t x_{t+1}}\right] \tag{7.8}$$

For $\mu = 1$ (no forgetting), this simplifies to the standard RLS covariance update:

$$\boxed{P_{t+1} = P_t - \frac{P_t x_{t+1} x_{t+1}^\top P_t}{1 + x_{t+1}^\top P_t x_{t+1}}} \tag{7.9}$$

This is the **RLS covariance update** — the matrix $P_t$ loses a rank-1 component at each step.

**The RLS gain vector.** Define the **Kalman gain**:

$$k_{t+1} = \frac{P_t x_{t+1}}{1 + x_{t+1}^\top P_t x_{t+1}} \in \mathbb{R}^N \tag{7.10}$$

Then (7.9) can be written compactly as:

$$P_{t+1} = P_t - k_{t+1} x_{t+1}^\top P_t = (I - k_{t+1} x_{t+1}^\top) P_t \tag{7.11}$$

---

## 7.6 Updating the Weight Vector

Now we update the readout weight vector. The cross-correlation satisfies:

$$q_{t+1} = \mu q_t + y_{t+1}^* x_{t+1} \tag{7.12}$$

The new weight vector is $w_{t+1} = P_{t+1} q_{t+1}$. Substituting:

$$w_{t+1} = P_{t+1}(\mu q_t + y_{t+1}^* x_{t+1})$$

$$= \mu P_{t+1} q_t + y_{t+1}^* P_{t+1} x_{t+1}$$

Now we need $P_{t+1} q_t$. From (7.8) (with $\mu = 1$ for clarity):

$$P_{t+1} q_t = \left(P_t - k_{t+1} x_{t+1}^\top P_t\right) q_t = P_t q_t - k_{t+1} (x_{t+1}^\top P_t q_t)$$

$$= w_t - k_{t+1} (x_{t+1}^\top w_t)$$

Also, $P_{t+1} x_{t+1}$: using (7.11):

$$P_{t+1} x_{t+1} = (I - k_{t+1} x_{t+1}^\top) P_t x_{t+1} = P_t x_{t+1} - k_{t+1} (x_{t+1}^\top P_t x_{t+1})$$

From the definition (7.10): $k_{t+1} (1 + x_{t+1}^\top P_t x_{t+1}) = P_t x_{t+1}$, so $P_t x_{t+1} = k_{t+1}(1 + x_{t+1}^\top P_t x_{t+1})$. Therefore:

$$P_{t+1} x_{t+1} = k_{t+1}(1 + x_{t+1}^\top P_t x_{t+1}) - k_{t+1}(x_{t+1}^\top P_t x_{t+1}) = k_{t+1}$$

So $P_{t+1} x_{t+1} = k_{t+1}$. Substituting back (with $\mu = 1$):

$$w_{t+1} = w_t - k_{t+1}(x_{t+1}^\top w_t) + y_{t+1}^* k_{t+1}$$

$$= w_t + k_{t+1}(y_{t+1}^* - x_{t+1}^\top w_t) \tag{7.13}$$

Define the **prediction error** $e_{t+1} = y_{t+1}^* - x_{t+1}^\top w_t = y_{t+1}^* - \hat{y}_{t+1}$. Then:

$$\boxed{w_{t+1} = w_t + k_{t+1} e_{t+1}} \tag{7.14}$$

This has an elegant interpretation: the weight vector is updated by a step in the direction of $k_{t+1}$, proportional to the prediction error $e_{t+1}$. The Kalman gain $k_{t+1}$ determines both the direction and magnitude of the update.

---

## 7.7 Complete RLS Algorithm

Combining the results, the RLS algorithm for the ESN readout is:

**Initialization:**
$$P_0 = \delta^{-1} I_N, \quad w_0 = 0 \tag{7.15}$$

where $\delta > 0$ is a small constant (e.g., $\delta = 0.01$ or $\delta = 1$). The choice $P_0 = \delta^{-1} I$ corresponds to starting with flat prior uncertainty about the weights.

**For each time step $t = 1, 2, \ldots$:**
1. **Observe** reservoir state $x_t$ and target $y_t^*$.
2. **Compute prediction** and error:
$$\hat{y}_t = w_{t-1}^\top x_t, \quad e_t = y_t^* - \hat{y}_t \tag{7.16}$$
3. **Compute gain:**
$$k_t = \frac{P_{t-1} x_t}{1 + x_t^\top P_{t-1} x_t} \tag{7.17}$$
4. **Update weights:**
$$w_t = w_{t-1} + k_t \, e_t \tag{7.18}$$
5. **Update covariance:**
$$P_t = P_{t-1} - k_t x_t^\top P_{t-1} = (I - k_t x_t^\top) P_{t-1} \tag{7.19}$$

For $L > 1$ outputs, run $L$ parallel RLS algorithms (one per output), or equivalently, replace $w \in \mathbb{R}^N$ with $W^{out} \in \mathbb{R}^{L \times N}$ and $e \in \mathbb{R}$ with $e \in \mathbb{R}^L$. The gain $k_t \in \mathbb{R}^N$ and the covariance $P_t \in \mathbb{R}^{N \times N}$ are shared across all outputs.

**Full matrix update (for $L$ outputs simultaneously):**
$$W_t^{out} = W_{t-1}^{out} + e_t k_t^\top \tag{7.20}$$

where $e_t = y_t^* - W_{t-1}^{out} x_t \in \mathbb{R}^L$ is now a vector error (note: outer product $e_t k_t^\top \in \mathbb{R}^{L \times N}$).

---

## 7.8 Computational Cost Analysis

Each RLS update requires:
- Computing $P_{t-1} x_t$: matrix-vector multiply, $O(N^2)$.
- Computing $x_t^\top P_{t-1} x_t = (P_{t-1} x_t)^\top x_t / \|...\|$: inner product, $O(N)$.
- Computing $k_t$: vector scale, $O(N)$.
- Updating $w_t$: DAXPY, $O(N)$.
- Updating $P_t$: rank-1 update $P_t = P_{t-1} - k_t x_t^\top P_{t-1}$, $O(N^2)$.

Total: $O(N^2)$ per time step.

**Comparison with batch ridge regression:** Batch training costs $O(N^2 T + N^3)$ total, equivalent to $O(N^2 + N^3/T)$ per step amortized. For large $T$ and moderate $N$, the per-step costs are comparable. The advantage of RLS is memory: batch training requires storing all $T$ states ($O(NT)$ memory) while RLS requires only $P_t$ ($O(N^2)$ memory).

For $N = 1000$ neurons and $T = 10^6$ steps:
- Batch: $O(10^{12})$ total operations, $O(10^9)$ bytes (1 GB) memory for $X$.
- RLS: $O(10^{12})$ total operations, $O(10^6)$ bytes (1 MB) memory for $P_t$.

Memory savings are dramatic; computational savings are modest.

---

## 7.9 The Kalman Filter Interpretation

The RLS algorithm is a special case of the **Kalman filter** [Kalman1960]. To see this, write the ESN readout in state-space form:

**Observation model:**
$$y_t^* = x_t^\top w_t + \epsilon_t, \quad \epsilon_t \sim \mathcal{N}(0, \sigma^2) \tag{7.21}$$

**State (weight) dynamics:**
$$w_t = w_{t-1} + \eta_t, \quad \eta_t \sim \mathcal{N}(0, Q) \tag{7.22}$$

For RLS with $\mu = 1$, we take $Q = 0$ (the weights do not evolve — they are fixed parameters). For RLS with $\mu < 1$ (forgetting), we set $Q \propto (1-\mu) P_t$ to model slow drift.

The Kalman filter for this system gives exactly the RLS updates:
- $P_t$ is the posterior covariance of $w$ given $x_1, \ldots, x_t$.
- $w_t$ is the posterior mean (MAP estimate) of $w$ given the same data.
- $k_t$ is the Kalman gain: the optimal weighting of the new observation vs. the prior.

This interpretation is powerful. It tells us that RLS is not merely an algorithm but an optimal Bayesian filter: at each step, it maintains the full posterior distribution over readout weights (a Gaussian, characterized by mean $w_t$ and covariance $P_t$), and it updates this distribution optimally when each new observation arrives.

**The forgetting factor revisited.** Setting $\mu < 1$ in (7.1) is equivalent to assuming in the Kalman filter that the weights undergo a random walk:

$$w_t = \sqrt{\mu} w_{t-1} + \eta_t$$

with $\eta_t \sim \mathcal{N}(0, (1-\mu) P_{t-1})$. This inflates the covariance at each step, preventing the algorithm from "locking in" to an estimate. The result is that older observations are down-weighted relative to newer ones, enabling adaptation to non-stationary statistics.

---

## 7.10 Convergence Properties

**Theorem 7.1 (RLS Convergence).** Under mild conditions (the sequence $\{x_t\}$ is persistently exciting, meaning $C_t / t \to C_\infty$ for some positive definite $C_\infty$), the RLS estimate converges:

$$w_t \to w^* = C_\infty^{-1} q_\infty, \quad P_t \to 0 \tag{7.23}$$

where $w^*$ is the true optimal readout weight vector. Convergence is exponential in $t$.

**Persistent excitation** means the reservoir states are "spread out" in $\mathbb{R}^N$ and do not collapse to a subspace. This is almost always satisfied in practice for nonlinear reservoirs driven by non-constant inputs.

**Caution:** As $P_t \to 0$ (after convergence), the gain $k_t \to 0$ and the algorithm stops adapting. For non-stationary tasks, set $\mu < 1$ to maintain a nonzero $P_t$ and continue adaptation. The steady-state covariance with forgetting is $P_\infty \approx (1-\mu) C_\infty^{-1} / \mu$ (under stationarity), which balances the rate of forgetting $(1-\mu)$ with the rate of information accumulation.
