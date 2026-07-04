# Chapter 15: Plural Logic

First-order logic speaks of *one* thing at a time. Plural logic speaks of *many* things at once — not a set of things, but the things themselves, plurally — and in doing so it recovers the expressive power of second-order logic while claiming to add no new objects to the world.

## Overview

Ordinary English quantifies plurally: "there are some critics who admire only one another," "the natural numbers satisfy Peano's axioms." George Boolos, in *To Be Is to Be a Value of a Variable (or to Be Some Values of Some Variables)* (1984), argued that such talk is not covert quantification over sets but an irreducible logical device in its own right, and built a formal system — **plural first-order logic (PFO)** — to regiment it. The language of Chapter 3 is extended with **plural variables** $xx, yy, zz$, the single new predicate $x \prec xx$ ("$x$ is one of the $xx$"), plural quantifiers $\exists xx$ and $\forall xx$, and a **plural comprehension** schema $\exists x\,\phi(x) \to \exists xx\,\forall x(x \prec xx \leftrightarrow \phi(x))$ that looks like naïve set comprehension but generates no paradox, because a plurality is not an object and $xx \prec xx$ is not even grammatical.

The chapter develops the formal system and its proof theory (introduction and elimination rules for plural quantifiers, the comprehension schema, the indiscernibility of coextensive pluralities) with worked derivations, then proves Boolos's central expressiveness result: **plural logic is expressively equivalent to monadic second-order logic**, translating $X$ to $xx$ and $X(t)$ to $t \prec xx$. Given a pairing device in the first-order base, plurals reach the strength of full second-order logic — enough to give the *categorical* second-order Dedekind–Peano axioms and to define finiteness, both impossible in first-order logic (Chapter 9, Löwenheim–Skolem). The prize Boolos claimed is **ontological innocence**: this power costs no commitment to sets, classes, or properties. That thesis is examined critically (Resnik, Parsons, Linnebo) alongside the later developments — Oliver and Smiley's systematic *Plural Logic*, Rayo and Yablo's semantics, Linnebo and Rayo's superplural hierarchy — and the applications that make plural logic matter: the semantics of mathematics, Lewis's **megethology** (mereology plus plurals simulating set theory), absolutely unrestricted quantification, and a nominalist-friendly reading of Frege's program.

## Why It Matters

Plural logic sits on the fault line between *logic* and *set theory*. If Boolos is right, second-order quantification is genuine logic, not "set theory in sheep's clothing" (Quine), and Frege's dream of reducing arithmetic to logic survives its paradoxes. Plural quantification is the standard modern tool for defending **absolute generality** (quantifying over absolutely everything, including all sets) and for reconstructing mathematics nominalistically. It is also the correct semantics for a large fragment of natural language — collective predication that no singular paraphrase captures — and it connects directly to Chapter 16: mereology plus plurals is Lewis's reduction of set theory to the theory of parts and size.

## Chapter Roadmap

1. [Singular vs. Plural Reference](01_foundations/01_singular_vs_plural.md) — plural terms and plural reference, distributive vs. collective predication, and why first-order logic can paraphrase plurals only by smuggling in sets.
2. [Boolos's Plural Quantification](01_foundations/02_boolos_plural.md) — the Geach–Kaplan sentence, the language PFO$^+$, the ontological-innocence thesis, and the link to neo-logicism.
3. [The Formal System and Its Proof Theory](02_formal_system/01_pfo_and_proof_theory.md) — full syntax, the comprehension schema, plural quantifier rules, indiscernibility, worked derivations, and the plural semantics.
4. [Second-Order Logic and Expressive Power](02_formal_system/02_expressive_power.md) — the interpretation of monadic second-order logic, defining finiteness and categorical arithmetic, and the innocence debate.
5. [Higher-Order Plurals and the Semantics of Mathematics](03_developments/01_higher_order_and_foundations.md) — superplurals (Rayo, Linnebo), megethology, plural set theory, absolute generality, and paradox avoidance.

## Prerequisites

- [Chapter 3: First-Order Logic](../ch03_first_order_logic/) — plural logic is an extension of the first-order language and its deductive apparatus.
- [Chapter 6: Set Theory](../ch06_set_theory/) — the essential contrast: what plural comprehension gives without positing a set.
- [Chapter 9: Model Theory](../ch09_model_theory/) — Löwenheim–Skolem and compactness, the first-order limits that plurals transcend.
- Helpful: [Chapter 16: Mereology](../ch16_mereology/) — megethology combines plural quantification with mereological fusion.
