# 5.2.4 Superconducting Nanowire Single-Photon Detectors (SNSPDs)

## The Superconducting Advantage

The performance limitations of SPADs — modest efficiency (~20%), slow recovery (>10 ns dead time), high dark counts at telecom wavelengths, large timing jitter — stem from the physics of avalanche multiplication in semiconductors. The SNSPD sidesteps these limitations by using a completely different physical mechanism: the disruption of superconductivity by a single photon.

An SNSPD consists of a narrow (50–150 nm) meander of superconducting nanowire (typically NbN or WSi) on a substrate, cooled to 0.8–3 K and biased at a current just below the critical current $I_c$. When a photon is absorbed, it creates a "hot spot" — a small region of normal (non-superconducting) material — that causes the current to redistribute around the hot spot. The increased current density in the narrow channels flanking the hot spot drives them above $I_c$, quenching superconductivity across the full wire width. The resulting resistive region produces a voltage pulse detectable by a 50 Ω transmission line. The wire recovers to the superconducting state in ~5–10 ns as the hot spot cools.

## SNSPD Performance

| Parameter | State of art (2023) | Notes |
|---|---|---|
| System detection efficiency (SDE) | > 98% at 1550 nm | World record; typical commercial: 80–95% |
| Dark count rate | < 1 count/s | 1000× better than InGaAs SPAD |
| Timing jitter | < 3 ps FWHM | 10–100× better than SPAD |
| Dead time | 5–10 ns | 5–10× better than SPAD |
| Polarization sensitivity | Low (~3 dB) with careful design | Can be suppressed |
| Operating temperature | 0.8–3 K (closed-cycle cryocooler) | Major practical disadvantage |

The 98% SDE record was achieved by Reddy et al. (2020) using cavity-integrated WSi SNSPDs in a silicon photonic waveguide [1]. For comparison, the best InGaAs SPADs achieve ~80% PDE only under cryogenic gating and with very high dark count rates.

## SNSPDs in Quantum Photonic Computing

The combination of near-unit efficiency, low dark counts, and fast timing makes SNSPDs the detector of choice for:

1. **Boson sampling and Gaussian boson sampling**: Detecting $n$-photon coincidences requires $\eta^n$ efficiency; 95% SDE vs. 20% SPAD SDE makes a difference of $(0.95/0.2)^{20} = 10^{13}$ in count rate for 20-photon detection.

2. **Linear optical quantum computing**: LOQC protocols require high-fidelity photon detection with low dark counts to maintain the fidelity of quantum gate operations.

3. **Quantum key distribution (QKD)**: SNSPD-based QKD receivers achieve the maximum secure key generation rates.

4. **Photon-number-resolving (PNR) detection**: Multiplexed SNSPD arrays can distinguish photon number (0, 1, 2, ...) by counting coincidences. PNR detection enables higher-quality heralded single-photon sources.

## The Practical Challenge: Cryogenics

The fundamental disadvantage of SNSPDs is their cryogenic operating temperature (0.8–3 K), requiring closed-cycle dilution refrigerators or sorption coolers with significant electrical power consumption (1–2 kW for 4 K, 5–10 kW for sub-1 K). This makes SNSPDs unsuitable for anything but laboratory and specialized high-value applications.

The photonic computing community is watching two potential developments:
1. **Higher-$T_c$ superconducting nanowires**: Materials like MgB₂ ($T_c = 39$ K) could enable operation at 10–15 K (accessible with compact Stirling coolers), dramatically reducing the cost and footprint of cryogenic operation.
2. **Room-temperature single-photon detection via other mechanisms** (e.g., quantum dots with resonant fluorescence detection): Still far from SNSPD performance but under active research.

## Reference

[1] Reddy, D.V., Nerem, R.R., Nam, S.W., Mirin, R.P., & Verma, V.B. (2020). "Superconducting nanowire single-photon detectors with 98% system detection efficiency at 1550 nm." *Optica*, 7(12), 1649–1653.
