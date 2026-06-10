# 6.2.3 Attenuation Mechanisms

## The Loss Budget of Silica Fiber

The total attenuation of optical fiber is the sum of several physical mechanisms, each with a distinct wavelength dependence:

$$\alpha(\lambda) = \alpha_{Rayleigh}(\lambda) + \alpha_{IR}(\lambda) + \alpha_{UV}(\lambda) + \alpha_{OH}(\lambda) + \alpha_{scat,imp}(\lambda)$$

Understanding each contribution explains why the minimum-loss window is at 1550 nm and why this window is approximately 30–40 nm wide.

## Rayleigh Scattering

As derived in Chapter 3 (Section 3.4.1), Rayleigh scattering from density fluctuations frozen in at the glass transition temperature gives:

$$\alpha_{Rayleigh} = \frac{A}{\lambda^4}$$

with $A \approx 0.78$ dB·km$^{-1}$·μm$^4$ for standard silica. At 1550 nm: $\alpha_{Rayleigh} = 0.78/1.55^4 = 0.137$ dB/km.

Rayleigh scattering is fundamental — it cannot be reduced by purification, only by wavelength choice (higher wavelength) or material selection. The $\lambda^{-4}$ dependence makes Rayleigh the dominant loss at short wavelengths.

## Infrared Absorption

Silica has Si-O vibrational absorption bands in the infrared. The fundamental stretching mode is at ~9 μm; the overtones and combination bands extend into the telecom wavelength range:
- Third overtone: ~1.7 μm (begins to matter above ~1650 nm)
- Combined: exponentially increasing absorption above ~1700 nm

This IR absorption tail is the long-wavelength boundary of the silica transparency window. It rises steeply from ~0.001 dB/km at 1550 nm to ~0.1 dB/km at 1700 nm and much higher beyond.

## OH (Water) Absorption

Hydroxyl (OH) impurities in silica create absorption peaks:
- 1383 nm peak: ~2 dB/km in standard fiber; ~0.001 dB/km in low-water-peak fiber
- 950 nm, 1240 nm: smaller peaks
- 725 nm: smaller still

The 1383 nm OH peak splits the silica transmission window into the 1310 nm (O-band) and 1550 nm (C-band) windows. Modern low-water-peak fiber (ITU-T G.652.D, "E-band fiber") reduces the OH peak to < 0.4 dB/km, opening the E-band (1360–1460 nm) for WDM.

## The Loss Minimum and Its Significance

The minimum loss occurs where $d\alpha/d\lambda = 0$:

$$\frac{d\alpha_{Rayleigh}}{d\lambda} + \frac{d\alpha_{IR}}{d\lambda} = 0$$

This balance occurs near 1570 nm for standard silica, giving $\alpha_{min} \approx 0.18$ dB/km. The minimum is determined by fundamental physics (quantum mechanics of photon scattering and molecular vibration), not engineering. No amount of fiber engineering can move it significantly.

**Historical significance**: When Charles Kao proposed in 1966 that ultrapure glass fibers could achieve < 20 dB/km loss [1], the best available glass had loss > 1000 dB/km. Kao's prediction required the loss to be dominated by Rayleigh scattering (not impurity absorption), which implied that dramatic reduction was possible by purification. Corning achieved 20 dB/km in 1970 (Kapron, Keck, Maurer), and 0.18 dB/km was reached in 1979 (Miya et al.) [2]. The minimum was physics-limited.

**Photonic computing implication**: Off-chip optical interconnects (chip-to-chip or board-to-board) operate at distances of 1 cm to 1 m. At 1550 nm with 0.18 dB/km fiber loss, a 1 m link loses $1.8 \times 10^{-4}$ dB — negligible. The dominant losses are at the fiber-chip interfaces (1–2 dB each) and in the photonic chip itself (waveguide loss, device insertion loss). Fiber propagation loss is simply not a design constraint for chip-scale photonic computing; it only matters at distances > 1 km.

## References

[1] Kao, K.C., & Hockham, G.A. (1966). "Dielectric-fibre surface waveguides for optical frequencies." *Proceedings of the IEE*, 113(7), 1151–1158.
[2] Miya, T., Terunuma, Y., Hosaka, T., & Miyashita, T. (1979). "Ultimate low-loss single-mode fibre at 1.55 μm." *Electronics Letters*, 15(4), 106–108.
