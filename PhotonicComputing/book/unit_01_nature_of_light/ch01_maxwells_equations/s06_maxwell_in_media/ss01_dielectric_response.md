# 1.6.1 The Dielectric Response: From Microscopic to Macroscopic

## What Happens Inside a Dielectric

When an electromagnetic wave propagates through a dielectric material (glass, silicon, lithium niobate), the oscillating electric field of the wave exerts forces on the bound electrons in each atom. The electrons cannot escape (they are bound), but they are displaced slightly from their equilibrium positions, creating atomic electric dipole moments.

This is the phenomenon of **electric polarization**. Each atom is like a tiny spring-mass system: the nucleus is the heavy fixed mass, the electron cloud is the light displaced mass, and the restoring force from the nuclear charge plays the role of the spring. An applied electric field displaces the electron cloud, creating a dipole moment $\mathbf{p} = \alpha\mathbf{E}$, where $\alpha$ is the **polarizability** of the atom.

The macroscopic polarization density (dipole moment per unit volume) is:

$$\mathbf{P} = N\alpha\mathbf{E} = \varepsilon_0\chi_e\mathbf{E}$$

where $N$ is the number density of atoms and $\chi_e = N\alpha/\varepsilon_0$ is the **electric susceptibility**.

## Clausius-Mossotti Relation

The relationship between the microscopic polarizability $\alpha$ and the macroscopic dielectric constant $\varepsilon_r$ is not simply $\varepsilon_r = 1 + N\alpha/\varepsilon_0$. The field that acts on each atom is not the macroscopic field $\mathbf{E}$ but the **local field** — the field at the atom's location, which includes the contribution of all the other polarized atoms. For a simple cubic lattice or a liquid, the Clausius-Mossotti relation gives:

$$\frac{\varepsilon_r - 1}{\varepsilon_r + 2} = \frac{N\alpha}{3\varepsilon_0}$$

This relation is important in understanding why denser materials generally have higher refractive indices, and in the design of optical materials with specific refractive indices.

## Frequency Dependence: Dispersion

The polarizability $\alpha$ is not constant; it depends on the frequency of the applied field. At low frequencies, the electrons can follow the oscillation easily, and $\alpha$ is large. At high frequencies (near or above the resonant frequency of the electron-nucleus system), the electrons cannot follow, and $\alpha$ decreases.

The Lorentz oscillator model (developed in detail in Chapter 3) gives:

$$\alpha(\omega) = \frac{e^2/m_e}{\omega_0^2 - \omega^2 - i\gamma\omega}$$

where $\omega_0$ is the resonance frequency of the bound electron, $\gamma$ is the damping rate, and $e$, $m_e$ are the electron charge and mass.

The imaginary part of $\alpha$ leads to an imaginary part of $\chi_e$ and therefore an imaginary part of the refractive index — which corresponds to absorption.

**The key result for photonic computing**: Different materials are transparent at different wavelengths, determined by their electron resonances. Silicon has no electronic resonances in the near-infrared (its bandgap corresponds to $\lambda_g \approx 1127$ nm, and photons at 1550 nm have energy well below the bandgap), so it is transparent at 1550 nm and makes an excellent waveguide material at this wavelength. At 532 nm (green light), silicon strongly absorbs — which is why green laser pointers cannot be used to demonstrate silicon photonic waveguides.

## The Real Refractive Index: Physical Meaning

For a lossless dielectric at frequency $\omega$:

$$n(\omega) = \sqrt{\varepsilon_r(\omega)} = \sqrt{1 + \chi_e(\omega)}$$

The refractive index tells us:
1. **Phase velocity**: $v_p = c/n$ — light travels more slowly in the medium.
2. **Wavelength**: $\lambda = \lambda_0/n$ — the spatial period is compressed.
3. **Phase accumulation**: $\phi = (2\pi n/\lambda_0) L = kL$ per length $L$.
4. **Snell's law**: $n_1\sin\theta_1 = n_2\sin\theta_2$ at an interface.

The slowing of phase velocity in a medium is not because photons travel slower. It is because the electromagnetic wave is repeatedly absorbed and reemitted by the atoms in the medium, with a net effect that the wave pattern moves at $c/n < c$. Individual photon interactions occur at $c$; the macroscopic wave moves at $c/n$.

## Table of Refractive Indices for Photonic Computing Materials

| Material | $n$ at 1550 nm | Notes |
|----------|---------------|-------|
| Vacuum | 1.000 | Exact by definition |
| Air | 1.0003 | Essentially vacuum for optics |
| SiO₂ (silica glass) | 1.444 | Standard SMF-28 core; waveguide cladding |
| Si₃N₄ (silicon nitride) | 2.000 | Low-loss waveguide platform |
| Si (silicon) | 3.476 | High-confinement waveguide; SOI platform |
| LiNbO₃ (lithium niobate) | 2.21 (ordinary) | Electro-optic modulator platform |
| InP (indium phosphide) | 3.17 | III-V laser platform |
| Ge (germanium) | 4.28 | Photodetector material |

*These values are from standard references [1] and should be used in calculations throughout the book. Note that all values are frequency-dependent — the 1550 nm values are given here because that is the primary photonic computing wavelength.*

---

## References

[1] Palik, E.D. (Ed.) (1998). *Handbook of Optical Constants of Solids*. Academic Press. [The authoritative reference for optical constants of materials.]

[2] Born, M., & Wolf, E. (1999). *Principles of Optics*, 7th ed. Cambridge University Press. Ch. 2. [Comprehensive treatment of the electromagnetic theory of propagation in dielectrics.]
