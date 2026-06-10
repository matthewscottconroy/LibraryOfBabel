# Locally Cartesian Closed Categories and Dependent Types

## The Step from STLC to Dependent Types

In STLC, the type of a term does not depend on the values of other terms. The function type $A \to B$ is the type of functions from $A$ to $B$, full stop. The type $B$ does not vary with the input.

In dependent type theory, types can depend on terms. The type $B(x)$ for $x : A$ is a "type family" indexed by $A$: different values of $x$ give potentially different types $B(x)$. This type-level dependency changes the categorical structure needed.

A CCC is insufficient: it models $\Pi$ types only when the codomain type is constant (non-dependent functions $A \to B$). For fully dependent $\Pi_{x:A} B(x)$, the right categorical structure is a *locally cartesian closed category* (LCCC).

## Locally Cartesian Closed Categories

**Definition.** A category $\mathcal{C}$ is *locally cartesian closed* (LCCC) if for every morphism $f : B \to A$ in $\mathcal{C}$, the slice category $\mathcal{C}/A$ is cartesian closed.

Alternatively: $\mathcal{C}$ is an LCCC if every slice category $\mathcal{C}/A$ is a CCC.

Recall: the *slice category* $\mathcal{C}/A$ has as objects the morphisms $f : B \to A$ in $\mathcal{C}$ (with codomain $A$), and as morphisms the commuting triangles. The terminal object of $\mathcal{C}/A$ is $\mathsf{id}_A : A \to A$.

**Why slices?** A type family $B : A \to \mathcal{U}$ (a dependent type over $A$) is modeled as a morphism $p : \tilde{B} \to A$ in $\mathcal{C}$ — the "total space" $\tilde{B} = \sum_{a:A} B(a)$ mapping to the "base" $A$ via the first projection. This is the object $(\tilde{B}, p) \in \mathcal{C}/A$.

The slice category $\mathcal{C}/A$ is the category of type families over $A$: objects are type families, morphisms are family-indexed functions between them.

## Substitution as Pullback

The fundamental operation in dependent type theory is *substitution*: given a type family $B : A \to \mathcal{U}$ and a term $t : \Gamma \to A$, the substituted type family $B[t] : \Gamma \to \mathcal{U}$ is the family you get by "pulling back" $B$ along $t$.

Categorically: given a morphism $t : \Gamma \to A$ in $\mathcal{C}$ and an object $(p : \tilde{B} \to A)$ in $\mathcal{C}/A$, the pullback of $p$ along $t$ gives an object $(p' : \tilde{B}[t] \to \Gamma)$ in $\mathcal{C}/\Gamma$.

This pullback operation is a *functor* $t^* : \mathcal{C}/A \to \mathcal{C}/\Gamma$ called *substitution* or *reindexing*. The functoriality of $t^*$ corresponds to the substitution lemma in type theory: $(B[s])[t] = B[s \circ t]$.

**The Beck-Chevalley condition.** For the interpretation of quantifiers to be well-behaved under substitution, the substitution functors must commute with $\Sigma$ and $\Pi$ in a specific way. For any pullback square:

$$\begin{array}{ccc} \Delta & \xrightarrow{u} & \Gamma \\ \downarrow_{v} & & \downarrow_{t} \\ A' & \xrightarrow{s} & A \end{array}$$

the Beck-Chevalley condition says: $s^* \circ \Sigma_t \cong \Sigma_v \circ u^*$ (for $\Sigma$) and $s^* \circ \Pi_t \cong \Pi_v \circ u^*$ (for $\Pi$). These conditions ensure that existential and universal quantification commute with substitution — which is what you expect from logic.

## Dependent Sum and Product as Adjoints

In an LCCC, substitution $t^* : \mathcal{C}/A \to \mathcal{C}/\Gamma$ has both a left and right adjoint:

$$\Sigma_t \dashv t^* \dashv \Pi_t$$

**Left adjoint $\Sigma_t$ (dependent sum):** $\Sigma_t(\tilde{B}, p)$ is the object $(\tilde{B}, t \circ p)$ in $\mathcal{C}/\Gamma$ — the same total space, but now mapped to $\Gamma$ via $t \circ p$ instead of $A$ via $p$.

In type theory: $\Sigma_t(B : A \to \mathcal{U}) = \sum_{a:A} B(a)$ viewed as a type over $\Gamma$ via the composition with $t$.

The adjunction $\Sigma_t \dashv t^*$ encodes the universal property of the dependent sum: a map from $\Sigma_{a:A} B(a)$ to a type $C$ over $\Gamma$ is the same as a map from $B$ to $t^*(C)$ over $A$ (i.e., a "fiber-wise" map).

**Right adjoint $\Pi_t$ (dependent product):** $\Pi_t(\tilde{B}, p)$ is the right adjoint. Its construction requires the LCCC structure of $\mathcal{C}/\Gamma$: in the slice category $\mathcal{C}/\Gamma$, the exponential of two objects gives the dependent product.

In type theory: $\Pi_t(B : A \to \mathcal{U}) = \prod_{a:A} B(a)$ viewed as a type over $\Gamma$ (independent of which $a$ you choose, you get a function type).

The adjunction $t^* \dashv \Pi_t$ encodes the universal property of the dependent product: a map from $t^*(C)$ (a type pulled back along $t$) into $B$ over $A$ corresponds to a map from $C$ (a type over $\Gamma$) into $\Pi_t(B)$.

## The Full Dictionary for Dependent Type Theory

| Dependent Type Theory | LCCC |
|---|---|
| Context $\Gamma$ | Object $\llbracket \Gamma \rrbracket$ |
| Type family $\Gamma \vdash B : \mathcal{U}$ | Morphism $p : \tilde{B} \to \llbracket \Gamma \rrbracket$ (object of $\mathcal{C}/\llbracket\Gamma\rrbracket$) |
| Term $\Gamma \vdash t : B$ | Section of $p$: morphism $s : \llbracket \Gamma \rrbracket \to \tilde{B}$ with $p \circ s = \mathsf{id}$ |
| Context extension $\Gamma, x:A$ | Object $(\tilde{A}, p_A)$ in $\mathcal{C}/\llbracket\Gamma\rrbracket$ |
| Substitution $B[t/x]$ | Pullback $t^*(B)$ |
| Dependent sum $\sum_{x:A} B(x)$ | Left adjoint $\Sigma_{p_A}$ applied to $B$ |
| Dependent product $\prod_{x:A} B(x)$ | Right adjoint $\Pi_{p_A}$ applied to $B$ |
| Unit type $\mathbf{1}$ | Terminal object of the slice |
| Empty type $\mathbf{0}$ | Initial object (when it exists) |
| $\beta$-rule for $\Pi$ | Counit equation of adjunction |
| $\eta$-rule for $\Pi$ | Uniqueness clause of adjunction |

## The Frobenius Law

A key property of the $\Sigma \dashv \pi^* \dashv \Pi$ triple is the *Frobenius law* (or Frobenius reciprocity):

$$\Sigma_{t}(B \times t^*(C)) \cong \Sigma_t(B) \times C$$

for any $C$ over $\Gamma$ and $B$ over $A$. In type theory: $\sum_{x:A} (B(x) \times C) \simeq (\sum_{x:A} B(x)) \times C$ when $C$ doesn't depend on $x$.

This is the Frobenius axiom for quantifiers in predicate logic: $\exists x. (P(x) \wedge Q) \Leftrightarrow (\exists x. P(x)) \wedge Q$ when $Q$ doesn't involve $x$. The categorical proof: it follows from the adjunction $\Sigma_t \dashv t^*$ and the fact that $t^*$ preserves products.

## Comprehension and Display Maps

An alternative approach to the categorical semantics of dependent types uses *display maps* (or *comprehension schemes*).

**Definition.** A *display map category* is a category $\mathcal{C}$ with a class $\mathcal{D}$ of morphisms (the *display maps*) satisfying:
1. All isomorphisms are display maps
2. Pullbacks of display maps are display maps (and the pullback exists)
3. Every object has a "generic" section covering it

A type family $B : A \to \mathcal{U}$ is modeled as a display map $p : \tilde{B} \to A$. The pullback of $p$ along $t : \Gamma \to A$ gives the substituted type family $B[t]$.

This approach is used in the *categories with families* (CwF) framework (Dybjer, 1996), which provides a clean categorical presentation of dependent type theory without the size issues that can arise with full LCCCs.

## Soundness and Completeness for Dependent Type Theory

**Soundness:** Every LCCC is a model of dependent type theory. The type rules correspond to the adjunctions; the equations hold by the categorical axioms.

**Completeness:** Every model of dependent type theory gives an LCCC. The *classifying LCCC* of dependent type theory is constructed from the syntax: contexts are objects, type families are morphisms into "the universe object," and terms are sections. This LCCC is the initial model.

The completeness theorem tells us: any equation between types or terms that holds in all LCCCs is provable in dependent type theory. Conversely, any provable equation holds in all LCCCs.

## What LCCCs Cannot Do: The Identity Type

An LCCC models dependent type theory — $\Pi$ and $\Sigma$ types — but not the identity type. To model the identity type $a =_A b$, you need additional structure: a *path object* for each object $A$, modeled as a specific factorization $A \to \mathsf{Path}(A) \to A \times A$ (see Section 5).

The absence of the identity type in the bare LCCC structure shows that the identity type is *genuinely additional structure*, not derivable from products and exponentials. This is why the $J$ eliminator (path induction) is a separate rule in dependent type theory — it cannot be derived from the rules for $\Pi$ and $\Sigma$.

In terms of the lambda cube: LCCCs model the corner with $\Pi$ and $\Sigma$ types (the calculus of constructions without identity types). The identity type lives at a higher level, requiring path objects and the Awodey-Warren framework.
