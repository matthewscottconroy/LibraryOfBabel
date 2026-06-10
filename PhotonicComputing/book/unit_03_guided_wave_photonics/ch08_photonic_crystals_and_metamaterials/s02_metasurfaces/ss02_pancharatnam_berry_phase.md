# 8.2.2 The Pancharatnam-Berry Phase

## Phase Without Resonance

Both mechanisms described in Section 8.2.1 (resonant phase and propagation phase) achieve phase control by tuning the electromagnetic response of each meta-atom — its resonance frequency or its effective optical path length. But there is a third mechanism that is qualitatively different: the **geometric phase**, also called the Pancharatnam-Berry phase.

The geometric phase is remarkable because it depends only on the *orientation* of the meta-atom, not on its electromagnetic resonance properties. Rotating an anisotropic meta-atom by an angle $\alpha$ around the normal axis gives a phase shift of $\pm 2\alpha$ to the two circular polarization components of the incident light. By continuously varying the rotation angle $\alpha(x, y)$ across the metasurface, any phase profile can be implemented — with no need to change the meta-atom size, shape, or material at all.

## Derivation from Polarization Optics

Consider an anisotropic meta-atom that acts as a half-wave plate (HWP): it converts left circular polarization (LCP, $|L\rangle$) to right circular polarization (RCP, $|R\rangle$) and vice versa. The Jones matrix for an ideal HWP at orientation angle $\alpha$ (angle between the fast axis and $x$-axis) is:

$$J_\alpha = \begin{pmatrix}\cos 2\alpha & \sin 2\alpha \\ \sin 2\alpha & -\cos 2\alpha\end{pmatrix}$$

Apply this to an incident right circular polarization state $|R\rangle = \frac{1}{\sqrt{2}}\begin{pmatrix}1 \\ -i\end{pmatrix}$:

$$J_\alpha |R\rangle = \begin{pmatrix}\cos 2\alpha & \sin 2\alpha \\ \sin 2\alpha & -\cos 2\alpha\end{pmatrix}\frac{1}{\sqrt{2}}\begin{pmatrix}1 \\ -i\end{pmatrix}$$

$$= \frac{1}{\sqrt{2}}\begin{pmatrix}\cos 2\alpha - i\sin 2\alpha \\ \sin 2\alpha + i\cos 2\alpha\end{pmatrix} = \frac{1}{\sqrt{2}}e^{-i2\alpha}\begin{pmatrix}1 \\ i\end{pmatrix} = e^{-i2\alpha}|L\rangle$$

The output is left circular polarization $|L\rangle$ with a **phase factor $e^{-i2\alpha}$** that depends only on the rotation angle $\alpha$ of the HWP.

Similarly, for incident left circular polarization $|L\rangle$:

$$J_\alpha |L\rangle = e^{+i2\alpha}|R\rangle$$

So:
- RCP incident → LCP transmitted with phase $e^{-i2\alpha}$ (phase decreases with $\alpha$)
- LCP incident → RCP transmitted with phase $e^{+i2\alpha}$ (phase increases with $\alpha$)

This is the **Pancharatnam-Berry phase**: rotating the half-wave plate by $\alpha$ introduces a phase of $\pm 2\alpha$ to circular polarization components. To cover the full $2\pi$ phase range, $\alpha$ need only span $[0, \pi]$ — a single rotation.

## Physical Origin: Parallel Transport on the Poincaré Sphere

The geometric interpretation is beautiful. The Poincaré sphere is a unit sphere in the space of polarization states, where:
- North pole = LCP
- South pole = RCP  
- Equator = linear polarizations
- A point on the sphere represents a general elliptical polarization

When a polarization state is transported around a closed loop on the Poincaré sphere — by passing through a sequence of wave plates — it acquires a geometric phase equal to *half the solid angle enclosed by the loop*:

$$\phi_{\text{Berry}} = -\frac{\Omega}{2}$$

where $\Omega$ is the solid angle subtended by the path at the center of the sphere [1].

For the HWP with varying orientation: starting at $|R\rangle$ (south pole), the HWP converts to $|L\rangle$ (north pole) via the equator at angle $2\alpha$. As $\alpha$ varies from 0 to $\pi$, the path on the equator goes around the sphere, and the enclosed solid angle goes from 0 to $4\pi$ (twice around), giving a phase from 0 to $-2\pi$. The $2\alpha$ phase in the above calculation is a reflection of this topological character.

This is "geometric phase" in the sense that it depends only on the path traced on the Poincaré sphere, not on the rate at which the path is traversed or the details of the interaction — only on the geometry of the polarization space.

## Advantages of Geometric Phase Metasurfaces

The Pancharatnam-Berry (PB) mechanism has several advantages over resonant phase metasurfaces:

1. **Broadband operation**: The geometric phase depends only on the rotation angle $\alpha$, which is a purely geometric property independent of wavelength. A PB metasurface designed for 1550 nm can work (with some efficiency variation) over a bandwidth of hundreds of nanometers, unlike a resonant metasurface whose phase profile shifts with wavelength.

2. **Single etch depth**: All meta-atoms have the same geometry (same size, same shape, same height) and differ only in rotation. This makes fabrication dramatically simpler: a single etch step produces the entire phase profile.

3. **High efficiency**: Because all meta-atoms have the same resonance, the transmission efficiency is uniform across the surface. A resonant metasurface with varying geometry has position-dependent efficiency, causing amplitude modulation in addition to the desired phase modulation.

4. **Polarization multiplexing**: The opposite phase for LCP and RCP ($+2\alpha$ vs. $-2\alpha$) allows two different optical functions to be encoded in a single layer — one for each circular polarization. This doubles the information density of the metasurface.

The main limitation is that PB phase requires circular polarization input and converts input polarization (RCP → LCP or vice versa). If the application requires linear polarization (as is often the case in silicon photonics, where TE and TM modes are well-defined), a PB metasurface requires polarization conversion optics before and after.

## PB Phase Metasurfaces at 1550 nm

Several demonstrations of PB metasurfaces at 1550 nm have been reported. A representative design uses silicon rectangles (aspect ratio ~3:1, e.g., 250 nm × 80 nm) with height 500 nm on glass, operating as HWPs through the dielectric resonance of the silicon antenna:

- Target efficiency: 95% (ideal HWP converts all power from RCP to LCP with phase $2\alpha$)
- Measured transmission efficiency: 70–85% [2]
- Phase controllability: Full $2\pi$ range by varying $\alpha$ from 0 to $\pi$
- Bandwidth: ±200 nm from design wavelength for >80% efficiency

These metasurfaces have been used to demonstrate flat metalenses, beam splitters, holograms, and vortex beam generators — the same optical functions as conventional optical elements but in a 500-nm-thick layer.

For photonic computing, the key application is spatial optical processing: implementing a complex-valued transmission matrix $T(x, y)$ in a single flat layer. The next subsection examines this in the context of diffractive optical computing.

---

## References

[1] Berry, M.V. (1984). "Quantal phase factors accompanying adiabatic changes." *Proceedings of the Royal Society A*, 392(1802), 45–57. [The foundational paper on geometric phase in quantum mechanics; the optical version (Pancharatnam-Berry phase) is a direct classical analog.]

[2] Lin, D., Fan, P., Hasman, E., & Brongersma, M.L. (2014). "Dielectric gradient metasurface optical elements." *Science*, 345(6194), 298–302. [High-efficiency dielectric (Si) PB metasurface at telecom wavelengths; establishes the design methodology for orientation-based phase control.]
