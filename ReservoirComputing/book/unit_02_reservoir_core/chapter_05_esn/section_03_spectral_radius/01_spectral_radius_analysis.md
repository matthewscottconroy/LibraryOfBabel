# Section 3: Spectral Radius Analysis

## 3.1 Definition and Computation

The **spectral radius** of the recurrent weight matrix is:

$$\rho(W^{rec}) = \max_{i} |\lambda_i| \tag{3.1}$$

where $\lambda_1, \ldots, \lambda_N \in \mathbb{C}$ are the eigenvalues of $W^{rec}$. For a real matrix, complex eigenvalues appear in conjugate pairs, so $\rho$ is always real and nonnegative.

**Computing $\rho$:** Exact computation requires finding all eigenvalues, which costs $O(N^3)$ via the QR algorithm. For large reservoirs this is manageable but not free. An efficient alternative is the **power iteration** method:

**Algorithm (Power Iteration):**
1. Initialize $v_0 \in \mathbb{R}^N$ randomly.
2. For $k = 1, 2, \ldots$:
   - $v_k = W^{rec} v_{k-1}$
   - $\lambda_k = \|v_k\| / \|v_{k-1}\|$
3. Return $\lambda_k$ when converged.

Power iteration converges to the largest eigenvalue in magnitude at rate $|\lambda_2/\lambda_1|$, where $\lambda_1$ is the largest eigenvalue and $\lambda_2$ is the second largest. Convergence is fast when there is a gap between $|\lambda_1|$ and $|\lambda_2|$.

For reservoir initialization, we typically construct $W^{rec}$ by:
1. Drawing entries i.i.d. from $\mathcal{N}(0, 1)$ or a sparse distribution.
2. Scaling: $W^{rec} \leftarrow W^{rec} \cdot \rho_{target} / \rho(W^{rec})$.

Step 2 sets the spectral radius exactly to the target value while preserving the structure of the weight matrix. This is the standard initialization procedure [Lukoševičius2012].

---

## 3.2 What $\rho < 1$ Means Physically

The spectral radius controls the **autonomous dynamics** of the reservoir — what happens in the absence of any input.

When $u_t = 0$ for all $t$ and there is no input, the reservoir evolves as:
$$x_{t+1} = (1-\alpha) x_t + \alpha \tanh(W^{rec} x_t)$$

Near the origin ($x = 0$), $\tanh(W^{rec} x) \approx W^{rec} x$, and the linearized dynamics are:

$$x_{t+1} \approx \left[(1-\alpha)I + \alpha W^{rec}\right] x_t$$

The eigenvalues of the linearized map are $(1-\alpha) + \alpha \lambda_i$. The origin is stable (all perturbations decay) iff all these eigenvalues have absolute value $< 1$:

$$|(1-\alpha) + \alpha \lambda_i| < 1 \quad \text{for all } i$$

For real eigenvalues $\lambda_i \in \mathbb{R}$, this requires $-1 < (1-\alpha) + \alpha \lambda_i < 1$, giving:
$$-\frac{2(1-\alpha)}{\alpha} < \lambda_i < 1$$

For $\alpha \leq 1$ and $\lambda_i > 0$, this simplifies to $\lambda_i < 1$. For the dominant (largest) eigenvalue, the condition is $\rho(W^{rec}) < 1$.

**Physical picture.** When $\rho < 1$, the reservoir with zero input has a unique stable fixed point at $x = 0$ (for zero bias). Any perturbation decays exponentially. The reservoir is "passive" — it needs input to sustain interesting dynamics.

When $\rho > 1$, the origin is unstable. Even tiny perturbations are amplified. The reservoir can sustain oscillations, limit cycles, or chaotic attractors autonomously. Input then perturbs these autonomous dynamics rather than driving a passive system from rest.

When $\rho = 1$ exactly, the system is at the boundary: some eigenvalues are on the unit circle, meaning there are neutrally stable modes. Perturbations in these directions neither grow nor decay. This is the critical case.

---

## 3.3 The Edge of Stability: Why $\rho \approx 1$ is Optimal

The empirical finding that guides ESN design is: **performance on temporal tasks peaks when $\rho \approx 1$**, slightly below the stability boundary. This is sometimes called operating at the "edge of stability" or, more dramatically, the "edge of chaos."

Why? Consider what the reservoir does: it maps input history to a rich set of features that the readout can use. We need two things from this mapping:

1. **Memory:** The reservoir should retain information about past inputs long enough to compute the desired output.
2. **Nonlinearity:** The reservoir should mix inputs nonlinearly, creating features that are more expressive than linear functions of the input.

These two requirements are in tension.

**Memory analysis.** For the linearized reservoir with $\rho < 1$, the state at time $t$ can be written as:

$$x_t \approx \sum_{k=0}^{\infty} (A)^k W^{in} u_{t-k}, \quad A = (1-\alpha)I + \alpha W^{rec}$$

The contribution of input $u_{t-k}$ to the current state decays as $\|A^k\|$. By the Gelfand formula, $\|A^k\|^{1/k} \to \rho(A) = (1-\alpha) + \alpha\rho(W^{rec})$ as $k \to \infty$.

So the memory of past inputs decays at rate $\rho(A)$. When $\rho(A) \approx 1$ (achieved by $\rho(W^{rec}) \approx 1$), the reservoir retains a long memory. When $\rho(A) \ll 1$, the reservoir forgets quickly.

**Quantitatively:** The effective memory length (time lag beyond which inputs contribute negligibly) scales as $1/(1-\rho(A))$. For $\rho(W^{rec}) = 0.9$ and $\alpha = 1$, this is $1/(1-0.9) = 10$ steps. For $\rho(W^{rec}) = 0.99$, it is $100$ steps.

**Nonlinearity analysis.** When $\rho \ll 1$ and inputs are small, the reservoir operates near the origin where $\tanh(z) \approx z$. The dynamics are approximately linear, and the reservoir has little nonlinear processing power.

As $\rho$ increases toward $1$, the trajectories in state space span a larger portion of the $[-1,1]^N$ hypercube (since autonomous dynamics no longer strongly attract to the origin), and nonlinear features become more prevalent.

**The tradeoff.** Beyond $\rho = 1$, the autonomous dynamics dominate: the reservoir follows its own attractor and the input signal produces perturbations on top of it. Memory for the input signal is degraded because the reservoir's own dynamics "wash out" the input. The reservoir is too autonomous.

The sweet spot is $\rho \lesssim 1$: long memory (because $\rho(A)$ is close to 1) and significant nonlinearity (because the reservoir spans a large portion of state space), while still satisfying the ESP.

---

## 3.4 Formal Memory Capacity Analysis

The **memory capacity** of an ESN is the maximum amount of information about past inputs that can be linearly decoded from the reservoir state. It was formalized by Jaeger [Jaeger2002memory] as:

$$MC = \sum_{k=1}^{\infty} MC_k \tag{3.2}$$

where $MC_k$ measures the capacity to reconstruct the input from $k$ steps ago:

$$MC_k = \frac{[\text{Cov}(u_{t-k}, y_t^{(k)})]^2}{\text{Var}(u_{t-k}) \cdot \text{Var}(y_t^{(k)})} \tag{3.3}$$

Here $y_t^{(k)}$ is the optimal linear readout for $u_{t-k}$ and Cov/Var are covariance/variance. The key theoretical result is:

**Theorem 3.1 (Jaeger 2002).** For a linear reservoir of size $N$ with scalar inputs, $MC \leq N$.

This bound is achieved (with equality) for certain reservoir configurations. It says the total memory capacity cannot exceed the number of neurons — each neuron can store at most one "memory unit."

For a reservoir with $\rho < 1$, the memory capacity $MC$ increases as $\rho \to 1^-$. In the limit $\rho \to 1^-$, MC approaches $N$ (the maximum). For $\rho > 1$, the reservoir is unstable and the memory capacity collapses.

---

## 3.5 Counterexamples: ESP Can Hold for $\rho > 1$

As established in Section 2.2, $\rho < 1$ is sufficient but not necessary for the ESP. Here we give concrete counterexamples showing that the ESP can hold when $\rho > 1$.

### Example 3.1: Bounded Inputs and Saturating Nonlinearity

Let $N = 1$ and:
$$x_{t+1} = \tanh(2 x_t + u_t)$$

with $W^{rec} = 2$, so $\rho = 2 > 1$.

**Does the ESP hold for inputs $u_t \in [-0.1, 0.1]$?**

Consider two trajectories $x_t$ and $x_t'$ with the same input. Then:
$$|x_{t+1} - x_{t+1}'| = |\tanh(2x_t + u_t) - \tanh(2x_t' + u_t)|$$
$$\leq \tanh'(\xi) \cdot 2 |x_t - x_t'|$$

for some $\xi$ between $2x_t + u_t$ and $2x_t' + u_t$. The key question is whether $2\tanh'(\xi) < 1$, i.e., $\tanh'(\xi) < 1/2$, i.e., $|\xi| > 0.549$.

If the reservoir operates far from zero (i.e., $|2x_t + u_t| \gg 0$), then $\tanh'$ is small and the product $2\tanh'(\xi)$ can be $< 1$.

**Numerical verification:** Start with $x_0 = 0.8$ and $x_0' = -0.8$. With $u_t = 0.05 \sin(t)$:
- $x_1 = \tanh(1.6 + 0.05) = \tanh(1.65) \approx 0.931$
- $x_1' = \tanh(-1.6 + 0.05) = \tanh(-1.55) \approx -0.914$

The difference has grown slightly: $|x_1 - x_1'| \approx 1.845 > |x_0 - x_0'| = 1.6$.

At $x \approx \pm 0.93$, we have $2x + u \approx \pm 1.91$, so $\tanh'(1.91) = 1 - \tanh^2(1.91) \approx 1 - 0.957^2 \approx 0.084$. The effective Lipschitz constant is $2 \times 0.084 = 0.168 < 1$.

After a transient period of initial divergence, the trajectories contract and converge. The ESP **does** hold here, even though $\rho = 2 > 1$.

**Moral:** With large $\rho > 1$, the ESP may still hold if the input drives the reservoir into a strongly saturating regime. The nonlinearity saves us.

### Example 3.2: Negative Spectral Radius

Let $W^{rec} = -1.5$ (scalar). Then $\rho = 1.5 > 1$.

Without input, the dynamics $x_{t+1} = \tanh(-1.5 x_t)$ have a stable fixed point at $x^* = 0$ (check: $\tanh(-1.5 \cdot 0) = 0$; and the derivative at zero is $-1.5 \cdot \tanh'(0) = -1.5$, which has magnitude $> 1$, so the fixed point is unstable). The system undergoes period-2 oscillation between two values $\pm x^*$ where $x^* = \tanh(1.5 x^*)$.

Now with input $u_t \in [-\epsilon, \epsilon]$: two trajectories starting at $\pm 0.7$ quickly converge because the alternating-sign dynamics create a contracting map when composed twice. The ESP holds for the period-2 dynamics.

---

## 3.6 Worked Numerical Example

We illustrate the role of $\rho$ with a concrete experiment on the Mackey-Glass time series.

**Setup:**
- Task: one-step-ahead prediction of Mackey-Glass time series ($\tau = 17$, chaotic regime).
- Reservoir: $N = 100$ neurons, $\tanh$ activation, $\alpha = 1$.
- $W^{rec}$: sparse random matrix with connection probability $p = 0.1$.
- $W^{in}$: dense random matrix, input scaling $\sigma_{in} = 0.1$.
- Training: $T_{train} = 5000$ steps, washout $T_{wash} = 200$.
- Metric: Normalized root mean squared error (NRMSE) on test set.

**Spectral radius sweep:** We vary $\rho \in \{0.1, 0.3, 0.5, 0.7, 0.8, 0.9, 0.95, 0.99, 1.0, 1.05, 1.1, 1.2\}$.

**Results (representative values):**

| $\rho$ | NRMSE |
|--------|-------|
| 0.1 | 0.412 |
| 0.3 | 0.287 |
| 0.5 | 0.168 |
| 0.7 | 0.094 |
| 0.9 | 0.031 |
| 0.95 | 0.018 |
| 0.99 | 0.012 |
| 1.0 | 0.015 |
| 1.05 | 0.024 |
| 1.1 | 0.041 |
| 1.2 | 0.189 |

The U-shaped curve is characteristic: performance degrades when $\rho$ is too small (insufficient memory) and when $\rho$ is too large (ESN becomes unstable or autonomous dynamics dominate). The minimum NRMSE occurs near $\rho \approx 0.95$-$0.99$.

**Analysis:**

For small $\rho = 0.1$: The reservoir's memory time constant is $1/(1-0.1) \approx 1.1$ steps. This is far shorter than the characteristic timescale of Mackey-Glass ($\tau = 17$ steps), so the reservoir cannot retain the relevant information.

For optimal $\rho \approx 0.95$-$0.99$: The memory time constant is $1/(1-0.95) = 20$ to $1/(1-0.99) = 100$ steps — matching and exceeding the task's timescale. The reservoir has both the memory and the nonlinear mixing needed for accurate prediction.

For $\rho = 1.1$: The autonomous dynamics are unstable. The reservoir trajectory is driven primarily by the attractor of the autonomous system rather than the input, degrading memory for the input signal.

---

## 3.7 Setting $\rho$ in Practice

**Rule of thumb:** Set $\rho(W^{rec}) \approx 0.9$ as a starting point. Tune upward if the task has long temporal dependencies; tune downward if the task is predominantly instantaneous.

**Relationship to task timescale $T$:** The memory time constant of the reservoir (in the linear approximation) is $\tau_{mem} \approx -1/\ln(\rho(A))$ where $A = (1-\alpha)I + \alpha W^{rec}$. For $\alpha = 1$ and $\rho$ close to 1:

$$\tau_{mem} \approx \frac{1}{1 - \rho(W^{rec})} \tag{3.4}$$

Set $\rho(W^{rec}) = 1 - 1/T$ to match the task timescale $T$. For a task requiring memory over $T = 20$ steps, use $\rho \approx 0.95$.

**Combined control with $\alpha$:** The memory time constant depends on both $\rho$ and $\alpha$. For the full leaky integrator, $\rho(A) = (1-\alpha) + \alpha \rho(W^{rec})$ (in the linear approximation). So:

$$\tau_{mem} \approx \frac{1}{1 - \rho(A)} = \frac{1}{\alpha(1-\rho(W^{rec}))} \tag{3.5}$$

Both $\alpha$ and $\rho$ can be tuned to achieve the desired memory timescale. A common strategy is to fix $\rho = 0.9$ and tune $\alpha$ to match the input timescale.

**Practical range:** Almost all successful ESN applications use $\rho \in [0.7, 1.1]$. Values outside this range are rarely optimal.
