# 3.1 The Yoneda Lemma

## The Central Theorem

The Yoneda lemma is one of the most important theorems in mathematics. This is not hyperbole. It appears in every branch of modern mathematics and provides the foundation for categorical reasoning. Its content:

*An object is completely determined by its relationships to all other objects.*

More precisely: to understand $A$, you don't need to know what $A$ "is" internally. You only need to know, for every other object $B$, the set of morphisms $\mathsf{Hom}(B, A)$ — and how these sets relate to each other as $B$ varies.

## Representable Functors

**Definition.** A functor $F : \mathcal{C}^{op} \to \mathbf{Set}$ is *representable* if there exists an object $A \in \mathcal{C}$ and a natural isomorphism:

$$F \cong \mathsf{Hom}(-, A)$$

The object $A$ is called the *representing object* for $F$.

The functor $\mathsf{Hom}(-, A) : \mathcal{C}^{op} \to \mathbf{Set}$ sends each object $B$ to the set $\mathsf{Hom}(B, A)$ and each morphism $f : B' \to B$ to the precomposition function $f^* : \mathsf{Hom}(B, A) \to \mathsf{Hom}(B', A)$, $g \mapsto g \circ f$.

**Examples of representable functors:**
- In $\mathbf{Set}$: $\mathsf{Hom}(\{*\}, -) \cong \mathsf{id}$. Elements of a set $A$ correspond to functions from the one-element set to $A$.
- In $\mathbf{Grp}$: $\mathsf{Hom}(\mathbb{Z}, -) \cong U$ (the forgetful functor). Group homomorphisms $\mathbb{Z} \to G$ correspond to elements of $G$ (where is $1$ sent?).
- In $\mathbf{Top}$: $\mathsf{Hom}(\{*\}, -)$ is the functor sending a space to its underlying set of points.

## The Yoneda Embedding

**Definition.** The *Yoneda embedding* is the functor $\mathbf{y} : \mathcal{C} \to [\mathcal{C}^{op}, \mathbf{Set}]$ defined by:

$$\mathbf{y}(A) = \mathsf{Hom}(-, A)$$

On morphisms: for $f : A \to B$, define $\mathbf{y}(f) = f_* : \mathsf{Hom}(-, A) \Rightarrow \mathsf{Hom}(-, B)$, the natural transformation with components $(f_*)_C : g \mapsto f \circ g$.

**Check that $\mathbf{y}$ is a functor:**
- $\mathbf{y}(\mathsf{id}_A) = (\mathsf{id}_A)_* = \mathsf{id}_{\mathsf{Hom}(-, A)}$ ✓
- $\mathbf{y}(g \circ f) = (g \circ f)_* = g_* \circ f_* = \mathbf{y}(g) \circ \mathbf{y}(f)$ ✓

The Yoneda embedding maps each object $A$ to the functor "maps into $A$."

## The Yoneda Lemma

**Theorem (Yoneda Lemma).** For any locally small category $\mathcal{C}$, functor $F : \mathcal{C}^{op} \to \mathbf{Set}$, and object $A \in \mathcal{C}$:

$$[\mathcal{C}^{op}, \mathbf{Set}](\mathsf{Hom}(-, A),\, F) \cong F(A)$$

naturally in $A$ and $F$. Here $[\mathcal{C}^{op}, \mathbf{Set}](\mathsf{Hom}(-, A), F)$ is the set of natural transformations from $\mathsf{Hom}(-, A)$ to $F$.

In words: natural transformations from the representable functor $\mathsf{Hom}(-, A)$ to $F$ are in bijection with elements of $F(A)$.

## Proof of the Yoneda Lemma

We construct the bijection explicitly.

**Forward direction:** Given $\alpha : \mathsf{Hom}(-, A) \Rightarrow F$ (a natural transformation), define:

$$\Phi(\alpha) = \alpha_A(\mathsf{id}_A) \in F(A)$$

Evaluate the component at $A$ and plug in the identity.

**Backward direction:** Given $x \in F(A)$, define $\Psi(x) : \mathsf{Hom}(-, A) \Rightarrow F$ by:

$$\Psi(x)_B : \mathsf{Hom}(B, A) \to F(B), \quad f \mapsto F(f)(x)$$

For each $f : B \to A$ (a morphism in $\mathsf{Hom}(B, A)$), apply $F(f)$ (which maps $F(A)$ to $F(B)$, since $F$ is contravariant) to $x$ to get $F(f)(x) \in F(B)$.

**Verify $\Psi(x)$ is natural:** For $g : B' \to B$, we need $F(g) \circ \Psi(x)_B = \Psi(x)_{B'} \circ g^*$ (where $g^*(f) = f \circ g$). Check:

Left side: $F(g)(F(f)(x)) = F(f \circ g)(x)$ (since $F$ is a functor: $F(g) \circ F(f) = F(f \circ g)$).

Right side: $\Psi(x)_{B'}(f \circ g) = F(f \circ g)(x)$.

They agree. ✓

**Verify $\Phi \circ \Psi = \mathsf{id}$:** $\Phi(\Psi(x)) = \Psi(x)_A(\mathsf{id}_A) = F(\mathsf{id}_A)(x) = \mathsf{id}_{F(A)}(x) = x$. ✓

**Verify $\Psi \circ \Phi = \mathsf{id}$:** Given $\alpha : \mathsf{Hom}(-, A) \Rightarrow F$, we need to show $\Psi(\Phi(\alpha)) = \alpha$.

$\Psi(\Phi(\alpha))_B(f) = F(f)(\alpha_A(\mathsf{id}_A))$.

By naturality of $\alpha$ applied to $f : B \to A$:

$$\alpha_B \circ f^* = F(f) \circ \alpha_A$$

Applied to $\mathsf{id}_A$: $\alpha_B(f^*(\mathsf{id}_A)) = F(f)(\alpha_A(\mathsf{id}_A))$.

But $f^*(\mathsf{id}_A) = \mathsf{id}_A \circ f = f$.

So $\alpha_B(f) = F(f)(\alpha_A(\mathsf{id}_A)) = \Psi(\Phi(\alpha))_B(f)$. ✓

This completes the proof that $\Phi$ and $\Psi$ are mutual inverses. $\square$

## The Corollary: Yoneda Embedding is Fully Faithful

**Corollary.** The Yoneda embedding $\mathbf{y} : \mathcal{C} \to [\mathcal{C}^{op}, \mathbf{Set}]$ is fully faithful:

$$\mathsf{Hom}_{\mathcal{C}}(A, B) \cong [\mathcal{C}^{op}, \mathbf{Set}](\mathbf{y}(A), \mathbf{y}(B))$$

naturally in $A$ and $B$.

*Proof.* Apply Yoneda with $F = \mathsf{Hom}(-, B)$ and object $A$: natural transformations $\mathsf{Hom}(-, A) \Rightarrow \mathsf{Hom}(-, B)$ are in bijection with $\mathsf{Hom}(-, B)(A) = \mathsf{Hom}(A, B)$. $\square$

**What this means:** Two objects $A$ and $B$ are isomorphic in $\mathcal{C}$ if and only if their representable functors $\mathsf{Hom}(-, A)$ and $\mathsf{Hom}(-, B)$ are naturally isomorphic. The Yoneda embedding faithfully represents $\mathcal{C}$ inside the presheaf category.

## The Philosophical Meaning

The Yoneda Lemma says: *an object is determined by its morphisms into it*.

To know $A$, you don't need to know what $A$ "is" internally. You only need to know, for every $B$, the set $\mathsf{Hom}(B, A)$ — and how this set changes as $B$ changes (functorially).

This is the categorical version of the principle that mathematical objects should be defined by their *universal properties*, not their internal constitution. The natural numbers, for example, are characterized by: any set with a zero-element and a successor function receives a unique map from $\mathbb{N}$ preserving the structure. This universal property completely determines $\mathbb{N}$ up to isomorphism — all models satisfying it are isomorphic.

In type theory, this principle becomes the elimination rule: a type is determined by how you can eliminate it. The recursion principle for $\mathbb{N}$ says: to define a function out of $\mathbb{N}$ into any type $C$, give values at zero and at successors. This is the universal property of $\mathbb{N}$ internalized as a type-theoretic rule.

## Yoneda in Type Theory

In HoTT, the Yoneda Lemma has a direct type-theoretic formulation. For a type $A$ and a type family $B : A \to \mathsf{Type}$:

$$\left(\prod_{x:A} (x = a) \to B(x)\right) \simeq B(a)$$

This says: a natural transformation from the "identity type family" $\lambda x. (x = a)$ to $B$ is equivalent to an element of $B(a)$.

*Proof:* The forward map sends $\alpha : \prod_{x:A} (x = a) \to B(x)$ to $\alpha(a, \mathsf{refl}_a) : B(a)$.

The backward map sends $b : B(a)$ to $\lambda x. \lambda p. \mathsf{transport}^B(p^{-1}, b)$ (transport $b$ along the inverse path).

This is the type-theoretic Yoneda Lemma. It's provable using just J and transport (no axioms needed). It says: to specify a natural transformation from the identity type (at $a$) to a family $B$, you just need to specify the value at the base case $(a, \mathsf{refl}_a)$ — exactly the Yoneda Lemma.

## Cayley's Theorem as Yoneda

The group-theory result that every group embeds in a symmetric group (Cayley's theorem) is a special case of the Yoneda Lemma applied to the one-object category $\mathbf{B}G$ corresponding to a group $G$.

The Yoneda embedding $\mathbf{y} : \mathbf{B}G \to [\mathbf{B}G^{op}, \mathbf{Set}]$ is fully faithful, and the presheaf category $[\mathbf{B}G^{op}, \mathbf{Set}]$ is the category of $G$-sets. A group element $g \in G = \mathsf{Hom}(\star, \star)$ acts on $G$ (viewed as a $G$-set) by left multiplication. This is the Cayley embedding.

## Corollaries and Applications

**Universal elements.** An element $u \in F(A)$ is a *universal element* for $F$ if the corresponding natural transformation $\Psi(u) : \mathsf{Hom}(-, A) \Rightarrow F$ is a natural isomorphism (i.e., $F$ is representable with representing object $A$ and universal element $u$). 

**Density.** Every presheaf $F : \mathcal{C}^{op} \to \mathbf{Set}$ is a colimit of representable presheaves. This is the "density theorem": any functor can be approximated by representable ones.

**Internal hom and enriched categories.** When $\mathcal{C}$ is enriched over a monoidal category $\mathcal{V}$ (hom-objects are in $\mathcal{V}$ rather than $\mathbf{Set}$), the Yoneda Lemma generalizes. The internal hom $[A, B]$ in a CCC is the representing object for $\mathsf{Hom}(-, B)^A$.

## Why This Matters for HoTT

The Yoneda Lemma is central to HoTT's approach to mathematics:

1. **Objects via universal properties.** Every mathematical object in HoTT is defined by its universal property (the elimination rule). The Yoneda Lemma says this determines the object up to unique isomorphism.

2. **Univalence as Yoneda for universes.** The Univalence Axiom says the universe $\mathsf{Type}$ satisfies a form of Yoneda: an equivalence $A \simeq B$ is the same as an equality $A = B$ in the universe. This means $\mathsf{Type}$ is "self-describing" — its identity type is exactly equivalences of types.

3. **HITs via generators and relations.** Higher Inductive Types are defined by generators (constructors) and relations (path constructors). The Yoneda Lemma tells you that these generators and relations completely determine the type up to equivalence.

4. **Synthetic Yoneda.** In HoTT, you can prove the Yoneda Lemma synthetically (using only type-theoretic language, without external set theory). The synthetic Yoneda Lemma is used in Voevodsky's foundations program to establish the correspondence between types and spaces.
