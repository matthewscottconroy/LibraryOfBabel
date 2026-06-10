# 3.3.5 — Optical Solitons

## The Puzzle of the Dispersion-Free Pulse

A pulse propagating in a dispersive medium spreads. Different frequency components travel at different speeds, and the pulse that was compact at the input becomes broad and chirped at the output. In a normally dispersive medium, the blue components (higher frequency) lag behind the red components; in anomalously dispersive media, they lead. The broadening is governed by the dispersion length $L_D = T_0^2/|\beta_2|$, where $T_0$ is the pulse duration and $\beta_2$ is the GVD.

In a nonlinear medium, the Kerr effect creates a phase shift proportional to instantaneous intensity. For a pulse with a Gaussian time profile, this phase shift is also Gaussian in time — and corresponds (Section 3.3.3) to a *downchirp* in the anomalous GVD regime: the leading edge is phase-advanced (red-shifted) and the trailing edge is phase-retarded (blue-shifted). 

Wait — in anomalous dispersion, the blue components are faster. If the Kerr effect creates blue components at the front (which should travel faster) and red at the back, then the nonlinear-induced chirp would tend to *compress* the pulse that dispersion tends to spread. Could there be an exact balance?

Yes. This is the optical *soliton*.

## The Nonlinear Schrödinger Equation

The propagation of a pulse envelope $A(z,t)$ (with the carrier $e^{i\beta_0 z - i\omega_0 t}$ factored out) in a fiber with GVD $\beta_2$ and Kerr nonlinearity $\gamma$ is described by the *nonlinear Schrödinger equation* (NLSE):

$$i\frac{\partial A}{\partial z} = \frac{\beta_2}{2}\frac{\partial^2 A}{\partial t^2} - \gamma|A|^2 A$$

(in a frame moving at the group velocity, with loss neglected). The two terms on the right:
- $\frac{\beta_2}{2}\frac{\partial^2 A}{\partial t^2}$: dispersion — tends to spread the pulse in time.
- $-\gamma|A|^2 A$: Kerr nonlinearity — creates intensity-dependent phase.

This equation is formally identical to the Schrödinger equation for a nonlinear quantum mechanical problem — hence "nonlinear Schrödinger equation." It was solved exactly by Zakharov and Shabat (1972) [1] using the inverse scattering transform, which revealed that the NLSE has exact soliton solutions.

## The Fundamental Soliton

For anomalous dispersion ($\beta_2 < 0$) and positive $\gamma > 0$, the NLSE has a family of *bright soliton* solutions. The fundamental (N=1) soliton:

$$A(z,t) = A_0 \, \text{sech}\!\left(\frac{t}{T_0}\right) e^{iz/(2L_D)}$$

where $A_0^2 = |\beta_2|/(\gamma T_0^2)$ (the peak power determined by the pulse width and material parameters), $L_D = T_0^2/|\beta_2|$, and the $\text{sech}(t/T_0)$ temporal profile is the hyperbolic secant.

The key property: **this pulse propagates without change in shape, forever** (in the absence of loss). The dispersion and nonlinearity exactly cancel. The Kerr-induced chirp exactly compensates the GVD-induced spreading.

The *soliton number* $N$ is defined by:

$$N^2 = \frac{L_D}{L_\text{NL}} = \frac{\gamma P_0 T_0^2}{|\beta_2|}$$

The $N=1$ (fundamental) soliton propagates undistorted. $N > 1$ (*higher-order solitons*) undergo periodic breathing — the pulse shape changes periodically with the *soliton period* $z_0 = \pi L_D/2$, but returns exactly to its initial shape after each period.

**Worked example**: Standard single-mode fiber at 1550 nm has $\beta_2 = -20$ ps²/km and $\gamma = 1.3 \times 10^{-3}$ W⁻¹m⁻¹. For a 1 ps pulse ($T_0 = 1$ ps):

$$L_D = \frac{(10^{-12})^2}{20 \times 10^{-27}} = 50 \text{ m}$$

$$P_0 = \frac{|\beta_2|}{\gamma T_0^2} = \frac{20 \times 10^{-27}}{1.3 \times 10^{-3} \times 10^{-24}} = 15.4 \text{ mW}$$

A 1 ps, 15.4 mW peak power pulse forms a fundamental soliton at 1550 nm in standard SMF.

## Soliton Properties

**Robustness**: Solitons are *topologically protected* — they emerge intact from perturbations, shedding excess energy as radiation. If you launch a pulse close to the soliton conditions, it adjusts itself to become a soliton (absorbing or emitting a dispersive wave to reach the exact $N=1$ condition). This robustness makes solitons attractive for long-distance communications.

**Gordon-Haus jitter**: In reality, amplified spontaneous emission (ASE) noise from amplifiers causes small random shifts in the soliton frequency, which translate (via the soliton's group velocity dependence on frequency) into random position jitter. This *Gordon-Haus jitter* [2] was the major limiting factor for soliton communications systems before coherent detection enabled electronic dispersion compensation.

**Soliton collisions**: Two solitons of different frequencies (in a WDM system) pass through each other without loss of identity — a consequence of the integrability of the NLSE. They emerge from the collision shifted in position and phase, but otherwise unchanged. This collision-based picture inspired some proposals for soliton-based all-optical computing.

## Solitons in Microresonators

In microresonators (ring resonators), a similar balance can occur: the Kerr nonlinearity and the anomalous dispersion of the resonator can support *dissipative Kerr solitons* — stable, localized waveforms in the resonator that correspond to Kerr frequency combs with a coherent comb spectrum [3].

Dissipative solitons differ from conservative fiber solitons: they are maintained against loss by the pump laser, and their properties (soliton step in the pump power tuning curve, characteristic triangular spectrum) are understood via the Lugiato-Lefever equation (the NLSE plus loss and pump terms).

For photonic computing: Kerr soliton microcombs (Section 3.3.4) provide the multi-wavelength source for WDM-based photonic neural networks. The coherence of the comb (all lines derived from a single pump laser) ensures mutual coherence between wavelength channels — a prerequisite for coherent photonic computing using multiple wavelengths.

## Summary

- Optical solitons arise when Kerr self-phase modulation (in anomalous GVD) exactly compensates GVD-induced spreading.
- Fundamental soliton: $A(z,t) = A_0\text{sech}(t/T_0)e^{iz/(2L_D)}$ — propagates undistorted.
- Soliton number $N = \sqrt{L_D/L_\text{NL}}$; $N=1$ for fundamental soliton.
- Robust and immune to perturbations; important for long-distance fiber communications.
- Dissipative Kerr solitons in microresonators produce coherent frequency combs for WDM photonic computing.

---

*References*

[1] Zakharov, V.E. & Shabat, A.B. (1972). Exact theory of two-dimensional self-focusing and one-dimensional self-modulation of waves in nonlinear media. *Soviet Physics JETP*, 34(1), 62–69.

[2] Gordon, J.P. & Haus, H.A. (1986). Random walk of coherently amplified solitons in optical fiber transmission. *Optics Letters*, 11(10), 665–667. [DOI: 10.1364/OL.11.000665]

[3] Herr, T. et al. (2014). Temporal solitons in optical microresonators. *Nature Photonics*, 8(2), 145–152. [DOI: 10.1038/nphoton.2013.343]
