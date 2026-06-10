# Unit XIV: Quantum Fields in Curved Spacetime

---

## Unit Introduction

What happens when quantum mechanics meets general relativity? A full theory of quantum gravity — quantum spacetime itself — does not exist. But there is a well-developed semiclassical approximation: **quantum field theory in curved spacetime** (QFT in CST). Here, the gravitational field is treated classically (the metric $g_{\mu\nu}$ obeys Einstein's equations), while the matter fields are quantum. The stress-energy tensor $\langle T_{\mu\nu}\rangle$ — the quantum expectation value — sources the classical metric:
$$G_{\mu\nu} = \frac{8\pi G}{c^4}\langle T_{\mu\nu}\rangle$$

This semiclassical approximation is valid when $\hbar/S_{\rm action} \ll 1$ — when quantum corrections to the geometry are small. It breaks down near spacetime singularities and at the Planck scale $\ell_P = \sqrt{\hbar G/c^3} \approx 10^{-35}$ m.

Within this framework, three remarkable phenomena emerge:

**The Unruh effect** (Fulling 1973, Davies 1975, Unruh 1976): An observer accelerating through flat Minkowski spacetime perceives the quantum vacuum as a thermal bath at temperature $T_U = \hbar a/(2\pi c k_B)$, where $a$ is the proper acceleration. What is the quantum vacuum for an inertial observer is a thermal state for an accelerated observer. This is a profound statement about the observer-dependence of the particle concept in quantum field theory.

**Hawking radiation** (Hawking 1974): A black hole emits thermal radiation at temperature $T_H = \hbar c^3/(8\pi G M k_B)$. This arises from the way the black hole's horizon affects the quantum vacuum — pair creation near the horizon sends particles to infinity while partners fall in. Hawking radiation is negligible for stellar-mass black holes ($T_H \sim 6\times 10^{-8}$ K for a $10 M_\odot$ BH) but becomes dominant for primordial black holes.

**Black hole thermodynamics** (Bekenstein 1972, Hawking 1974): Black holes obey laws formally identical to thermodynamics, with entropy $S = k_B A/(4\ell_P^2)$ proportional to the horizon area and temperature $T_H$ given above. The four laws of black hole mechanics (proven in classical GR by Bardeen, Carter, and Hawking 1973) become the actual laws of thermodynamics when quantum effects are included. The **generalized second law** $\delta(S_{\rm matter} + S_{\rm BH}) \geq 0$ — entropy of matter plus black hole entropy never decreases.

This unit develops these ideas from the ground up, building the necessary QFT machinery and applying it to curved spacetime.

---

## Unit Chapters

- [Chapter 55: Quantum Field Theory in Curved Spacetime](chapter-55-qft-curved-spacetime/README.md)
- [Chapter 56: The Unruh Effect](chapter-56-unruh-effect/README.md)
- [Chapter 57: Hawking Radiation](chapter-57-hawking-radiation/README.md)
- [Chapter 58: Black Hole Thermodynamics](chapter-58-bh-thermodynamics/README.md)
