# Chapter 14 — Hamiltonian Systems and KAM Theory

> *Hamiltonian systems model everything that conserves energy: celestial mechanics, molecular dynamics, plasma physics, quantum field theory. KAM theory explains why solar system orbits are (approximately) stable despite resonances — and why this is not obvious.*

**Prerequisites:** Chapters 3 (differential forms, manifolds), 4 (ODEs, flows), 8 (stability theory).

---

## What This Chapter Is About

Isaac Newton proved that planetary orbits are ellipses — but that was for two bodies. The moment you add a third planet, the problem becomes analytically intractable, and the question of whether the solar system is stable has occupied mathematicians for 300 years. Poincaré showed in 1890 that the Newtonian $n$-body problem is not integrable — perturbation series diverge, and the classical program of celestial mechanics breaks down. This was a crisis.

KAM theory — Kolmogorov-Arnold-Moser, developed in the 1950s and 60s — resolved the crisis, partially but profoundly. It says that despite the breakdown of perturbation theory, most orbits in a nearly integrable system are quasi-periodic and persist under small perturbations. Not all orbits — the resonant ones are destroyed — but "most" in the sense of positive measure. The solar system is not provably stable forever, but it is stable for most initial conditions, for astronomically long times.

The story begins with symplectic geometry. A Hamiltonian system lives not on a Riemannian manifold but on a *symplectic* manifold — a space equipped with a closed, non-degenerate 2-form $\omega$. This 2-form encodes the pairing between positions and momenta, and it is preserved by the Hamiltonian flow. Darboux's theorem says that all symplectic manifolds look locally the same — there are no local symplectic invariants, in stark contrast to Riemannian geometry — so the local dynamics of all Hamiltonian systems is locally equivalent to a standard model.

Completely integrable systems are those with as many conserved quantities as degrees of freedom. By the Liouville-Arnold theorem, the phase space of a completely integrable system is foliated by invariant tori, and on each torus the motion is quasi-periodic. Action-angle coordinates make this explicit: the Hamiltonian depends only on the "action" variables (the radii of the tori), and the "angle" variables rotate uniformly. These systems are solvable in principle, but they are measure-zero in the space of all Hamiltonian systems.

KAM theory asks: if you perturb an integrable system by a small Hamiltonian, which tori survive? The answer involves *Diophantine conditions*: tori with frequency vectors that are badly approximable by rationals survive, while tori with rational or near-rational frequency ratios are destroyed. The Diophantine tori form a Cantor-like set of positive measure that becomes dense as the perturbation size $\varepsilon \to 0$. The proof uses a Newton-iteration scheme — quadratic convergence to overcome the small divisors that plague naive perturbation theory.

When tori break down — at non-Diophantine frequencies — they do not disappear cleanly. Aubry-Mather theory says that they leave behind a remnant: an invariant Cantor set (a "cantorus") that constrains the dynamics without forming a barrier. These cantori are the remnants of broken tori and are the right objects to study when KAM theory fails.

In three or more degrees of freedom, KAM tori no longer separate the phase space: a $(n-1)$-dimensional torus does not divide an $n$-dimensional energy surface for $n \geq 3$. Orbits can therefore drift slowly through the "web" of resonances — this is Arnold diffusion. The drift is extremely slow (of order $e^{-1/\varepsilon}$), but it is real and has been proven to occur in examples by Arnold himself (1964) and more recently by Cheng-Yan and Bernard-Kaloshin-Zhang.

The chapter closes with the connection to quantum mechanics. Classically integrable systems correspond to quantum systems with Poisson-distributed eigenvalue spacings; classically chaotic systems correspond to quantum systems with GUE-distributed spacings (the Bohigas-Giannoni-Schmit conjecture). Quantum ergodicity (Shnirelman-Zelditch-Colin de Verdière) says that for ergodic geodesic flow, almost all eigenfunctions equidistribute.

**What this chapter builds:** Symplectic geometry as the natural setting for Hamiltonian mechanics; Liouville integrability and action-angle variables; the KAM theorem on persistence of invariant tori; Aubry-Mather theory for breakdown of tori; and the connections to Arnold diffusion and modern symplectic topology.

---

## Sections

- [14.1 Symplectic Geometry](symplectic-geometry.md) — The symplectic form, Darboux's theorem, and Hamilton's equations
- [14.2 Integrable Systems](integrable-systems.md) — Liouville-Arnold theorem and action-angle variables
- [14.3 KAM Theory](kam-theory.md) — Diophantine conditions, the KAM theorem, and small divisors
- [14.4 Twist Maps and Aubry-Mather Theory](twist-maps-and-aubry-mather-theory.md) — What replaces tori when KAM fails
- [14.5 Arnold Diffusion](arnold-diffusion.md) — Slow drift through resonance webs in high dimensions
- [14.6 Generating Functions and Variational Principles](generating-functions-and-variational-principles.md) — Mather's variational approach
- [14.7 Connections to Quantum Mechanics](connections-to-quantum-mechanics.md) — Quantum chaos and eigenfunction equidistribution

---

- [Exercises](exercises.md)
- [Chapter Notes](notes.md)
