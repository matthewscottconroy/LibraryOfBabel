# 9.1.3 The Optical Channel and Its Limits

## The Optical Channel Model

A complete optical communications system has several noise sources (as analyzed in Chapter 5 for the detector):
1. **ASE noise** (from EDFAs): The dominant noise source in long-haul systems. Power spectral density $S_{\text{ASE}} = n_{sp}(G-1)\hbar\omega$.
2. **Shot noise**: Quantum noise from photon counting, $\sigma^2_{\text{shot}} = 2eI_{\text{ph}}B$.
3. **Thermal noise**: Johnson noise in the receiver electronics, $\sigma^2_{\text{thermal}} = 4k_BTB/R_F$.
4. **Nonlinear noise**: SPM, XPM, FWM contributions modeled as effective AWGN in the Gaussian noise model.

For long-haul systems (many cascaded EDFAs), ASE noise dominates. For short-reach systems (no amplifiers), shot noise or thermal noise dominates.

## The Linear Optical Channel Capacity

For an EDFA-amplified fiber link with $N_{\text{span}}$ spans of length $L_s$ and amplifier noise figure $F_n$, the OSNR (optical signal-to-noise ratio) at the receiver is (from Section 6.4.1):

$$\text{OSNR} = \frac{P_{\text{launch}}}{N_{\text{span}} F_n \hbar\omega B_{\text{ref}}}$$

where $B_{\text{ref}}$ is the reference bandwidth (typically 12.5 GHz = 0.1 nm). For a system with total capacity $C_{\text{total}}$ over $N_{\text{ch}}$ WDM channels each of bandwidth $B_{\text{ch}}$:

$$C_{\text{total}} = N_{\text{ch}} B_{\text{ch}} \log_2(1 + \text{OSNR} \times B_{\text{ref}}/B_{\text{ch}})$$

Current state-of-art long-haul systems:
- Spectral bandwidth: C+L band, ~10 THz
- OSNR per channel: 18–25 dB (after FEC threshold)
- Spectral efficiency: 5–9 bits/s/Hz
- Total capacity: 50–90 Tbit/s per fiber pair [1]

## The Quantum Limit: Photon Counting Capacity

At short distances (no amplifiers) and very low noise levels, the shot noise becomes dominant and the relevant limit is the quantum channel capacity.

The quantum channel for optical communications is the **bosonic channel**: each temporal mode of the electromagnetic field carries, on average, $\bar{n}$ photons (for coherent state encoding). The quantum capacity (Holevo bound) for a coherent state channel with shot noise is [2]:

$$C_q = B g(\bar{n}) \quad \text{where} \quad g(\bar{n}) = (1+\bar{n})\log_2(1+\bar{n}) - \bar{n}\log_2\bar{n}$$

This is the **Holevo capacity** for the bosonic channel. For large $\bar{n}$: $g(\bar{n}) \approx \log_2\bar{n}$ — the classical Shannon-Hartley limit. For $\bar{n} \ll 1$: $g(\bar{n}) \approx \bar{n}\log_2(1/\bar{n})$ — capacity per photon diverges, but the total capacity goes to zero.

The classical Shannon-Hartley theorem, applied with shot noise as the noise source, gives:

$$C_{\text{classical}} = B\log_2(1 + \text{SNR}_{\text{shot}}) = B\log_2\left(1 + \frac{(\mathcal{R}P)^2}{2e\mathcal{R}PB \cdot R_F}\right)$$

Wait — let me use a cleaner model. For a photodetector receiving $\bar{n}$ photons per mode:
- Signal: photocurrent $\propto \bar{n}$
- Shot noise: $\propto \sqrt{\bar{n}}$
- SNR $= \bar{n}^2/\bar{n} = \bar{n}$ (for Poisson statistics)

So $C_{\text{classical,shot}} = B\log_2(1 + \bar{n})$, which is slightly less than the quantum Holevo capacity $g(\bar{n})$.

The **quantum advantage** (Holevo vs. classical) is:

$$\frac{C_q}{C_{\text{classical,shot}}} = \frac{g(\bar{n})}{\log_2(1+\bar{n})}$$

For $\bar{n} = 1$: $g(1) = 2\log_2 2 - 0 = 2$ bits, vs. $\log_2 2 = 1$ bit. The quantum capacity is twice the classical at $\bar{n} = 1$.

For $\bar{n} = 10$: $g(10) = 11\log_2 11 - 10\log_2 10 = 38.5 - 33.2 = 5.3$ vs. $\log_2 11 = 3.46$. The quantum advantage is 1.5×.

The gap decreases as $\bar{n}$ increases; for $\bar{n} > 100$ it is negligible. Modern optical systems operate at $\bar{n} \sim 10^3$–$10^6$ photons per bit, far above the quantum limit — not because the quantum limit is unachievable, but because other system constraints (power margin, wavelength plan) require operating well above the minimum power level.

## Spectral Efficiency vs. Distance

One of the most useful characterizations of an optical communication system is its **spectral efficiency-distance product** (SE × distance), which shows the fundamental tradeoff between how fast you can transmit and how far:

- Long-haul (>1000 km): SE limited by OSNR → ~5–9 bits/s/Hz
- Metro (~100 km): SE ~7–12 bits/s/Hz  
- Data center interconnects (~1–10 km): SE ~4–6 bits/s/Hz (direct detection dominates)
- Chip-to-chip interconnects (~cm–m): SE ~1–4 bits/s/Hz (energy-per-bit dominated)

For photonic computing, the relevant regime is chip-to-chip and board-to-board: distances of 1 cm to 1 m, where energy efficiency (pJ/bit) matters more than spectral efficiency. The Shannon limit is not typically the constraint in these applications; instead, the constraints are modulator drive voltage, detector sensitivity, and the energy overhead of clocking and driving the SerDes (serializer/deserializer) electronics.

---

## References

[1] Winzer, P.J., Neilson, D.T., & Chraplyvy, A.R. (2018). "Fiber-optic transmission and networking: The previous 20 and the next 20 years." *Optics Express*, 26(18), 24190–24239. [Comprehensive review of fiber capacity history and projections; state-of-art capacity numbers.]

[2] Giovannetti, V., Guha, S., Lloyd, S., Maccone, L., Shapiro, J.H., & Yuen, H.P. (2004). "Classical capacity of the lossy bosonic channel: The exact solution." *Physical Review Letters*, 92(2), 027902. [Proof that coherent states achieve the Holevo capacity for the bosonic channel; quantum limit of optical communications.]
