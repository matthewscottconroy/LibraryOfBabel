# 2.4.1 — States of Polarization

## The Polarization of a Plane Wave

For a plane wave propagating in the $+z$ direction, the electric field is confined to the $x$-$y$ plane. The most general form is:

$$\mathbf{E}(z,t) = \hat{x} E_x \cos(\omega t - kz + \phi_x) + \hat{y} E_y \cos(\omega t - kz + \phi_y)$$

where $E_x$, $E_y$ are real amplitudes and $\phi_x$, $\phi_y$ are phases. The polarization state is determined by the ratio $E_y/E_x$ and the relative phase $\delta = \phi_y - \phi_x$.

**In complex notation** (more compact):

$$\mathbf{E}(z,t) = \text{Re}\left[(\hat{x} E_x e^{i\phi_x} + \hat{y} E_y e^{i\phi_y}) e^{i(kz - \omega t)}\right]$$

The complex vector $\mathbf{E}_0 = \hat{x} E_x e^{i\phi_x} + \hat{y} E_y e^{i\phi_y}$ fully characterizes the polarization state (up to an overall phase and magnitude).

## Linear Polarization

When $\delta = \phi_y - \phi_x = 0$ or $\pi$: the two components oscillate in phase (or exactly out of phase). The electric field tip traces a straight line in the $x$-$y$ plane at angle $\psi = \arctan(E_y/E_x)$ to the $x$-axis.

- **$x$-polarized (horizontal)**: $E_y = 0$. The field is $E_x\cos(\omega t - kz)\hat{x}$.
- **$y$-polarized (vertical)**: $E_x = 0$.
- **45° polarized**: $E_x = E_y$, $\delta = 0$.

Linearly polarized light can be produced by a polarizer (which transmits one linear polarization and blocks the orthogonal one), by reflection at Brewster's angle (Section 2.1.2), or by passing through a birefringent crystal.

## Circular Polarization

When $E_x = E_y = E_0$ and $\delta = \pm\pi/2$: the field components have equal amplitude but are $90°$ out of phase. The electric field tip traces a circle:

$$\mathbf{E}(0,t) = E_0[\cos(\omega t)\hat{x} \pm \sin(\omega t)\hat{y}]$$

- **Left circular polarization (LCP)** ($\delta = +\pi/2$): field rotates counterclockwise when viewed by the observer facing the incoming wave.
- **Right circular polarization (RCP)** ($\delta = -\pi/2$): clockwise rotation.

Circular polarization carries spin angular momentum $\pm\hbar$ per photon (Section 1.5.3). LCP and RCP are eigenstates of the angular momentum operator. Any linearly polarized wave is a superposition of equal LCP and RCP components.

**Circular polarization basis**: It is sometimes convenient to work in the circular polarization basis $(\hat{e}_+, \hat{e}_-)$ rather than the linear basis $(\hat{x}, \hat{y})$:

$$\hat{e}_\pm = \frac{1}{\sqrt{2}}(\hat{x} \pm i\hat{y})$$

In this basis, $\hat{x} = (\hat{e}_+ + \hat{e}_-)/\sqrt{2}$ and $\hat{y} = (\hat{e}_+ - \hat{e}_-)/(i\sqrt{2})$. The circular basis is particularly natural in quantum optics (Unit VII), where $\hat{e}_+$ and $\hat{e}_-$ are associated with photons of helicity $+1$ and $-1$.

## Elliptical Polarization: The General Case

For arbitrary $E_x$, $E_y$, $\delta$, the electric field tip traces an ellipse in the $x$-$y$ plane — *elliptical polarization*. The ellipse is characterized by:
- **Orientation angle** $\psi$: angle of the major axis with respect to the $x$-axis
- **Ellipticity** $\chi$: ratio of minor to major axis (or equivalently, the angle $\chi = \arctan(b/a)$, $-\pi/4 \leq \chi \leq \pi/4$)

These are related to $E_x$, $E_y$, $\delta$ by:

$$\tan 2\psi = \frac{2E_x E_y \cos\delta}{E_x^2 - E_y^2}$$

$$\sin 2\chi = \frac{2E_x E_y \sin\delta}{E_x^2 + E_y^2}$$

Linear polarization is a degenerate ellipse ($\chi = 0$, $b = 0$). Circular polarization is a circle ($\chi = \pm\pi/4$, $a = b$). Elliptical polarization is the general case.

## The Polarization State Space

The set of all polarization states (for a fixed propagation direction) forms a two-dimensional complex vector space. The basis states are any two orthogonal polarizations. Three natural choices:
- **Linear basis**: $\{|H\rangle, |V\rangle\}$ (horizontal, vertical)
- **Diagonal basis**: $\{|D\rangle, |A\rangle\}$ (diagonal at $\pm 45°$)
- **Circular basis**: $\{|R\rangle, |L\rangle\}$ (right, left circular)

The mathematical structure is isomorphic to a qubit in quantum mechanics — a two-dimensional Hilbert space. In quantum photonics (Unit VII), the polarization degree of freedom is used to encode one qubit per photon. In classical photonics, the polarization modes are used to double the information capacity of a waveguide.

**Orthogonal polarizations cannot interfere**: This follows from the dot product structure. If $\mathbf{E}_1 \cdot \mathbf{E}_2^* = 0$ (orthogonal), the interference term $2\text{Re}(\mathbf{E}_1 \cdot \mathbf{E}_2^*)$ vanishes. Orthogonal polarization modes pass through each other without interaction (in a linear medium) — this is the basis of polarization-division multiplexing.

## Polarization in Silicon Photonic Waveguides

Silicon nanowire waveguides support two distinct polarization modes:
- **TE mode**: Electric field primarily in the horizontal ($x$) direction (in the plane of the wafer)
- **TM mode**: Electric field primarily in the vertical ($y$) direction (normal to the wafer)

Due to the rectangular cross-section ($\sim 450 \times 220$ nm), the TE and TM modes have *different effective indices* ($n_\text{TE} \approx 2.4$, $n_\text{TM} \approx 1.8$ for typical dimensions at 1550 nm). They therefore accumulate different phases as they propagate. Any coupling between TE and TM (from waveguide bends, asymmetries, or rough sidewalls) creates an unpredictable output polarization state.

For this reason, photonic computing circuits are almost always operated in a single polarization (usually TE), and polarization-maintaining designs are used throughout. Polarization splitters at the chip input convert the incoming light (which may have arbitrary polarization) into two TE-mode streams: one for the original TE component and one for the TM component (rotated to TE by a polarization rotator). This is the *polarization diversity* scheme.

## Summary

- Polarization state is determined by the amplitudes $E_x$, $E_y$ and phase difference $\delta = \phi_y - \phi_x$.
- Linear: $\delta = 0$ or $\pi$; circular: $E_x = E_y$, $\delta = \pm\pi/2$; elliptical: general case.
- Circular polarization carries spin angular momentum $\pm\hbar$ per photon.
- The polarization state space is a 2D complex vector space — isomorphic to a qubit.
- In silicon photonics, TE/TM mode birefringence requires single-polarization or polarization-diversity designs.
