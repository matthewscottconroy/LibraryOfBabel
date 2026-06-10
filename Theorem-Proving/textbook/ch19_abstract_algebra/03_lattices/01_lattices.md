# Lattices and Boolean Algebras

Lattices are the algebraic distillation of order theory — and Boolean algebras are the algebraic structure of classical logic itself.

## Lattices

A *lattice* is a partially ordered set (L, ≤) in which every pair of elements has a *meet* (greatest lower bound, ⊓) and a *join* (least upper bound, ⊔).

Equivalently (algebraically): L is a set with two binary operations ⊓ and ⊔ satisfying:
- Commutativity: a ⊓ b = b ⊓ a, a ⊔ b = b ⊔ a
- Associativity
- Absorption: a ⊓ (a ⊔ b) = a, a ⊔ (a ⊓ b) = a

Examples:
- (𝒫(X), ⊆) with ⊓ = ∩, ⊔ = ∪
- (ℕ, |) with ⊓ = gcd, ⊔ = lcm
- Propositions ordered by implication, with ⊓ = ∧, ⊔ = ∨

## Boolean Algebras

A *Boolean algebra* is a complemented distributive lattice: a lattice with a bottom ⊥, top ⊤, and complement operation ¬ satisfying:
- Distributivity: a ⊓ (b ⊔ c) = (a ⊓ b) ⊔ (a ⊓ c)
- Complement: a ⊓ ¬a = ⊥, a ⊔ ¬a = ⊤

**Stone's Representation Theorem** (1936): Every Boolean algebra is isomorphic to a field of sets — a subalgebra of some 𝒫(X).

This theorem connects the abstract algebraic axioms to concrete set-theoretic structures. It's the prototype of *representation theorems* throughout mathematics: abstract structures have concrete models.

## Heyting Algebras

A *Heyting algebra* is a distributive lattice with a *relative pseudo-complement*: an operation → such that:

c ≤ (a → b) iff (c ⊓ a) ≤ b

Heyting algebras are to *intuitionistic logic* as Boolean algebras are to *classical logic*:

| Logic | Algebraic model |
|-------|----------------|
| Classical propositional | Boolean algebra |
| Intuitionistic propositional | Heyting algebra |
| Modal (S4) | Interior algebra |
| Linear logic | *-autonomous categories |

Heyting algebras generalize Boolean algebras: in a Boolean algebra, ¬¬a = a (double negation), but this fails in general Heyting algebras. The law of excluded middle (a ⊔ ¬a = ⊤) holds in Boolean algebras but not Heyting algebras — corresponding exactly to the failure of LEM in intuitionistic logic.

## Completeness Theorem (Algebraic)

The algebraic completeness theorem for classical propositional logic: a propositional formula φ is a tautology iff it is valid in every Boolean algebra.

For intuitionistic logic: φ is intuitionistically valid iff it is valid in every Heyting algebra (or equivalently, in the Heyting algebra of open sets of a topological space).

This gives an alternative to Kripke-frame semantics — purely algebraic, closer to the proof-theoretic foundations.

## Applications in Computer Science

- **Hardware**: Logic gates implement Boolean algebra — AND (⊓), OR (⊔), NOT (¬).
- **Dataflow analysis**: Program analysis uses lattices; the analysis result is a fixed point of a monotone function (Tarski's theorem guarantees existence).
- **Type systems**: Subtype lattices; type intersection and union.
- **Domain theory**: Scott domains (complete partial orders) — the mathematical foundations of denotational semantics for programming languages.
