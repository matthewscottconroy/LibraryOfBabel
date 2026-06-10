# Reservoir Computing and Attractor Reconstruction

## Takens' Embedding Theorem

The theoretical foundation for chaotic time series prediction from scalar observations is Takens' [1981] delay embedding theorem. Let $M$ be a compact smooth manifold, $\phi : M \to M$ a smooth diffeomorphism (the dynamics), and $h : M \to \mathbb{R}$ a smooth observation function. The delay embedding map $\Phi_\tau : M \to \mathbb{R}^{2m+1}$ defined by

$$\Phi_\tau(x) = [h(x), h(\phi^{-\tau}(x)), h(\phi^{-2\tau}(x)), \ldots, h(\phi^{-2m\tau}(x))]^\top$$

is a diffeomorphism onto its image for generic $(h, \tau)$, provided the embedding dimension is $2m + 1 \geq 2d_A + 1$, where $d_A = \dim(M)$ is the attractor dimension [Takens 1981].

The theorem guarantees that the delay embedding is topologically equivalent (homeomorphic) to the original attractor — it faithfully reconstructs the attractor's geometry in delay-coordinate space. This has two implications for reservoir computing: (1) scalar observations are sufficient to reconstruct the full attractor, and (2) any smooth function of the attractor state (such as the next observation) can be computed as a smooth function of the delay embedding.

## The Reservoir as a Nonlinear Delay Embedding

A reservoir with fading memory implements a nonlinear generalization of the Takens embedding. The reservoir state at time $t$ is a smooth function of the input history:

$$\mathbf{x}_t = \Phi(\mathbf{u}_{(-\infty, t]}),$$

where $\Phi$ is the echo state map. For a scalar input $u_t = h(s_t)$ (observation of the hidden state $s_t$), the reservoir state is a nonlinear function of the delay embedding of $h$. By Takens' theorem, for sufficiently many reservoir neurons and appropriate connectivity, the reservoir state contains all information about the hidden attractor state that is contained in the observation history.

The Boyd–Chua theorem for continuous systems [Boyd & Chua 1985] provides the reservoir-computing analog of Takens' theorem: any fading-memory functional of the input can be uniformly approximated by a reservoir with a linear readout. Together, these two theorems guarantee that an ESN with sufficient size can predict any smooth function of the hidden attractor state from a scalar observation, provided the attractor is compact and the observation function is generic.

## Attractor Dimension Estimation from Reservoir States

After running the reservoir over a long chaotic time series, the collected reservoir states $\{\mathbf{x}_t\}$ lie on a manifold (the echo state manifold) in $\mathbb{R}^N$. The geometry of this manifold should reflect the geometry of the input attractor, by the Takens/Boyd–Chua argument.

The attractor dimension can be estimated from the reservoir states using the correlation dimension estimator [Grassberger & Procaccia 1983]:

$$D_2 = \lim_{\epsilon \to 0} \frac{\log C(\epsilon)}{\log \epsilon}, \quad C(\epsilon) = \lim_{T \to \infty} \frac{2}{T(T-1)} \sum_{i < j} \mathbf{1}[\|\mathbf{x}_i - \mathbf{x}_j\| < \epsilon].$$

If the reservoir correctly embeds the input attractor, the correlation dimension computed from reservoir states should match that computed from the raw observations. This provides a consistency check on the reservoir's fidelity as an embedding.

## Lu et al. 2017: Reservoir Attractor Reconstruction

Lu et al. [2017] demonstrated that an ESN can accurately reconstruct the Lorenz attractor from partial observations. Their setup: observe only $x(t)$ (not $y$ or $z$), train an ESN with $N = 1000$ neurons, and evaluate whether the autonomous ESN (in closed loop) reproduces the statistics of the full Lorenz attractor.

**Metrics used:**
- Correlation dimension $D_2$ of the reservoir attractor vs. true Lorenz attractor ($D_2^{\text{Lorenz}} \approx 2.05$)
- Power spectrum comparison
- Lyapunov exponent of the autonomous ESN

Their results: the trained ESN, run autonomously, produced a strange attractor with $D_2 \approx 2.05$ (matching Lorenz), qualitatively correct power spectrum, and positive Lyapunov exponents. The ESN had successfully reconstructed the Lorenz attractor from single-variable observations — a nontrivial result that validates the Takens/reservoir connection [Lu et al. 2017].

## Demonstrating Attractor Reconstruction

The practical test of attractor reconstruction by a reservoir is a three-step procedure:

1. **Compute the correlation dimension of the true attractor** from the observed time series using the Grassberger–Procaccia algorithm.
2. **Run the trained reservoir in autonomous mode** for a long time, collecting the predicted trajectory $\{\hat{\mathbf{u}}_t\}_{t \geq T_{\text{train}}}$.
3. **Compute the correlation dimension of the predicted trajectory** and compare.

Agreement indicates that the reservoir has learned the attractor geometry, not merely a good short-term predictor. Disagreement (e.g., reservoir collapses to a fixed point or limit cycle) indicates that the autonomous dynamics diverge from the true attractor.

---

## References

- Takens, F. (1981). Detecting strange attractors in turbulence. In D. A. Rand & L. S. Young (Eds.), *Dynamical Systems and Turbulence* (pp. 366–381). Springer.
- Lu, Z., Pathak, J., Hunt, B., Girvan, M., Brockett, R., & Ott, E. (2017). Reservoir observers: Model-free inference of unmeasured variables in chaotic systems. *Chaos*, 27(4), 041102.
- Grassberger, P., & Procaccia, I. (1983). Characterization of strange attractors. *Physical Review Letters*, 50(5), 346–349.
