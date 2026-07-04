# Chapter 16: Mereology

The logic of *parts and wholes* — an alternative to set theory as a foundation for ontology, and a first-order theory with a very different fate: where set theory is undecidable, classical mereology can be decided.

## Overview

Mereology, founded by Stanisław Leśniewski (1916) and redeveloped by Leonard and Goodman as the Calculus of Individuals (1940), takes a single primitive — **parthood**, $P(x,y)$ — and axiomatizes it as a **partial order**: reflexive, antisymmetric, transitive (Ground Mereology **M**). From $P$ one defines **proper parthood** ($PP(x,y) \equiv P(x,y) \land x \neq y$), **overlap** ($O(x,y) \equiv \exists z(P(z,x) \land P(z,y))$), disjointness, and underlap, and one distinguishes **atoms** (objects with no proper parts, $\mathrm{Atom}(x) \equiv \neg\exists y\,PP(y,x)$) from **gunk** (objects whose parts all have further proper parts).

Strengthening M yields the classical systems. **Weak Supplementation** demands a remainder whenever a proper part falls short of its whole; **Strong Supplementation** yields the **extensionality** of parthood — composite objects with the same proper parts are identical, a theorem we prove. Adding the **unrestricted fusion** axiom schema (every instantiated condition $\phi$ has a mereological sum) produces **General Extensional Mereology (GEM)**, whose models Tarski showed to be exactly complete Boolean algebras with the bottom element removed. GEM proves the existence of products, sums, differences, and a universal object — but tolerates no empty object: there is no null individual.

The comparison with set theory is a study in contrasts: membership is neither reflexive nor transitive while parthood is both; $\{x\} \neq x$ but fusions add no new layer; David Lewis's *Parts of Classes* reconstructs set theory as mereology plus a singleton function. Sharpest of all is the **decidability asymmetry**: the first-order theory of GEM (with or without atoms) is decidable, via Tarski's decision procedure for Boolean algebras, while ZF set theory is essentially undecidable and incomplete. The applications sections bring the formalism to bear on the statue and the clay (material constitution), Peter van Inwagen's **Special Composition Question** (nihilism, universalism, restricted composition, and the Lewis–Sider vagueness argument), temporal parts, and **mereotopology** — connection, RCC-8, and its NP-complete constraint calculus used in GIS and ontologies.

## Why It Matters

Mereology is where formal logic meets ontology: precise axioms (stated in first-order logic), genuine theorems (extensionality, Boolean representation, products from overlap), and live metaphysical stakes (does the statue = the clay? when do things compose?). It also delivers one of the cleanest decidability contrasts in logic — a theory of "collections" that quantifier elimination can tame — and its topological extension powers real systems: qualitative spatial reasoning, geographic information systems, and biomedical ontologies, with SAT/SMT encodings linking it to formal verification.

## Chapter Roadmap

1. [Parts and Wholes](01_foundations/01_parts_and_wholes.md) — the parthood primitive, the partial-order axioms of M, defined notions (proper part, overlap, disjointness, underlap), the transitivity objection, atoms and gunk.
2. [Classical Mereology](01_foundations/02_classical_mereology.md) — weak and strong supplementation, the extensionality theorem, the fusion schema, GEM, Tarski's Boolean algebra theorem, and the overlap-implies-product proof.
3. [Mereology vs. Set Theory](02_comparison/01_mereology_vs_sets.md) — membership vs. parthood, no null individual, singletons, Lewis's *Parts of Classes* and megethology, composition as identity, decidable GEM vs. undecidable ZF.
4. [Physical Objects](03_applications/01_physical_objects.md) — the statue and the clay, the Special Composition Question, the vagueness argument for universalism, temporal parts, the trout-turkey.
5. [Mereotopology](03_applications/02_mereotopology.md) — the connection primitive, Whitehead's point-free geometry, RCC-8 and its composition table, NP-completeness and tractable fragments, GIS and ontology applications.

## Prerequisites

- [Chapter 3: First-Order Logic](../ch03_first_order_logic/) — all axiom systems here are first-order theories.
- [Chapter 6: Set Theory](../ch06_set_theory/) — the essential foil for the comparison section.
- Helpful: [Chapter 15: Plural Logic](../ch15_plural_logic/) — plural quantification underlies megethology and cardinality talk without sets.
