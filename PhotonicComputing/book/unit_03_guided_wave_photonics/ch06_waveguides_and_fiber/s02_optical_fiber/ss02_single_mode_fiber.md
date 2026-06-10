# 6.2.2 Single-Mode Fiber: The SMF-28 Standard

## Why Single-Mode Matters

Multimode fiber supports many modes with different group velocities. A short pulse injected into a multimode fiber arrives at the far end spread in time — intermodal dispersion — because different modes travel at different speeds. For a 50 μm GRIN fiber at 850 nm ($V \approx 30$), the intermodal dispersion is approximately 0.5–2 ps/(nm·km), limiting the transmission product to ~2 Gbps·km. A 10 Gbps signal would be unrecognizable after 200 m.

Single-mode fiber eliminates intermodal dispersion by supporting only one mode. The only remaining dispersion is **intramodal** (chromatic) dispersion from the wavelength dependence of the group velocity. This is manageable and can be compensated. The transmission distance-bandwidth product of SMF is limited only by chromatic dispersion, nonlinearity, and loss — allowing terabit-per-second transmission over thousands of kilometers.

## The SMF-28 Fiber

Corning SMF-28 (and its ITU-T G.652 standard) is the most widely deployed optical fiber in history. Its specifications:

| Parameter | SMF-28 value |
|---|---|
| Core diameter | 8.2 μm |
| Cladding diameter | 125 μm |
| Core refractive index | 1.4677 at 1550 nm |
| Cladding refractive index | 1.4627 at 1550 nm |
| Numerical aperture | 0.14 |
| Mode field diameter (MFD) | 10.4 ± 0.8 μm at 1550 nm |
| Cutoff wavelength | < 1260 nm |
| Attenuation | ≤ 0.18 dB/km at 1550 nm |
| Chromatic dispersion | 17 ps/(nm·km) at 1550 nm |
| Dispersion slope | 0.090 ps/(nm²·km) |
| Nonlinear coefficient $\gamma$ | 1.3 W$^{-1}$km$^{-1}$ |
| Mode field area $A_{eff}$ | 85 μm² |

The **mode field diameter (MFD)** is the $1/e^2$ diameter of the Gaussian approximation to the LP$_{01}$ mode intensity profile. It is the quantity that determines coupling efficiency between two fibers or between a fiber and a chip (see Chapter 2, Section 2.6.4).

## The Fiber-to-Chip Coupling Challenge

The MFD mismatch between SMF-28 (10.4 μm) and a silicon photonic waveguide (0.45 × 0.22 μm, MFD ≈ 0.5 μm) is a factor of ~20 in diameter, corresponding to a factor of ~400 in mode area. Direct end-fire coupling would give catastrophic diffraction loss.

Two solutions (from Chapter 2, Section 2.6.4):
1. **Inverse taper**: Taper the Si waveguide tip to <100 nm width, where the silicon mode is cut off and the field expands into the SiO₂ cladding, increasing MFD to ~3–5 μm. Total coupling loss: < 2 dB.
2. **Grating coupler**: A periodic grating diffracts light from the fiber (coming from above at ~10° incidence) into the waveguide mode. Total coupling loss: < 1 dB for optimized grating couplers, but with ~30 nm bandwidth.

This coupling loss (1–2 dB per facet) is a significant overhead for any photonic computing system: a 32-port chip with 64 fiber connections loses 64 × 1.5 dB = 96 dB just in coupling — which cannot be the right architecture for large-scale systems. On-chip sources (Chapter 4.4.3) and on-chip detectors (Chapter 5.2.5) are the path to eliminating most of these fiber interfaces.
