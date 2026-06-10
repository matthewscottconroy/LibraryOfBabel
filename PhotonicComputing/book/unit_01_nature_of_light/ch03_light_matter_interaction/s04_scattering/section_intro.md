# Section 3.4: Scattering

Not all light-matter interaction involves absorption or coherent reemission. A third class of processes — scattering — involves the *redirection* of light by inhomogeneities in the medium, sometimes with a change in frequency (inelastic scattering) and sometimes without (elastic scattering). The distinction matters because elastic and inelastic scattering have very different physical mechanisms and applications.

Scattering is both a loss mechanism and a resource. In optical fibers, Rayleigh scattering from density fluctuations is the fundamental limit on transmission loss — the reason that 0.2 dB/km is the minimum achievable in silica fiber, and the reason that 1550 nm is the optimal wavelength. In contrast, Raman scattering and Brillouin scattering are exploited as gain mechanisms (Raman amplifiers, Brillouin fiber lasers) and sensing mechanisms (distributed fiber strain and temperature sensors).

For photonic computing systems, the scattering-related limits are:
- **Waveguide sidewall roughness scattering**: the dominant loss mechanism in silicon nanowire waveguides (2–3 dB/cm). This limits the maximum chip scale before signal levels become unacceptably low and requires optical amplification to compensate.
- **Rayleigh scattering in fiber**: limits the distance over which photonic signals can be transported without amplification.
- **Raman scattering in silicon**: the stimulated Raman effect can provide on-chip optical gain in silicon (silicon Raman amplifier/laser), which could address the on-chip amplification challenge — but requires pulsed operation or efficient carrier removal to overcome TPA limitations.

## Subsections

- **3.4.1 — Rayleigh Scattering**: Elastic scattering from density fluctuations; the $\lambda^{-4}$ dependence; why 1550 nm minimizes fiber loss.
- **3.4.2 — Raman Scattering**: Inelastic scattering involving phonon creation/annihilation; stimulated Raman amplification; silicon Raman lasers.
- **3.4.3 — Brillouin Scattering**: Scattering from acoustic phonons; stimulated Brillouin scattering (SBS); fiber sensors and SBS-limited optical power in waveguides.
