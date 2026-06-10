# 1.6.3 Conducting Media, Skin Depth, and Plasmonic Materials

## Free Electrons in Conductors

In a metal or a doped semiconductor, there are free electrons (and/or holes) that can move in response to an applied electric field. These free carriers contribute a Drude term to the dielectric function:

$$\varepsilon_r(\omega) = \varepsilon_b - \frac{\omega_p^2}{\omega^2 + i\gamma\omega}$$

where $\varepsilon_b$ is the background dielectric constant from bound electrons, $\omega_p = \sqrt{Ne^2/(m^*\varepsilon_0)}$ is the **plasma frequency**, $N$ is the free carrier density, $m^*$ is the effective mass, and $\gamma$ is the carrier scattering rate.

For a good metal like gold at optical frequencies, $\omega \ll \omega_p$ (the light frequency is far below the plasma frequency), and $\varepsilon_r$ is predominantly negative and imaginary. This makes metals highly reflective at optical frequencies (the field cannot penetrate), and it is the basis of mirrors.

## Skin Depth

When an electromagnetic wave impinges on a conducting medium, it decays exponentially into the material. The characteristic decay length is the **skin depth**:

$$\delta = \sqrt{\frac{2}{\omega\mu_0\sigma}}$$

where $\sigma$ is the electrical conductivity. At optical frequencies ($\omega \sim 10^{15}$ rad/s) and for gold ($\sigma \approx 4.5\times10^7$ S/m):

$$\delta_{\text{Au}} = \sqrt{\frac{2}{10^{15}\times4\pi\times10^{-7}\times4.5\times10^7}} \approx 25\ \text{nm}$$

The field penetrates only about 25 nm into gold before being absorbed. This extremely short skin depth means that optical fields on a photonic chip are confined within tens of nanometers of metal surfaces — relevant for plasmonic waveguides and metallic electrodes.

## The Plasma Frequency and Why Metals Are Shiny

At frequencies well below $\omega_p$, $\varepsilon_r < 0$ — the dielectric function is negative — and electromagnetic waves are evanescent in the metal. The reflectivity at a metal-air interface approaches unity. This is why polished metals are shiny.

At frequencies above $\omega_p$, $\varepsilon_r > 0$ and metals become transparent (the electrons can no longer follow the oscillation). For gold, $\omega_p$ corresponds to a wavelength of about 200 nm (deep ultraviolet) — far above the visible range, which is why gold reflects all visible wavelengths equally well.

For doped silicon, $\omega_p$ depends on carrier density $N$: $\omega_p^2 = Ne^2/(m_e^*\varepsilon_0)$. For $N = 10^{18}$ cm$^{-3}$ (typical for a modulator depletion region), $\omega_p$ corresponds to roughly 50–100 μm wavelength — far below the 1550 nm operating wavelength. The free carriers in silicon therefore act as a perturbation on the dielectric constant at 1550 nm, rather than dominating it — which is why the Soref-Bennett equations treat carrier effects as small corrections to the silicon refractive index.

## Relevance to Photonic Computing: Metal Contacts and Electrodes

Silicon photonic modulators require metal electrodes to apply the electric field that changes the carrier density and thereby modulates the refractive index. These metal contacts must be placed close to the optical waveguide (to minimize the resistance of the path to the junction), but not so close that the evanescent field of the guided mode overlaps significantly with the metal and causes ohmic loss.

Typical silicon photonic designs place metal contacts 500 nm to 1 μm from the waveguide edge — far enough to keep the modal loss below 1–2 dB/cm, while close enough to keep the RC time constant small enough for high-speed operation [1].

## Plasmonics: Light Below the Diffraction Limit

At the interface between a metal (negative $\varepsilon_r$) and a dielectric (positive $\varepsilon_r$), a special mode called a **surface plasmon polariton (SPP)** can exist. The SPP is a coupled oscillation of the electromagnetic field and the surface charge density on the metal. Its wavevector:

$$k_{SP} = \frac{\omega}{c}\sqrt{\frac{\varepsilon_m\varepsilon_d}{\varepsilon_m + \varepsilon_d}}$$

is always larger than the free-space wavevector $\omega/c$, meaning the SPP is confined to a region smaller than the free-space wavelength — below the diffraction limit.

SPPs can be guided along metal strips, focused to nanometer-scale spots, and used to squeeze electromagnetic energy into volumes far smaller than $\lambda^3$. This sub-diffraction confinement is exploited in plasmonic modulators and sensors. However, the metal loss associated with SPPs limits the propagation length to tens of micrometers at 1550 nm, which constrains the scalability of plasmonic photonic circuits. We return to plasmonics in Chapter 8.

---

## References

[1] Reed, G.T., Mashanovich, G., Gardes, F.Y., & Thomson, D.J. (2010). "Silicon optical modulators." *Nature Photonics*, 4(8), 518–526. [Comprehensive review of silicon modulator design, including electrode placement considerations.]

[2] Maier, S.A. (2007). *Plasmonics: Fundamentals and Applications*. Springer. [Standard reference on surface plasmon polaritons and their properties.]
