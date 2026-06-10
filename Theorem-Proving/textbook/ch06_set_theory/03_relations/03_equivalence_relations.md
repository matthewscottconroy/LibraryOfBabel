# Equivalence Relations and Partitions

## Overview
An equivalence relation captures the idea of "same for our purposes." It is reflexive,
symmetric, and transitive. Every equivalence relation partitions its domain into
**equivalence classes** — a fundamental construction in mathematics.

## Learning Objectives
- Define equivalence relation (reflexive, symmetric, transitive)
- Compute equivalence classes for a given relation
- State and prove the partition theorem

## Properties
A relation R on A is:
- **Reflexive**: ∀x xRx
- **Symmetric**: ∀x∀y (xRy → yRx)
- **Transitive**: ∀x∀y∀z (xRy ∧ yRz → xRz)
- **Equivalence relation**: reflexive + symmetric + transitive

## Equivalence Classes
For an equivalence relation R on A, the **equivalence class** of a:
[a]_R = {x ∈ A | xRa}

Key theorem: {[a]_R | a ∈ A} is a **partition** of A — a collection of non-empty,
pairwise disjoint subsets that together cover A.

## Examples
- Congruence mod n on ℤ: a ≡ b (mod n) iff n | (a-b). Classes: [0],[1],...,[n-1]
- Same shape in Tarski's World: equivalence classes are {cubes, tetrahedra, dodecahedra}
- Isomorphism of groups: equivalence on the class of groups
- Homotopy of paths: equivalence on paths (used in topology and HoTT)

## Lean 4
```lean
-- Defining an equivalence relation in Lean
def modEquiv (n : Int) : Int → Int → Prop := fun a b => n ∣ (a - b)

theorem modEquiv_refl (n a : Int) : modEquiv n a a := by simp [modEquiv]
theorem modEquiv_symm (n a b : Int) (h : modEquiv n a b) : modEquiv n b a := by
  simp [modEquiv] at *; omega
```

## Exercises
See `problems/ch06_set_theory/02_relation_property_proofs.md`
