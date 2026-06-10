# Type Inference

One of the most elegant results in programming language theory: types need not be written — they can be *inferred* automatically, with the same precision as if the programmer had annotated everything by hand.

## The Problem

In the Simply Typed Lambda Calculus (STLC), every term must have a unique type. But requiring programmers to write types explicitly is burdensome:

```
λ(f : A → B). λ(x : A). f x
```

Can we deduce that `f : A → B` and `x : A` purely from how `f` and `x` are *used* — without the annotations?

Yes — and the algorithm to do so is beautiful.

## Hindley-Milner Type Inference

The Hindley-Milner (HM) system, developed independently by Roger Hindley (1969) and Robin Milner (1978), solves this for a language with let-polymorphism. The core insight: type inference is *constraint generation + constraint solving*.

**Step 1: Assign type variables**

Replace unknown types with fresh type variables α, β, γ, …:

```
λf. λx. f x
```

Assign: `f : α`, `x : β`. The result of applying `f` to `x` has some type `γ`.

**Step 2: Generate constraints**

From `f x`:
- `f` must be a function type whose argument matches `x`'s type: `α = β → γ`

**Step 3: Unify**

The constraint `α = β → γ` is solved by substituting: `α ↦ β → γ`.

Result: the whole term `λf. λx. f x` has type `(β → γ) → β → γ`.

This is the *most general type* — a polymorphic type scheme with free variables `β` and `γ`.

## Unification

The constraint-solving step uses *unification*, due to Robinson (1965). Given two type expressions, find a substitution that makes them identical:

```
Unify(α → Bool, Int → β):
  α = Int, β = Bool   ✓

Unify(α → α, Int → Bool):
  α = Int, α = Bool   ✗ (contradiction — occurs check passes but types mismatch)

Unify(α, α → β):
  α = α → β   ✗ (occurs check fails — α appears in its own definition)
```

The *occurs check* prevents circular types. Without it, you'd infer `α = α → β` and could write self-applying terms like `λx. x x` — which is how untyped lambda calculus escapes into Turing completeness but also into undecidability.

## Let-Polymorphism

Pure STLC is *monomorphic* — once you infer `id : α → α` in a context, you can't use `id` at both `Int → Int` and `Bool → Bool` in the same expression.

Milner's key insight: generalize at let-bindings.

```
let id = λx. x in
  (id 42, id True)
```

After inferring `id : α → α`, *generalize* to the scheme `∀α. α → α`. At each use site, *instantiate* with a fresh type variable.

This is the basis of Haskell's type system, OCaml's type system, and most ML-family languages. The algorithm (Algorithm W) runs in nearly-linear time on typical programs, though the worst case is exponential (constructed pathological cases).

## Inference vs. Checking

There's a spectrum:

| Mode | Description | Example |
|------|-------------|---------|
| **Full inference** | No annotations needed | OCaml, Haskell |
| **Bidirectional** | Propagate types inward and outward | Rust, Scala |
| **Checking only** | All types annotated | Early Java, C |

**Bidirectional type checking** (Pierce & Turner 2000) is the modern sweet spot: propagate *expected* types downward (checking mode) and *synthesize* types upward (synthesis mode). This handles dependent types and other features where full inference is undecidable.

## Decidability and Limits

HM inference is decidable and complete for simply-typed languages with let-polymorphism. But add features and inference breaks:

- **Rank-2 polymorphism**: decidable but complex (GHC's `RankNTypes`)
- **Rank-n polymorphism** (n ≥ 3): undecidable in general
- **Dependent types**: undecidable in full generality; require annotations at key points
- **Subtyping**: HM + subtyping is undecidable (without restrictions)

This is why Lean, Agda, and Coq require more annotations than Haskell — their type theories are more expressive, and the price is that inference can't do all the work.

## The Wonder of It

Type inference converts a static verification problem — "does this program have a valid type?" — into a kind of *algebraic equation solving*. The types emerge from the structure of the computation itself, without the programmer having to spell them out.

When GHC infers the type of a complex Haskell expression and presents it to you, it is executing a sophisticated deduction from constraints you never explicitly wrote. The type is a *theorem* about your program, proved automatically.
