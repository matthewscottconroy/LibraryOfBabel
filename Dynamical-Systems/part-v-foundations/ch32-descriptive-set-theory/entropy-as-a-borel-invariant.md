# 32.4 Entropy as a Borel Invariant

Section 32.3 ended with a negative result: ergodic measure-preserving systems are unclassifiable. But the story is more nuanced than a flat "no." Some subclasses of ergodic systems are classifiable — Bernoulli shifts being the most important example. Understanding what makes Bernoulli shifts special, and what the entropy invariant actually "does" in the descriptive set-theoretic framework, illuminates both the positive and negative results.

**Theorem 32.4.1.** The KS entropy $h: \text{Erg}(X,\mu) \to [0,\infty]$ is a Borel function on the space of ergodic MPTs.

Entropy is Borel — meaning you can compute it as a measurable function on the space of all ergodic systems. This is not trivial: entropy is defined as a limit of partitional entropies, and the convergence involves suprema over all partitions. But the structure of these operations (suprema over countable families, limits of sequences) preserves Borel measurability.

Being Borel means entropy can serve as a Borel invariant: if $T$ and $S$ are isomorphic, then $h(T) = h(S)$. This is necessary for a complete invariant. But is it sufficient?

**Theorem 32.4.2 (Ornstein's Theorem — Classification by Entropy for Bernoulli Shifts).** The isomorphism relation restricted to Bernoulli shifts is smooth (classifiable): two Bernoulli shifts are isomorphic iff they have the same entropy. The invariant $h$ is a *complete invariant* for Bernoulli shifts.

For Bernoulli shifts, entropy is both necessary and sufficient. This is Ornstein's theorem (Chapter 7), viewed from the descriptive set-theoretic angle. The isomorphism relation on Bernoulli shifts is Borel reducible to $\text{id}(\mathbb{R})$ — the simplest possible Borel equivalence relation. Bernoulli shifts are as classifiable as it's possible to be.

**Contrast:** For all ergodic MPTs, isomorphism is not classifiable. For Bernoulli shifts (a special subclass), it is. This illustrates the power of Ornstein theory (Chapter 7) and the limitations of general classification.

The contrast is worth dwelling on. Bernoulli shifts are ergodic, mixing, and appear everywhere in applications. They are the paradigmatic examples of "strongly chaotic" systems. And they are perfectly classifiable by a single real number. Why?

Because Bernoulli shifts have a very special structure: they are "as random as possible" (they have no non-trivial factors other than the trivial system and themselves), and the entropy captures this in a complete way. Entropy measures how much "randomness per unit time" the system generates. For Bernoulli shifts, this is the only invariant that matters.

For general ergodic systems, there are other invariants: the spectral type, the presence of compact factors, the isomorphism class of the Kronecker factor, and infinitely more. No single real number — and in fact no countable collection of invariants — can capture all of them.

The descriptive set-theoretic framework puts this in precise terms: the isomorphism relation on Bernoulli shifts is $\leq_B \text{id}(\mathbb{R})$ (classifiable by a real number), while the isomorphism relation on all ergodic systems is $\geq_B E_\infty$ (as hard as any orbit equivalence relation of a Polish group action). There is a dramatic jump in complexity between the special class and the general class.
