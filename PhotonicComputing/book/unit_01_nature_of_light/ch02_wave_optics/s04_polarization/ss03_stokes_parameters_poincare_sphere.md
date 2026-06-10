# 2.4.3 — Stokes Parameters and the Poincaré Sphere

## Motivation: Partially Polarized Light

The Jones calculus describes *fully polarized*, *coherent* light. Real light sources — LEDs, thermal emitters, broadband lasers — are often *partially polarized*: the polarization state fluctuates randomly over time, so only a fraction of the intensity is in a definite polarization state.

For partially polarized light, the Jones vector at any instant is $\mathbf{J}(t) = (E_x(t), E_y(t))^T$, where $E_x(t)$ and $E_y(t)$ are random processes. The observable quantities are time-averaged intensities and correlations of the field components — not the instantaneous field values.

The *Stokes parameters* provide a complete description of the polarization state (including partial polarization) in terms of measurable intensity quantities.

## The Stokes Parameters

The four Stokes parameters $S_0, S_1, S_2, S_3$ are defined as:

$$S_0 = \langle |E_x|^2 \rangle + \langle |E_y|^2 \rangle = I_x + I_y$$

$$S_1 = \langle |E_x|^2 \rangle - \langle |E_y|^2 \rangle = I_x - I_y$$

$$S_2 = 2\text{Re}\langle E_x E_y^* \rangle = I_{+45°} - I_{-45°}$$

$$S_3 = 2\text{Im}\langle E_x E_y^* \rangle = I_R - I_L$$

where:
- $S_0$ is the total intensity
- $S_1$ is the preference for horizontal vs. vertical linear polarization
- $S_2$ is the preference for $+45°$ vs. $-45°$ linear polarization
- $S_3$ is the preference for right vs. left circular polarization
- $I_{+45°}$ and $I_{-45°}$ are intensities after projection onto $\pm 45°$ polarizers
- $I_R$ and $I_L$ are intensities after projection onto circular polarization states

**Measurability**: All four Stokes parameters can be measured directly using polarizers and wave plates — they are real, measurable intensities, not complex amplitudes. This is why the Stokes description is useful for partially polarized light, where phase information is lost in the time average.

**Degree of polarization (DOP)**:

$$\text{DOP} = \frac{\sqrt{S_1^2 + S_2^2 + S_3^2}}{S_0} \in [0, 1]$$

DOP = 1: fully polarized (pure polarization state). DOP = 0: completely unpolarized (incoherent mixture of all polarization states equally). $0 < \text{DOP} < 1$: partially polarized.

For fully polarized light: $S_0^2 = S_1^2 + S_2^2 + S_3^2$ — the Stokes vector lies on a sphere.

## The Poincaré Sphere

The normalized Stokes parameters $(S_1/S_0, S_2/S_0, S_3/S_0)$ define a point on or inside the unit sphere — the *Poincaré sphere* [1].

**Key points on the Poincaré sphere**:

| Polarization state | $(S_1, S_2, S_3)/S_0$ | Location |
|-------------------|----------------------|----------|
| Horizontal linear | $(+1, 0, 0)$ | North pole of equator... no: right point |
| Vertical linear | $(-1, 0, 0)$ | Left point |
| $+45°$ linear | $(0, +1, 0)$ | Front point |
| $-45°$ linear | $(0, -1, 0)$ | Back point |
| Right circular | $(0, 0, +1)$ | North pole |
| Left circular | $(0, 0, -1)$ | South pole |

All linear polarizations lie on the equator of the Poincaré sphere. The poles are circular polarizations. Elliptical polarizations are intermediate points.

**The geometrical meaning**: Every unitary polarization transformation corresponds to a rotation of the Poincaré sphere. A half-wave plate with fast axis at angle $\psi$ corresponds to a rotation of the sphere by $2\times 2\psi = 4\psi$ about the axis through the $H$ and $V$ poles (i.e., the $S_1$ axis). A quarter-wave plate at $\psi = 0$ corresponds to a rotation of $\pi/2$ about $S_1$.

This geometric picture is powerful: the effect of any sequence of wave plates on any polarization state can be visualized as a sequence of rotations of the Poincaré sphere. Design of polarization control networks is equivalent to finding a sequence of rotations that maps one point on the sphere to another.

**Connection to quantum mechanics**: The Poincaré sphere is isomorphic to the Bloch sphere of a qubit. The Bloch sphere representation of a single qubit maps $|0\rangle \to$ north pole, $|1\rangle \to$ south pole, and superpositions to other points on the sphere. The polarization states $|H\rangle$, $|V\rangle$, $|D\rangle$, $|A\rangle$, $|R\rangle$, $|L\rangle$ map to the six faces of the cube inscribed in the Bloch/Poincaré sphere. This isomorphism is not accidental — it reflects the identical mathematical structure (2D complex Hilbert space) of both polarization and spin-1/2 systems.

## The Mueller Matrix

For partially polarized light, Jones matrices cannot be used (they apply to coherent fields). Instead, optical elements are described by $4 \times 4$ real *Mueller matrices* $\mathsf{M}$ acting on the Stokes vector $\mathbf{S} = (S_0, S_1, S_2, S_3)^T$:

$$\mathbf{S}_\text{out} = \mathsf{M} \cdot \mathbf{S}_\text{in}$$

Mueller matrices can describe polarizers, wave plates, depolarizers, and partially polarizing elements. They are determined experimentally by sending known Stokes vectors through the element and measuring the output.

**Relation to Jones matrices**: For a fully polarizing, non-depolarizing element with Jones matrix $\mathsf{J}$, the corresponding Mueller matrix is:

$$M_{ij} = \frac{1}{2}\text{Tr}(\sigma_i \mathsf{J} \sigma_j \mathsf{J}^\dagger)$$

where $\sigma_i$ are the Pauli matrices ($\sigma_0 = \mathsf{I}$, $\sigma_1 = \sigma_x$, $\sigma_2 = \sigma_y$, $\sigma_3 = \sigma_z$). This formula is the bridge between the two descriptions.

## Application to Photonic Computing: Polarization Drift

In silicon photonic circuits, fabrication variations and temperature changes cause the TE/TM effective indices to drift over time. This changes the polarization state at every point in the circuit in an uncontrolled way. For polarization-sensitive operations (like polarization-encoded qubits, or polarization-multiplexed signals), this drift must be compensated by active polarization controllers.

Polarization controllers using thermo-optic phase shifters can implement arbitrary SU(2) rotations on the polarization state of a guided mode. Such controllers have been demonstrated with $< 1°$ precision on integrated photonic platforms [2]. The design uses the Poincaré sphere picture: knowing the current polarization state (from a monitor photodetector with polarizers) and the target state, the rotation needed can be computed and applied.

## Summary

- Stokes parameters $(S_0, S_1, S_2, S_3)$: intensity-based description of polarization, valid for partially polarized light.
- Degree of polarization DOP $= \sqrt{S_1^2 + S_2^2 + S_3^2}/S_0 \in [0,1]$.
- Poincaré sphere: geometric picture of polarization state space; unitary transformations are rotations.
- Mueller matrix: $4 \times 4$ real matrix for partially polarized light; bridges Jones and Stokes formalisms.
- Active polarization control is required in photonic chips for polarization-sensitive applications.

---

*References*

[1] Poincaré, H. (1892). *Théorie Mathématique de la Lumière*, Vol. 2. Georges Carré, Paris. [Introduction of the Poincaré sphere representation.]

[2] Bandyopadhyay, S. et al. (2021). Single chip photonic deep neural network with accelerated training. *arXiv:2208.01623*. [Demonstrates active polarization control on photonic chip.]
