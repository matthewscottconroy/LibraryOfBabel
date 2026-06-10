# Section 3.2: The Quantum Picture — Two-Level Atoms and Laser Gain

The classical Lorentz oscillator is a powerful model, but it misses the essential quantum mechanical feature of light-matter interaction: energy is exchanged in discrete quanta. A two-level atom can absorb a photon of frequency $\omega_0 = (E_2 - E_1)/\hbar$ (promoting the atom from state 1 to state 2) or emit one (returning from state 2 to state 1). In the classical model, the atom is a harmonic oscillator with continuously variable amplitude; in quantum mechanics, it is either in the ground state or the excited state (or a superposition).

This quantization has profound consequences. Most strikingly: a quantum atom in its excited state can emit a photon *spontaneously*, even in the absence of any input field. Spontaneous emission has no classical analog (it arises from the zero-point fluctuations of the electromagnetic field, which are quantum mechanical). It is the mechanism by which atoms in excited states always eventually decay — and it sets a fundamental timescale $\tau_\text{sp} = 1/A_{21}$ (the spontaneous emission lifetime) that determines the minimum linewidth of any optical transition.

The twin to spontaneous emission is *stimulated emission*: an excited atom emitting into an incoming photon mode, producing two photons in the same mode. Stimulated emission is the basis of the laser: if a medium contains more atoms in the excited state than in the ground state (population inversion), stimulated emission exceeds absorption, and the medium amplifies light. This is optical gain — the miracle at the heart of photonic computing, because without laser amplifiers to compensate losses, photonic systems would be limited to tiny scales.

This section develops the quantum picture at the level needed for the rest of this book — focused on results and their physical consequences rather than on the full mathematical apparatus of quantum field theory.

## Subsections

- **3.2.1 — The Two-Level System**: Quantum energy levels, transition frequency, transition dipole moment, Rabi oscillations.
- **3.2.2 — Einstein $A$ and $B$ Coefficients**: The three rates (spontaneous emission, stimulated emission, absorption); their interrelationships; thermal equilibrium and the blackbody spectrum.
- **3.2.3 — Population Inversion and Optical Gain**: The gain coefficient; why three or four levels are needed; threshold for lasing; gain saturation.
