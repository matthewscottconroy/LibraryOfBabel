# 1.2.3 Faraday's Law of Induction

## The Equation

$$\oint_C \mathbf{E} \cdot d\boldsymbol{\ell} = -\frac{d}{dt}\int_S \mathbf{B} \cdot d\mathbf{A}$$

The left side is the *electromotive force* (EMF) around a closed loop $C$ — the work done per unit charge in moving a positive test charge around the loop. The right side is the negative rate of change of the magnetic flux through any surface $S$ bounded by the loop $C$.

## The Key Physical Content

In electrostatics, the electric field is conservative: the line integral of $\mathbf{E}$ around any closed loop is zero. (This follows from the fact that $\mathbf{E}$ can be written as minus the gradient of a potential: $\mathbf{E} = -\nabla V$.) Faraday's law says that when the magnetic flux through a loop is changing, the electric field is *no longer conservative*: its line integral around a closed loop is not zero.

This is a fundamental departure from the static picture. The induced electric field "circulates" around the region of changing magnetic flux. It is this circulating electric field that drives the current in a generator, and it is the mutual interplay of circulating electric and circulating magnetic fields that constitutes an electromagnetic wave.

## Why the Negative Sign: Lenz's Law

The negative sign is physically significant. It expresses Lenz's law: the induced EMF opposes the change in flux that produced it.

If the magnetic flux through a loop is increasing, the induced EMF drives a current that creates a magnetic field opposing that increase. If the flux is decreasing, the induced current creates a field supporting it. Either way, the induced current *resists the change*.

This is energy conservation at work. To increase the flux through a coil (by pushing a magnet toward it, say), you must do work against the force exerted by the induced current on the magnet. That work goes into the electrical energy of the current. If Lenz's law were reversed — if the sign were positive instead of negative — the induced current would *reinforce* the flux change, leading to a runaway amplification and a violation of energy conservation.

**Worked example: sinusoidal magnetic field.** Suppose the magnetic field through a flat coil of area $A$ varies as $B(t) = B_0 \sin(\omega t)$. The flux is $\Phi = B_0 A \sin(\omega t)$. Faraday's law gives:

$$\mathcal{E} = -\frac{d\Phi}{dt} = -B_0 A \omega \cos(\omega t)$$

The induced EMF is 90° out of phase with the driving magnetic field. This phase relationship is important in electromagnetic wave propagation, where the electric and magnetic fields are also 90° out of phase in their time derivatives.

## Faraday's Law as the Engine of Wave Propagation

Faraday's law — changing $\mathbf{B}$ creates $\mathbf{E}$ — is one of two "coupling" equations in Maxwell's set. The other is the Ampère-Maxwell law — changing $\mathbf{E}$ creates $\mathbf{B}$. Together, these two equations create the feedback loop that sustains an electromagnetic wave:

1. An oscillating electric field creates an oscillating magnetic field (Ampère-Maxwell).
2. The oscillating magnetic field creates an oscillating electric field that propagates the pattern forward (Faraday).
3. Repeat indefinitely.

This is the mechanism of light propagation. It requires no medium, because the fields generate each other. It propagates at a speed determined solely by $\varepsilon_0$ and $\mu_0$ — the properties of free space. We derive this rigorously in Section 1.4.

## Relevance to Photonic Devices

Faraday's law is the physical basis of:
- **Optical modulators**: changing the effective "magnetic" (more precisely, the optical path) properties of a material by applying an electric or magnetic field modulates the phase of a propagating wave.
- **Sensing**: fiber-optic current sensors measure the Faraday rotation of polarization caused by a magnetic field — a direct application of Faraday's law in a dielectric medium [1].
- **Nonreciprocal devices**: optical isolators (which allow light to travel in only one direction) exploit the Faraday effect in magneto-optic materials to break time-reversal symmetry [2].

---

## References

[1] Smith, A.M. (1978). "Polarization and magnetooptic properties of single-mode optical fiber." *Applied Optics*, 17(1), 52–56.

[2] Aplet, L.J., & Carson, J.W. (1964). "A Faraday effect optical isolator." *Applied Optics*, 3(4), 544–545. [Early demonstration of the optical isolator principle.]
