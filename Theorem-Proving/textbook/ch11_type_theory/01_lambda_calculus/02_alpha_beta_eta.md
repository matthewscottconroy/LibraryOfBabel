# Alpha, Beta, and Eta Reduction

Three reduction rules govern the dynamics of lambda calculus. Together they define what it means for two expressions to be "the same computation."

## Alpha Reduction: Renaming Bound Variables

The expression `λx. x` and `λy. y` are *syntactically different* but *semantically identical* — both are the identity function. Alpha reduction (α-reduction) captures this:

> **α**: `λx. M` →_α `λy. M[x := y]` (provided `y` does not appear free in `M`)

The name of a bound variable is an implementation detail. In mathematics we routinely say "let x be arbitrary" and then switch to "let y be arbitrary" without changing meaning. Alpha-equivalence is an equivalence relation on lambda terms; we usually work with equivalence classes.

**Why the proviso?** If `M = λy. x y`, then naively replacing `x` with `y` gives `λy. y y` — the variable `y` was *captured* by the outer binder. We require the new variable to be *fresh*.

```
λx. λy. x y   →_α   λz. λy. z y     (rename outer x to z)
```

In Haskell, GHC's internal representation (Core) uses *de Bruijn indices* — bound variables are represented as numbers indicating how many binders out they refer to — precisely to eliminate alpha-renaming issues entirely.

## Beta Reduction: Function Application

Beta reduction is the engine of computation:

> **β**: `(λx. M) N` →_β `M[x := N]`

Apply a function to an argument by substituting the argument for the parameter throughout the body.

```
(λx. x + 1) 5   →_β   5 + 1   →   6

(λf. λx. f (f x)) (λy. y * 2)
  →_β  λx. (λy. y * 2) ((λy. y * 2) x)
  →_β  λx. (λy. y * 2) (x * 2)
  →_β  λx. (x * 2) * 2
  =    λx. x * 4
```

The substitution `M[x := N]` must be *capture-avoiding*: if `N` contains free variables, they must not become bound by binders in `M`. If necessary, alpha-rename `M` first.

**Normal forms**: A term with no redex (reducible expression) — no subterm of the form `(λx. M) N` — is in *beta normal form*. Not every term has a normal form: the omega combinator `(λx. x x)(λx. x x)` reduces to itself forever.

**Church-Rosser theorem**: If `M →* N₁` and `M →* N₂`, then there exists `P` with `N₁ →* P` and `N₂ →* P`. Reduction is *confluent* — different reduction orders reach the same result (if they terminate). This means beta normal forms are unique when they exist.

## Eta Reduction: Extensionality

> **η**: `λx. M x` →_η `M` (provided `x` does not appear free in `M`)

Eta reduction captures *extensional equality*: if `f` and `g` agree on all inputs, they should be considered equal. The term `λx. f x` is just `f` with extra bureaucracy — it applies `f` to whatever argument it receives, which is exactly what `f` does.

```
λx. (+ 3) x   →_η   (+ 3)
```

Eta reduction is philosophically interesting: it corresponds to the *extensionality principle* for functions. In intensional type theories (like Coq's core CIC), eta-equality may not hold definitionally for all types — it must be added as an axiom or proved as a proposition.

## Reduction Strategies

Given a term with multiple redexes, *which* do we reduce first?

| Strategy | Description | Terminates when? |
|----------|-------------|-----------------|
| **Normal order** | Leftmost, outermost first | Whenever a normal form exists |
| **Applicative order** | Leftmost, innermost first (evaluate arguments first) | May loop on non-terminating args |
| **Call by name** | Like normal order, but no reduction under λ | Lazy semantics |
| **Call by value** | Evaluate argument before applying | Strict/eager semantics |
| **Call by need** | Call by name + memoization | Haskell's evaluation model |

**Normal order** is the most powerful: the standardization theorem guarantees that if any reduction sequence terminates, normal order terminates. This is why Haskell's lazy evaluation can handle infinite lists — it never evaluates an argument until forced.

**Applicative order** is what most strict languages use. It's efficient (no repeated work) but will loop if an argument diverges, even if the function ignores it.

```haskell
-- Haskell (call by need): this works
take 5 [1..]    -- [1,2,3,4,5]

-- Applicative order equivalent would loop trying to evaluate [1..]
```

## The Relation to Computation

These three rules are not merely formal curiosities. They constitute a *universal model of computation*:

- Alpha: variable names don't matter (hygiene)
- Beta: function application (the computation step)
- Eta: functions are determined by their behavior (extensionality)

Church's thesis, in the lambda calculus formulation, says that every computable function is representable as a lambda term, and every computation step corresponds to a sequence of beta reductions. When you run a Haskell or OCaml program, you are — at some level of abstraction — performing beta reductions.

The Curry-Howard correspondence maps this to proof theory: beta reduction corresponds to *proof simplification* (cut elimination), and the three reduction rules correspond to the three structural rules of sequent calculi.
