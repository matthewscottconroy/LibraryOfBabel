# 43.3 Modal Type Theory and Temporal Logic

## 43.3.1 Linear Temporal Logic in HoTT

Linear Temporal Logic (LTL) is the formal language for reasoning about sequences — infinite executions of a system. It extends propositional logic with temporal operators that speak about the future.

**Definition 43.3.1.** *Linear Temporal Logic (LTL)* extends propositional logic with temporal operators:
- $\bigcirc P$ ("next $P$"): $P$ holds at the next time step
- $\square P$ ("always $P$"): $P$ holds at all future times
- $\diamond P$ ("eventually $P$"): $P$ holds at some future time
- $P \mathrel{\mathcal{U}} Q$ ("$P$ until $Q$"): $P$ holds until $Q$ holds

**Collatz Conjecture in LTL:** For the Collatz map $C: {\mathbb N} \to {\mathbb N}$:
$$\forall n \in {\mathbb N}^+. \diamond (C^k(n) = 1) \quad (\text{the Collatz conjecture})$$

This is the Collatz conjecture stated precisely in LTL: for every starting value $n$, eventually the orbit reaches 1. The $\diamond$ operator says "at some future time," and we're asserting this holds for all starting values.

LTL in type theory is implemented via the *guarded modality*.

**Definition 43.3.2 (Guarded Type Theory).** The *guarded $\triangleright$ modality* (Nakano, 2000) satisfies:
$$\frac{\Gamma, x: \triangleright A \vdash t: A}{\Gamma \vdash \nu x. t: A}$$
(a fixed point can be defined coinductively if the recursion is "guarded"). This captures the productive nature of corecursion.

**Theorem 43.3.3 (Guarded Recursion = Productive Corecursion).** Any guarded recursive definition of a stream is productive (computes an infinite stream). This is the type-theoretic guarantee corresponding to the dynamical requirement that the orbit function generates an infinite orbit.

Productivity is the coinductive analogue of termination. A recursive definition terminates (produces a finite answer) if it's well-founded; a corecursive definition is productive if it always produces the next output after a finite number of steps. The guarded modality $\triangleright A$ ("later $A$") enforces productivity: you can only use $x: \triangleright A$ after you've produced the current output, ensuring the definition makes progress.

## 43.3.2 Spatial Logic and Topological Dynamics

**Definition 43.3.4 (Spatial Type Theory).** A *spatial type theory* (Shulman, 2018) includes a *shape modality* $\int$ (sharp) and a *flat modality* $\flat$ (flat) satisfying:
$$\int A \vdash A \vdash \flat A \quad (\text{cohesive structure})$$

In the dynamical interpretation:
- $\flat A$: discrete set (no topology)
- $A$: topological space
- $\int A$: "shape" of $A$ (forgetting topology, keeping homotopy type)

**Theorem 43.3.5 (Cohesive HoTT — Lawvere).** The modalities $(\int, \flat, \sharp)$ model the "cohesive topos" structure of spaces. Dynamical systems live in the cohesive layer $A$, while their discrete shadows (e.g., symbolic dynamics) live in $\flat A$.

The transition from a continuous dynamical system to its symbolic dynamics — from the flow on a manifold to the shift on a symbol sequence — is precisely the transition from the topological type $A$ to its discrete shadow $\flat A$. Cohesive HoTT makes this transition syntactically explicit.
