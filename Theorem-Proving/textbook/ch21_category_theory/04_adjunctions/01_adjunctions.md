# Adjunctions

Adjunctions are the most important concept in category theory — "the most important concept in mathematics" (Mac Lane). They formalize the universal notion of *optimal solution to a construction problem*.

## Definition

An *adjunction* between functors F : C → D and G : D → C consists of a natural bijection:

```
Hom_D(FA, B) ≅ Hom_C(A, GB)
```

for all A ∈ ob(C) and B ∈ ob(D). We write F ⊣ G ("F is left adjoint to G").

Read: morphisms from FA to B correspond bijectively to morphisms from A to GB, naturally in A and B.

## The Ubiquity of Adjunctions

Almost every fundamental construction in mathematics is part of an adjunction:

| Left adjoint F | Right adjoint G | Category pair |
|----------------|-----------------|---------------|
| Free group | Forgetful functor | Grp ⊣ Set |
| Tensor ⊗ | Internal Hom | Monoidal categories |
| Suspension | Loop space | Topology |
| Pullback | Dependent sum (Σ) | Slices |
| Colimit | Diagonal | Diagrams |
| Existential ∃ | Reindexing | Logic/type theory |
| Reindexing | Universal ∀ | Logic/type theory |

## The Unit and Counit

An adjunction F ⊣ G determines:
- **Unit**: η : id_C → G ∘ F (natural transformation from identity to G after F)
- **Counit**: ε : F ∘ G → id_D (natural transformation from F after G to identity)

Satisfying triangle identities: (εF) ∘ (Fη) = id_F and (Gε) ∘ (ηG) = id_G.

The unit η_A : A → GFA is the "canonical map from A to the best approximation in the image of G". For example, if G forgets group structure and F freely generates a group, then η_A : A → GFA sends each element of set A to its image as a generator in the free group.

## Curry-Howard via Adjunctions

The Curry-Howard correspondence is an adjunction:

In the category of types:
- **Products** (Cartesian): A × B
- **Function types**: A → B = A ⊸ B

The adjunction: Hom(A × B, C) ≅ Hom(A, B → C)
"A function from A×B to C is the same as a function from A to (B → C)."

This is *currying* in programming — named after Haskell Curry, who observed the correspondence. In logic: to prove A ∧ B → C, it suffices to prove A → (B → C). The proof rules for conjunction and implication are adjoint.

## Adjunctions and Logic

In the internal logic of a topos or the type-theoretic logic:

- ∃ ⊣ pullback (reindexing) ⊣ ∀

Formally: substituting a variable along a function f : X → Y:
- Existential quantification ∃_f is left adjoint to substitution f*
- Universal quantification ∀_f is right adjoint to substitution f*

This gives a categorical explanation of *why* ∃ and ∀ are dual — they are adjoint to the same operation, from opposite sides.
