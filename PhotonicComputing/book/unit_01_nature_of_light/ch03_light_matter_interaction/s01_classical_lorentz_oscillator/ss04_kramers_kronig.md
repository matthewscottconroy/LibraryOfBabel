# 3.1.4 — The Kramers-Kronig Relations

## Causality and the Kramers-Kronig Theorem

The Kramers-Kronig (KK) relations are among the most beautiful and powerful results in all of optics. They say: if a medium responds causally to an electromagnetic field — that is, if the polarization at time $t$ depends only on the electric field at times $t' \leq t$ (no response can precede its cause) — then the real and imaginary parts of the susceptibility $\chi(\omega)$ are not independent. They are related by integral transforms.

This is remarkable: a purely abstract philosophical principle (causality) imposes a quantitative constraint on measurable physical quantities. The relations were derived independently by Ralph Kronig (1926) [1] and Hendrik Kramers (1927) [2].

## Derivation via Contour Integration

The susceptibility $\chi(\omega)$ is the Fourier transform of the response function $\chi(t)$ (the polarization at time $t$ in response to a delta-function field at $t = 0$):

$$\chi(\omega) = \int_{-\infty}^{\infty} \chi(t) e^{i\omega t} dt$$

Causality requires: $\chi(t) = 0$ for $t < 0$ (no polarization before the field arrives).

This condition means $\chi(\omega)$ is *analytic in the upper half* of the complex $\omega$-plane: for $\text{Im}(\omega) > 0$, the exponential $e^{i\omega t} = e^{i\text{Re}(\omega) t} e^{-\text{Im}(\omega) t}$ decays exponentially for $t > 0$, ensuring convergence. The integral defines an analytic function.

By the Cauchy integral formula: for any point $\omega$ on the real axis, if $\chi(\omega')$ is analytic in the upper half-plane and vanishes as $|\omega'| \to \infty$:

$$\chi(\omega) = \frac{1}{i\pi} \mathcal{P}\int_{-\infty}^{\infty} \frac{\chi(\omega')}{\omega' - \omega} d\omega'$$

where $\mathcal{P}$ denotes the Cauchy principal value. Taking real and imaginary parts:

$$\chi'(\omega) = \frac{1}{\pi}\mathcal{P}\int_{-\infty}^{\infty}\frac{\chi''(\omega')}{\omega' - \omega}d\omega'$$

$$\chi''(\omega) = -\frac{1}{\pi}\mathcal{P}\int_{-\infty}^{\infty}\frac{\chi'(\omega')}{\omega' - \omega}d\omega'$$

These are the **Kramers-Kronig relations** in their general form. Using the symmetry $\chi'(-\omega) = \chi'(\omega)$ and $\chi''(-\omega) = -\chi''(\omega)$ (from the reality of $\chi(t)$), they can be rewritten as integrals over positive frequencies only:

$$n(\omega) - 1 = \frac{2}{\pi}\mathcal{P}\int_0^\infty\frac{\omega'\kappa(\omega')}{\omega'^2 - \omega^2}d\omega'$$

$$\kappa(\omega) = -\frac{2\omega}{\pi}\mathcal{P}\int_0^\infty\frac{n(\omega') - 1}{\omega'^2 - \omega^2}d\omega'$$

where we have used $1 + \chi_e' \approx n^2$ and $\chi_e'' \approx 2n\kappa$ (valid near transparency). These are the KK relations in the form quoted in Chapter 1 (Section 1.6.2).

## Verification: The Lorentz Oscillator

Let us verify the KK relations for the Lorentz oscillator. The imaginary part:

$$\chi_e''(\omega) = \frac{\omega_p^2\gamma\omega}{(\omega_0^2-\omega^2)^2+\gamma^2\omega^2}$$

Substituting into the first KK relation and computing the principal value integral (a residue calculation) gives exactly:

$$\chi_e'(\omega) = \frac{\omega_p^2(\omega_0^2-\omega^2)}{(\omega_0^2-\omega^2)^2+\gamma^2\omega^2}$$

which matches the Lorentz oscillator result. The KK relations are thus not just an abstract theorem — they are satisfied by every physical model of a causal medium.

## Physical Consequences

**1. You cannot have absorption without dispersion.**

If $\chi_e''(\omega) \neq 0$ for some $\omega$, then $\chi_e'(\omega)$ is nonzero everywhere (because the KK integral is nonzero: a localized bump in $\chi''$ contributes a non-zero integral for all $\omega'$). This means that a material that absorbs light at any wavelength has a modified refractive index at all wavelengths. No absorption without dispersion — ever.

**2. You cannot have a frequency-independent refractive index (in a physical material).**

A medium with perfectly constant $n(\omega) = n_0$ at all frequencies would have $\chi'(\omega) = n_0^2 - 1$ = constant and therefore $\chi''(\omega) = 0$ everywhere (from the second KK relation). A non-absorbing medium at all frequencies has a perfectly dispersionless refractive index. But wait — the KK relation also says that if $\chi'' = 0$ everywhere, then $\chi'$ must equal a constant (from the first relation, with the integral vanishing). This is consistent: a truly non-absorbing medium has constant $n$. But no real material is non-absorbing at all frequencies (every material has absorption at some wavelength). Therefore, every real material has some dispersion.

**3. The area under the absorption spectrum is fixed.**

The *sum rule* (f-sum rule): the integral $\int_0^\infty \omega\chi''(\omega)d\omega = \pi\omega_p^2/2$ is fixed by the density of electrons (via $\omega_p$). This is a constraint: if you reduce absorption at one frequency by chemical modification of a material, you must increase it somewhere else. The total absorptive strength is conserved.

**4. The electro-optic effect and absorption are linked.**

For silicon, the Soref-Bennett equations are a specific instance of the KK relations. The carrier-induced change in $n$ ($\Delta n$) and in absorption ($\Delta\alpha$) at 1550 nm are related by the KK integral applied to the free-carrier Drude susceptibility. The specific numerical coefficients in the Soref-Bennett equations are the KK transform of the free-carrier absorption spectrum. This is not a coincidence — it is the KK relations applied to the plasma dispersion effect [3].

**Practical consequence for silicon modulators**: an electro-optic phase modulator that changes $n$ via carrier injection also unavoidably changes the absorption coefficient. This chirp — the simultaneous amplitude and phase modulation — is an intrinsic property of silicon modulators and cannot be avoided. It is a direct consequence of the KK relations and the physical mechanism (plasma dispersion effect). This is why lithium niobate modulators (which use the Pockels effect, a different physical mechanism not limited by KK in the same way) can achieve lower chirp than silicon modulators.

## Practical Use of KK Relations

The KK relations are used to:

1. **Compute $n(\omega)$ from measured $\kappa(\omega)$** (or vice versa). If you measure the absorption spectrum of a material (e.g., by transmission spectroscopy), you can compute the full refractive index spectrum by the KK transform — without having to measure the phase of the reflected or transmitted wave. This is the basis of many optical characterization techniques.

2. **Verify consistency of measured optical data**. If a measured $n(\omega)$ and $\kappa(\omega)$ do not satisfy the KK relations, the data are inconsistent (at least one of them is wrong, or the material is active/non-passive).

3. **Constrain device design**. As shown above, the KK relations constrain the tradeoff between refractive index change and induced absorption in electro-optic devices. This is a design constraint that cannot be circumvented by clever engineering — it is a physical law.

## Summary

- Kramers-Kronig relations: $n(\omega)$ and $\kappa(\omega)$ are Hilbert transform pairs — consequence of causality alone.
- Cannot have absorption without dispersion; cannot have frequency-independent $n$ in a physical material.
- f-sum rule: total absorption strength $\int_0^\infty\omega\chi''(\omega)d\omega$ is fixed by electron density.
- Silicon modulator chirp is a direct consequence of KK relations applied to the plasma dispersion effect.

---

*References*

[1] Kronig, R. de L. (1926). On the theory of dispersion of X-rays. *Journal of the Optical Society of America*, 12(6), 547–557. [DOI: 10.1364/JOSA.12.000547]

[2] Kramers, H.A. (1927). La diffusion de la lumière par les atomes. *Atti del Congresso Internazionale dei Fisici*, Como, Vol. 2, pp. 545–557.

[3] Soref, R.A. & Bennett, B.R. (1987). Electrooptical effects in silicon. *IEEE Journal of Quantum Electronics*, 23(1), 123–129. [DOI: 10.1109/JQE.1987.1073206]

[4] Toll, J.S. (1956). Causality and the dispersion relation: logical foundations. *Physical Review*, 104(6), 1760–1770. [DOI: 10.1103/PhysRev.104.1760] [Rigorous derivation of the KK relations from causality.]
