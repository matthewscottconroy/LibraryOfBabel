# The Univalence Axiom

> "Univalence is the formalization of a common practice: mathematicians routinely identify isomorphic structures. Univalence says this identification is not merely convenient — it is *true*."
> — Vladimir Voevodsky

## A Principle Mathematicians Already Believe

Here is something every mathematician does without thinking about it: they treat isomorphic mathematical structures as *identical*. If two groups are isomorphic, they use facts about one to prove things about the other. If two topological spaces are homeomorphic, they speak of "the" space.

But in traditional foundations (ZFC), this practice is slightly dishonest. Two groups can be isomorphic without being literally the same set. The identification is a convenient fiction — justified by the fact that isomorphic groups share all group-theoretic properties, but not literally true in the foundation.

**Homotopy Type Theory (HoTT)** and the **Univalence Axiom** make this honest. They assert that *isomorphic types are identical* — not merely interchangeable, but literally equal as types. This transforms what was a mathematician's informal practice into a foundational principle.

## Background: Identity Types

In Martin-Löf type theory, for any type $A$ and any two terms $a, b : A$, there is an **identity type** $a =_A b$ — the type of *proofs that $a$ equals $b$*.

In ordinary type theory, the identity type for types themselves ($A =_\mathsf{Type} B$) is quite rigid: $A = B$ means $A$ and $B$ are definitionally the same type. This is a very strict notion of equality.

But there is another way for types to be "the same": **equivalence** — a bijection with coherent structure (quasi-isomorphism). Two types $A$ and $B$ are **equivalent**, written $A \simeq B$, if there is a function $f : A \to B$ that is an isomorphism in the appropriate sense (has a quasi-inverse).

There is always a function:
$$\text{idtoequiv} : (A = B) \to (A \simeq B)$$

If $A$ and $B$ are literally equal (as types), then they are certainly equivalent.

**The Univalence Axiom** (Voevodsky) asserts the converse: this map is itself an equivalence:
$$\text{ua} : (A \simeq B) \simeq (A = B)$$

Equivalence *implies* equality. Isomorphic types are identical types.

## What Univalence Enables

**Theorem (Transport)**: If $P : \mathsf{Type} \to \mathsf{Prop}$ is any property of types, and $A \simeq B$, then $P(A) \leftrightarrow P(B)$.

*Proof via Univalence*: By univalence, $A \simeq B$ gives $A = B$. By substitution of equals, $P(A) = P(B)$. $\square$

Without univalence, this requires manually showing that every property $P$ "respects isomorphism" — a tedious obligation. With univalence, it is automatic: equal things have equal properties, and isomorphic things are equal.

**Functional Extensionality**: Two functions $f, g : A \to B$ are equal if they are pointwise equal: $(\forall x : A, f(x) = g(x)) \to f = g$.

This is a consequence of univalence (in HoTT, it follows from the univalence of function types). In ordinary dependent type theory (without univalence), function extensionality must be added as a separate axiom.

**Example: Integer representation**

Consider two representations of integers:
- $\mathbb{Z}_1 = \mathbb{N} + \mathbb{N}$ (positive and negative naturals separately)
- $\mathbb{Z}_2 = \{(a, b) : \mathbb{N} \times \mathbb{N}\} / \sim$ where $(a,b) \sim (c,d)$ iff $a + d = b + c$

Both are isomorphic to "the integers." In HoTT with univalence, both are *equal* as types (assuming the right notion of structure), so any theorem proved about one is automatically a theorem about the other — with no work needed to "transfer" it.

## The Homotopy Interpretation

The name "Homotopy Type Theory" reflects a deep geometric interpretation: types are like **topological spaces**, and terms of an identity type are like **paths** between points.

- A term $p : a = b$ is a path from $a$ to $b$ in the "space" $A$
- Two paths can be **homotoped** (continuously deformed into each other) — and a proof that two paths are equal is a homotopy
- This gives an infinite hierarchy: paths, homotopies between paths, homotopies between homotopies, ...

Under this interpretation:
- Ordinary types are like **sets** (discrete spaces — all paths are trivial)
- Types with non-trivial identity types are like spaces with non-trivial fundamental groups
- The univalence axiom corresponds to the fact that a homotopy equivalence of spaces *is* a path between them (in the "space of spaces")

**$n$-Types** (Homotopy levels):
- **-1-types** (mere propositions): at most one element up to identity — truth values
- **0-types** (sets): all identity proofs are equal — classical sets
- **1-types** (groupoids): identity proofs can differ but are themselves equal up to homotopy
- **$n$-types** (homotopy $n$-types): the homotopy hierarchy continues

Most of standard mathematics lives at the level of sets (0-types). Higher structures arise naturally in homotopy theory, algebraic topology, and category theory.

## Univalence in Lean 4 and Cubical Agda

Standard Lean 4 (based on CIC) adds univalence as an axiom:

```lean
-- In Lean 4, propext and funext are axioms
-- Univalence is not in the kernel but can be stated:
axiom univalence : ∀ {α β : Type u}, (α ≃ β) → α = β

-- Propositional extensionality (consequence of univalence for propositions)
#check propext  -- propext : (a ↔ b) → a = b

-- Function extensionality (follows from univalence)
#check funext   -- funext : (∀ x, f x = g x) → f = g
```

**Cubical Agda** (and Cubical Type Theory more generally) gives a *computational* interpretation of univalence — it is not just an axiom but a computable operation, so that proofs using univalence can be run and computed. This is an active area of research and development.

```agda
-- In Cubical Agda:
-- ua : A ≃ B → A ≡ B
-- is a definable function, not just an axiom
-- transport along (ua e) *computes* using e
```

## Why Univalence Matters for Foundations

Univalence resolves a long-standing tension in foundations between:
- **Extensional** type theories: equality is semantic, but type checking can be undecidable
- **Intensional** type theories: equality is syntactic (definitional), type checking is decidable, but equality is too weak

HoTT with univalence gives a middle path: equality is semantic (isomorphism implies equality) but the system remains consistent and usable. The *homotopy* interpretation explains why different "proofs of equality" (paths) can coexist without contradiction.

For the practicing mathematician, univalence is the formalization of the common faith that "isomorphic structures are interchangeable" — finally made rigorous and automatically usable in a proof assistant.

## Current Status

HoTT and cubical type theory are active research frontiers:
- Lean 4's Mathlib does not use univalence extensively (it uses classical logic instead)
- Cubical Agda has a fully computational interpretation
- The HoTT book (Voevodsky et al., 2013) formalized a large body of homotopy theory in Coq
- "Synthetic homotopy theory" uses HoTT to develop algebraic topology with proofs verified in a proof assistant

## Exercises
See [problems/ch11_type_theory/04_hott_exercises.md](../../../problems/ch11_type_theory/04_hott_exercises.md)
