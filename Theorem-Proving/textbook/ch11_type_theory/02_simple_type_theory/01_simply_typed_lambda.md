# Simply Typed Lambda Calculus

> "Types are a syntactic discipline for enforcing levels of abstraction."
> — John Reynolds, *Types, Abstraction, and Parametric Polymorphism*, 1983

## The Problem with Untyped Lambda Calculus

Untyped lambda calculus is a universal model of computation — any computable function can be expressed in it. But this power comes at a price: **non-termination and self-application are everywhere**.

In untyped lambda calculus:
- $\Omega = (\lambda x.\, x\; x)\; (\lambda x.\, x\; x)$ reduces to itself, looping forever
- $(\lambda x.\, x\; x)$ can be applied to itself: self-application is perfectly well-formed
- The $Y$ combinator $\lambda f.\, (\lambda x.\, f\;(x\;x))\;(\lambda x.\, f\;(x\;x))$ creates recursive functions but also non-termination

These features are necessary for universal computation but catastrophic for formal reasoning. If we want to use lambda calculus as the basis of a proof assistant or programming language with guaranteed termination, we need constraints.

The simply typed lambda calculus (STLC), introduced by Alonzo Church in 1940, imposes just enough structure to rule out infinite loops while retaining significant expressiveness.

## The Types

**Base types**: $\iota$ (individuals), $o$ (propositions), or abstract types $A, B, C, \ldots$

**Function types**: If $\sigma$ and $\tau$ are types, then $\sigma \to \tau$ is the type of functions from $\sigma$ to $\tau$.

Types associate to the right: $\sigma \to \tau \to \rho = \sigma \to (\tau \to \rho)$.

**Type examples**:
- $\iota \to \iota$: a function from individuals to individuals
- $(\iota \to o) \to o$: a functional taking a predicate to a proposition
- $(\iota \to \iota) \to \iota \to \iota$: a higher-order function (like `map` or `fold`)

## Typing Rules

A **typing context** $\Gamma$ is a finite set of variable-type assignments $\{x_1 : \sigma_1, \ldots, x_n : \sigma_n\}$. A **typing judgment** $\Gamma \vdash e : \tau$ means "in context $\Gamma$, expression $e$ has type $\tau$."

**Variable rule**:
$$\frac{x : \sigma \in \Gamma}{\Gamma \vdash x : \sigma}$$

**Abstraction rule** (→I):
$$\frac{\Gamma, x : \sigma \vdash e : \tau}{\Gamma \vdash \lambda x.\, e : \sigma \to \tau}$$

**Application rule** (→E):
$$\frac{\Gamma \vdash f : \sigma \to \tau \qquad \Gamma \vdash a : \sigma}{\Gamma \vdash f\; a : \tau}$$

That is all. Three rules for three syntactic constructs.

## Key Theorem: Strong Normalization

**Theorem**: Every well-typed term in STLC is **strongly normalizing** — every reduction sequence terminates.

This means:
- No infinite loops: $\Omega = (\lambda x.\, x\;x)\;(\lambda x.\, x\;x)$ is **untypable** in STLC
- Self-application $(\lambda x.\, x\;x)$ has no valid type (assigning $x : \sigma$ requires $\sigma = \sigma \to \tau$ — a circular type equation with no solution in STLC)
- The $Y$ combinator is untypable
- Every STLC program halts

**Tradeoff**: STLC is not Turing-complete. It cannot express all computable functions — in particular, it cannot express unbounded recursion. For recursive functions, we need fixed-point operators (which require extending the type system: System T, PCF, or simply adding a general `fix` primitive).

## The Curry-Howard Correspondence for STLC

The types of STLC correspond exactly to propositional logic with only implication (→):

| Type theory | Logic |
|-------------|-------|
| Type $\sigma$ | Proposition $\sigma$ |
| Term $e : \sigma$ | Proof of $\sigma$ |
| Function type $\sigma \to \tau$ | Implication $\sigma \to \tau$ |
| Lambda abstraction $\lambda x.\, e$ | Implication introduction |
| Application $f\; a$ | Modus ponens (→ elimination) |
| Strong normalization | Cut elimination |

Every STLC term is a proof in minimal propositional logic, and every proof in minimal propositional logic is a STLC term. The correspondence is exact.

**Example**: The identity function $\lambda x.\, x : \sigma \to \sigma$ corresponds to the proof $A \vdash A$ (identity axiom).

**Example**: Composition $\lambda f.\, \lambda g.\, \lambda x.\, f\;(g\;x) : (\beta \to \gamma) \to (\alpha \to \beta) \to \alpha \to \gamma$ corresponds to the proof of hypothetical syllogism.

## Hindley-Milner: Polymorphism and Type Inference

STLC requires every term to have a unique monomorphic type. This is limiting: the identity function $\lambda x.\, x$ should work on integers, strings, and anything else — but in pure STLC, it has type $A \to A$ for a fixed $A$ chosen at the start.

**Hindley-Milner type inference** (1969/1978) extends STLC with:
- **Type variables** $\alpha, \beta, \ldots$ ranging over types
- **Universal quantification** (let-polymorphism): $\text{let } f = \lambda x.\, x \text{ in } (f\; 3, f\; \text{"hello"})$ assigns $f$ a polymorphic type $\forall \alpha.\, \alpha \to \alpha$

And crucially: **types can be inferred** — you do not need to write type annotations. The W algorithm (or Algorithm M) infers principal types for all expressions.

This is the type system of **ML, Haskell, OCaml, F#**: highly polymorphic, with complete type inference, maintaining strong guarantees.

```haskell
-- Haskell uses Hindley-Milner
-- identity works on any type
id :: a -> a
id x = x

-- compose works on any compatible function types
(.) :: (b -> c) -> (a -> b) -> (a -> c)
f . g = \x -> f (g x)

-- Type checker infers types without annotations:
triple = map (*3) [1,2,3]  -- :: [Int]
```

## In Lean 4

Lean 4 goes beyond STLC (it uses dependent types), but the STLC typing rules are all present as special cases:

```lean
-- Variable rule: x is in context
def id_fn : α → α := fun x => x  -- λ x. x : α → α

-- Application rule:
def apply_twice : (α → α) → α → α :=
  fun f x => f (f x)  -- f applied to (f applied to x)

-- STLC is embedded: every non-dependent function type is an STLC type
-- Dependent types extend this with Π-types where the codomain can vary
```

## Exercises
See [problems/ch11_type_theory/01_type_inference_exercises.md](../../../problems/ch11_type_theory/01_type_inference_exercises.md)
