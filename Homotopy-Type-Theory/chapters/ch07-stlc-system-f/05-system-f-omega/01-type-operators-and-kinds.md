# 5.1 System Fω: Type Operators and Kinds

## Beyond Polymorphism: Type Functions

System F allows quantifying over types. But it doesn't allow *functions that take types and return types* — type-level functions. Yet these are ubiquitous in functional programming:

- `List : Type → Type` (lists of any element type)
- `Maybe : Type → Type` (optional values)
- `Either : Type → Type → Type` (sum of two types)
- `State : Type → Type → Type` (stateful computation)

These are *type operators* — functions at the type level. System Fω (pronounced "F omega") adds type-level functions and their types (called *kinds*).

## Kinds: Types of Types

In System Fω, types are classified by *kinds*.

**Kind grammar:**
$$\kappa ::= \star \mid \kappa \to \kappa$$

- $\star$ is the kind of *proper types* — types that classify terms. $\mathbb{N} : \star$, $\mathbb{B} : \star$, $A \to B : \star$ (when $A, B : \star$).
- $\kappa \to \kappa'$ is the kind of *type operators* — functions from types to types. $\mathsf{List} : \star \to \star$ (takes a type, returns a type). $\mathsf{Either} : \star \to \star \to \star$.

**Kinding judgments:** $\Gamma \vdash A : \kappa$ says "in context $\Gamma$, type expression $A$ has kind $\kappa$."

## Type-Level Lambda Calculus

System Fω extends System F with:

**Type-level abstraction:** $\lambda \alpha : \kappa. A$ (a function from types to types)

**Type-level application:** $A\, B$ at the type level (apply a type operator to a type)

**Kinding rules:**

$$\frac{\Gamma, \alpha : \kappa \vdash A : \kappa'}{\Gamma \vdash \lambda \alpha : \kappa. A : \kappa \to \kappa'} \quad (\text{Type Abs})$$

$$\frac{\Gamma \vdash A : \kappa \to \kappa' \quad \Gamma \vdash B : \kappa}{\Gamma \vdash A\, B : \kappa'} \quad (\text{Type App})$$

**Type-level $\beta$-reduction:**
$$(\lambda \alpha : \kappa. A)\, B \equiv_\beta A[\alpha := B]$$

## Examples of Type Operators

**The list type operator:**
$$\mathsf{List} : \star \to \star$$
We can define this as a type-level function (or just declare it as a primitive with kind $\star \to \star$).

$$\mathsf{List}\, \mathbb{N} : \star \quad (\text{lists of naturals})$$
$$\mathsf{List}\, (\mathsf{List}\, \mathbb{N}) : \star \quad (\text{lists of lists of naturals})$$

**The functor type:**
$$\mathsf{Functor} : (\star \to \star) \to \star$$
$$\mathsf{Functor}\, F = \Pi_{A B : \star}. (A \to B) \to F\, A \to F\, B$$

A functor for type operator $F$ is a function that lifts maps $A \to B$ to maps $F\, A \to F\, B$.

**Products with a type operator:**
$$\mathsf{Prod} = \lambda \alpha : \star. \lambda \beta : \star. \alpha \times \beta : \star \to \star \to \star$$

Applying: $\mathsf{Prod}\, \mathbb{N}\, \mathbb{B} = \mathbb{N} \times \mathbb{B}$ (kind $\star$).

## System Fω and Haskell

System Fω is the theoretical foundation of Haskell's type system (specifically, the kind system). In Haskell:

```haskell
-- These are type operators with kind * → *
data Maybe a = Nothing | Just a
data List a = Nil | Cons a (List a)

-- Kind annotations (in GHC):
-- Maybe :: * -> *
-- List :: * -> *
-- Either :: * -> * -> *
```

Haskell's type classes (like `Functor`, `Monad`) are constrained to specific kinds:

```haskell
class Functor (f :: * -> *) where
  fmap :: (a -> b) -> f a -> f b
-- f must have kind * -> *, not just *
```

The kind system ensures that type applications are well-formed: you can't apply `Maybe` to `Int → Bool` if `Maybe` expects a type of kind `*`.

## The Calculus of Constructions

System Fω is one corner of Barendregt's *lambda cube* — a diagram of 8 type systems obtained by independently allowing:
1. Terms depending on terms ($\lambda \to$: STLC)
2. Types depending on types (System Fω: type operators)
3. Terms depending on types (System F: polymorphism)

The three extensions can be combined:
- STLC: terms depend on terms
- System F: + terms depend on types (polymorphism)
- System Fω: + types depend on types (type operators)
- System Fω + System F: all three at the type-level
- **Calculus of Constructions (CoC)**: all four (add terms depending on values = dependent types)

CoC combines Fω (type operators) with full dependent types. It's the foundation of Coq/Rocq.

**The missing corner:** Dependent types add "types depending on terms" — this is what Chapter 8 develops. The combination of all four gives the full *Calculus of Inductive Constructions (CIC)*, which underlies Coq.

## The Λ-Cube and the Path to MLTT

The lambda cube organizes typed systems by their expressive power:

$$\text{STLC} \subset \text{System F} \subset \text{Fω} \subset \text{CoC} \subset \text{CIC}$$

Martin-Löf Type Theory (MLTT) is not on the cube — it takes a different path, focusing on dependent types with explicit inductive definitions and an intensional identity type. MLTT is in many ways more powerful than CoC for proof theory (it includes $\Pi$ and $\Sigma$ types, universes, and inductive types with dependent eliminators) while remaining consistent.

HoTT is built on MLTT plus the Univalence Axiom and Higher Inductive Types. The lambda cube is the "algebraic" route to foundations; MLTT + HoTT is the "intensional" route that takes homotopy theory seriously.

## Summary: The Three Typed Systems

| System | Key Feature | Logic Correspondence | Expressiveness |
|---|---|---|---|
| STLC | Simple types | Propositional IPC | Primitive recursive arithmetic |
| System F | $\forall$ over types | Second-order 2IPC | All provably-total functions of 2nd-order arithmetic |
| System Fω | Type operators | 2nd-order + type constructors | Higher-order arithmetic |
| CoC (STLC + F + Fω + Dep) | Dependent types | Higher-order pred. logic | Full mathematics |

Each step up the hierarchy adds expressiveness (can prove more theorems, define more functions) but maintains consistency (no proof of $\bot$) via strong normalization.

The next chapter (Chapter 8) takes the final step: dependent types, where types can depend on term *values* (not just type variables). This is what allows formal mathematics in Lean 4, Agda, and ultimately HoTT.
