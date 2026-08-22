# Adjunctions

## The Central Concept

Saunders Mac Lane called adjunctions "the most important concept in category theory." After seeing the definition and its consequences, this claim becomes difficult to dispute. Adjunctions appear in every branch of mathematics. They unify many constructions that look superficially different. They are the categorical formalization of the ubiquitous mathematical pattern: "giving [data of type X] is the same as giving [data of type Y]."

An adjunction between two functors $F : \mathcal{C} \to \mathcal{D}$ and $G : \mathcal{D} \to \mathcal{C}$ captures the idea that $F$ and $G$ are "inverse to each other in a weak, natural sense." They are not actual inverses (which would require $G \circ F = \mathsf{Id}_{\mathcal{C}}$), but they are related by a natural bijection between hom-sets.

## The Hom-Set Definition

**Definition.** An *adjunction* $F \dashv G$ between functors $F : \mathcal{C} \to \mathcal{D}$ (left adjoint) and $G : \mathcal{D} \to \mathcal{C}$ (right adjoint) is a natural bijection:

$$\mathsf{Hom}_{\mathcal{D}}(F(A), B) \cong \mathsf{Hom}_{\mathcal{C}}(A, G(B))$$

for all $A \in \mathcal{C}$ and $B \in \mathcal{D}$, natural in both $A$ and $B$.

Naturality in $A$: for any $h : A' \to A$ in $\mathcal{C}$ and $f : F(A) \to B$ in $\mathcal{D}$, the bijection sends $f \circ F(h)$ to $\hat{f} \circ h$ (where $\hat{f}$ is the transpose of $f$).

Naturality in $B$: for any $k : B \to B'$ in $\mathcal{D}$ and $f : F(A) \to B$, the bijection sends $k \circ f$ to $G(k) \circ \hat{f}$.

The bijection $\mathsf{Hom}(F(A), B) \cong \mathsf{Hom}(A, G(B))$ is sometimes called *currying* (in the product-exponential case), *transposing* (in general), or *the adjunction*.

## The Unit-Counit Definition

An equivalent and often more convenient formulation uses the unit and counit.

**Unit:** A natural transformation $\eta : \mathsf{Id}_{\mathcal{C}} \Rightarrow G \circ F$ with components $\eta_A : A \to G(F(A))$.

**Counit:** A natural transformation $\varepsilon : F \circ G \Rightarrow \mathsf{Id}_{\mathcal{D}}$ with components $\varepsilon_B : F(G(B)) \to B$.

satisfying the *triangular identities*:
$$(\varepsilon_{F(A)}) \circ F(\eta_A) = \mathsf{id}_{F(A)} \quad \text{and} \quad G(\varepsilon_B) \circ \eta_{G(B)} = \mathsf{id}_{G(B)}$$

**How to derive unit/counit from the hom-set bijection:**
- $\eta_A : A \to G(F(A))$ is the image of $\mathsf{id}_{F(A)} : F(A) \to F(A)$ under the bijection $\mathsf{Hom}(F(A), F(A)) \cong \mathsf{Hom}(A, G(F(A)))$.
- $\varepsilon_B : F(G(B)) \to B$ is the preimage of $\mathsf{id}_{G(B)} : G(B) \to G(B)$ under $\mathsf{Hom}(F(G(B)), B) \cong \mathsf{Hom}(G(B), G(B))$.

The unit says: for every object $A$, there is a canonical map from $A$ into $G(F(A))$ — how to embed $A$ into the "round-trip" $G \circ F$. The counit says: for every $B$, there is a canonical projection $F(G(B)) \to B$ — how to get back from the round-trip $F \circ G$ to $B$.

The triangular identities say: going around the triangle $A \to G(F(A)) \to G(F(A))$ (using $\eta$ and $G\varepsilon_{F}$) gives the identity, and similarly for the other triangle. The round-trips are "almost the identity," with the deviation measured by $\eta$ and $\varepsilon$.

## Fundamental Examples

**Free-Forgetful Adjunction.** This is the paradigmatic example. The free group functor $F : \mathbf{Set} \to \mathbf{Grp}$ is left adjoint to the forgetful functor $U : \mathbf{Grp} \to \mathbf{Set}$:

$$\mathsf{Hom}_\mathbf{Grp}(F(S), G) \cong \mathsf{Hom}_\mathbf{Set}(S, U(G))$$

A group homomorphism from the free group $F(S)$ to a group $G$ is the same as a function from the set $S$ to the underlying set of $G$. To define a group homomorphism from $F(S)$, you just need to specify where the generators go — and you can send them anywhere.

Unit: $\eta_S : S \to U(F(S))$ sends each element to the corresponding generator.
Counit: $\varepsilon_G : F(U(G)) \to G$ sends each element-viewed-as-generator to itself.

This pattern — free $\dashv$ forgetful — appears everywhere: free monoid $\dashv$ forgetful, free vector space $\dashv$ forgetful, free $R$-module $\dashv$ forgetful, free topological space (discrete) $\dashv$ forgetful. Whenever there is a "free" construction and a "forget structure" operation, there is an adjunction.

**Product-Exponential (Currying).** In any cartesian closed category (see Section 6), the product functor $(-) \times A : \mathcal{C} \to \mathcal{C}$ is left adjoint to the exponential functor $[A, -] : \mathcal{C} \to \mathcal{C}$:

$$\mathsf{Hom}(B \times A, C) \cong \mathsf{Hom}(B, [A, C])$$

A function $B \times A \to C$ (taking a pair to a result) is the same as a function $B \to [A, C]$ (taking an argument to a function). This is *currying*.

Unit: $\eta_B : B \to [A, B \times A]$, $b \mapsto \lambda a. (b, a)$.
Counit: $\varepsilon_C : [A, C] \times A \to C$, $(\phi, a) \mapsto \phi(a)$ (function application / evaluation).

In type theory: $A \times B \to C \simeq A \to (B \to C)$. The currying equivalence is the unit-counit of the product-exponential adjunction.

**$\Sigma \dashv \Delta \dashv \Pi$ in dependent type theory.** For any type $A$ and context morphism $\pi : \Gamma.A \to \Gamma$ (projection), there is an adjoint triple in the slice categories:

$$\Sigma_\pi \dashv \pi^* \dashv \Pi_\pi$$

- $\pi^* : \mathcal{C}/\Gamma \to \mathcal{C}/(\Gamma.A)$ is substitution (pullback along $\pi$)
- $\Sigma_\pi : \mathcal{C}/(\Gamma.A) \to \mathcal{C}/\Gamma$ is dependent sum ($\exists$)
- $\Pi_\pi : \mathcal{C}/(\Gamma.A) \to \mathcal{C}/\Gamma$ is dependent product ($\forall$)

The adjunctions say: $\Sigma x:A. B(x) \to C$ is the same as $B(a) \to C[a/x]$ for all $a$ (Frobenius reciprocity); and similarly for $\Pi$.

This is the categorical semantics of quantification. The rules for $\Pi$ and $\Sigma$ in type theory are exactly the rules that come from these adjunctions.

**Propositional Truncation.** In HoTT, the propositional truncation $\|-\| : \mathcal{U} \to \mathsf{Prop}$ is left adjoint to the inclusion $\iota : \mathsf{Prop} \hookrightarrow \mathcal{U}$:

$$\mathsf{Hom}_\mathsf{Prop}(\|A\|, P) \cong \mathsf{Hom}_{\mathcal{U}}(A, P)$$

for any proposition $P$. A map from the truncation $\|A\|$ to a proposition $P$ is the same as a map from $A$ to $P$ (a proof of $A \to P$). This is the universal property of propositional truncation: to prove something about $\|A\|$ in a proposition, it suffices to prove it about $A$.

Unit: the map $A \to \|A\|$ (the proof that $A$ implies its truncation).
Counit: the identity on $P$ (since $P$ is already a proposition, $\|P\| \cong P$).

**Loop-Suspension Adjunction.** In the ∞-categorical setting of HoTT, the suspension functor $\Sigma : \mathcal{U}_{n\text{-conn}} \to \mathcal{U}_{(n+1)\text{-conn}}$ is left adjoint to the loop space functor $\Omega : \mathcal{U}_* \to \mathcal{U}_*$. This is the type-theoretic version of the classical loop-suspension adjunction $\pi_{n+1}(\Sigma X) \cong \pi_n(X)$ in algebraic topology.

## The Fundamental Theorem: Adjoints Preserve (Co)Limits

**Theorem.** Right adjoints preserve limits. Left adjoints preserve colimits.

*Proof (right adjoint case).* Let $F \dashv G$ with $G : \mathcal{D} \to \mathcal{C}$. Let $D : \mathcal{J} \to \mathcal{D}$ be a small diagram with limit $(L, (\ell_j)_j)$ in $\mathcal{D}$.

We claim $(G(L), (G(\ell_j))_j)$ is a limit of $G \circ D$ in $\mathcal{C}$.

A cone over $G \circ D$ with vertex $C$ is a family of maps $\phi_j : C \to G(D(j))$. By the adjunction $\mathsf{Hom}(F(C), D(j)) \cong \mathsf{Hom}(C, G(D(j)))$, these correspond to a family of maps $\hat{\phi}_j : F(C) \to D(j)$ — a cone over $D$ with vertex $F(C)$. By the universal property of $L$, there is a unique map $u : F(C) \to L$ with $\ell_j \circ u = \hat{\phi}_j$ for all $j$. By the adjunction again, $u$ corresponds to a unique map $\hat{u} : C \to G(L)$ with $G(\ell_j) \circ \hat{u} = \phi_j$ for all $j$. So $(G(L), (G(\ell_j))_j)$ is a universal cone. $\square$

**Consequences:**
- $\mathbf{Set}$ is complete and cocomplete; every limit and colimit exists.
- The forgetful functor $\mathbf{Grp} \to \mathbf{Set}$ (right adjoint) preserves products and equalizers.
- The free functor $\mathbf{Set} \to \mathbf{Grp}$ (left adjoint) preserves coproducts and coequalizers.
- Propositional truncation (left adjoint) preserves coproducts: $\|A + B\| \simeq \|A\| \vee \|B\|$.
- $\Pi$ types (right adjoint to pullback) preserve limits: functions into a product are products of functions.

## Adjunctions and Universal Properties

Every universal property, in the categorical sense, is an adjunction. The universal property of the product $A \times B$ is the adjunction $\Delta \dashv \times$ (where $\Delta : \mathcal{C} \to \mathcal{C} \times \mathcal{C}$ is the diagonal). The universal property of the free group is the free-forgetful adjunction. The universal property of the function type $[A, B]$ is the product-exponential adjunction.

The recognition criterion: if you have a construction $K$ such that "giving a map out of $K(A)$ to $B$ is the same as giving [simpler data from $A$ to something derived from $B$]," then $K$ is a left adjoint. If "giving a map from $A$ into $K(B)$ is the same as giving [simpler data]," then $K$ is a right adjoint.

This is why adjunctions appear everywhere: they are the mathematical content of universal properties, and universal properties are the right way to define most objects in mathematics.

## Adjoint Functor Theorems

When does a functor have an adjoint? The *adjoint functor theorems* give conditions:

**Freyd's General Adjoint Functor Theorem.** A functor $G : \mathcal{D} \to \mathcal{C}$ has a left adjoint if and only if:
1. $G$ preserves all small limits that exist in $\mathcal{D}$
2. $\mathcal{C}$ satisfies the *solution set condition*: for each $A \in \mathcal{C}$, there is a small set of pairs $(B_i, f_i : A \to G(B_i))$ through which every map $A \to G(D)$ factors.

**Special Adjoint Functor Theorem.** If $\mathcal{D}$ is locally small, complete, well-powered, and has a cogenerating set, then any limit-preserving functor $G : \mathcal{D} \to \mathcal{C}$ has a left adjoint.

These theorems show that in "nice" categories (complete, well-powered), limit preservation implies the existence of a left adjoint. Conversely, by the theorem that right adjoints preserve limits, anything with a left adjoint must preserve limits.

In HoTT, the ∞-topos structure ensures the existence of adjunctions: truncation functors have adjoints, suspension has a loop space adjoint, and the ∞-categorical analogues of the adjoint functor theorem hold.

## Summary: The Adjunction Menagerie

| Adjunction | Left Adjoint $F$ | Right Adjoint $G$ |
|---|---|---|
| Free-Forgetful | Free group $F$ | Forget $U$ |
| Currying | Product $(-) \times A$ | Exponential $[A, -]$ |
| Dependent types | $\Sigma_\pi$ | Substitution $\pi^*$ |
| Dependent types | Substitution $\pi^*$ | $\Pi_\pi$ |
| Truncation | Propositional truncation $\|-\|$ | Inclusion $\mathsf{Prop} \hookrightarrow \mathcal{U}$ |
| $n$-truncation | $\|-\|_n$ | Inclusion of $n$-types |
| Loop-Suspension | Suspension $\Sigma$ | Loop space $\Omega$ |

Each row is a theorem, not just a definition: the functors listed are genuinely adjoint, with an explicit hom-set bijection (or unit-counit pair) satisfying the triangular identities.
