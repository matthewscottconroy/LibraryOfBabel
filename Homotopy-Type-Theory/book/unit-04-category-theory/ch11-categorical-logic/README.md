# Chapter 11: Categorical Logic and the Semantics of Type Theory

## The Hook: Consistency Without a Model

In 1935, Gerhard Gentzen proved the consistency of Peano arithmetic — using transfinite induction up to $\varepsilon_0$. This was a consistency proof, but it required methods stronger than arithmetic itself. The circularity is uncomfortable.

In 1994, Martin Hofmann and Thomas Streicher gave a very different kind of consistency proof for Martin-Löf Type Theory. They constructed a specific *categorical model* — the groupoid model — in which all the type theory rules hold. This model proves consistency syntactically: MLTT is consistent because it has a model, and the model exists in ordinary mathematics (set theory).

The groupoid model also proved something unexpected: in the model, the identity type $a =_A b$ is not a proposition (not simply "true or false") but a *groupoid* — a category where every morphism is invertible. Different proofs of $a = b$ can be distinct objects in this groupoid, and distinct proofs can be genuinely different.

This was the discovery that launched HoTT: identity types carry *more structure* than classical logic assumed. The groupoid model showed that Uniqueness of Identity Proofs (UIP) — the axiom that any two proofs of the same identity are themselves identical — is *not provable* in MLTT. It is independent.

The independence proof is a categorical argument: UIP holds in the *set model* (interpreting types as sets) but fails in the *groupoid model* (interpreting types as groupoids). Since MLTT has models where UIP holds and models where it fails, UIP is independent of MLTT — neither provable nor refutable.

This is categorical logic at work: using categorical models to prove independence results, exactly as forcing does in set theory.

## What This Chapter Covers

**Section 1: CCCs and STLC.** The cartesian closed category framework for the simply typed lambda calculus. Products model product types; exponentials model function types. Currying is the product-exponential adjunction. Soundness and completeness of the categorical semantics.

**Section 2: LCCCs and Dependent Types.** Locally cartesian closed categories model dependent type theory. Slice categories model types in context; pullbacks model substitution; $\Sigma$ types are left adjoints to pullback; $\Pi$ types are right adjoints. The Beck-Chevalley condition for substitution in quantifiers.

**Section 3: Fibered Categories.** An alternative framework using Grothendieck fibrations. A fibration over $\mathcal{B}$ models a type theory over a base category of contexts. Cloven fibrations, split fibrations, comprehension categories, and contextual categories.

**Section 4: Toposes.** Categories behaving like $\mathbf{Set}$. The subobject classifier. Internal intuitionistic higher-order logic. Presheaf toposes and sheaf toposes. The independence of LEM from intuitionistic logic via models in the effective topos.

**Section 5: Identity Types Categorically.** Path object semantics (Awodey-Warren). The groupoid model (Hofmann-Streicher). Voevodsky's simplicial set model (univalence holds). The tower of models: sets, groupoids, 2-groupoids, ∞-groupoids.

## The Central Insight

The rules of type theory are not arbitrary conventions. They are exactly the rules that hold in every category of a given kind. Understanding this correspondence means understanding *why* the rules take the form they do.

Why does $\Pi_{x:A} B(x)$ have the $\beta$ rule $(\lambda x. b)(a) \equiv b[a/x]$? Because in any LCCC, the right adjoint to pullback (which models $\Pi$) satisfies the counit equation $\Pi_\pi(\pi^*(Y)) \cong Y$ — and this is exactly the $\beta$ rule.

Why does the identity type have the $J$ eliminator? Because in any category with path objects and the right factorization system, the identity type's introduction rule (reflexivity: $r : A \to \mathsf{Path}(A)$) satisfies a universal lifting property — and this universal property is exactly the $J$ rule.

The categorical semantics makes the type theory *explanatory*, not merely *formal*. It tells you what the rules *mean*, not just what they *say*.
