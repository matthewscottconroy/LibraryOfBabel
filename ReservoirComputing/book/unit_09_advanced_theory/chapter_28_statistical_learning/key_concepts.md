# Key Concepts: Chapter 28 — Statistical Learning Theory for Reservoir Computing

## PAC Learning

**Probably Approximately Correct (PAC) learning** [Valiant 1984]: a framework for quantifying how much data is needed for a learning algorithm to generalize. An algorithm PAC-learns a class $\mathcal{H}$ if, for any $\varepsilon, \delta > 0$, it can output a hypothesis with true error $\leq \varepsilon$ with probability $\geq 1-\delta$, given a number of examples that depends polynomially on $1/\varepsilon$, $1/\delta$, and problem parameters.

*In the reservoir context:* PAC learning guarantees that a linear readout trained on $T \sim O(N/\varepsilon)$ examples will generalize, regardless of the input distribution.

## VC Dimension

**Vapnik-Chervonenkis (VC) dimension** [Vapnik & Chervonenkis 1971]: the size of the largest set of points that the hypothesis class can *shatter* (label in all possible ways). For linear halfspaces in $\mathbb{R}^N$, $d_\mathrm{VC} = N$.

*Key result:* A class is PAC learnable if and only if its VC dimension is finite. Sample complexity $m(\varepsilon, \delta) = O(d_\mathrm{VC}/\varepsilon \cdot \log(d_\mathrm{VC}/\varepsilon) + (1/\varepsilon)\log(1/\delta))$.

## Sauer-Shelah Lemma

The growth function $\Pi_\mathcal{H}(m)$ — the maximum number of distinct labelings $\mathcal{H}$ produces on $m$ points — satisfies $\Pi_\mathcal{H}(m) \leq (em/d_\mathrm{VC})^{d_\mathrm{VC}}$ for $m > d_\mathrm{VC}$. This polynomial (rather than exponential) growth is what makes large-VC-dimension classes PAC learnable.

## Rademacher Complexity

**Rademacher complexity** $\mathcal{R}_m(\mathcal{F})$: measures the ability of function class $\mathcal{F}$ to correlate with random $\pm 1$ labels. Defined as

$$
\mathcal{R}_m(\mathcal{F}) = \mathbb{E}_{\sigma, S}\!\left[\sup_{f \in \mathcal{F}} \frac{1}{m}\sum_i \sigma_i f(x_i)\right].
$$

Yields distribution-dependent generalization bounds: $\mathcal{L}(f) \leq \hat{\mathcal{L}}(f) + 2\mathcal{R}_m(\mathcal{F}) + O(\sqrt{\ln(1/\delta)/m})$.

*Key property:* For the linear readout class $\{\mathbf{w}^T\mathbf{x} : \|\mathbf{w}\|_2 \leq B\}$, $\mathcal{R}_m = O(B\sqrt{\mathbb{E}\|\mathbf{x}\|^2}/\sqrt{m})$ — depends on the geometry of reservoir states, not on $N$ directly.

## Covering Numbers and Metric Entropy

**$\varepsilon$-covering number** $\mathcal{N}(\mathcal{F}, \varepsilon, d)$: minimum number of balls of radius $\varepsilon$ needed to cover $\mathcal{F}$. **Metric entropy**: $\log \mathcal{N}(\mathcal{F}, \varepsilon)$.

**Dudley's integral bound**: $\mathcal{R}_m(\mathcal{F}) \leq \frac{12}{\sqrt{m}}\int_0^\infty \sqrt{\log \mathcal{N}(\mathcal{F}, \varepsilon)}\,d\varepsilon$.

*In the reservoir context:* If reservoir states lie in a low-dimensional subspace (stable rank $r(\Sigma) \ll N$), the effective covering number is that of a $r(\Sigma)$-dimensional class.

## Double Descent

The **double-descent** phenomenon [Belkin et al. 2019]: test error follows a U-shaped curve in the underparameterized regime ($N < T$), peaks at the interpolation threshold ($N \approx T$), and decreases again in the overparameterized regime ($N > T$).

**Benign overfitting** [Bartlett et al. 2020]: conditions under which the minimum-norm interpolating solution generalizes despite fitting noise. Requires the eigenvalue spectrum of the state covariance to have a slowly decaying tail.

*Reservoir implication:* Large reservoirs ($N \gg T$) fit minimum-norm readouts that can generalize if the state covariance spectrum decays slowly (polynomially).

## Minimum-Norm Least Squares

**Minimum-norm interpolating solution**: $\hat{\mathbf{w}}_{\min} = \mathbf{X}^+\mathbf{y}$ (Moore-Penrose pseudoinverse). When $N > T$, this is the unique solution to $\mathbf{X}\mathbf{w} = \mathbf{y}$ with smallest $\ell^2$ norm.

*Key fact:* Gradient descent initialized at $\mathbf{0}$ converges to $\hat{\mathbf{w}}_{\min}$ in the overparameterized regime.

## Implicit Regularization

**Implicit regularization**: the regularization effect induced by the optimization algorithm, independent of any explicit penalty term. Gradient descent implicitly regularizes toward small-norm solutions; coordinate descent toward sparse solutions.

**Early stopping as regularization**: stopping gradient descent at step $k$ approximates ridge regression with $\lambda \approx 1/(\eta k)$.

## Neural Tangent Kernel (NTK)

**Neural tangent kernel** [Jacot et al. 2018]: $K_\mathrm{NTK}(x,x') = \langle \partial_\theta f_\theta(x), \partial_\theta f_\theta(x') \rangle$. In the infinite-width limit, gradient flow is equivalent to kernel regression with $K_\mathrm{NTK}$.

*Reservoir NTK:* For fixed reservoir with linear readout, $K_\mathrm{RC}(\mathbf{x}, \mathbf{x}') = \mathbf{x}^T\mathbf{x}'$ (linear kernel). Implicit regularizer is the $\ell^2$ norm of the readout vector.

## Stable Rank

**Stable rank** of a matrix $A$: $r(A) = \|A\|_F^2/\|A\|_\mathrm{op}^2 = \sum_i \sigma_i^2/\sigma_1^2$. For the state covariance $\Sigma$, $r(\Sigma) = \mathrm{tr}(\Sigma)/\lambda_1(\Sigma)$. Measures the effective dimensionality of the reservoir state distribution.

## References

- Bartlett, P. L., Montanari, A., and Rakhlin, A. (2020). Benign overfitting in linear regression. *PNAS*, 117(48), 30063–30070.
- Belkin, M., Hsu, D., Ma, S., and Mandal, S. (2019). Reconciling modern machine-learning practice and the classical bias-variance trade-off. *PNAS*, 116(32), 15849–15854.
- Jacot, A., Gabriel, F., and Hongler, C. (2018). Neural tangent kernel. *NeurIPS*, 31.
- Valiant, L. G. (1984). A theory of the learnable. *CACM*, 27(11), 1134–1142.
- Vapnik, V. N. and Chervonenkis, A. Y. (1971). On the uniform convergence of relative frequencies of events to their probabilities. *Theory of Probability and its Applications*, 16(2), 264–280.
