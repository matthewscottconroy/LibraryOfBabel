# Chapter 10: Category Theory

## The Mathematics of Mathematics

Category theory was invented by Samuel Eilenberg and Saunders Mac Lane in 1945, motivated by a specific technical problem in algebraic topology. They needed a way to express the naturality of certain constructions — the double-dual embedding of a vector space into its bidual, for instance, is "natural" in a way that the single-dual embedding is not. To make "natural" precise, they needed a general framework for comparing mathematical structures.

The result was category theory: a language for expressing structural relationships between mathematical objects. A category captures the essence of a mathematical domain by specifying objects (the things) and morphisms (the structure-preserving maps between things), without caring about the internal constitution of the objects.

This abstraction proved extraordinarily fruitful. The same categorical concepts — functors, natural transformations, limits, adjunctions — appear in algebra, topology, logic, and computer science. Category theory is the universal language of structure.

## Why Category Theory for HoTT?

Our curriculum reaches category theory at this point for four reasons:

**Semantics of type theory.** Type theories have categorical models: simply typed lambda calculus corresponds to cartesian closed categories (CCCs), dependent type theory corresponds to locally cartesian closed categories (LCCCs), and HoTT corresponds to $\infty$-toposes. Understanding why MLTT is consistent and what its models look like requires category theory.

**The right notion of equality.** In ordinary mathematics, isomorphism is the right notion of sameness for most structures. Two groups are "the same" if they're isomorphic, not if they're literally identical as sets. Category theory makes this precise: objects are identified up to isomorphism, and this is captured by the notion of equivalence of categories. This connects to HoTT's Univalence Axiom, which says that equality of types is the same as equivalence of types.

**Higher structure.** Higher category theory — $\infty$-categories, $\infty$-groupoids — is the mathematical framework for HoTT. Types in HoTT are $\infty$-groupoids. The homotopy hypothesis says $\infty$-groupoids are the same as homotopy types (spaces). This is a theorem in mathematics and a guiding principle of HoTT.

**Formalization.** Lean 4's Mathlib is organized around categories. To use Mathlib fluently, you need to understand functors, natural transformations, adjunctions, and the Yoneda lemma. The `CategoryTheory` library in Mathlib implements all of this.

## Chapter Roadmap

**Section 1: Categories and Functors.** The basic definitions: categories (objects + morphisms + composition), functors (structure-preserving maps between categories), examples from all over mathematics.

**Section 2: Natural Transformations.** Maps between functors. The naturality square. The category of functors between two categories.

**Section 3: The Yoneda Lemma.** One of the deepest theorems in mathematics. An object is completely determined by its relationships to other objects. Representable functors. The Yoneda embedding is fully faithful.

**Section 4: Limits and Colimits.** Terminal and initial objects, products, coproducts, equalizers, pullbacks. The general notion of limit as a universal cone. Adjunctions preserve limits/colimits.

**Section 5: Adjunctions.** The most important concept in category theory. Free-forgetful adjunctions, product-exponential adjunctions, the unit-counit formulation.

**Section 6: Monads.** Monads as abstractions of algebraic structure (and of computational effects). Every adjunction gives a monad. The connection to monads in functional programming.

**Section 7: Categorical Semantics of Type Theory.** How categories model type theories: CCCs model STLC, LCCCs model dependent type theory, toposes model higher-order logic. The internal language correspondence. The path to $\infty$-toposes and HoTT.

**Section 8: Exercises.**

## The Slogan: Universal Properties

If there's one slogan for category theory, it's *universal properties*. Instead of constructing an object by building it from simpler pieces (the "internal" approach), define it by specifying how everything else relates to it (the "external" or "relational" approach).

The Cartesian product $A \times B$ is defined not as "the set of ordered pairs" but as "the object with projections $A \times B \to A$ and $A \times B \to B$ through which any object with maps to $A$ and $B$ factors uniquely." The real numbers are defined not as Dedekind cuts but as "the complete ordered field, unique up to isomorphism." The direct sum of groups is defined by a universal property.

This is the categorical perspective: objects are defined by their relationships, and isomorphism is the appropriate notion of sameness. Anything true of an object defined by a universal property is true of all isomorphic objects — so you never have to worry about which specific model of the real numbers you're working with, as long as it satisfies the universal property.

HoTT internalizes this perspective: the Univalence Axiom says that an equality between types (in the universe) is the same as an equivalence between types. So a theorem about the natural numbers (as an object in type theory) is automatically a theorem about any equivalent object. The universal property becomes a theorem, not a convention.
