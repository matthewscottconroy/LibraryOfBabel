# Section 2.5: Coherence

We have been treating light as a perfectly monochromatic, perfectly sinusoidal wave with a definite, stable phase. Real light is never exactly like this. A laser has a finite spectral linewidth: its output frequency fluctuates slightly over time, so the phase of the wave drifts randomly. An LED emits a broad spectrum. A thermal source emits light whose phase is entirely random — a superposition of many statistically independent modes.

*Coherence* is the quantitative measure of how wave-like a given light source actually is — how closely it approaches the idealization of a perfectly monochromatic, perfectly spatially uniform wave. It is not a binary property (coherent vs. incoherent) but a continuous variable measured by the *degree of coherence*, which can range from 0 (completely incoherent, perfectly random) to 1 (perfectly coherent, perfectly monochromatic).

Coherence has two distinct aspects:

**Temporal coherence** measures the stability of the wave's phase at a single point in space over time. It is related to the spectral purity of the source: a narrow-linewidth laser has high temporal coherence; a broadband LED has low temporal coherence. The temporal coherence time $\tau_c \sim 1/\Delta\nu$ (inverse of the spectral bandwidth) sets the maximum path length difference over which two copies of the wave can still interfere.

**Spatial coherence** measures the correlation of the wave's phase at different points in space at the same time. It is related to the angular size of the source: a point source (like a single-mode laser output, or a distant star) has high spatial coherence; an extended source (like a lamp filament) has low spatial coherence. Spatial coherence determines whether two points in the wavefront can interfere.

For photonic computing, coherence is not a matter of taste — it is a fundamental engineering constraint:

1. **MZI-based processors require temporal coherence**: The two arms of an MZI may have different lengths. For interference to occur, the path length difference must be less than the coherence length $L_c = c\tau_c = c/\Delta\nu$. For a typical silicon photonic chip with path differences of $\sim 1$ mm, the laser linewidth must be $\Delta\nu < c/L = 300$ GHz (a linewidth of 300 GHz at 1550 nm is $\Delta\lambda \approx 2.4$ nm — easily achieved by standard DFB lasers with $\Delta\nu < 1$ MHz).

2. **Incoherent photonic computing** uses intensity (not field amplitude) as the computational variable. The absence of interference simplifies the design (no phase control needed) but limits the computational operations to non-negative-real-valued weights. Incoherent architectures are explored in Chapter 12.

3. **Quantum photonic processors** typically use single-photon sources whose coherence properties determine the quality of the Hong-Ou-Mandel interference (the fundamental two-photon interference effect underlying linear optical quantum computing, Section 17.3).

## Subsections

- **2.5.1 — Temporal Coherence**: The mutual coherence function, coherence time, coherence length, Wiener-Khinchin theorem.
- **2.5.2 — Spatial Coherence**: The cross-spectral density, spatial coherence length, the van Cittert-Zernike theorem.
- **2.5.3 — The van Cittert-Zernike Theorem**: How spatial coherence develops on propagation from an incoherent source; connection to the angular size of sources.
- **2.5.4 — Coherence in Photonic Computing**: How coherence requirements constrain photonic computing architectures; coherent vs. incoherent optical processing; laser sources and their coherence.
