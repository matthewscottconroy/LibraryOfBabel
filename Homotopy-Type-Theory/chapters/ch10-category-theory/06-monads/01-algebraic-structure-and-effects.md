# 6.1 Monads

## What a Monad Is

A monad is an endofunctor with extra structure that lets you "compose" its applications in a consistent way. Monads appear in:
- Category theory: as algebraic structures generalizing groups
- Computer science: as abstractions of computational effects (IO, state, error handling, nondeterminism)
- Logic/type theory: as the "modalities" of HoTT (propositional truncation, $n$-truncation)

**Definition.** A *monad* on a category $\mathcal{C}$ is a triple $(T, \eta, \mu)$ where:
- $T : \mathcal{C} \to \mathcal{C}$ is a functor
- $\eta : \mathsf{id}_\mathcal{C} \Rightarrow T$ is a natural transformation (the *unit*)
- $\mu : T^2 \Rightarrow T$ is a natural transformation (the *multiplication*)

satisfying:
- **Left unit:** $\mu \circ T\eta = \mathsf{id}_T$, i.e., $\mu_A \circ T(\eta_A) = \mathsf{id}_{T(A)}$
- **Right unit:** $\mu \circ \eta T = \mathsf{id}_T$, i.e., $\mu_A \circ \eta_{T(A)} = \mathsf{id}_{T(A)}$
- **Associativity:** $\mu \circ T\mu = \mu \circ \mu T$, i.e., $\mu_A \circ T(\mu_A) = \mu_A \circ \mu_{T(A)}$

The unit laws say $\eta$ is a "two-sided unit" for $\mu$. Associativity says $\mu$ is associative.

## Monads from Adjunctions

Every adjunction $F \dashv G$ gives a monad:
- $T = G \circ F$
- $\eta$: the adjunction unit
- $\mu = G(\varepsilon_F)$: applying $G$ to the adjunction counit

**Free-Forgetful example.** The adjunction $F_\mathbf{Grp} \dashv U_\mathbf{Grp}$ gives the monad $T = U \circ F : \mathbf{Set} \to \mathbf{Set}$. On a set $S$, $T(S) = U(F(S))$ is the underlying set of the free group on $S$ — the set of all reduced words over $S \cup S^{-1}$.

The unit $\eta_S : S \to T(S)$ sends $s$ to the one-letter word $s$.
The multiplication $\mu_S : T(T(S)) \to T(S)$ "flattens" a word of words into a word (substituting generators for their word representations and reducing).

This is the "list monad" analog, but for groups.

**Maybe monad.** For $\mathbf{Set}$: $T(A) = A + \{*\}$ (add a new element $*$ representing "failure"). $\eta_A : A \to A + \{*\}$ is the inclusion. $\mu_A : (A + \{*\}) + \{*\} \to A + \{*\}$ flattens nested failures.

This comes from the adjunction between pointed sets and sets: $(-) + \{*\} \dashv$ forgetful.

## Kleisli Categories

Given a monad $(T, \eta, \mu)$ on $\mathcal{C}$, the *Kleisli category* $\mathcal{C}_T$ has:
- Objects: the same as $\mathcal{C}$
- Morphisms from $A$ to $B$: morphisms $A \to T(B)$ in $\mathcal{C}$

Composition in $\mathcal{C}_T$: given $f : A \to T(B)$ and $g : B \to T(C)$, their Kleisli composite is:
$$A \xrightarrow{f} T(B) \xrightarrow{T(g)} T(T(C)) \xrightarrow{\mu_C} T(C)$$

The identity in $\mathcal{C}_T$ at $A$ is $\eta_A : A \to T(A)$.

**The monad laws ensure this is a valid category:** Left unit means $\eta_A$ is a left identity, right unit means it's a right identity, and associativity means composition is associative.

**In functional programming:** Kleisli composition is the `>>=` ("bind") operation. A morphism $f : A \to T(B)$ in the Kleisli category is a *monadic function*: it takes an $A$ and produces a $T(B)$ (a "computation of type $B$").

For the Maybe monad: $f : A \to B + \{*\}$ either returns a $B$ or fails. Kleisli composition sequences two such functions: if the first fails, the whole sequence fails; otherwise, feed the result to the second.

## Monads in Haskell

In Haskell, a monad is a type class:
```haskell
class Monad m where
  return :: a -> m a          -- unit η
  (>>=)  :: m a -> (a -> m b) -> m b  -- Kleisli composition
```

The monad laws (left unit, right unit, associativity) are the same axioms.

**Examples:**
- `Maybe`: computations that might fail
- `[]` (list): nondeterministic computations
- `IO`: computations with I/O effects
- `State s`: computations that read and write a state
- `Reader r`: computations that read from an environment

Monads in Haskell are monads in the categorical sense, on the category of Haskell types and functions.

## Monads in Type Theory and HoTT

In dependent type theory, there are important monads:

**Propositional truncation $\|-\|$.** The operation $A \mapsto \|A\|$ (make $A$ a proposition — a type with at most one element) is a monad:
- Unit: $A \to \|A\|$ (truncate)
- Multiplication: $\|\|A\|\| \to \|A\|$ (truncating an already-truncated type does nothing)

The Kleisli category of this monad has types as objects and "functions that produce elements of propositions" as morphisms. This is the world of classical-style logic: you can use truncation to hide witnesses.

**$n$-truncation $\|-\|_n$.** Generalizes propositional truncation to $h$-level $n$: types with homotopy groups above level $n$ collapsed. Each $n$-truncation is a monad.

**Modalities.** In modal HoTT (Chapter 26), a *modality* is a special kind of monad on types satisfying extra conditions (idempotence: $\bigcirc \bigcirc A \simeq \bigcirc A$). Truncations, shape modality, flat modality, and sharp modality are all examples.

## Eilenberg-Moore Algebras

The Kleisli category is one construction associated to a monad. Another is the *Eilenberg-Moore category* $\mathcal{C}^T$:

- Objects: *$T$-algebras*, pairs $(A, \alpha)$ where $A \in \mathcal{C}$ and $\alpha : T(A) \to A$ is a morphism satisfying $\alpha \circ \eta_A = \mathsf{id}_A$ and $\alpha \circ \mu_A = \alpha \circ T(\alpha)$
- Morphisms: morphisms in $\mathcal{C}$ compatible with the algebra structure

**Example.** For the monad $T = U \circ F_\mathbf{Grp}$ (free group monad on $\mathbf{Set}$), a $T$-algebra is a set $A$ with a map $\alpha : T(A) \to A$ (interpret words over $A$ as elements of $A$) satisfying the algebra axioms. These are exactly groups! The Eilenberg-Moore category of the free group monad is $\mathbf{Grp}$.

This is a general theorem: if a monad arises from a free-forgetful adjunction, its Eilenberg-Moore category is equivalent to the original category of algebras.

## The Monad-Adjunction Correspondence

**Theorem (Kleisli, Eilenberg-Moore).** Every monad arises from an adjunction:
1. The Kleisli adjunction: $\mathcal{C} \to \mathcal{C}_T$ (left adjoint: inclusion, right adjoint: inclusion of free objects)
2. The Eilenberg-Moore adjunction: $\mathcal{C} \to \mathcal{C}^T$ (free functor left adjoint to forgetful functor)

The Kleisli category is the "initial" resolution and the Eilenberg-Moore category is the "terminal" resolution. Any adjunction that gives the same monad $T$ factors through both.

## Monads and Logic

The logical interpretation: a monad on the category of propositions (or the category of types) is a *modal operator*. The unit says: if $P$ holds, then $\bigcirc P$ holds. The multiplication says: $\bigcirc\bigcirc P \to \bigcirc P$.

In modal logic, $\bigcirc$ is the "box" operator $\square$ (necessity): $\square P$ says "it's necessary that $P$." A monad axiomatizes the logic of necessity (the S4 modality, with $\square P \to P$ given by the counit of an adjunction, and $\square P \to \square\square P$ by the unit).

HoTT's modalities (propositional truncation, $n$-truncation) are modal operators in this sense. This connects type theory to modal logic, a theme developed in Chapter 26.

## Summary

| Concept | Definition | Example |
|---|---|---|
| Monad $(T, \eta, \mu)$ | Functor with unit and multiplication | Free group, Maybe, List |
| Monad from adjunction | $T = G \circ F$ | Free-Forgetful |
| Kleisli category | Morphisms $A \to T(B)$ | Haskell `>>=` |
| T-algebra | $(A, \alpha : T(A) \to A)$ | Groups (free group monad) |
| Eilenberg-Moore category | Category of $T$-algebras | Algebraic structures |
| Modality in HoTT | Idempotent monad | Truncations |

Monads are the "elephant in the room" of functional programming (Haskell's monads) and a deep concept in category theory (every algebraic structure arises as a monad). In HoTT, they appear as truncations and modalities, which are the tools for moving between different layers of the h-level hierarchy.
