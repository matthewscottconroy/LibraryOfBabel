# Chapter 17: Exercises

---

## Section 17.1 — Maxwell's Equations

**17.1.1.** *Deriving displacement current from charge conservation.*

Ampère's original law (without displacement current) was $\nabla\times\mathbf{B} = \mu_0\mathbf{J}$.

(a) Take the divergence of both sides and use the identity $\nabla\cdot(\nabla\times\mathbf{B}) = 0$ to show that Ampère's law without displacement current implies $\nabla\cdot\mathbf{J} = 0$, i.e., only steady currents are allowed.

(b) The continuity equation (charge conservation) is $\partial\rho/\partial t + \nabla\cdot\mathbf{J} = 0$. In a non-steady situation, $\partial\rho/\partial t \neq 0$. Maxwell fixed Ampère's law by adding $\varepsilon_0\partial\mathbf{E}/\partial t$ to the right side. Show that the modified law is consistent with charge conservation by using Gauss's law $\nabla\cdot\mathbf{E} = \rho/\varepsilon_0$.

(c) A parallel-plate capacitor is being charged. Current $I$ flows into one plate. Between the plates, there is no current ($\mathbf{J} = 0$), but the electric field is changing. Compute the displacement current density between the plates and show it equals $I/A$ (where $A$ is the plate area). Draw an Amperian loop around the wire and around the gap — in both cases, the enclosed "current" (real + displacement) is the same.

(d) Evaluate the displacement current density $\varepsilon_0\partial\mathbf{E}/\partial t$ inside a radio transmitter at 100 MHz with peak electric field $E_0 = 1000$ V/m. Compare this to a realistic real current density.

---

**17.1.2.** *The electromagnetic wave equation from Maxwell.*

Starting from Maxwell's equations in vacuum ($\rho = 0$, $\mathbf{J} = 0$):

(a) Take the curl of Faraday's law $\nabla\times\mathbf{E} = -\partial\mathbf{B}/\partial t$ and use the identity $\nabla\times(\nabla\times\mathbf{V}) = \nabla(\nabla\cdot\mathbf{V}) - \nabla^2\mathbf{V}$ to derive $\nabla^2\mathbf{E} = \mu_0\varepsilon_0\partial^2\mathbf{E}/\partial t^2$.

(b) Show that the wave speed is $c = 1/\sqrt{\mu_0\varepsilon_0}$. Insert SI values: $\mu_0 = 4\pi\times 10^{-7}$ N/A$^2$, $\varepsilon_0 = 8.854\times 10^{-12}$ C$^2$/(N·m$^2$). Verify that $c \approx 3\times 10^8$ m/s.

(c) Maxwell (1865) noted that $1/\sqrt{\mu_0\varepsilon_0}$ agreed with the measured speed of light and concluded that "light is an electromagnetic disturbance propagated through the field according to electromagnetic laws." This was one of the great unifications in physics. What two previously separate phenomena were unified?

(d) A plane wave $\mathbf{E} = E_0\hat{x}\cos(kz - \omega t)$ satisfies the wave equation. Verify this. Find the corresponding $\mathbf{B}$ field from Faraday's law. Show that $\mathbf{E}$ and $\mathbf{B}$ are perpendicular to each other and to the propagation direction, and that $|\mathbf{B}| = |\mathbf{E}|/c$.

---

**17.1.3.** *Maxwell's equations in differential form language.*

The electromagnetic field is a 2-form $F = \frac{1}{2}F_{\mu\nu}dx^\mu\wedge dx^\nu$ on Minkowski spacetime.

(a) Write out all six independent components of $F_{\mu\nu}$ in terms of $E_x, E_y, E_z, B_x, B_y, B_z$.

(b) Show that $dF = 0$ is equivalent to the two homogeneous Maxwell equations: $\nabla\cdot\mathbf{B} = 0$ and $\nabla\times\mathbf{E} + \partial\mathbf{B}/\partial t = 0$.

(c) The Hodge dual $\star F$ in Minkowski space (with metric signature $-+++$) has components $(\star F)_{\mu\nu} = \frac{1}{2}\varepsilon_{\mu\nu\rho\sigma}F^{\rho\sigma}$. Show that $\star F$ has $B$-fields where $F$ had $E$-fields and $-E$-fields where $F$ had $B$-fields (the duality transformation $\mathbf{E}\to c\mathbf{B}$, $c\mathbf{B}\to -\mathbf{E}$).

(d) Show that $d\star F = \mu_0\star J$ (where $J = J^\mu\partial_\mu$ and $\star J$ is the corresponding 3-form) is equivalent to the two inhomogeneous Maxwell equations: $\nabla\cdot\mathbf{E} = \rho/\varepsilon_0$ and $\nabla\times\mathbf{B} - \mu_0\varepsilon_0\partial\mathbf{E}/\partial t = \mu_0\mathbf{J}$.

---

**17.1.4.** *Maxwell in curved spacetime.*

In curved spacetime with metric $g_{\mu\nu}$, Maxwell's equations become $\nabla_\mu F^{\mu\nu} = \mu_0 J^\nu$ and $\nabla_{[\rho}F_{\mu\nu]} = 0$, or equivalently $\frac{1}{\sqrt{-g}}\partial_\mu(\sqrt{-g}F^{\mu\nu}) = \mu_0 J^\nu$.

(a) Show that $\nabla_\mu F^{\mu\nu} = \frac{1}{\sqrt{-g}}\partial_\mu(\sqrt{-g}F^{\mu\nu})$ for antisymmetric tensors. (This is a consequence of the contracted Christoffel identity $\Gamma^\mu_{\mu\nu} = \partial_\nu\ln\sqrt{-g}$.)

(b) The homogeneous equation $\nabla_{[\rho}F_{\mu\nu]} = 0$ is equivalent to $\partial_{[\rho}F_{\mu\nu]} = 0$ (for the Levi-Civita connection). Why? (Hint: the Christoffel symbols cancel by symmetry when you antisymmetrize.)

(c) Consider a static spherically symmetric star with Schwarzschild metric outside. A magnetic monopole is placed at the center. By symmetry, the field must be $F_{r0} = 0$ and $F_{\theta\phi} = g_m\sin\theta$ (for some constant $g_m$). Verify that this satisfies both curved-space Maxwell equations in the vacuum region.

(d) Electromagnetic energy is gravitational source — the stress-energy tensor $T^{\mu\nu}_{\rm EM} = F^{\mu\alpha}F^\nu_{\ \alpha} - \frac{1}{4}g^{\mu\nu}F_{\alpha\beta}F^{\alpha\beta}$ sources the Einstein equations. A Reissner-Nordström black hole has both mass $M$ and charge $Q$. What do you expect happens to the Schwarzschild radius $r_s = 2GM/c^2$ when the black hole is charged?

---

## Section 17.2 — Electromagnetic Waves

**17.2.1.** *Poynting's theorem and electromagnetic energy.*

(a) Starting from Maxwell's equations, derive Poynting's theorem:
$$\frac{\partial u}{\partial t} + \nabla\cdot\mathbf{S} = -\mathbf{J}\cdot\mathbf{E}$$
where $u = \frac{1}{2}(\varepsilon_0 E^2 + B^2/\mu_0)$ is the electromagnetic energy density and $\mathbf{S} = \mathbf{E}\times\mathbf{B}/\mu_0$ is the Poynting vector.

(b) Interpret each term physically. The term $-\mathbf{J}\cdot\mathbf{E}$ is the rate of work done by the field on charges (Joule heating). The theorem says: the rate of change of electromagnetic energy = energy flux into the region + work done on charges.

(c) For the plane wave $\mathbf{E} = E_0\hat{x}\cos(kz-\omega t)$, $\mathbf{B} = (E_0/c)\hat{y}\cos(kz-\omega t)$: compute the time-averaged Poynting vector $\langle\mathbf{S}\rangle$ and time-averaged energy density $\langle u\rangle$. Show that $|\langle\mathbf{S}\rangle| = c\langle u\rangle$ — electromagnetic energy travels at speed $c$.

(d) The Sun delivers about 1360 W/m$^2$ to the top of Earth's atmosphere. Estimate the amplitude of the electric and magnetic fields of sunlight at Earth's distance. How does this compare to household electrical fields?

---

**17.2.2.** *The Larmor formula and gravitational wave analogy.*

An accelerating charge $q$ with acceleration $\mathbf{a}$ radiates electromagnetic power:
$$P_{\rm EM} = \frac{q^2 a^2}{6\pi\varepsilon_0 c^3}$$

An accelerating mass distribution (changing mass quadrupole $Q_{ij}$) radiates gravitational waves with power:
$$P_{\rm GW} = \frac{G}{5c^5}\dddot{Q}_{ij}\dddot{Q}^{ij}$$

(a) What is the EM analog of the mass quadrupole? Why is there no dipole gravitational radiation (while there is EM dipole radiation)?

(b) A 1 kg ball oscillates on a spring with amplitude $A = 0.1$ m and frequency $f = 1$ Hz. Estimate $P_{\rm GW}$. Compare to the Larmor power radiated by a proton ($q = e$, $m = m_p$) undergoing the same oscillation.

(c) The binary pulsar PSR B1913+16 has two neutron stars of mass $\sim 1.4 M_\odot$ in an 8-hour orbit with semi-major axis $\sim 2\times 10^9$ m. Estimate $P_{\rm GW}$ from the quadrupole formula. How does this compare to the Sun's luminosity $L_\odot \approx 4\times 10^{26}$ W? At what rate is the orbital period decreasing?

(d) At the final moment of black hole merger (when GW150914 was produced), $P_{\rm GW} \sim 10^{49}$ W (for about 0.01 seconds). How does this compare to the combined luminosity of all stars in the observable universe ($\sim 10^{43}$ W)?

---

## Thought Experiments

**T17.1.** *Maxwell's demon and the second law.*

Maxwell (1871) proposed a thought experiment: a demon sitting at a trap door between two chambers of gas, allowing fast molecules to pass one way and slow molecules the other. This would decrease entropy without doing work, violating the second law of thermodynamics.

In 1929, Leó Szilárd showed the demon must *measure* molecular speeds — and measurement requires acquiring information that must eventually be erased, which costs at least $k_B T\ln 2$ of work per bit. Rolf Landauer (1961) proved this rigorously.

Now connect this to electromagnetism: a photon carrying information has energy $E = hf$. The minimum photon energy for a measurement at temperature $T$ is $E_{\rm min} \sim k_B T$. Show that this implies a minimum measurement frequency $f_{\rm min} = k_B T/h$. At room temperature ($T = 300$ K), what is this frequency? What part of the electromagnetic spectrum does it correspond to?

---

**T17.2.** *Why does radiation carry momentum?*

Electromagnetic waves carry both energy (Poynting vector) and momentum (radiation pressure). The momentum density is $\mathbf{g} = \mathbf{S}/c^2 = \varepsilon_0(\mathbf{E}\times\mathbf{B})$.

Imagine shining a flashlight in space. The photons carry momentum $p = E/c$, so the flashlight recoils. This is the photon rocket principle. But classically, where does the momentum of radiation "come from"?

The answer is that radiation reacts back on the source. When a charge is accelerated to produce radiation, the radiation field exerts a back-reaction force — the Abraham-Lorentz radiation reaction force $\mathbf{F} = \frac{\mu_0 q^2}{6\pi c}\dot{\mathbf{a}}$. This force is proportional to the *jerk* (time derivative of acceleration), not the acceleration itself.

Why is this strange, physically? (Hint: consider what happens if the charge is released at $t = 0$ with no external force but a nonzero acceleration — does the Abraham-Lorentz force cause problems?)

---

## Laboratory Exercise: Measuring the Speed of Light

**L17.1.** *Time-domain measurement using light pulses.*

The speed of light can be measured by sending short laser pulses to a retroreflector and measuring the round-trip time. This is exactly how LIDAR and laser ranging work (the same technique was used to measure the Earth-Moon distance to centimeter precision).

**Setup:** A pulsed laser, a retroreflector 1 m away, a fast oscilloscope, and a beamsplitter.

**Procedure:**
1. Direct a short laser pulse (ideally picosecond regime; nanosecond is fine) at a retroreflector.
2. Use a beamsplitter to direct a reference pulse to one channel of the oscilloscope and the reflected pulse to another channel.
3. Measure the time delay $\Delta t$ between pulses.
4. The speed of light is $c = 2L/\Delta t$.

**Analysis:** With a 1 m path ($L = 1$ m), the expected delay is $\Delta t = 2L/c \approx 6.67$ ns. A modern oscilloscope with 1 GHz bandwidth can resolve this easily. What systematic errors affect the measurement? How would you improve precision to 1 part in $10^4$?

**Alternative (low-cost):** Using a picosecond-pulse LED module and a photodetector with a fast oscilloscope, repeat the measurement. Many undergraduate labs have performed this with accuracy to 1% or better.

