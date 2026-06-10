# 5.2.2 Avalanche Photodiodes (APDs)

## Impact Ionization: Internal Gain

A p-i-n photodiode converts one photon to one electron (approximately). An avalanche photodiode (APD) multiplies this: one photon creates one primary electron-hole pair, which then triggers an avalanche of additional carriers through impact ionization, producing a gain of $M = 10$–100 electrons per absorbed photon.

**Impact ionization**: When a carrier (electron or hole) in a high electric field accumulates enough kinetic energy (above the impact ionization threshold), it can collide with a valence band electron, promoting it to the conduction band and creating an additional electron-hole pair. This process cascades: each new carrier can ionize further, exponentially multiplying the original photocurrent.

**Structure**: APDs use a heterostructure design (separate absorption and multiplication regions, SAM) to achieve high electric field in the multiplication region (for gain) while maintaining moderate field in the absorption region (for good drift velocity and low dark current).

## Gain-Bandwidth Product Limitation

The avalanche process takes time: each ionization event adds to the total transit time. For a gain $M$, the extra transit time is approximately $M\tau_0$ where $\tau_0$ is the transit time of a single carrier across the multiplication region. The bandwidth of an APD is:

$$f_{APD} = \frac{f_{p-i-n}}{M}$$

at gains much larger than 1. The gain-bandwidth product (GBP) is a constant:

$$M \cdot f_{3\text{dB}} = \text{const} = \frac{v_s}{2\pi k d_M}$$

where $k = \alpha_h/\alpha_e$ is the ratio of hole to electron ionization coefficients and $d_M$ is the multiplication region thickness.

**Key values**:
- Si APD: $k \approx 0.01$ (electrons ionize much more readily than holes) → very low excess noise, GBP ~200–500 GHz but only operates at $\lambda < 1$ μm (Si bandgap)
- InGaAs/InP APD: $k \approx 0.4$ (both carriers ionize) → higher excess noise, GBP ~100–150 GHz, works at 1310/1550 nm
- InAlAs/InGaAs APD: $k \approx 0.1$ → improved noise, GBP ~200 GHz, newer technology

## Noise in APDs: Excess Noise Factor

The avalanche process is stochastic: different primary carriers produce different numbers of secondary carriers. This multiplication randomness adds **excess noise** with factor $F(M)$:

$$\langle i^2_{APD}\rangle = 2e I_{ph} M^2 F(M) B$$

$$F(M) \approx k M + (1-k)(2 - 1/M)$$

where $k$ is the ionization ratio. For $k = 0$ (electron-only ionization): $F = 2$. For $k = 1$ (equal ionization): $F = M$ (noise grows as fast as signal — no benefit). Silicon APDs with $k \approx 0.01$ have $F \approx 2$ at $M = 10$, giving nearly shot-noise-limited performance with 10× gain.

**When to use an APD**: APDs improve sensitivity when the dominant noise source is the transimpedance amplifier (TIA) thermal noise, not shot noise. The gain $M$ amplifies the signal before the TIA noise is added, improving the signal-to-noise ratio. Optimum gain minimizes total noise:

$$M_{opt} = \sqrt{\frac{4k_BT/R_L}{2e I_{dark} F k}}$$

For low dark current and high $k_BT/R_L$ (high thermal noise), $M_{opt}$ can be 5–20.

## APDs in Photonic Computing

APDs are not typically used in the output stages of analog photonic matrix processors, where the photocurrents are large enough (~0.1–10 mA) that thermal noise is not the dominant limitation. They are more relevant for:

1. **Long-reach optical links** (fiber to/from photonic computing chips): APD receivers extend the link budget by 5–10 dB compared to p-i-n receivers
2. **Quantum photonic computing output**: In measurement-based quantum computing, the detection of heralding photons requires near-unit efficiency — APDs and SPADs are used here
3. **LiDAR and ranging** (photonic computing applications in autonomous systems)
