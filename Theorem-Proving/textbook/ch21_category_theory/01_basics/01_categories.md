# Categories

Category theory, founded by Samuel Eilenberg and Saunders Mac Lane in 1945, abstracts the common structure of mathematical constructions: objects connected by morphisms that compose associatively.

## Definition

A *category* C consists of:
- A collection of **objects**: ob(C)
- For each pair of objects A, B: a set of **morphisms** Hom(A, B), written f : A → B
- For each object A: an **identity morphism** id_A : A → A
- **Composition**: for f : A → B and g : B → C, a morphism g ∘ f : A → C

Satisfying:
- **Unitality**: id_B ∘ f = f and f ∘ id_A = f
- **Associativity**: h ∘ (g ∘ f) = (h ∘ g) ∘ f

## The Universal Examples

| Category | Objects | Morphisms |
|----------|---------|-----------|
| **Set** | Sets | Functions |
| **Grp** | Groups | Group homomorphisms |
| **Top** | Topological spaces | Continuous maps |
| **Vect_k** | Vector spaces over k | Linear maps |
| **Pos** | Posets | Monotone functions |
| **Cat** | Small categories | Functors |
| **Type** | Types | Programs (functions) |

## Small and Large Categories

A category where ob(C) and all Hom(A,B) are sets is *small*. A category where ob(C) is a proper class (like **Set** — all sets) is *large*.

This distinction avoids size paradoxes (Russell's paradox for categories).

## Special Morphisms

- **Isomorphism**: f : A → B such that ∃g : B → A with g ∘ f = id_A and f ∘ g = id_B.
- **Monomorphism** (monic): g ∘ f = h ∘ f implies g = h. (Categorical analogue of injective.)
- **Epimorphism** (epic): f ∘ g = f ∘ h implies g = h. (Categorical analogue of surjective.)

In **Set**: monomorphisms = injections, epimorphisms = surjections, isomorphisms = bijections. In other categories, these notions can diverge.

## The Power of Abstraction

The same theorem, proved once in categorical language, applies to:
- Groups, rings, vector spaces, modules (algebra)
- Topological spaces, sheaves (topology)
- Posets, lattices (order theory)
- Types, programs (computer science)
- Databases, logic (applied)

Category theory identifies when seemingly different structures are "the same" — when a theorem proved in one domain automatically transfers to another via functorial reasoning.

This is not mere analogy. A *functor* is a precise map between categories preserving structure. When two categories are related by a functor, proofs can flow along it.

## Opposite Categories

For any category C, its *opposite* (or *dual*) C^op has the same objects but all arrows reversed: if f : A → B in C, then f^op : B → A in C^op.

Duality is powerful: any theorem in C yields a dual theorem in C^op. In **Set^op**, injection becomes surjection and vice versa. In algebra, limits become colimits, products become coproducts.

The motto: *turn arrows around, and the theory dualizes.*
