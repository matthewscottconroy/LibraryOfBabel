# 19.2.3 Photon-Number Resolution and Waveguide-Integrated Detection

## Why Click Detectors Are Not Enough

SPADs and standard SNSPDs answer "was there light?" — not "how many photons?". For several tasks in this unit that distinction is decisive:

- **Heralding purity:** a threshold herald cannot veto two-pair SPDC events; a photon-number-resolving (PNR) herald can, breaking the $g^{(2)}_h(0) \approx 2\mu$ trade-off of Section 19.1.4.
- **The KLM protocol (Chapter 20):** the nonlinear sign gate's heralding pattern requires knowing that *exactly one* photon (not two) arrived at an ancilla detector.
- **Gaussian boson sampling:** output patterns are photon-number configurations; threshold detectors change the sampled distribution (from hafnians to "Torontonians") and its verification.

## Transition-Edge Sensors: True Energy Resolution

The transition-edge sensor (TES) is a superconducting microcalorimeter: a tungsten (or Ti/Au) film electrothermally locked onto the knife-edge of its superconducting transition, where resistance is a steep function of temperature. An absorbed photon's energy heats the film; the resistance change, read out by a SQUID amplifier, is *proportional to deposited energy*. A TES therefore does not merely count — it measures $n\hbar\omega$, resolving 1, 2, 3, … up to ~10–20 photons with discrete, beautifully separated peaks.

Numbers (NIST-style devices): detection efficiency 95% at 1556 nm (Lita et al., 2008, with later devices ~98%), effectively zero intrinsic dark counts (thermal noise sets a resolution floor instead), but recovery times of ~1 μs (count rates ≲ 1 MHz), timing jitter of ~ns, and an operating temperature of ~100 mK requiring a dilution refrigerator or ADR. TESs detected the photons in Xanadu's Borealis Gaussian boson sampler (Chapter 20) — 16 PNR channels — and in numerous loophole-free Bell tests. They are the gold standard for *fidelity*; their slowness and millikelvin housing keep them a specialist's tool.

**Quasi-PNR with nanowires.** SNSPDs recover partial number resolution by segmentation: an $N$-element multipixel array (or a tapered/series "imaging" nanowire whose pulse height or slew rate encodes how many segments fired) resolves $n$ photons with fidelity limited by the binomial chance of two photons striking one segment, $P_{collision} \approx \binom{n}{2}/N$. Arrays with tens of elements give useful PNR up to $n \sim 5$–10 at full SNSPD speed — the pragmatic choice for heralding and for the pseudo-PNR detection used in Jiuzhang 3.0.

## Waveguide-Integrated SNSPDs

Fiber-coupled detectors treat detection as a packaging problem — one fiber, one cryostat feedthrough, one device — which cannot scale to thousands of channels. The integrated solution places the nanowire *on top of the photonic waveguide*: light propagating in the Si, Si₃N₄, GaAs, or LiNbO₃ waveguide couples evanescently into the 4–8 nm film above it and is absorbed along the propagation direction.

This **traveling-wave geometry** decouples absorption from film thickness: absorption scales as $1 - e^{-\alpha L}$ with $\alpha \sim 0.5$–1 dB/μm, so a 20–50 μm nanowire absorbs >99% of the guided light while occupying a few square microns of chip area. Pernice et al. (2012) demonstrated the canonical device on silicon: 91% on-chip detection efficiency, ~18 ps jitter, sub-Hz dark counts, GHz-scale count rates. Since then, waveguide SNSPDs have been integrated on Si₃N₄, AlN, GaAs (directly above quantum-dot sources), thin-film LiNbO₃, and — in PsiQuantum's 300-mm foundry flow — manufactured by the thousands per wafer alongside sources, filters, and interferometers (Aghaee Rad et al., 2025).

Design points worth internalizing:

1. **On-chip efficiency vs. system efficiency.** The 91–99% figures are waveguide-referenced; getting light onto the chip still costs the fiber-coupling loss (0.5–2 dB). A fully integrated processor — source, circuit, detector on one die — pays this cost once at the input, or never.
2. **Detectors and pumps do not mix.** An on-chip SFWM source is pumped with ~$10^{6}$–$10^{9}$ photons per pulse; the detector must see none of them. Integrated pump-rejection filters with >100 dB extinction, achieved by cascading ring or Bragg filters, are as essential to integrated detection as the nanowire itself.
3. **Cryo-photonics co-design.** Putting detectors on-chip drags the whole photonic circuit to 2–4 K: thermo-optic phase shifters (the workhorse tuning mechanism of Unit 3, ~mW each) are thermally intolerable there, motivating cryogenic-compatible phase shifters — BTO Pockels devices being the leading candidate. Feed-forward electronics (Chapter 20 needs measurement-conditioned switching within nanoseconds) must live close to the detectors, driving work on cryo-CMOS and SFQ logic co-packaging.

## The Detector Scorecard, Completed

| Metric | InGaAs SPAD | SNSPD (fiber) | SNSPD (waveguide) | TES |
|---|---|---|---|---|
| SDE | 10–30% | up to 98% | >90% on-chip | 95–98% |
| DCR | $10^3$–$10^4$ cps | <1 cps | <1 cps | ~0 (noise-floor) |
| Jitter | 50–200 ps | 3–50 ps | ~15–50 ps | ~ns |
| Max rate | ~MHz (gated) | tens of MHz | ~GHz-class | ~1 MHz |
| PNR | No | Quasi (arrays) | Quasi (multi-wire) | Yes (energy-resolving) |
| Temperature | 200–250 K | 0.8–4 K | 0.8–4 K | ~0.1 K |
| Scalability | Modules | ~$10^2$–$10^3$ ch | $10^4$+ per wafer | ~$10^1$ ch |

With sources (19.1) and detectors (19.2) in hand, one question remains: what physics lets us make a single emitter bright, fast, and coherent enough to feed these detectors? The answer is cavity QED.
