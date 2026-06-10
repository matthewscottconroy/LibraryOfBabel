# Chapter 11: Categorical Logic and the Semantics of Type Theory

## When Type Theory Meets Category Theory

In Chapter 10, we developed category theory as an abstract language for structural mathematics. In Chapter 9, we developed MLTT as the formal system underlying HoTT. Now we bring them together: categorical logic is the study of how type theories and logical systems correspond to categories.

The central insight is the *internal language* correspondence: every sufficiently structured category carries a logical/type-theoretic language, and every type theory has an associated category of models. This is a precise, bidirectional correspondence — not just an analogy.

Why does this matter?

**It explains why the rules work.** The typing rules of dependent type theory are not arbitrary choices — they're exactly the rules that hold in every locally cartesian closed category. When you learn that Π types satisfy the $\eta$-law, you're learning that LCCC exponentials are determined by their action on objects.

**It provides consistency proofs.** To prove that HoTT + Univalence is consistent, Voevodsky constructed a specific categorical model (Kan simplicial sets) in which all the axioms hold. The model's existence proves consistency.

**It proves independence.** UIP (Uniqueness of Identity Proofs) fails in the groupoid model but holds in the set-theoretic model. This means UIP is independent of MLTT — neither provable nor disprovable.

**It connects to geometry.** The category of sheaves on a topological space $X$ is a topos whose internal logic models "local" reasoning over $X$. Different geometric settings give different toposes, and theorems in HoTT that hold in all $\infty$-toposes are "synthetic" geometric theorems.

## What We'll Cover

**Section 1: CCCs and STLC.** The cartesian closed category framework for the simply typed lambda calculus. Products model Cartesian products of types; exponentials model function types. The bijection $\mathsf{Hom}(C \times A, B) \cong \mathsf{Hom}(C, [A,B])$ is currying.

**Section 2: LCCCs and Dependent Types.** Locally cartesian closed categories model dependent type theory. Slice categories model types in context; pullbacks model substitution; Σ types are products in slices; Π types are exponentials in slices.

**Section 3: Fibered Categories.** An alternative, often cleaner framework using Grothendieck fibrations. Types are fibers; terms are sections; the fundamental fibration models the identity type.

**Section 4: Toposes.** Categories that behave like $\mathbf{Set}$. Every topos has an internal intuitionistic higher-order logic. Presheaf categories are the main examples; sheaves on spaces give geometry-aware logics.

**Section 5: Semantics of the Identity Type.** The path object semantics (Awodey-Warren), the groupoid model (UIP fails), and Voevodsky's simplicial set model (Univalence holds). The foundation of HoTT's consistency.

## The Stack of Models

Here's the hierarchy of categorical structures and the type theories they model:

$$\text{CCC} \longleftrightarrow \text{STLC}$$
$$\text{LCCC} \longleftrightarrow \text{Dependent type theory (no identity)}$$
$$\text{LCCC + path objects} \longleftrightarrow \text{MLTT (with identity)}$$
$$\text{Topos} \longleftrightarrow \text{MLTT + higher-order logic}$$
$$\text{$\infty$-Topos} \longleftrightarrow \text{HoTT (MLTT + Univalence + HITs)}$$

Each step up the hierarchy adds more structure to the category and more axioms to the type theory. The correspondences are theorems, not just analogies.

The capstone — the $\infty$-topos / HoTT correspondence — is one of the deepest results in contemporary mathematics, connecting abstract homotopy theory ($\infty$-categories) with formal type theory (HoTT). We'll develop the pieces needed to understand this connection, with the full $\infty$-categorical story appearing in Chapters 12 and onward.
