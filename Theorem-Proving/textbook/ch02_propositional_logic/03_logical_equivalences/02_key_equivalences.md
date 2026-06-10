# Key Logical Equivalences

## Overview
A comprehensive list of the major propositional equivalences, organized by family.
These are the algebraic laws of propositional logic — the rewrite rules that let us
transform formulas into equivalent, simpler, or more useful forms.

## Learning Objectives
- Name and state the major equivalence families
- Apply equivalences in forward and backward directions
- Use them to simplify and transform formulas

## The Major Equivalences

### Double Negation
```
¬¬φ ≡ φ
```

### Commutativity
```
φ ∧ ψ ≡ ψ ∧ φ
φ ∨ ψ ≡ ψ ∨ φ
```

### Associativity
```
(φ ∧ ψ) ∧ χ ≡ φ ∧ (ψ ∧ χ)
(φ ∨ ψ) ∨ χ ≡ φ ∨ (ψ ∨ χ)
```

### Distributivity
```
φ ∧ (ψ ∨ χ) ≡ (φ ∧ ψ) ∨ (φ ∧ χ)
φ ∨ (ψ ∧ χ) ≡ (φ ∨ ψ) ∧ (φ ∨ χ)
```

### Absorption
```
φ ∧ (φ ∨ ψ) ≡ φ
φ ∨ (φ ∧ ψ) ≡ φ
```

### Idempotency
```
φ ∧ φ ≡ φ
φ ∨ φ ≡ φ
```

### Identity and Annihilation
```
φ ∧ ⊤ ≡ φ       φ ∧ ⊥ ≡ ⊥
φ ∨ ⊥ ≡ φ       φ ∨ ⊤ ≡ ⊤
```

### Implication
```
φ → ψ ≡ ¬φ ∨ ψ                    (material implication)
φ → ψ ≡ ¬ψ → ¬φ                   (contrapositive)
(φ → ψ) ∧ (φ → χ) ≡ φ → (ψ ∧ χ)  (exportation)
```

### Biconditional
```
φ ↔ ψ ≡ (φ → ψ) ∧ (ψ → φ)
φ ↔ ψ ≡ (φ ∧ ψ) ∨ (¬φ ∧ ¬ψ)
```

## Exercises
See `problems/ch02_propositional_logic/02_equivalence_proofs.md`
