# Chapter 22: Metaphysics and the Nature of Objects

Metaphysics regimented — existence, identity, and abstractness treated not as pictures of a shadowy realm but as questions about **quantifiers, the identity predicate, and the models a formal theory admits**.

## Overview

Metaphysics is the oldest and least tractable branch of philosophy, and for most of its history its disputes were conducted in words about words. The twentieth century changed the terms of engagement: once existence is expressed by a quantifier and identity by a logical predicate, ontological questions acquire a formal skeleton, and progress becomes a matter of getting the logic right. This chapter treats the central metaphysical topics through that skeleton.

We begin with **Quine's criterion** — "to be is to be the value of a bound variable" — which turns "what is there?" into a question about the quantifiers of a regimented theory, and with the free logics that arise once we allow empty names and non-existent objects. **Identity** is next: Leibniz's Law and its two directions, its second-order definability, the three-line proof of the *necessity of identity*, and the puzzles (statue and clay, contingent identity) where it collides with modality and mereology. We then turn to **abstract objects** — Benacerraf's epistemological dilemma, the Quine–Putnam indispensability argument, Field's nominalist reply, and the neo-Fregean recovery of arithmetic from Hume's Principle — and to the **realism/antirealism** debate as Dummett recast it: not a picture but a choice of *logic*, with bivalence and classical logic on the realist side and intuitionism on the antirealist. A final section develops **formal ontology** proper: properties and universals in higher-order logic, possible-worlds semantics as the metaphysics of modality, essence and the Kripkean *de re*, and Fine's argument that essence outruns necessity.

Throughout, the tools are exactly those of the preceding chapters — first-order and second-order logic (Chapters 3, 15), modal logic (Chapter 12), type theory (Chapter 11), mereology (Chapter 16). Pursued rigorously, the nature of objects turns out to be applied logic.

## Why It Matters

The realism disputes are not idle: which logic a proof assistant adopts — classical LEM in Lean's Mathlib, intuitionistic by default in Coq and Agda — *is* a stance on antirealism, with computational consequences (program extraction, Chapter 11). Quine's criterion is the working tool by which one reads the ontology off any formalized science. And the indispensability and nominalization debates bear directly on what a *foundation* for mathematics must deliver. Metaphysics, done formally, is continuous with logic and computer science rather than opposed to them.

## Chapter Roadmap

1. [Ontology and Quantification](01_ontology/01_what_exists.md) — Quine's criterion, ontological commitment, regimentation, the Russellian dissolution of Meinong, existence as a second-order notion, and free logic for empty terms.
2. [Identity and Individuation](02_identity/01_identity.md) — Leibniz's Law and its converse, identity in first- and second-order logic, the necessity of identity, contingent-identity puzzles, material constitution, and relative identity.
3. [Abstract Objects: Platonism and Nominalism](03_abstract/01_abstract_objects.md) — the abstract/concrete divide, Benacerraf's dilemma, indispensability, Field's conservativeness, neo-Fregean abstraction and Frege's theorem.
4. [Realism, Antirealism, and the Logic of Truth](04_realism/01_realism.md) — platonism and its rivals, Dummett's reframing as a dispute over bivalence, the manifestation argument, intuitionism, and independence phenomena.
5. [Formal Ontology: Properties, Modality, and Essence](05_formal/01_formal_ontology.md) — universals in higher-order logic, possible worlds as formal ontology, *de re* modality and essentialism, and Fine's essence beyond modality.

## Prerequisites

- [Chapter 3: First-Order Logic](../ch03_first_order_logic/) — quantifiers, identity, and models are the medium of the whole chapter.
- [Chapter 12: Modal and Philosophical Logic](../ch12_modal_and_philosophical_logic/) — possible-worlds semantics underlies the treatment of necessity, essence, and modal metaphysics.
- Helpful: [Chapter 15: Plural Logic](../ch15_plural_logic/) and [Chapter 11: Type Theory](../ch11_type_theory/) for higher-order quantification; [Chapter 16: Mereology](../ch16_mereology/) for constitution and nominalism; [Chapter 6: Set Theory](../ch06_set_theory/) for abstraction principles.
