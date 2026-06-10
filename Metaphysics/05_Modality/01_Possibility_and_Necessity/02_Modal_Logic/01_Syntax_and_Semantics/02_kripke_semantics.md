# Kripke Semantics for Modal Logic

Modal logic had axioms and inference rules long before it had a semantics. The question of what modal formulas *mean* — what makes them true or false — was answered by the possible-worlds semantics developed by Saul Kripke in the late 1950s and early 1960s, alongside parallel work by Jaakko Hintikka, Stig Kanger, and others. Kripke published his first completeness results at nineteen. What emerged was a rigorous model-theoretic framework that connected formal axioms to conditions on a relation between worlds — a connection with deep philosophical implications.

## The Core Definitions

A **Kripke frame** is a pair F = ⟨W, R⟩ where W is a non-empty set (the possible worlds) and R ⊆ W × W is the **accessibility relation**: wRv reads "v is accessible from w," or equivalently "from the perspective of w, v is a genuine possibility."

A **Kripke model** M = ⟨W, R, V⟩ adds a valuation function V: Prop × W → {T, F}, assigning truth values to atomic propositions at each world.

The semantic clauses for complex formulas are defined by structural induction:

- M, w ⊨ p iff V(p, w) = T
- M, w ⊨ ¬φ iff M, w ⊭ φ
- M, w ⊨ φ ∧ ψ iff M, w ⊨ φ and M, w ⊨ ψ
- M, w ⊨ □φ iff for all v ∈ W such that wRv, M, v ⊨ φ
- M, w ⊨ ◇φ iff there exists v ∈ W such that wRv and M, v ⊨ φ

In plain English: □φ is true at w if and only if φ is true at every world accessible from w. ◇φ is true at w if and only if φ is true at some world accessible from w. The accessibility relation controls which worlds count as relevant when evaluating modal claims.

## Correspondence Theory

The power of Kripke semantics lies in the correspondence between frame conditions on R and modal axioms. Each axiom schema is valid precisely over the class of frames where R satisfies the corresponding relational property:

| Axiom | Schema | Frame condition |
|-------|--------|-----------------|
| T     | □P → P | R is reflexive: ∀w, wRw |
| 4     | □P → □□P | R is transitive: wRv ∧ vRu → wRu |
| 5     | ◇P → □◇P | R is Euclidean: wRv ∧ wRu → vRu |
| B     | P → □◇P | R is symmetric: wRv → vRw |
| D     | □P → ◇P | R is serial: ∀w, ∃v, wRv |

The T axiom (□P → P) is valid at w iff w is accessible from itself: if wRw and □P is true at w, then P is true at w (since every accessible world including w itself satisfies P). The connection between reflexivity and the principle that necessary truths are actual truths is not coincidental — it is exact.

## S5 and the Universal Accessibility Relation

S5 corresponds to frames where R is an equivalence relation — reflexive, symmetric, and transitive. An equivalence relation partitions W into equivalence classes; within each class, all worlds are mutually accessible.

The philosophical consequence: in S5, all worlds are (from a global perspective) mutually accessible. There are no "local" necessities that hold in one world's neighborhood but not another's. The modal space is flat: what is necessary is necessary from any perspective, and what is possible is possible from any perspective. This is expressed by the characteristic S5 equivalences:

- □P ↔ □□P (iterated necessity collapses)
- ◇P ↔ ◇◇P (iterated possibility collapses)
- ◇□P → □P (if possibly necessarily P, then necessarily P)

That last formula is the key step in the modal ontological argument: if possibly necessarily a God exists (◇□∃xGx), then necessarily a God exists (□∃xGx). The inference is valid in S5 but not in weaker systems. S5 is assumed in most metaphysical discussions, including Lewis's modal realism, Plantinga's modal argument, and most possible-worlds semantics for natural language.

## Non-Standard Accessibility

Different modalities correspond to different accessibility relations. Epistemic accessibility — w' is epistemically accessible from w if w' is consistent with everything known at w — is typically reflexive (T) and possibly transitive (S4), but not necessarily Euclidean. Deontic accessibility — w' is deontically ideal relative to w's normative standards — is typically serial (D) but not reflexive (it would mean the actual world is deontically ideal, which seems too optimistic). Temporal accessibility — future-directed, where wRv iff v is a later time than w — is transitive but neither reflexive nor symmetric.

The formal apparatus is thus neutral between different readings: the same logical machinery handles necessity, obligation, knowledge, and time, with the differences borne entirely by the accessibility relation. This is a major source of its power.

## Philosophical Significance

Kripke's completeness theorems show that each standard modal system's theorems are exactly the formulas valid in the corresponding class of frames — syntactic provability matches semantic validity. This established modal logic on a par with classical logic as a rigorous mathematical discipline.

The deeper philosophical question is what the worlds in the models represent. For pure formal logic, they are abstract structures. For modal metaphysics, they represent genuine possibilities. Lewis takes them to be concrete realities; Plantinga takes them to be abstract states of affairs; fictionalists treat them as elements of a theoretical fiction. The formal apparatus is neutral between these interpretations — what varies is the metaphysical loading placed on the semantics.

Timothy Williamson (*Modal Logic as Metaphysics*, 2013) has argued that modal logic, properly understood, is not merely a representation language but is directly constitutive of metaphysical truths. On this view, the formalism of Kripke semantics does not merely describe the structure of modal reality — it is part of that structure. Whether or not one accepts so strong a claim, Kripke semantics remains the indispensable technical foundation for modal metaphysics.
