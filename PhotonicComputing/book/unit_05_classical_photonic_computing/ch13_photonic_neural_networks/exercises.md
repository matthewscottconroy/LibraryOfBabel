# Chapter 13: Exercises

## Mathematical Exercises

**13.1** (Operation accounting: MLP and transformer) Where the arithmetic lives, from Subsections 13.1.1 and 13.5.1.

(a) For the MLP $784 \to 300 \to 100 \to 10$, tabulate MACs and pointwise activations per layer, total each, and the fraction of arithmetic that is MACs.

(b) For one transformer block with model dimension $d = 768$, sequence length $L = 512$, $h = 12$ heads, and a $4\times$ feedforward expansion, count the MACs in (i) the four projections $W_Q, W_K, W_V, W_O$ ($4Ld^2$), (ii) the attention products $QK^{\top}$ and $AV$ ($2L^2d$), and (iii) the feedforward sublayer ($8Ld^2$). Report each as a fraction of the block total.

(c) Which of the three groups in (b) is *dynamic* (activation-by-activation) rather than weight-stationary? Give its fraction, and find the sequence length $L^{\star}$ at which the attention products equal the projection cost.

(d) In autoregressive generation with a cached key/value history of length $L$, the per-new-token attention cost is $O(Ld)$ and grows with context while the projection cost per token is fixed at $O(d^2)$. At $d = 768$, beyond what context length does attention dominate a single generated token? Comment on why long-context inference is attention-bound.

**13.2** (Gradient estimation and noise-aware flatness) From Subsections 13.3.2 and 13.3.3.

(a) For SPSA with random signs $\Delta_k = \pm 1$, show the single-sample estimate $\widehat{g}_k = [\mathcal{L}(\boldsymbol\theta + \epsilon\boldsymbol\Delta) - \mathcal{L}(\boldsymbol\theta - \epsilon\boldsymbol\Delta)]/(2\epsilon\Delta_k)$ is an unbiased estimate of $\partial\mathcal{L}/\partial\theta_k$ to $O(\epsilon^2)$, and that its dominant error term is $\sum_{j\neq k} g_j \Delta_j/\Delta_k$, of variance $\approx \|\nabla\mathcal{L}\|^2 - g_k^2$.

(b) A coherent mesh with $N = 64$ modes has $P = N(N-1)/2 = 2016$ phase parameters. Single-sample SPSA has directional cosine $\sim 1/\sqrt{P}$ with the true gradient, so $\sim P$ samples are needed to match one exact-gradient step. With two weight reprogrammings per SPSA pair at 20 μs each, estimate the wall-clock time for one such effective gradient step, and identify the binding cost.

(c) Expand $\mathcal{L}(\boldsymbol\theta + \boldsymbol\delta) \approx \mathcal{L}(\boldsymbol\theta) + \nabla\mathcal{L}^{\top}\boldsymbol\delta + \tfrac12\boldsymbol\delta^{\top}H\boldsymbol\delta$ for zero-mean phase noise $\boldsymbol\delta$ with covariance $\sigma^2 I$. Show $\mathbb{E}[\mathcal{L}] \approx \mathcal{L} + \tfrac12\sigma^2\,\mathrm{tr}(H)$ near a minimum, and interpret why training with injected noise selects flat (low-$\mathrm{tr}\,H$) minima.

(d) Two minima have equal training loss but Hessian traces $\mathrm{tr}(H) = 500$ and $5000$ (loss units per rad$^2$). For deployment phase noise $\sigma = 0.03$ rad, compute the expected excess loss of each and state which to deploy.

**13.3** (Reservoir readout and memory) From Subsections 13.4.1 and 13.4.2.

(a) The linear readout minimizes $\|W_{\text{out}}X - Y\|_F^2 + \lambda\|W_{\text{out}}\|_F^2$, where the columns of $X \in \mathbb{R}^{N \times T}$ are reservoir states. Derive the ridge solution $W_{\text{out}} = Y X^{\top}(X X^{\top} + \lambda I)^{-1}$ and explain the role of $\lambda$ for an ill-conditioned state-collection matrix.

(b) For a linearized reservoir $\mathbf{x}(n) = A\mathbf{x}(n-1) + \mathbf{b}\,u(n)$, unroll to $\mathbf{x}(n) = \sum_{k\geq 0} A^k \mathbf{b}\,u(n-k)$ and show the echo-state property requires spectral radius $\rho(A) < 1$. For $\rho = 0.90$, compute the memory timescale $\tau_m \approx -1/\ln\rho$ in steps.

(c) State the linear memory-capacity bound $\mathrm{MC} = \sum_{k\geq 1}\mathrm{corr}^2(\widehat{y}_k, u(n-k)) \leq N$. For a single-node time-delay reservoir with $N = 50$ virtual nodes, give the maximum $\mathrm{MC}$ and explain why raising $\rho$ toward 1 trades memory depth against nonlinear processing.

(d) For $\rho = 0.95$, how many past input steps contribute more than 1% weight ($\rho^k > 0.01$)?

---

## Design Exercises

**13.4** (Activation strategy under a power budget) A 4-hidden-layer network has 256 neurons per layer (1024 activations total) and a 1 W on-chip budget for the activation subsystem. Use O-E-O at 1 pJ/activation, $\sim$100 ps latency (Subsection 13.2.2).

(a) Compute the O-E-O activation power at 10 GS/s and at 1 GS/s per neuron. Which throughput fits the budget?

(b) A saturable-absorber all-optical activation needs $\sim$1 mW hold power per neuron to sit near saturation; a photonic-crystal nanocavity bistable switch (Nozaki et al., 2010) switches at $\sim$0.42 fJ but with $Q \sim 10^4$ ($\sim$8 ps photon lifetime, $\sim$50 μW threshold power). Compute the aggregate optical power each would demand for 1024 neurons and compare with (a).

(c) Neither all-optical device provides electronic gain or level restoration. Over 4 cascaded layers, argue qualitatively what happens to signal levels and why O-E-O's re-modulation solves it (contrast with the cascadability failure of Chapter 11 optical logic).

(d) Invoking the $O(N)$-activations-versus-$O(N^2)$-matrix argument of Subsection 13.1.1, decide and justify an activation strategy for this network at 1 GS/s, including which quantity actually limits the budget.

**13.5** (Single-node time-delay reservoir) Design an Appeltant-style time-delay reservoir (Subsection 13.4.2) with a nonlinear node of response time $T = 100$ ps.

(a) For $N = 400$ virtual nodes at node spacing $\theta = 0.2\,T = 20$ ps, compute the loop delay $\tau = N\theta$ and the physical fiber length ($n_g = 1.47$).

(b) Compute the input symbol rate $1/\tau$ and the virtual-node update rate $1/\theta$. For spoken-digit words of $\sim$100 symbols, estimate the classification throughput in words/s and compare with Larger et al. (2017).

(c) Explain the role of the ratio $\theta/T$: what goes wrong at $\theta \gg T$ and at $\theta \ll T$, and why $\theta \approx 0.2\,T$ is chosen.

(d) You must double the reservoir dimension to $N = 800$ while holding the 125 MS/s input rate. Show why this forces $\theta = 10$ ps and therefore a faster ($T = 50$ ps) node, and state the resulting trade between dimensionality and node bandwidth.

**13.6** (Hardware-in-the-loop training schedule) Physics-aware training (Wright et al., 2022): hardware forward pass, digital backward pass. Weight update settles in 50 μs; a hardware forward pass costs 1 μs/sample. Train on 60,000 examples for 20 epochs.

(a) For minibatch $B = 1$ (per-sample updates) versus $B = 100$, compute the number of weight updates and the total weight-settling time. Show how batching hides the update cost.

(b) For $B = 100$, compute total forward-pass time and total settling time; which dominates?

(c) To keep weight-update overhead below 20% of step wall-clock, derive the minimum batch size ($B\,t_{\text{fwd}} > 4\,t_{\text{update}}$).

(d) Compare this schedule with pure in-situ SPSA (Exercise 13.2b), which reprograms weights every gradient estimate: by roughly what factor does physics-aware training reduce weight-update count, and what does it pay for that reduction?

---

## Conceptual Exercises

**13.7** Explain, in terms of precision, weight-update bandwidth, and gradient-accumulation requirements (Subsections 13.1.3–13.1.4 and 13.3), why photonic accelerators are pursued for *inference* rather than *training*. Address why the same $O(N^2)$-optics/$O(N)$-conversion economics that favors inference works against training.

**13.8** An ONN trained offline at FP32 loses 9 percentage points of accuracy on deployment to a silicon photonic mesh with O-E-O activations. Rank the following causes by likely contribution and give one diagnostic measurement for each: (i) MZI coupler splitting-ratio error, (ii) thermal crosstalk between heaters, (iii) DAC phase quantization, (iv) photodetector shot noise, (v) laser wavelength drift, (vi) O-E-O activation transfer-function mismatch. Which are curable by noise-aware training and which require in-situ methods?

**13.9** Self-attention multiplies activations by activations, not activations by static weights. Explain (a) why this undermines the weight-stationary advantage that Chapter 12 processors rely on, (b) why the microsecond weight-update wall is the specific mechanism of failure, and (c) which photonic multiplier architecture (Subsection 13.5.2) sidesteps the problem and how.

---

## Lab / Computational Exercises

**13.10** (Noise-aware ONN training) In PyTorch, build a 2-layer MLP ($784 \to 64 \to 10$) whose linear layers are parameterized as Clements-mesh phases. Inject Gaussian phase noise $\sigma \in \{0.01, 0.02, 0.05\}$ rad per phase shifter on the forward pass. Train (i) naively (no noise) and (ii) noise-aware (noise sampled every forward pass), and evaluate both under deployment noise. Report the accuracy triplet (clean-train/clean-test, clean-train/noisy-test, noisy-train/noisy-test) at each $\sigma$, add per-output shot noise for 6-bit ENOB, and explain the ordering.

**13.11** (Time-delay reservoir for NARMA-10) Implement a single-node reservoir with a Mackey–Glass or $\tanh$ nonlinearity, an input mask, $N$ virtual nodes at spacing $\theta$, and feedback gain tuned for the echo-state property. Train a ridge readout on the NARMA-10 benchmark and report NRMSE. Sweep $N \in \{50, 100, 200, 400\}$ and the feedback gain (spectral-radius proxy); plot NRMSE versus $N$ and locate the edge-of-stability optimum.

**13.12** (Quantized photonic attention) Implement one scaled dot-product attention head ($L = 128$, $d_k = 64$) in NumPy/PyTorch. Quantize $Q, K, V$ to INT8 and INT6, add shot noise to each optically computed dot product at 7-bit ENOB, keep softmax in floating point (electronic), and measure the degradation of $\|AV\|$ and of a downstream toy-classification accuracy versus the FP32 baseline. Verify that quantizing the dynamic products $QK^{\top}$ and $AV$ costs more accuracy than quantizing the static projections at equal bit-width.
