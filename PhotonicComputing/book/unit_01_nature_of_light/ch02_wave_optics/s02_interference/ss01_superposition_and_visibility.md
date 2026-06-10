# 2.2.1 — Superposition and Visibility

## The Superposition Principle

Maxwell's equations in linear media are linear: if $\mathbf{E}_1$ and $\mathbf{E}_2$ are both solutions, so is $\mathbf{E}_1 + \mathbf{E}_2$. This is the superposition principle, and it is the mathematical reason interference exists.

Consider two monochromatic plane waves of the same frequency $\omega$ and same polarization, arriving at a point with complex amplitudes $E_1$ and $E_2$:

$$E_1 = E_{01} e^{i\phi_1}, \qquad E_2 = E_{02} e^{i\phi_2}$$

The total field is $E = E_1 + E_2$, and the intensity is proportional to $|E|^2$:

$$I = |E_1 + E_2|^2 = |E_1|^2 + |E_2|^2 + 2\text{Re}(E_1 E_2^*)$$

$$= I_1 + I_2 + 2\sqrt{I_1 I_2} \cos(\phi_1 - \phi_2)$$

where we used $|E_1|^2 = I_1$, $|E_2|^2 = I_2$, and $E_1 E_2^* = \sqrt{I_1 I_2} e^{i(\phi_1 - \phi_2)}$.

This is the fundamental interference formula. The last term — the *interference term* — depends on the phase difference $\Delta\phi = \phi_1 - \phi_2$. When $\Delta\phi = 0, 2\pi, 4\pi, \ldots$: **constructive interference**, $I = (\sqrt{I_1} + \sqrt{I_2})^2$. When $\Delta\phi = \pi, 3\pi, \ldots$: **destructive interference**, $I = (\sqrt{I_1} - \sqrt{I_2})^2$.

For equal intensities $I_1 = I_2 = I_0$:
- Constructive: $I = 4I_0$ (double the amplitude, four times the intensity)
- Destructive: $I = 0$ (complete cancellation)

Note that energy is conserved: the average of $4I_0$ and $0$ over a full phase cycle is $2I_0 = I_1 + I_2$. Interference redistributes energy spatially or temporally; it does not create or destroy it.

## Fringe Visibility

When the phase difference $\Delta\phi$ varies across space (as in a double-slit experiment) or time (as in a scanning interferometer), the intensity traces out a sinusoidal pattern — an *interference fringe pattern*. The contrast of this pattern is quantified by the *visibility* (or fringe contrast):

$$V = \frac{I_\text{max} - I_\text{min}}{I_\text{max} + I_\text{min}}$$

For the two-beam case above:
- $I_\text{max} = I_1 + I_2 + 2\sqrt{I_1 I_2}$ (constructive)
- $I_\text{min} = I_1 + I_2 - 2\sqrt{I_1 I_2}$ (destructive)

$$V = \frac{4\sqrt{I_1 I_2}}{2(I_1 + I_2)} = \frac{2\sqrt{I_1 I_2}}{I_1 + I_2}$$

For equal intensities ($I_1 = I_2$): $V = 1$ (perfect contrast). For very unequal intensities ($I_1 \gg I_2$): $V \approx 2\sqrt{I_2/I_1} \ll 1$ (low contrast). The visibility is maximized when the two beams have equal intensity.

**Why this matters for photonic computing**: An MZI used as a switch or weight element requires high-visibility interference to achieve a high extinction ratio (the ratio of power in the "on" state to power in the "off" state). If the two MZI arms have different losses, $I_1 \neq I_2$ at the combiner, and the extinction ratio is limited by the visibility formula. This is a practical concern in silicon photonic MZIs, where waveguide imperfections cause arm-to-arm loss imbalance.

## Conditions for Interference

For the interference term $2\sqrt{I_1 I_2}\cos\Delta\phi$ to be observable, several conditions must be met:

**1. Same frequency (temporal coherence)**  
If the two waves have different frequencies $\omega_1$ and $\omega_2$, the phase difference $\Delta\phi(t) = (\omega_1 - \omega_2)t + \text{const}$ varies in time. If the detector is slow (integrates over many oscillation periods), the time-averaged interference term is:

$$\langle \cos[(\omega_1 - \omega_2)t] \rangle = 0 \quad \text{(if } \omega_1 \neq \omega_2\text{)}$$

No interference fringes are observed. The two waves must have the same frequency (or nearly so, with the frequency difference small compared to the detector bandwidth and the inverse of the integration time).

**2. Same polarization**  
The interference term $2\text{Re}(E_1 E_2^*)$ involves the dot product of the two electric field vectors. If the fields are orthogonally polarized ($\mathbf{E}_1 \perp \mathbf{E}_2$), the dot product is zero and there is no interference. Interference requires parallel (or partially parallel) polarization components.

This is why polarization matters in photonic computing: if polarization is not controlled, two optical signals that are supposed to interfere may be orthogonally polarized and produce no output. Silicon photonic waveguides are typically operated in a single polarization mode (usually TE) to ensure consistent interference behavior.

**3. Stable phase relationship (coherence)**  
Real light sources are not perfectly monochromatic. A source with spectral width $\Delta\nu$ has a *coherence time* $\tau_c \sim 1/\Delta\nu$ over which the phase of the wave is stable. If the path length difference between the two interfering beams introduces a time delay $\tau$ longer than $\tau_c$, the phase difference fluctuates randomly in time, and the interference term averages to zero.

A laser with linewidth $\Delta\nu = 1$ MHz has coherence time $\tau_c \sim 1$ μs and coherence length $L_c = c\tau_c \sim 300$ m. A photonic integrated circuit with path length differences of micrometers to millimeters is well within the coherence length of any practical laser source — this is why coherent photonic computing (which relies on interference) is feasible.

This topic is treated in full in Section 2.5.

## The Phase Difference as an Information Carrier

The fundamental encoding in interference-based computing is the phase difference $\Delta\phi$. Changing $\Delta\phi$ from $0$ to $\pi$ moves the output intensity continuously from $I_\text{max}$ to $I_\text{min}$ — from fully constructive to fully destructive interference. In the MZI architecture, $\Delta\phi$ is set by an electro-optic modulator that shifts the refractive index of one arm via the plasma dispersion effect (Soref-Bennett relations, Section 1.6.2). A phase shift of $\pi$ rad corresponds to a waveguide index change of $\Delta n \cdot L = \lambda/2$, which for a 1 mm silicon modulator requires $\Delta n \approx 7.75 \times 10^{-4}$.

The steepness of the transfer function $I(\Delta\phi) = (I_1 + I_2)(1 + V\cos\Delta\phi)/2$ around the quadrature point ($\Delta\phi = \pi/2$) determines the sensitivity: the rate of intensity change per unit phase change is $dI/d(\Delta\phi) = -(I_1 + I_2)V\sin\Delta\phi/2$, maximized at $\Delta\phi = \pi/2$. Operating at quadrature maximizes sensitivity to small phase changes — important for analog photonic computing where precision matters.

## Summary

- The intensity of two superposed coherent waves: $I = I_1 + I_2 + 2\sqrt{I_1 I_2}\cos\Delta\phi$.
- Fringe visibility: $V = 2\sqrt{I_1 I_2}/(I_1 + I_2)$, maximized for equal intensities.
- Conditions for interference: same frequency, same polarization, stable phase (coherence).
- In photonic computing, phase difference $\Delta\phi$ is the fundamental analog signal; interference converts it to intensity.
