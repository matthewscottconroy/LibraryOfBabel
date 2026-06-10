# Important Concepts: Chapter 5 — Photodetectors

---

## 1. Quantum Efficiency and Responsivity

$$\eta = \frac{\text{electron-hole pairs collected}}{\text{incident photons}}, \quad \mathcal{R} = \frac{\eta e}{\hbar\omega} = \frac{\eta e \lambda}{hc} \text{ [A/W]}$$

At 1550 nm: $\mathcal{R}_{max} = 1.25$ A/W (for $\eta = 1$). Typical Ge-on-Si: $\mathcal{R} \approx 0.8$–1.0 A/W.

---

## 2. Bandwidth-Efficiency Tradeoff

- **Transit-time limit**: $f_{tr} \approx 0.45 v_s/d$ (thicker absorber → slower)
- **RC limit**: $f_{RC} = 1/(2\pi R C_j)$ (thicker absorber → smaller C → faster RC, but slower transit)
- **Waveguide photodetector**: decouples the two — absorption along waveguide axis, depletion across it. Best of both worlds: high QE + high bandwidth.

State of art (Ge-on-Si waveguide PD): > 60 GHz at > 0.9 A/W.

---

## 3. Shot Noise — The Quantum Limit

$$\langle i^2_{shot}\rangle = 2eI_{ph}B$$

White noise spectrum. Irreducible quantum floor. Cannot be reduced by circuit design — only by reducing bandwidth or increasing signal power.

---

## 4. Johnson (Thermal) Noise

$$\langle i^2_{thermal}\rangle = \frac{4k_BTB}{R_F}$$

Dominates at low signal power; decreases with increasing $R_F$ (but bandwidth decreases too). Shot-noise-limited operation requires $I_{ph} > 2k_BT/(eR_F)$.

---

## 5. SNR Regimes

| Regime | Dominant noise | SNR ∝ |
|---|---|---|
| Thermally limited | Johnson | $P^2$ |
| Shot-noise limited | Shot | $P$ |
| RIN limited | Laser RIN | 1 (independent of P) |

For typical photonic computing conditions (1 mW, 1 GHz BW, $R_F = 10$ kΩ): thermally limited with ENOB ~6–9 bits.

---

## 6. Detector Comparison

| Detector | Wavelength | $\eta$ | Bandwidth | Sensitivity | Use |
|---|---|---|---|---|---|
| p-i-n (InGaAs) | 900–1700 nm | 70–95% | up to 60 GHz | ~−25 to −30 dBm | Analog, classical |
| APD (InGaAs) | 900–1700 nm | 70% | up to 40 GHz | 5–10 dB better | Long-reach links |
| SPAD (InGaAs) | 900–1600 nm | 15–25% | <100 MHz | Single-photon | Quantum (telecom) |
| SNSPD | 400–2000 nm | > 90% | > 1 GHz | Single-photon | Quantum (best) |
| Ge-on-Si (waveguide) | 1100–1600 nm | 80–95% | 40–60 GHz | −25 dBm | On-chip integration |

---

## 7. SNSPD vs. SPAD

For quantum photonic computing, SNSPDs dominate in every performance metric:
- SDE: 90–98% (SNSPD) vs. 15–25% (SPAD at 1550 nm)
- DCR: <1 cps (SNSPD) vs. 100–10,000 cps (SPAD)
- Jitter: <3 ps (SNSPD) vs. 50–200 ps (SPAD)
- Dead time: 5–10 ns (SNSPD) vs. 10–100 ns (SPAD)
- Cost/complexity: Cryogenic (disadvantage) vs. room temperature (advantage)

---

## 8. TIA Key Tradeoff

$$R_F \cdot f_{3\text{dB}}^2 = \frac{A}{2\pi C_T}$$

Larger feedback resistor $R_F$ → lower noise (higher SNR) but lower bandwidth. For photonic computing at 1 GHz, use larger $R_F$ (~1–10 kΩ) and gain 10–20 dB SNR over a 10–25 GHz TIA.

---

## 9. Coherent Detection: 3 dB Advantage + RIN Rejection

Balanced homodyne detection achieves:
$$\text{SNR}_{coherent} = \frac{\mathcal{R}P_s}{eB}$$

versus direct detection $\mathcal{R}P_s/(2eB)$. Additionally, laser RIN cancels in the balanced subtraction. Requires LO laser (phase-coherent with signal) and 90° hybrid for IQ detection.

---

## 10. Key Numbers for Photonic Computing Design

| Quantity | Value | Implication |
|---|---|---|
| Shot noise floor (1 mW, 1 GHz) | 0.18 μA rms | Sets minimum noise at this power/BW |
| Johnson noise ($R_F$ = 1 kΩ, 1 GHz) | 4 μA rms | Dominates at 1 mW signal |
| Shot-noise-limited threshold | $P > 52$ μW (1 kΩ, 300 K) | Need > 50 μW for shot-noise-limited |
| Ge-on-Si dark current | 50–200 nA | Negligible above 1 μW signal |
| SNSPD cryogenic power | 1–2 kW at 4 K | Major cost for quantum computing systems |
| Ideal ENOB (shot-noise, 1 mW, 1 GHz) | ~8.8 bits | Quantum limit |
| Practical ENOB (analog photonic) | 5–8 bits | Modulator nonlinearity + thermal noise |
