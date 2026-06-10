# 8.3.1 Surface Plasmon Polaritons

## The Plasma Frequency of Metals

To understand surface plasmons, we need to understand the optical response of metals. In Section 7.3.1, we used the Drude model to describe free carriers in silicon as a plasma. The same model applies to electrons in metals, but with much higher carrier densities:

For gold ($n_e \approx 5.9 \times 10^{28}$ m⁻³, $m^* \approx m_0$, $\tau_{\text{collision}} \approx 9 \times 10^{-15}$ s):

$$\omega_p = \sqrt{\frac{n_e e^2}{\varepsilon_0 m^*}} = \sqrt{\frac{5.9\times10^{28} \times (1.6\times10^{-19})^2}{8.85\times10^{-12} \times 9.11\times10^{-31}}} \approx 1.37 \times 10^{16} \text{ rad/s}$$

This corresponds to a plasma wavelength $\lambda_p = 2\pi c/\omega_p \approx 137$ nm — in the ultraviolet. Below $\omega_p$, the dielectric function is negative (the metal is opaque); above $\omega_p$, it becomes positive (the metal is transparent). Gold and silver have $\lambda_p \approx 130$–170 nm.

The Drude dielectric function for metals at frequency $\omega$:

$$\varepsilon(\omega) = 1 - \frac{\omega_p^2}{\omega^2 + i\omega/\tau} = \varepsilon_1(\omega) + i\varepsilon_2(\omega)$$

For gold at $\lambda = 1550$ nm ($\omega = 1.21 \times 10^{15}$ rad/s), using measured values [1]:
- $\varepsilon_1 \approx -114$ (large and negative)
- $\varepsilon_2 \approx 11$ (imaginary part, representing absorption)

The large negative real part is the key: it means gold behaves as a good metal at 1550 nm (opaque to propagation in the bulk), but supports surface modes.

## Derivation of the SPP Dispersion Relation

Consider a planar interface between a metal (dielectric function $\varepsilon_m < 0$) and a dielectric (dielectric function $\varepsilon_d > 0$). We look for a TM-polarized mode (magnetic field $H_y$ parallel to the interface, electric field in the $xz$ plane) propagating in the $x$-direction with wavevector $k_x$ and decaying in $|z|$.

In the dielectric ($z > 0$):
$$H_y = A e^{ik_x x - \kappa_d z - i\omega t}, \quad \kappa_d = \sqrt{k_x^2 - \varepsilon_d\omega^2/c^2} > 0$$

In the metal ($z < 0$):
$$H_y = A e^{ik_x x + \kappa_m z - i\omega t}, \quad \kappa_m = \sqrt{k_x^2 - \varepsilon_m\omega^2/c^2}$$

For $\varepsilon_m < 0$, $\kappa_m > 0$ if $k_x^2 > \varepsilon_m \omega^2/c^2$ — which is always satisfied for $\varepsilon_m < 0$ and any real $k_x$.

From $\nabla \times \mathbf{H} = -i\omega\varepsilon_0\varepsilon\mathbf{E}$, the tangential electric field components are:

$$E_x = \frac{ik_x}{\omega\varepsilon_0\varepsilon} H_y, \quad E_z = \frac{\kappa}{\omega\varepsilon_0\varepsilon} H_y$$

Matching the boundary conditions at $z = 0$ (continuity of $H_y$ and $E_x$):

$$H_y^+ = H_y^-: \quad A = A \quad \checkmark$$

$$E_x^+ = E_x^-: \quad \frac{ik_x}{\omega\varepsilon_0\varepsilon_d}A = -\frac{ik_x}{\omega\varepsilon_0\varepsilon_m}A$$

Wait — the sign convention for $E_x$ differs in the two regions because $E_x \propto \partial H_y/\partial z$, which has opposite signs above and below the interface. The correct condition is:

$$\frac{\kappa_d}{\varepsilon_d} + \frac{\kappa_m}{\varepsilon_m} = 0 \implies \frac{\kappa_d}{\varepsilon_d} = -\frac{\kappa_m}{\varepsilon_m}$$

Since $\kappa_d, \kappa_m > 0$ and $\varepsilon_d > 0$, we need $\varepsilon_m < 0$ — confirmed for metals. Combining with the definitions of $\kappa$:

$$\kappa_d^2 = k_x^2 - \varepsilon_d\left(\frac{\omega}{c}\right)^2$$
$$\kappa_m^2 = k_x^2 - \varepsilon_m\left(\frac{\omega}{c}\right)^2$$

And the condition $\kappa_d/\varepsilon_d = -\kappa_m/\varepsilon_m$, squaring and using the expressions for $\kappa$:

$$\frac{k_x^2 - \varepsilon_d(\omega/c)^2}{\varepsilon_d^2} = \frac{k_x^2 - \varepsilon_m(\omega/c)^2}{\varepsilon_m^2}$$

Solving for $k_x$:

$$\boxed{k_{\text{SPP}} = \frac{\omega}{c}\sqrt{\frac{\varepsilon_m\varepsilon_d}{\varepsilon_m + \varepsilon_d}}}$$

This is the **SPP dispersion relation**. It exists only when $\varepsilon_m + \varepsilon_d < 0$, i.e., $\varepsilon_m < -\varepsilon_d$ — the metal must be sufficiently negative.

## Properties of the SPP Mode

For gold at 1550 nm with $\varepsilon_m = -114 + 11i$ and $\varepsilon_d = 1$ (air):

$$k_{\text{SPP}} = \frac{\omega}{c}\sqrt{\frac{(-114+11i) \times 1}{(-114+11i) + 1}} \approx \frac{\omega}{c}\sqrt{\frac{-114}{-113}} \approx \frac{\omega}{c} \times 1.004$$

The SPP wavevector is only 0.4% larger than the free-space wavevector — the mode barely bends toward the surface. The confinement in the dielectric:

$$\kappa_d = \sqrt{k_{\text{SPP}}^2 - k_0^2} \approx k_0\sqrt{0.004} = 0.063k_0$$

Confinement length in air: $1/\kappa_d \approx 15.9 \times (\lambda/2\pi) \approx 3900$ nm at 1550 nm. This is several wavelengths — weak confinement because gold's large negative $\varepsilon_m$ pushes the SPP dispersion close to the light line.

The confinement in the metal (skin depth):

$$\kappa_m = \sqrt{k_{\text{SPP}}^2 - \varepsilon_m k_0^2} \approx k_0\sqrt{114} \approx 10.7k_0$$

Skin depth: $1/\kappa_m \approx 23$ nm — comparable to the optical skin depth of gold (~25 nm at 1550 nm). The field is strongly confined on the metal side.

For shorter wavelengths (closer to the surface plasmon resonance $\omega_{SP} = \omega_p/\sqrt{1+\varepsilon_d}$), the confinement increases dramatically but so does the loss.

## Propagation Length

The SPP propagates with complex wavevector $k_{\text{SPP}} = k' + ik''$. The propagation length is:

$$L_{\text{SPP}} = \frac{1}{2k''} = \frac{c}{2\omega}\left(\frac{\varepsilon_m' + \varepsilon_d}{\varepsilon_m'\varepsilon_d}\right)^{3/2}\frac{(\varepsilon_m')^2}{\varepsilon_m''}$$

where $\varepsilon_m = \varepsilon_m' + i\varepsilon_m''$ and the formula assumes $|\varepsilon_m'| \gg \varepsilon_m''$.

For gold at 1550 nm:

$$L_{\text{SPP}}^{\text{Au}} \approx \frac{c}{2\omega}\frac{(-114)^2}{11} \times \frac{(-113)^{3/2}}{(-114)^3 \times 1} \approx 50 \text{ μm}$$

For silver at 1550 nm ($\varepsilon_m = -129 + 3.3i$):

$$L_{\text{SPP}}^{\text{Ag}} \approx 300 \text{ μm}$$

These are the fundamental limits. Silver is better than gold at 1550 nm by about 6×, but even silver SPPs propagate only ~300 μm — comparable to the size of a photonic chip, but far less than the millimeter-to-centimeter distances required for most photonic interconnect applications.

## The Confinement-Loss Tradeoff

A fundamental tradeoff exists in all plasmonic structures: tighter confinement (smaller mode) comes at the cost of higher propagation loss. This can be seen from the dispersion relation: as $k_{\text{SPP}}$ increases above $k_0$ (tighter confinement), the imaginary part also increases rapidly.

Quantitatively, for an SPP of confinement area $A_{\text{eff}}$, the propagation length satisfies:

$$L_{\text{SPP}} \propto \frac{A_{\text{eff}}}{\lambda^2} \times \frac{(\varepsilon_m')^2}{\varepsilon_m''}$$

Tighter confinement ($A_{\text{eff}} \to 0$) means shorter propagation. This is not an engineering problem — it is a consequence of causality and the Kramers-Kronig relations: materials with large negative $\varepsilon'$ also have substantial $\varepsilon''$ (imaginary part, absorption).

The figure of merit for plasmonic applications is:

$$\text{FOM}_{\text{plasm}} = \frac{L_{\text{SPP}}}{\lambda} = \text{number of wavelengths of propagation} = \frac{|\varepsilon_m'|^2}{\varepsilon_m''}$$

For gold at 1550 nm: $\text{FOM} \approx 114^2/11 \approx 1180$, so $L \approx 1180 \times \lambda = 1.83$ mm — but this is for the *long-range* SPP (weakly confined). For a tightly confined mode, the FOM is much worse.

---

## References

[1] Johnson, P.B. & Christy, R.W. (1972). "Optical constants of the noble metals." *Physical Review B*, 6(12), 4370–4379. [The definitive measurement of optical constants for Au, Ag, Cu; the standard reference for plasmonic materials.]

[2] Ritchie, R.H. (1957). "Plasma losses by fast electrons in thin films." *Physical Review*, 106(5), 874–881. [Original theoretical prediction of surface plasmon polaritons.]

[3] Raether, H. (1988). *Surface Plasmons on Smooth and Rough Surfaces and on Gratings*. Springer. [The standard reference monograph for SPP physics, covering dispersion, coupling, and applications.]
