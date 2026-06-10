# Chapter 10: Category Theory

## The Hook: Eilenberg and Mac Lane's Question

In 1942, Samuel Eilenberg and Saunders Mac Lane were studying homology groups. They noticed something peculiar: the isomorphism between a vector space $V$ and its double dual $V^{**}$ felt *natural* in a way the isomorphism $V \cong V^*$ did not. The double-dual isomorphism was canonical — it didn't depend on any choices. The single-dual isomorphism required you to choose a basis.

They tried to make "natural" precise. The answer required them to invent not one but three new concepts: *categories* (to give a domain for the naturality), *functors* (to express what the dualization was doing), and *natural transformations* (to say what naturality of the isomorphism actually means).

Eilenberg and Mac Lane expected a technical paper about algebraic topology. They got the foundation of a new branch of mathematics.

Category theory is now the universal language of mathematical structure. The same concepts — limits, adjunctions, representable functors, the Yoneda lemma — appear in algebra, topology, logic, geometry, and computer science. The reason is that categories capture something essential: not what mathematical objects *are*, but how they *relate* to one another. And it is relationships, not internal constitution, that carry mathematical meaning.

## The Central Slogan: Morphisms, Not Objects

Classical mathematics focuses on objects: what is a group? a topological space? a vector space? Category theory shifts the focus to morphisms: what maps exist between objects, and how do they compose?

This shift has a philosophical consequence: objects become defined by their relationships rather than their internal structure. The Cartesian product $A \times B$ is not the set of ordered pairs — it is the object with the universal property that maps into it correspond to pairs of maps into $A$ and $B$ separately. The free group on a set $S$ is not the "set of words in $S$" — it is the group with the universal property that group homomorphisms from it correspond to functions from $S$ to the underlying set of any group.

Universal properties define objects uniquely up to isomorphism. And since anything true of an object is true of all isomorphic objects, the specific model doesn't matter — only the universal property does.

HoTT internalizes this philosophy. The Univalence Axiom says: isomorphic types are equal. Equivalent types are identical. You don't have to choose which specific model of a type you're working with — equivalent objects are the same, and the type theory enforces this as a theorem.

## What This Chapter Covers

**Section 1: Categories and Functors.** The basic definitions: categories (objects, morphisms, composition, associativity, units), functors (structure-preserving maps between categories), examples from algebra, topology, logic, and type theory. Isomorphisms in categories. The opposite category and duality.

**Section 2: Natural Transformations and the Yoneda Lemma.** Natural transformations as maps between functors. The naturality square. The Yoneda lemma: every object is completely determined by its hom-functor. The Yoneda embedding is fully faithful. Representable functors.

**Section 3: Limits and Colimits.** Terminal and initial objects, products and coproducts, equalizers and coequalizers, pullbacks and pushouts. The general notion of limit as a universal cone over a diagram. Limits as right adjoints; colimits as left adjoints.

**Section 4: Adjunctions.** The most important concept in category theory. Hom-set definition. Unit-counit definition. The triangular identities. Fundamental examples: free-forgetful, product-exponential (currying), $\Sigma \dashv \Delta \dashv \Pi$ in type theory. The theorem that right adjoints preserve limits and left adjoints preserve colimits.

**Section 5: Monads.** Monads as triple $(T, \eta, \mu)$. Every adjunction gives a monad. The Kleisli and Eilenberg-Moore categories. Monads as algebraic effects (in functional programming). The connection to the type-theoretic modalities in HoTT.

**Section 6: Categorical Semantics of Type Theory.** How categories model type theories. The full hierarchy: $\text{CCC} \leftrightarrow \text{STLC}$, $\text{LCCC} \leftrightarrow \text{dependent type theory}$, $\text{topos} \leftrightarrow \text{MLTT + HOL}$, $\infty\text{-topos} \leftrightarrow \text{HoTT}$.

## The HoTT Connection

The connection to HoTT runs through the entire chapter.

The action-on-paths function $\mathsf{ap}_f : (a = b) \to (f(a) = f(b))$ is the functor action of $f$ on the morphisms of the fundamental ∞-groupoid. Transport along a path $p : a =_A b$ is the functor action of $p$ on a type family — it is substitution along a morphism in the category of contexts.

Adjunctions appear in HoTT directly: $\Sigma \dashv \Delta \dashv \Pi$ is the adjoint triple expressing dependent quantification; the propositional truncation $\|-\|$ is the left adjoint to the inclusion of propositions into types; the $n$-truncation $\|-\|_n$ is the left adjoint to the inclusion of $n$-types. Every time you push a proof through a truncation boundary, you are using an adjunction.

The Yoneda lemma has a direct type-theoretic incarnation: the identity type $a =_A b$ is the "hom-type" of $A$ as an ∞-groupoid, and the Yoneda lemma for ∞-groupoids says that $A$ is equivalent to the type of maps out of the "represented" type. This is why path induction (the $J$ eliminator) works: it expresses the Yoneda lemma for the interval type.
