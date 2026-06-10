# 5.1 Adjunctions

## The Most Important Concept in Category Theory

Adjunctions are, in the words of Saunders Mac Lane, "the most important concept in category theory." This is a strong claim, but it's justified: adjunctions appear everywhere in mathematics, and they unify many apparently different constructions.

An adjunction between $F : \mathcal{C} \to \mathcal{D}$ and $G : \mathcal{D} \to \mathcal{C}$ captures the idea that $F$ and $G$ are "inverse to each other in a weak sense" — not actual inverses (which would require $F \circ G = \mathsf{id}$), but inverses up to natural bijection.

## Definition

**Definition.** An *adjunction* $F \dashv G$ between functors $F : \mathcal{C} \to \mathcal{D}$ (left adjoint) and $G : \mathcal{D} \to \mathcal{C}$ (right adjoint) is a natural bijection:

$$\mathsf{Hom}_\mathcal{D}(F(A), B) \cong \mathsf{Hom}_\mathcal{C}(A, G(B))$$

for all $A \in \mathcal{C}$ and $B \in \mathcal{D}$, natural in both $A$ and $B$.

The left side: morphisms from $F(A)$ to $B$ in $\mathcal{D}$.
The right side: morphisms from $A$ to $G(B)$ in $\mathcal{C}$.

These two sets of morphisms are in bijection, and the bijection is natural (compatible with composition in both categories).

## The Unit-Counit Formulation

The adjunction can equivalently be expressed by:

**Unit:** A natural transformation $\eta : \mathsf{id}_\mathcal{C} \Rightarrow G \circ F$ with components $\eta_A : A \to G(F(A))$.

**Counit:** A natural transformation $\varepsilon : F \circ G \Rightarrow \mathsf{id}_\mathcal{D}$ with components $\varepsilon_B : F(G(B)) \to B$.

satisfying the *triangular identities*:
$$(\varepsilon_{F(A)}) \circ F(\eta_A) = \mathsf{id}_{F(A)}$$
$$G(\varepsilon_B) \circ \eta_{G(B)} = \mathsf{id}_{G(B)}$$

**How to get from the bijection to unit/counit:**
- Unit $\eta_A$: the image of $\mathsf{id}_{F(A)}$ under the bijection $\mathsf{Hom}(F(A), F(A)) \cong \mathsf{Hom}(A, G(F(A)))$
- Counit $\varepsilon_B$: the preimage of $\mathsf{id}_{G(B)}$ under the bijection $\mathsf{Hom}(F(G(B)), B) \cong \mathsf{Hom}(G(B), G(B))$

The unit says: for every $A$, there's a canonical map $\eta_A : A \to G(F(A))$ (how to embed $A$ into $G(F(A))$). The counit says: for every $B$, there's a canonical map $\varepsilon_B : F(G(B)) \to B$ (how to project from $F(G(B))$ to $B$).

## Fundamental Examples

**Free-Forgetful Adjunction.** The most paradigmatic example. The free group functor $F : \mathbf{Set} \to \mathbf{Grp}$ is left adjoint to the forgetful functor $U : \mathbf{Grp} \to \mathbf{Set}$:

$$\mathsf{Hom}_\mathbf{Grp}(F(S), G) \cong \mathsf{Hom}_\mathbf{Set}(S, U(G))$$

A group homomorphism from the free group $F(S)$ to a group $G$ is the same as a function from the set $S$ to the underlying set $U(G)$. To define a group homomorphism from $F(S)$, you just need to know where the generators (elements of $S$) go — and you can send them anywhere.

Unit: $\eta_S : S \to U(F(S))$ sends each element to the corresponding generator in the free group.
Counit: $\varepsilon_G : F(U(G)) \to G$ sends each generator (element of $G$ viewed as a set) to itself in $G$.

This pattern — free ⊣ forgetful — appears everywhere: free monoid ⊣ forgetful, free vector space ⊣ forgetful, free $R$-module ⊣ forgetful, free topological space (discrete) ⊣ forgetful.

**Product-Exponential (Currying).** In $\mathbf{Set}$, the product functor $(-) \times A : \mathbf{Set} \to \mathbf{Set}$ is left adjoint to the exponential (function) functor $A^{(-)} = [A, -] : \mathbf{Set} \to \mathbf{Set}$:

$$\mathsf{Hom}(B \times A, C) \cong \mathsf{Hom}(B, C^A) = \mathsf{Hom}(B, [A, C])$$

A function $B \times A \to C$ is the same as a function $B \to [A, C]$ (a function from $B$ to functions from $A$ to $C$). This is currying.

In type theory: $A \times B \to C \simeq A \to (B \to C)$ is the dependent currying equivalence (Section 2.1 of Chapter 8). The product-exponential adjunction in type theory is exactly this.

Unit: $\eta_B : B \to [A, B \times A]$, $b \mapsto \lambda a. (b, a)$.
Counit: $\varepsilon_C : [A, C] \times A \to C$, $(\phi, a) \mapsto \phi(a)$ (evaluation).

**Σ ⊣ Δ ⊣ Π in dependent type theory.** For a type $A$ and the projection $\pi : A \times B \to A$ (or more generally, a substitution), there are three functors between slice categories:

- $\Sigma_\pi : \mathcal{C}/B \to \mathcal{C}/A$ (dependent sum / left adjoint)
- $\pi^* : \mathcal{C}/A \to \mathcal{C}/B$ (substitution / middle)
- $\Pi_\pi : \mathcal{C}/B \to \mathcal{C}/A$ (dependent product / right adjoint)

The adjunctions $\Sigma_\pi \dashv \pi^* \dashv \Pi_\pi$ are the categorical expression of the dependent quantifiers $\exists$ and $\forall$:

$$\mathsf{Hom}(\Sigma_\pi(X), Y) \cong \mathsf{Hom}(X, \pi^*(Y)) \cong \mathsf{Hom}(\pi^*(X), Y \text{ in the next slice})$$

This is the categorical semantics of dependent type theory (Section 7).

**Diagonal ⊣ Product.** The diagonal functor $\Delta : \mathcal{C} \to \mathcal{C} \times \mathcal{C}$, $A \mapsto (A, A)$, is left adjoint to the product functor $\times : \mathcal{C} \times \mathcal{C} \to \mathcal{C}$ (when products exist):

$$\mathsf{Hom}(\Delta(A), (B, C)) = \mathsf{Hom}(A, B) \times \mathsf{Hom}(A, C) \cong \mathsf{Hom}(A, B \times C)$$

This is the universal property of the product: a map from $A$ to $B \times C$ is the same as a pair of maps from $A$ to $B$ and from $A$ to $C$.

## The Fundamental Theorem: Adjoints Preserve Limits/Colimits

**Theorem.** Right adjoints preserve limits. Left adjoints preserve colimits.

*Proof for right adjoints.* Let $G : \mathcal{D} \to \mathcal{C}$ be a right adjoint to $F : \mathcal{C} \to \mathcal{D}$. Let $D : \mathcal{J} \to \mathcal{D}$ be a diagram with limit $\varprojlim D$.

A cone over $G \circ D$ with vertex $C$ consists of maps $C \to G(D(j))$. By the adjunction, these correspond to maps $F(C) \to D(j)$, i.e., a cone over $D$ with vertex $F(C)$. By the universal property of $\varprojlim D$, this cone corresponds uniquely to a map $F(C) \to \varprojlim D$, which by the adjunction corresponds to $C \to G(\varprojlim D)$. This shows $G(\varprojlim D)$ satisfies the universal property of $\varprojlim (G \circ D)$. $\square$

**Practical consequences:**
- The forgetful functor $\mathbf{Grp} \to \mathbf{Set}$ (right adjoint) preserves products: the product of groups has the same underlying set as the product of the underlying sets.
- The free functor $\mathbf{Set} \to \mathbf{Grp}$ (left adjoint) preserves coproducts: $F(S \sqcup T) \cong F(S) * F(T)$ (free product of groups).
- In type theory: $(-) \times A$ (left adjoint to $[A, -]$) distributes over coproducts: $(B + C) \times A \cong (B \times A) + (C \times A)$.

## Adjunctions and Universal Properties

Every universal property is an adjunction. If $X$ has the property that "maps out of $X$ into $Y$ correspond to maps out of something else into something else," then you have an adjunction.

Examples:
- The product $A \times B$: maps from $C$ to $A \times B$ correspond to pairs of maps. This is the adjunction $\Delta \dashv \times$.
- The free group $F(S)$: maps from $F(S)$ to $G$ correspond to functions from $S$ to $U(G)$.
- The polynomial $A[x]$: ring maps from $A[x]$ to $R$ correspond to ring maps from $A$ to $R$ together with an element of $R$ (where to send $x$).

The recognition: if you can express "giving a map out of $X$ is the same as giving [some simpler data]," you have a left adjoint. If "giving a map into $Y$ is the same as giving [some simpler data]," you have a right adjoint.

## Monads from Adjunctions

Every adjunction $F \dashv G$ gives a monad $(T, \eta, \mu)$ on $\mathcal{C}$:
- $T = G \circ F : \mathcal{C} \to \mathcal{C}$ (the monad functor)
- $\eta : \mathsf{id}_\mathcal{C} \Rightarrow T$ (unit: the adjunction unit)
- $\mu : T^2 \Rightarrow T$ (multiplication: $G(\varepsilon_{F(A)}) : G(F(G(F(A)))) \to G(F(A))$)

Every monad arises from some adjunction (Kleisli or Eilenberg-Moore). So monads and adjunctions are closely related.

## Adjunctions in HoTT

In HoTT, adjunctions are ubiquitous:
- Σ ⊣ Δ ⊣ Π are the dependent quantifiers
- Propositional truncation $\|-\|$ is left adjoint to the inclusion of propositions into types
- The $n$-truncation $\|-\|_n$ is left adjoint to the inclusion of $n$-types into types
- Suspension Σ is left adjoint to the loop space $\Omega$ (in the $\infty$-categorical sense)

The last example is particularly deep: it's the type-theoretic version of the loop-suspension adjunction in stable homotopy theory.

## Summary

| Adjunction | Left Adjoint | Right Adjoint | Bijection |
|---|---|---|---|
| Free-Forgetful | Free functor $F$ | Forgetful functor $U$ | Hom(F(S), G) ≅ Hom(S, U(G)) |
| Product-Exponential | $- \times A$ | $[A, -]$ | Hom(B×A, C) ≅ Hom(B, [A,C]) |
| Σ ⊣ Δ | Σ (dependent sum) | Substitution Δ | Type theory |
| Δ ⊣ Π | Substitution Δ | Π (dependent product) | Type theory |
| Truncation | $\|-\|$ | Inclusion Prop ↪ Type | Logic/HoTT |

Adjunctions are the categorical expression of duality, of "the same information from two different perspectives," of universal properties. They appear in every branch of mathematics precisely because they capture a fundamental pattern of mathematical thought: two structures that are "inverse to each other in the right way."
