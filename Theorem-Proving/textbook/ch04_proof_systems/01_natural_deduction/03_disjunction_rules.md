# Disjunction Rules in Natural Deduction

## The Asymmetry of Disjunction

Conjunction has a simple, perfectly symmetric pair of rules. Disjunction is more interesting: its elimination rule is significantly more complex than its introduction rules, reflecting the deeper computational content of disjunction.

**Disjunction Introduction (∨I₁ and ∨I₂):**

$$\frac{\Gamma \vdash \varphi}{\Gamma \vdash \varphi \vee \psi} \; (\vee I_1) \qquad \frac{\Gamma \vdash \psi}{\Gamma \vdash \varphi \vee \psi} \; (\vee I_2)$$

To prove P∨Q, you can either prove P (left injection) or prove Q (right injection). Simple — but notice you must choose *which* disjunct you are proving.

**Disjunction Elimination (∨E) — Case Analysis:**

$$\frac{\Gamma \vdash \varphi \vee \psi \qquad \Gamma, \varphi \vdash \chi \qquad \Gamma, \psi \vdash \chi}{\Gamma \vdash \chi} \; (\vee E)$$

From "P or Q," to conclude R, you must handle both cases:
- Case 1: Assume P holds, derive R
- Case 2: Assume Q holds, derive R

This is the familiar proof technique of **case analysis**: no matter which disjunct holds, we reach the same conclusion.

## Why ∨E is Hard

The difficulty is that when you have $\varphi \vee \psi$, you do not know *which* disjunct is true. You cannot simply "extract" either component (the way ∧E extracts components of a conjunction). You can only use the disjunction by showing your conclusion follows regardless.

This is why, in the Curry-Howard correspondence, $\varphi \vee \psi$ corresponds to a **sum type** (also called a disjoint union or tagged union in programming) — to use a value of type `A | B`, you must pattern match on it, handling both `Left a` and `Right b` cases.

## Worked Example: Commutativity of ∨

**Claim**: $\varphi \vee \psi \vdash \psi \vee \varphi$

```
1. φ ∨ ψ                [assumption]
2.   φ                  [assume φ for case 1]
3.   ψ ∨ φ              [∨I₂ on 2]
4.   ψ                  [assume ψ for case 2]
5.   ψ ∨ φ              [∨I₁ on 4]
6. ψ ∨ φ                [∨E on 1, 2-3, 4-5]
```

## In Lean 4

```lean
-- ∨ introduction
example (hp : P) : P ∨ Q := Or.inl hp
example (hq : Q) : P ∨ Q := Or.inr hq

-- ∨ elimination: case analysis
example (h : P ∨ Q) (f : P → R) (g : Q → R) : R :=
  h.elim f g

-- Commutativity
theorem or_comm (h : P ∨ Q) : Q ∨ P :=
  h.elim (fun hp => Or.inr hp) (fun hq => Or.inl hq)

-- Proof by cases (idiomatic Lean)
theorem or_comm' (h : P ∨ Q) : Q ∨ P := by
  cases h with
  | inl hp => exact Or.inr hp
  | inr hq => exact Or.inl hq
```

## Constructive vs. Classical

Disjunction in **constructive logic** is particularly strong: if you prove $\varphi \vee \psi$ constructively, you must know *which* disjunct holds — you must have either a proof of $\varphi$ or a proof of $\psi$ in hand.

In **classical logic**, you can derive disjunctions without committing to which side: for example, $P \vee \neg P$ (excluded middle) is classically valid but not constructively provable — we do not always know which of $P$ or $\neg P$ holds.

This distinction matters in Lean and Coq: adding `open Classical` or importing classical axioms makes LEM available; without them, every proof of a disjunction must exhibit which disjunct holds.

## Exercises
See [problems/ch04_proof_systems/](../../../problems/ch04_proof_systems/)
