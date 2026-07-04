# 25.2.1 Throughput and Energy Efficiency

## Counting Operations Without Cheating

The multiply-accumulate, $y \leftarrow y + w \cdot x$, is the atomic transaction of neural-network arithmetic. By near-universal convention it counts as **two operations** (one multiply, one add), so

$$1\ \text{TMAC/s} = 2\ \text{TOPS}$$

Both conventions appear in the literature, sometimes in the same comparison. The first item on any audit is therefore embarrassingly simple: determine whether a quoted TOPS counted MACs once or twice, and whether the competitor's number used the same rule. A factor of two is worth a product generation.

An $N \times N$ photonic matrix-vector multiplier clocked at vector rate $f$ has peak throughput

$$R_{\text{peak}} = N^2 f\ \ \text{MAC/s} = 2N^2 f\ \ \text{OPS}$$

For $N = 64$ at $f = 10$ GHz: 41 TMAC/s = 82 TOPS. At 50 GHz: 410 TOPS — from one passive mesh the size of a fingernail. These are real, physical numbers; the entire content of this subsection is what must be spent to realize them and how rarely workloads let you.

**Peak versus delivered.** Peak assumes every cycle carries a full $N$-vector against a resident $N \times N$ matrix. Real layers have inconvenient shapes (a 100×64 weight block wastes 36% of a 64-wide core; a 10-class output layer wastes 84%), pipelines bubble while matrices are reprogrammed, and calibration steals cycles. GPUs suffer the same disease — well-shaped large GEMMs run at 40–70% of peak; small or memory-bound kernels at a few percent [5, 6] — so the honest comparison is delivered-to-delivered on a named workload, never peak-to-peak. **Utilization** $= R_{\text{delivered}}/R_{\text{peak}}$ should be reported with every benchmark; its absence is a red flag.

**Latency versus throughput.** Throughput is operations per second at steady state; latency is the delay from one input to its result. Light transits a 5-mm mesh in $t = n_g L / c \approx 70$ ps, but the end-to-end latency adds DAC settling, modulator, TIA, ADC, and digital post-processing — realistically nanoseconds per layer (Section 25.3.1). Latency matters commercially at batch size 1 (interactive inference, control loops), which is exactly where GPUs are weakest; this asymmetry is photonics' best market argument and deserves its own metric, **batch-1 latency at stated accuracy**, rather than being blended into TOPS.

**Reconfiguration rate.** Analog cores add a metric digital chips don't need: how fast, and at what energy, the weight matrix can be changed. Thermo-optic phase shifters settle in ~1–100 μs; MEMS in ~0.1–1 ms; PCM writes in ~0.1–1 μs with finite endurance; DAC-limited electro-optic biasing can reach nanoseconds. A core that multiplies at 10 GHz but reprograms in 10 μs must keep each matrix resident for $\gtrsim 10^5$ vectors to stay utilized — fine for inference serving, fatal for workloads whose operands both change per step (Section 25.3.3).

## Energy per MAC: The Master Equation

Divide every power drawn from the wall by the delivered MAC rate. For an $N \times N$ core at vector rate $f$:

$$\boxed{E_{\text{MAC}} = \underbrace{\frac{E_{\text{laser}}}{N}}_{\text{light, ÷WPE}} + \underbrace{\frac{E_{\text{DAC}} + E_{\text{mod}} + E_{\text{TIA}} + E_{\text{ADC}}}{N}}_{\text{conversion tax (25.1.1)}} + \underbrace{\frac{P_{\text{static}}}{N^2 f}}_{\text{weight hold + control}} + \underbrace{E_{\text{digital}}}_{\text{accumulate, move, requantize}}}$$

with $E_{\text{laser}}$ the wall-plug laser energy per output symbol: detected photon budget × path loss ÷ wall-plug efficiency. The reciprocal conversion to the marketing unit is

$$\text{TOPS/W} = \frac{2}{E_{\text{MAC}}\ [\text{pJ}]} \quad\Rightarrow\quad 1\ \text{pJ/MAC} \leftrightarrow 2\ \text{TOPS/W},\qquad 100\ \text{fJ/MAC} \leftrightarrow 20\ \text{TOPS/W}$$

## Worked Example: A 64×64 Engine at 10 GHz, 6-Bit Target

Assumptions, each traceable to earlier chapters: detected photon budget $10 \times 2^{2b} \approx 4.1\times10^4$ photons per output symbol for $b = 6$ with 10× noise margin (Section 25.2.2); photon energy 0.128 aJ ($\lambda = 1550$ nm); 10 dB total optical path loss; laser wall-plug efficiency 10%; conversion energies from the table of 25.1.1 (DAC 1 pJ, modulator 0.2 pJ, TIA 1 pJ, ADC 3 pJ per sample); thermo-optic hold 30 W for the full mesh (Chapter 7) versus ~40 mW for MEMS/PCM alternatives; 50 fJ/MAC allowance for digital accumulation, requantization, and local SRAM traffic; clocking at 20% of conversion power.

| Contribution | Basis | fJ per MAC |
|--------------|-------|-----------|
| Laser (wall plug) | 5.3 fJ detected → 53 fJ on-chip → 530 fJ wall-plug, ÷ 64 | 8 |
| Conversion tax | 5.2 pJ per in/out sample pair, ÷ 64 | 81 |
| Clock distribution | +20% of conversion | 16 |
| Digital post-processing | accumulate + requantize + SRAM | 50 |
| Weight hold — **thermo-optic** | 30 W ÷ (4096 × 10 GHz) | **732** |
| Weight hold — **MEMS/PCM** | 40 mW ÷ (4096 × 10 GHz) | **1** |
| **Total, thermo-optic hold** | | **≈ 890 → 2.3 TOPS/W** |
| **Total, non-volatile hold** | | **≈ 156 → 12.8 TOPS/W** |

Three lessons, each general:

1. **The photonic computation itself (the laser line) is ~5% of the budget.** Everything else is electronics. A 10× better modulator changes this table by a rounding error; a 2× better ADC changes it materially.
2. **Static weight-holding power can single-handedly erase the advantage.** With thermo-optic phase shifters, this 82-TOPS engine burns 36 W and lands at 2.3 TOPS/W — *below* an H100's dense-INT8 2.8 TOPS/W. With non-volatile or MEMS weights it draws 6.4 W and reaches ~13 TOPS/W, roughly 4–5× the H100 system figure. The choice of phase-shifter technology is worth more than every optical innovation combined.
3. **Scaling $N$ helps until optics says no.** At $N = 256$ the conversion tax falls to ~20 fJ/MAC and the total approaches ~90 fJ/MAC (~22 TOPS/W) — but a 256-deep MZI mesh at 0.15 dB per stage implies ~77 dB of loss, demanding exponentially more laser power (Section 25.2.2). Architectures escape along other axes: WDM channel counts (ring crossbars), spatial tiling of smaller meshes, or free-space optics with $N \sim 10^3$ at the cost of alignment and camera-rate readout.

**Workload-level sanity check.** ResNet-50 inference costs ≈ 2 GMAC per 224×224 image. At 156 fJ/MAC: 0.31 mJ per image; the H100-class 0.71 pJ/MAC system figure gives 1.4 mJ. Both exclude off-chip DRAM traffic for activations — which, at ~nJ per 64-bit access [1], can dominate *both* machines if reuse is poor. Energy per inference at stated accuracy, not TOPS/W, is the number a datacenter operator ultimately pays for; MLPerf exists precisely to force that reporting (25.2.3).

A final note on peak-TOPS/W scatter plots [5]: photonic projections often appear as points floating an order of magnitude above the electronic frontier. When re-plotted using wall-plug $E_{\text{MAC}}$ from the master equation — same boundary as the electronic points — most published photonic systems to date land *on or below* that frontier, with a credible path (large $N$, non-volatile weights, 3D-integrated conversion, few-bit workloads) to sitting several-fold above it. That, and not "1000×," is the defensible claim.

---

## References

[1] Horowitz, M. (2014). "1.1 Computing's energy problem (and what we can do about it)." *ISSCC Digest of Technical Papers*, 10–14. [Digital arithmetic and memory energies used throughout the worked example.]

[2] Nahmias, M.A., Ferreira de Lima, T., Tait, A.N., Peng, H.-T., Shastri, B.J., & Prucnal, P.R. (2020). "Photonic multiply-accumulate operations for neural networks." *IEEE Journal of Selected Topics in Quantum Electronics*, 26(1), 7701518. [Formal definition of the photonic MAC, energy-per-MAC scaling laws, and comparison methodology against digital hardware.]

[3] Al-Qadasi, M.A., Chrostowski, L., Shastri, B.J., & Shekhar, S. (2022). "Scaling up silicon photonic-based accelerators: challenges and opportunities." *APL Photonics*, 7(2), 020902. [Independent full-system TOPS/W model whose conclusions parallel the worked example: conversion and laser overheads dominate; N and precision set the crossover.]

[4] Hamerly, R., Bernstein, L., Sludds, A., Soljačić, M., & Englund, D. (2019). "Large-scale optical neural networks based on photoelectric multiplication." *Physical Review X*, 9(2), 021032. [Architecture-level analysis showing how large N drives optical energy toward and below the femtojoule scale, including standard-quantum-limit accounting.]

[5] Reuther, A., Michaleas, P., Jones, M., Gadepally, V., Samsi, S., & Kepner, J. (2019–2022). "Survey and benchmarking of machine learning accelerators" and successors. *IEEE HPEC*. [The peak-performance vs. power scatter plots that define the electronic frontier referenced here.]

[6] Jouppi, N.P., Young, C., Patil, N., Patterson, D., et al. (2017). "In-datacenter performance analysis of a tensor processing unit." *Proceedings of ISCA 2017*, 1–12. [The exemplary delivered-versus-peak accounting: roofline analysis of a 92-TOPS accelerator on production workloads.]

[7] Williams, S., Waterman, A., & Patterson, D. (2009). "Roofline: an insightful visual performance model for multicore architectures." *Communications of the ACM*, 52(4), 65–76. [The peak-versus-bandwidth-bound framework adapted here as the 'conversion roofline' for analog accelerators.]
