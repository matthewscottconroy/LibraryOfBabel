# Further Reading and References — Chapter 17: Quantum Mechanics Foundations

## Tier I: Essential References

**Nielsen, M.A. & Chuang, I.L. (2010). *Quantum Computation and Quantum Information*, 10th anniversary ed. Cambridge University Press.**

"Mike & Ike," the standard text of quantum information science. Chapter 2 (the postulates, density matrices, the Bloch sphere, the Schmidt decomposition, POVMs) covers the same ground as our Sections 17.1 and 17.4 with full mathematical care, and Chapter 12 develops entanglement as a resource. The treatment is platform-agnostic — no optics — which makes it the perfect complement to this chapter's photonics-first approach.

*How to use this for photonic computing*: read Chapter 2 alongside Section 17.1; return for Chapters 9–12 when studying error correction and entanglement distillation in our Chapters 20 and 22.

---

**Gerry, C.C. & Knight, P.L. (2005). *Introductory Quantum Optics*. Cambridge University Press.**

The most approachable rigorous introduction to quantized light. Chapters 2–3 (field quantization, coherent states), Chapter 7 (squeezed light), and Chapter 3's phase-space material map directly onto our Section 17.3, with more worked detail than any competitor at this level. The problem sets are excellent and calibrated for self-study.

*How to use this for photonic computing*: Chapters 2, 3, and 7 are the companion reading for Sections 17.2–17.3; Chapter 6 previews the beam-splitter quantum optics of our Chapter 18.

---

**Walls, D.F. & Milburn, G.J. (2008). *Quantum Optics*, 2nd ed. Springer.**

The standard graduate reference, more compact and more advanced than Gerry & Knight. Definitive treatments of quasi-probability distributions ($P$, $Q$, Wigner), squeezing, and input-output theory. Terser prose; best read after the concepts are already familiar.

---

**Fox, M. (2006). *Quantum Optics: An Introduction*. Oxford University Press (Oxford Master Series).**

An experimentalist's introduction, unusually good at connecting formalism to laboratory practice — photon statistics, real detectors, and quantum information applications appear early and often. The gentlest of the four Tier-I texts; ideal if Sections 17.2–17.3 felt steep.

---

## Tier II: Highly Recommended

### On the Foundations

**Dirac, P.A.M. (1958). *The Principles of Quantum Mechanics*, 4th ed. Oxford University Press.**

The book that created the notation and much of the conceptual frame of this chapter. Chapter 1's discussion of superposition using polarized photons remains one of the finest openings in physics literature.

---

**Peres, A. (1993). *Quantum Theory: Concepts and Methods*. Kluwer.**

The best treatment of what the formalism *means* operationally: measurements, POVMs, Bell inequalities, and the limits of quantum state discrimination, all with uncompromising clarity.

---

**Sakurai, J.J. & Napolitano, J. (2017). *Modern Quantum Mechanics*, 2nd ed. Cambridge University Press.**

The standard graduate quantum mechanics text, for readers wanting the full treatment of the machinery (angular momentum, perturbation theory) that this chapter deliberately bypassed.

---

### On Entanglement and Nonlocality

**Horodecki, R., Horodecki, P., Horodecki, M., & Horodecki, K. (2009). "Quantum entanglement." *Reviews of Modern Physics*, 81(2), 865–942.**

The encyclopedic review of entanglement theory: separability criteria, measures, distillation, bound entanglement. The reference for Section 17.4.4's material at research depth.

---

**Bell, J.S. (2004). *Speakable and Unspeakable in Quantum Mechanics*, 2nd ed. Cambridge University Press.**

Bell's collected papers, including the 1964 theorem and his later refinements. Lucid, opinionated, and philosophically serious.

---

## Key Original Papers

- Bell, J.S. (1964). "On the Einstein Podolsky Rosen paradox." *Physics Physique Fizika*, 1(3), 195–200. [Bell's theorem.]
- Clauser, J.F., Horne, M.A., Shimony, A., & Holt, R.A. (1969). "Proposed experiment to test local hidden-variable theories." *Physical Review Letters*, 23(15), 880–884. [The CHSH inequality of Section 17.4.3.]
- Einstein, A., Podolsky, B., & Rosen, N. (1935). "Can quantum-mechanical description of physical reality be considered complete?" *Physical Review*, 47(10), 777–780. [EPR.]
- Glauber, R.J. (1963). "Coherent and incoherent states of the radiation field." *Physical Review*, 131(6), 2766–2788. [Coherent states as the description of laser fields; the $P$ representation.]
- Wigner, E. (1932). "On the quantum correction for thermodynamic equilibrium." *Physical Review*, 40(5), 749–759. [The Wigner function.]
- Wootters, W.K. & Zurek, W.H. (1982). "A single quantum cannot be cloned." *Nature*, 299, 802–803. [No-cloning; see also Dieks, D. (1982), *Physics Letters A*, 92, 271.]
- Wootters, W.K. (1998). "Entanglement of formation of an arbitrary state of two qubits." *Physical Review Letters*, 80(10), 2245–2248. [Concurrence.]
- Peres, A. (1996). "Separability criterion for density matrices." *Physical Review Letters*, 77(8), 1413–1415. [The PPT criterion of Section 17.4.4.]
- Freedman, S.J. & Clauser, J.F. (1972). "Experimental test of local hidden-variable theories." *Physical Review Letters*, 28(14), 938–941. [First Bell test.]
- Aspect, A., Grangier, P., & Roger, G. (1982). "Experimental realization of Einstein-Podolsky-Rosen-Bohm Gedankenexperiment: a new violation of Bell's inequalities." *Physical Review Letters*, 49(2), 91–94; and Aspect, A., Dalibard, J., & Roger, G. (1982). "Experimental test of Bell's inequalities using time-varying analyzers." *Physical Review Letters*, 49(25), 1804–1807.
- Hensen, B. et al. (2015). "Loophole-free Bell inequality violation using electron spins separated by 1.3 kilometres." *Nature*, 526, 682–686; Giustina, M. et al. (2015). "Significant-loophole-free test of Bell's theorem with entangled photons." *Physical Review Letters*, 115, 250401; Shalm, L.K. et al. (2015). "Strong loophole-free test of local realism." *Physical Review Letters*, 115, 250402. [The 2015 loophole-free trio.]
- Vahlbruch, H., Mehmet, M., Danzmann, K., & Schnabel, R. (2016). "Detection of 15 dB squeezed states of light and their application for the absolute calibration of photoelectric quantum efficiency." *Physical Review Letters*, 117, 110801. [The squeezing record cited in Section 17.3.3.]
- Mari, A. & Eisert, J. (2012). "Positive Wigner functions render classical simulation of quantum computation efficient." *Physical Review Letters*, 109, 230503. [The simulability boundary of Section 17.3.4.]

## Software

**QuTiP (Quantum Toolbox in Python)** — open-source library for exactly the calculations of this chapter: Fock spaces, coherent/squeezed states, Wigner functions, partial traces, entanglement measures. The programming projects for this chapter (state visualization, CHSH simulation) are naturally built on it. **Strawberry Fields** (Xanadu) covers the continuous-variable side and returns in Chapter 21.
