# 4.1.3 Rate Equations and Gain Saturation

## The Rate Equation Framework

The rate equations are the practical workhorse of laser design. They describe the time evolution of two coupled quantities: the population inversion $\Delta N = N_2 - N_1$ (or equivalently the carrier density $N$ in a semiconductor), and the photon number $S$ (or photon density $\phi$) in the cavity mode. They are mean-field equations — they describe averages over the ensemble of atoms and photons, ignoring quantum fluctuations (which are responsible for laser noise and are treated in Section 4.3.4).

For a four-level system, the canonical rate equations are:

$$\frac{dN}{dt} = \frac{J}{ed} - \frac{N}{\tau} - v_g g(N) S$$

$$\frac{dS}{dt} = \Gamma v_g g(N) S - \frac{S}{\tau_p} + \Gamma \beta \frac{N}{\tau_r}$$

where:
- $N$ = carrier density (cm$^{-3}$)
- $S$ = photon density (cm$^{-3}$)
- $J$ = injection current density (A/cm²)
- $e$ = electron charge
- $d$ = active layer thickness
- $\tau$ = total carrier lifetime ($1/\tau = 1/\tau_r + 1/\tau_{nr}$; radiative + non-radiative)
- $v_g = c/n_g$ = group velocity
- $g(N)$ = material gain coefficient (cm$^{-1}$)
- $\Gamma$ = confinement factor (fraction of mode in active region)
- $\tau_p$ = photon lifetime in cavity
- $\beta$ = spontaneous emission coupling factor

The three terms on the right-hand side of each equation have clear physical interpretations:

**Carrier equation**:
1. $J/(ed)$: pumping — carriers are injected at rate $J/e$ per unit area, per unit thickness $d$
2. $N/\tau$: spontaneous recombination — carriers recombine with lifetime $\tau$
3. $v_g g(N) S$: stimulated recombination — each photon in the cavity mode triggers emission with rate $v_g g(N)$

**Photon equation**:
1. $\Gamma v_g g(N) S$: stimulated emission gain — photons reproduce at rate $v_g g$, weighted by the mode confinement $\Gamma$
2. $-S/\tau_p$: cavity loss — photons leak out of the cavity (output coupling and internal loss) with photon lifetime $\tau_p$
3. $\Gamma\beta N/\tau_r$: spontaneous emission into mode — a small fraction $\beta$ of spontaneous emission couples to the lasing mode and seeds it

## The Material Gain Function

The gain coefficient $g(N)$ is the central material property. For semiconductor lasers, the most commonly used approximation is the logarithmic gain model:

$$g(N) = g_0 \ln\left(\frac{N}{N_0}\right)$$

which fits experimental data better than the linear model $g \approx a(N - N_0)$ at high carrier densities. Here $g_0$ is a gain parameter (typically 1500–3000 cm$^{-1}$) and $N_0$ is the transparency carrier density ($\approx 10^{18}$ cm$^{-3}$ for InGaAsP at 1550 nm).

The gain also depends on wavelength (or equivalently photon energy $\hbar\omega$), through the joint density of states and the Fermi-Dirac occupation factors. At a given carrier density $N$, the gain spectrum $g(N, \omega)$ peaks at a wavelength that blue-shifts as $N$ increases. This wavelength shift with carrier density — and hence with current — is one source of chirp in directly modulated semiconductor lasers.

## Steady-State Solution: Threshold

At steady state ($dN/dt = dS/dt = 0$), ignoring the small spontaneous emission term:

From the photon equation:
$$\Gamma v_g g(N_{th}) = \frac{1}{\tau_p}$$

$$g(N_{th}) = \frac{1}{\Gamma v_g \tau_p} \equiv g_{th}$$

The threshold gain is fixed by cavity parameters and confinement. Solving for $N_{th}$ from $g(N_{th}) = g_{th}$ gives the threshold carrier density.

From the carrier equation, the threshold current density is:

$$J_{th} = \frac{e d N_{th}}{\tau}$$

**Typical values for 1550 nm InGaAsP DFB laser**:
- $N_{th} \approx 1.5 \times 10^{18}$ cm$^{-3}$
- $d = 100$ nm (quantum well active region)
- $\tau \approx 2$ ns
- $J_{th} \approx 1.2 \times 10^3$ A/cm²
- For $50 \times 2$ μm² active area: $I_{th} = J_{th} \cdot A \approx 1.2$ mA

This sub-milliamp to low-milliamp threshold range is one of the great practical advantages of semiconductor lasers for photonic computing: they can be driven by CMOS logic levels without special high-power drive circuits.

## Above Threshold: Photon Density and Power

Above threshold, the carrier density clamps at $N_{th}$ (gain clamping): any additional carriers injected by increased current are immediately converted to photons by stimulated emission. The photon density grows linearly with current above threshold:

$$S = \frac{\tau_p}{ed} (J - J_{th}) \cdot \frac{\Gamma}{1} \approx \frac{\Gamma \tau_p}{ed} (J - J_{th})$$

The optical output power (from one facet) is:

$$P_{out} = \frac{\hbar\omega}{e} \cdot \frac{\alpha_m}{\alpha_m + \alpha_i} \cdot \frac{1}{2}(I - I_{th}) \cdot \eta_i$$

where $\alpha_m = (1/2L)\ln(1/R_1 R_2)$ is the mirror loss, $\alpha_i$ is the internal (distributed) loss, $\eta_i$ is the internal quantum efficiency (fraction of injected carriers that recombine radiatively), and the factor of 1/2 accounts for equal output from two facets.

This can be written as:

$$P_{out} = \eta_d \cdot \frac{\hbar\omega}{e} \cdot (I - I_{th})$$

where $\eta_d$ is the *differential (external) quantum efficiency* — the slope of the $P$ vs. $I$ (LI) curve above threshold. For a good 1550 nm DFB laser, $\eta_d \approx 0.1$ mW/mA to $0.5$ mW/mA, meaning 100 μA of additional drive current produces 10–50 μW of additional optical power.

## Gain Saturation

The linear gain model $g(N)$ holds at low photon densities. At high photon densities, the gain is compressed because the intense stimulated emission depletes the carriers faster than they can be replenished by the pump. This is modeled by the gain saturation coefficient $\varepsilon$:

$$g(N, S) = \frac{g(N)}{1 + \varepsilon S}$$

where $\varepsilon \approx 10^{-17}$ cm³ is the gain compression factor. This saturated gain model is critical for analyzing:

1. **High-speed modulation**: At high modulation frequencies (>10 GHz), the photon density swings widely and gain saturation limits the modulation depth. The −3 dB bandwidth of direct modulation is:

$$f_{-3\text{dB}} \approx \frac{1}{2\pi}\sqrt{\frac{2}{\tau_p \tau_s}}$$

where $\tau_s = 1/(\partial g/\partial N \cdot v_g \cdot S)$ is related to the differential gain. For modern DFB lasers, $f_{-3\text{dB}} \approx 20$–30 GHz, limited by the photon and carrier lifetimes.

2. **Relative intensity noise (RIN)**: Gain saturation provides a stabilizing feedback that reduces RIN. High-speed photonic computing requires low RIN to maintain the analog precision of the optical signals.

3. **Spectral hole burning**: At extremely high intensities, stimulated emission depletes carriers at a specific carrier momentum (energy), creating a spectral "hole" in the gain curve. This causes gain compression and limits the maximum coherent output power of single-mode lasers.

## The Confinement Factor $\Gamma$

The confinement factor $\Gamma$ appears throughout the rate equations because only a fraction of the optical mode overlaps with the active (gain) region. For a ridge waveguide laser with a 100 nm quantum well:

$$\Gamma = \frac{\int_{\text{active}} |E(x,y)|^2 \, dx\,dy}{\int_{\text{total}} |E(x,y)|^2 \, dx\,dy}$$

Typical values: $\Gamma \approx 0.03$–0.1 for a single quantum well, and up to 0.3–0.5 for bulk or multi-quantum-well active regions.

**Trade-off**: Higher $\Gamma$ increases the modal gain $\Gamma g$, lowering threshold. But a more confined mode also has stronger coupling to waveguide roughness, increasing the internal loss $\alpha_i$. Optimizing $\Gamma$ is one of the central design choices in laser waveguide engineering.

For photonic computing, the confinement factor also determines how efficiently the on-chip laser mode couples to the signal waveguide — a poor confinement match (mode mismatch) introduces insertion loss at every laser-to-waveguide interface.
