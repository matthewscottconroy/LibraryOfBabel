# Chapter 5 Overview: Proof Strategies

---

## Central Question

Formal proof systems tell us what constitutes a valid proof; they do not tell us how to *find* one. The art of proof — the creative, strategic dimension — is not reducible to rule-following. This chapter catalogues the main strategies mathematicians use, gives formal accounts of each, and explains how they translate into the formal systems of Chapter 4.

---

## Why This Chapter Matters

Students who have mastered truth tables and can verify a proof often struggle to discover proofs independently. Proof strategies are the bridge between formal rules and mathematical intuition. Each strategy corresponds to a specific structural pattern in natural deduction or sequent calculus proofs, making them amenable to automation in proof assistants (Lean 4's `apply`, `intro`, `cases`, `induction` tactics all implement specific strategies).

---

## Key Definitions

**Goal-directed proof.** A proof search strategy in which one starts with the desired conclusion and works backward, applying rules in reverse. In natural deduction, this means applying *elimination* rules backward: to prove $\phi$, identify what would imply $\phi$ and prove those antecedents.

**Forward proof.** A proof strategy in which one starts from the hypotheses and applies *introduction* rules forward, accumulating new consequences.

**Backward chaining.** A goal-directed strategy used in resolution provers and Prolog: to prove $G$, find a rule $H_1 \land \cdots \land H_n \to G$ and recursively prove each $H_i$.

---

## The Main Strategies

### Direct Proof

**Strategy:** To prove $P \to Q$, assume $P$ and derive $Q$.

**Formal counterpart:** The $\to I$ rule in natural deduction. In sequent calculus: $\Rightarrow R$ rule for implication.

**Example:** Prove "if $n$ is even, then $n^2$ is even."

Assume $n$ is even. Then $n = 2k$ for some integer $k$. Then $n^2 = 4k^2 = 2(2k^2)$, which is even. $\square$

**When to use:** Always try direct proof first. It is the most transparent and simplest strategy.

### Proof by Contradiction (Indirect Proof, Reductio)

**Strategy:** To prove $\phi$, assume $\neg\phi$ and derive $\bot$ (a contradiction).

**Formal counterpart:** The RAA rule in classical natural deduction. Requires classical logic (LEM).

**Example:** Prove "$\sqrt{2}$ is irrational."

Assume for contradiction that $\sqrt{2} = p/q$ in lowest terms. Then $2q^2 = p^2$, so $p^2$ is even, so $p$ is even (say $p = 2m$). Then $2q^2 = 4m^2$, so $q^2 = 2m^2$, so $q$ is even. But $p$ and $q$ are both even, contradicting "lowest terms." $\square$

**When to use:** When you know the conclusion is true but the path from hypotheses to conclusion is not visible; when the hypothesis "assume the negation" gives you something concrete to work with; for existence proofs where the object's non-existence leads to a contradiction.

**Pitfall:** Overusing contradiction when a direct proof is available. Proof by contradiction is often less enlightening — it tells you the result is true but not *why* it must be so.

### Proof by Contrapositive

**Strategy:** To prove $P \to Q$, instead prove $\neg Q \to \neg P$.

**Formal counterpart:** Follows from the classical equivalence $(\phi \to \psi) \equiv (\neg\psi \to \neg\phi)$. In natural deduction: apply $\to I$ and $\neg\neg E$.

**Example:** Prove "if $n^2$ is odd, then $n$ is odd."

Contrapositive: if $n$ is even, then $n^2$ is even. This is the direct proof above. $\square$

**When to use:** When $\neg Q$ gives more useful hypotheses than $P$, or when $\neg Q \to \neg P$ has a more natural direct proof than $P \to Q$.

### Proof by Induction

**Mathematical induction:** To prove $\forall n \in \mathbb{N}, P(n)$:
1. *Base case:* Prove $P(0)$.
2. *Inductive step:* Prove $\forall n, P(n) \to P(n+1)$.

**Formal counterpart:** The induction principle for $\mathbb{N}$ in Peano arithmetic: $P(0) \land \forall n(P(n) \to P(n+1)) \to \forall n\, P(n)$.

**Strong induction:** To prove $P(n)$, assume $P(k)$ for all $k < n$ (rather than just $P(n-1)$). Equivalent to standard induction but more flexible.

**Structural induction:** To prove a property $P$ holds for all elements of an inductively defined set (formulas, terms, lists, trees), prove it for each constructor case, assuming it holds for all sub-components (the inductive hypothesis).

**Example (structural induction on formulas):** Every formula has an equal number of left and right parentheses.

*Base case:* Atoms have 0 left and 0 right parentheses. Equal. ✓

*Inductive case:* $(\phi \land \psi)$: by the inductive hypothesis, $\phi$ has $a$ lefts and $a$ rights, and $\psi$ has $b$ lefts and $b$ rights. Then $(\phi \land \psi)$ has $a + b + 1$ lefts and $a + b + 1$ rights. Equal. ✓ (And similarly for all other connectives.) $\square$

### Case Analysis (Proof by Cases)

**Strategy:** To prove $\phi$, first prove $P \lor \neg P$ (or enumerate all possibilities), then prove $\phi$ under assumption $P$ and prove $\phi$ under assumption $\neg P$.

**Formal counterpart:** The $\lor E$ rule, applied with $P \lor \neg P$ as the disjunction.

**When to use:** When the conclusion's truth depends on the value of some discrete quantity; when a property is more naturally proved for different regions of a parameter space.

**Example:** Prove that for any integer $n$, $n(n+1)$ is even.

*Case 1:* $n$ is even. Then $n = 2k$, so $n(n+1) = 2k(n+1)$ is even. ✓

*Case 2:* $n$ is odd. Then $n+1 = 2m$ for some $m$, so $n(n+1) = n \cdot 2m$ is even. ✓

### Construction / Existence Proofs

**Constructive existence:** To prove $\exists x\, P(x)$, produce an explicit witness $t$ and prove $P(t)$.

**Formal counterpart:** The $\exists I$ rule. This corresponds to the computational content of the proof: the proof is itself an algorithm for finding the witness.

**Non-constructive existence (by contradiction):** Assume $\forall x\, \neg P(x)$ and derive a contradiction. This proves $\exists x\, P(x)$ classically without giving a witness.

---

## Proof Strategies in Proof Assistants

In Lean 4, each strategy corresponds to a tactic:

| Strategy | Lean 4 tactic |
|----------|--------------|
| Assume $P$, prove $Q$ | `intro h` |
| Apply an implication | `apply h` |
| Split conjunction goal | `constructor` |
| Case analysis on $P \lor Q$ | `rcases h with h1 \| h2` |
| Proof by contradiction | `by_contra h` |
| Mathematical induction | `induction n with` |
| Structural induction | `induction` on inductive type |
| Produce witness | `exact ⟨t, ht⟩` or `use t` |
| Finish trivial goal | `trivial`, `exact h`, `assumption` |

---

## The Role of Diagrams and Intuition

Formal strategies do not replace geometric intuition; they formalise it. The strategy of "draw a diagram, then identify cases based on the diagram's regions" corresponds to choosing a case split; the strategy of "extend the construction and use the extension's properties" corresponds to finding an auxiliary lemma.

A useful heuristic: **a proof by contradiction that does not use the negation hypothesis is actually a direct proof in disguise.** If the negation is assumed but never used, the proof structure is $\neg\phi \to (\phi \text{ from other sources})$, which should be written as a direct proof of $\phi$.

---

## Historical Context

**Euclid (circa 300 BCE)** established proof by construction as the gold standard in mathematics. His *Elements* provides existence proofs by exhibiting geometric constructions.

**Gottfried Wilhelm Leibniz (17th century)** envisioned proof as mechanical symbol manipulation — long before this was formalised. His dream of "calculemus" is now realised in proof assistants.

**George Pólya (1945)** published *How to Solve It*, a systematic catalogue of heuristics for mathematical problem-solving, including proof strategies. While not formal, his catalogue informed the development of automated proof systems.

**The coq proof assistant (1984–present)** made proof strategies explicit as *tactics*. Every tactic in Coq corresponds precisely to an inference rule or a derived rule in the underlying proof system.

---

## Connections to Other Chapters

- **Chapter 4** defines the formal proof systems within which these strategies operate.
- **Chapter 7** (Induction and Recursion) develops mathematical induction and structural recursion in depth.
- **Chapter 13** shows how these strategies are implemented as tactics in Lean 4 and Coq.
