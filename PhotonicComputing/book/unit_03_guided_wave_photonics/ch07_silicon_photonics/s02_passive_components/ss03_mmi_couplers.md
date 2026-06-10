# 7.2.3 Multimode Interference (MMI) Couplers

## Self-Imaging in Multimode Waveguides

A multimode waveguide (wider than single-mode) supports multiple guided modes with different propagation constants $\beta_m$. An input field injected at one point excites a superposition of modes, which then propagate and interfere. The self-imaging principle (Ulrich, 1975) states that at specific lengths, the input field is reproduced (single image) or multiple copies appear (multiple images) — because all modes rephase coherently.

For a multimode waveguide of width $W_{MMI}$ at length:

$$L_\pi = \frac{4n_{eff}W_{eff}^2}{3\lambda}$$

(the $3L_\pi$ self-imaging condition), two images form at positions $\pm W_{MMI}/4$ — this is the 1×2 MMI splitter.

For a 2×2 MMI (50:50 splitter), the optimal length is $3L_\pi/2$ for general interference, giving equal power splitting at the two outputs with a $\pm\pi/4$ phase difference.

## MMI Parameters for Silicon Photonics

For a 450 × 220 nm access waveguide and SiO₂ cladding 2×2 MMI at 1550 nm:
- Typical MMI width: $W_{MMI} = 6$ μm
- MMI length: $L_{MMI} \approx 20$–30 μm
- Insertion loss: < 0.3 dB
- Imbalance: < 0.5 dB
- Wavelength bandwidth: > 100 nm (3 dB)
- Fabrication tolerance: ±50 nm in width → < 0.2 dB imbalance change

## MMI vs. Directional Coupler: Tradeoffs

| Property | Directional Coupler | MMI Coupler |
|---|---|---|
| Device length | ~10–30 μm | ~20–50 μm |
| Wavelength dependence | Moderate (~1%/nm) | Low (<0.1%/nm) |
| Fabrication tolerance | Sensitive (exponential on gap) | Robust (self-imaging is tolerant) |
| Phase symmetry | $\pi/2$ phase on cross port | ±$\pi/4$ phase at outputs |
| Loss | < 0.1 dB | < 0.3 dB |
| Design complexity | Simple | Slightly more complex |

For photonic computing applications that require broadband, fabrication-tolerant splitters (e.g., the beamsplitters in an MZI neural network mesh), MMI couplers are preferred. For applications where the exact splitting ratio matters and can be calibrated (e.g., ring resonator couplers, tight-tolerance filters), directional couplers allow finer-grained control.
