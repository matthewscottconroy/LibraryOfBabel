# Exercises — Chapter 43

These exercises develop the HoTT correspondence with dynamical systems, from the induction principle through streams and temporal logic. Exercise 43.4 asks you to formulate a univalence statement for Bernoulli shifts.

---

**Exercise 43.1.** (HoTT Basics) In HoTT, prove that the type $\prod_{n:\mathbb{N}} P(n)$ is equivalent to the type $P(0) \times \prod_{n:\mathbb{N}} P(n+1)$ (currying). This is the type-theoretic induction principle.

**Exercise 43.2.** (Streams) Write a corecursive definition (in Haskell or Agda syntax) of the logistic map orbit: given $r \in {\mathbb R}$ and $x_0 \in (0,1)$, produce the stream $x_0, rx_0(1-x_0), r(rx_0(1-x_0))(1-rx_0(1-x_0)), \ldots$

**Exercise 43.3.** (LTL) Formalize the statement "the doubling map is ergodic" in LTL. Specifically, express: for Lebesgue-a.e. $x$, for any interval $I \subseteq [0,1]$, the orbit of $x$ visits $I$ with frequency $|I|$.

**Exercise 43.4.** (Research) HoTT's univalence axiom says $A = B \simeq A \simeq B$ (equality is equivalence). In dynamical systems, Ornstein's theorem says KS entropy classifies Bernoulli shifts up to isomorphism. Can you formulate a "univalence" statement for Bernoulli shifts in HoTT? What would it look like?
