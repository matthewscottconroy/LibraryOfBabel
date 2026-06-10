# 1.5.3 Angular Momentum of Light: Spin and Orbital

## Two Kinds of Angular Momentum

Electromagnetic waves can carry angular momentum — a physical quantity that can be transferred to matter and measurable in the laboratory. There are two types:

1. **Spin angular momentum (SAM)**: Associated with the circular polarization of the wave.
2. **Orbital angular momentum (OAM)**: Associated with the spatial phase structure (helical wavefronts) of the beam.

## Spin Angular Momentum

A right-circularly polarized plane wave carries spin angular momentum $+\hbar$ per photon; left-circular carries $-\hbar$ per photon. This is a quantum result (the photon is a spin-1 particle), but the classical field theory of SAM is consistent with it:

For a circularly polarized plane wave with intensity $I$, the time-averaged spin angular momentum density is $\sigma_{\pm} \hbar / (hf)$ per unit volume, where $\sigma_\pm = \pm 1$ for right/left circular polarization.

Beth (1936) measured the torque exerted on a birefringent plate by circularly polarized light, confirming that each photon carries $\pm\hbar$ of spin angular momentum [1].

**Relevance**: In polarization-encoding photonic processors, the two polarization states can encode one bit of information. In quantum photonics, circular polarization states $|\sigma^+\rangle$ and $|\sigma^-\rangle$ form the natural qubit basis for polarization-encoded qubits.

## Orbital Angular Momentum

In 1992, Allen et al. showed that laser beams with a helical phase structure $e^{i\ell\phi}$ (where $\phi$ is the azimuthal angle) carry orbital angular momentum $\ell\hbar$ per photon, where $\ell$ is any integer [2].

These **Laguerre-Gaussian (LG) beams** have a helical wavefront that winds $\ell$ times around the beam axis per wavelength. The beam has a phase singularity (vortex) at the center, so the intensity is zero on axis — the beam looks like a "doughnut."

The OAM quantum number $\ell$ can in principle take any integer value, providing an infinite-dimensional state space per photon — in contrast to polarization (spin), which is two-dimensional. This has prompted interest in using OAM states for high-dimensional quantum information encoding [3] and for optical communications by multiplexing OAM modes (though the practical advantages over WDM are still debated) [4].

**For photonic computing**: OAM beams are being explored as a basis for free-space optical computing beyond the scalar diffraction limit, and as carriers for high-dimensional quantum information in quantum photonic processors. The engineering challenges are significant — maintaining OAM coherence over a chip and efficiently coupling OAM modes into waveguides are active research problems.

---

## References

[1] Beth, R.A. (1936). "Mechanical detection and measurement of the angular momentum of light." *Physical Review*, 50(2), 115–125. [The first measurement of optical angular momentum.]

[2] Allen, L., Beijersbergen, M.W., Spreeuw, R.J.C., & Woerdman, J.P. (1992). "Orbital angular momentum of light and the transformation of Laguerre-Gaussian laser modes." *Physical Review A*, 45(11), 8185–8189. [The foundational paper on OAM of light.]

[3] Franke-Arnold, S., Barnett, S.M., Padgett, M.J., & Allen, L. (2002). "Two-photon entanglement of orbital angular momentum states." *Physical Review A*, 65(3), 033823.

[4] Wang, J., et al. (2012). "Terabit free-space data transmission employing orbital angular momentum multiplexing." *Nature Photonics*, 6(7), 488–496.
