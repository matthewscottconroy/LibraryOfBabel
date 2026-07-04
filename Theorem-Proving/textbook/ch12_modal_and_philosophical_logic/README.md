# Chapter 12: Modal and Philosophical Logic

Necessarily, possibly, always, known, obligatory — one pair of operators, $\Box$ and $\Diamond$, gives a single mathematical framework for all of these modalities, and Kripke's possible-worlds semantics makes it precise.

## Overview

The core is propositional **modal logic**: the language adds $\Box\phi$ ("necessarily") and its dual $\Diamond\phi \equiv \neg\Box\neg\phi$ ("possibly"), read alethically, epistemically, deontically, temporally, or as provability. A **Kripke frame** $(W, R)$ supplies worlds and an accessibility relation; a **Kripke model** $(W, R, V)$ adds a valuation, with $\mathcal{M}, w \vDash \Box\phi$ iff $\phi$ holds at every $R$-successor of $w$. The base system **K** consists of tautologies, the distribution axiom $\Box(\phi \to \psi) \to (\Box\phi \to \Box\psi)$, and necessitation. **Correspondence theory** matches axioms to frame properties: T ($\Box\phi \to \phi$) $\leftrightarrow$ reflexivity, 4 ($\Box\phi \to \Box\Box\phi$) $\leftrightarrow$ transitivity, 5 ($\Diamond\phi \to \Box\Diamond\phi$) $\leftrightarrow$ the Euclidean property, B $\leftrightarrow$ symmetry, D ($\Box\phi \to \Diamond\phi$) $\leftrightarrow$ seriality — yielding the hierarchy **K, T, S4** (preorders; complete via Gödel's translation for intuitionistic logic) and **S5** (equivalence relations; metaphysical necessity). Bisimulation characterizes modal indistinguishability, and provability logic **GL** adds Löb's axiom $\Box(\Box\phi \to \phi) \to \Box\phi$, arithmetically complete for PA-provability (Solovay).

Three applied modal families follow. **Temporal logic**: LTL with operators $X$, $F$, $G$, $U$, $R$ and the safety/liveness/fairness specification patterns; CTL with path quantifiers $A$/$E$ ($AG$, $EF$, $AF$, $EG$), incomparable in expressive power with LTL; and **model checking** — polynomial-time CTL labeling, with the state-explosion problem attacked by BDD-based symbolic checking, SAT-based bounded model checking, and CEGAR. **Epistemic logic**: $K_i\phi$ with S5 (KT45) for knowledge — factivity (T), positive (4) and negative (5) introspection — and KD45 for belief; the muddy children puzzle, **common knowledge** $C\phi$ versus mutual knowledge $E\phi$, the coordinated attack impossibility, and public announcement logic. **Deontic logic**: obligation $O$, permission $P$, prohibition $F$ in system KD ($O\phi \to P\phi$) with ideal-world semantics, and the paradoxes of Ross, the Good Samaritan, and Chisholm.

The chapter ends with **philosophy of logic**: logicism (Frege's program, Hume's Principle, the inconsistency of Basic Law V via Russell's paradox, *Principia Mathematica*, Wright's neo-logicism and Frege's Theorem); formalism (Hilbert's program and its Gödelian refutation; proof assistants as its partial realization); intuitionism (Brouwer's rejection of LEM, the BHK interpretation, Heyting's formalization, the double-negation translation, Martin-Löf type theory); Platonism vs. nominalism (Wigner's applicability puzzle, the Quine–Putnam indispensability argument, Benacerraf's dilemma, Field's fictionalism, structuralism); and truth, meaning, and reference (Frege's sense/reference distinction, Tarski's T-schema and the object/metalanguage split, compositionality, Kripke's rigid designators).

## Why It Matters

Modal logic is the bridge from pure logic to its applications and its philosophy. Kripke structures are the system models that Chapter 13's model checkers verify and Chapter 14's temporal logics refine; GL modalizes Chapter 10's provability predicate; and the intuitionism debate explains why Lean and Coq (Chapters 11, 13) are constructive at their core.

## Chapter Roadmap

1. [Modal Logic](01_modal_logic/01_necessity_and_possibility.md) — $\Box$/$\Diamond$, kinds of necessity, the K axiom; further files develop Kripke semantics with a Python model checker, frame validity and bisimulation, and the systems K/T/S4/S5.
2. [Temporal Logic](02_temporal_logic/01_ltl.md) — LTL operators and specification patterns; companion files on CTL and on model checking (BDDs, bounded model checking, CEGAR).
3. [Epistemic Logic](03_epistemic_logic/01_knowledge_and_belief.md) — knowledge as S5, belief as KD45, introspection, muddy children; a second file on common knowledge and coordinated attack.
4. [Deontic Logic](04_deontic_logic/01_obligation_permission.md) — obligation and permission in KD, and the classic deontic paradoxes.
5. [Philosophy of Logic](05_philosophy_of_logic/01_logicism.md) — logicism; further files on formalism, intuitionism, Platonism vs. nominalism, and truth/meaning/reference.

## Prerequisites

Propositional logic (Chapter 2), first-order semantics (Chapter 3), and proof systems (Chapter 4). Chapter 10's incompleteness theorems are assumed in the discussions of provability logic and formalism.
