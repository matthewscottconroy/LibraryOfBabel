# 23.2.3 Process Variation and Yield Models

Two different things go wrong on a wafer, and they demand different mathematics. **Defects** — particles, voids, shorts — kill devices outright; they are counted with defect-density yield models. **Parametric variation** — every device works, but with dimensions slightly off — is the deeper problem for photonics, because analog optical circuits respond continuously to geometry. This subsection treats both, and turns them into the design budgets that photonic computing systems live by.

## The Anatomy of Variation

Geometric variation decomposes by spatial scale:

- **Within-die** (μm–mm): local lithography and etch-loading effects; smooth gradients plus device-to-device residuals. Neighboring devices are strongly *correlated* — the basis of matched design.
- **Die-to-die / within-wafer** (mm–cm): radial etch and deposition non-uniformity; SOI thickness maps. Often the largest term.
- **Wafer-to-wafer and lot-to-lot**: drift in tool calibration between runs.

Representative magnitudes for a mature 193 nm silicon photonics line: waveguide-width control of a few nanometers (Selvaraja et al. demonstrated ~2 nm σ linewidth uniformity with 193 nm lithography [IEEE JSTQE, 2010]), SOI thickness variation of ±1–5 nm across a wafer, partial-etch depth control of ±few nm. E-beam prototyping trades better resolution for similar or worse *uniformity* plus stitching offsets.

## From Nanometers to Optical Error: Sensitivity Analysis

The bridge from geometry to optics is the effective index. For the fundamental TE mode of a 500 × 220 nm strip waveguide near 1550 nm, mode solvers give sensitivities of approximately

$$\frac{\partial n_{eff}}{\partial w} \approx 1.5 \times 10^{-3}\ \text{nm}^{-1}, \qquad \frac{\partial n_{eff}}{\partial t} \approx 5 \times 10^{-3}\ \text{nm}^{-1}$$

— thickness is roughly 3× more sensitive than width, and both fall steeply for wider waveguides (a 3 μm-wide multimode strip is ~50× less width-sensitive, which is why long routing and delay lines are drawn wide and tapered down only where single-mode behavior is needed).

**Worked example — linewidth to resonance shift.** A ring resonator's resonance sits at $m\lambda_{res} = 2\pi R\, n_{eff}$, so a perturbation $\delta n_{eff}$ shifts it (accounting for dispersion via the group index $n_g$) by

$$\delta\lambda_{res} = \lambda \frac{\delta n_{eff}}{n_g}$$

Take $\lambda = 1550$ nm, $n_g = 4.2$, and a width error $\delta w = +5$ nm: $\delta n_{eff} = 7.5\times10^{-3}$ and

$$\delta\lambda_{res} = 1550\ \text{nm} \times \frac{7.5\times10^{-3}}{4.2} \approx 2.8\ \text{nm}$$

i.e., roughly **0.55 nm of resonance shift per nanometer of width error**, and by the same algebra ~**1.8 nm per nanometer of thickness error**. Now compare to the device's own scales: a Q = 15,000 ring has a linewidth of ~0.1 nm, and a 32-channel WDM weight bank on a 100 GHz grid spaces channels by 0.8 nm. Routine ±2–3 nm dimensional scatter therefore displaces resonances by *tens of linewidths* and *several channel spacings* — fabrication alone determines nothing about which channel a ring lands on. Every ring in a photonic computing system carries a tuner; the only question is the budget.

**Tuning budget.** Suppose thermal tuning delivers ~0.25 nm/mW (a typical non-undercut heater). When fabrication scatter is larger than an FSR, each ring is simply tuned (red-shifted) to its *nearest* resonance target, so the required correction is uniformly distributed over one FSR: for an 18 nm FSR the mean correction is FSR/2 = 9 nm, i.e. ~**36 mW per ring** — and a 64-ring weight bank budgets over 2 W of static power just to stand still. (Bidirectional trimming or deliberate pre-biasing halves the mean correction to FSR/4; undercut heaters cut the power per nm by ~5×.) (Chapter 25 weighs exactly this overhead against electronic accelerators; design countermeasures include athermal claddings, larger-FSR rings, and deliberate resonance pre-biasing.)

**Phase-error budget for meshes.** An MZI arm of length $L$ with width error $\delta w$ accrues phase error $\delta\phi = (2\pi/\lambda)(\partial n_{eff}/\partial w)\,\delta w\, L$. For $L = 100$ μm and $\delta w = 1$ nm: $\delta\phi \approx 0.6$ rad. Uncalibrated, a Clements mesh with such per-arm errors implements a random unitary unrelated to the target; hence the calibrate-then-compensate doctrine of Chapter 12, and the appeal of self-configuring meshes.

## Correlation Is the Designer's Friend

Because variation is spatially correlated (correlation lengths of hundreds of μm to mm), *differences* between nearby devices are far better controlled than absolute values. Practical corollaries:

1. **Match critically paired components** (MZI arms, ring pairs, the two halves of a balanced detector) by placing them close, with identical orientation and surroundings (dummy fill parity included).
2. **Common-mode rejection by architecture**: balanced interferometers, differential drive, and ratiometric readout convert absolute drift into a much smaller differential error.
3. **PCM structures**: foundries scatter process-control monitors — test rings, cutback waveguides, CD bars — across every wafer; their measurements produce the wafer maps from which correlated Monte Carlo models are built. Your own on-die monitor rings serve the same role at chip scale and can feed runtime calibration.

Simulation-side, this is modeled as corner analysis (fast/slow width–thickness corners) plus **spatially correlated Monte Carlo**: draw $(\delta w, \delta t)$ fields with an assumed correlation length, evaluate the circuit model (Section 24.2) per draw, and read off yield against a spec — *parametric yield*.

## Defect-Limited Yield: The Poisson Model

For catastrophic defects, the classical model treats defects as a Poisson process with density $D_0$ (defects per unit area, per critical layer). The probability a die of critical area $A$ collects zero killer defects across $n$ critical layers with densities $D_i$ is

$$Y = \prod_{i=1}^{n} e^{-A D_i} = e^{-A \sum_i D_i}$$

**Worked example.** Die area 10 mm² = 0.1 cm², three critical layers with $D_0 = 0.1$ cm⁻² each: $Y = e^{-0.1 \times 0.3} = e^{-0.03} \approx 97\%$. Photonic dies are small and photonic layers few, so defect yield is generally benign — until you build *large* photonic processors: a reticle-filling 8 cm² photonic interposer with the same defect densities yields $e^{-2.4} \approx 9\%$. This is the same wall that forced electronics toward chiplets, and it drives the analogous photonic conclusions: modular chips, redundant rows/columns in meshes, and architectures (like programmable photonic fabrics) that can route around dead elements.

Refinements — negative-binomial (clustered-defect) models, critical-area analysis weighting $A$ by each layer's sensitivity — matter in production but do not change the scaling story.

## The Takeaway Table

| Perturbation | Typical magnitude | First-order optical effect |
|---|---|---|
| Width $\delta w$ | ±2–5 nm | $\delta\lambda \approx 0.55\,\delta w$; MZI phase ~0.6 rad per nm per 100 μm |
| Thickness $\delta t$ | ±1–5 nm | $\delta\lambda \approx 1.8\,\delta t$ |
| Partial-etch depth | ±2–5 nm | grating coupler λ-shift ~1 nm/nm; rib $n_{eff}$ shift |
| Sidewall roughness σ | 1–3 nm RMS | 1–3 dB/cm strip loss (∝ σ²) |
| Temperature | ±1–5 °C ambient | 0.07–0.08 nm/°C ring shift (Chapter 7) |

Fabrication statistics, thermal drift, and tuning budgets are one continuous subject; the systems of Units V–VI succeed exactly insofar as their calibration loops close over this table.
