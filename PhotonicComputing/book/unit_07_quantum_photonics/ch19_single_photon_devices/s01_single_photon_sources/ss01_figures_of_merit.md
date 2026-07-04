# 19.1.1 Figures of Merit: Brightness, Purity, Indistinguishability

## The Three Requirements

A single-photon source is characterized by three numbers. Each answers a different question, each is measured by a different experiment, and — this is the difficulty — improving any one of them tends to degrade the others.

### Brightness (Efficiency) $\eta$

**Question:** When I trigger the source, what is the probability that exactly one photon arrives in the mode where I need it?

Brightness must be dissected into a chain of conditional efficiencies, because the literature quotes them inconsistently:

$$\eta_{\text{end-to-end}} = \eta_{\text{prep}} \cdot \eta_{\text{emit}} \cdot \eta_{\text{extract}} \cdot \eta_{\text{couple}}$$

- $\eta_{\text{prep}}$: probability the trigger actually prepares the excited state (for a quantum dot under resonant $\pi$-pulse excitation, $\eta_{\text{prep}} > 0.98$).
- $\eta_{\text{emit}}$: probability the excitation decays by emitting a photon into the intended radiation channel rather than a phonon or a nonradiative pathway.
- $\eta_{\text{extract}}$: probability the photon escapes the high-index host material. Unassisted, this is catastrophic — for GaAs ($n = 3.5$), total internal reflection traps all photons outside a cone of half-angle $\theta_c = \arcsin(1/n) \approx 17°$, giving $\eta_{\text{extract}} \approx 1/(4n^2) \approx 2\%$. Photonic engineering (Section 19.3) exists largely to fix this number.
- $\eta_{\text{couple}}$: probability the extracted photon enters the single spatial mode (fiber or waveguide) that feeds the downstream circuit.

The best demonstrated end-to-end value for an on-demand source is $\eta \approx 0.57$ into a single-mode fiber, from a gated quantum dot in an open microcavity (Tomm et al., 2021). For context: proposals for fault-tolerant photonic computing assume per-photon delivery efficiencies well above 0.9, so even the record leaves a gap that architecture (multiplexing, loss-tolerant encoding) must close.

### Single-Photon Purity: $g^{(2)}(0)$

**Question:** When the source fires, how often does it emit *two* photons instead of one?

Purity is quantified by the second-order correlation function at zero delay, measured in a Hanbury Brown–Twiss (HBT) setup (Chapter 18). For a pulsed source with probability $p_1$ of emitting one photon and $p_2 \ll p_1$ of emitting two,

$$g^{(2)}(0) \approx \frac{2p_2}{(p_1 + 2p_2)^2} \approx \frac{2p_2}{p_1^2}$$

An attenuated laser has $g^{(2)}(0) = 1$ no matter how weak — attenuation reduces $p_1$ and $p_2$ together, never their ratio. That is why "faint laser pulses" are not single photons. The conventional thresholds:

- $g^{(2)}(0) < 0.5$: certifies a dominant single-photon component (the "quantum" threshold).
- $g^{(2)}(0) < 0.01$: the requirement usually quoted for scalable photonic quantum computing, since two-photon events masquerade as logical errors that error correction must then absorb.

State of the art: quantum dots under resonant excitation have reached $g^{(2)}(0) = 7.5 \times 10^{-5}$ (Schweickert et al., 2018) — purity is the solved problem of the three.

### Indistinguishability $M$

**Question:** Are consecutive photons quantum-mechanically identical?

Two photons interfere via the Hong-Ou-Mandel effect only to the extent that their wavepackets overlap in every degree of freedom: spectrum, polarization, spatial mode, arrival time. Indistinguishability is defined as the overlap of the single-photon wavefunctions,

$$M = |\langle \psi_1 | \psi_2 \rangle|^2,$$

and is measured as the visibility of the HOM dip between two photons emitted successively by the same source (or by two different sources, a far harder test). Dephasing mechanisms — spectral diffusion from charge noise, phonon scattering, timing jitter of the emission — reduce $M$ below 1. For a two-level emitter with radiative lifetime $T_1$ and total coherence time $T_2$,

$$M = \frac{T_2}{2T_1},$$

so the Fourier-transform limit $T_2 = 2T_1$ gives $M = 1$. This single relation drives much of source engineering: either slow the dephasing (cryogenics, charge stabilization) or speed up the emission (Purcell enhancement, Section 19.3.3) so the photon is launched before dephasing occurs.

State of the art: HOM visibilities of 98–99.5% for photons emitted nanoseconds apart from one quantum dot (Somaschi et al., 2016; Ding et al., 2016); ~93% raw visibility between two *remote* quantum dots.

## Why All Three at Once Is Hard

The three metrics are coupled through the physics of real emitters:

1. **Purity vs. brightness (SPDC):** a down-conversion source's pair-generation probability $\mu$ sets both its brightness ($\propto \mu$) and its heralded impurity ($g^{(2)}_h(0) \approx 2\mu$, Section 19.1.4). Demanding $g^{(2)}(0) < 0.01$ caps the per-pulse brightness at $\mu \lesssim 0.005$.
2. **Indistinguishability vs. brightness (solid-state emitters):** spectral filtering can carve a Fourier-limited line out of a broad emission spectrum, boosting $M$ — but every filtered photon is a lost photon. The NV center is the extreme case: only ~3% of its emission is in the usable zero-phonon line.
3. **Brightness vs. purity (excitation leakage):** pumping harder to guarantee excitation increases the chance of re-excitation within one pulse, producing two photons and spoiling $g^{(2)}(0)$.

A useful summary metric for multiphoton experiments is the $n$-photon coincidence rate. If the repetition rate is $R$ and the per-photon system efficiency is $\eta$, the $n$-fold rate is

$$R_n = R\,\eta^n.$$

**Worked example.** A quantum dot source is pumped at $R = 76$ MHz. With $\eta = 0.30$ (a realistic full-system number including detection), the 20-photon rate is $76\,\text{MHz} \times 0.3^{20} \approx 2.6 \times 10^{-3}$ Hz — about 9 events per hour, roughly the regime of the 2019 20-photon boson sampling experiments. Raising $\eta$ to 0.6 raises the rate by $2^{20} \approx 10^6$, to ~2.8 kHz. Nothing else in the system rewards improvement as steeply as per-photon efficiency.

## Representative Numbers by Platform

| Platform | On-demand? | Best $g^{(2)}(0)$ | Best $M$ | End-to-end $\eta$ | Wavelength |
|---|---|---|---|---|---|
| InGaAs quantum dot (cavity) | Yes | $<10^{-3}$ | 0.985–0.995 | ~0.5–0.6 (fiber) | 900–950 nm (1550 via InAs/InP or conversion) |
| NV center in diamond | Yes | <0.1 (RT) | low (ZPL ~3%) | ~$10^{-3}$–$10^{-2}$ | 637 nm ZPL |
| SiV/SnV in diamond | Yes | <0.05 | ~0.7–0.9 (cavity) | ~$10^{-2}$ | 737/619 nm |
| hBN defect (2D) | Yes (RT) | 0.05–0.3 | low | ~$10^{-2}$ | 570–800 nm |
| Heralded SPDC/SFWM | Heralded | $\approx 2\mu$ (tunable) | >0.99 (engineered) | 0.5–0.9 heralding | any phase-matched, incl. 1550 nm |

The next three subsections examine how each platform earns — and pays for — its entries in this table.
