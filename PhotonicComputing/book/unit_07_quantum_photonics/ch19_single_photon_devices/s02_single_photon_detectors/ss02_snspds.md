# 19.2.2 Superconducting Nanowire Single-Photon Detectors (SNSPDs)

## Operating Principle

An SNSPD is conceptually austere: a superconducting wire, biased with a current just below its critical current, absorbs a photon and momentarily stops superconducting. The standard geometry is a nanowire ~4–8 nm thick and 50–120 nm wide, meandered to cover a 10–20 μm spot, cooled well below its transition temperature $T_c$ (NbN: $T_c \approx 10$ K, operated at 2–4 K; amorphous WSi/MoSi: $T_c \approx 3$–5 K, operated at 0.8–2.5 K).

The detection cycle:

1. **Absorption:** a 0.8 eV photon (1550 nm) breaks Cooper pairs, creating a cloud of quasiparticles — a local "hotspot" where superconductivity is suppressed.
2. **Resistive belt formation:** the bias current, forced to detour around the hotspot, exceeds the local critical current density at the wire edges; a resistive belt forms across the full width of the wire. (The microscopic details — quasiparticle diffusion, vortex-antivortex unbinding — remain an active research area, but the phenomenology is robust.)
3. **Voltage pulse:** the wire's resistance (~kΩ) diverts the bias current into the 50 Ω readout, producing a millivolt-scale pulse after amplification.
4. **Recovery:** the hotspot cools within ~100 ps and the current returns to the wire with time constant $\tau = L_k/R_{load}$, where $L_k$ is the nanowire's **kinetic inductance** (typically ~100 nH–1 μH). Kinetic inductance, not thermal physics, sets the dead time: ~10–50 ns, i.e. count rates of tens of MHz per pixel. Bias resetting too fast causes **latching** (the wire sticks in the resistive state), the practical limit on shortening recovery.

The first SNSPD (Gol'tsman et al., 2001) was a laboratory curiosity with modest efficiency. Two decades of optical and materials engineering turned it into the reference detector for quantum optics.

## Performance: The State of the Art

**System detection efficiency.** SDE factorizes as

$$\eta_{SDE} = \eta_{couple} \times \eta_{absorb} \times \eta_{internal},$$

(fiber-to-chip coupling; absorption in the ~5-nm-thick film; probability that absorption yields a pulse). The absorption problem is solved by embedding the meander in a vertical optical cavity — a mirror and anti-reflection stack that recycles light through the film. Amorphous materials (WSi, MoSi) achieve near-unity internal efficiency at wide bias margins. Landmarks: 93% SDE at 1550 nm with WSi (Marsili et al., 2013); 98% SDE at 1550 nm (Reddy et al., 2020); multiple groups now report SDE ≥ 98%, and commercial multichannel systems routinely specify 80–95%.

**Dark counts.** Intrinsic dark counts (current-assisted vortex hopping) are exponentially suppressed below ~0.8 $I_c$; the practical floor is blackbody photons guided down the fiber. With cold fiber filtering, DCR < 1 cps is standard and values of $10^{-2}$–$10^{-4}$ cps have been achieved for deep-space and dark-matter applications. Compare $10^3$–$10^4$ cps for InGaAs SPADs: four to seven orders of magnitude.

**Timing jitter.** System jitter of 15–50 ps FWHM is routine (dominated by amplifier noise and geometric path-length differences along the meander); short straight nanowires with cryogenic amplifiers reached 2.6–3 ps (Korzh et al., 2020). For photonic computing, jitter matters because temporal indistinguishability and clocking both live at the tens-of-picoseconds scale; for QKD and LiDAR, jitter is directly the timing resolution.

**Wavelength coverage.** From UV to mid-IR (>10 μm demonstrated with narrow wires) — set by hotspot physics, not a bandgap.

**Array scale.** SNSPDs multiplex: row-column readout, time-tagged thermally-coupled buses, and SFQ digital readout have produced kilopixel and, in 2023, a 400,000-pixel SNSPD camera (Oripov et al., 2023). For quantum computing, arrays matter because a fusion-based machine needs $10^4$–$10^6$ detector channels.

## Worked Example: Dark Counts in a Heralded-Source Budget

A heralded SPDC source is pumped at $R = 100$ MHz with per-pulse heralding-click probability $p_h = 10^{-3}$ (i.e., $10^5$ true heralds/s). Compare detectors on the herald arm:

- **SNSPD:** DCR = 1 cps. False-herald fraction $= 1/(10^5 + 1) \approx 10^{-5}$. Negligible.
- **InGaAs SPAD (free-running):** DCR = $5\times10^3$ cps. False-herald fraction $\approx 5\times10^3/1.05\times10^5 \approx 4.8\%$ — every false herald delivers *vacuum* into the quantum circuit, adding ~5% effective loss before the photon even exists. At lower pump powers (purer source, $p_h = 10^{-4}$), the contamination reaches 33%.

This calculation, repeated across every detector in a many-photon experiment, is why essentially all frontier quantum photonics runs on SNSPDs despite the 2–4 K cryogenics.

## Engineering Constraints

The costs are real: a closed-cycle 2.5 K cryocooler (~kW wall power, ~\$100k class), one coax line per pixel unless multiplexed readout is used, and polarization sensitivity of meander geometries (addressed by spiral/fractal layouts or dual-polarization designs). Amorphous-material detectors with the highest efficiencies prefer sub-1 K stages. None of these costs scale kindly to the $10^5$–$10^6$-channel counts of a fault-tolerant photonic machine — which is exactly why waveguide-integrated SNSPDs, the subject of the next subsection, exist.
