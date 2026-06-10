# NVAR and the Kernel Connection

## Polynomial Features as Kernel Evaluation

The polynomial feature map underlying NVAR has an explicit kernel interpretation. For a lagged input vector $\boldsymbol{\phi} = [\mathbf{u}_t^\top, \mathbf{u}_{t-k}^\top, \ldots]^\top \in \mathbb{R}^{sd}$, the degree-$p$ polynomial feature map $\psi_p : \mathbb{R}^{sd} \to \mathbb{R}^{D_p}$ satisfies:

$$\psi_p(\boldsymbol{\phi})^\top \psi_p(\boldsymbol{\phi}') = k_p(\boldsymbol{\phi}, \boldsymbol{\phi}'),$$

where the degree-$p$ homogeneous polynomial kernel is $k_p(\boldsymbol{\phi}, \boldsymbol{\phi}') = (\boldsymbol{\phi}^\top \boldsymbol{\phi}')^p$. The inhomogeneous version includes lower-degree terms:

$$k_d(\boldsymbol{\phi}, \boldsymbol{\phi}') = (\boldsymbol{\phi}^\top \boldsymbol{\phi}' + c)^d = \sum_{j=0}^d \binom{d}{j} c^{d-j} (\boldsymbol{\phi}^\top \boldsymbol{\phi}')^j,$$

which is the standard degree-$d$ polynomial kernel. **NVAR is equivalent to kernel regression with the degree-$d$ polynomial kernel applied to lagged inputs** [Scholkopf & Smola 2002].

This equivalence is exact, not approximate. The NVAR readout

$$\hat{y} = \mathbf{W}^{\text{out}} \psi_p(\boldsymbol{\phi})$$

is identical to the kernel regression predictor

$$\hat{y} = \sum_{t=1}^T \alpha_t k_p(\boldsymbol{\phi}_t, \boldsymbol{\phi})$$

via the representer theorem, with $\boldsymbol{\alpha} = (\mathbf{K} + \lambda \mathbf{I})^{-1} \mathbf{y}^*$ and $\mathbf{W}^{\text{out}} = \boldsymbol{\alpha}^\top \boldsymbol{\Phi}^\top$, where $\boldsymbol{\Phi}$ is the feature matrix [Scholkopf & Smola 2002].

## The Mercer Representation

Mercer's theorem guarantees that any positive-definite kernel $k : \mathcal{X} \times \mathcal{X} \to \mathbb{R}$ has a feature expansion:

$$k(\mathbf{x}, \mathbf{y}) = \sum_{i=1}^\infty \lambda_i \phi_i(\mathbf{x}) \phi_i(\mathbf{y}),$$

where $\{\phi_i\}$ are orthonormal eigenfunctions of the kernel integral operator and $\lambda_i \geq 0$ are eigenvalues [Mercer 1909]. For the polynomial kernel, this expansion has finitely many terms. For the RBF kernel, it is infinite.

The feature map is $\boldsymbol{\Phi}(\mathbf{x}) = [\sqrt{\lambda_1}\phi_1(\mathbf{x}), \sqrt{\lambda_2}\phi_2(\mathbf{x}), \ldots]^\top$, so $k(\mathbf{x}, \mathbf{y}) = \boldsymbol{\Phi}(\mathbf{x})^\top \boldsymbol{\Phi}(\mathbf{y})$. NVAR uses the exact, finite-dimensional Mercer expansion of the polynomial kernel. ESNs implicitly use a random approximation to the Mercer expansion of an unknown (task-dependent) kernel.

## Random Fourier Features and the ESN Connection

Rahimi & Recht [2007] showed that any shift-invariant kernel $k(\mathbf{x} - \mathbf{y})$ can be approximated by random Fourier features:

$$k(\mathbf{x} - \mathbf{y}) \approx z(\mathbf{x})^\top z(\mathbf{y}),$$

where $z(\mathbf{x}) = \sqrt{2/N}[\cos(\boldsymbol{\omega}_1^\top \mathbf{x} + b_1), \ldots, \cos(\boldsymbol{\omega}_N^\top \mathbf{x} + b_N)]^\top$ with $\boldsymbol{\omega}_i \sim p(\boldsymbol{\omega})$ drawn from the Fourier transform of $k$ and $b_i \sim \text{Uniform}[0, 2\pi]$. The approximation error is:

$$\sup_{\mathbf{x}, \mathbf{y}} |k(\mathbf{x}-\mathbf{y}) - z(\mathbf{x})^\top z(\mathbf{y})| \leq O\!\left(\sqrt{\frac{\log N}{N}}\right),$$

with high probability [Rahimi & Recht 2007].

ESN reservoir states play the role of $z(\mathbf{u}_t)$ — random features of the input history. The specific kernel being approximated depends on the reservoir's random weight distribution, the nonlinearity, and the input statistics. For Gaussian random weights and $\tanh$ nonlinearity, the implied kernel is approximately a nonlinear function of the input correlation, related to the arc-cosine kernel [Cho & Saul 2009].

## NVAR vs. ESN as Kernel Machines

The unifying view is:

| Model | Kernel | Feature map | Exact or approximate |
|-------|--------|-------------|---------------------|
| NVAR degree-$p$ | Polynomial $(c + \boldsymbol{\phi}^\top \boldsymbol{\phi}')^p$ | Explicit monomials | Exact |
| ESN $N$-neurons | Implicit shift-invariant kernel | Random projections + $\tanh$ | Approximate, $O(N^{-1/2})$ |
| Hybrid NVAR+ESN | Polynomial + implicit kernel | Monomials + random projections | Hybrid |

NVAR is exact kernel computation; the ESN is approximate kernel computation via random features. When is exact better? When the kernel is well-matched to the task and the polynomial degree is low. When is approximate better? When the relevant kernel is unknown, high-dimensional, or non-polynomial, so that the random feature approximation of a flexible implicit kernel outperforms any fixed polynomial kernel.

## Choosing the Kernel

This unified perspective suggests a principled approach to method selection. If the dynamics of the system being modeled are known (e.g., polynomial ODEs), use NVAR with the matched polynomial degree — exact kernel computation is optimal. If the dynamics are unknown, use ESN — random features provide an unbiased estimate of the optimal kernel without committing to a specific functional form. If partial structure is known, use the hybrid: include polynomial features for the known terms and reservoir features for the residual [Bollt 2021].

---

## References

- Rahimi, A., & Recht, B. (2007). Random features for large-scale kernel machines. *Advances in Neural Information Processing Systems*, 20.
- Mercer, J. (1909). Functions of positive and negative type and their connection with the theory of integral equations. *Philosophical Transactions of the Royal Society A*, 209, 415–446.
- Scholkopf, B., & Smola, A. J. (2002). *Learning with Kernels*. MIT Press.
- Bollt, E. (2021). On explaining the surprising success of reservoir computing forecaster of chaos? *Chaos*, 31(1), 013108.
