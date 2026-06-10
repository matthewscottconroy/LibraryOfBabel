# Sequent Calculus

## Overview
Gentzen's sequent calculus LK provides a symmetric view of proof: a **sequent**
Γ ⊢ Δ has a multiset of formulas on each side. Left rules decompose premises;
right rules decompose conclusions. Cut elimination (Gentzen's Hauptsatz) is the
central metatheorem.

## Learning Objectives
- Read and write sequent calculus derivations
- Apply left and right rules for each connective
- State the cut rule and explain its eliminability

## Sequents
A **sequent** Γ ⊢ Δ (Γ and Δ are finite multisets of formulas) is valid iff:
whenever all formulas in Γ are true, at least one formula in Δ is true.

## Selected Rules

### Identity and Cut
```
──────── (Ax)     Γ ⊢ Δ, φ    φ, Γ' ⊢ Δ'
φ ⊢ φ             ─────────────────────── (Cut)
                        Γ, Γ' ⊢ Δ, Δ'
```

### Conjunction
```
Γ, φ, ψ ⊢ Δ               Γ ⊢ Δ, φ    Γ ⊢ Δ, ψ
──────────── (∧L)           ─────────────────────── (∧R)
Γ, φ∧ψ ⊢ Δ                    Γ ⊢ Δ, φ∧ψ
```

### Implication
```
Γ ⊢ Δ, φ    ψ, Γ' ⊢ Δ'       φ, Γ ⊢ Δ, ψ
──────────────────────── (→L)  ──────────── (→R)
  Γ, Γ', φ→ψ ⊢ Δ, Δ'          Γ ⊢ Δ, φ→ψ
```

## Cut Elimination
The **cut rule** is admissible: every proof using cut can be transformed into a
cut-free proof. This is Gentzen's Hauptsatz. Consequences:
- **Consistency**: ⊢ ⊥ has no proof (cut-free proofs are subformula-closed)
- **Subformula property**: every formula in a cut-free proof is a subformula of the endsequent
- **Decidability**: cut-free proof search terminates for propositional logic

## Tool Connections
- **Coq**: the `cut` tactic introduces the cut rule explicitly
- **logitext.mit.edu**: interactive sequent calculus prover

## Exercises
See `problems/ch04_proof_systems/02_sequent_calculus_exercises.md`
