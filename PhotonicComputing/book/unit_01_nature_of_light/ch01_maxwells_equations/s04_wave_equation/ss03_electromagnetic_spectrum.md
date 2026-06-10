# 1.4.3 The Electromagnetic Spectrum and Photonic Computing Wavelengths

## All Electromagnetic Waves Are the Same Physics

Gamma rays, X-rays, ultraviolet light, visible light, infrared radiation, microwaves, and radio waves are all electromagnetic waves — all solutions to the same wave equation, differing only in frequency. The full electromagnetic spectrum spans 24 orders of magnitude in frequency, from below 1 Hz (extremely low-frequency radio waves used for submarine communication) to above $10^{24}$ Hz (highest-energy gamma rays from cosmic sources).

The relationship between frequency, wavelength, and energy is:

$$c = f\lambda \quad \Rightarrow \quad \lambda = \frac{c}{f}$$

$$E_{\text{photon}} = hf = \frac{hc}{\lambda}$$

where $h = 6.626 \times 10^{-34}$ J·s is Planck's constant and $hc = 1.240\ \text{eV}\cdot\mu\text{m}$.

## The Telecom Windows: Why 1310 nm and 1550 nm?

Photonic computing systems almost universally operate in the infrared, particularly at 1310 nm or 1550 nm. The choice is not arbitrary — it is dictated by the properties of silica (SiO₂) optical fiber.

Silica fiber has an absorption spectrum that includes [1]:
- **OH-ion (water) absorption**: strong peaks near 950, 1240, and 1380 nm from residual water in the glass
- **Infrared absorption**: rising steeply above ~1700 nm due to phonon absorption
- **Rayleigh scattering**: $\alpha_R \propto \lambda^{-4}$, dominant at shorter wavelengths, arising from density fluctuations frozen in when the fiber solidified

The minimum loss in silica fiber occurs at approximately **1550 nm** (loss ~0.2 dB/km in standard SMF-28 fiber). This corresponds to a power attenuation of about 5% per kilometer — extraordinary transparency achieved after decades of fiber engineering [2].

A secondary low-loss window at **1310 nm** (~0.35 dB/km) corresponds to the zero-dispersion wavelength of standard fiber — where the group velocity dispersion $\beta_2 \approx 0$, making it useful for short-haul systems where dispersion but not loss is the limiting factor.

**For photonic computing on chip**: The choice of 1550 nm is also driven by:
- Mature laser technology (DFB lasers, VCSELs) at this wavelength
- Mature silicon photonic components designed for this wavelength
- Silicon is transparent at 1550 nm (below the bandgap energy $E_g \approx 1.1$ eV corresponds to $\lambda_g = hc/E_g \approx 1127$ nm) — important since silicon would absorb visible light entirely
- Germanium detectors with high bandwidth are available at this wavelength

## Photon Energy at Telecom Wavelengths

At 1550 nm:
$$E = \frac{hc}{\lambda} = \frac{1.240\ \text{eV}\cdot\mu\text{m}}{1.55\ \mu\text{m}} \approx 0.80\ \text{eV} = 1.28 \times 10^{-19}\ \text{J}$$

At 850 nm (VCSEL wavelength for short-reach interconnects):
$$E = \frac{1.240}{0.85} \approx 1.46\ \text{eV} = 2.34 \times 10^{-19}\ \text{J}$$

These energies are important for understanding the quantum noise floor of photonic computing: the minimum energy required to detect a single "bit" of information is one photon energy, $E_{\text{photon}}$. Current photonic computing systems require tens to thousands of photons per operation — far above the quantum limit — but understanding the quantum limit is important for evaluating the ultimate energy efficiency of photonic computation.

## The Optical Frequency

The optical frequency at 1550 nm is:
$$f = c/\lambda = 2.998 \times 10^8 / 1.55 \times 10^{-6} \approx 1.93 \times 10^{14}\ \text{Hz} \approx 193\ \text{THz}$$

This is the "carrier frequency" of optical signals. The bandwidth available in the telecom C-band (1530–1565 nm) is approximately:

$$\Delta f = c/\lambda_1^2 \cdot \Delta\lambda = \frac{3 \times 10^8}{(1.55 \times 10^{-6})^2} \times 35 \times 10^{-9} \approx 4.4 \times 10^{12}\ \text{Hz} = 4.4\ \text{THz}$$

This enormous bandwidth — 4.4 THz in just the C-band — is the fundamental reason why optical systems can carry so much more information than electronic systems. An electronic cable operating at, say, 100 GHz bandwidth uses a fraction of a percent of what a single optical fiber can support. WDM (wavelength-division multiplexing) exploits this by running many independent channels within this bandwidth simultaneously.

---

## References

[1] Miya, T., Terunuma, Y., Hosaka, T., & Miyashita, T. (1979). "Ultimate low-loss single-mode fibre at 1.55 µm." *Electronics Letters*, 15(4), 106–108. [The paper reporting 0.2 dB/km loss at 1550 nm.]

[2] Kao, C.K., & Hockham, G.A. (1966). "Dielectric-fibre surface waveguides for optical frequencies." *Proceedings of the Institution of Electrical Engineers*, 113(7), 1151–1158. [Kao and Hockham's foundational paper proposing the use of glass fiber for optical communication; Kao received the Nobel Prize in Physics in 2009 for this work.]
