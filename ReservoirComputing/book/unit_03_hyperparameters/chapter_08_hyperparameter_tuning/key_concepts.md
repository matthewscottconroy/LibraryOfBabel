# Chapter 8: Key Concepts

**1. Spectral Radius ($\rho$)**
The spectral radius $\rho = \max_i |\lambda_i(W)|$ is the largest absolute eigenvalue of the recurrent weight matrix. It is the primary parameter controlling the reservoir's memory timescale: larger $\rho$ means slower decay of the reservoir's impulse response, longer effective memory, and memory profiles that decay geometrically with rate $\rho^2$. For the linear reservoir, the total memory capacity grows as $\rho^2 / (1-\rho^2)$ for small $\rho$, bounded by $N$.

**2. Geometric Memory Decay**
For a linear reservoir with spectral radius $\rho$, the $k$-step memory capacity satisfies $MC_k \propto \rho^{2k}$. This is derived from the impulse response expansion $\mathbf{r}(t) = \sum_k W^k \mathbf{w}^{in} u_{t-k}$: the cross-covariance between the state and input $k$ steps ago decays as $\|W^k \mathbf{w}^{in}\| \sim \rho^k$. The effective memory horizon is $\tau_{mem} = -1/(2\ln\rho)$.

**3. Echo State Property and Stability**
The echo state property (ESP) is guaranteed for tanh reservoirs when $\rho < 1$: the reservoir is a contraction mapping with Jacobian spectral radius $\rho(J) \leq \rho(W) < 1$. The ESP ensures the reservoir state is uniquely determined by the input history, a prerequisite for meaningful computation. As $\rho \to 1^-$, stability is maintained but transients grow and washout periods must be increased.

**4. Input Scaling ($\sigma_{in}$)**
The input scaling $\sigma_{in}$ scales the entries of the input weight matrix $W^{in}$. It controls the operating point of the tanh nonlinearity: small $\sigma_{in}$ keeps neurons in the near-linear regime ($|a_i| \ll 1$, $\tanh'(a_i) \approx 1$), while large $\sigma_{in}$ drives neurons toward saturation ($|a_i| \gg 1$, $\tanh'(a_i) \approx 0$). The transition occurs when the typical pre-activation variance $\sigma_a^2 \approx 1$.

**5. Effective Spectral Radius**
The product $\rho_{eff} = \bar{g}(\sigma_{in}) \cdot \rho_W$, where $\bar{g} = \mathbb{E}[1 - r^2]$ is the mean gain of the tanh nonlinearity in the stationary state. Input saturation reduces $\bar{g} < 1$, compressing the effective spectral radius below $\rho_W$. This is the mechanism by which large input scaling destroys long-range memory even when $\rho_W$ is set close to 1.

**6. Nonlinearity-Memory Trade-off**
The fundamental competition between nonlinear processing and linear memory in a fixed-size reservoir. Capacity used for quadratic and higher-order functions of the input history is capacity not available for linear memory. Input scaling mediates this trade-off: small $\sigma_{in}$ maximizes linear memory; moderate $\sigma_{in}$ activates useful nonlinearity; large $\sigma_{in}$ saturates neurons and destroys both memory and useful nonlinear structure.

**7. Leak Rate ($\alpha$)**
The leak rate $\alpha \in (0,1]$ parameterizes the leaky integrator update $\mathbf{r}(t) = (1-\alpha)\mathbf{r}(t-1) + \alpha\tanh(W\mathbf{r}(t-1) + \mathbf{w}^{in}u_t)$. It controls how quickly the reservoir state is updated. For $\alpha = 1$ this is the standard ESN; for $\alpha \to 0$ the state freezes. The leak rate sets an independent timescale — complementary to the spectral-radius timescale — that acts as a low-pass filter on the input.

**8. Effective Time Constant**
The time constant of the leaky integrator dynamics: $\tau_{eff} = 1/(\alpha(1-|\lambda_i|))$ for eigenmode $i$. The dominant time constant is $\tau_{dom} = 1/(\alpha(1-\rho))$. This formula allows the reservoir to be designed to match a specific signal timescale: set $\alpha = 1/(T_0(1-\rho))$ where $T_0$ is the signal's characteristic period.

**9. Heterogeneous Leak Rates**
Using different leak rates for different neurons creates a multi-timescale reservoir that simultaneously represents the input signal at multiple frequency bands. Each group of neurons acts as a low-pass filter with a different cutoff frequency. For signals with hierarchical temporal structure (e.g., speech, music, physiological signals), heterogeneous leak rates can dramatically outperform any single homogeneous setting.

**10. Bayesian Hyperparameter Optimization**
A principled approach to hyperparameter search that builds a probabilistic surrogate model (typically a Gaussian process) of the performance-vs-hyperparameter surface, then uses the model to select the next evaluation point by maximizing an acquisition function (expected improvement, upper confidence bound, etc.). More efficient than grid search or random search [BergstraBengio2012], particularly in high-dimensional hyperparameter spaces, because it concentrates evaluations in promising regions rather than exploring uniformly.
