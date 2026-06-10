# Chapter 9: Martin-Löf Type Theory

## Two Publications, One Identity Type, and a Discovery Nobody Planned

Per Martin-Löf published his type theory in 1975 and again in 1984, and the two versions differ in a way that would not seem to matter: can a type contain itself? The first version said yes (Type : Type) and was inconsistent. The second said no, and was not just consistent but the foundation of a new kind of mathematics. Between those two publications, something else happened: Martin-Löf introduced the identity type. He intended it as a technical device for stating extensionality. He had no idea it was a path space.

The identity type a = b is the type whose elements are proofs that a equals b. Martin-Löf added it to give a formal account of what it means to say two things are equal inside the type theory, as opposed to saying they are definitionally equal (which the type checker checks automatically). He gave it one constructor (reflexivity: every thing equals itself) and one elimination rule (the J rule: to prove a property of all equality proofs, it suffices to prove it when the proof is reflexivity).

This seemed like a technical bookkeeping device. Two things suggested otherwise.

First: Martin-Hofmann and Streicher showed in 1994 that Uniqueness of Identity Proofs — the statement that any two proofs of a = b are themselves equal — is not derivable from the J rule. The identity type can, in principle, have multiple distinct elements. This is surprising. In set theory, equality is a relation: either a = b or it does not. There is nothing to distinguish one "proof" of a = b from another. In MLTT, there can be.

Second: Martin-Hofmann and Streicher gave a model — the groupoid model — in which types are groupoids, elements of a type are objects of the groupoid, and elements of a = b are morphisms from a to b. In a nontrivial groupoid, there can be many morphisms between two objects. The identity type, in this model, is genuinely non-trivial.

From there it was only a short step — though it took fifteen more years — to the homotopy interpretation. If elements of a = b are morphisms, they can be composed (transitivity), inverted (symmetry), and they satisfy the groupoid laws. If a = b can have multiple elements, then a =_{a=b} b' (equality between equality proofs) can also have multiple elements, and so on up. Types are not sets. They are infinity-groupoids. They are spaces, in the sense of homotopy theory.

Martin-Löf did not plan this. He was trying to give a careful account of what equality means for a constructive mathematician. What he built, without knowing it, was the logical framework in which homotopy theory becomes synthetic — computable, type-checkable, and precise.

## What This Chapter Covers

This chapter presents MLTT as a formal system — precisely, completely, with all the rules.

**Section 1: The Four Judgments.** MLTT has four primitive forms of assertion: a type is well-formed, two types are definitionally equal, a term has a type, and two terms are definitionally equal. We explain contexts (lists of variable declarations where later types can depend on earlier values), the structural rules, and why four judgments are necessary.

**Section 2: Type Formers.** The full FIEC presentation of all type formers: Π, Σ, +, 𝟙, 𝟘, ℕ, W, and the universe. For each type former, we give the formation rule, all introduction rules, the elimination rule, and all computation rules.

**Section 3: The Identity Type.** Formation, introduction (reflexivity), the J elimination rule in full, the computation rule. We derive symmetry, transitivity, and the groupoid laws. We explain why UIP is not derivable. We gesture at the homotopy interpretation that will be developed later.

**Section 4: Path Induction.** Based and unbased versions of path induction. Deriving each from the other. The contractibility of the based path space. Why "it suffices to handle the refl case" is a non-trivial mathematical fact.

**Section 5: Transport and ap.** Transport: moving elements along paths. ap: applying functions to paths. These are the two workhorses built from J. We prove their basic properties and compute examples.

**Section 6: Intensional vs. Extensional MLTT.** The intensional version (foundation of HoTT): definitional equality is decidable, propositional equality carries information. The extensional version: the reflection rule collapses the two equalities, making type-checking undecidable. Why HoTT requires intensional MLTT.

## Why This Chapter Is the Foundation of Everything

HoTT is built inside MLTT. Every concept — paths, fibrations, equivalences, higher inductive types, univalence — is either a type in MLTT or an axiom added to MLTT. The identity type is the type that paths live in. The J rule is the only way to reason about paths. Transport and ap are derived from J.

Getting MLTT right is not optional background material. It is the work.
