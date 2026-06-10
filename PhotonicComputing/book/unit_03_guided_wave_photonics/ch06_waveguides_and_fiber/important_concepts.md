# Important Concepts: Chapter 6 — Waveguides and Fiber

---

## 1. TIR Guidance and the Ray Optic Model

Light is guided when angle exceeds critical angle: $\theta_c = \arcsin(n_2/n_1)$. For Si/SiO₂: $\theta_c = 24.5°$. The ray model motivates guidance but cannot predict mode profiles or cutoff conditions.

---

## 2. TE/TM Eigenvalue Equations

**TE even modes**: $\kappa\tan(\kappa d/2) = \gamma$  
**TM even modes**: $\kappa\tan(\kappa d/2) = (n_1/n_2)^2\gamma$

High-index-contrast waveguides (silicon) show large TE/TM splitting. Standard silicon photonic circuit uses TE polarization (lower loss, better coupling).

---

## 3. Single-Mode Condition

$V = 2\pi d\text{NA}/\lambda < 2.405$ (fiber); approximately $w < 450$ nm at 220 nm height for Si strip at 1550 nm. Standard silicon photonic waveguide: 450 × 220 nm — widest single-mode strip, maximizing mode area.

---

## 4. Confinement Factor $\Gamma$

$$\Gamma = \frac{\int_{core}|E|^2 dA}{\int_{all}|E|^2 dA}$$

For 450 × 220 nm Si strip at 1550 nm, TE: $\Gamma \approx 0.8$, $A_{eff} \approx 0.14$ μm².  
Nonlinear coefficient: $\gamma = n_2\omega/(cA_{eff}) \approx 290$ W$^{-1}$m$^{-1}$ — 5000× higher than SMF.

---

## 5. SMF-28: Key Parameters

| MFD | Loss | Dispersion | $A_{eff}$ | $\gamma$ |
|---|---|---|---|---|
| 10.4 μm | 0.18 dB/km | 17 ps/(nm·km) | 85 μm² | 1.3 W$^{-1}$km$^{-1}$ |

Cutoff at 1260 nm; $\lambda_{ZD} \approx 1310$ nm.

---

## 6. Fiber Attenuation Mechanisms

- Rayleigh: $\alpha_R = 0.78/\lambda^4$ dB/km (μm) → 0.137 dB/km at 1550 nm. Fundamental.
- IR absorption: rises steeply above 1700 nm. Fundamental.
- OH absorption: 1383 nm peak (removable by purification).
- Minimum: 0.18 dB/km at 1570 nm.

---

## 7. WDM Nonlinear Limits

| Effect | Threshold | Mitigation |
|---|---|---|
| SBS | ~1 mW (coherent, single channel) | Spectral broadening, polarization scrambling |
| SRS | ~0.5 W (single channel in 50 km) | WDM power management |
| SPM | $\phi_{NL} > 1$ rad at ~35 mW in 22 km $L_{eff}$ | Launch power optimization |
| FWM | Near $\lambda_{ZD}$; negligible in SMF-28 at 100 GHz spacing | Use finite dispersion fiber |

---

## 8. EDFA Key Parameters

- Gain: 20–40 dB per stage; gain bandwidth 1530–1565 nm (C-band)
- Quantum NF limit: 3 dB ($n_{sp} = 1$, full inversion)
- Practical NF: 4–7 dB
- Saturation output power: 10–20 dBm
- Upper level lifetime: 10 ms (slow dynamics → transient gain issues)

---

## 9. Fiber-to-Chip Coupling Loss

Direct (mode mismatch Si/SMF): ~−22 dB catastrophic.  
Inverse taper: ~1–2 dB total.  
Grating coupler: ~0.5–1.5 dB, 30 nm bandwidth.  
Coupling loss is the dominant loss for chip-scale photonic computing systems with external fibers.
