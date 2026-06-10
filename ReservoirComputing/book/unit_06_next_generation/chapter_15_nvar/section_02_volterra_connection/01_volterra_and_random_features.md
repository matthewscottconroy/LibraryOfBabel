# 15.2.1 NVAR as Volterra Series; ESN as Random Feature Approximation

## NVAR Is a Truncated Volterra Series

Recall from Chapter 1 that the Volterra series represents any fading-memory functional as a sum of multilinear kernels:

$$y(t) = h_0 + \sum_{k=0}^\infty h_1(k) u(t-k) + \sum_{k_1, k_2 \geq 0} h_2(k_1, k_2) u(t-k_1) u(t-k_2) + \cdots$$

The NVAR with history length $k_{max}$ and polynomial degree $d$ is exactly the **truncated Volterra series** at order $d$, with the history window restricted to $k_{max}+1$ steps:

$$y(t) \approx \sum_{j=0}^d \sum_{k_1, \ldots, k_j = 0}^{k_{max}} w_{j; k_1, \ldots, k_j} \prod_{i=1}^j u(t - k_i)$$

The NVAR coefficients $W^{out}$ are exactly the estimated Volterra kernels $\{w_{j;k_1,\ldots,k_j}\}$ in a basis of standard monomials. The training is efficient because the Volterra series is linear in its coefficients: ridge regression gives the optimal estimate.

**Computational complexity of the Volterra approach.** The number of features grows as:

$$D = \sum_{j=1}^d \binom{(k_{max}+1)n + j - 1}{j}$$

For the Lorenz system ($n=3$, $k_{max}=1$, $d=2$): $D = 6 + 21 = 27$. For $n=3$, $k_{max}=2$, $d=3$: $D = 9 + 54 + 165 = 228$. For $n=10$, $k_{max}=5$, $d=3$: $D = 60 + 1830 + 37820 = 39710$ — rapidly becoming impractical. This exponential growth in feature count is the classic curse of dimensionality for Volterra methods, discussed in Chapter 1.

## The Kernel Perspective

The NVAR defines an implicit kernel on input histories. For two histories $\mathbf{u}_{t:k}$ and $\mathbf{v}_{t:k}$ (stacked recent observations), the polynomial kernel of degree $d$ is:

$$K_d(\mathbf{u}, \mathbf{v}) = (1 + \mathbf{u} \cdot \mathbf{v})^d$$

The NVAR computes predictions in the reproducing kernel Hilbert space (RKHS) of this kernel. The optimal NVAR predictor (with infinite training data) is:

$$\hat{y}_t = \sum_s \alpha_s K_d(\mathbf{u}_{t:k}, \mathbf{u}_{s:k}) y_s$$

which is a kernel regression with the polynomial kernel on stacked histories. The feature vector $\mathbf{o}_t$ is the explicit feature map $\phi: \mathbb{R}^{(k+1)n} \to \mathbb{R}^D$ such that $K_d(\mathbf{u}, \mathbf{v}) = \phi(\mathbf{u}) \cdot \phi(\mathbf{v})$.

## The Random Features Theorem and ESN

Rahimi and Recht [Rahimi2007] proved a fundamental theorem connecting kernel methods and random feature maps:

**Theorem (Rahimi & Recht 2007).** Let $k: \mathbb{R}^d \times \mathbb{R}^d \to \mathbb{R}$ be a continuous, shift-invariant kernel: $k(\mathbf{x}, \mathbf{y}) = k(\mathbf{x} - \mathbf{y})$. By Bochner's theorem, $k$ has a positive Fourier transform:

$$k(\mathbf{x} - \mathbf{y}) = \int \hat{k}(\boldsymbol{\omega}) e^{i\boldsymbol{\omega} \cdot (\mathbf{x} - \mathbf{y})} d\boldsymbol{\omega}$$

Normalize so that $\hat{k}$ is a probability distribution. Then, for random frequencies $\boldsymbol{\omega}_j \sim \hat{k}$ and random phases $b_j \sim \text{Uniform}(0, 2\pi)$, the map:

$$\mathbf{z}(\mathbf{x}) = \sqrt{\frac{2}{D}}\left[\cos(\boldsymbol{\omega}_1 \cdot \mathbf{x} + b_1), \ldots, \cos(\boldsymbol{\omega}_D \cdot \mathbf{x} + b_D)\right]^\top$$

satisfies:

$$\mathbb{E}[\mathbf{z}(\mathbf{x})^\top \mathbf{z}(\mathbf{y})] = k(\mathbf{x} - \mathbf{y})$$

Moreover, for any $\varepsilon > 0$ and compact set $\mathcal{X}$:

$$\Pr\left[\sup_{\mathbf{x}, \mathbf{y} \in \mathcal{X}} |\mathbf{z}(\mathbf{x})^\top \mathbf{z}(\mathbf{y}) - k(\mathbf{x} - \mathbf{y})| > \varepsilon\right] \leq 2 \exp\!\left(-D\varepsilon^2 / 4\right) \cdot |\mathcal{X}|_\varepsilon$$

where $|\mathcal{X}|_\varepsilon$ is the $\varepsilon$-covering number of $\mathcal{X}$.

**In words:** $D$ random cosine features provide a $O(D^{-1/2})$ approximation to any shift-invariant kernel with high probability. The approximation improves as $D$ grows.

## ESN as Random Features

An ESN computes a deterministic nonlinear feature map of the input history:

$$\mathbf{x}_t = F(\mathbf{x}_{t-1}, \mathbf{u}_t; W^{rec}, W^{in})$$

For a linear ESN ($f = \text{identity}$) with random Gaussian weights, the state at time $t$ after receiving input history $(\mathbf{u}_{t-k}, \ldots, \mathbf{u}_t)$ is:

$$\mathbf{x}_t = \sum_{k=0}^\infty (W^{rec})^k W^{in} \mathbf{u}_{t-k}$$

The inner product between two states driven by histories $\mathbf{u}$ and $\mathbf{v}$ is:

$$\mathbf{x}_t^{(\mathbf{u})} \cdot \mathbf{x}_t^{(\mathbf{v})} = \sum_{j,k=0}^\infty \mathbf{u}_{t-j}^\top (W^{in})^\top (W^{rec})^j (W^{rec})^k W^{in} \mathbf{v}_{t-k}$$

For random $W^{rec}$ and $W^{in}$ with appropriate scaling, this quantity concentrates around an explicit function of the input histories — the **reservoir kernel**:

$$K^{ESN}(\mathbf{u}_{1:T}, \mathbf{v}_{1:T}) = \lim_{N \to \infty} \frac{1}{N} \mathbf{x}_T^{(\mathbf{u})} \cdot \mathbf{x}_T^{(\mathbf{v})}$$

The precise form of $K^{ESN}$ depends on the spectral radius of $W^{rec}$ and the distribution of $W^{in}$. For a linear reservoir with Gaussian weights, it is a weighted sum of products of input time series, which is a particular shift-invariant kernel on the space of input histories.

For **nonlinear ESNs** (with tanh activations), the reservoir kernel has a more complex form. It can be computed via the **neural tangent kernel** framework or via signal propagation theory [Poole2016], and it corresponds to an infinite-width random neural network kernel.

The key conclusion: **the ESN is a random feature approximation of a smooth kernel on input histories**. The specific kernel depends on the activation function and the spectral radius. Different hyperparameter choices correspond to different kernel choices.

## Why ESN and NVAR Compute Different Kernels

This perspective explains the complementary strengths of the two methods:

| Property | NVAR | ESN |
|---|---|---|
| Kernel type | Polynomial | Smooth (RBF-like) |
| Memory depth | Explicit window $k_{max}$ | Implicit, set by $\rho$ and $\alpha$ |
| Effective dimension | $D = O(n^d k_{max}^d)$ polynomial | $N$ (reservoir size) |
| Converges to exact | As $d, k_{max} \to \infty$ | As $N \to \infty$ |
| Good for | Low-dim, polynomial dynamics | High-dim, smooth dynamics |

The polynomial kernel of NVAR is exact for targets that are polynomial functions of recent inputs. The smooth kernel of the ESN is better suited for targets that are smooth but not polynomial — which includes most high-dimensional systems and long-memory tasks.

---

## References

- [Gauthier2021] Gauthier, D.J., Bollt, E., Griffith, A., & Barbosa, W.A.S. (2021). Next generation reservoir computing. *Nature Communications*, 12, 5564.
- [Rahimi2007] Rahimi, A. & Recht, B. (2007). Random features for large-scale kernel machines. *Advances in Neural Information Processing Systems*, 20.
- [Schetzen1980] Schetzen, M. (1980). *The Volterra and Wiener Theories of Nonlinear Systems*. Wiley.
- [Poole2016] Poole, B., Lahiri, S., Raghu, M., Sohl-Dickstein, J., & Ganguli, S. (2016). Exponential expressivity in deep neural networks through transient chaos. *NIPS 2016*.
- [Gonon2020] Gonon, L. & Ortega, J.P. (2020). Reservoir computing universality with stochastic inputs. *IEEE TNNLS*, 31(1), 100–112.
