# Section 2.4: Polarization

Light is a transverse electromagnetic wave: the electric field oscillates perpendicular to the direction of propagation. In three dimensions, the propagation direction defines an axis, and the electric field vector can lie anywhere in the perpendicular plane. The *polarization state* of a plane wave is a specification of how the electric field vector moves as a function of time.

Polarization is not a subtlety or a complication. It is a fundamental degree of freedom of the electromagnetic field, as physical and consequential as frequency, amplitude, or phase. It has the following consequences for photonic computing:

1. **Polarization controls interference**: Two waves with orthogonal polarizations cannot interfere. In a photonic processor that relies on interference (all MZI-based processors), maintaining a consistent polarization state throughout the chip is a prerequisite for correct operation. Silicon photonic waveguides have different propagation constants for TE and TM modes; mode mixing causes phase errors. Most practical photonic chips are designed for single-polarization operation.

2. **Polarization doubles information capacity**: In principle, the two polarization modes of a waveguide can carry independent signals (polarization-division multiplexing, PDM). This doubles the information capacity without increasing bandwidth or power. PDM is standard in long-haul fiber communications and is beginning to appear in photonic computing architectures.

3. **Polarization as a computational resource**: In some architectures, the two polarization modes act as two independent computational channels processed in parallel by the same waveguide. In quantum photonic processors, polarization encodes qubits.

4. **Birefringence and phase control**: Anisotropic materials (LiNbO₃, calcite) have different refractive indices for different polarizations — birefringence. This is the basis of wave plates (which rotate polarization states), Pockels effect modulators, and polarization beam splitters. All of these are important components of photonic systems.

## Subsections

- **2.4.1 — States of Polarization**: Linear, circular, and elliptical polarization; the polarization ellipse.
- **2.4.2 — Jones Calculus**: Representing polarization states as 2-vectors and optical elements as 2×2 matrices; the Jones formalism for coherent optics.
- **2.4.3 — Stokes Parameters and the Poincaré Sphere**: The Stokes vector representation for partially polarized light; the Poincaré sphere geometry.
- **2.4.4 — Birefringence and Wave Plates**: Birefringent materials; quarter-wave and half-wave plates; polarization rotators; applications in photonic systems.
