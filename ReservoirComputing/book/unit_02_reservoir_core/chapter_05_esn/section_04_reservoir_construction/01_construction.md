# Section 5.4: Reservoir Construction in Practice

## 5.4.1 Overview

Constructing an echo state network reservoir involves three independent operations: generating the recurrent weight matrix $W^{rec}$, generating the input weight matrix $W^{in}$, and setting the bias vector $\mathbf{b}$. Each operation has standard recipes with well-understood effects on reservoir dynamics. This section develops the construction procedure step by step, with the mathematical justification for each choice [Jaeger2001, Lukosevicius2012].

## 5.4.2 Step 1: Generate the Raw Recurrent Weight Matrix

Begin by drawing an $N \times N$ matrix $\tilde{W}$ with entries sampled independently from a zero-mean distribution. The two standard choices are:

**Gaussian weights:**
$$\tilde{W}_{ij} \sim \mathcal{N}(0, 1), \quad \text{independently for all } i, j.$$

**Uniform weights:**
$$\tilde{W}_{ij} \sim \text{Uniform}[-1, 1], \quad \text{independently for all } i, j.$$

For large $N$, the two choices produce nearly identical spectral properties (universality of the circular law; see Section 27.1). Zero mean is important: a matrix with positive mean has a dominant real eigenvalue driven by the mean, which creates a single saturating mode rather than a diverse set of timescales.

**Sparse connectivity.** Rather than a fully connected reservoir, introduce a connectivity fraction $p \in (0, 1]$: retain each weight independently with probability $p$ and set it to zero otherwise. Concretely, generate a binary mask $M_{ij} \sim \text{Bernoulli}(p)$ and set

$$\tilde{W}^{sparse}_{ij} = M_{ij} \cdot \tilde{W}_{ij}.$$

The resulting matrix has expected number of nonzero entries $pN^2$. For $N = 100$ and $p = 0.1$, this is 1000 connections — sparse enough to store and multiply efficiently.

**Expected spectral radius of sparse random matrices.** By the Wigner semicircle law (adapted for sparse matrices), the spectral radius of $\tilde{W}^{sparse}$ concentrates near

$$\rho(\tilde{W}^{sparse}) \approx 2\sigma\sqrt{p \cdot N},$$

where $\sigma^2$ is the variance of the nonzero entries. For Gaussian entries with $\sigma^2 = 1$ and $p$ such that $pN$ is large, this is the relevant scaling. In the sparse regime where $p$ is fixed as $N \to \infty$, the spectral radius concentrates near $2\sigma\sqrt{p}$ after the standard $1/\sqrt{N}$ normalization [Tao2012]. Practically, you compute $\rho(\tilde{W})$ numerically (Step 3) rather than relying on asymptotic formulas.

## 5.4.3 Step 2: Normalize to the Target Spectral Radius

Let $\rho_0 = \rho(\tilde{W})$ denote the spectral radius of the raw matrix, computed as the magnitude of the largest eigenvalue:

$$\rho_0 = \max_{i} |\lambda_i(\tilde{W})|.$$

Scale the matrix to the target spectral radius $\rho$:

$$W^{rec} = \frac{\rho}{\rho_0} \cdot \tilde{W}.$$

**Why this works.** Scaling all weights by a constant $\alpha$ scales all eigenvalues by $\alpha$ (by linearity of the characteristic polynomial). Therefore $\rho(\alpha \tilde{W}) = \alpha \rho(\tilde{W})$, and the choice $\alpha = \rho / \rho_0$ gives $\rho(W^{rec}) = \rho$ exactly.

**Numerical computation.** Computing all $N$ eigenvalues costs $O(N^3)$. For large $N$, use power iteration or the implicitly-restarted Arnoldi method (as in `scipy.sparse.linalg.eigs`) to find only the largest eigenvalue in $O(N^2)$ or $O(kNn_{iter})$ for sparse matrices with $k$ Arnoldi vectors.

**The target value $\rho$.** The spectral radius is the primary hyperparameter controlling the memory-nonlinearity tradeoff (Chapter 7). Standard guidance [Jaeger2001]:
- $\rho < 1$: required for the echo state property in linear reservoirs.
- $\rho \approx 0.9$: near the edge of chaos; good default for many tasks.
- $\rho > 1$: can still have ESP in the nonlinear reservoir due to the compressive nonlinearity (Section 5.2), but dynamics are richer and potentially chaotic.

## 5.4.4 Step 3: Generate Input Weights

The input weight matrix $W^{in} \in \mathbb{R}^{N \times K}$ (for $K$-dimensional input) injects the external signal into the reservoir. Standard choices:

**Uniform signed input weights:**
$$W^{in}_{ij} \sim \sigma_{in} \cdot \text{Uniform}\{-1, +1\},$$
where $\sigma_{in}$ is the input scaling hyperparameter. Signed weights are preferred to ensure each neuron is driven in both directions, preventing all neurons from saturating on the same side.

**Gaussian input weights:**
$$W^{in}_{ij} \sim \mathcal{N}(0, \sigma_{in}^2).$$

For a scalar input ($K = 1$), $W^{in} \in \mathbb{R}^N$ is a single column vector. It is common to connect the input to only a subset of neurons (randomly chosen, each connected with probability $p_{in}$), creating sparse input connectivity.

**Effect of $\sigma_{in}$.** Large $\sigma_{in}$ drives neurons into saturation, reducing effective dimensionality but increasing nonlinear mixing. Small $\sigma_{in}$ keeps neurons in the linear regime, preserving memory but reducing nonlinear capacity (Section 7.3).

## 5.4.5 Signed vs. All-Positive Weights

The standard choice of zero-mean weights (roughly half positive, half negative) has an important property: it prevents the reservoir from collapsing into a single saturated mode.

**All-positive weights** ($W^{rec}_{ij} \geq 0$) are sometimes used when the application demands non-negative activations (e.g., spiking neuron models, rate models with non-negative firing rates). However, all-positive recurrent weights tend to produce a dominant positive eigenvalue that drives the mean activation to saturation, substantially reducing effective dimensionality. If all-positive weights are required (e.g., by Dale's law constraints in neurobiologically plausible models; see Section 6.6), use inhibitory connections through a separate population.

**Balanced signed weights** achieve approximately equal total excitatory and inhibitory drive, which is necessary for the balanced amplification mechanism (Section 6.6) and is the condition under which random matrices have spectral radius governed by the Wigner/circular law rather than by the mean.

## 5.4.6 Alternative Topologies

**Ring topology (Simple Cycle Reservoir).** Set $W^{rec}_{i, i-1 \pmod{N}} = \rho$ and all other entries to zero (Section 9.2). This achieves maximum linear memory capacity $MC = N$ but zero nonlinear mixing capacity.

**Delay-line reservoir (DLR).** A variant with feedback connections added to the ring, creating a slightly richer eigenspectrum while retaining $O(N)$ connectivity. Rodan and Tino [RodanTino2011, RodanTino2012] characterized the performance of several such topologies.

**Small-world and scale-free topologies.** Watts-Strogatz small-world graphs (high clustering, short paths) and Barabasi-Albert scale-free graphs (power-law degree distributions) have been studied as reservoir topologies (Section 9.6). Results are mixed: structured topologies can outperform random Erdos-Renyi graphs on specific tasks but rarely provide consistent improvements across task classes [Lukosevicius2012].

## 5.4.7 The Bias Vector

The bias vector $\mathbf{b} \in \mathbb{R}^N$ shifts each neuron's operating point:

$$\mathbf{x}(t) = (1 - \alpha)\mathbf{x}(t-1) + \alpha \tanh(W^{rec} \mathbf{x}(t-1) + W^{in} \mathbf{u}(t) + \mathbf{b}),$$

where $\alpha \in (0, 1]$ is the leak rate. Random biases $b_i \sim \text{Uniform}[-\sigma_b, \sigma_b]$ break the symmetry of the reservoir's zero-state initialization, ensuring that different neurons begin in different regions of the tanh nonlinearity even before any input arrives.

Without bias, the zero initial state $\mathbf{x}(0) = \mathbf{0}$ is a fixed point of the reservoir dynamics when $\mathbf{u}(t) = 0$, and small inputs produce only small, symmetric responses. A nonzero bias breaks this degeneracy. Typical bias scaling: $\sigma_b \in [0.1, 1.0]$.

## 5.4.8 Complete Construction Algorithm

The complete procedure is as follows.

**Input:** $N$ (reservoir size), $\rho$ (target spectral radius), $p$ (connectivity fraction), $\sigma_{in}$ (input scaling), $\sigma_b$ (bias scaling), $K$ (input dimension).

1. Draw $\tilde{W}_{ij} \sim \mathcal{N}(0, 1)$ for all $i, j \in \{1, \ldots, N\}$.
2. Apply sparse mask: $\tilde{W}_{ij} \leftarrow 0$ with probability $1 - p$, independently.
3. Compute $\rho_0 = \rho(\tilde{W})$ (largest eigenvalue magnitude).
4. Set $W^{rec} = (\rho / \rho_0) \cdot \tilde{W}$.
5. Draw $W^{in}_{ij} \sim \sigma_{in} \cdot \text{Uniform}\{-1, +1\}$ for $i \in \{1,\ldots,N\}$, $j \in \{1,\ldots,K\}$.
6. Draw $b_i \sim \text{Uniform}[-\sigma_b, \sigma_b]$ for $i \in \{1, \ldots, N\}$.

The reservoir $(W^{rec}, W^{in}, \mathbf{b})$ is then fixed for all subsequent training and evaluation.

---

## References

- **[Jaeger2001]** H. Jaeger. "The echo state approach to analysing and training recurrent neural networks." *GMD Report 148*, German National Research Center for Information Technology, 2001.
- **[Lukosevicius2012]** M. Lukosevicius. "A practical guide to applying echo state networks." In *Neural Networks: Tricks of the Trade*, Springer, pp. 659-686, 2012.
- **[RodanTino2011]** A. Rodan and P. Tino. "Minimum complexity echo state network." *IEEE Transactions on Neural Networks*, 22(1):131-144, 2011.
- **[RodanTino2012]** A. Rodan and P. Tino. "Simple deterministically constructed cycle reservoirs with regular jumps." *Neural Computation*, 24(7):1822-1852, 2012.
- **[Tao2012]** T. Tao. *Topics in Random Matrix Theory*. American Mathematical Society, 2012.
