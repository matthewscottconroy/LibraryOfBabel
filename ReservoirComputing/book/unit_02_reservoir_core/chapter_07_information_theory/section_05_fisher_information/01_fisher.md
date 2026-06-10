# Section 7.5: Fisher Information and Reservoir Sensitivity

## 7.5.1 The Cramér-Rao Framework

Fisher information quantifies how much a distribution changes in response to a parameter perturbation — equivalently, how much information an observation carries about an unknown parameter. It provides a fundamental lower bound on estimation variance, and in the reservoir context, it measures how sensitively the reservoir state responds to changes in the input or reservoir parameters.

**Definition 7.5.1 (Fisher Information Matrix).** Let $p(\mathbf{x}; \boldsymbol{\theta})$ be a family of distributions parameterized by $\boldsymbol{\theta} \in \mathbb{R}^m$. The *Fisher information matrix* (FIM) is the $m \times m$ matrix

$$\mathcal{I}(\boldsymbol{\theta})_{ij} = \mathbb{E}_{\mathbf{x} \sim p(\cdot;\boldsymbol{\theta})}\!\left[\frac{\partial \log p(\mathbf{x};\boldsymbol{\theta})}{\partial \theta_i} \cdot \frac{\partial \log p(\mathbf{x};\boldsymbol{\theta})}{\partial \theta_j}\right],$$

where the expectation is over observations $\mathbf{x}$ drawn from $p(\mathbf{x};\boldsymbol{\theta})$. The vector $\frac{\partial}{\partial \boldsymbol{\theta}} \log p(\mathbf{x};\boldsymbol{\theta})$ is the *score function*.

Under regularity conditions (interchange of derivative and integral):

$$\mathcal{I}(\boldsymbol{\theta})_{ij} = -\mathbb{E}\!\left[\frac{\partial^2 \log p(\mathbf{x};\boldsymbol{\theta})}{\partial \theta_i \partial \theta_j}\right],$$

the FIM equals the negative expected Hessian of the log-likelihood — a measure of the curvature of the log-likelihood surface.

**Theorem 7.5.1 (Cramér-Rao Bound).** *Let $\hat{\boldsymbol{\theta}}(\mathbf{x})$ be any unbiased estimator of $\boldsymbol{\theta}$. Then*

$$\text{Cov}(\hat{\boldsymbol{\theta}}) \geq \mathcal{I}(\boldsymbol{\theta})^{-1}$$

*in the positive-semidefinite sense: for any vector $\mathbf{a}$, $\mathbf{a}^\top \text{Cov}(\hat{\boldsymbol{\theta}}) \mathbf{a} \geq \mathbf{a}^\top \mathcal{I}(\boldsymbol{\theta})^{-1} \mathbf{a}$.*

The Cramér-Rao bound [Cramer1946, Fisher1925] is tight for the maximum likelihood estimator in the large-sample limit. Intuitively: the more information the observations carry about $\boldsymbol{\theta}$ (high $\mathcal{I}(\boldsymbol{\theta})$), the smaller the achievable estimation variance.

## 7.5.2 Fisher Information in the Reservoir Context

In reservoir computing, the "parameter" of interest is the input signal $\boldsymbol{\theta} = \mathbf{u}$, and the "observation" is the reservoir state $\mathbf{x}$. The relevant Fisher information is the matrix

$$\mathcal{I}(\mathbf{u})_{ij} = \mathbb{E}\!\left[\frac{\partial \log p(\mathbf{x}|\mathbf{u})}{\partial u_i} \cdot \frac{\partial \log p(\mathbf{x}|\mathbf{u})}{\partial u_j}\right],$$

where the expectation is over the distribution of reservoir states $\mathbf{x}$ given input $\mathbf{u}$, including the noise in the system (synaptic noise, intrinsic variability).

The FIM $\mathcal{I}(\mathbf{u})$ measures how sensitively the distribution of reservoir states $p(\mathbf{x}|\mathbf{u})$ responds to infinitesimal changes in the input $\mathbf{u}$. High Fisher information means that small changes in $\mathbf{u}$ produce large, detectable changes in the state distribution — the reservoir is a sensitive detector. Low Fisher information means the reservoir is insensitive to input perturbations — it has poor input resolution.

**Noisy reservoir model.** Consider the stochastic reservoir

$$\mathbf{x}(t) = f(W^{rec} \mathbf{x}(t-1) + W^{in} \mathbf{u}(t) + \boldsymbol{\xi}(t)),$$

where $\boldsymbol{\xi}(t) \sim \mathcal{N}(\mathbf{0}, \sigma^2 I)$ is additive Gaussian noise. For small noise, $p(\mathbf{x}(t)|\mathbf{x}(t-1), \mathbf{u}(t)) \approx \mathcal{N}(f(W^{rec}\mathbf{x}(t-1) + W^{in}\mathbf{u}(t)), \sigma^2 J_t J_t^\top)$, where $J_t = \text{diag}(f'(\cdot)) W^{in}$ is the Jacobian of the state with respect to the input.

The Fisher information for the input component $u_k(t)$ at time step $t$ is

$$\mathcal{I}(u_k(t)) = \frac{1}{\sigma^2} \mathbb{E}\!\left[\left\|J_t \mathbf{e}_k\right\|^2\right] = \frac{1}{\sigma^2} \mathbb{E}\!\left[\sum_i \left(f'(z_i)\, W^{in}_{ik}\right)^2\right],$$

where $z_i = (W^{rec}\mathbf{x}(t-1) + W^{in}\mathbf{u}(t))_i$ is the pre-activation of neuron $i$, and $f' = \tanh'$ is maximal in the linear regime ($f'(0) = 1$) and minimal in saturation ($f'(z) \to 0$ as $|z| \to \infty$).

## 7.5.3 Fisher Information at the Edge of Chaos

The connection between Fisher information and dynamical criticality is one of the most elegant results in computational neuroscience [Ganguli2014]. In a reservoir near the edge of chaos, the Jacobian of the dynamics is close to the identity — the reservoir amplifies perturbations in some directions while damping others, with the largest Lyapunov exponent $\lambda_{max} \approx 0$.

**Proposition 7.5.1 (Fisher Information Maximization at Edge of Chaos).** *For a linear reservoir with $\mathbf{x}(t) = W\mathbf{x}(t-1) + W^{in}\mathbf{u}(t) + \boldsymbol{\xi}(t)$ and Gaussian noise $\boldsymbol{\xi}(t) \sim \mathcal{N}(\mathbf{0}, \sigma^2 I)$, the steady-state Fisher information for the input at lag $\tau$ is*

$$\mathcal{I}(u(t-\tau)) = \frac{1}{\sigma^2} \|(W^{rec})^\tau W^{in}\|_F^2 = \frac{1}{\sigma^2} \sum_{k=1}^N s_k^{2\tau} \|(U^\top W^{in}\|_k^2,$$

*where $s_k$ are the singular values of $W^{rec}$ and $U$ its left singular vectors. This is maximized when $s_k \approx 1$ for all $k$ — the edge-of-chaos condition $\rho \approx 1$.*

The proof follows from the $\tau$-step Jacobian $(W^{rec})^\tau$: Fisher information at lag $\tau$ equals the squared norm of this Jacobian applied to $W^{in}$, which grows with the singular values. Singular values near 1 maximize the Fisher information for all lags simultaneously, while singular values less than 1 suppress information at large lags.

This formalizes a key insight: the edge of chaos is optimal not merely by intuition but by the Fisher information criterion — it is the operating regime where the reservoir retains the most information about past inputs.

## 7.5.4 Connection to Lyapunov Exponents

The Lyapunov exponents $\lambda_1 \geq \lambda_2 \geq \cdots$ of a nonlinear reservoir characterize the long-term expansion or contraction rates of infinitesimal perturbations. The maximum Lyapunov exponent $\lambda_{max}$ governs the leading-order sensitivity: a perturbation $\delta\mathbf{x}(0)$ grows as $\|\delta\mathbf{x}(t)\| \sim e^{\lambda_{max} t}$ for large $t$.

The connection to Fisher information is:

$$\mathcal{I}(u(t-\tau)) \sim e^{2\lambda_{max} \tau} \quad \text{for large } \tau,$$

in the regime where the dominant Lyapunov exponent governs the dynamics. For $\lambda_{max} < 0$ (stable regime), Fisher information decays exponentially with lag — the reservoir forgets inputs quickly. For $\lambda_{max} = 0$ (edge of chaos), Fisher information is approximately constant across lags — the reservoir retains equal sensitivity to inputs at all past times (within the fading memory window). For $\lambda_{max} > 0$ (chaotic regime), Fisher information grows with lag — but this growth is unstable and corrupted by chaotic divergence.

The sweet spot is $\lambda_{max} = 0$: the edge of chaos maximizes Fisher information while avoiding chaotic divergence. This is why the edge of chaos is the optimal operating regime from an information-theoretic perspective [Ganguli2014].

## 7.5.5 Optimal Reservoir Design via Fisher Information Maximization

The Fisher information framework suggests a principled approach to reservoir design: choose $(W^{rec}, W^{in})$ to maximize the total Fisher information about the input,

$$\max_{W^{rec}, W^{in}} \sum_{\tau=0}^{\infty} \mathcal{I}(u(t-\tau)),$$

subject to the echo state property ($\rho(W^{rec}) < 1$).

For linear reservoirs with Gaussian noise, this optimization has an analytic solution: the optimal $W^{rec}$ is any matrix with all singular values equal to $\rho < 1$ (a *unitary-like* matrix, scaled by $\rho$), and the optimal $W^{in}$ projects the input equally onto all left singular vectors of $W^{rec}$. The SCR (Section 9.2) is one instance of this optimal design: its cyclic permutation structure gives all eigenvalues magnitude exactly $\rho$, achieving the maximum linear memory capacity [RodanTino2011].

For nonlinear reservoirs, the FIM optimization is harder because it depends on the state distribution. Numerical optimization via gradient ascent on the FIM trace has been explored [Ganguli2014], with results confirming that optimized reservoirs outperform random baselines on memory tasks while remaining competitive on nonlinear tasks.

---

## References

- **[Cramer1946]** H. Cramér. *Mathematical Methods of Statistics*. Princeton University Press, 1946.
- **[Fisher1925]** R. A. Fisher. "Theory of statistical estimation." *Proceedings of the Cambridge Philosophical Society*, 22(5):700-725, 1925.
- **[Ganguli2014]** S. Ganguli and H. Sompolinsky. "Compressed sensing, sparsity, and dimensionality in neuronal information processing and data analysis." *Annual Review of Neuroscience*, 35:485-508, 2012.
- **[RodanTino2011]** A. Rodan and P. Tino. "Minimum complexity echo state network." *IEEE Transactions on Neural Networks*, 22(1):131-144, 2011.
