# Exercises: Higher Category Theory

## Section 1: 2-Categories and Bicategories

**Exercise 1.** In the strict 2-category $\mathbf{Cat}$:

(a) What are the objects, 1-morphisms, and 2-morphisms?
(b) Write out the vertical composition of natural transformations $\alpha : F \Rightarrow G$ and $\beta : G \Rightarrow H$ explicitly.
(c) Write out the horizontal composition of $\alpha : F \Rightarrow G$ (functors $\mathcal{C} \to \mathcal{D}$) and $\beta : H \Rightarrow K$ (functors $\mathcal{D} \to \mathcal{E}$).
(d) Verify the interchange law: $(\beta' \circ_v \beta) \circ_h (\alpha' \circ_v \alpha) = (\beta' \circ_h \alpha') \circ_v (\beta \circ_h \alpha)$.

**Exercise 2.** Define the *span bicategory* $\mathbf{Span}(\mathbf{Set})$:

- Objects: sets
- 1-morphisms from $A$ to $B$: spans $A \leftarrow C \rightarrow B$ (pairs of functions from a common set $C$)
- 2-morphisms: morphisms of spans (commuting triangles)
- Composition: by pullback

(a) Define the identity 1-morphism on a set $A$.
(b) Define the composition of two spans using pullback.
(c) Show that composition is only associative up to isomorphism (not strict). Produce the associator.

**Exercise 3.** Verify the pentagon axiom for the span bicategory: for four composable spans, the two paths (associating left vs. right) give the same result up to a coherent isomorphism.

**Exercise 4.** A *2-functor* $F : \mathcal{C} \to \mathcal{D}$ between strict 2-categories sends objects, 1-morphisms, and 2-morphisms to their counterparts, preserving all composition. Write out the axioms for a 2-functor and give an example: the "underlying 1-category" 2-functor from $\mathbf{Cat}$ to itself (the identity).

## Section 2: Groupoids

**Exercise 5.** Verify that the fundamental groupoid $\Pi_1(X)$ of a topological space $X$ is a groupoid:

(a) Composition of paths (concatenation) is well-defined up to homotopy.
(b) Composition is associative up to homotopy.
(c) The constant path $c_x : x \to x$ is an identity up to homotopy.
(d) Every path has an inverse (the reversed path) up to homotopy.

**Exercise 6.** Let $G$ be a group and $BG$ the one-object groupoid with $\mathsf{Hom}(*, *) = G$.

(a) Describe the functor category $[\mathbf{B}G, \mathbf{Set}]$. What mathematical structures are its objects?
(b) Describe natural transformations in $[\mathbf{B}G, \mathbf{Set}]$. What are these classically?
(c) Use the Yoneda lemma to show: $\mathsf{Nat}(\mathsf{Hom}(*, -), F) \cong F(*)$ for any $F : \mathbf{B}G \to \mathbf{Set}$.

**Exercise 7.** In MLTT, prove that every type $A$ satisfies the groupoid laws propositionally:

(a) Path concatenation is associative: $(p \cdot q) \cdot r =_{a=d} p \cdot (q \cdot r)$ for paths $p : a = b$, $q : b = c$, $r : c = d$.
(b) $\mathsf{refl} \cdot p = p$ and $p \cdot \mathsf{refl} = p$.
(c) $p \cdot p^{-1} = \mathsf{refl}$ and $p^{-1} \cdot p = \mathsf{refl}$.

(Hint: all three follow from the $J$ eliminator by path induction on $p$.)

**Exercise 8.** Show that UIP (Uniqueness of Identity Proofs) is equivalent to the statement that every type in MLTT is a "discrete groupoid" — a groupoid with only trivial 2-morphisms.

## Section 3: The Homotopy Hypothesis

**Exercise 9.** The *singular simplicial set* $\mathsf{Sing}(X)$ of a topological space $X$:

(a) Describe $\mathsf{Sing}(X)_n$ for $n = 0, 1, 2$.
(b) Show that $\mathsf{Sing}(X)$ satisfies the inner horn filling condition (it is a quasi-category).
(c) Show that $\mathsf{Sing}(X)$ satisfies all horn filling conditions (it is a Kan complex).

**Exercise 10.** The *geometric realization* of a simplicial set $K$:

(a) Define $|K|$ as a CW complex: there is one $n$-cell for each non-degenerate $n$-simplex.
(b) Compute $|\Delta[n]|$ geometrically.
(c) Compute $|\Lambda^1[2]|$ geometrically — what topological space is it?
(d) What is $|K|$ for the simplicial set with $K_0 = \{*\}$, $K_1 = \{e\}$ (one 1-simplex with $d_0(e) = d_1(e) = *$), and all higher simplices degenerate? (This is the simplicial model of $S^1$.)

**Exercise 11.** The adjunction $|{-}| \dashv \mathsf{Sing}$ is the unit-counit pair:

(a) The unit $\eta_K : K \to \mathsf{Sing}(|K|)$ sends each simplex to its characteristic map. Describe $\eta_K$ for small $K$.
(b) The counit $\varepsilon_X : |\mathsf{Sing}(X)| \to X$ sends each singular simplex to its image. Why is $\varepsilon_X$ a weak homotopy equivalence (for $X$ a CW complex)?

## Section 4: (∞,1)-Categories

**Exercise 12.** A quasi-category $\mathcal{C}$ has 0-simplices (objects), 1-simplices (morphisms), 2-simplices (composition witnesses), etc.

(a) Two 1-simplices $f, g : a \to b$ are *homotopic* if there is a 2-simplex with $d_0(H) = g$, $d_1(H) = f$, and $d_2(H)$ degenerate. Show this is an equivalence relation.
(b) A 1-simplex $f : a \to b$ is an *equivalence* if there exists $g : b \to a$ such that $g \circ f$ is homotopic to $\mathsf{id}_a$ and $f \circ g$ is homotopic to $\mathsf{id}_b$. State this in terms of simplices and 2-simplices.

**Exercise 13.** Let $\mathcal{C}$ be a quasi-category.

(a) Show that any Kan complex is a quasi-category.
(b) Show that a quasi-category is a Kan complex iff every 1-simplex has an inverse up to homotopy (is an equivalence).

**Exercise 14.** The *functor quasi-category* $\mathsf{Fun}(\mathcal{C}, \mathcal{D})$ has $n$-simplices given by simplicial maps $\mathcal{C} \times \Delta[n] \to \mathcal{D}$. Show that if $\mathcal{D}$ is a Kan complex, then $\mathsf{Fun}(\mathcal{C}, \mathcal{D})$ is a Kan complex for any simplicial set $\mathcal{C}$.

## Section 5: ∞-Groupoids and Kan Complexes

**Exercise 15.** For a Kan complex $K$:

(a) Define $\pi_0(K)$ as the set of connected components (equivalence classes of 0-simplices where $a \sim b$ if there is a 1-simplex from $a$ to $b$).
(b) Define $\pi_1(K, v)$ for a vertex $v \in K_0$ as the set of homotopy classes of 1-simplices from $v$ to $v$ (loops at $v$), with group structure given by the composition from horn filling. Show this is a group.
(c) Compute $\pi_1(\mathsf{Sing}(S^1), *)$ and verify it is $\mathbb{Z}$.

**Exercise 16.** The simplicial model of the circle: define a Kan complex $S^1$ with:
- $K_0 = \{*\}$ (one vertex)
- $K_1 = \{e, \mathsf{id}\}$ (one non-degenerate 1-simplex $e : * \to *$ and the degeneracy of $*$)
- $K_2$: determined by Kan filling conditions on $K_0$ and $K_1$

Verify this is a Kan complex and that $\pi_1(K, *) \cong \mathbb{Z}$.

**Exercise 17.** In HoTT:

(a) Define the $n$-th homotopy group $\pi_n(A, a)$ for a type $A$ and basepoint $a : A$ as the $n$-th iterated loop space truncated to a set.
(b) Show $\pi_1(A, a) = \|(a =_A a)\|_0$ (the set-truncation of the loop space).
(c) For the HIT circle $S^1$, state (without full proof) the theorem $\pi_1(S^1) = \mathbb{Z}$ and describe the encode-decode strategy for its proof.

## Proof-Level Exercises

**Exercise 18.** (Hard) Prove that any bicategory $\mathcal{B}$ is equivalent (as a bicategory) to a strict 2-category. Construct the strictification $\mathsf{St}(\mathcal{B})$ as follows: objects are sequences of composable 1-morphisms; 1-morphisms are... (follow Mac Lane's strictification construction for monoidal categories).

**Exercise 19.** (Hard) Prove the ∞-categorical Yoneda lemma: for a quasi-category $\mathcal{C}$ and a functor $F : \mathcal{C} \to \mathcal{S}$ (to the quasi-category of Kan complexes), the natural map $\mathsf{Map}(\mathsf{Hom}(X, -), F) \to F(X)$ is an equivalence of Kan complexes. (This requires Lurie's machinery; see HTT Chapter 5.)

**Exercise 20.** In HoTT, prove that the ∞-groupoid structure of a type is functorial: given a function $f : A \to B$, the induced map $f_* : \Pi_\infty(A) \to \Pi_\infty(B)$ (given by $\mathsf{ap}_f$ at level 1, $\mathsf{ap}_{\mathsf{ap}_f}$ at level 2, etc.) is a functor of ∞-groupoids. (You need to show it preserves composition and identities at each level, and that these preservation laws satisfy the appropriate coherence conditions.)
