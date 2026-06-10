# Conjunction Rules in Natural Deduction

## Introduction and Elimination

In Gentzen's natural deduction, every connective is characterized by two kinds of rules:
- **Introduction rules** (I): how to *prove* a formula with that connective as main operator
- **Elimination rules** (E): how to *use* a formula with that connective as main operator

This symmetric design is the hallmark of natural deduction. Conjunction (∧) has the clearest and most symmetric pair of all.

## The Rules

**Conjunction Introduction (∧I):**

$$\frac{\Gamma \vdash \varphi \qquad \Gamma \vdash \psi}{\Gamma \vdash \varphi \wedge \psi} \; (\wedge I)$$

To prove "P and Q," prove P separately and prove Q separately.

**Conjunction Elimination (∧E₁ and ∧E₂):**

$$\frac{\Gamma \vdash \varphi \wedge \psi}{\Gamma \vdash \varphi} \; (\wedge E_1) \qquad \frac{\Gamma \vdash \varphi \wedge \psi}{\Gamma \vdash \psi} \; (\wedge E_2)$$

From "P and Q," you can extract P (left projection) or Q (right projection).

## Worked Example: ∧ is Commutative

**Claim**: $\varphi \wedge \psi \vdash \psi \wedge \varphi$ — if we have P∧Q, we can derive Q∧P.

```
1. φ ∧ ψ              [assumption]
2. ψ                  [∧E₂ on 1]
3. φ                  [∧E₁ on 1]
4. ψ ∧ φ              [∧I on 2, 3]
```

This is exactly what you would say informally: "If P and Q, then certainly Q, and certainly P, so Q and P."

## In Lean 4

```lean
-- ∧ introduction
example (hp : P) (hq : Q) : P ∧ Q := And.intro hp hq

-- ∧ elimination
example (h : P ∧ Q) : P := h.left      -- ∧E₁
example (h : P ∧ Q) : Q := h.right     -- ∧E₂

-- Commutativity
theorem and_comm (h : P ∧ Q) : Q ∧ P :=
  And.intro h.right h.left

-- Associativity
theorem and_assoc (h : (P ∧ Q) ∧ R) : P ∧ (Q ∧ R) :=
  And.intro h.left.left (And.intro h.left.right h.right)
```

## The Local Soundness / Completeness Perspective

A critical observation: the ∧I/∧E pair satisfies **local reduction** (soundness at the micro level):

If you introduce a conjunction and immediately eliminate it:
```
[proof of φ]     [proof of ψ]
─────────────────────────────   ∧I
         φ ∧ ψ
         ───────               ∧E₁
           φ
```

This reduces to simply: [proof of φ]. The detour through ∧ is eliminable. This is the key to the **normalization theorem** — every natural deduction proof can be transformed into one with no "detours" (no introduction immediately followed by elimination of the same connective). The normalized proof is the one that mirrors the direct mathematical argument.

## Exercises
See [problems/ch04_proof_systems/](../../../problems/ch04_proof_systems/)
