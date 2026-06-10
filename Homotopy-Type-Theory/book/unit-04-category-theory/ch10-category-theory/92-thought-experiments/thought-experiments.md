# Thought Experiments: Category Theory

## 1. The Monoid Is a Category with One Object

A monoid $(M, \cdot, e)$ is a one-object category $\mathbf{B}M$. The single object is a placeholder; the mathematical content lives entirely in the morphisms. Composition of morphisms is monoid multiplication. The identity morphism is the monoid identity.

Now: consider a group $G$ as a one-object category. Every morphism is an isomorphism (since every group element has an inverse). A *functor* from $\mathbf{B}G$ to $\mathbf{Set}$ is a set $S$ with a $G$-action — a *representation* of $G$. A natural transformation between two such functors is a $G$-equivariant map. The *category of representations* of $G$ is the functor category $[\mathbf{B}G, \mathbf{Set}]$.

Ask: what does the Yoneda lemma say in this context? The Yoneda embedding sends the single object $*$ to the presheaf $\mathsf{Hom}(-, *) : \mathbf{B}G^{op} \to \mathbf{Set}$, which is $G$ acting on itself by right multiplication — the *regular representation*. The Yoneda lemma then says: every representation contains a copy of the regular representation, and natural transformations from the regular representation to any representation $S$ are exactly the elements of $S$.

This is Cayley's theorem: every group embeds into the symmetric group on its underlying set. Category theory recovers classical algebra as a special case of its general machinery.

Does this suggest that Cayley's theorem is not just about groups, but about any algebraic structure that can be viewed as a one-object category with certain properties? What would the "Cayley theorem" for monoids say? For a general small category?

## 2. The Naturality Condition Is Doing Real Mathematical Work

The naturality condition in the definition of a natural transformation looks like a bureaucratic requirement: the square commutes. But it is not bureaucratic. It is doing real mathematical work.

Consider the dual vector space functor $V \mapsto V^*$ (linear functions $V \to k$). This is a contravariant functor on $\mathbf{Vect}_k$. Is there a natural isomorphism $V \cong V^*$ — one that doesn't depend on any choices?

For finite-dimensional $V$: there is an isomorphism $V \cong V^*$, but it requires choosing a basis. The naturality square would say: for any linear map $f : V \to W$, the isomorphism intertwines $f$ and $f^*$. But $f : V \to W$ and $f^* : W^* \to V^*$ go in *opposite directions* — so for the square to commute would require a map that is simultaneously covariant and contravariant. No natural isomorphism $\mathsf{Id} \cong (-)^*$ exists (even for finite-dimensional spaces).

For the double dual $V \mapsto V^{**}$: the map $\eta_V : V \to V^{**}$, $v \mapsto (\phi \mapsto \phi(v))$, *is* natural — it commutes with all linear maps $f$. The naturality square holds precisely because $\eta_V$ is defined without making any choices.

So the naturality condition is not just an aesthetic requirement. It is what distinguishes constructions that are "canonical" (choice-free) from those that merely exist (requiring choices). This is the original question that motivated Eilenberg and Mac Lane.

Now ask: what other mathematical constructions appear "canonical" but actually require choices when you examine them carefully? Can naturality always detect this? And what happens in HoTT, where "canonical" becomes "contractible space of choices" — not just uniqueness, but unique-up-to-unique-path?

## 3. The Yoneda Lemma as a Completeness Theorem

The Yoneda embedding $\mathsf{y} : \mathcal{C} \hookrightarrow [\mathcal{C}^{op}, \mathbf{Set}]$ is fully faithful: it embeds any category into a larger category of presheaves, recovering the original category completely.

The presheaf category $[\mathcal{C}^{op}, \mathbf{Set}]$ is "free" in a sense: it is the free cocompletion of $\mathcal{C}$. Any functor $F : \mathcal{C} \to \mathcal{D}$ where $\mathcal{D}$ is cocomplete extends uniquely to a colimit-preserving functor $\hat{F} : [\mathcal{C}^{op}, \mathbf{Set}] \to \mathcal{D}$.

This is a *completeness theorem* for category theory: any small category can be fully faithfully embedded in a "complete" category (one with all colimits). The internal structure of $\mathcal{C}$ is preserved exactly.

Now think about this from the type-theoretic perspective. The Yoneda embedding for types in HoTT sends each type $A$ to the type family $\lambda B. (B \to A)$ (or dually, $\lambda B. (A \to B)$). The Yoneda lemma then says: a map from $\lambda B. (B \to A)$ to a type family $F$ is the same as an element of $F(A)$.

What is the "free cocompletion" of the category of types? Can we construct it inside HoTT? What would it mean for all HITs to be present in this free cocompletion?

## 4. Adjunctions Are Everywhere — But Are They Fundamental?

Mac Lane said adjunctions are "the most important concept in category theory." But this claim has a philosophical dimension worth examining.

An adjunction says: two mathematical phenomena that look different are actually two sides of the same coin. Currying says giving a function $(A, B) \to C$ is the same as giving a function $A \to (B \to C)$. Free-forgetful says giving a group homomorphism from a free group is the same as giving a set function from the generators. $\Sigma \dashv \Pi$ says dependent sum and dependent product are adjoint to substitution.

But what makes an adjunction *conceptually* deep versus merely *technically* convenient?

Consider: the adjunction between suspension $\Sigma$ and loop space $\Omega$ is not merely a bijection of hom-sets. It reflects a deep symmetry between "adding a loop" and "taking loops." This symmetry is invisible at the level of individual homotopy groups but becomes apparent when you see the adjunction.

The question: when you discover an adjunction, have you discovered a mathematical fact (that two constructions are related), or have you discovered a *definition* (that this is the right way to think about both constructions)? Is the adjunction $\Sigma \dashv \Delta \dashv \Pi$ in type theory a *theorem* about the rules, or does it *explain why* the rules have to take the form they do?

## 5. What the Subobject Classifier Classifies

In a topos $\mathcal{C}$, the subobject classifier $\Omega$ is an object such that subobjects of any object $A$ correspond to morphisms $A \to \Omega$. In $\mathbf{Set}$: $\Omega = \{0, 1\}$, and the map $\chi_S : A \to \{0, 1\}$ classifying a subset $S \subseteq A$ is its characteristic function.

In the topos of sheaves on a topological space $X$: $\Omega =$ the sheaf of open sets. Truth values are not just "true" and "false" — they are the open sets of $X$. A proposition can be "true on this open set but not that one."

Now ask: what is the subobject classifier in the type-theoretic setting?

In the context of HoTT, the "universe" $\mathcal{U}$ (of small types) plays a role analogous to $\Omega$: a subtype of $A$ corresponds to a type family $A \to \mathcal{U}$ (taking each element to the proposition that it is in the subtype). Subtypes correspond to maps $A \to \mathsf{Prop}$ (the universe of propositions).

But $\mathsf{Prop}$ in HoTT is not the same as $\{0, 1\}$ or the sheaf of open sets — it is the type of all *propositions* (h-props). This means: in HoTT, truth values are not just "true/false" but the full universe of propositions, with proof relevance.

Does HoTT have a subobject classifier? The answer is: in the ∞-topos semantics, the universe $\mathcal{U}$ is the *object classifier*, a more general notion than the subobject classifier. What does this mean for the relationship between HoTT and classical set theory?

## 6. Limits vs. Colimits: The Asymmetry

Limits and colimits are dual — you get one from the other by reversing arrows. Yet they behave very differently in practice.

Limits (products, pullbacks, equalizers) tend to be "conservative": they preserve structure. Right adjoints preserve limits. They model conjunction, intersection, specification.

Colimits (coproducts, pushouts, coequalizers) tend to be "liberal": they create new objects by gluing. Left adjoints preserve colimits. They model disjunction, union, identification.

In type theory:
- Limits are the simple type constructors: product types, function types, $\Pi$ types
- Colimits require HITs: pushout types, coequalizer types, suspension

Classical type theory (MLTT without HITs) can do limits but not colimits (beyond $\Sigma$ types). HoTT adds the colimits via HITs.

Here is the question: why is it easy to construct limits in type theory but hard to construct colimits? The answer involves the difference between *forming* a type and *identifying points* in a type. A product type forms new pairs from existing elements; a pushout type *identifies* existing elements by adding path constructors.

Path constructors are the type-theoretic mechanism for colimits. But they require higher-dimensional structure — paths of paths, paths of paths of paths — which is why simple type theory cannot handle them and HoTT, with its full ∞-groupoid structure, can.

## 7. The Equivalence vs. Isomorphism Distinction

In category theory, "the same" means *equivalent*, not *isomorphic* (as categories). Two categories are equivalent if there exist functors $F : \mathcal{C} \to \mathcal{D}$ and $G : \mathcal{D} \to \mathcal{C}$ with $G \circ F \cong \mathsf{Id}$ and $F \circ G \cong \mathsf{Id}$ (natural isomorphism, not equality).

For example: the category of finite-dimensional $\mathbb{R}$-vector spaces is equivalent to the category of $n \times m$ real matrices (for varying $n, m$) under the identification that each vector space $\mathbb{R}^n$ corresponds to $n$. This equivalence is not an isomorphism of categories: the objects are different (abstract vector spaces vs. concrete matrix spaces), but the two categories are indistinguishable by categorical means.

This suggests that "equality of categories" should mean "equivalence," not "literal equality." The principle: isomorphic objects should be equal. And this principle, applied to the universe of all (small) categories, gives: equivalent categories should be equal.

But in classical set theory, this is not provable. The set of objects of $\mathcal{C}$ is not literally the same as the set of objects of an equivalent category $\mathcal{D}$.

HoTT resolves this via univalence: equivalent types are equal (as elements of the universe). This applies to categories too (in the right formulation): in univalent category theory, two Rezk-complete categories that are equivalent are equal as elements of the universe of (small) categories.

The thought experiment: What would mathematics look like if we *always* worked up to equivalence, never up to equality? Which theorems would be easier to state? Which would be harder? And does HoTT's univalence axiom make this "always work up to equivalence" philosophy not just a useful convention but a theorem?
