# 3.1.1 — The Driven Harmonic Oscillator

## The Model

Consider a single electron of mass $m$ and charge $-e$ ($e > 0$) bound to a nucleus by a harmonic restoring force $F_\text{restore} = -m\omega_0^2 x$ (where $\omega_0$ is the natural (angular) frequency of oscillation) and subject to a damping force $F_\text{damp} = -m\gamma\dot{x}$ (where $\gamma$ is the damping rate). The electron is driven by the electric field of a plane wave $E(t) = E_0\cos\omega t = \text{Re}[E_0 e^{-i\omega t}]$.

Newton's second law gives the equation of motion:

$$m\ddot{x} + m\gamma\dot{x} + m\omega_0^2 x = -eE(t) = -eE_0\cos\omega t$$

This is the equation of a driven, damped harmonic oscillator. We seek the steady-state (particular) solution after the transient (homogeneous solution) has decayed. Using complex notation (with the understanding that the physical displacement is the real part):

$$m\ddot{x} + m\gamma\dot{x} + m\omega_0^2 x = -eE_0 e^{-i\omega t}$$

We try $x(t) = x_0 e^{-i\omega t}$:

$$m(-\omega^2 - i\gamma\omega + \omega_0^2)x_0 e^{-i\omega t} = -eE_0 e^{-i\omega t}$$

Solving for $x_0$:

$$x_0 = \frac{-e/m}{\omega_0^2 - \omega^2 - i\gamma\omega} E_0$$

## The Complex Amplitude

The steady-state displacement is $x(t) = x_0 e^{-i\omega t}$, with complex amplitude:

$$x_0(\omega) = \frac{-e/m}{\omega_0^2 - \omega^2 - i\gamma\omega} E_0$$

This is a *complex* amplitude: the electron's displacement is not in phase with the driving field. The magnitude $|x_0|$ and phase $\arg(x_0)$ vary with frequency $\omega$ in a characteristic way.

**At resonance** ($\omega = \omega_0$): the denominator is purely imaginary ($= -i\gamma\omega_0$), so $x_0 = ie/(m\gamma\omega_0) \cdot E_0$. The displacement is $\pi/2$ ahead of the field in phase — the electron is moving fastest (maximum velocity) when the field is maximum. The amplitude is $|x_0| = eE_0/(m\gamma\omega_0)$, which diverges as $\gamma \to 0$.

**Far below resonance** ($\omega \ll \omega_0$): $x_0 \approx -eE_0/(m\omega_0^2)$ — the displacement is in phase with the field (just restoring-force balance), with no frequency dependence. This is the quasi-static limit.

**Far above resonance** ($\omega \gg \omega_0$): $x_0 \approx eE_0/(m\omega^2)$ — the displacement is small and in phase with the field (inertia-dominated response). The electron barely moves at high frequencies.

## Physical Interpretation: The Oscillator as a Model Atom

The damping constant $\gamma$ represents the rate at which the electron loses energy:
- **Radiation damping** (classical): an accelerating charge radiates, losing energy. The radiation reaction force gives a damping rate $\gamma_\text{rad} = e^2\omega^2/(6\pi\varepsilon_0 m c^3)$ (Abraham-Lorentz force). At optical frequencies, this is typically much smaller than collision damping.
- **Collision damping**: in a gas, the electron's oscillation is interrupted by collisions with other atoms at rate $\gamma_\text{coll} = 1/\tau_\text{coll}$.
- **In solids**: damping is due to electron-phonon scattering, electron-electron scattering, and defect scattering.

The resonant frequency $\omega_0$ corresponds to the natural frequency of the electron's oscillation in the atomic potential — in quantum mechanical language, to the frequency of the $1\to 2$ transition: $\omega_0 = (E_2 - E_1)/\hbar$.

## The Macroscopic Polarization

For $N$ electrons per unit volume, each displaced by $x$, the polarization density (dipole moment per unit volume) is:

$$P = -Nex = -Ne \cdot x_0(\omega) e^{-i\omega t} = \varepsilon_0 \chi(\omega) E_0 e^{-i\omega t}$$

where we identify the *electric susceptibility*:

$$\chi(\omega) = \frac{-Ne^2/(\varepsilon_0 m)}{\omega_0^2 - \omega^2 - i\gamma\omega} = \frac{-\omega_p^2}{\omega_0^2 - \omega^2 - i\gamma\omega}$$

where $\omega_p^2 = Ne^2/(\varepsilon_0 m)$ is the *plasma frequency* (the natural frequency of collective oscillation of the electron gas).

For silicon at telecom wavelengths: $N = 5 \times 10^{28}$ electrons/m³ (free electron density for intrinsic silicon; for the full band structure calculation, an effective number of oscillators per atom and a strength factor must be used). The plasma frequency of the free electron gas in a metal is $\omega_p \sim 10^{16}$ rad/s; for intrinsic silicon, the effective plasma frequency is much lower.

## Resonance Behavior: Lorentzian Lineshape

The squared magnitude of the amplitude:

$$|x_0|^2 \propto \frac{1}{(\omega_0^2 - \omega^2)^2 + \gamma^2\omega^2}$$

Near resonance ($\omega \approx \omega_0$, $|\omega - \omega_0| \ll \omega_0$): $\omega_0^2 - \omega^2 \approx 2\omega_0(\omega_0 - \omega)$. Then:

$$|x_0|^2 \propto \frac{1}{4\omega_0^2(\omega_0 - \omega)^2 + \gamma^2\omega_0^2} \propto \frac{1}{(\omega - \omega_0)^2 + (\gamma/2)^2}$$

This is the *Lorentzian lineshape* with half-width at half-maximum (HWHM) of $\gamma/2$, or full-width $\gamma$. The Lorentzian is the response function of any damped harmonic oscillator and appears throughout physics and engineering (resonance curves, spectral lines, filter transfer functions, phonon peaks in Raman spectra, etc.).

## Quality Factor

The *quality factor* $Q$ of the oscillator is:

$$Q = \frac{\omega_0}{\gamma}$$

This is the ratio of the resonant frequency to the linewidth (same definition as the $Q$ factor of a Fabry-Pérot resonator, Section 2.2.3 — not a coincidence). High $Q$ means narrow resonance, slow damping, many oscillation cycles per damping time. For optical transitions in atoms, $Q \sim 10^8$; for phonon modes in solids, $Q \sim 10^2$–$10^4$; for free electrons in a metal, $Q \sim 1$–$10$.

## Energy Storage and Dissipation

The work done by the electric field on the oscillating electron is $W = F\dot{x} = -eE\dot{x}$. Time-averaging using the steady-state solutions:

$$\langle W \rangle = \frac{1}{2}\text{Re}(-eE_0)\text{Re}(i\omega x_0^*) = \frac{e^2E_0^2}{2m} \cdot \frac{\gamma\omega^2}{(\omega_0^2-\omega^2)^2 + \gamma^2\omega^2}$$

This is the power absorbed per electron. Near resonance, $\langle W \rangle \propto \text{Lorentzian}(\omega)$ — the absorption spectrum is a Lorentzian. The imaginary part of $\chi(\omega)$ (which will be derived in Section 3.1.2) is exactly this Lorentzian, multiplied by constants.

## Summary

- Driven damped harmonic oscillator: $m\ddot{x} + m\gamma\dot{x} + m\omega_0^2 x = -eE_0 e^{-i\omega t}$.
- Steady-state amplitude: $x_0 = -(e/m)E_0/(\omega_0^2 - \omega^2 - i\gamma\omega)$.
- Complex amplitude → complex susceptibility $\chi(\omega)$.
- Lorentzian lineshape with FWHM $= \gamma$ centered at $\omega_0$.
- Quality factor $Q = \omega_0/\gamma$.
- Power absorption $\propto \text{Im}[\chi(\omega)] \propto$ Lorentzian.
