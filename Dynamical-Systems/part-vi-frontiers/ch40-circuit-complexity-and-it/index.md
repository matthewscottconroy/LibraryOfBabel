# Chapter 40 — Circuit Complexity and Information Theory

> *Can every NP problem be solved in polynomial time? The P vs NP question. Information-theoretic lower bounds for circuits — entropy arguments — give the only unconditional complexity lower bounds we know. The frontier is small-depth circuits, monotone circuits, and the connection between Fourier analysis and computational complexity.*

**Prerequisites:** Chapter 26 (entropy method, communication complexity, expanders), Chapter 18 (Kolmogorov complexity), Chapter 16 (Shannon entropy, mutual information).

---

## What This Chapter Is About

We do not know how to prove P ≠ NP. This is one of the most embarrassing facts in theoretical computer science. We believe the answer is yes, P ≠ NP — we'd bet on it. But we can't prove it, despite decades of effort by the best mathematical minds.

What we can do is prove lower bounds for restricted models of computation. Shannon showed in 1949 that most Boolean functions require exponential circuit size — a counting argument. Håstad showed in 1987 that parity cannot be computed by constant-depth circuits of polynomial size — Håstad's switching lemma. Razborov showed in 1985 that the clique function requires exponential monotone circuit size — the approximation method.

All of these use information theory in an essential way. Shannon's argument is explicitly about describing functions with few bits. Håstad's argument uses entropy bounds on restricted functions. Razborov's argument uses approximation theory that's essentially about mutual information.

Then Razborov and Rudich proved in 1994 that all these methods — the "natural proofs" — cannot prove superpolynomial lower bounds against general circuits, assuming pseudorandom generators exist. We've hit a barrier.

This chapter develops the theory: the counting argument, AC$^0$ lower bounds via Fourier analysis, monotone circuit lower bounds, and the natural proof barrier that explains why general lower bounds are so hard to prove.

---

## Sections

- [40.1 Boolean Circuits and Complexity](boolean-circuits.md)
- [40.2 Lower Bounds via Entropy and Counting](entropy-counting-lower-bounds.md)
- [40.3 AC⁰ Lower Bounds: Switching Lemma](ac0-lower-bounds.md)
- [40.4 Monotone Complexity](monotone-complexity.md)
- [40.5 Natural Proofs and the Limits of Lower Bounds](natural-proofs-barrier.md)
- [40.6 Information Complexity and Communication](information-complexity-communication.md)
- [Exercises](exercises.md)
- [Chapter Notes](notes.md)
