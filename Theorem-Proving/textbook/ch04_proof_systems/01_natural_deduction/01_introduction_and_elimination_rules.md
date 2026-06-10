# Natural Deduction: Logic That Reasons the Way We Think

> *"The calculus of natural deduction... is characterized by the fact that it contains no axioms — only rules."*
> — Gerhard Gentzen, 1935

---

In 1935, Gerhard Gentzen, a 25-year-old German mathematician, published a paper that transformed proof theory. He introduced two proof systems — natural deduction and the sequent calculus — that are still the dominant formal frameworks for logic today. Lean 4 and Coq are, at their core, implementations of dependent type theory that reflects natural deduction's structure. Every proof you write in a proof assistant is, implicitly, a natural deduction derivation.

What makes natural deduction "natural"? The answer is that it is designed to mirror how mathematicians actually reason — not how they might reason if they started from a minimal set of axioms and derived everything mechanically, but how they actually think: introducing hypotheses, combining and eliminating compound statements, reasoning by cases, deriving contradictions.

## The Core Idea: Introduction and Elimination

Every connective in natural deduction is governed by a **pair** of rules:

- An **introduction rule** (I): how to *prove* a formula with that connective as its main symbol
- An **elimination rule** (E): how to *use* a formula with that connective as its main symbol

This pairing reflects a deep philosophical principle: the meaning of a connective is captured by what you need to establish it (introduction) and what you can do with it (elimination). The rules should be balanced — the elimination rule should extract exactly what the introduction rule put in.

This balance, called **harmony** or **proof-theoretic adequacy**, is the intuitive reason why natural deduction is "right" — why the rules are not arbitrary but flow from the meaning of the connectives. Redundancy (where you can derive more than you put in) and insufficiency (where you cannot get back what you put in) are both violations of harmony.

## Conjunction Rules

**∧-Introduction** (∧I): To prove P ∧ Q, prove P and prove Q independently, then combine.

```
  ⊢ P    ⊢ Q
  ──────────── ∧I
    ⊢ P ∧ Q
```

**∧-Elimination** (∧E₁, ∧E₂): From P ∧ Q, you can extract either component.

```
  ⊢ P ∧ Q          ⊢ P ∧ Q
  ──────── ∧E₁     ──────── ∧E₂
    ⊢ P               ⊢ Q
```

In Lean 4: `⟨hp, hq⟩` applies ∧I; `h.1` and `h.2` apply ∧E₁ and ∧E₂.

Harmony check: ∧I builds a pair from two proofs; ∧E projects back to either proof. If you introduce and immediately eliminate, you get back exactly what you started with: from `⊢ P` and `⊢ Q`, build `⊢ P ∧ Q`, then extract `⊢ P`. No information gained or lost.

## Implication Rules: The Heart of Deduction

**→-Introduction** (→I): To prove P → Q, assume P as a hypothesis and derive Q. The hypothesis P is then **discharged** — it is no longer an assumption but has been "consumed" by the rule.

```
  [P]
   ⋮
   Q
──────── →I
 P → Q
```

The square brackets around P indicate it is a *hypothesis that will be discharged*. This is the formal way of capturing "suppose P ... then Q" reasoning.

**→-Elimination** (→E), also known as **modus ponens**: From P and P → Q, derive Q.

```
  ⊢ P    ⊢ P → Q
  ──────────────── →E
        ⊢ Q
```

These are the most-used rules in mathematics. "Suppose n is even... then n² is even" is an application of →I. "Since n is even and every even number has an even square, n² is even" is an application of →E.

**Why hypothesis discharge matters**: In formal proof systems, we must be explicit about which hypotheses are "in scope" at each step of a proof. Discharged hypotheses are no longer available. This prevents the circular use of assumptions.

## Disjunction Rules

**∨-Introduction** (∨I₁, ∨I₂): From a proof of either disjunct, conclude the disjunction.

```
    ⊢ P                ⊢ Q
────────── ∨I₁       ────────── ∨I₂
  ⊢ P ∨ Q             ⊢ P ∨ Q
```

**∨-Elimination** (∨E): From P ∨ Q, and from proofs that both P → R and Q → R, conclude R. This is proof by cases.

```
  ⊢ P ∨ Q    [P] ⋮ R    [Q] ⋮ R
  ─────────────────────────────── ∨E
                  ⊢ R
```

In Lean 4: `h.elim (fun hp => ...) (fun hq => ...)` applies ∨E.

Notice that ∨-elimination commits to handling *both* cases: you must prove R under the assumption P *and* under the assumption Q. This is the formal counterpart of "we consider two cases" in mathematical prose.

## Negation and the Law of Excluded Middle

**¬-Introduction** (¬I): To prove ¬P, assume P and derive ⊥ (contradiction). Then discharge the assumption P.

```
  [P]
   ⋮
   ⊥
──────── ¬I
  ¬P
```

**¬-Elimination** (¬E): From P and ¬P, conclude ⊥.

```
  ⊢ P    ⊢ ¬P
  ──────────── ¬E
       ⊢ ⊥
```

**⊥-Elimination** (⊥E), also known as *ex falso quodlibet* ("from false, anything"): From ⊥, conclude any formula.

```
  ⊢ ⊥
──────── ⊥E
  ⊢ Q
```

These rules, together, give us **intuitionistic logic** — the default logic of Lean and Coq. They are sufficient for constructive mathematics.

**Double Negation Elimination (DNE)**: To get **classical logic**, we add one more rule:

```
  ⊢ ¬¬P
──────── DNE
  ⊢ P
```

This rule — from "it is not the case that P fails" conclude P — is classically valid but not constructively provable. Adding it (or equivalently, adding the Law of Excluded Middle: P ∨ ¬P) gives classical natural deduction, the basis for classical mathematics.

## Why Natural Deduction Beats Axiomatic Systems

Earlier logicians (Frege, Russell, Hilbert) formalized logic using **axiomatic systems**: a small set of axioms plus a single rule of inference (modus ponens). This is mathematically elegant but pedagogically opaque. In an axiomatic system, the "insight" behind a proof is often hidden in a choice of axiom to apply; the reasoning does not resemble how mathematicians think.

Natural deduction makes the reasoning structure transparent. Each step in a proof corresponds to a recognizable reasoning move. The structure of the proof *mirrors* the structure of the conclusion: the proof of P ∧ Q has two subproofs (mirroring ∧I); the proof of P → Q has a subproof under a hypothesis (mirroring →I); the proof of P ∨ Q by cases has two subproofs from the two disjuncts (mirroring ∨E).

This structural correspondence is what makes natural deduction proofs readable and teachable — and what makes Lean 4's tactic proofs feel like mathematical argument rather than symbol manipulation.

---

*Next: The sequent calculus — a symmetric reformulation that enables the cut-elimination theorem.*
