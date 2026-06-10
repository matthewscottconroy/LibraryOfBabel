# Timescale Hierarchy in Deep Reservoirs

## Effective Time Constant of a Leaky Integrator Layer

A single-layer leaky integrator ESN with leak rate $\alpha$ and spectral radius $\rho$ has the update equation

$$\mathbf{x}_t = (1 - \alpha)\mathbf{x}_{t-1} + \alpha \tanh(\mathbf{W}^{\text{rec}} \mathbf{x}_{t-1} + \mathbf{W}^{\text{in}} \mathbf{u}_t).$$

Linearizing around a fixed point and treating the tanh as approximately linear (valid for small signals), each neuron $i$ satisfies approximately

$$x_t^{(i)} \approx (1 - \alpha + \alpha \rho_i) x_{t-1}^{(i)} + \alpha [\mathbf{W}^{\text{in}} \mathbf{u}]_i,$$

where $\rho_i$ is the contribution of the $i$-th mode. The effective decay rate per step is $1 - \alpha(1 - \rho_i)$, so the effective time constant (in time steps) is

$$\tau_{\text{eff}} \approx \frac{1}{\alpha(1 - \rho)},$$

where $\rho$ is taken as the spectral radius. This formula shows that smaller $\alpha$ (slower forgetting) or larger $\rho$ (closer to marginal stability) both increase the effective memory window [Gallicchio et al. 2018].

## Geometric Timescale Hierarchy

In a DeepESN with $L$ layers, set the leak rates as

$$\alpha_\ell = \frac{\alpha_1}{\ell}, \quad \ell = 1, \ldots, L,$$

with spectral radii approximately equal across layers. Then the effective time constants scale as

$$\tau_{\text{eff}}^{(\ell)} = \frac{1}{\alpha_\ell(1-\rho)} = \frac{\ell}{\alpha_1(1-\rho)} \approx \ell \cdot \tau_{\text{eff}}^{(1)}.$$

This creates an arithmetic (linear) hierarchy of timescales. An exponential hierarchy can be achieved with $\alpha_\ell = \alpha_1 \beta^{-(\ell-1)}$ for $\beta > 1$:

$$\tau_{\text{eff}}^{(\ell)} \approx \beta^{\ell - 1} \cdot \tau_{\text{eff}}^{(1)},$$

which is more aggressive but risks making upper layers so slow that they fail to respond on training-sequence timescales.

## Transfer Function Analysis

The z-transform provides the precise characterization of each layer's frequency response. For a scalar leaky integrator with leak rate $\alpha$ and no recurrent coupling ($\mathbf{W}^{\text{rec}} = \mathbf{0}$), the z-domain transfer function from input to state is:

$$H(z) = \frac{\alpha}{z - (1 - \alpha)}.$$

This has a single pole at $z = 1 - \alpha$ and a zero at $z = 0$. The magnitude response is

$$|H(e^{j\omega})| = \frac{\alpha}{|e^{j\omega} - (1 - \alpha)|} = \frac{\alpha}{\sqrt{(1 - (1-\alpha)\cos\omega)^2 + (1-\alpha)^2 \sin^2\omega}}.$$

For small $\omega$ (low frequencies), $|H(e^{j\omega})| \approx 1$; for $\omega$ near $\pi$ (high frequencies), $|H(e^{j\pi})| = \alpha / (2 - \alpha) \ll 1$ for small $\alpha$. The $-3\,\text{dB}$ cutoff frequency is approximately $\omega_c \approx \alpha$ for small $\alpha$ [Oppenheim & Schafer 1999].

When the recurrent matrix is included with spectral radius $\rho < 1$, the effective pole moves to $z \approx 1 - \alpha(1-\rho)$, and the cutoff frequency becomes

$$\omega_c \approx \alpha(1-\rho).$$

Layer $\ell$ thus acts as a first-order IIR low-pass filter with cutoff frequency $\omega_c^{(\ell)} = \alpha_\ell(1 - \rho_\ell)$.

## Deep ESN as a Cascade of Low-Pass Filters

Each layer receives the output of the layer below and applies another low-pass filter. The cascade of $L$ layers produces an overall transfer function (for the linearized system):

$$H_{\text{total}}(z) = \prod_{\ell=1}^L H_\ell(z),$$

where each $H_\ell$ has cutoff $\omega_c^{(\ell)}$. The cascade of low-pass filters with decreasing cutoffs means that upper layers see progressively more heavily filtered, low-frequency versions of the input. Layer $L$ effectively sees only the slow trends in $\mathbf{u}_t$, while layer 1 retains rapid fluctuations.

The power spectrum of the layer-$\ell$ state for white-noise input $\mathbf{u}_t$ is proportional to $|H_\ell(e^{j\omega})|^2 \cdot |H_{\ell-1}(e^{j\omega})|^2 \cdots |H_1(e^{j\omega})|^2$, which is a product of low-pass functions with successively lower cutoffs. This predicts that the power spectrum of each successive layer should be concentrated at progressively lower frequencies — a prediction that can be empirically verified by computing layer-by-layer power spectra [Gallicchio et al. 2018].

## Empirical Validation

Gallicchio et al. [2018] confirmed the timescale hierarchy empirically. Driving a 5-layer DeepESN with broadband noise and computing the power spectrum of each layer's states, they found that (1) layer 1 had a nearly flat spectrum (reflecting the input), (2) each successive layer had a spectrum shifted toward lower frequencies, and (3) the -3 dB cutoff of layer $\ell$ was approximately $\alpha_\ell(1-\rho_\ell)$ in agreement with the theory. The correspondence between predicted and observed cutoff frequencies validates the cascade low-pass filter model.

This empirical validation is important because it means the timescale hierarchy is not merely a design aspiration but a measurable property of the trained deep reservoir, arising automatically from the choice of layer-wise $\alpha_\ell$ values [Gallicchio et al. 2018].

---

## References

- Gallicchio, C., Micheli, A., & Pedrelli, L. (2018). Design of deep echo state networks. *Neural Networks*, 108, 33–47.
- Oppenheim, A. V., & Schafer, R. W. (1999). *Discrete-Time Signal Processing* (2nd ed.). Prentice Hall.
