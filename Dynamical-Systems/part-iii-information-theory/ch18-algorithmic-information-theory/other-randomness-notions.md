# 18.7 Other Randomness Notions

Martin-Löf randomness is the "gold standard" — the strongest sensible notion of randomness for infinite sequences. But it is not the only one. By varying the class of tests or the class of martingales, you get a hierarchy of weaker notions, each with its own character and applications.

**Schnorr Randomness:** Defined using computable (not merely r.e.) tests: the tests must be not just semidecidable but fully decidable. Equivalently, $\omega$ is Schnorr random iff every *computable martingale* succeeds on $\omega$ only at a computable rate. Schnorr randomness is strictly weaker than ML-randomness: there are ML-random sequences that are not Schnorr random.

**Computable Randomness:** $\omega$ is computably random iff no computable martingale succeeds on $\omega$ at all. (A martingale succeeds on $\omega$ if it accumulates unbounded capital by betting on $\omega$'s bits.) This is strictly weaker than Schnorr randomness: there are Schnorr random sequences that a computable martingale can exploit.

**Kurtz Randomness:** The weakest notion: $\omega$ is Kurtz random iff it is not in any computable measure-zero set. This is the "generic" notion — almost all sequences are Kurtz random, but many Kurtz random sequences are not even computably random.

**The Hierarchy:**
$$\text{ML-random} \Rightarrow \text{Schnorr random} \Rightarrow \text{computably random} \Rightarrow \text{Kurtz random.}$$

None of these implications reverse — each is strictly stronger than the next.

Which notion is "correct"? This depends on your application. For most theoretical purposes in computability theory and dynamical systems, ML-randomness is the right notion. For cryptographic applications, one wants a notion that no efficient adversary can detect, which points toward computational randomness notions (where the martingale is polynomial-time bounded) rather than the computability-theoretic hierarchy above.

The hierarchy reflects a fundamental tension: stronger randomness notions capture more of our intuitions about "random-looking" sequences but are harder to guarantee in practice. A physically generated sequence (from, say, quantum noise) might be computably random without being ML-random. The question of which notion is "physically realizable" connects to deep questions about the nature of physical randomness.

For dynamical systems, ML-randomness is the right choice: it corresponds to "typical" points in the ergodic-theoretic sense, as we see in the next section.
