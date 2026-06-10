# 3.4.3 — Brillouin Scattering

## Acoustic Phonons and Electrostriction

Brillouin scattering (Leon Brillouin, 1922 [1]) is inelastic scattering of light from *acoustic* phonons — long-wavelength sound waves in the material. Unlike Raman scattering (which involves optical phonons with frequencies $\sim 10^{13}$ Hz), Brillouin scattering involves acoustic phonons with frequencies $\sim 10^{10}$ Hz (10–20 GHz for optical fibers).

The physical mechanism: an intense optical wave exerts a radiation pressure force on the medium through *electrostriction* — the tendency of materials to compress in regions of high optical intensity. This creates a periodic density variation (acoustic wave) with a wavelength matching the optical interference pattern. The acoustic wave acts as a diffraction grating, scattering part of the forward-propagating light backward (backward SBS is dominant in fibers) with a Doppler shift $\Omega_B$ equal to the acoustic frequency.

The Brillouin frequency shift:

$$\Omega_B = \frac{2n v_a}{\lambda}$$

where $n$ is the refractive index, $v_a$ is the acoustic velocity, and $\lambda$ is the optical wavelength. For silica fiber at 1550 nm: $n = 1.445$, $v_a = 5960$ m/s:

$$\Omega_B = \frac{2 \times 1.445 \times 5960}{1550 \times 10^{-9}} \approx 11.1 \text{ GHz}$$

The Brillouin shift is around 10–11 GHz in standard silica fiber — in the microwave frequency range.

## Stimulated Brillouin Scattering (SBS)

Like Raman scattering, Brillouin scattering has a stimulated version. When the pump power exceeds a threshold, the back-scattered Stokes wave grows exponentially. The SBS threshold in a fiber of length $L$ and effective mode area $A_\text{eff}$:

$$P_\text{th} \approx \frac{21 A_\text{eff}}{g_B L_\text{eff}}$$

where $g_B \approx 5 \times 10^{-11}$ m/W is the peak Brillouin gain coefficient for silica and $L_\text{eff} = (1 - e^{-\alpha L})/\alpha$ is the effective length. For a 100 km fiber ($\alpha = 0.046$ km⁻¹, $L_\text{eff} \approx 22$ km) with $A_\text{eff} = 80$ μm²:

$$P_\text{th} = \frac{21 \times 80 \times 10^{-12}}{5 \times 10^{-11} \times 22 \times 10^3} \approx 1.5 \text{ mW}$$

**SBS is the lowest-threshold nonlinear effect in optical fiber** — just 1–10 mW of cw power is needed to reach SBS threshold in long fibers. Above threshold, the pump power is efficiently transferred to the backward-traveling Stokes wave, limiting forward transmission. SBS is therefore a key constraint on the maximum cw optical power that can be launched into a fiber link.

**Solutions for SBS suppression**:
1. Spectral broadening of the pump: SBS has a narrow gain bandwidth $\sim 20$ MHz. If the pump linewidth $> 20$ MHz, the SBS threshold increases (lower effective gain).
2. Strain or temperature gradients: vary the Brillouin shift along the fiber, reducing the effective interaction length.
3. Pulsed operation: SBS requires a buildup time $\sim L/v_a$ (acoustic wave formation); short pulses avoid SBS.

## Distributed Fiber Sensing via SBS

The dependence of the Brillouin shift $\Omega_B$ on both temperature and strain:

$$\frac{d\Omega_B}{dT} \approx +1.1 \text{ MHz/°C}, \quad \frac{d\Omega_B}{d\varepsilon} \approx +500 \text{ MHz/\%strain}$$

makes Brillouin scattering the basis of *distributed fiber sensors* — sensing the temperature and strain profile along the entire length of a fiber by analyzing the SBS spectrum as a function of position. The technique is *Brillouin optical time-domain analysis* (BOTDA) or *BOTDR*:

- A pump pulse is launched into one end of the fiber.
- At each point along the fiber, a small fraction of the pump energy is transferred to the Stokes wave via spontaneous Brillouin scattering.
- The Stokes wave returns to the input end, and its spectrum (peak frequency) encodes the local temperature/strain at the point of origin.
- Time-domain analysis (from the arrival time of the Stokes pulse) gives the spatial resolution: $\Delta z = v_g \tau/2$ (where $\tau$ is the pulse duration).

BOTDA systems can monitor the temperature and strain profile of tens of km of fiber with 1 m spatial resolution and $< 1°$C temperature accuracy. This is used for structural health monitoring of bridges, pipelines, railways, and large buildings — a major commercial application of fiber optics.

## Phonon-Photon Coupling and Optomechanics

Brillouin scattering is the classical manifestation of *photon-phonon coupling* — the interaction between light and mechanical vibrations. In high-Q optomechanical resonators (Chapter 1 reference: Aspelmeyer et al.), a similar coupling enables:
- Laser cooling of mechanical vibrations (approaching the quantum ground state)
- Sensing tiny forces and displacements
- Phonon manipulation for quantum information processing

For photonic computing, phonon-photon coupling is primarily a noise source (Brillouin and thermomechanical noise limit the precision of analog photonic computations) but also a potential resource for phonon-mediated all-optical processing.

## Brillouin Lasers and Narrow-Linewidth Sources

SBS can be exploited constructively: a Brillouin laser uses SBS gain in a cavity to generate an extremely narrow-linewidth laser output. Brillouin lasers have demonstrated linewidths of $< 1$ Hz [2] — the narrowest of any laser type. Such narrow-linewidth sources are useful for coherent optical communications (reduced phase noise), high-resolution spectroscopy, and precision sensing.

Integrated Brillouin lasers in microresonators have been demonstrated, opening the possibility of on-chip narrow-linewidth sources for photonic computing applications requiring high coherence [3].

## Summary

- Brillouin scattering: light scatters from acoustic phonons; frequency shift $\Omega_B = 2nv_a/\lambda \approx 11$ GHz in silica at 1550 nm.
- SBS threshold: $\sim 1$–10 mW in long fibers; limits maximum cw power in fiber links.
- Distributed sensing: BOTDA measures temperature and strain profile along entire fiber length.
- Brillouin lasers: sub-Hz linewidth; potential for on-chip narrow-linewidth sources.

---

*References*

[1] Brillouin, L. (1922). Diffusion de la lumière et des rayons X par un corps transparent homogène. *Annales de Physique*, 17, 88–122.

[2] Grudinin, I.S., Matsko, A.B., & Maleki, L. (2009). Brillouin lasing with a CaF₂ whispering gallery mode resonator. *Physical Review Letters*, 102(4), 043902. [DOI: 10.1103/PhysRevLett.102.043902]

[3] Otterstrom, N.T., Behunin, R.O., Kittlaus, E.A., Wang, Z., & Rakich, P.T. (2018). A silicon Brillouin laser. *Science*, 360(6393), 1113–1116. [DOI: 10.1126/science.aar6113]
