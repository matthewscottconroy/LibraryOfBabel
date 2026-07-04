# 21.1.1 — Quadrature Variables

## The Field Mode as a Harmonic Oscillator

A single mode of the electromagnetic field with annihilation operator $\hat{a}$ and creation operator $\hat{a}^\dagger$ (Chapter 18) satisfies $[\hat{a}, \hat{a}^\dagger] = 1$. From these we define the two *quadrature operators*:

$$\hat{x} = \frac{\hat{a} + \hat{a}^\dagger}{\sqrt{2}}, \qquad \hat{p} = \frac{\hat{a} - \hat{a}^\dagger}{i\sqrt{2}}$$

These are dimensionless, Hermitian, and satisfy the canonical commutation relation

$$[\hat{x}, \hat{p}] = i$$

in units where $\hbar = 1$. They are the exact formal analogues of position and momentum for a mechanical oscillator, but physically they are the amplitudes of the field's $\cos\omega t$ and $\sin\omega t$ components: writing the classical field as $E(t) \propto x\cos\omega t + p\sin\omega t$, the quadratures are the in-phase and out-of-phase amplitudes relative to a phase reference. That reference is supplied in practice by a laser — the local oscillator — which is why quadratures, unlike photon number, are only defined relative to a phase standard.

A general rotated quadrature interpolates between them:

$$\hat{x}_\theta = \hat{x}\cos\theta + \hat{p}\sin\theta = \frac{\hat{a}e^{-i\theta} + \hat{a}^\dagger e^{i\theta}}{\sqrt{2}}$$

so that $\hat{x}_0 = \hat{x}$ and $\hat{x}_{\pi/2} = \hat{p}$. The pair $(\hat{x}, \hat{p})$ spans a *phase space*: any single-mode state can be pictured as a quasi-probability distribution (the Wigner function, Section 21.1.2) over the $(x, p)$ plane.

**Convention warning.** The literature uses several normalizations: $\hat{x} = (\hat{a}+\hat{a}^\dagger)/\sqrt{2}$ with vacuum variance $1/2$ (used here, following Weedbrook et al. [1] up to a factor: they use vacuum variance 1); $\hat{x} = \hat{a}+\hat{a}^\dagger$ with vacuum variance 1; and $\hat{x} = (\hat{a}+\hat{a}^\dagger)/2$ with vacuum variance $1/4$. Always check the vacuum variance ("shot-noise unit") before comparing formulas across papers.

## Vacuum Noise and the Uncertainty Relation

The commutator $[\hat{x},\hat{p}]=i$ implies the Heisenberg relation

$$\Delta x \, \Delta p \geq \frac{1}{2}$$

The vacuum state saturates it symmetrically. Using $\hat{a}|0\rangle = 0$:

$$\langle 0|\hat{x}^2|0\rangle = \frac{1}{2}\langle 0|(\hat{a}+\hat{a}^\dagger)(\hat{a}+\hat{a}^\dagger)|0\rangle = \frac{1}{2} \quad\Rightarrow\quad \Delta x = \Delta p = \frac{1}{\sqrt{2}}$$

This is the *vacuum noise* or *shot noise*: even with zero photons, both quadratures fluctuate. Every homodyne measurement of the vacuum returns a Gaussian-distributed random number with variance $1/2$. This noise is not a detector artifact; it is the irreducible quantum fuzziness of the field, and it sets the reference level (0 dB) against which squeezing is measured.

A *coherent state* $|\alpha\rangle$ (the output of an ideal laser, Chapter 18) is the vacuum displaced in phase space: $\langle\hat{x}\rangle = \sqrt{2}\,\text{Re}(\alpha)$, $\langle\hat{p}\rangle = \sqrt{2}\,\text{Im}(\alpha)$, with the same isotropic noise disk $\Delta x = \Delta p = 1/\sqrt{2}$ as the vacuum. In phase space it is a circle of vacuum-sized uncertainty centered at $\sqrt{2}(\text{Re}\,\alpha, \text{Im}\,\alpha)$.

## Squeezed States

A *squeezed state* redistributes the vacuum uncertainty: one quadrature drops below the vacuum level at the expense of the conjugate one. The single-mode squeezing operator is

$$\hat{S}(r) = \exp\left[\frac{r}{2}\left(\hat{a}^2 - \hat{a}^{\dagger 2}\right)\right]$$

which transforms the quadratures as $\hat{S}^\dagger \hat{x} \hat{S} = e^{-r}\hat{x}$ and $\hat{S}^\dagger \hat{p} \hat{S} = e^{+r}\hat{p}$. The squeezed vacuum $\hat{S}(r)|0\rangle$ therefore has

$$\Delta x = \frac{e^{-r}}{\sqrt{2}}, \qquad \Delta p = \frac{e^{+r}}{\sqrt{2}}, \qquad \Delta x\,\Delta p = \frac{1}{2}$$

still a minimum-uncertainty state, but with an elliptical noise contour. Squeezing is quantified in decibels relative to vacuum:

$$S_{\text{dB}} = -10\log_{10}\frac{(\Delta x)^2}{(\Delta x)^2_{\text{vac}}} = \frac{20\, r}{\ln 10} \approx 8.69\, r$$

**Worked example.** The world record for optical squeezing is 15 dB, achieved by Vahlbruch, Mehmet, Danzmann, and Schnabel in 2016 with a below-threshold optical parametric oscillator at 1064 nm [2]. The squeezing parameter is $r = 15/8.69 = 1.73$, i.e., quadrature fluctuations reduced to $10^{-15/20} = 0.178$ of the vacuum standard deviation ($3.2\%$ of the vacuum variance). Reaching this required total optical losses below about 2.5%: a lost fraction $\eta_{\text{loss}}$ of the light replaces squeezed noise with vacuum noise, $(\Delta x)^2 \to (1-\eta_{\text{loss}})(\Delta x)^2_{\text{sq}} + \eta_{\text{loss}}\cdot\frac{1}{2}$, so even 5% loss caps observable squeezing at $\approx 13$ dB no matter how strong the source. *Loss, not pump power, is the binding constraint on squeezing* — a lesson that carries directly over to integrated photonics, where waveguide losses currently limit on-chip squeezing to several dB. Squeezed light is not merely a laboratory curiosity: GEO 600 has used it since 2010, and Advanced LIGO since 2019, to push interferometric strain sensitivity below the shot-noise limit.

Physically, squeezed vacuum is generated by degenerate parametric down-conversion (Chapter 18): a $\chi^{(2)}$ crystal pumped at $2\omega$ emits photon pairs at $\omega$, and the Hamiltonian $\hat{H} \propto i(\hat{a}^{\dagger 2} - \hat{a}^2)$ is precisely the generator of $\hat{S}(r)$. Because pair emission is a parametric process driven by a classical pump, the squeezing is produced *deterministically* — every pulse, no heralding — which is the foundational advantage of the CV platform. Note that squeezed vacuum contains photons ($\langle \hat{n}\rangle = \sinh^2 r$, about 2.7 photons at 15 dB) and only *even* photon numbers, since photons are created in pairs.

## Homodyne Detection: Measuring a Quadrature

Quadratures are measured by *balanced homodyne detection*. The signal mode $\hat{a}$ interferes on a 50:50 beam splitter with a strong coherent *local oscillator* (LO) $\beta = |\beta|e^{i\theta}$ at the same frequency. The two output ports are detected on ordinary photodiodes and the photocurrents subtracted:

$$\hat{n}_- = \hat{n}_1 - \hat{n}_2 = \hat{a}^\dagger\hat{b} + \hat{b}^\dagger\hat{a} \;\xrightarrow{\;\hat{b}\to\beta\;}\; |\beta|\left(\hat{a}e^{-i\theta} + \hat{a}^\dagger e^{i\theta}\right) = \sqrt{2}\,|\beta|\,\hat{x}_\theta$$

The subtracted photocurrent is directly proportional to the rotated quadrature $\hat{x}_\theta$, with the LO phase $\theta$ selecting *which* quadrature is measured. The LO plays two roles: it provides the phase reference that makes "quadrature" meaningful, and it amplifies the microscopic quantum signal to a macroscopic photocurrent, swamping electronic noise. Homodyne detection is the most nearly ideal measurement in quantum physics: silicon and InGaAs photodiodes reach quantum efficiencies above 99%, bandwidths extend to many GHz, and the whole apparatus works at room temperature. Contrast this with the single-photon detectors of Chapter 19, which achieve $\sim 98\%$ efficiency only in superconducting devices at sub-kelvin temperatures.

*Heterodyne* (or double-homodyne / "dual-rail" homodyne) detection splits the signal 50:50 and homodynes the two halves with LO phases $\theta$ and $\theta + \pi/2$, measuring $\hat{x}$ and $\hat{p}$ *simultaneously*. Since $\hat{x}$ and $\hat{p}$ do not commute, this cannot be noiseless: the vacuum entering the splitting beam splitter adds exactly one extra unit of vacuum noise to each outcome, so each quadrature is obtained with variance $(\Delta x)^2 + \frac{1}{2}$. Heterodyne implements the measurement of $\hat{a}$ itself (a projection onto coherent states) and realizes the optimal simultaneous phase-amplitude measurement permitted by the uncertainty principle.

## Why Quadratures Are Good Computational Variables

Three features make quadratures attractive carriers of quantum information:

1. **Deterministic sources.** Squeezed states — the universal entanglement resource in CV — are generated on demand by parametric amplifiers, unlike heralded single photons.
2. **Near-perfect measurement.** Homodyne detection is fast, efficient, room-temperature, and reads out a continuous value, providing "analog syndrome information" that error correction can exploit (Section 21.1.3).
3. **Natural match to linear optics.** Beam splitters and phase shifters act on quadratures as simple linear (symplectic) transformations — the subject of the next subsection.

The corresponding weakness is that finite squeezing is finite: $e^{-r} > 0$ always, so every CV operation inherits a Gaussian noise floor $\propto e^{-2r}$, and errors are continuous rather than discrete. Managing that noise — ultimately by encoding qubits into oscillators — is the central theme of the rest of this chapter.

## Summary

- Quadratures $\hat{x} = (\hat{a}+\hat{a}^\dagger)/\sqrt{2}$, $\hat{p} = (\hat{a}-\hat{a}^\dagger)/i\sqrt{2}$ obey $[\hat{x},\hat{p}] = i$; they are the field's in-phase and quadrature amplitudes relative to a phase reference.
- Vacuum: $\Delta x = \Delta p = 1/\sqrt{2}$ (shot noise, 0 dB). Coherent states displace the vacuum disk without changing its noise.
- Squeezed states: $\Delta x = e^{-r}/\sqrt{2}$ below vacuum; record $15$ dB ($r \approx 1.73$); loss is the limiting factor.
- Homodyne detection measures $\hat{x}_\theta$ with $>99\%$ efficiency at room temperature; heterodyne measures $\hat{x}$ and $\hat{p}$ jointly at the cost of one added vacuum unit.
- CV strengths: deterministic resources and near-ideal detection. CV weakness: finite squeezing acts as ever-present Gaussian noise.

---

*References*

[1] Weedbrook, C., Pirandola, S., García-Patrón, R., Cerf, N.J., Ralph, T.C., Shapiro, J.H., & Lloyd, S. (2012). Gaussian quantum information. *Reviews of Modern Physics*, 84(2), 621–669. [DOI: 10.1103/RevModPhys.84.621] [The standard modern reference for quadrature conventions and Gaussian formalism.]

[2] Vahlbruch, H., Mehmet, M., Danzmann, K., & Schnabel, R. (2016). Detection of 15 dB squeezed states of light and their application for the absolute calibration of photoelectric quantum efficiency. *Physical Review Letters*, 117(11), 110801. [DOI: 10.1103/PhysRevLett.117.110801] [The squeezing world record.]

[3] Braunstein, S.L. & van Loock, P. (2005). Quantum information with continuous variables. *Reviews of Modern Physics*, 77(2), 513–577. [DOI: 10.1103/RevModPhys.77.513]

[4] Loudon, R. (2000). *The Quantum Theory of Light*, 3rd ed. Oxford University Press. [Chapters 5–6 treat quadratures, squeezing, and homodyne detection in detail.]

[5] Tse, M., et al. (LIGO Scientific Collaboration) (2019). Quantum-enhanced Advanced LIGO detectors in the era of gravitational-wave astronomy. *Physical Review Letters*, 123(23), 231107. [DOI: 10.1103/PhysRevLett.123.231107] [Squeezed light deployed in LIGO.]
