# Chapter 17 — Entropy and Its Generalizations

> *Shannon entropy is the unique entropy satisfying a natural set of axioms. Relax the axioms and you get a whole family of entropies — Rényi, Tsallis, min-entropy — each optimal for different operational tasks.*

**Prerequisites:** Chapter 16 (Shannon entropy, KL divergence).

---

Shannon entropy is the answer to one specific question: what is the minimum number of bits per symbol needed to compress a source, averaged over many samples? It is the right quantity for that task, and only for that task.

The moment you change the question — ask about one-shot compression, or cryptographic key length, or statistical mechanics with long-range interactions, or quantum states — Shannon entropy may no longer be the right tool. Different operational problems lead to different entropy measures, each with its own formula, its own axioms, and its own domain of applicability.

This chapter surveys the main generalizations of Shannon entropy. We begin with the *Rényi family* — a one-parameter family that interpolates between several important special cases — and then zoom in on *min-entropy*, the quantity that governs one-shot cryptography and privacy amplification. We then move to *differential entropy* for continuous sources, the *maximum entropy principle* of Jaynes, *Tsallis entropy* from non-equilibrium statistical mechanics, and the *von Neumann entropy* of quantum states. Along the way, we keep track of which properties carry over from Shannon entropy and which do not.

The organizing theme is operational meaning: each entropy corresponds to an answer to a specific question. Rényi entropy of order $\alpha$ appears in guessing problems and large deviations. Min-entropy is the right quantity for randomness extraction. Von Neumann entropy governs quantum compression and entanglement. Knowing *which entropy to use when* is as important as knowing the formulas.

**What this chapter builds:**
- The Rényi family of entropies and their limits
- Min-entropy and smooth min-entropy for one-shot cryptography
- Rényi divergence and its data processing inequality
- Differential entropy for continuous distributions
- The maximum entropy principle and exponential families
- Tsallis entropy and non-extensive statistical mechanics
- Von Neumann entropy and strong subadditivity
- Comparisons and connections between entropy measures

**Sections:**
- [17.1 Rényi Entropy](renyi-entropy.md)
- [17.2 Min-Entropy](min-entropy.md)
- [17.3 Rényi Divergence](renyi-divergence.md)
- [17.4 Differential Entropy](differential-entropy.md)
- [17.5 The Maximum Entropy Principle](maximum-entropy-principle.md)
- [17.6 Tsallis Entropy](tsallis-entropy.md)
- [17.7 Von Neumann Entropy](von-neumann-entropy.md)
- [17.8 Connections Between Entropy Measures](connections-between-entropy-measures.md)
- [Exercises](exercises.md)
- [Notes](notes.md)
