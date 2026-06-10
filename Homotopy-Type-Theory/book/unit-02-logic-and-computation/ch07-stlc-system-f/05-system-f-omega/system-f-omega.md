# System F$\omega$ and the Lambda Cube

## Types Operating on Types

System F extends STLC with quantification over types: a term can take a type as an argument. But System F does not allow types to take types as arguments — there are no *type-level functions*.

Consider: in Haskell, `Maybe` is a type constructor that takes a type and returns a type. `Maybe Int` is the type of optional integers; `Maybe Bool` is the type of optional booleans. This is a type-level function: `Maybe : Type → Type`. System F cannot express this directly — its types are built from type variables, function types, and universal quantification, but there is no mechanism for type-level abstraction.

System F$\omega$ adds exactly this: *type operators*, which are functions from types to types (or from types-of-types to types-of-types, in full generality). The mechanism is a type-level lambda calculus, with *kinds* playing the role that types play for terms.

## Kinds

**Kinds** classify types, just as types classify terms.

The grammar of kinds:
$$\kappa ::= \star \mid \kappa_1 \to \kappa_2$$

- **$\star$** (pronounced "star"): the kind of *proper types* — types that classify terms. $\mathbb{N} : \star$, $\mathbb{B} : \star$, $\mathbb{N} \to \mathbb{N} : \star$, $\forall \alpha.\, \alpha \to \alpha : \star$.
- **$\kappa_1 \to \kappa_2$**: the kind of *type operators* — functions from types of kind $\kappa_1$ to types of kind $\kappa_2$.

Examples:
- $\mathsf{Maybe} : \star \to \star$ (takes a type, returns a type)
- $\mathsf{Either} : \star \to \star \to \star$ (takes two types, returns a type)
- $\mathsf{Monad} : (\star \to \star) \to \star$ (takes a type operator, returns a type... this is higher-kinded)

In Haskell, `Maybe :: * -> *`, `Either :: * -> * -> *`, `Monad :: (* -> *) -> Constraint`. The kind system of Haskell is (a restricted version of) the kind system of F$\omega$.

## Type-Level Lambda Calculus

In F$\omega$, types can contain type-level lambda abstractions:

$$A, B, C ::= \alpha \mid A \to B \mid \forall \alpha : \kappa.\, A \mid \lambda \alpha : \kappa.\, A \mid A\, B$$

- **$\lambda \alpha : \kappa.\, A$**: type-level abstraction — a type operator that takes a type $\alpha$ of kind $\kappa$ and returns the type $A$.
- **$A\, B$**: type-level application — apply a type operator $A$ to type $B$.

Kind-checking rules:

$$\frac{\Gamma \vdash A : \kappa_1 \to \kappa_2 \quad \Gamma \vdash B : \kappa_1}{\Gamma \vdash A\, B : \kappa_2}$$

$$\frac{\Gamma, \alpha : \kappa_1 \vdash A : \kappa_2}{\Gamma \vdash \lambda \alpha : \kappa_1.\, A : \kappa_1 \to \kappa_2}$$

Type-level beta reduction:
$$(\lambda \alpha : \kappa.\, A)\, B \to_\beta A[\alpha := B]$$

**Example**: define $\mathsf{Pair} = \lambda \alpha : \star.\, \lambda \beta : \star.\, \forall \gamma.\, (\alpha \to \beta \to \gamma) \to \gamma : \star \to \star \to \star$. Then $\mathsf{Pair}\, \mathbb{N}\, \mathbb{B}$ is the Church-encoded type of pairs of a natural number and a boolean.

## The Lambda Cube

System F$\omega$ is one of eight type systems organized by Henk Barendregt into the *lambda cube*. The cube has three axes, corresponding to three kinds of dependency:

1. **Terms depending on terms** (ordinary functions): all systems on the cube have this.
2. **Types depending on types** (type operators): the $\lambda\omega$ axis.
3. **Types depending on terms** (dependent types): the $\lambda P$ axis.
4. **Terms depending on types** (polymorphism): the $\lambda 2$ axis.

The eight systems:

| System | Term→Term | Type→Type | Type→Term | Term→Type |
|---|---|---|---|---|
| $\lambda\to$ (STLC) | ✓ | | | |
| $\lambda 2$ (System F) | ✓ | | ✓ | |
| $\lambda\omega$ | ✓ | ✓ | | |
| $\lambda P$ (LF) | ✓ | | | ✓ |
| $\lambda 2\omega$ (F$\omega$) | ✓ | ✓ | ✓ | |
| $\lambda P2$ | ✓ | | ✓ | ✓ |
| $\lambda P\omega$ | ✓ | ✓ | | ✓ |
| $\lambda C$ (CoC) | ✓ | ✓ | ✓ | ✓ |

**$\lambda\to$** (STLC): terms depend on terms. Types are fixed.

**$\lambda 2$** (System F): terms also depend on types (polymorphism). Type abstraction and application.

**$\lambda\omega$** (System F$\omega$): types also depend on types (type operators). Type-level lambda calculus.

**$\lambda P$** (LF, Logical Framework): types depend on terms (dependent types). No polymorphism.

**$\lambda C$** (Calculus of Constructions, CoC): all four dependencies. The top of the cube.

## The Calculus of Constructions

The Calculus of Constructions (CoC), introduced by Coquand and Huet in 1988, is the top of the lambda cube. It allows:
- Functions from terms to terms (ordinary computation)
- Functions from types to terms (polymorphism)
- Functions from terms to types (dependent types)
- Functions from types to types (type operators)

In CoC, there is a single *kind* $\square$ (the kind of kinds) and the type $\star$ of proper types, with $\star : \square$. The term language unifies terms and types: there is a single syntactic category, with $\star$ and $\square$ as constants.

The typing rule (the "product" rule) in CoC:

$$\frac{\Gamma \vdash A : s_1 \quad \Gamma, x : A \vdash B : s_2}{\Gamma \vdash \Pi_{x:A} B : s_2}$$

where $s_1, s_2 \in \{\star, \square\}$. The different choices of $s_1$ and $s_2$ give:
- $s_1 = \star, s_2 = \star$: ordinary function types (STLC part).
- $s_1 = \square, s_2 = \star$: polymorphism (System F part).
- $s_1 = \star, s_2 = \square$: type operators (F$\omega$ part).
- $s_1 = \square, s_2 = \square$: higher-kinded types.

When $B$ does not depend on $x$, $\Pi_{x:A} B = A \to B$ — the ordinary function type. CoC thus extends all the previous systems.

## From CoC to MLTT and HoTT

CoC is powerful but lacks *inductive types*. To formalize mathematics, we need $\mathbb{N}$, lists, trees, and other recursive data structures. These are added in the *Calculus of Inductive Constructions* (CIC), the type theory underlying Coq.

Martin-Löf Type Theory (MLTT) takes a different approach: it starts with a predicative universe hierarchy (not impredicative like CoC), adds dependent types from the beginning, and includes inductive definitions as a primitive.

HoTT extends MLTT with:
1. **The univalence axiom**: equivalent types are equal.
2. **Higher inductive types**: types defined by paths and higher paths.

These two additions transform MLTT from a type theory for constructive mathematics into a type theory for homotopy theory. The lambda cube perspective: HoTT is not simply one of the eight systems of the lambda cube, but rather an extension of MLTT (which sits between $\lambda P$ and $\lambda C$ in the cube) with axioms that give types homotopy-theoretic structure.

The path from STLC through the lambda cube to MLTT to HoTT is a path of increasing expressive power and mathematical richness. Each step adds a new dimension of dependency or structure, while maintaining the fundamental Curry-Howard insight that types are propositions and terms are proofs.

## Practical Significance: Haskell and OCaml

System F and F$\omega$ are the theoretical foundations of industrial type systems.

**Haskell** uses a variant of F$\omega$ extended with type classes and data types. The `Functor`, `Monad`, and `Applicative` classes in Haskell's standard library are higher-kinded type constraints — they constrain type operators of kind `* → *`. The `fmap :: Functor f => (a → b) → f a → f b` function is a term of a System F$\omega$ type, universally quantified over the type operator `f`.

**OCaml** uses Hindley-Milner type inference, which is a restricted version of System F restricted to rank-1 polymorphism: $\forall$ quantifiers can only appear at the outermost level of a type. This restriction makes type inference decidable (Hindley-Milner type inference is complete for rank-1 polymorphism) while still providing useful polymorphism.

**Dependent type extensions** in Haskell (GADTs, type families, singleton types) extend beyond F$\omega$ toward dependent types, moving along the $\lambda P$ axis of the lambda cube. Each extension adds expressive power at the cost of more complex type inference (eventually becoming undecidable).

Understanding the lambda cube explains why these tradeoffs exist and what the limits of each type system are. The theoretical framework — from STLC through F$\omega$ to CoC and beyond — is not just an academic exercise. It is the map of the design space of type systems, and every practical type system lives somewhere on that map.
