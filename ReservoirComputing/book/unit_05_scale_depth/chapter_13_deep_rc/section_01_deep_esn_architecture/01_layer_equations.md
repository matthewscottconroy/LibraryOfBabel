# 13.1.1 The Deep ESN Architecture: Layer Equations and Echo State Property

## From One Layer to Many

A single-layer ESN with $N$ neurons updates its state according to:

$$\mathbf{x}_t = (1 - \alpha)\mathbf{x}_{t-1} + \alpha \tanh\!\left(W^{rec}\mathbf{x}_{t-1} + W^{in}\mathbf{u}_t\right)$$

where $\alpha \in (0,1]$ is the leaking rate. In a deep ESN with $L$ layers, each layer $\ell \in \{1, 2, \ldots, L\}$ has its own state, recurrent weight matrix, and inter-layer weight matrix. The governing equations are:

$$\mathbf{x}_t^{(\ell)} = (1 - \alpha_\ell)\mathbf{x}_{t-1}^{(\ell)} + \alpha_\ell \tanh\!\left(W_\ell^{rec}\mathbf{x}_{t-1}^{(\ell)} + W_\ell^{in}\mathbf{x}_t^{(\ell-1)}\right), \quad \ell = 1, 2, \ldots, L$$

with the convention that $\mathbf{x}_t^{(0)} = \mathbf{u}_t$ is the external input. Each layer $\ell$ has:

- **State dimension** $N_\ell$ (may differ across layers)
- **Leaking rate** $\alpha_\ell \in (0, 1]$ (may differ across layers — this is the key design parameter for timescale control)
- **Recurrent matrix** $W_\ell^{rec} \in \mathbb{R}^{N_\ell \times N_\ell}$ (sparse, randomly initialized, fixed)
- **Inter-layer matrix** $W_\ell^{in} \in \mathbb{R}^{N_\ell \times N_{\ell-1}}$ (randomly initialized, fixed)

The readout is a linear function of the concatenated states of all layers (or a subset of layers):

$$\mathbf{y}_t = W^{out}\left[\mathbf{x}_t^{(1)\top}, \mathbf{x}_t^{(2)\top}, \ldots, \mathbf{x}_t^{(L)\top}\right]^\top$$

This concatenation is essential: the readout sees the full hierarchy, from the fast, input-driven lower-layer states to the slow, long-memory upper-layer states.

## Architectural Design Choices

**Uniform vs. tapered width.** Common choices are uniform width ($N_1 = N_2 = \cdots = N_L$) or linearly increasing width (lower layers narrower, upper layers wider). The empirical literature [Gallicchio2017b] suggests that for most tasks, uniform width is competitive and more interpretable.

**Skip connections.** One may add direct connections from the input $\mathbf{u}_t$ to every layer, not just layer 1. This prevents the vanishing of input signal in deep stacks and is analogous to residual connections in deep feedforward networks. The layer equation with skip connections becomes:

$$\mathbf{x}_t^{(\ell)} = (1 - \alpha_\ell)\mathbf{x}_{t-1}^{(\ell)} + \alpha_\ell \tanh\!\left(W_\ell^{rec}\mathbf{x}_{t-1}^{(\ell)} + W_\ell^{in}\mathbf{x}_t^{(\ell-1)} + W_\ell^{skip}\mathbf{u}_t\right)$$

**Readout strategy.** An alternative to reading from all layers simultaneously is to train a separate readout per layer and combine them (ensemble-like), or to read only from the final layer. Gallicchio and Micheli [Gallicchio2017a] show that reading from all layers generally outperforms reading from any single layer.

## The Echo State Property in Deep Networks

The echo state property (ESP) requires, informally, that the reservoir state be uniquely determined by the input history, independent of initial conditions. For a single layer, a sufficient condition is that the spectral radius $\rho(W^{rec}) < 1$ in the case of saturated neurons, or that the largest singular value satisfies $\sigma_{\max}(W^{rec}) \cdot \alpha < 1$ for leaky neurons (see Chapter 5).

For deep networks, the ESP must hold at every layer. The key insight is that layer $\ell$ is driven not by the external input $\mathbf{u}_t$ but by the state of the layer below, $\mathbf{x}_t^{(\ell-1)}$. If each lower layer satisfies the ESP, then its state $\mathbf{x}_t^{(\ell-1)}$ is a well-defined functional of the input history, and the upper layer inherits a valid (if transformed) input signal.

**Theorem (Sufficient Condition for Deep ESP).** Suppose that for each layer $\ell$:

1. The spectral radius satisfies $\rho(W_\ell^{rec}) < \frac{1}{\alpha_\ell}(1 - (1-\alpha_\ell)) = 1$. More precisely, with leaky integration, the sufficient condition is that the matrix $(1-\alpha_\ell)I + \alpha_\ell W_\ell^{rec}$ has spectral radius strictly less than 1 when the neurons are in their linear regime.
2. The tanh nonlinearity ensures all layer states are bounded.

Then the deep ESN has the echo state property: for any bounded input sequence $\{\mathbf{u}_t\}$, the state trajectory $\{\mathbf{x}_t^{(1)}, \ldots, \mathbf{x}_t^{(L)}\}$ is uniquely determined by the input history, independent of initial conditions.

**Proof sketch.** We proceed by induction on the layer index. For $\ell = 1$: the ESP for layer 1 follows from the standard single-layer argument. The inter-layer input to layer 1 is $\mathbf{x}_t^{(0)} = \mathbf{u}_t$, which is bounded by assumption. The contraction condition $(1-\alpha_1) + \alpha_1 \rho(W_1^{rec}) < 1$ ensures that the driven system at layer 1 is a contraction in a suitable norm (the existence of such a Lyapunov function follows from the Banach fixed-point theorem applied on the space of state sequences).

For $\ell > 1$: by the inductive hypothesis, $\mathbf{x}_t^{(\ell-1)}$ is a unique functional of $\{\mathbf{u}_s : s \leq t\}$, and it is bounded (since tanh outputs lie in $(-1, 1)^{N_{\ell-1}}$). The layer-$\ell$ update is therefore driven by a bounded, uniquely-determined input signal. The same contraction argument applies, yielding the ESP for layer $\ell$. By induction, the ESP holds for all layers. $\square$

**Remark.** The condition is sufficient but not necessary. In practice, reservoirs often operate near the edge of stability (spectral radius close to 1), and the nonlinearity provides stabilization that the linear analysis misses. The sufficient condition should be understood as a design guideline, not a sharp characterization.

## Practical Initialization

The standard initialization procedure for a deep ESN is:

1. For each layer $\ell$, generate a sparse random matrix with desired connectivity $p_\ell$ (fraction of non-zero entries). Typical values: $p \approx 0.1$ for $N = 100$, $p \approx 0.01$ for $N = 1000$.
2. Compute the spectral radius $\rho_\ell = \rho(W_\ell^{rec})$ via the power iteration or full eigendecomposition.
3. Rescale: $W_\ell^{rec} \leftarrow (\rho_{target} / \rho_\ell) W_\ell^{rec}$, where $\rho_{target} < 1$ is the desired spectral radius.
4. For $W_\ell^{in}$, use a dense or sparse random matrix with entries drawn from $\text{Uniform}(-s, s)$ for input scale $s > 0$.
5. Set leaking rates $\alpha_1 \geq \alpha_2 \geq \cdots \geq \alpha_L$ to impose a timescale hierarchy (see Section 13.2).

This initialization ensures the sufficient condition for the deep ESP is satisfied. Tuning $\rho_{target}$ and $\alpha_\ell$ is the primary empirical knob for controlling the memory-computation tradeoff at each layer.

---

## References

- [Gallicchio2017a] Gallicchio, C. & Micheli, A. (2017). Echo state property of deep reservoir computing networks. *Cognitive Computation*, 9(3), 337–350.
- [Gallicchio2017b] Gallicchio, C., Micheli, A., & Pedrelli, L. (2017). Deep reservoir computing: A critical experimental analysis. *Neurocomputing*, 268, 87–99.
- [Jaeger2001] Jaeger, H. (2001). The echo state approach to analysing and training recurrent neural networks. GMD Technical Report 148.
- [Lukoševičius2012] Lukoševičius, M. (2012). A practical guide to applying echo state networks. In *Neural Networks: Tricks of the Trade*. Springer. 659–686.
