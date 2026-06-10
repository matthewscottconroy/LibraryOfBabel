# 13.2.1 Timescale Hierarchy: The Mathematics of Slow and Fast Layers

## The Central Observation

One of the most striking properties of deep ESNs is that different layers naturally develop different temporal dynamics, even when initialized with identical hyperparameters. Lower layers — those receiving direct input drive — respond quickly to changes in the input signal. Upper layers — driven by the already-smoothed states of the layers below — integrate information over longer horizons. This is not a qualitative hand-waving observation but a quantitative, provable consequence of the architecture.

The result is a hierarchy of timescales that is architecturally emergent: you do not need to explicitly engineer slow neurons at upper layers. However, you can control and amplify this hierarchy by choosing the leaking rates $\alpha_1 \geq \alpha_2 \geq \cdots \geq \alpha_L$ to be decreasing with layer depth.

## Effective Memory Time Constant: Single Layer

Before analyzing the deep case, let us establish the time constant for a single leaky integrator. Consider a scalar neuron with no recurrent connections (for analytical clarity):

$$x_t = (1-\alpha) x_{t-1} + \alpha u_t$$

This is a first-order IIR filter. Its impulse response — the response to $u_0 = 1$, $u_t = 0$ for $t > 0$ — is:

$$x_t = (1-\alpha)^t \cdot x_0 + \alpha \sum_{k=0}^{t-1}(1-\alpha)^k u_{t-1-k}$$

For the impulse input, $x_t = \alpha (1-\alpha)^{t-1}$ for $t \geq 1$. The state decays geometrically with ratio $(1-\alpha)$.

The **effective memory time constant** $\tau$ is defined as the time for the impulse response to decay to $e^{-1}$ of its initial value:

$$(1-\alpha)^\tau = e^{-1} \implies \tau = \frac{-1}{\ln(1-\alpha)} \approx \frac{1}{\alpha}$$

where the approximation holds for small $\alpha \ll 1$ (using $\ln(1-\alpha) \approx -\alpha$). For $\alpha = 0.1$, $\tau \approx 10$ time steps. For $\alpha = 0.01$, $\tau \approx 100$ time steps.

## Timescale Propagation in Deep Architectures

Now consider the deep stack. Layer 1 has leaking rate $\alpha_1$ and receives input $\mathbf{u}_t$. Its state (for the scalar, no-recurrence case) is:

$$x_t^{(1)} = \sum_{k=0}^{\infty} \alpha_1(1-\alpha_1)^k u_{t-k}$$

This is a convolution of $u_t$ with the exponential kernel $\alpha_1(1-\alpha_1)^k$. The power spectrum of $x_t^{(1)}$ is:

$$S_{x^{(1)}}(\omega) = |H_1(e^{i\omega})|^2 S_u(\omega), \quad H_1(z) = \frac{\alpha_1 z}{z - (1-\alpha_1)}$$

Layer 2 receives $x_t^{(1)}$ as its input. Its state is a filtered version of layer 1's state:

$$x_t^{(2)} = \sum_{k=0}^{\infty} \alpha_2(1-\alpha_2)^k x_{t-k}^{(1)}$$

The effective impulse response seen at layer 2 — the response at layer 2 to an impulse at the input — is the convolution of the two exponential filters:

$$h^{(2)}(t) = (\alpha_1(1-\alpha_1)^t) * (\alpha_2(1-\alpha_2)^t)$$

The convolution of two decaying exponentials is the binomial mixture:

$$h^{(2)}(t) = \frac{\alpha_1 \alpha_2}{(1-\alpha_2) - (1-\alpha_1)}\left[(1-\alpha_1)^t - (1-\alpha_2)^t\right], \quad \alpha_1 \neq \alpha_2$$

When $\alpha_1 = \alpha_2 = \alpha$ (equal leaking rates), the convolution is:

$$h^{(2)}(t) = \alpha^2 t (1-\alpha)^{t-1}$$

which is a Gamma distribution shape — it peaks at $t = 1/\alpha - 1$ and decays thereafter, giving a longer effective memory than a single layer. More precisely, the **effective time constant at layer $\ell$** (for equal leaking rates) is:

$$\tau^{(\ell)} \approx \frac{\ell}{\alpha}$$

**Proof for equal leaking rates.** The impulse response at layer $\ell$ with equal leaking rates $\alpha$ is proportional to a negative binomial (Pascal) distribution:

$$h^{(\ell)}(t) \propto \binom{t + \ell - 2}{\ell - 1} (1-\alpha)^{t-1} \alpha^\ell$$

The mean of this distribution — and hence the center of mass of the memory kernel — is $\ell(1-\alpha)/\alpha \approx \ell/\alpha$ for small $\alpha$. This is precisely $\ell \cdot \tau^{(1)}$: the time constant grows linearly with depth. $\square$

**For decreasing leaking rates** $\alpha_1 > \alpha_2 > \cdots > \alpha_L$, the separation between timescales is amplified. The effective time constant at layer $\ell$ is approximately $1/\alpha_\ell$, and by choosing $\alpha_\ell$ to decrease geometrically (e.g., $\alpha_\ell = \alpha_1 \cdot r^{\ell-1}$ for $r < 1$), one achieves exponentially separated timescales: $\tau^{(\ell)} \propto r^{-(\ell-1)}/\alpha_1$.

## The Transfer Function Perspective

The frequency-domain analysis makes the timescale hierarchy vivid. The transfer function of the $\ell$-th layer filter (in the linear, no-recurrence approximation) is:

$$H_\ell(e^{i\omega}) = \frac{\alpha_\ell}{1 - (1-\alpha_\ell)e^{-i\omega}}$$

This is a low-pass filter with cutoff frequency:

$$\omega_c^{(\ell)} = \arccos\!\left(\frac{1 + (1-\alpha_\ell)^2}{2(1-\alpha_\ell)}\right) \approx \alpha_\ell \quad \text{(for small } \alpha_\ell\text{)}$$

The composite transfer function from input to layer $\ell$ is the product of all layer transfer functions:

$$H^{(\ell)}(e^{i\omega}) = \prod_{k=1}^{\ell} H_k(e^{i\omega})$$

This is a cascade of low-pass filters. The resulting composite filter has a cutoff frequency approximately equal to $\min_k \omega_c^{(k)} = \omega_c^{(\ell)}$ for decreasing $\alpha$. The key conclusion: **higher layers see only the low-frequency content of the input**. The slow components of the input signal propagate all the way to the top of the stack; the fast, high-frequency components are progressively filtered out.

## Recurrent Dynamics and the Timescale Hierarchy

The analysis above treated each layer as a pure integrator (no recurrent connections). In the full deep ESN, the recurrent matrix $W_\ell^{rec}$ adds internal dynamics at each layer. The recurrent dynamics interact with the integrative dynamics to produce a richer spectral structure.

For a layer with spectral radius $\rho_\ell$ and leaking rate $\alpha_\ell$, the effective time constant of the dominant recurrent mode is approximately:

$$\tau_\ell^{eff} = \frac{-1}{\ln\left|(1-\alpha_\ell) + \alpha_\ell \rho_\ell\right|}$$

When $\rho_\ell < 1$, this is finite. As $\rho_\ell \to 1^-$, the effective time constant diverges — the reservoir approaches marginal stability, with very slow modes. The deep architecture allows different layers to operate at different points on this stability spectrum, with lower layers (high $\alpha_\ell$, lower $\rho_\ell$) having shorter time constants and upper layers (low $\alpha_\ell$, higher $\rho_\ell$) having longer time constants.

This is the **edge-of-stability principle for deep ESNs**: place lower layers comfortably inside the stable regime (short time constants, fast response) and push upper layers closer to the edge (longer time constants, richer dynamics).

## Empirical Validation

Gallicchio et al. [Gallicchio2017b] validate this analysis empirically on sequential MNIST and several speech tasks. They compute the **intrinsic plasticity** and **effective state entropy** at each layer and show:

1. Lower layers have higher state entropy — they respond to fine-grained input variation.
2. Upper layers have lower state entropy — they encode coarse, temporally integrated features.
3. The linear dependence of effective time constant on layer depth (for equal leaking rates) holds quantitatively.

The gap between theoretical prediction and measurement is attributable to the recurrent dynamics, which the pure-integrator analysis ignores.

---

## References

- [Gallicchio2017a] Gallicchio, C. & Micheli, A. (2017). Echo state property of deep reservoir computing networks. *Cognitive Computation*, 9(3), 337–350.
- [Gallicchio2017b] Gallicchio, C., Micheli, A., & Pedrelli, L. (2017). Deep reservoir computing: A critical experimental analysis. *Neurocomputing*, 268, 87–99.
- [Jaeger2002b] Jaeger, H. (2002). Adaptive nonlinear system identification with echo state networks. *Advances in Neural Information Processing Systems*, 15.
- [Lukoševičius2012] Lukoševičius, M. (2012). A practical guide to applying echo state networks. In *Neural Networks: Tricks of the Trade*. Springer.
