# Applications: STLC and System F

## 1. Functional Programming Languages: ML, OCaml, Haskell

The Hindley-Milner type inference algorithm — the practical implementation of rank-1 System F for programming — is the type system of the ML family of languages and Haskell. Every functional programmer interacts with System F daily through these languages.

In OCaml, the polymorphic function `List.map : ('a -> 'b) -> 'a list -> 'b list` is a System F term with type $\forall \alpha.\, \forall \beta.\, (\alpha \to \beta) \to \mathsf{List}\, \alpha \to \mathsf{List}\, \beta$. The type checker verifies that the function is applied correctly — it enforces the precondition (a function from `'a` to `'b`) and guarantees the postcondition (a list of `'b` values). The Curry-Howard reading: the type is the proposition "if there is a function from $\alpha$ to $\beta$ and a list of $\alpha$ values, then there is a list of $\beta$ values" — and the implementation is the proof.

Free theorems have direct practical consequences. Any Haskell function of type `forall a. [a] -> [a]` can only rearrange or drop elements — this is a theorem from the type alone, with no inspection of the implementation. This means: when reviewing a polymorphic library function in Haskell, you know its behavior is constrained to a small class of possibilities just from its type signature. This reduces the cognitive burden of code review and enables more confident refactoring.

The ML module system extends System F$\omega$ with module types (signatures) and module expressions (structures). A module type is a kind; a structure is a term at that kind. The functors of ML (functions from structures to structures) are type operators: functions from module-types to module-types. This is the F$\omega$ layer of the lambda cube applied to software engineering. SML's module system and OCaml's module system are among the most powerful module systems in practical programming languages, enabling generic programming with guaranteed correctness.

## 2. Generic Programming in Haskell: Type Classes and Higher-Kinded Types

Haskell's type class system extends System F with *constraints* — type-level predicates that restrict polymorphism. A function `sort :: Ord a => [a] -> [a]` is polymorphic in `a` but constrained to types with an ordering relation. Under Curry-Howard, the constraint `Ord a` is a proposition about `a`, and the instance dictionary (the runtime implementation of the ordering) is the proof of that proposition.

Higher-kinded type classes — `Functor`, `Applicative`, `Monad`, `Traversable` — are constraints on type operators rather than types. `Functor :: (* -> *) -> Constraint` says: "`f` is a type operator with a coherent mapping operation." This is F$\omega$ in action: the kind `* -> *` is a higher kind, and `Functor` is a constraint at that higher kind.

The *Generic Deriving* mechanism in GHC (Glasgow Haskell Compiler) allows automatic derivation of generic operations. A type's "generic representation" is computed by a type-level function (a type family, which is an F$\omega$ type operator) that decomposes the type into products and sums of fields. Generic operations (serialization, comparison, pretty-printing) are defined once for the generic representation and automatically lifted to specific types. This is generic programming via the lambda cube: type operators compute representations, and type-level functions translate between representations.

The efficiency of Haskell's type class resolution — the mechanism by which constraint dictionaries are passed implicitly — is a direct application of the parametricity argument. The compiler, knowing that all implementations of `Functor` must satisfy the functor laws (which are free theorems), can optimize the dispatch.

## 3. Compiler Intermediate Representations: System F as an IR

In modern compilers for functional languages, System F is used as an *intermediate representation* (IR) — a language into which the source program is compiled and in which optimizations are performed.

The GHC Core IR (used in the Glasgow Haskell Compiler) is essentially System F with extensions: polymorphism, algebraic data types, explicit coercions for type equality witnesses, and explicit type and kind arguments. Every optimization in GHC's core-to-core pipeline (inlining, fusion, worker-wrapper transformation, etc.) is a transformation of System F terms that preserves typing.

The key property: System F term transformation preserves semantics exactly because the type system is sound. A well-typed rewrite of a System F term produces a semantically equivalent term with the same type. No optimization can "break" a well-typed program; the type checker catches any transformation error.

The Coq extraction mechanism (for extracting certified OCaml programs from Coq proofs) operates on a similar principle: the CIC term (proof) is compiled to OCaml by erasing types and type-class-like structures (which are computational irrelevant under the Curry-Howard correspondence). The resulting OCaml program is equivalent to the original in behavior because the types guide the erasure — and parametricity guarantees that the erased type arguments were never needed computationally.

## 4. Parametricity and Data Abstraction

Reynolds' parametricity theorem is the formal foundation for *data abstraction* in programming languages. A module or abstract data type that hides its representation type uses existential quantification: $\exists \alpha.\, \text{ops}(\alpha)$ — "there exists a type $\alpha$ with these operations." The parametricity theorem ensures that code outside the module cannot inspect $\alpha$ — it is truly abstract.

The connection to program security: parametricity implies *representation independence*. Two implementations of the same abstract type — one using arrays, one using linked lists — are interchangeable at the client interface level. Any client code that uses only the abstract operations produces the same results regardless of which implementation is chosen. This is the formal statement of encapsulation.

Parametricity has been applied to:
- **Correctness of refactoring**: if a module's representation is changed but its interface is preserved, and the implementation is type-correct, then all clients remain correct (by parametricity).
- **Information hiding**: a module type $\exists \alpha.\, \text{ops}(\alpha)$ ensures that clients cannot access the hidden state $\alpha$ directly — parametricity rules this out.
- **Module coupling**: two modules are decoupled if their interaction is mediated by an abstract interface; parametricity makes this precise.

In Rust, the `trait` system provides a form of constrained parametricity. A trait bound `T: Send + Sync` says the type `T` satisfies the concurrency safety properties. Parametricity implies that code generic over `Send + Sync` types cannot circumvent the safety properties — the type system enforces the abstraction, and parametricity makes this guarantee formal.

## 5. Verified Compilers and Semantic Preservation

A compiler is *semantically correct* if the compiled program behaves the same as the source program. Verifying compiler correctness formally requires stating and proving that each compilation pass preserves semantics.

The Standard ML of New Jersey (SML/NJ) compiler performs CPS transformation, closure conversion, and code generation. Each pass has been verified (in research systems) to preserve typing: if the input to a pass is a well-typed System F term, the output is a well-typed term of a related type system, and the operational semantics is preserved.

The Compcert compiler for C (Xavier Leroy, INRIA), formalized in Coq, proves that each compiler pass preserves the observable behavior of C programs. While Compcert's source language (C) is not a typed lambda calculus, the verification relies on similar principles: each pass is a transformation that respects the semantic invariants expressed by types, and the preservation argument uses substitution lemmas analogous to those of STLC.

CertiCoq (Anand et al.) extends this to Coq itself: a verified compiler for Coq programs to C, proving that the CIC terms are correctly compiled to C code. The challenge: CIC is a dependent type theory far more expressive than STLC or System F, so the preservation arguments require substantially more sophisticated type theory. The foundation is the same — STLC progress and preservation, extended to dependent types.

## 6. Proof Normalization in Interactive Theorem Provers

The strong normalization theorem for System F (and its extensions) is the computational backbone of interactive theorem provers. When a user enters a proof term in Coq, Agda, or Lean, the proof checker:
1. Verifies the term is well-typed (type checking).
2. Reduces the term to a normal form when needed (for definitional equality checking).

Step 2 is guaranteed to terminate by strong normalization. If the type theory were not strongly normalizing, the proof checker might loop forever on a valid proof. The user would have no way to know whether the checker is still working or has diverged.

In Coq, definitional equality (whether two terms are equal by reduction) is checked by normalizing both terms and comparing the normal forms. This works only if normalization terminates — which it does, by the strong normalization theorem for the Calculus of Inductive Constructions.

Strong normalization failures would be catastrophic: they would allow "proofs" that never reduce to a value, which would be indistinguishable from proofs of $\bot$ (false). The type checker would accept inconsistent proofs, and the proof assistant would be unsound.

This is why termination checking in proof assistants is so strictly enforced. Coq and Agda require all recursive functions to pass a structural termination check (ensuring recursion is on structurally smaller arguments). Lean 4 has a similar check. These checks are not bureaucratic formalities — they are what ensure strong normalization holds, and strong normalization is what ensures the proof assistant is consistent.

## 7. Parametricity and Privacy in Differential Privacy

A surprising application of parametricity: the formal foundations of *differential privacy* — a mathematical framework for statistical disclosure limitation — can be understood through a parametricity lens.

Differential privacy says: a randomized algorithm $M$ satisfies $\varepsilon$-differential privacy if for any two adjacent databases $D$ and $D'$ (differing in one record) and any output set $S$, the probabilities $P(M(D) \in S)$ and $P(M(D') \in S)$ differ by at most a factor of $e^\varepsilon$.

Fuzz (Reed and Pierce) is a programming language for differential privacy where the type system tracks the *sensitivity* of computations — how much the output changes when the input changes. The type system is a linear type system where sensitivities are tracked as coefficients in the types. A function of type $1$-sensitive means the output changes by at most 1 when the input changes by 1; a composition of two $1$-sensitive functions is $2$-sensitive.

The parametricity theorem for this linear type system gives *metric preservation*: if a term has type $k$-sensitive, then for any two inputs at distance $d$, the outputs are at distance $k \cdot d$. This is the type-theoretic formulation of differential privacy's sensitivity analysis, and it is provable as a free theorem from the type signature alone.

This application demonstrates that parametricity is not just a theoretical tool for functional programming but a general principle for reasoning about program behavior in terms of types — including in security and privacy applications where the "type" of a computation tracks its information-theoretic properties.
