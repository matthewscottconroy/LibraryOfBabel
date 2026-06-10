# 1.6.2 The Complex Refractive Index and Absorption

## When a Material Absorbs

In a lossy material — a metal, a semiconductor below its transparency wavelength, or any material with absorption at the operating wavelength — the dielectric response is complex:

$$\varepsilon_r(\omega) = \varepsilon_r'(\omega) + i\varepsilon_r''(\omega)$$

where $\varepsilon_r' > 0$ and the sign of $\varepsilon_r''$ determines whether the material absorbs ($\varepsilon_r'' < 0$ in the convention $e^{-i\omega t}$, used here) or amplifies ($\varepsilon_r'' > 0$ — the case in a laser gain medium).

The complex refractive index:

$$\tilde{n} = \sqrt{\varepsilon_r} = n + i\kappa$$

has real part $n$ (the phase refractive index, determining phase velocity) and imaginary part $\kappa$ (the extinction coefficient, determining absorption).

The relationship between $(\varepsilon_r', \varepsilon_r'')$ and $(n, \kappa)$:

$$\varepsilon_r' = n^2 - \kappa^2, \quad \varepsilon_r'' = 2n\kappa$$

$$n = \sqrt{\frac{\varepsilon_r' + \sqrt{\varepsilon_r'^2 + \varepsilon_r''^2}}{2}}, \quad \kappa = \frac{|\varepsilon_r''|}{2n}$$

## The Beer-Lambert Law

A plane wave in an absorbing medium with complex refractive index $\tilde{n} = n + i\kappa$ propagating in the $z$-direction:

$$E(z) = E_0 e^{i\tilde{n}\omega z/c} = E_0 e^{in\omega z/c} e^{-\kappa\omega z/c}$$

The intensity $I \propto |E|^2$:

$$I(z) = I_0 e^{-2\kappa\omega z/c} = I_0 e^{-\alpha z}$$

where $\alpha = 2\kappa\omega/c = 4\pi\kappa/\lambda_0$ is the **power absorption coefficient** [m⁻¹].

This is the Beer-Lambert law. In photonics, absorption is often quoted in dB/cm:

$$\alpha_{\text{dB/cm}} = 4.343\alpha \times 100\ [\text{dB/cm}] \quad (\text{for }\alpha\text{ in m}^{-1})$$

**Standard waveguide losses**:
- Silicon strip waveguide: $\alpha \approx 2$–3 dB/cm (from sidewall roughness, not material absorption)
- Silicon nitride waveguide: $\alpha \approx 0.1$ dB/m = 0.001 dB/cm (ultra-low loss)
- Silica optical fiber: $\alpha = 0.2$ dB/km at 1550 nm

## The Connection to Gain in Lasers

In a laser gain medium, the light is amplified rather than absorbed. This corresponds to $\kappa < 0$ in the convention where the imaginary part of the refractive index represents loss. Equivalently, the gain coefficient $g = -\alpha = 4\pi|\kappa|/\lambda_0 > 0$ describes exponential growth of intensity:

$$I(z) = I_0 e^{gz}$$

The gain arises from stimulated emission — a process in which an excited atom is caused to emit a photon by the presence of another photon. The emitted photon is identical to the stimulating photon in frequency, direction, polarization, and phase. This coherent amplification is what makes lasers possible, and the physics of gain is developed in Chapter 4.

## Kramers-Kronig Relations: Connecting $n$ and $\kappa$

The real and imaginary parts of the complex refractive index are not independent. They are related by the **Kramers-Kronig relations** — a consequence of causality (the material cannot respond before the field is applied) and the analyticity of the response function in the upper complex frequency half-plane.

The Kramers-Kronig relations state:

$$n(\omega) - 1 = \frac{2}{\pi}\mathcal{P}\int_0^\infty \frac{\omega' \kappa(\omega')}{\omega'^2 - \omega^2}d\omega'$$

$$\kappa(\omega) = -\frac{2\omega}{\pi}\mathcal{P}\int_0^\infty \frac{n(\omega') - 1}{\omega'^2 - \omega^2}d\omega'$$

where $\mathcal{P}$ denotes the Cauchy principal value of the integral.

**Physical implication**: If you know the absorption spectrum $\kappa(\omega)$ at all frequencies, you can calculate the dispersion $n(\omega)$ — and vice versa. You cannot have absorption without changing the refractive index; you cannot change the refractive index without changing the absorption somewhere.

**Importance for silicon photonics**: The plasma dispersion effect in silicon — the change in refractive index when free carriers are injected or depleted by a voltage — is always accompanied by a change in absorption. The Soref-Bennett equations [1] quantify both effects simultaneously:

$$\Delta n = -\left[8.8\times10^{-22}\Delta N_e + 8.5\times10^{-18}(\Delta N_h)^{0.8}\right]$$
$$\Delta\alpha = 8.5\times10^{-18}\Delta N_e + 6.0\times10^{-18}\Delta N_h$$

where $\Delta N_e$ and $\Delta N_h$ are electron and hole carrier density changes [cm⁻³]. These Kramers-Kronig-consistent relations are the basis of all silicon electro-optic modulators.

---

## References

[1] Soref, R.A., & Bennett, B.R. (1987). "Electrooptical effects in silicon." *IEEE Journal of Quantum Electronics*, 23(1), 123–129.

[2] Toll, J.S. (1956). "Causality and the dispersion relation: Logical foundations." *Physical Review*, 104(6), 1760–1770. [Classic derivation of Kramers-Kronig from causality.]
