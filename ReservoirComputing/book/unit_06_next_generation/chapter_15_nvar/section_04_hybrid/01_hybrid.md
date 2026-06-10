# Hybrid NVAR-Reservoir Approaches

## The Complementary Strengths Argument

NVAR and ESN have complementary failure modes. NVAR fails when inputs are high-dimensional or when temporal dependencies extend beyond the delay window. ESN fails when the polynomial structure of the dynamics is known but the random reservoir fails to recover it efficiently. The natural response is a hybrid: combine both feature sets in a single readout.

The hybrid feature vector is:

$$\mathbf{h}_t = [\mathbf{x}_t; \mathbf{o}_t^{\text{poly}}] \in \mathbb{R}^{N + D_{\text{NVAR}}},$$

where $\mathbf{x}_t \in \mathbb{R}^N$ is the reservoir state and $\mathbf{o}_t^{\text{poly}} \in \mathbb{R}^{D_{\text{NVAR}}}$ is the polynomial feature vector. The readout is a linear map on $\mathbf{h}_t$:

$$\hat{\mathbf{y}}_t = \mathbf{W}^{\text{out}} \mathbf{h}_t,$$

trained by ridge regression on the combined feature matrix [Gauthier et al. 2021]. The reservoir provides global temporal context (nonlinear functions of the entire input history), while the polynomial features provide local, structurally matched features for the immediate temporal neighborhood.

## Why Combining Helps

The reservoir and polynomial features are genuinely complementary in their information content. The polynomial features $\mathbf{o}_t^{\text{poly}}$ are deterministic and exactly capture degree-$p$ monomials of $s$ lagged inputs. The reservoir states $\mathbf{x}_t$ are random projections of the infinite input history, capturing (in expectation) all computable functions of the past through the universal approximation guarantee.

The information contained in $\mathbf{o}_t^{\text{poly}}$ is already present in $\mathbf{x}_t$ (since the reservoir state is a function of the full history, including the $s$ most recent steps). However, the reservoir may not represent this information in a linearly accessible form — the polynomial features make it explicitly accessible. Conversely, $\mathbf{x}_t$ contains information about the distant past that $\mathbf{o}_t^{\text{poly}}$ does not. The combination ensures that both recent polynomial structure and distant history are available to the linear readout [Bollt 2021].

## The Random Feature Connection

The connection between reservoir states and random features of the input history is precise. Reservoir states are approximately random features of the input signal in the sense of Rahimi & Recht [2007]: they approximate the evaluation of a kernel function between the current input history and all training histories.

Formally, if the reservoir has fading memory and sufficient size, then for any two input histories $\mathbf{u}_{(-\infty,t]}$ and $\mathbf{v}_{(-\infty,s]}$:

$$\mathbf{x}_t^{(u) \top} \mathbf{x}_s^{(v)} \approx k(\mathbf{u}_{(-\infty,t]}, \mathbf{v}_{(-\infty,s]}),$$

where $k$ is the kernel induced by the reservoir's random feature map. By the Rahimi–Recht theorem, for random features drawn from a Fourier basis:

$$\hat{k}(\mathbf{x}, \mathbf{y}) = \frac{1}{N}\sum_{i=1}^N \phi(\boldsymbol{\omega}_i^\top \mathbf{x})\phi(\boldsymbol{\omega}_i^\top \mathbf{y}) \xrightarrow{N \to \infty} k_{\text{RBF}}(\mathbf{x}, \mathbf{y}),$$

where $\boldsymbol{\omega}_i \sim p(\boldsymbol{\omega})$ are drawn from the Fourier transform of the RBF kernel. The reservoir's $\tanh$ nonlinearity provides a different but related random feature map. The convergence rate is $O(1/\sqrt{N})$, so larger reservoirs better approximate the underlying kernel [Rahimi & Recht 2007].

In this view, NVAR is exact kernel regression with the polynomial kernel, and ESN is approximate kernel regression with an implicit shift-invariant kernel. The hybrid combines both, accessing both polynomial structure and the random-feature approximation.

## The NVAR-to-ESN Continuum

The hybrid framework reveals a continuum of architectures parameterized by the relative weight given to polynomial versus reservoir features:

- **Pure NVAR ($N = 0$):** Only polynomial features; deterministic, low capacity for large $d$.
- **Hybrid:** Both polynomial and reservoir features; best of both worlds, highest capacity.
- **Pure ESN ($D_{\text{NVAR}} = 0$):** Only reservoir features; random, universally expressive but not parameter-efficient for structured tasks.

Moving along this continuum, one interpolates between exact polynomial computation and random approximation of an unknown kernel. The optimal position depends on the task: structured polynomial dynamics favor NVAR; complex high-dimensional temporal tasks favor ESN; mixed tasks favor the hybrid [Gauthier et al. 2021].

## Practical Recommendations

For Lorenz-type systems: the pure NVAR is sufficient and recommended for its simplicity and reproducibility. For high-dimensional systems with known low-order polynomial terms (e.g., turbulence, where the Navier–Stokes equations are quadratic): include polynomial features for the known terms and reservoir features for the residual dynamics. For black-box time series: pure ESN or hybrid with small $D_{\text{NVAR}}$ as a regularization term that ensures polynomial structure is captured exactly.

---

## References

- Gauthier, D. J., Bollt, E., Griffith, A., & Barbosa, W. A. S. (2021). Next generation reservoir computing. *Nature Communications*, 12(1), 5564.
- Rahimi, A., & Recht, B. (2007). Random features for large-scale kernel machines. *Advances in Neural Information Processing Systems*, 20.
- Bollt, E. (2021). On explaining the surprising success of reservoir computing forecaster of chaos? *Chaos*, 31(1), 013108.
