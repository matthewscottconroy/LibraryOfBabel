# 43.5 The Formalization Program

## 43.5.1 Formalizing Ergodic Theory in HoTT

The formalization program asks: can we prove the theorems of this book in a proof assistant, using HoTT foundations? The answer is: partially yes, mostly not yet, and the obstacles are revealing.

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

The almost-everywhere problem is particularly deep. In classical mathematics, "almost everywhere" is defined negatively: "except on a set of measure zero." In constructive mathematics, negative definitions are problematic — you can't construct the exceptional set, you can only say it's small. Reformulating ergodic theory around positive statements ("the orbit visits this set frequently") rather than negative ones ("the orbit is not in this null set") requires rebuilding much of the theory from scratch.

## 43.5.2 Synthetic Dynamical Systems

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

This is a beautiful unification. Cantor's theorem, Gödel's incompleteness, Rice's theorem (from Chapter 27), and Curry's paradox all share the same logical structure: they all follow from the non-existence of a certain surjection. In HoTT, this becomes Lawvere's theorem — a single abstract theorem that captures all of these as special cases. The "dynamics" of self-reference and diagonalization has a precise type-theoretic home.
