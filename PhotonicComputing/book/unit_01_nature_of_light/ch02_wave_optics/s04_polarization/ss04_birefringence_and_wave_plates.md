# 2.4.4 — Birefringence and Wave Plates

## What Is Birefringence?

A material is *birefringent* if its refractive index depends on the polarization direction of the light passing through it. Birefringence arises from optical anisotropy: the electrons in the material respond differently to electric fields along different crystal axes. The result is that two orthogonal polarization components of the same wave travel at different speeds — they accumulate different phases as they propagate.

Birefringence is not a rare property. It occurs in:
- **Crystalline materials**: Calcite ($\Delta n \approx 0.17$), quartz ($\Delta n \approx 0.009$), lithium niobate ($\Delta n \approx 0.08$), KTP, BBO, and most non-cubic crystals.
- **Stress-induced birefringence**: Mechanical stress in glass or silicon changes the refractive index anisotropically (photoelastic effect).
- **Waveguide geometry**: Rectangular waveguides (like silicon nanowire waveguides) are geometrically birefringent because of their asymmetric cross-section, even if the material itself is isotropic.

## The Ordinary and Extraordinary Rays

In a uniaxial crystal (one unique optical axis), two types of refractive index apply:
- **Ordinary index $n_o$**: applies to light with electric field perpendicular to the optical axis; propagation speed $c/n_o$ regardless of propagation direction.
- **Extraordinary index $n_e$**: applies to light with electric field in the plane containing the optical axis and the propagation direction; propagation speed depends on angle.

For a wave propagating perpendicular to the optical axis:
- Ordinary ray: $n = n_o$
- Extraordinary ray: $n = n_e$

The birefringence is $\Delta n = n_e - n_o$. For calcite: $n_o = 1.658$, $n_e = 1.486$, $\Delta n = -0.172$ (negative uniaxial). For quartz: $n_o = 1.544$, $n_e = 1.553$, $\Delta n = +0.009$ (positive uniaxial).

## Phase Retardation

A birefringent plate of thickness $d$ introduces a phase difference (retardation) between the ordinary and extraordinary rays:

$$\Gamma = \frac{2\pi}{\lambda} (n_e - n_o) d = \frac{2\pi}{\lambda} \Delta n \cdot d$$

This is the phase difference accumulated between the two polarization components. The fast axis is the axis with the lower refractive index (faster propagation); the slow axis has the higher index.

## Wave Plates

A *wave plate* (or *retardation plate*) is a birefringent plate designed to introduce a specific phase retardation $\Gamma$ between the two polarization components. The two most important wave plates:

### Quarter-Wave Plate (QWP): $\Gamma = \pi/2$

Thickness: $d = \lambda/(4\Delta n)$.

**Effect on $H$-polarized light** (fast axis at $45°$): As derived in Section 2.4.2, a QWP with fast axis at $45°$ converts horizontal polarization to right circular polarization. More generally:
- Linear polarization at $45°$ to the axes → circular polarization
- Circular polarization → linear polarization
- Arbitrary input → elliptical polarization (in general)

**Application**: QWPs are used to convert between linear and circular polarization. In optical isolators, a QWP and Faraday rotator together create a non-reciprocal polarization rotation. In quantum optics experiments, QWPs prepare and analyze circular polarization states (qubits).

### Half-Wave Plate (HWP): $\Gamma = \pi$

Thickness: $d = \lambda/(2\Delta n)$.

**Effect**: A HWP reflects the polarization state through the fast axis direction. If the fast axis is at angle $\psi$ from horizontal, horizontal polarization is rotated to $2\psi$. More precisely:
- The HWP inverts the component perpendicular to the fast axis while leaving the parallel component unchanged.
- It transforms right circular polarization to left circular (and vice versa).
- It rotates linear polarization by $2\psi$.

**Application**: HWPs are polarization rotators. Rotating a HWP by angle $\psi$ rotates the polarization by $2\psi$ — a 2:1 mechanical-to-optical rotation ratio. This is used in the design of variable-ratio beam splitters (a HWP followed by a polarizing beam splitter splits in a ratio $\cos^2 2\psi : \sin^2 2\psi$).

## Lithium Niobate (LiNbO₃): The Modulator Material

Lithium niobate is the most important birefringent material in photonic computing. It is a *ferroelectric* crystal with a spontaneous electric polarization along the $c$-axis. Its relevant properties:
- Ordinary index $n_o = 2.21$ at 1550 nm; extraordinary $n_e = 2.14$.
- **Large Pockels coefficient** $r_{33} = 30.8$ pm/V (for $z$-propagating, $z$-polarized light) — one of the largest among common materials.
- **No free-carrier absorption** at telecom wavelengths (unlike silicon).
- Bandgap $\sim 4$ eV: transparent from 0.4 to 5.5 μm.

The Pockels effect (linear electro-optic effect): an applied electric field $E_z$ changes the extraordinary index by:

$$\Delta n_e = -\frac{n_e^3 r_{33}}{2} E_z$$

For a waveguide of length $L$ with voltage $V$ applied over electrode gap $d$: the phase shift is:

$$\Delta\phi = \frac{2\pi}{\lambda} \Delta n_e \cdot L = -\frac{\pi n_e^3 r_{33} L}{\lambda d} V$$

The *half-wave voltage* $V_\pi$ (voltage for $\pi$ phase shift):

$$V_\pi = \frac{\lambda d}{n_e^3 r_{33} L}$$

For $\lambda = 1550$ nm, $d = 10$ μm, $L = 1$ cm: $V_\pi = 1550 \times 10^{-9} \times 10 \times 10^{-6}/(2.14^3 \times 30.8 \times 10^{-12} \times 10^{-2}) \approx 1.1$ V. Thin-film lithium niobate on insulator (TFLN) waveguides achieve $V_\pi L < 2$ V·cm, enabling high-speed (100 GHz bandwidth) modulators with $V_\pi < 1$ V at centimeter scale [1, 2].

This is why TFLN is increasingly attractive for photonic computing: the Pockels effect is *linear* (phase proportional to voltage), *fast* (limited only by the RC time constant of the electrode, not by carrier dynamics as in silicon), and *low-energy* (no free-carrier injection/depletion required).

## Birefringence in Silicon Waveguides

Silicon itself is cubic (centrosymmetric) and has no birefringence in bulk. However, silicon waveguides are geometrically birefringent:
- A 450 nm × 220 nm Si wire waveguide has $n_\text{eff,TE} \approx 2.4$ and $n_\text{eff,TM} \approx 1.8$ at 1550 nm.
- This birefringence $\Delta n_\text{eff} \approx 0.6$ is *enormous* — much larger than any natural crystal birefringence.
- Applied stress (from the SiO₂ cladding or mechanical loading) can also induce bulk birefringence in the silicon core via the photoelastic effect.

For photonic computing circuits, this TE/TM birefringence means:
1. The two modes accumulate very different phases over the same length.
2. A circuit designed for TE polarization will behave completely differently for TM.
3. Any coupling between TE and TM (from bends, sidewall roughness, or wafer-scale stress variation) will scramble the polarization and introduce errors in interferometric computations.

**Solutions**: (a) operate in single polarization (TE only), using polarization splitters at the input; (b) use a polarization-transparent design where the circuit function is identical for TE and TM modes (rare, possible only for specific circuits); (c) actively control polarization with integrated polarization controllers (complex, power-hungry).

## Summary

- Birefringence: anisotropic refractive index; two orthogonal polarizations propagate at different speeds.
- Phase retardation $\Gamma = (2\pi/\lambda)\Delta n \cdot d$ accumulated between ordinary and extraordinary rays.
- QWP ($\Gamma = \pi/2$): converts between linear and circular polarization.
- HWP ($\Gamma = \pi$): rotates linear polarization by $2\psi$ (where $\psi$ is the fast axis angle).
- LiNbO₃: the modulator material; Pockels effect gives $\pi$ phase shift with $V_\pi \sim 1$–3 V at cm scale.
- Silicon waveguide birefringence $\Delta n_\text{eff} \approx 0.6$: requires single-polarization designs.

---

*References*

[1] Wang, C., Zhang, M., Chen, X., Bertrand, M., Shams-Ansari, A., Chandrasekhar, S., Winzer, P., & Lončar, M. (2018). Integrated lithium niobate electro-optic modulators operating at CMOS-compatible voltages. *Nature*, 562(7725), 101–104. [DOI: 10.1038/s41586-018-0551-y]

[2] He, M. et al. (2019). High-performance hybrid silicon and lithium niobate Mach–Zehnder modulators for 100 Gbit s⁻¹ and beyond. *Nature Photonics*, 13(5), 359–364. [DOI: 10.1038/s41566-019-0378-6]
