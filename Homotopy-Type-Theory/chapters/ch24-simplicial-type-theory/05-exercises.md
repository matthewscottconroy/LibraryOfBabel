# Chapter 24 Exercises: Simplicial Type Theory

---

## Section 1: Two Intervals

**Exercise 1.1.** Explain why the cubical interval $\mathbb{I}$ and the simplicial interval $\mathbf{2}$ model different mathematical structures. Specifically:

1. What does the complement $\sim$ on $\mathbb{I}$ correspond to topologically?
2. Why does $\mathbf{2}$ (the poset $\{0 < 1\}$) not have a complement?
3. Give an example of a path in a type that is "directed" (should be in $\mathsf{hom}$) and one that is "undirected" (should be in $a = b$).

**Exercise 1.2.** For the simplicial interval $\mathbf{2}$:

1. Describe $\mathsf{hom}_\mathbf{2}(0_\mathbf{2}, 1_\mathbf{2})$ — how many elements does it have?
2. Describe $\mathsf{hom}_\mathbf{2}(1_\mathbf{2}, 0_\mathbf{2})$ — is it empty or not?
3. Is $\mathbf{2}$ itself a Segal type? If so, describe its composition.

**Exercise 1.3.** The 2-simplex $\Delta^2 = \{(t_1, t_2) : \mathbf{2} \times \mathbf{2} \mid t_1 \leq t_2\}$. Describe:

1. The three vertices of $\Delta^2$ (the three "elements at corners")
2. The three edges of $\Delta^2$ (including which direction they go)
3. The inner horn $\Lambda^2_1$ (which edges are included?)

**Exercise 1.4.** The extension type $\langle \phi \to f \rangle_{\psi \to A}$ generalizes function types. Show:

1. When $\phi = \emptyset$ (empty sub-shape), the extension type is $\psi \to A$ (no constraint)
2. When $\phi = \psi$ (everything), the extension type has exactly one element (the extension must be $f$)
3. When $\phi = \partial \mathbf{2}$ and $\psi = \mathbf{2}$, the extension type is $\mathsf{hom}_A(a, b)$

---

## Section 2: Segal Types

**Exercise 2.1.** Verify that the poset $\mathbb{N} = (ℕ, \leq)$ is a Segal type:

1. What is $\mathsf{hom}_{\mathbb{N}}(m, n)$?
2. What is the composition of $m \leq n$ and $n \leq p$?
3. What is the identity at $n$?
4. Is the Segal condition satisfied? (Is composition unique?)

**Exercise 2.2.** Show that the type $\mathsf{Bool}$ (as an ∞-groupoid) is a Segal type with $\mathsf{hom}_\mathsf{Bool}(a, b) = (a =_\mathsf{Bool} b)$. Describe:

1. All elements of $\mathsf{hom}_\mathsf{Bool}(\mathsf{true}, \mathsf{true})$
2. All elements of $\mathsf{hom}_\mathsf{Bool}(\mathsf{true}, \mathsf{false})$
3. The composition of two elements of $\mathsf{hom}_\mathsf{Bool}(\mathsf{true}, \mathsf{false})$

**Exercise 2.3.** Define a type $\mathsf{Rel}$ whose objects are sets with a binary relation:

```
Rel = Σ (A : Set) × (A → A → Prop)
```

1. Define $\mathsf{hom}_\mathsf{Rel}((A, R), (B, S))$ to be the type of relation-preserving functions.
2. Is $\mathsf{Rel}$ a Segal type? Verify the Segal condition.
3. What are the isomorphisms in $\mathsf{Rel}$?

**Exercise 2.4.** Explain why the Segal condition is automatically satisfied for any type $X$ (viewed as a Segal type with $\mathsf{hom}_X(a, b) = (a = b)$). What does the "unique composite" correspond to in this case?

**Exercise 2.5.** For a Segal type $A$, define the *opposite Segal type* $A^{op}$ with:
- Same objects as $A$
- $\mathsf{hom}_{A^{op}}(a, b) :\equiv \mathsf{hom}_A(b, a)$ (reversed morphisms)
- Composition reversed

Verify that $A^{op}$ is Segal if $A$ is Segal.

---

## Section 3: Rezk Types

**Exercise 3.1.** For each of the following, determine whether the Segal type is Rezk. Justify your answer.

1. The poset $\mathbb{N} = (ℕ, \leq)$
2. The preorder on $\{a, b\}$ with $a \leq b$ and $b \leq a$ (but $a \neq b$)
3. The type $\mathsf{Bool}$ (as an ∞-groupoid)
4. The universe $\mathsf{Type}$ (assuming univalence)

**Exercise 3.2.** Show that if $A$ is Rezk, then for all $a : A$, the type $\mathsf{Iso}_A(a, a)$ of automorphisms at $a$ is a group (under composition).

*Hint:* Composition gives a group multiplication; the Rezk condition gives that the automorphism group is equivalent to the loop space $\Omega_a A = (a =_A a)$.

**Exercise 3.3.** Describe the Rezk completion of the preorder $\{a, b\}$ with $a \leq b$ and $b \leq a$ (from Exercise 3.1(2)). What type is the result?

**Exercise 3.4.** In ordinary HoTT, every type is automatically Rezk (as a Segal type with hom = paths). Explain why this follows from the fact that every path is invertible (giving a path-inverse), which makes every "morphism" automatically an "isomorphism."

**Exercise 3.5.** State the Rezk condition for the universe $\mathsf{Type}$ and show it's equivalent to univalence. Specifically:
1. What is $\mathsf{Iso}_\mathsf{Type}(A, B)$ (isomorphisms in $\mathsf{Type}$ with hom = functions)?
2. Why does $\mathsf{Iso}_\mathsf{Type}(A, B) = (A \simeq B)$ (type equivalences)?
3. Conclude that the Rezk condition for $\mathsf{Type}$ is $(A = B) \simeq (A \simeq B)$ = univalence.

---

## Section 4: Functors and the Yoneda Lemma

**Exercise 4.1.** Explain why every function $F : A \to B$ between Segal types is automatically a functor. Specifically:

1. Why does $F$ preserve identity morphisms? (Show $F(\mathsf{id}_a) = \mathsf{id}_{F(a)}$ from the definition of $\mathsf{id}_a = \lambda t. a$.)
2. Why does $F$ preserve composition? (Use the uniqueness of Segal fillers.)

**Exercise 4.2.** In the Yoneda lemma, the map $\Psi : C(a) \to \mathsf{hom}_{A \to \mathsf{Type}}(\mathsf{hom}_A(a, -), C)$ is defined by $\Psi(u)_b(f) = C(f)(u)$. 

1. What does "$C(f)$" mean here? ($C : A \to \mathsf{Type}$ is a functor, $f : \mathsf{hom}_A(a, b)$.)
2. Why is $\Psi(u)$ naturally in $b$? (Why is the naturality automatic?)
3. Compute $\Psi(u)_a(\mathsf{id}_a)$ and verify it equals $u$.

**Exercise 4.3.** State and prove the contravariant Yoneda lemma: for a contravariant functor $C : A^{op} \to \mathsf{Type}$ and $a : A$:

$$\mathsf{hom}_{A^{op} \to \mathsf{Type}}(\mathsf{hom}_A(-, a), C) \simeq C(a)$$

**Exercise 4.4.** Define what it means for a functor $F : A \to B$ to be:
1. *Faithful*: injective on hom-sets (injective on $\mathsf{hom}_A(a, a') \to \mathsf{hom}_B(F(a), F(a'))$)
2. *Full*: surjective on hom-sets
3. *Fully faithful*: bijective on hom-sets

Using the Yoneda lemma, prove that the Yoneda embedding $a \mapsto \mathsf{hom}_A(a, -)$ is fully faithful.

**Exercise 4.5.** Define an adjunction between Segal types $A$ and $B$ in STT:
1. Give the definition using the natural equivalence $\mathsf{hom}_B(F(a), b) \simeq \mathsf{hom}_A(a, G(b))$.
2. Give the definition using the unit $\eta : \mathsf{id}_A \Rightarrow G \circ F$ and counit $\epsilon : F \circ G \Rightarrow \mathsf{id}_B$.
3. Prove these two definitions are equivalent (assuming the Segal condition).

---

## Section 5: Research-Level Exercises

**Exercise 5.1 (Directed Univalence).** Formulate a precise conjecture for "directed univalence": a Segal type $\mathsf{Type}^{cat}$ of ∞-categories satisfying the Rezk condition with isomorphisms = equivalences of ∞-categories. What would need to be true for this to hold?

**Exercise 5.2 (Riehl-Shulman).** Read the paper "A Synthetic Theory of ∞-Categories in Homotopy Type Theory" (Riehl-Shulman, 2017). 

1. Find their Theorem 5.5 (the Yoneda lemma). How does their proof compare to the sketch in this chapter?
2. Find their definition of "covariant fibration" (Definition 2.1). How does it relate to the notion used in this chapter?
3. What is the role of the Segal condition in their proof of Yoneda?

**Exercise 5.3 (Rzk).** Install the Rzk proof assistant and formalize:

1. The definition of a Segal type (`isSegal`)
2. The identity morphism `id`
3. Composition of two composable morphisms (using the Segal condition to produce a composite)
4. State the Yoneda lemma

*Reference:* The Rzk library at github.com/rzk-lang/rzk has examples to guide you.

**Exercise 5.4 (Limits and colimits).** In a Segal type $A$, define:
1. A *terminal object*: an object $t : A$ such that $\mathsf{hom}_A(a, t)$ is contractible for all $a$.
2. A *product* of $a, b : A$: an object $a \times b$ with morphisms to $a$ and $b$ satisfying the universal property.
3. Prove that a terminal object, if it exists, is unique up to isomorphism.
4. When is the Segal type $\mathsf{Type}$ (of types and functions) "bicomplete" (has all limits and colimits)?
