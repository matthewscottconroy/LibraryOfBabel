# Unit 04: Category Theory

## The Mathematics of Mathematical Structure

In 1945, Samuel Eilenberg and Saunders Mac Lane published a paper on the relationship between homology theories. To state their results, they needed a new language — one that could express when two mathematical constructions are "the same" not just set-theoretically but structurally. The language they invented was category theory. They did not expect it to become one of the most important frameworks in all of mathematics.

Category theory is the study of mathematical structure itself — not numbers, not spaces, not groups, but the patterns that appear in all of these simultaneously. A category is just objects and arrows between them, with a composition law. The simplicity is deceptive. From this sparse beginning, you can define products, coproducts, limits, colimits, adjunctions, monads, toposes, and ultimately the ∞-toposes that give HoTT its semantics.

This unit develops category theory from the ground up, through its application to logic and type theory, and into the higher-categorical structures that undergird HoTT. By the end, you will understand why types are ∞-groupoids, why HoTT is the internal language of ∞-toposes, and why univalence is not an arbitrary axiom but a deep categorical inevitability.

## Why This Unit Belongs Here

You have now seen proof theory, intuitionistic logic, the Curry-Howard correspondence, and the lambda calculus through System F. You know that dependent types extend the Curry-Howard correspondence to predicate logic. You know that Martin-Löf type theory's identity type is more subtle than it first appears.

Category theory provides the semantic framework that explains all of this. The rules of type theory are not arbitrary — they are exactly the rules that hold in every sufficiently structured category. The identity type is not puzzling — it is the internal language of path objects in categorical models. Univalence is not mysterious — it expresses the categorical principle that isomorphic objects are equal, which is true in every ∞-topos.

This is what the unit delivers: the categorical perspective that transforms type theory from a formal game into a deep mathematical theory.

## The Three Chapters

**Chapter 10: Category Theory.** The fundamentals. Categories (objects, morphisms, composition), functors (maps between categories), natural transformations (maps between functors), the Yoneda lemma (objects are determined by their relationships), limits and colimits (universal constructions), adjunctions (the central concept), monads (algebraic effects), and the categorical semantics of type theory (from CCCs for STLC to LCCCs for dependent types).

**Chapter 11: Categorical Logic.** The internal logic of categories. Cartesian closed categories model simply typed lambda calculus — the bijection $\mathsf{Hom}(C \times A, B) \cong \mathsf{Hom}(C, [A,B])$ is currying. Locally cartesian closed categories model dependent type theory — slice categories are contexts, pullbacks are substitution, $\Pi$ and $\Sigma$ types are right and left adjoints to substitution. Toposes model higher-order logic. Grothendieck fibrations provide an alternative, cleaner framework. The simplicial set model (Voevodsky) proves HoTT is consistent.

**Chapter 12: Higher Category Theory.** The structures that HoTT captures. 2-categories and bicategories: morphisms between morphisms. Groupoids: categories where every morphism is invertible — the 1-dimensional version of homotopy types. The homotopy hypothesis (Grothendieck): homotopy types are the same as ∞-groupoids. (∞,1)-categories (quasi-categories): the homotopy-coherent generalization of categories. ∞-groupoids: types in MLTT are ∞-groupoids via their iterated identity types. The crowning theorem: HoTT is the internal language of ∞-toposes.

## The Central Thread

A single theme runs through all three chapters: the relationship between logical/type-theoretic operations and categorical constructions.

| Logic/Type Theory | Category Theory |
|---|---|
| Function type $A \to B$ | Exponential object $[A, B]$ in a CCC |
| Product type $A \times B$ | Categorical product |
| Dependent function $\Pi_{x:A} B(x)$ | Right adjoint to pullback |
| Dependent pair $\Sigma_{x:A} B(x)$ | Left adjoint to pullback |
| Universe type $\mathcal{U}$ | Object classifier in the ∞-topos |
| Identity type $a =_A b$ | Path object; morphisms in the ∞-groupoid |
| Univalence | Equivalences = equalities in ∞-topos |
| Higher inductive type | Homotopy colimit |

This table is not a dictionary of analogies. It is a theorem: every well-typed term in HoTT has an interpretation in every ∞-topos, and the interpretations satisfy the same equations as the term-level equalities in HoTT. The correspondence is precise.

## Connection to HoTT

The connection is deepest in the identity type. A term $p : a =_A b$ is a path from $a$ to $b$ in the "space" $A$. But what does "space" mean formally?

The answer is: a type $A$ in MLTT is an ∞-groupoid, because it has elements (0-cells), identity proofs (1-cells / paths), identity proofs between identity proofs (2-cells / homotopies), and so on — with all cells invertible at every level (because path inversion is always possible). This tower of iterated identity types is exactly the data of an ∞-groupoid.

The homotopy hypothesis tells us: ∞-groupoids are homotopy types. So types in HoTT are homotopy types — not by analogy, but by definition. And HoTT is homotopy theory, internalized in a type theory.

Univalence, in this light, says: when two types are equivalent as ∞-groupoids, they are equal as objects in the universe. The universe is not just a bag of types — it is a type itself, and its identity type consists of equivalences. Univalence is the statement that the universe, as a type, correctly captures the homotopy structure of types.

This is the cathedral that category theory builds. Every stone in this unit — every limit, every adjunction, every fibration — is load-bearing.
