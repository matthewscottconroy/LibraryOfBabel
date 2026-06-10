# Functions as Relations

## The Set-Theoretic Definition

In everyday mathematics, a function $f : A \to B$ is described as a "rule" mapping each element of $A$ to exactly one element of $B$. Set theory makes this precise: a function is a special kind of **binary relation**.

A **function** (or **map**) $f : A \to B$ is a relation $f \subseteq A \times B$ satisfying:
1. **Totality**: $\forall a \in A,\; \exists b \in B,\; (a, b) \in f$ — every element of $A$ has at least one image
2. **Uniqueness**: $\forall a \in A,\; \forall b, b' \in B,\; (a,b) \in f \wedge (a,b') \in f \to b = b'$ — every element of $A$ has *at most one* image

The unique $b$ with $(a, b) \in f$ is written $f(a)$.

**Example**: $f : \mathbb{N} \to \mathbb{N}$ defined by $f(n) = n^2$ is the relation $\{(0,0), (1,1), (2,4), (3,9), \ldots\}$.

## Terminology

- **Domain**: $A$ — the set of inputs
- **Codomain**: $B$ — the set of possible outputs
- **Image** (range): $f(A) = \{f(a) \mid a \in A\} \subseteq B$ — the actual outputs
- **Preimage**: $f^{-1}(S) = \{a \in A \mid f(a) \in S\}$ for $S \subseteq B$

Note: the codomain $B$ and image $f(A)$ can differ. $f : \mathbb{N} \to \mathbb{Z}$ defined by $f(n) = n$ has codomain $\mathbb{Z}$ but image $\mathbb{N} \subset \mathbb{Z}$.

## Injections, Surjections, Bijections

**Injective** (one-to-one): $f(a) = f(a') \to a = a'$ — distinct inputs give distinct outputs. No two elements map to the same thing.

**Surjective** (onto): $\forall b \in B,\; \exists a \in A,\; f(a) = b$ — every element of the codomain is achieved. The image equals the codomain.

**Bijective**: both injective and surjective — a perfect pairing between $A$ and $B$.

Bijections are the cardinality-preserving maps: $|A| = |B|$ (for finite sets) iff a bijection $A \to B$ exists.

## Functions in Type Theory

In Lean 4 and Coq, functions are *primitive* — they are the basic abstraction of dependent type theory. The set-theoretic view (functions as sets of pairs) is replaced by the type-theoretic view (functions as lambda expressions):

```lean
-- Functions in Lean 4
def square : ℕ → ℕ := fun n => n ^ 2

-- Injective and surjective
example : Function.Injective (fun n : ℕ => 2 * n) := by
  intro m n h
  linarith

-- Bijective means has left and right inverses
example : Function.Bijective (id : ℕ → ℕ) :=
  ⟨Function.injective_id, Function.surjective_id⟩
```

## Exercises
See [problems/ch06_set_theory/03_functions_exercises.md](../../../problems/ch06_set_theory/03_functions_exercises.md)
