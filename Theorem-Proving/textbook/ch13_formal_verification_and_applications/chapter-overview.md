# Chapter 13 Overview: Formal Verification and Applications

---

## Central Question

How do we prove that software and hardware are correct — not just "probably correct" but *provably* correct, with a machine-checkable mathematical certificate? And what are the practical limits of such verification?

---

## Why This Chapter Matters

Formal verification is the application of logic to systems engineering. It has produced some of the most impressive technical achievements in recent computer science: the verified C compiler (CompCert), the formally verified seL4 microkernel, the Flyspeck project (Hales' proof of the Kepler conjecture), and the mathematics library Mathlib in Lean 4. Understanding the theoretical foundations (Hoare logic, separation logic, temporal logic model checking) and the practical tools (Lean 4, Coq) is essential for anyone working at the intersection of mathematics and computing.

---

## Key Definitions

**Hoare triple.** A Hoare triple $\{P\}\ C\ \{Q\}$ states: if precondition $P$ holds before executing command $C$, and $C$ terminates, then postcondition $Q$ holds after $C$. ($P$ and $Q$ are first-order assertions about the program state; $C$ is a program command.)

**Partial vs. total correctness:**
- *Partial correctness:* $\{P\}\ C\ \{Q\}$ — if $C$ terminates, then $Q$ holds.
- *Total correctness:* $[P]\ C\ [Q]$ — $C$ terminates *and* $Q$ holds.

**Separation logic assertion.** A separation logic assertion $P$ describes the heap:
- $e_1 \mapsto e_2$ ("points-to"): the heap has exactly the cell at address $e_1$ with value $e_2$.
- $P * Q$ (separating conjunction): the heap can be split into disjoint parts, one satisfying $P$ and one satisfying $Q$.
- $P \mathbin{-\!\!*} Q$ (magic wand): if you add a heap satisfying $P$, the combined heap satisfies $Q$.

**Invariant.** A loop invariant is a predicate that holds before and after each iteration of a loop. The key rule for while loops:

$$\frac{\{I \land B\}\ C\ \{I\}}{\{I\}\ \mathbf{while}\ B\ \mathbf{do}\ C\ \{I \land \neg B\}}$$

**Model checking.** An algorithmic approach to verifying that a finite-state model (a Kripke structure) satisfies a temporal logic specification. Exhaustively explores all reachable states.

---

## Hoare Logic

### The Hoare Rules

The core rules of Hoare logic for a simple imperative language:

**Assignment:** $\{P[e/x]\}\ x := e\ \{P\}$ (substitute $e$ for $x$ in the postcondition to get the precondition)

**Composition:** $\frac{\{P\}\ C_1\ \{Q\} \quad \{Q\}\ C_2\ \{R\}}{\{P\}\ C_1; C_2\ \{R\}}$

**If-then-else:** $\frac{\{P \land B\}\ C_1\ \{Q\} \quad \{P \land \neg B\}\ C_2\ \{Q\}}{\{P\}\ \mathbf{if}\ B\ \mathbf{then}\ C_1\ \mathbf{else}\ C_2\ \{Q\}}$

**While:** $\frac{\{I \land B\}\ C\ \{I\}}{\{I\}\ \mathbf{while}\ B\ \mathbf{do}\ C\ \{I \land \neg B\}}$

**Consequence:** $\frac{P' \Rightarrow P \quad \{P\}\ C\ \{Q\} \quad Q \Rightarrow Q'}{\{P'\}\ C\ \{Q'\}}$ (strengthen precondition, weaken postcondition)

### Soundness of Hoare Logic

**Theorem (Soundness).** All derivable Hoare triples $\{P\}\ C\ \{Q\}$ are valid: in any state satisfying $P$, executing $C$ and reaching a final state yields one satisfying $Q$.

*Proof.* By structural induction on the derivation. Each rule can be verified to be semantically sound. $\square$

**Theorem (Completeness, Cook 1978).** For any language whose assertions are sufficiently expressive (can express all weakest preconditions), the Hoare proof rules are complete.

*The completeness result is conditional:* it requires the assertion language to be able to express weakest preconditions, which for most languages requires an oracle for the assertion theory. In practice, completeness is achieved relatively (relative to the oracle), not absolutely.

---

## Separation Logic

### Motivation

Hoare logic has trouble reasoning about programs that manipulate heap-allocated data structures (linked lists, trees, etc.) because:
1. Aliasing: two program variables may point to the same heap cell; modifying one modifies the other.
2. Frame reasoning: how do you know that a procedure that modifies part of the heap doesn't inadvertently modify another part?

Separation logic, introduced by O'Hearn, Reynolds, and Yang (2001), solves both problems.

### The Frame Rule

The key inference rule:

$$\frac{\{P\}\ C\ \{Q\}}{\{P * R\}\ C\ \{Q * R\}}$$

(provided $C$ does not modify any free variable of $R$)

This says: if a command $C$ is verified with precondition $P$ and postcondition $Q$, then it can be "framed" with any additional heap assertion $R$ that $C$ does not touch. This makes local reasoning possible: verify each procedure in isolation, then compose.

### Bi-Abduction

Modern separation logic tools (like Facebook's Infer) use *bi-abduction* to automatically infer the frame $R$ and the precondition $P$ simultaneously, enabling scalable analysis of large codebases.

---

## Lean 4 and Coq

### Architecture

Both Lean 4 and Coq implement the **Calculus of Inductive Constructions (CIC)**: a dependent type theory (Chapter 11) with:
- $\Pi$-types (dependent functions) and $\Sigma$-types (dependent pairs)
- Inductive types with recursors and eliminators
- A universe hierarchy $\text{Prop} : \text{Type}_0 : \text{Type}_1 : \cdots$
- $\text{Prop}$ is the impredicative universe of propositions (proof-irrelevant in Coq; Lean 4 has both `Prop` and `Type`)

**Proof terms.** A proof of $P$ in Lean 4/Coq is a term of type $P$. Type-checking is proof verification. All proofs are verified by the *kernel* — a small, trusted piece of code that checks definitional equality. Everything else (tactics, automation, metaprogramming) is untrusted elaboration that must produce kernel-accepted terms.

### Tactics as Proof Search

Lean 4 tactics manipulate proof goals:
- `intro h` — introduce a hypothesis (the $\to I$ rule)
- `apply h` — apply a lemma or hypothesis (backward reasoning)
- `exact t` — close the goal with a term $t$
- `rw [h]` — rewrite using an equality
- `simp` — simplify using a database of rewrite rules
- `ring` — solve ring equalities automatically
- `omega` — solve linear integer arithmetic
- `aesop` — a proof search tactic using a tree search
- `decide` — decidable propositions verified by computation

### Mathlib

Mathlib (Lean 4's mathematics library) contains over 100,000 theorems covering algebra, analysis, topology, number theory, and more. It is the largest formalised mathematics library in existence and growing rapidly. Key formalised results include: Fermat's Last Theorem for specific exponents, the four-colour theorem, the prime number theorem, and much of graduate-level mathematics.

---

## Model Checking

### Kripke Structures and CTL

A Kripke structure $M = (S, I, R, L)$ models a concurrent system:
- $S$: finite set of states
- $I \subseteq S$: initial states
- $R \subseteq S \times S$: transition relation (total)
- $L: S \to 2^{AP}$: labelling of states by atomic propositions

Computation Tree Logic (CTL) formulas describe properties of the computation tree:
- $EX\phi$: some next state satisfies $\phi$
- $AX\phi$: all next states satisfy $\phi$
- $E[\phi\ U\ \psi]$: along some path, $\phi$ holds until $\psi$
- $A[\phi\ U\ \psi]$: along all paths, $\phi$ holds until $\psi$
- $EG\phi$: some path has $\phi$ globally
- $AG\phi$: all paths have $\phi$ globally

**CTL model checking algorithm (Clarke, Emerson, Sistla 1986).** Runs in time $O(|S| \cdot |\phi|)$ — polynomial in the product of model and formula size. This efficiency is what makes model checking practical.

---

## Historical Context

**Tony Hoare (1969)** introduced Hoare logic in a landmark paper "An axiomatic basis for computer programming." This paper defined the program logic framework and proved soundness for a simple language.

**Floyd (1967)** had earlier proposed annotating flowcharts with assertions; Hoare's contribution was the axiomatic form and the Hoare triple notation.

**Peter O'Hearn, John Reynolds, Hongseok Yang (2001)** introduced separation logic in a series of papers that revolutionised program verification by making modular reasoning about the heap possible.

**Edmund Clarke, E. Allen Emerson, Joseph Sifakis (1981, Turing Award 2007)** introduced model checking: algorithmic verification of finite-state concurrent systems against temporal logic specifications.

**Thierry Coquand and Gérard Huet (1988)** created the Calculus of Constructions, the foundation of Coq.

**Georges Gonthier (2008)** completed the fully formal Coq proof of the four-colour theorem, the first major theorem proved by a computer to be fully machine-verified.

**Thomas Hales et al. (Flyspeck project, 2014)** completed the formal verification of Hales' proof of the Kepler conjecture on sphere packing — a proof so long and computational that reviewers of the original (1998) submission felt they could not fully verify it manually.

---

## Connections to Other Chapters

- **Chapters 4, 5** (proof systems, strategies): all of these are implemented in Lean 4/Coq as inference rules and tactics.
- **Chapter 11** (type theory): Lean 4 and Coq are dependent type theories; understanding the theory explains the tools.
- **Chapter 14** (temporal logic): model checking verifies temporal logic properties of Kripke structures.
