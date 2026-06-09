# Chapter 29 — Thermodynamics and Information

> *Maxwell's demon was defeated by Landauer: erasing one bit of information costs $kT\ln 2$ joules. Entropy is physical — Boltzmann's entropy, Gibbs's entropy, and Shannon's entropy are the same thing in different units. Thermodynamics is information theory at finite temperature.*

**Prerequisites:** Chapter 16 (Shannon entropy), Chapter 17 (entropy generalizations), Chapter 22 (entropy in dynamical systems).

---

Here is the most surprising equation in this book: $1 \text{ bit} = k_B \ln 2 \text{ joules/kelvin}$.

This is not a loose analogy. It is an exact physical conversion factor. Shannon entropy and thermodynamic entropy are the same quantity, measured in different units. Boltzmann's constant $k_B = 1.38 \times 10^{-23}$ J/K is the conversion rate between bits and joules per kelvin. Information is physical. Erasing a bit of memory generates heat. Sorting molecules without generating heat requires a computer, and running that computer generates at least as much heat as the sorting saves.

Maxwell proposed his demon in 1867, imagining a creature who could sort fast and slow molecules and thereby reduce the entropy of a gas without doing work, violating the second law of thermodynamics. The demon puzzled physicists for nearly a century. Leo Szilard in 1929 understood that the demon must *observe* the molecules — and that observation is information. Rolf Landauer in 1961 made it precise: the bottleneck isn't observation, it's *erasure*. The demon accumulates information and must periodically clear its memory. That erasure is irreversible, and it costs exactly $k_B T \ln 2$ per bit. The second law is safe.

This chapter unpacks what that story means mathematically, and then extends it. The Boltzmann-Gibbs-Shannon connection is made exact. The Jarzynski equality gives a quantitative version of the second law that works for any protocol, not just equilibrium processes. And the thermodynamic formalism of Ruelle and Bowen shows that the entire structure of statistical mechanics — partition functions, free energy, phase transitions — has a perfect counterpart in the dynamics of hyperbolic systems.

---

## Sections

- [29.1 — The Boltzmann-Gibbs-Shannon Connection](the-boltzmann-gibbs-shannon-connection.md)
- [29.2 — Landauer's Principle](landauers-principle.md)
- [29.3 — The Jarzynski Equality and Fluctuation Theorems](the-jarzynski-equality-and-fluctuation-theorems.md)
- [29.4 — Thermodynamic Formalism](thermodynamic-formalism.md)
- [29.5 — Entropy Production and Irreversibility](entropy-production-and-irreversibility.md)
- [Exercises](exercises.md)
- [Chapter Notes](notes.md)
