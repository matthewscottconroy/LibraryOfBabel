# Chapter 43 — Homotopy Type Theory and Dynamical Systems

> *In HoTT, equality is path, path is homotopy, homotopy is dynamics. The univalence axiom says equivalent types are equal — a form of Ornstein's theorem for types. Corecursion and coinduction capture infinite dynamical processes. Modal type theories encode temporal and spatial logic of dynamical systems.*

**Prerequisites:** Chapter 6 (topological dynamics), Chapter 35 (isomorphism problem, descriptive set theory), Chapter 28 (category theory). Familiarity with dependent type theory helpful.

---

## 43.1 HoTT Basics: Types as Spaces

### 43.1.1 The Correspondence

**The Homotopy Type Theory (HoTT) Correspondence:**

| HoTT | Homotopy Theory | Classical Mathematics |
|---|---|---|
| Type $A$ | Topological space | Set |
| Term $a: A$ | Point $a \in A$ | Element |
| Identity type $a =_A b$ | Path from $a$ to $b$ | Equality |
| Path $p: a =_A b$ | Continuous path | Proof of equality |
| Path composition $p \cdot q$ | Path concatenation | Transitivity of equality |
| Higher identity $p =_{a=b} q$ | Homotopy between paths | Equality of proofs |
| $n$-truncated type | $n$-type (homotopy $n$-type) | $n$-groupoid |
| Contractible type ($-2$-type) | Contractible space | Singleton |
| Proposition ($-1$-type) | Prop: at most one element | Classical proposition |
| Set ($0$-type) | Discrete space | Set |

**Key Principle:** Equality is not atomic — it has structure. Two proofs of equality $p, q: a =_A b$ may or may not be equal, leading to higher-dimensional structure.

### 43.1.2 The Univalence Axiom

**Definition 43.1.1.** For types $A, B$, an *equivalence* is a function $f: A \to B$ with a quasi-inverse. The type of equivalences is $A \simeq B$.

**Axiom 43.1.2 (Univalence — Voevodsky).** For types $A, B$ in the same universe $\mathcal{U}$:
$$(A = B) \simeq (A \simeq B).$$

Equivalences of types are (equivalent to) equalities of types. This axiom is consistent with Martin-Löf type theory and is the foundation of HoTT.

**Dynamical Analogy:** Univalence says "equivalent structures are equal." In ergodic theory: Ornstein's theorem says isomorphic Bernoulli shifts are equal (as abstract dynamical systems). Univalence is the type-theoretic version of the principle that isomorphic objects should be identified.

---

## 43.2 Corecursion and Infinite Dynamical Systems

### 43.2.1 Coinductive Types

**Definition 43.2.1.** A *coinductive type* (or *codata type*) is defined by its *destructors* (projections) rather than its *constructors*. The canonical example is the type of streams:

```haskell
-- Codata (coinductive) definition:
Stream A := { head : A, tail : Stream A }
```

A stream of $A$s is an element with a head (of type $A$) and a tail (another stream of $A$s).

**Theorem 43.2.2 (Bisimulation = Equality for Streams).** Two streams $s, t: \text{Stream}(A)$ are *bisimilar* if there is a relation $R$ with $(s, t) \in R$ and whenever $(s', t') \in R$: $\text{head}(s') = \text{head}(t')$ and $(\text{tail}(s'), \text{tail}(t')) \in R$.

In type theory with coinduction, bisimulation implies equality: bisimilar streams are provably equal. This is the *coinduction principle*.

**Connection to Dynamical Systems:** A stream $s: \text{Stream}(A)$ is the orbit of a point under a dynamics. The head is the current state, the tail is the future orbit. Bisimulation is the stream-version of topological conjugacy: two orbits are "the same dynamics" if they are bisimilar.

### 43.2.2 Corecursion as Orbit Generation

**Definition 43.2.3.** A *corecursive* definition of a stream uses a *coalgebra* map $f: B \to A \times B$ (giving the next state and current output):

```
unfold : (B → A × B) → B → Stream A
unfold f b = let (a, b') = f(b) in a :: unfold f b'
```

**Example 43.2.4 (Doubling Map Stream).** The doubling map $T: [0,1] \to [0,1]$, $T(x) = 2x \pmod 1$, generates the binary expansion of $x$:

```
doublingBits : [0,1] → Stream {0,1}
doublingBits x = unfold (λx. (⌊2x⌋, 2x mod 1)) x
```

This corecursive definition computes the orbit of $x$ under the doubling map — the symbolic coding with respect to the partition $\{[0,1/2), [1/2,1)\}$.

---

## 43.3 Modal Type Theory and Temporal Logic

### 43.3.1 Linear Temporal Logic in HoTT

**Definition 43.3.1.** *Linear Temporal Logic (LTL)* extends propositional logic with temporal operators:
- $\bigcirc P$ ("next $P$"): $P$ holds at the next time step
- $\square P$ ("always $P$"): $P$ holds at all future times
- $\diamond P$ ("eventually $P$"): $P$ holds at some future time
- $P \mathrel{\mathcal{U}} Q$ ("$P$ until $Q$"): $P$ holds until $Q$ holds

**Collatz Conjecture in LTL:** For the Collatz map $C: {\mathbb N} \to {\mathbb N}$:
$$\forall n \in {\mathbb N}^+. \diamond (C^k(n) = 1) \quad (\text{the Collatz conjecture})$$

**Definition 43.3.2 (Guarded Type Theory).** The *guarded $\triangleright$ modality* (Nakano, 2000) satisfies:
$$\frac{\Gamma, x: \triangleright A \vdash t: A}{\Gamma \vdash \nu x. t: A}$$
(a fixed point can be defined coinductively if the recursion is "guarded"). This captures the productive nature of corecursion.

**Theorem 43.3.3 (Guarded Recursion = Productive Corecursion).** Any guarded recursive definition of a stream is productive (computes an infinite stream). This is the type-theoretic guarantee corresponding to the dynamical requirement that the orbit function generates an infinite orbit.

### 43.3.2 Spatial Logic and Topological Dynamics

**Definition 43.3.4 (Spatial Type Theory).** A *spatial type theory* (Shulman, 2018) includes a *shape modality* $\int$ (sharp) and a *flat modality* $\flat$ (flat) satisfying:
$$\int A \vdash A \vdash \flat A \quad (\text{cohesive structure})$$

In the dynamical interpretation:
- $\flat A$: discrete set (no topology)
- $A$: topological space
- $\int A$: "shape" of $A$ (forgetting topology, keeping homotopy type)

**Theorem 43.3.5 (Cohesive HoTT — Lawvere).** The modalities $(\int, \flat, \sharp)$ model the "cohesive topos" structure of spaces. Dynamical systems live in the cohesive layer $A$, while their discrete shadows (e.g., symbolic dynamics) live in $\flat A$.

---

## 43.4 Homotopy Groups and Dynamical Invariants

### 43.4.1 Fundamental Groups as Dynamical Invariants

**Theorem 43.4.1.** For a topological dynamical system $(X, f)$, the fundamental group $\pi_1(X)$ is an invariant (preserved by homeomorphism). More refined: the homotopy type of $X$ is a topological conjugacy invariant.

**In HoTT:** The fundamental group $\pi_1(A, a) := \Omega^1(A, a) = (a =_A a)$ (the type of self-paths at basepoint $a$). For a 1-type (groupoid): every equality has a proof, and the group of "proof equalities" is exactly $\pi_1(A, a)$.

**Definition 43.4.2 (Loop Space Dynamics).** For a based topological space $(X, x_0)$, the loop space $\Omega X = \{f: [0,1] \to X : f(0) = f(1) = x_0\}$ with the concatenation operation is a monoid. The iteration $\Omega^n X$ gives the $n$-th loop space.

**Connection to Ergodic Theory:** For a flow $\phi_t: X \to X$, a *periodic orbit* is a loop in $X$. The homotopy class of this loop in $\pi_1(X)$ is a topological invariant of the orbit. The *Massey products* in the cohomology of $X$ constrain which homotopy classes can support periodic orbits.

---

## 43.5 The Formalization Program

### 43.5.1 Formalizing Ergodic Theory in HoTT

**Goal:** Formalize the key theorems of this textbook in a proof assistant (Agda, Coq, Lean 4) using HoTT foundations.

**Status:**
- Birkhoff's ergodic theorem: formalized in Isabelle/HOL (Avigad-Hölzl, 2012)
- Shannon's AEP: formalized in Lean 4 (partial, 2023)
- Ornstein's theorem: not formalized (too complex for current tools)
- Topological entropy: partially formalized in Lean/Mathlib

**Challenges:**
1. *Measure theory in HoTT*: The standard Lebesgue measure theory uses classical logic (LEM, choice). HoTT is constructive — measure theory requires careful reformulation.
2. *Almost-everywhere statements*: "a.e." means "for all except a null set." In constructive type theory, "null set" must be replaced by a positive notion.
3. *Completeness*: Many ergodic theory proofs use the completeness of $L^2$ spaces, which requires countable choice in constructive settings.

### 43.5.2 Synthetic Dynamical Systems

**Definition 43.5.1 (Synthetic Approach).** A *synthetic* treatment of dynamical systems works entirely within HoTT, using the internal language of the appropriate topos:
- The "space" $X$ is a type with cohesive structure
- The "dynamics" $f: X \to X$ is an endomorphism of types
- "Ergodicity" is expressed as a modal statement: $\square\diamond A \vdash P(A)$ (every set eventually recurs)

**Theorem 43.5.2 (Lawvere's Fixpoint Theorem in HoTT).** For any endofunction $f: A \to A$ in a cohesive topos, the *fixpoint type* $\text{Fix}(f) = \{x: A \ | \ f(x) = x\}$ is a subtype of $A$. The Lawvere fixpoint theorem (generalizing Cantor's diagonalization) says: there is no surjection $A \to A^A$ in any topos.

**Connection to Dynamics:** Lawvere's theorem is the abstract version of:
- Cantor's theorem (no surjection ${\mathbb N} \to \mathcal{P}({\mathbb N})$)
- Gödel's incompleteness (no consistent proof of consistency from within)
- Rice's theorem (no algorithm decides all dynamical properties)
- Curry's paradox (no self-referential system is consistent)

All are instances of the diagonalization principle, which HoTT captures as a type-theoretic theorem.

---

## 43.6 Research Directions: HoTT and Dynamics

**Direction 43.6.1 (Formalization of Pesin Theory).** Pesin's entropy formula (KS entropy = sum of positive Lyapunov exponents) involves subtle measure theory. Formalizing it in Lean/Mathlib would require:
- Oseledec's theorem (formalized multiplicative ergodic theorem)
- Conditional measures on unstable manifolds
- A constructive proof of Ruelle's inequality

**Direction 43.6.2 (HoTT Proof of Ornstein's Theorem).** Ornstein's theorem uses the weak Bernoulli property and finitary codings. A HoTT proof would:
- Define the weak Bernoulli property as a type-theoretic predicate
- Use the univalence axiom to identify isomorphic Bernoulli shifts
- Construct an explicit (finitary) coding witnessing the isomorphism

**Direction 43.6.3 (Corecursive Dynamics and Bisimulation).** Use the bisimulation principle to define a "dynamical equivalence" for coinductive processes. This gives a constructive notion of topological conjugacy for infinite streams (symbolic orbits).

---

## Exercises

**Exercise 43.1.** (HoTT Basics) In HoTT, prove that the type $\prod_{n:\mathbb{N}} P(n)$ is equivalent to the type $P(0) \times \prod_{n:\mathbb{N}} P(n+1)$ (currying). This is the type-theoretic induction principle.

**Exercise 43.2.** (Streams) Write a corecursive definition (in Haskell or Agda syntax) of the logistic map orbit: given $r \in {\mathbb R}$ and $x_0 \in (0,1)$, produce the stream $x_0, rx_0(1-x_0), r(rx_0(1-x_0))(1-rx_0(1-x_0)), \ldots$

**Exercise 43.3.** (LTL) Formalize the statement "the doubling map is ergodic" in LTL. Specifically, express: for Lebesgue-a.e. $x$, for any interval $I \subseteq [0,1]$, the orbit of $x$ visits $I$ with frequency $|I|$.

**Exercise 43.4.** (Research) HoTT's univalence axiom says $A = B \simeq A \simeq B$ (equality is equivalence). In dynamical systems, Ornstein's theorem says KS entropy classifies Bernoulli shifts up to isomorphism. Can you formulate a "univalence" statement for Bernoulli shifts in HoTT? What would it look like?

---

## Chapter Notes

HoTT: The HoTT Book (Univalent Foundations Program, 2013) is free online. Voevodsky's original work on univalent foundations is in *An experimental library of formalized mathematics based on univalent foundations* (Mathematical Structures in Computer Science, 2015).

Guarded recursion: Nakano's *A Modality for Recursion* (LICS, 2000). Guarded type theory: Birkedal-Møgelberg-Schwinghammer-Støvring (LMCS, 2012). Atkey-McBride's *Productive Coprogramming with Guarded Recursion* (ICFP, 2013).

Cohesive HoTT: Schreiber-Shulman's *Quantum Gauge Field Theory in Cohesive Homotopy Type Theory* (EPTCS, 2012). Shulman's *Brouwer's fixed-point theorem in real-cohesive homotopy type theory* (Math. Structures in Comp. Sci., 2018).

Formalized ergodic theory: Avigad-Hölzl-Serafin's *A formally verified proof of the central limit theorem* (2017) and Hölzl's Isabelle formalization of ergodic theory. The Lean/Mathlib formalization of information theory is ongoing; see the Mathlib4 repository.
