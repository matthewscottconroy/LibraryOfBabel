# 7.4.3 Phase-Change Material Switches

## Non-Volatility: The Key Concept

Every switch technology we have examined so far — electro-optic, thermo-optic, MEMS — has one thing in common: maintaining a given state requires either continuous power (thermo-optic) or a continuous applied voltage (electro-optic, MEMS electrostatic). Remove the power, and the state reverts to a default. This is the volatile paradigm.

In electronic computing, the analogy is SRAM and DRAM: they hold state as long as power is supplied, but lose it on power off. Flash memory and magnetic storage are non-volatile: state is stored as a physical material property (charge in a floating gate, or magnetic domain orientation) that persists without power.

Phase-change materials (PCMs) are the photonic equivalent of flash memory. They can be switched between two (or more) structural phases — typically an amorphous phase and a crystalline phase — that have dramatically different optical properties, and they remain in whichever phase they were last set until deliberately changed. The switching requires a pulse of energy (optical or electrical), but once switched, the state is held at zero power indefinitely — for decades if the material is not disturbed.

This non-volatility is transformative for large-scale photonic computing. A matrix of non-volatile optical weights consumes no static power once programmed. For systems with $N^2 = 4096$ weights ($N = 64$), the difference between thermo-optic ($\sim 40$ W static) and non-volatile PCM ($\sim 0$ W static) is the difference between a system that is physically possible and one that is not.

## Ge₂Sb₂Te₅: The Archetype Phase-Change Material

Ge₂Sb₂Te₅ (GST) is the material that enabled the phase-change memory industry (Blu-ray recording, phase-change RAM). It has been intensively studied since the 1980s, and its properties are well-characterized.

**Amorphous phase**: GST as deposited (or after a rapid quench from the melt) is amorphous — the atoms are arranged randomly, like frozen glass. At 1550 nm: $n_a \approx 4.0$, $k_a \approx 0.4$ (where $\tilde{n} = n + ik$).

**Crystalline phase**: After annealing above the crystallization temperature (~160°C for GST-225), the atoms rearrange into a face-centered cubic (rock-salt) structure. At 1550 nm: $n_c \approx 6.5$, $k_c \approx 1.2$.

The contrast is large: $\Delta n \approx 2.5$, $\Delta k \approx 0.8$. But both phases have substantial absorption ($k > 0$) at 1550 nm, which limits the useful length of a GST-clad waveguide:

$$L_{\text{max}} \approx \frac{4.34}{2\pi k \Gamma / \lambda} = \frac{4.34\lambda}{2\pi k \Gamma}$$

For GST in amorphous phase on a silicon waveguide with $\Gamma_{\text{GST}} \approx 0.1$ (overlap with a thin GST cladding layer), $k_a = 0.4$, $\lambda = 1550$ nm:

$$L_{\text{max}} \approx \frac{4.34 \times 1550}{2\pi \times 0.4 \times 0.1} \approx 27 \text{ μm per dB of loss}$$

Even 1 dB of allowed loss limits the GST-clad section to ~27 μm. This means GST devices must be very short to keep insertion loss acceptable.

For a short GST-clad segment of length $L$, the phase shift between amorphous and crystalline states is:

$$\Delta\phi = \frac{2\pi}{\lambda}\Delta n_{\text{eff}} \cdot L \approx \frac{2\pi}{\lambda}\Gamma_{\text{GST}}\Delta n_{\text{GST}} \cdot L$$

For $\Gamma_{\text{GST}} = 0.1$, $\Delta n_{\text{GST}} = 2.5$, $L = 10$ μm:

$$\Delta\phi = \frac{2\pi}{1550} \times 0.1 \times 2.5 \times 10 \approx 0.01 \text{ rad}$$

This is too small for a useful phase modulator. The absorption, unfortunately, is the useful property for a binary absorptive switch (amplitude modulation), not phase modulation. GST-based devices at 1550 nm therefore operate primarily as **absorptive** (amplitude) switches rather than phase modulators [1].

## Ge₂Sb₂Se₄Te₁ (GSST): Reduced Absorption

A key breakthrough came from a composition optimization. Gu et al. (2021) demonstrated that substituting selenium for tellurium in GST — yielding Ge₂Sb₂Se₄Te₁ (GSST, though the specific notation varies in the literature) — produces a phase-change material with dramatically reduced absorption in the C-band while retaining substantial index contrast [2].

For GSST at 1550 nm:
- **Amorphous**: $n_a \approx 3.35$, $k_a \approx 0$ (essentially transparent)
- **Crystalline**: $n_c \approx 5.1$, $k_c \approx 0.3$

The amorphous phase is now nearly transparent, allowing much longer interaction lengths. For a waveguide with GSST in amorphous phase, $L_{\text{max}}$ diverges (zero absorption). For the crystalline phase with $k_c = 0.3$ and $\Gamma = 0.1$:

$$L_{\text{max}} = 27 \text{ μm} \times \frac{k_a}{k_c} \times \frac{1}{1} \approx 90 \text{ μm per dB}$$

— three times better than GST, and with the amorphous phase being lossless, the "off" state of the switch can be made essentially transparent.

The index contrast $\Delta n = n_c - n_a \approx 1.75$ is large enough to produce useful phase shifts over ~100 μm length:

$$\Delta\phi = \frac{2\pi}{1550} \times 0.1 \times 1.75 \times 100 \approx 0.071 \text{ rad}$$

Still not $\pi$ — for a full $\pi$ phase shift in GSST, one needs $L \approx 1.4$ mm or $\Gamma = 0.7$ (full immersion in GSST). Neither is trivially achievable, but both are closer to practical than GST.

## Switching Mechanism

GST and GSST are switched between phases using brief high-intensity pulses:

**Crystallization (amorphous → crystalline)**: A moderate-intensity, long pulse (typically 100 ns–10 μs, enough to heat the material above $T_c \approx 160°C$ but below $T_m \approx 600°C$) allows atoms to rearrange into the crystalline configuration. This is a slow diffusion-limited process — the material must be held above $T_c$ long enough for crystallites to nucleate and grow.

**Amorphization (crystalline → amorphous)**: A high-intensity, short pulse (typically 1–10 ns) heats the material above $T_m$ (melting it), followed by rapid quenching (the pulse ends and heat diffuses away). The rapid cooling prevents recrystallization, freezing the melt in an amorphous state.

For on-chip optical switching, both processes can be driven optically (using a pump pulse via the waveguide or a focused laser) or electrically (using a microheater adjacent to the PCM layer). Electrical switching is preferred for chip integration.

Measured switching parameters for GST-clad silicon waveguide switches:

| Parameter | GST (optical) | GST (electrical) | GSST (electrical) |
|-----------|---------------|------------------|-------------------|
| Crystallization pulse | 300 ns, 10 mW | 500 ns, 5 mW | 1 μs, 2 mW |
| Amorphization pulse | 1 ns, 300 mW | 5 ns, 50 mW | 10 ns, 20 mW |
| Energy per switch event | ~3 pJ | ~25 pJ | ~200 pJ |
| Switching cycles | >10⁶ | ~10⁴ | ~10³ |
| Retention (at RT) | >10 years | >10 years | ~10 years |

The lower cycle count for GSST reflects its younger development; GST's cycle endurance has been optimized by the optical data storage industry.

## Multi-Level Operation

An important property of phase-change materials for photonic computing is that partial crystallization is achievable: rather than switching between fully amorphous and fully crystalline states, the PCM can be left in an intermediate state with a fraction $f_c$ of the volume crystallized. For a linear mixture:

$$n_{\text{eff}}(f_c) \approx f_c n_c + (1-f_c) n_a$$

(This linear mixing rule is approximate; the actual effective medium theory is more complex.) By controlling the pulse energy and duration, $f_c$ can be set to arbitrary values between 0 and 1, enabling analog weight storage with multiple levels.

Feldmann et al. (2019) [3] demonstrated a PCM-based photonic synaptic device with 34 distinguishable levels per cell on a silicon photonic chip. Each level corresponds to a different crystallization fraction, with the optical transmission reading out the stored value. This is the photonic equivalent of multi-level cell (MLC) flash memory.

The precision achievable in PCM analog states is currently ~5 bits (32 levels) per cell in optimized demonstrations, limited by:
- Stochastic nucleation of crystalline grains (shot-to-shot variability in $f_c$)
- Thermal diffusion of crystalline domains at elevated temperature (retention drift at ~60°C)
- Device-to-device variability in the thin-film PCM thickness and composition

With improved deposition uniformity and pulse control algorithms, 6–7 bits per cell is a plausible near-term target.

## Integrated PCM Devices in Silicon Photonics

PCM layers are integrated into silicon photonic waveguides by depositing a thin film (typically 10–20 nm) of GST or GSST directly onto the waveguide surface. The standard process:

1. Silicon waveguide fabricated by standard dry etching (Section 7.1.2)
2. PCM thin film deposited by sputtering (for GST) or ALD (for GSST precursors): 10–20 nm thickness
3. Capping layer of ITO or TiN deposited to protect PCM from oxidation and provide electrical contact
4. Microheater (TiN strip) patterned above the PCM for electrical switching
5. Back-end-of-line metallization for electrode routing

Demonstrated devices include:
- Absorptive switches with >10 dB contrast in 1-μm GST cell [1]
- Phase-modulating switches using GSST with 0.04 dB/μm insertion loss change [2]
- All-optical synaptic devices driven by waveguide-coupled pump pulses [3]
- Integrated matrices for optical weight banking (Feldmann et al. 2021) [4]

Feldmann et al. 2021 [4] demonstrated the most complete PCM photonic computing demonstration to date: a 4-input, 4-output photonic matrix-vector multiplier using PCM-based weight banks on a silicon photonic chip. The matrix elements were implemented as PCM absorptive cells; the input was wavelength-encoded (WDM); the output was detected by germanium photodetectors on-chip. The system performed classification tasks (vowel recognition) with accuracy comparable to software, demonstrating end-to-end in-memory optical computation.

## Comparison with Volatile Switching

The complete comparison across switching technologies:

| Property | Electro-optic (EO) | Thermo-optic (TO) | MEMS | PCM |
|----------|-------------------|-------------------|------|-----|
| Static power | Low (capacitive) | High (resistive) | ~0 | 0 |
| Switching speed | GHz | 0.1–1 MHz | 0.1–1 MHz | 1–100 MHz |
| Analog precision | 6–8 bits | 8–10 bits | 5–7 bits | 4–6 bits |
| Non-volatile | No | No | Partial | Yes |
| Phase shift | Phase only | Phase only | Phase only | Phase + amplitude |
| Integration | Standard CMOS | Standard CMOS | Complex | Moderate |
| Maturity | High | High | Medium | Low-Medium |

For photonic computing with slowly updated, large weight matrices: PCM offers the most compelling long-term energy advantage. For rapidly reconfigurable systems: electro-optic remains dominant. Thermo-optic is the practical workhorse for research demonstrations and small-scale systems.

---

## References

[1] Rios, C., Stegmaier, M., Hosseini, P., Wang, D., Scherer, T., Wright, C.D., Bhaskaran, H., & Pernice, W.H.P. (2015). "Integrated all-photonic non-volatile multi-level memory." *Nature Photonics*, 9(11), 725–732. [First integrated PCM (GST) optical memory on silicon photonic waveguide; multi-level demonstration.]

[2] Gu, T., Kim, H.-J., Rivero-Baleine, C., & Hu, J. (2021). "Reconfigurable metasurfaces towards commercial success." *Nature Photonics*, 17, 48–58. [For GSST in photonics, the key paper is: Zhang, Y., Chou, J.B., Li, J., Li, H., Du, Q., Yadav, A., ... & Hu, J. (2019). "Broadband transparent optical phase change materials for high-performance nonvolatile photonics." *Nature Communications*, 10(1), 4279. Introduces GSST with near-zero absorption at 1550 nm.]

[3] Feldmann, J., Youngblood, N., Wright, C.D., Bhaskaran, H., & Pernice, W.H.P. (2019). "All-optical spiking neurosynaptic networks with self-learning capabilities." *Nature*, 569(7755), 208–214. [PCM-based photonic synapse with 34 analog levels; 1-shot and spike-based learning.]

[4] Feldmann, J., Youngblood, N., Karpov, M., Gehring, H., Li, X., Stappers, M., ... & Bhaskaran, H. (2021). "Parallel convolutional processing using an integrated photonic tensor core." *Nature*, 589(7840), 52–58. [PCM-based photonic tensor core: 4×4 matrix, WDM encoding, on-chip Ge photodetectors, vowel classification demo.]
