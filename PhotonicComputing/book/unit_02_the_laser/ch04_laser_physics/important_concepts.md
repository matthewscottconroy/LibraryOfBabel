# Important Concepts: Chapter 4 — Laser Physics

---

## 1. Why Two-Level Lasers Are Impossible

The fundamental constraint: $B_{12} = B_{21}$ (equal rates for stimulated absorption and stimulated emission). Under any optical pumping, the maximum steady-state population ratio is $N_2/N_1 \to 1$ as pump intensity $\to \infty$. **Inversion requires $N_2/N_1 > 1$, which is thermodynamically forbidden in a two-level system under resonant optical pumping.** Three- and four-level systems circumvent this by making the pump and laser transitions distinct frequencies.

---

## 2. Threshold Condition

Lasing occurs when round-trip gain = round-trip loss:

$$\Gamma g_{th} = \alpha_i + \alpha_m$$

$$\alpha_m = \frac{1}{2L}\ln\frac{1}{R_1 R_2}$$

Below threshold: spontaneous emission only (LED behavior). Above threshold: stimulated emission dominates, $N$ clamps at $N_{th}$, output power grows linearly with current.

---

## 3. The LI Curve and Slope Efficiency

$$P_{out} = \eta_d \frac{\hbar\omega}{e}(I - I_{th})$$

Differential quantum efficiency $\eta_d = \eta_i \cdot \alpha_m/(\alpha_m + \alpha_i)$. For a good 1550 nm DFB laser: $\eta_d \approx 0.1$–0.5 mW/mA, $I_{th} \approx 5$–20 mA.

---

## 4. Relaxation Oscillation Frequency

$$f_R = \frac{1}{2\pi}\sqrt{\frac{v_g \partial g/\partial N \cdot S}{\tau_p}} \propto \sqrt{I - I_{th}}$$

Sets the intrinsic modulation bandwidth: $f_{-3\text{dB}} \approx 1.55 f_R$. Typical semiconductor lasers: $f_R = 5$–15 GHz, $f_{-3\text{dB}} = 8$–25 GHz.

---

## 5. Schawlow-Townes Linewidth and Henry Factor

Fundamental linewidth (quantum limit):
$$\Delta\nu_{ST} = \frac{\hbar\omega v_g^2 \alpha_{tot}^2 n_{sp}}{4\pi P_{out}}$$

Modified for semiconductors:
$$\Delta\nu = \Delta\nu_{ST}(1 + \alpha_H^2)$$

where $\alpha_H \approx 3$–5 for InGaAsP. This enhancement comes from the coupling of gain changes to refractive index changes (the linewidth enhancement factor $\alpha_H$). Practical linewidths: 1–10 MHz (free-running DFB).

---

## 6. Resonator Stability: $0 \leq g_1 g_2 \leq 1$

where $g_i = 1 - L/R_i$. Stable resonator: paraxial rays remain confined after many round trips. Most integrated photonic lasers are waveguide-confined (inherently stable transversely); resonator stability analysis applies primarily to external-cavity and free-space systems.

---

## 7. VCSEL Key Properties

- Cavity length ~1–2 μm → FSR ~50–100 THz → **inherently single longitudinal mode**
- DBR mirrors with $R > 99\%$ required to compensate short gain length
- Sub-milliamp thresholds, wafer-scale fabrication, circular output beam
- Multi-mode (large aperture) VCSELs dominate 850 nm datacom
- Single-mode VCSELs (small aperture, < 4 μm) for coherent applications

---

## 8. DFB Laser: Grating Provides Frequency Selection

Bragg condition: $\Lambda = m\lambda_B/(2n_{eff})$. For 1550 nm, $n_{eff} = 3.2$: $\Lambda = 242$ nm (first-order). QWS-DFB: $\lambda/4$ phase shift at center forces single-mode lasing in stopband center. SMSR > 40 dB. Temperature tuning: ~0.1 nm/°C (requires TEC for stable WDM operation).

---

## 9. Heterogeneous III-V/Si Integration

The solution to silicon's inability to lase:
- III-V gain material (InP, GaAs) wafer-bonded to SOI substrate
- Evanescent coupling between III-V gain region and Si waveguide
- Confinement factor $\Gamma_{III-V} \approx 0.1$–0.3
- State of art (2023): threshold ~10 mA, output ~5 mW in Si waveguide, linewidth ~2 MHz
- Challenges: thermal management, reliability, yield at scale

---

## 10. Microresonator Kerr Combs

Single pump CW laser → cascaded FWM in high-Q ring → frequency comb. Dissipative Kerr soliton (DKS) state: stable, coherent, sech²-spectrum comb. Key parameters:

| Quantity | Expression | Typical value (Si₃N₄, $R$=100 μm) |
|---|---|---|
| FSR (line spacing) | $c/(2\pi n_g R)$ | ~230 GHz |
| Threshold pump | $\propto \kappa_{tot}^2 A_{eff}/(n_2 \omega)$ | ~50–100 mW |
| Lines in C-band | ~4.4 THz / FSR | ~19 lines |
| Power per line | ~0.5–2% of pump | 0.5–2 mW |

DKS combs enable WDM photonic matrix processing from a single compact pump laser source.

---

## 11. Key Numerical Values for Photonic Computing Design

| Quantity | Value | Implication |
|---|---|---|
| DFB threshold | 5–20 mA | Low power driver circuits |
| DFB linewidth | 1–10 MHz | Coherence length ~30–300 m; adequate for chip |
| DFB $\lambda/T$ coefficient | ~0.1 nm/°C | TEC needed for WDM; ~0.5 W power |
| TEC power | ~0.5 W/laser | Major power cost at scale |
| VCSEL efficiency | up to 60% | Best wall-plug efficiency of any laser |
| Si direct modulation BW | 20–30 GHz | ~50 Gbps NRZ per channel |
| On-chip III/V laser power | 1–10 mW | Marginal for large networks |
| Comb power/line | 0.5–2 mW | Borderline for direct modulation |
