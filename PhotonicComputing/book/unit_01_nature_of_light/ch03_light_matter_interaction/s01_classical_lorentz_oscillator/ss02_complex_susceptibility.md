# 3.1.2 — The Complex Susceptibility

## Defining the Susceptibility

The electric polarization density $\mathbf{P}$ is related to the electric field $\mathbf{E}$ via the electric susceptibility $\chi_e$:

$$\mathbf{P} = \varepsilon_0 \chi_e \mathbf{E}$$

(for linear, isotropic media). From the Lorentz oscillator result (Section 3.1.1), with $N$ oscillators per unit volume:

$$\chi_e(\omega) = \frac{Ne^2}{\varepsilon_0 m} \cdot \frac{1}{\omega_0^2 - \omega^2 - i\gamma\omega} = \frac{\omega_p^2}{\omega_0^2 - \omega^2 - i\gamma\omega}$$

where we have corrected the sign (the polarization points opposite to the electron displacement: $P = -Nex$, with $x_0 = -(e/m)E_0/(\omega_0^2 - \omega^2 - i\gamma\omega)$, so $P = \varepsilon_0\chi_e E$ with $\chi_e$ as above).

Note: in general, a material has multiple resonances (multiple electron oscillators at different frequencies), so the total susceptibility is:

$$\chi_e(\omega) = \sum_j \frac{f_j \omega_{p,j}^2}{\omega_{0j}^2 - \omega^2 - i\gamma_j\omega}$$

where $f_j$ is the *oscillator strength* of the $j$-th transition (a dimensionless number between 0 and 1, normalized so $\sum_j f_j = 1$ per electron — the Thomas-Reiche-Kuhn sum rule).

## From Susceptibility to Dielectric Constant and Refractive Index

The relative permittivity (dielectric constant) is:

$$\varepsilon_r(\omega) = 1 + \chi_e(\omega) = 1 + \frac{\omega_p^2}{\omega_0^2 - \omega^2 - i\gamma\omega}$$

The complex refractive index $\tilde{n}(\omega) = n(\omega) + i\kappa(\omega)$ satisfies $\tilde{n}^2 = \varepsilon_r$ (for non-magnetic media):

$$\tilde{n}^2 = (n + i\kappa)^2 = n^2 - \kappa^2 + 2in\kappa = \varepsilon_r' + i\varepsilon_r''$$

giving:
$$n^2 - \kappa^2 = \varepsilon_r' = 1 + \chi_e', \qquad 2n\kappa = \varepsilon_r'' = \chi_e''$$

where $\chi_e = \chi_e' + i\chi_e''$. In most optical regions (away from resonance), $\kappa \ll n$, so:

$$n \approx \sqrt{1 + \chi_e'}, \qquad \kappa \approx \frac{\chi_e''}{2n}$$

The absorption coefficient: $\alpha = 2\omega\kappa/c = \omega\chi_e''/(nc)$.

## Explicit Forms: Real and Imaginary Parts

From $\chi_e(\omega) = \omega_p^2/(\omega_0^2 - \omega^2 - i\gamma\omega)$, multiply numerator and denominator by the complex conjugate:

$$\chi_e(\omega) = \frac{\omega_p^2(\omega_0^2 - \omega^2 + i\gamma\omega)}{(\omega_0^2 - \omega^2)^2 + \gamma^2\omega^2}$$

$$\chi_e' = \frac{\omega_p^2(\omega_0^2 - \omega^2)}{(\omega_0^2 - \omega^2)^2 + \gamma^2\omega^2}$$

$$\chi_e'' = \frac{\omega_p^2\gamma\omega}{(\omega_0^2 - \omega^2)^2 + \gamma^2\omega^2}$$

**Key features**:
- $\chi_e'' \geq 0$ for $\omega > 0$: the imaginary part is always positive for a passive medium, corresponding to absorption.
- $\chi_e''$ has a Lorentzian shape centered at $\omega_0$ with width $\gamma$.
- $\chi_e'$ is dispersive (S-shaped curve crossing zero at $\omega_0$): positive (increases $n$) below resonance, negative (decreases $n$) above resonance.
- At $\omega = 0$: $\chi_e' = \omega_p^2/\omega_0^2 = $ static susceptibility.
- At $\omega \to \infty$: $\chi_e' \to -\omega_p^2/\omega^2 \to 0^-$; the refractive index approaches 1 from below.

## Physical Meaning of the Real and Imaginary Parts

**$\chi_e'$ (real part)**: the *in-phase* response. The polarization oscillates in phase with the driving field. This contributes to the energy stored in the medium (reactive response) and modifies the phase velocity of light (changes $n$). A material with $\chi_e' > 0$ has $n > 1$ — light travels slower than in vacuum.

**$\chi_e''$ (imaginary part)**: the *out-of-phase* ($\pi/2$ lagging) response. The polarization lags the field. The work done by the field on the polarization $(\mathbf{E} \cdot \dot{\mathbf{P}})$ is nonzero when $\mathbf{P}$ is out of phase with $\mathbf{E}$, and this corresponds to net energy transfer from the field to the medium — absorption. A material with $\chi_e'' > 0$ absorbs light.

**For gain media**: A population-inverted laser medium can have $\chi_e'' < 0$ — corresponding to net energy transfer from the medium to the field. This is stimulated emission (amplification), the basis of the laser (Section 3.2.3).

## The Sellmeier Equation

For most optical materials (glass, silicon, LiNbO₃), the absorption resonances are either in the ultraviolet or the infrared, and the material is transparent at the wavelength of interest. In this transparent region, $\chi_e'' \approx 0$ and only $\chi_e'$ matters. Far from the $j$-th resonance ($|\omega - \omega_{0j}| \gg \gamma_j$):

$$n^2(\omega) \approx 1 + \sum_j \frac{f_j\omega_{p,j}^2}{\omega_{0j}^2 - \omega^2}$$

Rewriting in terms of wavelength $\lambda$:

$$n^2(\lambda) = 1 + \sum_j \frac{A_j \lambda^2}{\lambda^2 - \lambda_{0j}^2}$$

This is the *Sellmeier equation* [1], where $A_j$ and $\lambda_{0j}$ are empirically determined for each material. For fused silica (SiO₂), the three-term Sellmeier equation (Malitson 1965 [2]) has $\lambda_{01} = 0.0684$ μm, $\lambda_{02} = 0.116$ μm, $\lambda_{03} = 9.896$ μm — two UV resonances and one IR (Si-O stretching mode). The fit is accurate to better than $10^{-5}$ in $n$ across the visible and near-infrared.

**Application**: The dispersion of silica fiber (the change in group velocity with wavelength, $\beta_2 = d^2k/d\omega^2$) is entirely determined by the Sellmeier equation. The zero-dispersion wavelength of standard silica fiber ($\lambda \approx 1270$ nm for bulk silica, shifted to $\lambda = 1310$ nm for standard single-mode fiber by waveguide dispersion) is a consequence of the balance between the UV and IR resonances of the Sellmeier sum.

## Application: The Drude Model for Free Carriers

For free electrons (no restoring force: $\omega_0 = 0$), the oscillator becomes the Drude model:

$$\varepsilon_r(\omega) = 1 - \frac{\omega_p^2}{\omega^2 + i\gamma\omega} = 1 - \frac{\omega_p^2}{\omega^2(1 + i\gamma/\omega)}$$

For $\omega \gg \gamma$ (optical frequencies):

$$\varepsilon_r(\omega) \approx 1 - \frac{\omega_p^2}{\omega^2}$$

This is less than 1 for $\omega < \omega_p$ (plasma frequency): the refractive index is imaginary and the material is opaque (light cannot propagate). For $\omega > \omega_p$: $\varepsilon_r > 0$ and the metal is transparent.

**For silicon**: The addition of free carriers (by doping or injection) adds a Drude contribution to the susceptibility. With carrier density $N_e$ (electrons) and $N_h$ (holes):

$$\Delta\varepsilon_r = -\frac{e^2}{\varepsilon_0 m_e^*\omega^2}N_e - \frac{e^2}{\varepsilon_0 m_h^*\omega^2}N_h$$

The corresponding changes in $n$ and $\kappa$ are the Soref-Bennett relations (Section 1.6.2), derived microscopically from the Drude model. The Drude model provides the microscopic justification for the empirical Soref-Bennett formulas.

## Summary

- Complex susceptibility: $\chi_e = \chi_e' + i\chi_e''$, with $\chi_e' \propto (\omega_0^2-\omega^2)/[...]$ (dispersive) and $\chi_e'' \propto \gamma\omega/[...]$ (absorptive, Lorentzian).
- Complex refractive index: $n^2-\kappa^2 = 1 + \chi_e'$; $2n\kappa = \chi_e''$.
- $\chi_e' > 0$ → increased $n$; $\chi_e'' > 0$ → absorption; $\chi_e'' < 0$ → gain.
- Sellmeier equation: $n^2(\lambda) = 1 + \sum_j A_j\lambda^2/(\lambda^2 - \lambda_{0j}^2)$, derived from Lorentz oscillators far from resonance.
- Drude model ($\omega_0 = 0$): free carriers, plasma frequency, metallic reflection below $\omega_p$.

---

*References*

[1] Sellmeier, W. (1872). Zur Erklärung der abnormen Farbenfolge im Spectrum einiger Substanzen. *Annalen der Physik und Chemie*, 147(6), 386–403.

[2] Malitson, I.H. (1965). Interspecimen comparison of the refractive index of fused silica. *Journal of the Optical Society of America*, 55(10), 1205–1209. [DOI: 10.1364/JOSA.55.001205]
