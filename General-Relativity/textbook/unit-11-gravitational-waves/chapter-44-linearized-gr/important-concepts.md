# Chapter 44: Important Concepts

---

## Linearization of the Einstein Equations

The full Einstein equations $G_{\mu\nu} = 8\pi G T_{\mu\nu}/c^4$ are ten coupled nonlinear PDEs. For weak fields, writing $g_{\mu\nu} = \eta_{\mu\nu} + h_{\mu\nu}$ with $|h_{\mu\nu}|\ll 1$ and expanding to first order in $h$ gives a linear theory that is exactly solvable. The linearized equations are valid when:
- Velocities: $v \ll c$
- Gravitational potential: $|\Phi/c^2| \ll 1$
- Wavelengths: much smaller than the curvature scale of the background

Linearized GR is the bridge between the full nonlinear theory and tractable analytic solutions. The Newtonian limit, post-Newtonian expansion, and gravitational wave theory all live in this linearized framework.

---

## The Lorenz Gauge

The gauge freedom in linearized GR (infinitesimal coordinate transformations $x^\mu \to x^\mu + \xi^\mu$) is analogous to gauge freedom in electromagnetism. The **Lorenz gauge** $\partial^\mu\bar{h}_{\mu\nu} = 0$ (where $\bar{h}_{\mu\nu} = h_{\mu\nu} - \frac{1}{2}\eta_{\mu\nu}h$ is the trace-reversed perturbation) is the GR analog of the electromagnetic Lorenz gauge $\partial^\mu A_\mu = 0$. In this gauge, the linearized Einstein equations simplify to:
$$\Box\bar{h}_{\mu\nu} = -\frac{16\pi G}{c^4}T_{\mu\nu}$$

This is a wave equation with source. In vacuum, $T_{\mu\nu} = 0$, it becomes the free wave equation $\Box\bar{h}_{\mu\nu} = 0$, with plane wave solutions propagating at $c$.

---

## Gravitational Waves as Spacetime Ripples

Gravitational waves are not waves **in** spacetime in the sense that sound waves are waves in air. They are waves **of** spacetime — the metric itself oscillates. The spacetime interval $ds^2 = g_{\mu\nu}dx^\mu dx^\nu$ oscillates in time. Two points at fixed coordinate separation have varying proper distance.

This is a genuinely relativistic phenomenon: in Newtonian gravity, the gravitational field propagates instantaneously (action at a distance). In GR, changes in the gravitational field propagate at $c$ as gravitational waves. This is required by special relativity — no physical influence can propagate faster than light.

---

## Transverse-Traceless (TT) Gauge

Residual gauge freedom within the Lorenz gauge allows the further conditions:
- **Transverse**: $k^i A_{ij} = 0$ (the wave oscillates perpendicular to its propagation direction)
- **Traceless**: $A^i_{\ i} = 0$ (the trace of the spatial part vanishes)
- **Time components vanish**: $A_{0\mu} = 0$

The TT gauge makes the physical content manifest: only 2 independent polarization states ($h_+$ and $h_\times$) survive. These represent the genuine, gauge-invariant degrees of freedom of gravitational radiation. The 8 remaining components are pure gauge — coordinate effects that can be transformed away.

---

## The Two Polarization States

The two polarization states of gravitational waves are:
- **Plus polarization** ($h_+$): stretches one transverse axis while compressing the perpendicular one
- **Cross polarization** ($h_\times$): same as plus but rotated $45°$

These are related by a $45°$ rotation, not a $90°$ rotation as for electromagnetic waves. This $45°$ relationship is a direct consequence of the spin-2 nature of the gravitational field: under a rotation by angle $\psi$ about the propagation direction, the polarization tensors transform with phase $e^{\pm 2i\psi}$ (helicity $\pm 2$). For comparison, EM polarization vectors transform with phase $e^{\pm i\psi}$ (helicity $\pm 1$).

---

## Gravitational Waves and Test Masses: The LIGO Measurement

A gravitational wave does not directly accelerate a free-falling test mass — in the local inertial frame of the mass, it remains at rest to first order. What the wave does is change the **proper distance** between two nearby masses. For the $+$ polarization:
$$\frac{\delta L}{L} = \frac{1}{2}h_+$$

The strain $h_+$ is the fractional length change — a dimensionless number. For GW150914: $h_+ \sim 10^{-21}$, $L = 4$ km, giving $\delta L \sim 2\times 10^{-18}$ m. This is the displacement LIGO must measure, and it is $1/500$ the diameter of a proton.

---

## The Quadrupole Formula: No Monopole, No Dipole

Gravitational radiation from a localized source is dominated by the mass quadrupole moment $Q^{ij}$. There is no monopole radiation (mass conservation) and no dipole radiation (momentum conservation). This contrasts with electromagnetism, where dipole radiation ($\ddot{p}^i \neq 0$ for charge distributions) is the leading term.

The absence of gravitational dipole radiation is one reason gravitational waves are so weak compared to electromagnetic radiation from charged accelerating sources of comparable energy.

The leading radiation:
$$P = \frac{G}{5c^5}\dddot{Q}_{ij}\dddot{Q}^{ij}$$

The factor $G/c^5 \approx 3.6\times 10^{-53}$ W$^{-1}$ — the characteristic "gravitational luminosity" scale — is so small that only the most violent astrophysical events produce detectable radiation.

---

## Gravitational Wave Frequency and Sources

The gravitational wave frequency from a source equals **twice** the orbital frequency of a binary system: $f_{\rm GW} = 2f_{\rm orbital}$. This is because the mass quadrupole tensor $Q^{ij}$ for a binary in circular orbit oscillates at $2\Omega$ (the orbital frequency appears squared in $Q^{ij} = \mu a^2 \cos(\Omega t)\cdots$, so the oscillation frequency doubles).

This factor of 2 has observable consequences: LIGO detects binary black holes at $\sim 35$–$150$ Hz while they are orbiting each other at $\sim 17$–$75$ Hz.

---

## Energy Carried by Gravitational Waves

Gravitational wave energy is a second-order (in $h$) effect. This is why it cannot be localized in a gauge-invariant way — at linear order, GW energy can be transformed away by a coordinate change. At second order (the Isaacson tensor), the energy is well-defined after averaging over several wavelengths.

The energy flux from a gravitational wave:
$$S = \frac{c^3\omega^2}{32\pi G}(A_+^2 + A_\times^2)$$

For GW150914 at Earth ($h \sim 10^{-21}$, $f = 150$ Hz): $S \sim 2\times 10^{-4}$ W/m². Though tiny at Earth, the total power radiated at the source peaked at $\sim 3.6\times 10^{49}$ W — exceeding the electromagnetic luminosity of all visible stars in the observable universe.

---

## Validity of the Linearized Approximation

Linearized GR is valid when $h_{\mu\nu} \ll 1$. For a source of mass $M$ and size $R$, moving with velocity $v$, the typical strain scales as:
$$h \sim \frac{R_s}{r}\left(\frac{v}{c}\right)^2 = \frac{2GM}{rc^2}\left(\frac{v}{c}\right)^2$$

For LIGO sources: $M \sim 30 M_\odot$, $r \sim 10^3$ Mpc, $v \sim 0.3c$ at merger, so $h \sim 10^{-21}$ — indeed $\ll 1$ at Earth. Near the source itself, at $r \sim GM/c^2$, the linearized approximation breaks down and numerical relativity is needed. This is why the merger phase (which dominates the waveform's amplitude) cannot be computed from linearized GR alone.
