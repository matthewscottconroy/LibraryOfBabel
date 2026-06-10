# Section 3.3: Nonlinear Optics

The physics we have developed so far is entirely linear: the polarization $\mathbf{P}$ is proportional to $\mathbf{E}$, Maxwell's equations are linear, and superposition holds exactly. This is an excellent approximation at low light intensities — the intensities encountered in everyday optics and most telecommunications. But when the optical field becomes intense, the electron's response to the driving field departs from the linear (harmonic oscillator) approximation, and new phenomena emerge.

How intense must the field be? The natural comparison is the atomic electric field that binds the electron: $E_\text{atom} \sim e/(4\pi\varepsilon_0 a_0^2) \approx 5 \times 10^{11}$ V/m. Nonlinear effects become significant when the optical field is an appreciable fraction of this. For first-order nonlinear effects (second-harmonic generation), significant effects appear at $E \sim E_\text{atom}/1000 \sim 10^8$ V/m, corresponding to intensity $I \sim 10^{13}$ W/m² = 10 GW/cm² — achievable with pulsed lasers.

However, in waveguides, the field is confined to very small mode areas ($A_\text{eff} \sim 0.1$–1 μm²), so the intensity for a given power is enormously enhanced. In a silicon nanowire waveguide, 1 mW of cw power gives $I = P/A_\text{eff} = 10^{-3}/10^{-13} = 10^{10}$ W/m² — still below the threshold for strong $\chi^{(2)}$ effects in silicon (which is zero by symmetry anyway), but approaching the regime where $\chi^{(3)}$ effects (the Kerr effect) become observable. Pulsed operation with peak powers of kilowatts gives intensities of $10^{16}$ W/m² — well into the nonlinear regime.

Nonlinear optics is not merely a curiosity. It is the physical basis of:
- **Wavelength conversion**: second-harmonic generation (SHG), optical parametric amplification, and sum/difference frequency generation — used to extend the wavelength range of lasers and to generate entangled photon pairs (crucial for quantum photonic computing, Unit VII).
- **The Kerr effect (intensity-dependent refractive index)**: $n(I) = n_0 + n_2 I$. This is the basis of Kerr switches, self-phase modulation (which broadens laser pulses in fibers), cross-phase modulation (which mediates interactions between different wavelength channels), and the formation of optical solitons.
- **Optical solitons**: pulses that propagate without spreading because the Kerr nonlinearity exactly balances the group velocity dispersion. Solitons are important for long-distance optical communications and for ultrashort pulse generation.
- **Phase-matched nonlinear interactions**: the conditions under which energy can be efficiently transferred between optical waves at different frequencies.

## Subsections

- **3.3.1 — The Nonlinear Polarization Expansion**: How $\mathbf{P}(\mathbf{E})$ is expanded in powers of $\mathbf{E}$; symmetry constraints on $\chi^{(2)}$ and $\chi^{(3)}$; units and magnitudes.
- **3.3.2 — Second-Harmonic Generation**: The $\chi^{(2)}$ process; coupled wave equations; phase matching.
- **3.3.3 — The Kerr Effect and Third-Order Nonlinearities**: Intensity-dependent refractive index; self-phase modulation; cross-phase modulation.
- **3.3.4 — Phase Matching**: The crucial condition for efficient nonlinear conversion; birefringence phase matching; quasi-phase matching; group-velocity matching.
- **3.3.5 — Optical Solitons**: The nonlinear Schrödinger equation; bright solitons in anomalous GVD; applications in fiber communications.
