# Chapter 7: Simply Typed Lambda Calculus and System F

## The Theorem Nobody Proved

John Reynolds discovered parametricity by trying to prove something obvious. He wanted to show that a polymorphic program — a program that works for all types — must be "uniform" in its behavior across types. It cannot inspect the types it works with, since it works for all of them. It must treat all type arguments symmetrically.

This intuition is clear. The proof, it turned out, required new mathematical machinery.

Reynolds noticed that a function of type $\forall \alpha.\, \alpha \to \alpha$ must be the identity function. There is no other option. A function of type $\forall \alpha.\, \mathsf{List}\, \alpha \to \mathsf{List}\, \alpha$ must be a function that rearranges or drops elements — it cannot inspect element values (since it doesn't know what type $\alpha$ is) and cannot create new elements (since it has no source of $\alpha$ values other than the input list). A function of type $\forall \alpha.\, \alpha \to \alpha \to \alpha$ must be either "always return the first argument" or "always return the second argument" — there are only two.

These are theorems about the type signature alone. Reynolds called them *free theorems* — theorems you get for free, without looking at the implementation. The type system has already constrained the possible behaviors so severely that the theorem is essentially proved by the type alone.

To prove free theorems, Reynolds developed *parametricity* — a mathematical framework capturing the uniformity of polymorphic programs via relations. A polymorphic term $t : \forall \alpha. A(\alpha)$, when its type parameter is instantiated at two related types, produces related results. The precise statement requires the notion of a *logical relation* indexed by type relations, and the proof is a theorem about System F — the polymorphic lambda calculus.

This is not just a curiosity. Parametricity is one of the most powerful tools in programming language theory. It gives semantic guarantees — theorems about what programs cannot do — purely from type information. It is the foundation of data abstraction, module systems, and the formal study of polymorphism.

This chapter develops the typed lambda calculi that make parametricity possible.

## The Story of Chapter 7

We begin where we left off: with the untyped lambda calculus, which is powerful and unsafe. We add types — first in the simply typed lambda calculus (STLC), recovering safety and termination at the cost of expressiveness. Then we add polymorphism — in System F, recovering expressiveness while maintaining safety. Then we add type operators — in System F$\omega$, reaching a level of abstraction sufficient for the Calculus of Constructions and the type theories underlying Coq and Lean.

**Section 1: Untyped Lambda Calculus.** The untyped lambda calculus: syntax, reduction, Church numerals, the Y combinator, non-termination. Why untyped lambda calculus cannot serve as a logic.

**Section 2: STLC Type Safety.** The progress and preservation theorems: well-typed programs don't get stuck, and types are preserved by computation. The proof method: the substitution lemma, canonical forms.

**Section 3: Church Encodings.** Representing data types — booleans, natural numbers, pairs, sums — as lambda terms. The power and limitations of Church encodings: they work in STLC but lack the dependent elimination that inductive types provide.

**Section 4: System F and Polymorphism.** Type abstraction and application: $\Lambda \alpha.\, t$ and $t\, [A]$. The polymorphic identity. Church encodings in System F. Reynolds' parametricity theorem and free theorems. Strong normalization for System F.

**Section 5: System F$\omega$ and the Lambda Cube.** Kinds: $\star$ and $k_1 \to k_2$. Type operators: functions from types to types. Fω combines F with type operators. The lambda cube. The Calculus of Constructions at the top.

## Connection to HoTT

System F is not HoTT. But it is an essential stepping stone.

System F's universal quantification over types — $\forall \alpha. A$ — is second-order logic. HoTT's $\Pi_{x:A} B(x)$ is dependent type theory. The difference is crucial: in System F, $\alpha$ ranges over *types*, while in HoTT, $x$ ranges over *terms* (elements of $A$). Types can depend on types in System F; in HoTT, types can depend on *values*, giving a much richer expressive power.

But the conceptual move — from ordinary function types to universally quantified types — is the same move, at different levels of the hierarchy. Understanding System F makes the move to dependent types feel like a natural extension: instead of quantifying over types, we quantify over terms; instead of type abstraction, we use lambda abstraction over arbitrary values.

Reynolds' parametricity theorem also has an analogue in HoTT: the univalence axiom implies parametricity (at least in some formulations). A type-theoretic proof that equivalent types are equal, combined with transport along paths, gives a form of uniformity across equivalent types that generalizes Reynolds' result. The connection is active research.
