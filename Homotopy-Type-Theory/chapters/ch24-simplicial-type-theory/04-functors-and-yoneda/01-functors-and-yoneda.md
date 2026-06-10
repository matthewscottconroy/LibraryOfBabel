# 4.1 Functors, Natural Transformations, and the Yoneda Lemma

## Functors are Functions

In classical category theory, a functor $F : \mathcal{C} \to \mathcal{D}$ requires:
- A function on objects: $F_0 : \text{Ob}(\mathcal{C}) \to \text{Ob}(\mathcal{D})$
- A function on morphisms: $F_1 : \text{Hom}_\mathcal{C}(a, b) \to \text{Hom}_\mathcal{D}(F(a), F(b))$
- Proof that $F$ preserves identities: $F(\mathsf{id}_a) = \mathsf{id}_{F(a)}$
- Proof that $F$ preserves composition: $F(g \circ f) = F(g) \circ F(f)$

This is four pieces of data with two laws.

**In simplicial type theory: a functor is just a function.**

**Theorem.** Let $A$ and $B$ be Segal types. Every function $F : A \to B$ is automatically a functor: it preserves identities and composition.

*Proof.*
- **Preservation of identity:** $F(\mathsf{id}_a) = F(\lambda t. a) = \lambda t. F(a) = \mathsf{id}_{F(a)}$. The identity morphism is the constant function, and $F$ applied to a constant function gives a constant function. ✓
- **Preservation of composition:** A composite $g \circ f : \mathsf{hom}_A(a, c)$ is the hypotenuse of the unique filler of the horn $(f, g)$. Applying $F$ to the filler gives a filler for the horn $(F(f), F(g))$, and by uniqueness of the Segal filler, $F(g \circ f) = F(g) \circ F(f)$. ✓

So in STT, **every function between Segal types is a functor**. The functoriality conditions are automatic from the type structure.

This is a synthetic miracle: the coherence conditions that classical category theory must verify are absorbed into the type theory.

## Natural Transformations are Directed Homotopies

In classical category theory, a natural transformation $\alpha : F \Rightarrow G$ (for functors $F, G : \mathcal{C} \to \mathcal{D}$) requires:
- Components: $\alpha_c : \mathsf{Hom}_\mathcal{D}(F(c), G(c))$ for each $c : \mathcal{C}$
- Naturality: for each $f : \mathsf{Hom}_\mathcal{C}(a, b)$, the square $G(f) \circ \alpha_a = \alpha_b \circ F(f)$ commutes.

In STT: a natural transformation is a *directed path* from $F$ to $G$ in the function type $A \to B$:

$$\alpha : \mathsf{hom}_{A \to B}(F, G)$$

This is an element of the hom type in the function type — literally a morphism from $F$ to $G$ in the "category of functors" $A \to B$.

Unfolding the hom type in a function type: $\mathsf{hom}_{A \to B}(F, G)$ is the type of functions $\mathbf{2} \to (A \to B)$ with specified endpoints, which by currying is the type of functions $A \to (\mathbf{2} \to B)$.

For $a : A$, we get $\alpha_a : \mathbf{2} \to B$ with $\alpha_a(0) = F(a)$ and $\alpha_a(1) = G(a)$. So $\alpha_a : \mathsf{hom}_B(F(a), G(a))$ — a morphism in $B$ from $F(a)$ to $G(a)$.

**Naturality is automatic.** The naturality condition says: for $f : \mathsf{hom}_A(a, b)$, the square
$$G(f) \circ \alpha_a = \alpha_b \circ F(f)$$
commutes. This follows from the fact that $\alpha : A \to (\mathbf{2} \to B)$ is a function: it must respect the simplicial structure, and the commutativity of the square is exactly this compatibility condition, which holds by the type theory.

So **every element of $\mathsf{hom}_{A \to B}(F, G)$ is a natural transformation, and naturality is automatic**.

## The Category of Functors

For Segal types $A$ and $B$, the function type $A \to B$ is also Segal:

**Theorem.** If $A$ and $B$ are Segal, then $A \to B$ is Segal.

Objects of $A \to B$: functors $F : A \to B$ (= functions)
Morphisms in $A \to B$: natural transformations $\alpha : F \Rightarrow G$
Composition: vertical composition of natural transformations

The Segal condition for $A \to B$ follows from the Segal condition for $B$ applied pointwise.

If $B$ is Rezk, then $A \to B$ is also Rezk. Two functors are equal (as elements of $A \to B$) iff they are naturally isomorphic.

## Representable Functors

For $a : A$ in a Segal type, the *representable functor* is:

$$\mathsf{hom}_A(a, -) : A \to \mathsf{Type}$$
$$b \mapsto \mathsf{hom}_A(a, b)$$

This is a covariant functor (in STT: just a function, but one that sends morphisms to morphisms by precomposition).

The representable functor is the "type of ways to map from $a$ to $-$." It's the synthetic analogue of the representable presheaf in classical category theory.

## The Synthetic Yoneda Lemma

**Theorem (Synthetic Yoneda Lemma, Riehl-Shulman).** Let $A$ be a Segal type, $a : A$, and $C : A \to \mathsf{Type}$ a covariant fibration. Then:

$$\mathsf{hom}_{A \to \mathsf{Type}}(\mathsf{hom}_A(a, -), C) \simeq C(a)$$

In words: natural transformations from the representable functor $\mathsf{hom}_A(a, -)$ to $C$ correspond to elements of $C(a)$.

**The proof.** Define the maps:

**$\Phi$:** Given a natural transformation $\alpha : \mathsf{hom}_{A \to \mathsf{Type}}(\mathsf{hom}_A(a, -), C)$, define:
$$\Phi(\alpha) :\equiv \alpha_a(\mathsf{id}_a) : C(a)$$
"Evaluate $\alpha$ at the identity morphism."

**$\Psi$:** Given $u : C(a)$, define the natural transformation:
$$\Psi(u)_b(f) :\equiv C(f)(u) : C(b)$$
for $f : \mathsf{hom}_A(a, b)$. "Transport $u$ along $f$ using the covariant structure of $C$."

**Checking $\Phi \circ \Psi = \mathsf{id}$:**
$$\Phi(\Psi(u)) = \Psi(u)_a(\mathsf{id}_a) = C(\mathsf{id}_a)(u) = u$$
using that $C(\mathsf{id}_a) = \mathsf{id}_{C(a)}$ (functoriality of $C$). ✓

**Checking $\Psi \circ \Phi = \mathsf{id}$:**
$$\Psi(\Phi(\alpha))_b(f) = C(f)(\alpha_a(\mathsf{id}_a))$$

We need to show this equals $\alpha_b(f)$. By naturality of $\alpha$:
$$\alpha_b(f) = \alpha_b(f \circ \mathsf{id}_a) = C(f)(\alpha_a(\mathsf{id}_a))$$

using the naturality condition $\alpha_b \circ \mathsf{hom}(-, f) = C(f) \circ \alpha_a$ applied to $\mathsf{id}_a$. ✓

So $\Phi$ and $\Psi$ are mutual inverses, giving the equivalence. $\square$

**Note.** The "naturality" used in the proof follows automatically from the fact that $\alpha$ is a natural transformation in STT (a morphism in the function type). No separate naturality axiom is needed.

## Corollaries

**Corollary 1 (Yoneda embedding is fully faithful).** The map $a \mapsto \mathsf{hom}_A(a, -)$ from $A$ to $A \to \mathsf{Type}$ is fully faithful: natural transformations between representable functors correspond to morphisms in $A$.

**Corollary 2 (Yoneda determines elements).** An element $u : C(a)$ is determined by the natural transformation $\Psi(u)$. Two elements $u, v : C(a)$ are equal iff $\Psi(u) = \Psi(v)$ (as natural transformations).

**Corollary 3 (Representable functors are projective).** For any covariant fibration $C$ and any surjection $E \twoheadrightarrow C$, every map from a representable functor to $C$ lifts to $E$.

## Adjunctions Synthetically

In STT, an adjunction between Segal types $A$ and $B$ consists of:
- Functors (= functions) $F : A \to B$ and $G : B \to A$
- A natural equivalence: $\mathsf{hom}_B(F(a), b) \simeq \mathsf{hom}_A(a, G(b))$, natural in $a : A$ and $b : B$

The naturality in $a$ and $b$ is automatic from the type structure.

**The unit and counit.** An adjunction can also be expressed via:
- Unit: $\eta : \mathsf{id}_A \Rightarrow G \circ F$ (a natural transformation from the identity to $G \circ F$)
- Counit: $\epsilon : F \circ G \Rightarrow \mathsf{id}_B$
- Triangle identities: $(\epsilon F) \circ (F \eta) = \mathsf{id}_F$ and $(G \epsilon) \circ (\eta G) = \mathsf{id}_G$

In STT, the triangle identities are propositions (they either hold or don't), and the naturality conditions are automatic.

**Free-forgetful adjunctions.** The paradigm case: $F$ (free) is left adjoint to $G$ (forgetful). For example:
- Free group functor $F : \mathsf{Set} \to \mathsf{Grp}$ is left adjoint to the forgetful functor $G : \mathsf{Grp} \to \mathsf{Set}$

In STT: $\mathsf{hom}_\mathsf{Grp}(F(S), H) \simeq \mathsf{hom}_\mathsf{Set}(S, G(H))$ — group homomorphisms from the free group on $S$ to $H$ correspond to functions from $S$ to the underlying set of $H$.

## The Rzk Proof Assistant

All of the above can be formalized in **Rzk**, a proof assistant implementing simplicial type theory.

```rzk
#lang rzk-1

-- The hom type
#def hom (A : U) (a b : A) : U :=
  (t : 2) → A [ t ≡ 0₂ ↦ a , t ≡ 1₂ ↦ b ]

-- The identity morphism
#def id (A : U) (a : A) : hom A a a :=
  λ t → a

-- The Yoneda map: evaluation at the identity
#def yoneda-φ
  (A : U) (isSegal-A : isSegal A)
  (a : A)
  (C : A → U) (isFib-C : isFibration A C)
  : (hom A a → ... → C a) → C a
  := λ α → α a (id A a)
```

The Rzk syntax is designed to closely match the mathematical notation of the Riehl-Shulman papers, making it easy to translate theorems from the papers directly into Rzk.

## Open Problems

Simplicial type theory is a young field. Major open problems (as of 2025):

**1. Canonicity.** Does STT have canonical forms? Can programs extracted from STT proofs be evaluated? This is open — the simplicial interval $\mathbf{2}$ doesn't obviously have computation rules the way the cubical interval $\mathbb{I}$ does.

**2. Directed univalence.** Is there a good notion of "the Segal type of ∞-categories" satisfying directed univalence? This would complete the analogy with univalence for the universe of types.

**3. Limits and colimits.** While the definitions are clear, formalizing the theory of (co)limits in Rzk is ongoing work.

**4. ∞-Topos theory.** Axiomatizing ∞-toposes in STT — characterizing them by their universal properties — is an ambitious research goal.

**5. Combining with cubical.** Can the cubical interval $\mathbb{I}$ and the simplicial interval $\mathbf{2}$ be used in a single type theory? This would give both homotopy and category theory with computational content.

The unification of synthetic homotopy theory and synthetic ∞-category theory is one of the deepest open problems in foundations of mathematics.
