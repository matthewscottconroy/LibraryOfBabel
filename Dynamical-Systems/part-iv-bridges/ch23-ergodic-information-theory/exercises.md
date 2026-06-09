# Exercises — Chapter 23

These exercises move from computation (Exercise 23.1) through proof (Exercise 23.2) to open-ended speculation (Exercise 23.3). Together they test your understanding of the SMB theorem, its relationship to the classical LLN, and its ergodic-theoretic content.

---

**Exercise 23.1.** For a Markov chain with transition matrix $P = \begin{pmatrix}0.7 & 0.3 \\ 0.4 & 0.6\end{pmatrix}$ and stationary distribution $\pi$, compute the entropy rate $h = -\sum_{ij} \pi_i P_{ij}\log P_{ij}$.

**Exercise 23.2.** Prove the SMB theorem for i.i.d. processes from the classical LLN. Show that for i.i.d. Bernoulli($p$) process, the typical set has $\approx 2^{n h_b(p)}$ elements where $h_b(p) = -p\log p - (1-p)\log(1-p)$.

**Exercise 23.3.** (Collatz) The Collatz map generates a sequence of parities $y_n = T^n(m) \pmod 2$. If the Collatz map has an ergodic invariant measure $\mu$, what does SMB say about the parity sequences?
