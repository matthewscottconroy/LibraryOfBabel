# 38.1.1 Shannon Entropy, Thermodynamic Entropy, and the Arrow of Time

---

## Two Entropies

Claude Shannon's 1948 paper "A Mathematical Theory of Communication" (*Bell System Technical Journal*, 27: 379–423, 623–656) introduced a quantitative measure of the information content of a message:

H = -∑ pᵢ log₂ pᵢ

where the sum is over all possible messages and pᵢ is the probability of message i. Shannon called this quantity "entropy" on the (possibly apocryphal) advice of John von Neumann, who noted that the formula was mathematically identical to Boltzmann's thermodynamic entropy formula:

S = -k_B ∑ pᵢ ln pᵢ

The formal identity is exact (up to the choice of base for the logarithm and the Boltzmann constant k_B). Both formulas measure the "spread" or "uncertainty" in a probability distribution: they are large when the distribution is uniform (many equally probable possibilities) and small when the distribution is concentrated (one or a few outcomes dominate).

Is this formal identity a coincidence, or does it reveal a deep physical connection?

## Brillouin and Physical Information

Leon Brillouin's book *Science and Information Theory* (1956) argued that the formal identity is not a coincidence but reflects a genuine physical equivalence. Brillouin's central claim, building on earlier work by Szilard (1929) on Maxwell's Demon, is that *information has physical consequences*. Specifically: acquiring information about a physical system requires decreasing the entropy of the system (increasing our knowledge decreases the probability spread), and this decrease must be paid for by an entropy increase somewhere else.

The connection to Maxwell's Demon is important. Maxwell had proposed a thought experiment in which a demon could decrease the entropy of a gas (concentrating fast molecules in one half and slow molecules in the other) without doing work, apparently violating the Second Law. Szilard (1929: 753–788) argued that the demon's act of measuring which molecules are fast must itself generate entropy — and that the entropy cost of measurement exactly compensates the entropy decrease achieved by sorting. Brillouin (1956: 160–174) formalized this argument: the information the demon acquires is physically equivalent to a decrease in entropy, and this decrease must come at a thermodynamic cost.

This line of argument supports the identification of Shannon entropy with thermodynamic entropy: acquiring information about a physical system decreases physical entropy, and physical entropy can be treated as a form of missing information about the microscopic state of the system.

## The Arrow of Time as an Informational Arrow

The connection between information and entropy has implications for the arrow of time. The second law says that thermodynamic entropy increases: systems evolve from low-entropy (orderly, low-uncertainty) states toward high-entropy (disordered, high-uncertainty) states. In informational terms: the physical world evolves from states of low missing information (states that are highly constrained and specific) toward states of high missing information (states that are disordered and non-specific).

This informational restatement of the second law connects to the asymmetry between past and present that Albert (2000: 21–37) and Price (1996: 24–48) have analyzed. The past is special — it was in an unusually low-entropy state (the "Past Hypothesis") — and this special past state is responsible for the thermodynamic arrow. In informational terms: the past state of the universe was highly specific (low entropy, high information), and the evolution of the universe is a process of spreading from that specific initial state toward higher-entropy, lower-information states.

## Are the Two Entropies the Same?

The debate about whether thermodynamic and Shannon entropy are "the same thing" is philosophically substantive. Several positions have been defended.

*They are the same*: thermodynamic entropy just *is* a measure of the missing information about the microscopic state of a system, given knowledge of its macroscopic state. Jaynes (1957: 620–630) developed the "maximum entropy" approach to statistical mechanics on this basis: the equilibrium distribution is the maximum-entropy distribution consistent with the macroscopic constraints, and this maximizes missing information. On this view, thermodynamics is a branch of inference theory.

*They are formally analogous but physically distinct*: thermodynamic entropy is a physical quantity with units (joules per kelvin); Shannon entropy is a dimensionless mathematical measure of information content. The formal identity of the formulas does not imply identity of the quantities they measure. On this view, the similarity is a deep mathematical coincidence that reflects the underlying structure of probability theory, not an identity of subject matter.

*They converge through Landauer's principle*: Landauer's principle (Section 38.3) provides a specific physical consequence of information processing — information erasure generates heat — that connects the two entropies in a concrete, testable way. This is evidence for the view that the connection is not merely formal but physically substantive.

The question remains contested. What is not contested is that the connection between information theory and thermodynamics has been enormously fruitful for both fields, and that the informational perspective on the arrow of time — seeing temporal asymmetry as an asymmetry in information flow — is a productive and illuminating reframing.

---

*See also: Chapter 21 on the arrow of time; Section 38.3 on Landauer's principle.*
