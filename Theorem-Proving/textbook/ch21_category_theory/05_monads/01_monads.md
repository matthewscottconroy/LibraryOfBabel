# Monads

A monad is a monoid in the category of endofunctors — and in programming, a design pattern for structuring computations with effects.

## The Abstract Definition

A *monad* on a category C consists of:
- A functor T : C → C (the "endofunctor")
- A natural transformation η : id → T (the *unit*, or *return*)
- A natural transformation μ : T ∘ T → T (the *multiplication*, or *join*)

Satisfying monoid-like laws:
- **Left unit**: μ ∘ Tη = id_T
- **Right unit**: μ ∘ ηT = id_T
- **Associativity**: μ ∘ Tμ = μ ∘ μT

## Monads in Programming

In Haskell and functional programming, a monad abstracts computations with effects:

```haskell
class Monad m where
  return :: a -> m a           -- η: wrap a value
  (>>=)  :: m a -> (a -> m b) -> m b  -- bind (Kleisli composition)
```

Laws:
```
return a >>= f       = f a         (left unit)
m >>= return         = m           (right unit)
(m >>= f) >>= g      = m >>= (\x -> f x >>= g)   (associativity)
```

| Monad | Type | Effect |
|-------|------|--------|
| Maybe | Maybe a | Failure/nullable values |
| List | [a] | Nondeterminism |
| State s | s -> (a, s) | Mutable state |
| IO | IO a | Input/output |
| Either e | Either e a | Errors with messages |
| Reader r | r -> a | Read-only environment |

## The Kleisli Category

A monad T on C defines the *Kleisli category* C_T:
- Objects: same as C
- Morphisms A → B in C_T: morphisms A → TB in C
- Identity: η_A : A → TA
- Composition: given f : A → TB and g : B → TC, compose as A → TB → T(TC) → TC (using Tf then μ_C)

The Kleisli category is the category of "effectful computations" — where every morphism carries a potential effect modeled by T.

## Monads and Logic

The *continuation monad* T(A) = (A → R) → R gives a categorical treatment of classical logic: the double-negation translation maps classical proofs to Kleisli morphisms for the continuation monad.

The *probability monad* T(A) = probability distributions over A models probabilistic computation and corresponds to Bayesian reasoning.

Moggi's monadic metalanguage (1989) used monads to give denotational semantics to programming languages with effects — unifying previously ad hoc treatments of state, exceptions, and nondeterminism under a single categorical framework.

Every monad arises from an adjunction (F ⊣ G gives the monad T = G ∘ F). Every monad determines an adjunction (the Kleisli adjunction). This connecting the abstract algebra of monads to the universal property of adjunctions.
