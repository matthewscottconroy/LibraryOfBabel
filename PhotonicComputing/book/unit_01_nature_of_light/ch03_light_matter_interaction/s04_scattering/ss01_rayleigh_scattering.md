# 3.4.1 — Rayleigh Scattering

## Physical Mechanism

Rayleigh scattering is elastic scattering of light from density fluctuations in a medium, or from particles much smaller than the wavelength ($a \ll \lambda$). It was explained by Lord Rayleigh in 1871 [1].

The mechanism: a small particle (or density fluctuation) has a slightly different polarizability than the surrounding medium. The incident wave drives an oscillating dipole in the particle, which reradiates in all directions. The reradiated power depends on the frequency of oscillation: a higher-frequency oscillation radiates more power (this is the Larmor formula for radiated power: $P \propto \ddot{p}^2 \propto \omega^4 p^2$).

The key result: the scattering cross-section scales as $\lambda^{-4}$ (or $\omega^4$). Short-wavelength (blue) light scatters much more than long-wavelength (red) light. This is why:
- The sky is blue: sunlight scattered by air molecules is dominated by the short-wavelength (blue) end of the visible spectrum.
- Sunsets are red: when sunlight travels through a long atmospheric path at the horizon, all the blue light has been scattered away, leaving only the red.

## Rayleigh Scattering Loss in Silica Fiber

In silica glass, the dominant Rayleigh scattering mechanism is thermodynamic density fluctuations frozen in during the glass solidification. These fluctuations have a characteristic length scale much smaller than the optical wavelength, so the $\lambda^{-4}$ Rayleigh formula applies.

The scattering loss coefficient:

$$\alpha_R = A/\lambda^4$$

where $A$ is a material-dependent constant. For fused silica: $A \approx 0.80$ dB/km·μm⁴. At $\lambda = 1550$ nm = 1.55 μm:

$$\alpha_R = 0.80/(1.55)^4 \approx 0.80/5.77 \approx 0.139 \text{ dB/km}$$

The total measured loss of state-of-the-art single-mode fiber at 1550 nm is $\sim 0.14$–0.20 dB/km, of which Rayleigh scattering contributes $\sim 0.14$ dB/km and absorption by residual OH impurities and multi-phonon absorption contributes the rest. This is essentially the theoretical minimum for silica fiber.

**Why 1550 nm?** At shorter wavelengths, Rayleigh scattering increases rapidly ($\propto \lambda^{-4}$). At longer wavelengths (beyond $\sim 1600$ nm), multi-phonon absorption of the Si-O stretching mode in silica increases. The loss minimum — the transmission window — is around 1550 nm. This is why 1550 nm (the C-band, 1530–1565 nm) is the dominant wavelength for long-haul fiber communications and, by extension, for photonic computing.

The Miya et al. 1979 paper [2] reported the first fiber with loss below 0.2 dB/km at 1550 nm, establishing 1550 nm as the standard telecom wavelength.

## Waveguide Scattering Loss

In silicon nanowire waveguides, the dominant loss mechanism is scattering from sidewall roughness — not bulk Rayleigh scattering, which is negligible in crystalline silicon. The rough sidewalls create random perturbations to the waveguide mode, causing coupling to radiation modes (light leaks out).

The sidewall roughness scattering loss scales approximately as $\alpha_\text{roughness} \propto \sigma^2 L_c / (A_\text{eff} \lambda^4)$ where $\sigma$ is the roughness amplitude (typical: 1–5 nm for current silicon photonics processes), $L_c$ is the roughness correlation length, and $A_\text{eff}$ is the mode area. The $\lambda^{-4}$ scaling applies here too — shorter wavelengths lose more to sidewall roughness.

State-of-the-art silicon waveguides achieve $\sim 2$–3 dB/cm at 1550 nm. This translates to $10^3 \times$ more loss per unit length than single-mode fiber — a critical limitation for large-scale photonic computing circuits. Strategies to reduce scattering loss:
1. **Smoother lithography**: deep-UV and EUV lithography reduce roughness to $< 1$ nm.
2. **Wider waveguides**: larger mode area reduces field intensity at the rough sidewalls, reducing coupling to radiation.
3. **Thermal oxidation**: growing a thin SiO₂ layer smooths the Si surface.
4. **Alternative materials**: Si₃N₄ has lower-contrast walls and achieves $0.1$ dB/m — 300× lower loss than Si at 1550 nm.

## The Tradeoff: Confinement vs. Loss

High index contrast (silicon's $\Delta n \approx 2$) enables tight confinement (small mode area, tight bends, dense integration) but at the cost of increased sensitivity to roughness. Low index contrast (Si₃N₄, $\Delta n \approx 0.5$; polymer, $\Delta n \approx 0.1$) gives lower loss but requires larger waveguide cross-sections and gentler bends.

For photonic computing, this is a fundamental engineering tradeoff:
- High-density integration (more neurons per chip) → high contrast waveguides → higher loss per element → more amplification needed.
- Low-loss propagation → low contrast waveguides → lower density → fewer neurons per chip.

The optimal choice depends on the application: for deep learning inference with many neural network layers, low loss is paramount; for short, dense switching circuits, high confinement may be preferred.

## Summary

- Rayleigh scattering: $\alpha \propto \lambda^{-4}$; elastic scattering from sub-wavelength density fluctuations.
- Silica fiber at 1550 nm: $\alpha_R \approx 0.14$ dB/km — essentially the physical minimum.
- Silicon waveguides: dominated by sidewall roughness scattering, $\sim 2$–3 dB/cm.
- Fundamental tradeoff: high index contrast (tight confinement, small footprint) ↔ higher scattering loss.

---

*References*

[1] Rayleigh, Lord (1871). On the light from the sky, its polarization and colour. *Philosophical Magazine*, 41, 107–120, 274–279.

[2] Miya, T., Terunuma, Y., Hosaka, T., & Miyashita, T. (1979). Ultimate low-loss single-mode fibre at 1.55 μm. *Electronics Letters*, 15(4), 106–108. [DOI: 10.1049/el:19790077]
