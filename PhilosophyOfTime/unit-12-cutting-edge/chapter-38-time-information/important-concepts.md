# Chapter 38 Important Concepts: Time and Information

---

**Shannon Entropy**
A measure of information content or uncertainty defined by the formula *H* = −Σ *p*ᵢ log₂ *p*ᵢ, where *p*ᵢ are the probabilities of possible outcomes or messages. Shannon entropy is formally identical to Boltzmann-Gibbs thermodynamic entropy (up to units and choice of logarithm base), a coincidence that has provoked deep philosophical questions about the relationship between information and physical entropy. Shannon entropy is maximized when all outcomes are equally probable (maximum uncertainty) and zero when one outcome is certain.

**Thermodynamic Entropy**
The Boltzmann-Gibbs measure of the "disorder" or "spread" of a physical system over its available microstates: *S* = *k_B* ln *W* (where *W* is the number of compatible microstates). The second law of thermodynamics states that thermodynamic entropy increases in closed systems. Thermodynamic entropy has a direct physical interpretation: systems with high entropy are in macrostates that are compatible with many different microstates, making the precise microstate highly uncertain.

**Information-Theoretic Arrow of Time**
The view that the thermodynamic arrow of time is fundamentally an informational asymmetry: the universe evolves from states of low missing information (highly constrained, low-entropy initial conditions) toward states of high missing information (disordered, high-entropy states). On this view, the direction of time is the direction of increasing informational uncertainty at the macroscopic level — the direction in which physical systems lose specific structure and approach maximum entropy.

**Unitarity**
A fundamental principle of quantum mechanics: the evolution of a quantum state is described by a unitary operator, which preserves the total probability and is reversible. Unitarity implies that information is conserved in quantum mechanics: the map from initial to final states is invertible, and in principle the initial state can be recovered from the final state. The apparent conflict between unitarity and Hawking radiation is the core of the black hole information paradox.

**Hawking Radiation**
Thermal radiation emitted by a black hole as a quantum effect near the event horizon, derived by Hawking (1975). Virtual particle-antiparticle pairs created near the horizon result in one particle falling into the black hole and one escaping to infinity; this process gradually reduces the mass of the black hole. Hawking radiation is thermal — it has the character of blackbody radiation — and according to the original calculation carries no information about the quantum state of the matter that formed the black hole.

**Black Hole Information Paradox**
The apparent conflict between quantum mechanics (which requires that information is conserved — unitarity) and Hawking's calculation (which implies that information falling into a black hole is lost when the black hole evaporates). If information is lost, the evolution of the black hole system is non-unitary and the final state cannot be used to reconstruct the initial state, violating a fundamental principle of quantum mechanics. The paradox raises deep questions about the nature of time, causality, and the relationship between quantum mechanics and gravity.

**Page Curve**
The curve showing how the entanglement entropy of the radiation emitted by an evaporating black hole should evolve over time if information is preserved. Don Page (1993) showed that if black hole evaporation is unitary, the entanglement entropy of the radiation should first increase (as the black hole radiates) and then decrease back to zero as the black hole fully evaporates, forming a characteristic curve. Recent calculations using the island formula have reproduced this behavior, suggesting that information is indeed preserved in black hole evaporation.

**Island Formula**
A prescription for calculating the entanglement entropy of Hawking radiation that includes contributions from "islands" — disconnected regions of the black hole interior that nonetheless contribute to the entropy of the exterior radiation through quantum gravity effects. The island formula, developed by Almheiri and collaborators (2019–2020), reproduces the Page curve and suggests that information is preserved in black hole evaporation. It is a semiclassical result — a quantum gravity correction to the naive Hawking calculation — rather than a full quantum gravity derivation.

**Landauer's Principle**
The principle, established by Rolf Landauer in 1961, that erasing one bit of information necessarily generates at least *k_B T* ln 2 of heat (approximately 3 × 10⁻²¹ joules at room temperature). Landauer's principle demonstrates that information is not merely abstract — it has physical consequences. Erasure is thermodynamically irreversible: it generates entropy. Reversible computation (computation that never erases information) can in principle be carried out without thermodynamic cost. The principle has been experimentally confirmed.

**Maxwell's Demon**
A thought experiment introduced by James Clerk Maxwell in 1867 in which a "demon" observes individual molecules and sorts them by speed, apparently reducing the entropy of a gas without expending energy. The demon was proposed to illustrate the statistical rather than absolute character of the second law. The resolution, developed by Szilard (1929) and Landauer (1961), is that the demon's act of acquiring, storing, and eventually erasing information about the molecules has a thermodynamic cost that exactly compensates the entropy reduction achieved by sorting. Maxwell's demon has become a paradigm case for understanding the relationship between information, measurement, and thermodynamics.

**Scrambling**
The process by which information in a quantum system becomes distributed across all degrees of freedom in a way that is in principle recoverable (the evolution remains unitary) but practically inaccessible (no local measurement can recover it). Black holes are proposed to be the fastest scramblers in nature: information that falls into a black hole becomes scrambled across the black hole's horizon degrees of freedom in a time of order *M* log *M* (in Planck units). Scrambling provides a way of reconciling the apparent information loss in Hawking radiation with unitarity: information is not destroyed but scrambled and re-emitted in the Hawking radiation in a practically irretrievable form.

**von Neumann Entropy**
The quantum mechanical generalization of thermodynamic entropy, defined as *S* = −*k_B* Tr(ρ ln ρ) where ρ is the density matrix of the quantum system. For pure quantum states, von Neumann entropy is zero; for mixed states (resulting from entanglement with other systems or from ignorance of the exact state), it is positive. Von Neumann entropy plays the role of both thermodynamic entropy and Shannon entropy in quantum mechanics and is central to quantum information theory.

**Digital Physics**
The radical proposal that the universe is fundamentally a discrete computation — that physical laws are computational rules, physical states are data structures, and time consists of sequential computational steps. Associated with Konrad Zuse (1969), Edward Fredkin, and Stephen Wolfram. Digital physics provides a framework in which the arrow of time is built into the computational structure (rules update states in a definite direction), though it faces objections concerning the empirical evidence for discreteness, the meaning of "computation" without an interpreter, and the problem of identifying the "clock rate" of the cosmic computer.

**Simulation Hypothesis**
The proposal, developed philosophically by Nick Bostrom (2003), that our universe may be a simulation running on the computational hardware of a more fundamental universe. The simulation hypothesis raises questions about the ontological status of time: if we are simulated, is our time the time of the simulation, the time of the substrate, or both? The hypothesis is not currently falsifiable and is thus of limited scientific but significant philosophical interest.

**Entanglement Entropy**
A measure of the quantum entanglement between two subsystems, defined as the von Neumann entropy of one subsystem's reduced density matrix. When two quantum systems are entangled, neither has a definite pure state on its own; the entanglement entropy measures how much information is "shared" between the two systems and inaccessible to local measurements on either alone. Entanglement entropy plays a central role in quantum information theory, quantum error correction, and — through the Ryu-Takayanagi formula — in holographic models of spacetime.
