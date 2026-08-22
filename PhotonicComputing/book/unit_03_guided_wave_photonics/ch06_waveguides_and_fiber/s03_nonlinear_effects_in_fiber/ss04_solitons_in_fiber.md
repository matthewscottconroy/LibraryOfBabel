# 6.3.4 Solitons in Fiber

The full treatment of optical solitons is given in Chapter 3, Section 3.3.5. Here we briefly summarize the fiber-specific parameters and the relevance to photonic data transmission and computing.

## Soliton Condition in SMF-28

For fundamental soliton propagation in SMF-28 at 1550 nm:
- $\beta_2 = -21.7$ ps²/km (anomalous dispersion)
- $\gamma = 1.3$ W$^{-1}$km$^{-1}$

Required peak power for a $T_0 = 1$ ps (FWHM 1.76 ps) soliton:

$$P_0 = \frac{|\beta_2|}{\gamma T_0^2} = \frac{21.7 \times 10^{-3}}{1.3 \times (10^{-12})^2} \approx 16.7 \text{ mW}$$

This is achievable with standard DFB laser output.

## Fiber Loss and the "Lossy Soliton"

Real fiber has loss $\alpha = 0.046$ km$^{-1}$. A soliton in lossy fiber loses energy as $P_0(z) = P_0 e^{-\alpha z}$, causing $T_0(z) \propto e^{+\alpha z/2}$ (soliton broadens as it loses energy, since $P_0 T_0^2 =$ const for a soliton). After one amplifier span ($L_{span} = 80$ km, $G = e^{\alpha L} = 37$ dB loss compensated by EDFA), the soliton has broadened by $e^{0.046 \times 80/2} = e^{1.84} \approx 6.3\times$. This is called the *adiabatic* limit, valid when $L_{span} \ll L_D$.

For picosecond solitons with $L_D = T_0^2/|\beta_2|$: for $T_0 = 10$ ps, $L_D = 4600$ km $\gg L_{span}$ — the adiabatic limit is excellent. For $T_0 = 1$ ps, $L_D = 46$ km $\approx L_{span}$ — no longer adiabatic; dispersion-managed solitons or other pulse formats are needed.

## Relevance to Photonic Computing

Optical solitons in fibers were proposed as information carriers for long-haul optical communications in the 1980s (Hasegawa & Tappert 1973, Mollenauer 1980). Modern coherent transmission systems use linear signaling formats (QAM) with DSP rather than solitons. However, soliton physics is relevant for:
1. **Fiber nonlinearity limits**: Understanding when SPM and anomalous dispersion create soliton-like compression vs. dispersive pulse spreading in WDM systems
2. **On-chip microresonator combs** (Chapter 4.4.4): Dissipative Kerr solitons in microresonators are the on-chip analogue of fiber solitons
3. **Ultrashort pulse propagation** in fiber-optic components used to inject signals into photonic computing chips
