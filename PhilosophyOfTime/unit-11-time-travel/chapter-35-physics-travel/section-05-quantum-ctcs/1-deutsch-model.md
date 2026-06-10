# 35.5.1 Deutsch's Quantum Model of Closed Timelike Curves

---

## The Setting

David Deutsch (*Physical Review D*, 1991: 3197–3217) considered what happens when a quantum system — a qubit, say — passes through a region of spacetime containing a CTC. The system enters the CTC region from outside, interacts with its earlier self (the version of the system that has already passed through the loop), and emerges.

The classical Novikov principle says: only self-consistent histories are realized. For a quantum system, the analogous requirement is that the *state* of the system entering the CTC region must be the same as the state that the system would have if it had gone around the loop. This is a *fixed-point* condition: the state at the entrance of the loop must be a fixed point of the dynamical evolution through the loop.

Deutsch reformulated this requirement in terms of density matrices. A *density matrix* ρ describes the quantum state of a system, including mixed states (statistical ensembles of pure states). The Deutsch consistency condition is:

ρ_in = Tr_env[U(ρ_in ⊗ ρ_env)U†]

where U is the unitary evolution operator for the interaction inside the CTC, ρ_env is the state of the CTC region, and the trace is over the environmental degrees of freedom. In words: the density matrix entering the CTC must equal the density matrix that results from evolving the system through the interaction with its earlier self.

## Resolving the Grandfather Paradox

The grandfather paradox has a quantum analog: a qubit enters a CTC with spin-up, interacts with itself in a way that flips the spin, and exits spin-down — but spin-down entering would flip to spin-up, and spin-up entering would flip to spin-down. No pure state is self-consistent.

Deutsch's solution: use a *mixed state*. The density matrix ρ = ½|↑⟩⟨↑| + ½|↓⟩⟨↓| (an equal mixture of spin-up and spin-down) is a fixed point of the spin-flip interaction. This mixed state is self-consistent: the system entering the loop in this mixed state exits the loop in the same mixed state.

The grandfather paradox is resolved: there exists a self-consistent quantum state for the qubit in the CTC scenario. The resolution requires using mixed states rather than pure states — the state entering the CTC is not a pure quantum state but a probabilistic mixture.

## The Information Creation Problem

The resolution comes at a philosophical cost. In standard quantum mechanics, a pure state can only become mixed through interaction with an environment (decoherence). But in Deutsch's model, the qubit enters the CTC as a mixed state — where did the mixedness come from? If the qubit was prepared as a pure state before entering the CTC, its density matrix should be pure when it enters. But a pure state is not a fixed point. So either the qubit must be mixed before entering, or something transforms its state from pure to mixed.

Deutsch argues that the fixed-point requirement *selects* the appropriate mixed state: among all possible states the qubit could have entering the CTC, physics realizes only the self-consistent one. This sounds like the Novikov principle applied to quantum mechanics. But the implication is striking: the CTC generates *additional information* in the form of the mixed state that was not present in the initial pure state. This is sometimes described as "information creation from nothing" — a feature that many physicists find deeply problematic, since it seems to violate the conservation of information that standard quantum mechanics (and quantum gravity intuitions) require.

Deutschian CTCs also allow the solution of NP-hard computational problems with polynomial resources (Aaronson and Watrous 2009): a computer with access to a CTC could solve problems that are computationally intractable in standard quantum computing. This computational power arises from the same "information from nothing" feature. Its existence would dramatically violate our usual understanding of computational complexity.

## Lloyd's Post-Selection Model

Seth Lloyd and colleagues (2011: 011108) proposed an alternative model of quantum CTCs based on *post-selection* — a mechanism widely used in quantum information theory. In the post-selected CTC (P-CTC) model, the self-consistency requirement is implemented by post-selecting outcomes: instead of requiring a fixed-point state, you require that the state at the end of the loop is the same as the state at the beginning, and you post-select on this outcome occurring.

The P-CTC model avoids the information creation problem: post-selection does not create new information but simply selects from outcomes that quantum mechanics permits. The paradox-resolution is different: in the grandfather case, the measurement outcome required for the paradox to arise never occurs — it is post-selected away.

The P-CTC model is equivalent, in some respects, to quantum teleportation: the self-consistency condition is realized by a teleportation protocol that transfers the state backward in time. This connection to quantum information theory makes the model more tractable and connects CTC physics to ongoing research in quantum computing and communication.

The two models — Deutsch's and Lloyd's — make different predictions and rest on different philosophical foundations. Their divergence reveals that there is no consensus on how quantum mechanics would interact with CTCs, and that different choices here reflect different views about the nature of quantum state, information, and consistency.

## Philosophical Implications

Several philosophical implications deserve attention.

*On information*: if Deutsch CTCs allow information creation, then the fundamental laws of physics permit a form of self-generation of information that is at odds with our standard understanding of where information comes from. Information, like energy and matter, is normally thought to be conserved — not created from nothing. Deutsch CTCs would violate this intuition.

*On the nature of quantum states*: the Deutsch model requires that physical systems near CTCs be described by mixed states that are not reducible to pure states plus ignorance of environmental factors. This is a nonstandard interpretation of what quantum states represent, and it has implications for the foundations of quantum mechanics.

*On consistency and explanation*: like the Novikov principle in the classical case, Deutsch's model implements consistency globally — the self-consistent density matrix is selected by the global fixed-point condition, not by local causal evolution. The holistic character of explanation in CTC spacetimes is even more radical in the quantum case, because it involves quantum superpositions and mixed states as the objects of the global constraint.

The quantum theory of CTCs remains an active research frontier, with no consensus on the correct framework. It is a vivid illustration of how the deepest questions about time — its structure, its directionality, its relationship to information and causation — ultimately require a unified framework of quantum gravity that we do not yet possess.

---

*See also: Chapter 37 on the emergence of spacetime; Chapter 38 on time and information; Deutsch (1991) and Lloyd et al. (2011) in Further Reading.*
