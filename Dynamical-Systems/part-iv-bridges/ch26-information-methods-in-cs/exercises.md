# Exercises — Chapter 26

These exercises span the chapter's range: combinatorics, communication complexity, and extractors. They are designed to make the methods concrete — each one asks you to carry out the entropy argument or information-theoretic proof, not just state it.

---

**Exercise 26.1.** (Shearer's Lemma) Use Shearer's lemma to prove the Cauchy-Schwarz inequality for counting: if $A \subseteq [n] \times [n]$ and $R_i$ are row projections, then $|A|^2 \leq n \sum_i |R_i|$.

**Exercise 26.2.** (Communication Complexity) Design a randomized protocol for $EQ_n$ using only $O(\log n)$ bits with error $\leq 1/3$. Use pairwise-independent hashing.

**Exercise 26.3.** Prove the Karchmer-Wigderson theorem for a specific function: show that the depth of the AND function on $n$ bits equals $\log n$, and verify by computing $D(KW_{AND_n})$.

**Exercise 26.4.** (Extractors) Show that pairwise-independent hash functions give a $(k, \varepsilon)$-extractor with $d = n$ seed bits and $m = k - 2\log(1/\varepsilon)$ output bits. Find the optimal trade-off.
