# Section 1.4: What We Actually Need — A Theory of Computational Memory

---

## Section Introduction

The previous sections have established what doesn't work: a feedforward network cannot handle arbitrary temporal dependencies, and the sliding window trick merely converts the problem into a parameter explosion. We need something better — and to build something better, we need to understand what "better" means mathematically.

This section develops a rigorous theory of computational memory. We ask: what properties must a system possess in order to be a good temporal processor? We introduce the concept of **fading memory** — a formalization of the intuition that recent inputs should matter more than ancient ones — and we state the theorem that makes reservoir computing theoretically justified: any system with fading memory can be approximated by a driven dynamical system.

This theorem, due to Boyd and Chua [Boyd1985], is the mathematical cornerstone on which all of reservoir computing rests. Understanding it deeply is worth the effort.
