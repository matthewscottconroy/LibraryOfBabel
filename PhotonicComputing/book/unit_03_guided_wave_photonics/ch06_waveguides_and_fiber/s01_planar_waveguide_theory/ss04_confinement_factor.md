# 6.1.4 Confinement Factor

## Definition and Physical Meaning

The confinement factor $\Gamma$ quantifies the fraction of the guided mode's optical power that resides in a specific region (typically the active or absorbing region). It determines the effective modal gain or loss:

$$g_{modal} = \Gamma \cdot g_{material}$$

$$\alpha_{modal} = \Gamma_{abs} \cdot \alpha_{material}$$

For a waveguide with a gain region occupying $x_1 \leq x \leq x_2$:

$$\Gamma = \frac{\int_{x_1}^{x_2} |E(x)|^2 n(x) dx}{\int_{-\infty}^{\infty} |E(x)|^2 n(x) dx}$$

(The $n(x)$ weighting accounts for the proper normalization of the Poynting vector in an inhomogeneous medium; for weakly-guiding waveguides, it is often approximated as unity.)

## Computing $\Gamma$ for TE Even Mode

For the symmetric slab waveguide (core $|x| < d/2$, total power $P_{total}$):

$$\Gamma = 1 - \frac{2}{\gamma d + 1 + b/(1-b)}$$

(after substituting the mode field and evaluating the integrals). For large $V$ (strong confinement): $\Gamma \to 1$ (mode mostly in core). For $V \to 0$ (near cutoff): $\Gamma \to 0$ (mode spreads into cladding).

For the fundamental TE mode of a silicon strip waveguide (450 × 220 nm), the full mode area is approximately $A_{eff} \approx 0.14$ μm², and $\Gamma \approx 0.8$ for the Si core region.

## Confinement Factor in Laser and Modulator Design

The confinement factor appears in two critical contexts:

**Laser threshold**: $g_{modal} = \alpha_i + \alpha_m$ → $\Gamma g_{material} = \alpha_{total}$. A higher $\Gamma$ lowers the threshold material gain required, reducing the threshold current. Quantum well lasers with $d \approx 8$–10 nm have small $\Gamma \approx 0.01$–0.05 per quantum well, which is why multiple quantum wells (MQW, 5–10 wells) are typically used to increase $\Gamma \cdot N_{QW}$.

**Electro-optic modulator efficiency**: In a silicon ring modulator, the phase shift $\Delta\phi = \Gamma_{Si} \Delta n_{Si} \cdot k_0 L$. Higher $\Gamma_{Si}$ gives more phase shift per unit length, reducing the required modulator length. Since $\Delta n_{Si}$ is set by the carrier density change (and therefore by the applied voltage and doping profile), $\Gamma_{Si}$ directly multiplies the modulation efficiency.

**Loss in bends**: When a waveguide bends, the mode shifts outward (centrifugal effect for the optical field), reducing $\Gamma$ and increasing radiative loss at the outer edge. This sets the minimum bend radius for a given waveguide geometry: for a 450 nm silicon strip, bend radii > 5 μm are typically safe; below 2 μm, loss increases rapidly.

## Mode Area and Nonlinearity

The effective mode area:

$$A_{eff} = \frac{\left(\int |E|^2 dA\right)^2}{\int |E|^4 dA}$$

is related to the nonlinear coefficient:

$$\gamma = \frac{n_2 \omega}{c A_{eff}}$$

For a 450 × 220 nm Si strip waveguide: $A_{eff} \approx 0.14$ μm², $n_2 = 6 \times 10^{-18}$ m²/W, $\omega = 2\pi c/\lambda$:

$$\gamma = \frac{6\times10^{-18} \times 2\pi \times 3\times10^8/1.55\times10^{-6}}{3\times10^8 \times 0.14\times10^{-12}} \approx 290 \text{ W}^{-1}\text{m}^{-1}$$

This is approximately $5000\times$ higher than standard single-mode fiber ($\gamma \approx 0.06$ W$^{-1}$m$^{-1}$). The high nonlinearity of silicon waveguides enables efficient nonlinear optical processes (FWM, SPM, SRS) at milliwatt power levels — useful for frequency conversion and comb generation, but also a source of unintended crosstalk in photonic computing circuits at high power.
