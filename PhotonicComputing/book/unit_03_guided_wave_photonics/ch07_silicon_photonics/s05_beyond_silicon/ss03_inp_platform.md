# 7.5.3 The Indium Phosphide Platform

## The Only Platform That Does Everything

There is a remark sometimes made in photonic integrated circuits conferences: "InP is the only platform where you can put a laser, a modulator, an amplifier, and a photodetector on the same chip and have them all work." This is not quite accurate — heterogeneous III-V/Si integration (Section 4.4.3) can achieve the same, with more difficulty — but it captures something true. InP is the natural substrate for III-V compound semiconductor optoelectronics, and it remains the platform of choice for the highest-performance integrated photonic circuits where cost is secondary to capability.

## The InP Material System

Indium phosphide (InP) is a III-V compound semiconductor with a direct bandgap of $E_g = 1.35$ eV at room temperature. "Direct bandgap" means the conduction band minimum and valence band maximum occur at the same point in momentum space ($\mathbf{k} = 0$, the $\Gamma$ point). This allows electrons and holes to recombine radiatively with high efficiency, emitting photons — which is the physical basis of semiconductor lasers and LEDs.

Silicon's indirect bandgap ($E_g = 1.12$ eV, but with the conduction band minimum at $\Delta$ points in k-space) means momentum must be supplied by a phonon for radiative recombination, making efficient light emission essentially impossible in bulk silicon.

The InP material system includes several ternary and quaternary alloys:
- **In₁₋ₓGaₓAsₓP₁₋ᵧ (InGaAsP)**: The workhorse alloy for 1310-nm and 1550-nm lasers and photodetectors. By adjusting Ga and As fractions (with the constraint of lattice matching to InP: $x \approx 0.47y$ for exact lattice match), the bandgap can be tuned from 0.74 eV (In₀.₅₃Ga₀.₄₇As, bandgap 0.74 eV, cutoff 1680 nm) to 1.35 eV (InP itself).
- **InAlGaAs**: Alternative quaternary for lasers with better electron confinement (larger conduction band offset), enabling higher temperature operation.
- **InGaAs (lattice-matched to InP, In₀.₅₃Ga₀.₄₇As)**: The standard absorber for 1550 nm photodetectors, with bandgap 0.74 eV and responsivity up to 1.0 A/W at 1550 nm.

## Waveguide and Component Technologies

InP waveguides are formed by dry etching the InP/InGaAsP layer structure. The refractive indices:
- InP: $n \approx 3.17$ at 1550 nm
- In₀.₅₃Ga₀.₄₇As: $n \approx 3.51$ at 1550 nm
- InGaAsP (1.3-Q material, bandgap 0.95 eV): $n \approx 3.30$ at 1550 nm

A typical InP shallow-etched waveguide (rib waveguide with 1.5 μm rib width, 0.5 μm etch depth from a 2.5 μm thick waveguiding layer) has:
- $n_{\text{eff}} \approx 3.20$
- Propagation loss: 0.3–1 dB/cm (limited by material absorption from doped layers)
- Minimum bend radius: ~100 μm (larger than silicon due to lower index contrast)

Passive components (MMI couplers, AWGs, waveguide crossings) are well-developed in InP, though the larger mode size and lower index contrast compared to silicon means devices are larger: a typical InP AWG is ~5 mm × 3 mm vs. ~2 mm × 1 mm for a silicon AWG with equivalent channel count.

## Active Components on InP

The distinguishing capability of InP is monolithic integration of active components:

### Semiconductor Optical Amplifiers (SOAs)

An SOA is essentially a laser diode operated below threshold, providing optical gain through stimulated emission. In InP, SOAs can be monolithically integrated with passive waveguides by growing a quantum well active region in selected areas (selective area epitaxy) or by butt-coupling different epitaxial regions.

SOA key parameters:
- Gain: 15–30 dB with 50–200 mA drive current
- Gain bandwidth: 50–100 nm (limited by quantum well gain spectrum)
- Noise figure: 6–10 dB (limited by spontaneous emission, similar to EDFAs)
- Saturation output power: 10–20 dBm

SOAs can compensate the insertion loss of passive components in large PICs — a critical capability that Si and Si₃N₄ platforms lack.

### InP Lasers

As detailed in Section 4.4.1, InP DFB lasers are the workhorse source for 1550 nm photonic systems. On a monolithic InP PIC, the laser is grown in the same epitaxial stack as the rest of the circuit, eliminating the packaging loss and alignment that hybrid integration requires.

Monolithic InP lasers integrated in large PICs (from Infinera, JDSU/Lumentum, and other InP PIC vendors) achieve:
- Threshold current: 10–30 mA per laser
- Output power (on-chip): 5–15 dBm after SOA boost
- Wavelength stability: ±0.01 nm with temperature control
- Linewidth: 500 kHz – 2 MHz (DFB, no external cavity)

### Electro-Absorption Modulators (EAMs)

While silicon uses the plasma dispersion effect and LiNbO₃ uses the Pockels effect, InGaAsP/InP can exploit the **quantum-confined Stark effect (QCSE)**: the absorption edge of a quantum well shifts to longer wavelengths under an applied electric field, allowing modulation by switching the material from transparent (field off) to absorbing (field on).

EAMs based on QCSE have:
- $V_\pi L_{\text{EAM}} \approx 0.3$ V·cm — 10× better than silicon
- Bandwidth: 40–70 GHz (monolithic integration)
- Extinction ratio: 10–15 dB
- Drive voltage: −1 to −3 V (reverse bias)
- Residual chirp: $\alpha_H \approx -1$ to $-2$ (some chirp, but controllable)

For photonic computing, EAMs combined with on-chip SOAs provide a compelling amplitude modulation pathway — the insertion loss of the EAM is compensated by the gain of the SOA, so the net insertion loss is near zero.

## InP Photonic Integrated Circuits in Practice

Infinera's coherent optical transceivers, used in long-haul fiber networks since the early 2010s, represent the state-of-art of large-scale InP PIC integration. A single InP chip from Infinera's generation-3 technology integrates [1]:
- 12 × DFB lasers
- 12 × I/Q modulators (MZI-based, electro-optic)
- 24 × photodiodes (balanced pairs)
- Optical multiplexers and demultiplexers (AWGs)
- Semiconductor optical amplifiers

All on a chip ~40 mm² in area, processing 1.2 Tbit/s total capacity. This density is achievable only with InP monolithic integration.

For photonic computing, this level of integration is directly relevant. An InP PIC can implement a complete photonic neural network layer — laser array, matrix of MZI modulators, output photodetectors — without any external components. The challenge is cost: an InP wafer run costs 10–100× more than a comparable silicon photonics run, and InP wafer sizes are still limited to 100–150 mm (vs. 300 mm for advanced silicon).

## InP Foundry Access

The InP PIC foundry ecosystem is less open than silicon photonics but maturing rapidly:

**Smart Photonics** (Eindhoven, Netherlands): MPW-based InP PIC foundry with active/passive integration. Generic process (JePPiX) with standardized building blocks.

**HHI Fraunhofer** (Berlin, Germany): InP foundry offering advanced integration (lasers, SOAs, detectors, modulators).

**POET Technologies** (Toronto, Canada): InP-based platform targeting low-cost integration of light sources with silicon electronics.

**imec** (Leuven, Belgium): InP-on-SOI heterogeneous integration, combining InP III-V gains with silicon photonics routing.

The JePPiX consortium in Europe offers multi-project wafer services for InP, analogous to the ePIXfab silicon photonics MPW. This open-access model is enabling academic researchers to design InP PICs without the multi-million-dollar commitment of a private wafer run.

## Platform Comparison for Photonic Computing

| Platform | Si | Si₃N₄ | LNOI | InP |
|----------|-----|--------|------|-----|
| On-chip laser | No (heterogeneous) | No | No | Yes |
| On-chip amplifier | No | No | Yes (OPA) | Yes (SOA) |
| Modulator type | Plasma dispersion | None | Pockels | EAM / Pockels-like |
| Modulator bandwidth | 25–60 GHz | N/A | >100 GHz | 40–70 GHz |
| $V_\pi L$ | 10–30 V·mm | N/A | 2–3 V·mm | ~3 V·mm (EAM) |
| On-chip detector | Ge-on-Si | No | No | InGaAs |
| Propagation loss | 1–3 dB/cm | 0.01–0.5 dB/cm | 0.02–0.1 dB/cm | 0.3–1 dB/cm |
| Wafer size | 300 mm | 300 mm | 100 mm | 100–150 mm |
| Cost per chip | $ | $ | $$ | $$$ |
| CMOS integration | Yes | Yes | No | No |
| Maturity | Very high | High | Medium | High |

The right platform depends on the application. Silicon photonics with heterogeneous III-V lasers is the dominant commercial choice for cost-sensitive high-volume applications. InP remains dominant for the highest-performance applications where cost is secondary. LNOI is the emerging choice for applications demanding the best modulator performance. Si₃N₄ is ideal for low-power, thermally stable precision photonic circuits and frequency comb generation.

For photonic computing as a system, the most likely near-term architecture is heterogeneous: Si photonic routing, Ge-on-Si detection, III-V lasers bonded to Si, and potentially LNOI modulators or Si₃N₄ passive networks — each material contributing where it excels.

---

## References

[1] Nagarajan, R., Joyner, C.H., Schneider, R.P., Bostak, J.S., Butrie, T., Dentai, A.G., ... & Missey, M. (2005). "Large-scale photonic integrated circuits." *IEEE Journal of Selected Topics in Quantum Electronics*, 11(1), 50–65. [Foundational paper on large-scale InP PICs; design and integration of 40+ components on a single chip.]
