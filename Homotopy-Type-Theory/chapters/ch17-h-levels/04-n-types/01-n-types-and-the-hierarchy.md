# 4.1 n-Types and the Full Hierarchy

## The Pattern Continues

We've seen:
- Contractible types (h-level $-2$): exactly one element up to paths
- Propositions (h-level $-1$): at most one element up to paths
- Sets (h-level $0$): paths between elements are unique (propositions)

The pattern is recursive: each level's "being simple" is defined by the previous level applied to identity types. This gives the full hierarchy of $n$-types.

## The General Definition

**Definition 4.1 ($n$-Type).** We define $\mathsf{is}\text{-}n\text{-}\mathsf{type} : \mathsf{Type} \to \mathsf{Type}$ by induction on $n : \mathbb{N}$ (with the convention that $-2$ is the starting point):

$$\mathsf{is}\text{-}(-2)\text{-}\mathsf{type}(A) :\equiv \mathsf{isContr}(A)$$
$$\mathsf{is}\text{-}(n+1)\text{-}\mathsf{type}(A) :\equiv \prod_{x, y : A}\, \mathsf{is}\text{-}n\text{-}\mathsf{type}(x = y)$$

Expanding:
- A $(-2)$-type is contractible
- A $(-1)$-type $A$ has: all identity types $x = y$ are contractible, i.e., $A$ is a proposition
- A $0$-type $A$ has: all identity types $x = y$ are propositions, i.e., $A$ is a set
- A $1$-type $A$ has: all identity types $x = y$ are sets, i.e., $A$ is a "1-groupoid"
- A $2$-type $A$ has: all identity types $x = y$ are 1-types, i.e., $A$ is a "2-groupoid"

In general, an $n$-type is a type whose $n+3$-fold iterated identity types are all contractible (trivial).

**The h-level of $A$** is the smallest $n$ such that $A$ is an $n$-type (if such $n$ exists). Types for which no such $n$ exists (like $S^2$, which has nontrivial $\pi_k$ for arbitrarily large $k$) have "infinite h-level."

## The Cumulative Property

**Theorem 4.2 (Cumulativity).** If $A$ is an $n$-type, then $A$ is an $(n+1)$-type.

*Proof.* By induction on $n$. The base case: if $A$ is contractible ($(-2)$-type), then $A$ is a proposition ($(-1)$-type) — this follows from Theorem 2.6 (contractible implies proposition).

For the inductive case: if $A$ is an $n$-type, then all identity types $x = y$ are $(n-1)$-types. By the inductive hypothesis, they're also $n$-types. So $A$ is an $(n+1)$-type. $\square$

This gives the nested hierarchy:
$$\cdots \subset \text{Contractible} \subset \text{hProp} \subset \text{hSet} \subset 1\text{-Type} \subset 2\text{-Type} \subset \cdots$$

Everything that's contractible is a proposition; everything that's a proposition is a set; etc.

## Homotopy Theory Correspondence

The h-level hierarchy in HoTT corresponds precisely to the Postnikov tower in classical homotopy theory:

**Classical:** An $n$-type is a topological space with $\pi_k = 0$ for all $k > n$.

**HoTT correspondence:**
| HoTT h-level | Classical n-type | Key property |
|---|---|---|
| $-2$ | Contractible space | $\pi_k = 0$ for all $k$ |
| $-1$ | Empty space or single point | $\pi_k = 0$ for all $k \geq 0$ |
| $0$ | Discrete space | $\pi_k = 0$ for all $k \geq 1$ |
| $1$ | Eilenberg-MacLane space $K(G,1)$ | $\pi_k = 0$ for $k \geq 2$ |
| $2$ | 2-type | $\pi_k = 0$ for $k \geq 3$ |
| $n$ | $n$-type | $\pi_k = 0$ for $k \geq n+1$ |

Note the offset: a HoTT $n$-type corresponds to a classical $(n+2)$-type (or equivalently, an $(n+2)$-truncated space). This is a convention choice.

**Example: Eilenberg-MacLane spaces.** A $1$-type in HoTT with fundamental group $G$ corresponds to $K(G, 1)$ in classical homotopy theory — a space whose only non-trivial homotopy group is $\pi_1 = G$. The classifying space $BG$ of a group $G$ is a $K(G,1)$.

**Example: $S^1$ is a 1-type.** The circle $S^1$ in HoTT has:
- $\pi_1(S^1) = \mathbb{Z}$ (fundamental group)
- $\pi_k(S^1) = 0$ for $k \geq 2$ (higher homotopy groups vanish)

So $S^1$ is a 1-type (h-level 1). This is a theorem in synthetic homotopy theory (Chapter 20).

**Example: $S^2$ has infinite h-level.** The 2-sphere has $\pi_2(S^2) = \mathbb{Z}$, $\pi_3(S^2) = \mathbb{Z}$ (Hopf fibration), and many other non-trivial homotopy groups. It's a 2-type but not a 1-type or lower. In HoTT, $S^2$ is defined as a HIT, and computing its homotopy groups is a significant achievement (Chapter 20).

## 1-Types: Groupoids in HoTT

A 1-type in HoTT is exactly a *groupoid* (in the categorical sense):

**Theorem 4.3.** A type $A$ is a 1-type if and only if the fundamental groupoid structure of $A$ (from Chapter 12) has all 2-cells being identities — i.e., $A$ is a *1-groupoid* where the 2-morphisms are trivial.

More precisely: $A$ is a 1-type iff for all $x, y : A$, the type $x = y$ is a set. And a set has only trivial 2-paths, so the hom-sets of the fundamental groupoid are sets.

**Examples of 1-types:**
- Any group $G$ (viewed as a one-object groupoid $BG$)
- The fundamental groupoid $\Pi_1(X)$ of any topological space $X$
- Any type whose connected components all have trivial $\pi_2, \pi_3, \ldots$

**The homotopy hypothesis for 1-types.** The Grothendieck homotopy hypothesis, restricted to 1-types, says: 1-types in HoTT (groupoids with discrete morphism spaces) correspond to classical 1-types ($K(G,1)$ spaces). This is a theorem, proved using the simplicial set model.

## Closure Properties of n-Types

$n$-types are closed under many type-theoretic constructions:

**Theorem 4.4 (Products).** If $A$ and $B$ are $n$-types, then $A \times B$ is an $n$-type.

*Proof.* By induction on $n$. For $n = -2$: products of contractible types are contractible (the center of the product is the pair of centers). For $n+1$: paths in $A \times B$ decompose as pairs of paths in $A$ and $B$. If $A$ and $B$ are $(n+1)$-types, then $x = y$ in $A \times B$ is $(A\text{-path}) \times (B\text{-path})$, and products of $n$-types are $n$-types by induction. $\square$

**Theorem 4.5 (Σ-types).** If $A$ is an $n$-type and $B : A \to \mathsf{Type}$ is a family of $n$-types, then $\sum_{x:A} B(x)$ is an $n$-type.

*Proof.* By the Σ-path characterization: paths in $\sum_{x:A} B(x)$ are pairs of a path in $A$ and a transported path in $B$. These are $n$-type paths by assumption. $\square$

**Theorem 4.6 (Function types).** If $B$ is an $n$-type, then $A \to B$ is an $n$-type for any $A$.

*Proof.* By funext: paths in $A \to B$ are families of paths in $B$, which are $n$-type paths. And products (over $A$) of $n$-type path spaces are $n$-type path spaces (by Theorem 4.4 generalized). $\square$

**Theorem 4.7 (Universe).** The universe $\mathsf{Type}_n$ of all $n$-types is itself a $(n+1)$-type.

This is a theorem in HoTT (provable from Univalence): paths in the universe of $n$-types are equivalences between $n$-types, and equivalences between $n$-types form an $n$-type, so the universe of $n$-types is an $(n+1)$-type. This is the universe analogue of the h-level hierarchy.

## The Postnikov Tower

Every type $A$ in HoTT has a *Postnikov tower*: a sequence of $n$-type approximations that "converge to" $A$ as $n \to \infty$:

$$A \to \cdots \to \|A\|_2 \to \|A\|_1 \to \|A\|_0 \to \|A\|_{-1} \to \|A\|_{-2}$$

where $\|A\|_n$ is the $n$-truncation (Section 5).

Each map $\|A\|_{n+1} \to \|A\|_n$ "forgets" the $(n+1)$-level homotopy of $A$:
- $\|A\|_{-1}$: just "is $A$ inhabited?" (the mere existence)
- $\|A\|_0$: the set $\pi_0(A)$ of connected components
- $\|A\|_1$: the "1-type approximation" of $A$ (with the right $\pi_0$ and $\pi_1$ but trivial higher groups)
- $\|A\|_n$: the "$n$-type approximation" with the right $\pi_0, \pi_1, \ldots, \pi_n$

In classical homotopy theory, this is the Postnikov tower. In HoTT, it's constructed via the truncation HIT (Chapter 19).

**The Postnikov-Whitehead theorem (homotopy theory).** A map $f : A \to B$ is an equivalence iff it induces isomorphisms on all homotopy groups. The HoTT version: $f : A \to B$ is an equivalence iff $\|f\|_n : \|A\|_n \to \|B\|_n$ is an equivalence for all $n$.

## Why n-Types Matter

The $n$-type hierarchy is not just a mathematical structure — it has practical implications:

**Proof irrelevance at different levels.** At h-level $-1$ (propositions), all proofs are equal — proof is irrelevant. At h-level 0 (sets), paths between elements are irrelevant but elements themselves matter. At h-level 1 (groupoids), 2-paths between paths are irrelevant.

This gives a stratified notion of proof relevance: you can be proof-irrelevant at level $n$ while being proof-relevant at levels below $n$.

**Truncated logic.** Mathematics formalized at h-level 0 (sets and their functions) matches classical set-theoretic mathematics. Mathematics formalized at h-level 1 includes categories (which have a groupoid of morphisms). Higher h-levels correspond to higher category theory.

**Computational extraction.** When a type is $n$-truncated, the computational content above level $n$ can often be erased. Propositions ($(-1)$-truncated) can always be erased — their proofs don't affect computation. Sets (0-truncated) have erasable 2-paths.

## n-Types in the Simplicial Set Model

In the simplicial set model:
- $n$-types correspond to $(n+2)$-coskeletal Kan complexes (all $k$-cells for $k > n+2$ are determined by lower cells)
- The Postnikov section $\|A\|_n$ is the $(n+2)$-skeleton of $A$
- Truncation is a left adjoint to the inclusion of $n$-types into all types

This geometric picture makes the h-level hierarchy concrete: $n$-types are the "geometrically simple" Kan complexes that don't have interesting structure above dimension $n+2$.

## Summary Table

| h-level | Name | Path spaces | Topology | Example |
|---|---|---|---|---|
| $-2$ | Contractible | Contractible | Single point | $\mathbf{1}$, $\sum_{x:A}(a=x)$ |
| $-1$ | Proposition | Contractible | $\emptyset$ or point | $\mathbf{0}$, $a = b$, $\|A\|$ |
| $0$ | Set | Propositions | Discrete space | $\mathbb{N}$, $\mathsf{Bool}$, $\mathbb{Z}$ |
| $1$ | 1-Groupoid | Sets | $K(G,1)$ space | $S^1$, $BG$ |
| $2$ | 2-Groupoid | 1-Groupoids | 2-type | $S^2$, $K(G,2)$ |
| $n$ | $n$-Groupoid | $(n-1)$-Groupoids | $n$-type | $S^n$ (if $\pi_k=0$ for $k>n$) |
| $\infty$ | ∞-Groupoid | ∞-Groupoids | Any space | $S^2$, general spaces |

The h-level hierarchy is the type-theoretic organization of mathematical complexity: each level adds one more "dimension" of interesting structure.
