# The Homotopy Hypothesis

## Grothendieck's Conjecture

In his 1983 manuscript *Pursuing Stacks*, Grothendieck formulated the homotopy hypothesis:

**The Homotopy Hypothesis.** The ∞-category of homotopy types (topological spaces up to weak homotopy equivalence) is equivalent to the ∞-category of ∞-groupoids.

This conjecture has been proved in multiple senses, depending on what model of ∞-groupoids you use:

- **Simplicial sets:** The Quillen model structure on simplicial sets, where the fibrant objects are Kan complexes, gives a Quillen equivalence between $\mathbf{Top}$ (with the Quillen model structure) and $\mathbf{sSet}$ (with the Kan-Quillen model structure). Kan complexes are the ∞-groupoids. This is the most precise formulation.

- **Globular sets:** Multiple definitions of ∞-groupoid using globular sets (Batanin, Tamsamani, Maltsiniotis) have been shown equivalent to Kan complexes, confirming the hypothesis in the globular setting.

- **Complicial sets:** Street's approach using complicial sets (stratified simplicial sets) provides another model equivalent to Kan complexes.

The homotopy hypothesis is thus a theorem in each of these precise formulations. What Grothendieck conjectured informally has been made rigorous.

## Two Sides of the Equivalence

The equivalence has two directions:

**From spaces to ∞-groupoids.** Given a topological space $X$, the *singular simplicial set* $\mathsf{Sing}(X)$ is defined by:
$$\mathsf{Sing}(X)_n = \mathsf{Hom}_\mathbf{Top}(\Delta^n, X)$$
— the set of continuous maps from the standard $n$-simplex to $X$.

$\mathsf{Sing}(X)$ is always a Kan complex: the horn-filling conditions correspond to the homotopy extension property for maps from $\Delta^n$ extended by filling horns in $X$. The Kan complex $\mathsf{Sing}(X)$ captures the homotopy type of $X$: its fundamental groupoid is $\Pi_1(X)$, and its higher homotopy groups are $\pi_n(\mathsf{Sing}(X)) \cong \pi_n(X)$.

**From ∞-groupoids to spaces.** Given a Kan complex $K$, the *geometric realization* $|K|$ is the topological space built by gluing simplices: one simplex of dimension $n$ for each $n$-simplex of $K$, with the gluing maps given by the face and degeneracy maps.

$|K|$ is a CW complex (a topological space built from cells). Its homotopy type is equivalent to the "homotopy type" of the Kan complex $K$: $\pi_n(|K|) \cong \pi_n(K)$ (the homotopy groups of the Kan complex, defined combinatorially).

**The Quillen equivalence.** The adjunction $|{-}| \dashv \mathsf{Sing}$ is a Quillen equivalence between $\mathbf{Top}$ and $\mathbf{sSet}$ (both with appropriate model structures). This means: the homotopy categories of $\mathbf{Top}$ and $\mathbf{sSet}$ are equivalent as categories. And at the ∞-level: the ∞-categories of homotopy types and of Kan complexes are equivalent.

## The Homotopy Hypothesis for MLTT

There is a type-theoretic version of the homotopy hypothesis:

**Theorem (informal).** Types in MLTT are ∞-groupoids, and the ∞-groupoid structure is fully encoded in the identity types.

More precisely:

1. For any type $A$ in MLTT, the structure $(A, =_A, =_{=_A}, \ldots)$ of iterated identity types forms an ∞-groupoid.

2. The operations $\mathsf{refl}$, path concatenation, path inversion, transport, and the $J$ eliminator give the ∞-groupoid structure: identity 1-morphisms, composition of 1-morphisms, inverses, functorial action on higher morphisms.

3. The groupoid laws hold up to paths of the appropriate dimension:
   - Associativity of path concatenation: $(p \cdot q) \cdot r =_{a = d} p \cdot (q \cdot r)$ (a 2-path)
   - Unit laws: $\mathsf{refl} \cdot p = p$ and $p \cdot \mathsf{refl} = p$ (2-paths)
   - These associativity and unit 2-paths themselves satisfy coherence laws (3-paths), and so on

4. The ∞-groupoid structure is functorial: every function $f : A \to B$ induces a functor $f_* : \Pi_\infty(A) \to \Pi_\infty(B)$ of ∞-groupoids, with the functor being the action $\mathsf{ap}_f$ on paths and $\mathsf{ap}_{\mathsf{ap}_f}$ on 2-paths, etc.

**The meaning.** Types in HoTT are not just "things with elements" — they are "spaces with paths and homotopies." The identity type $a =_A b$ is the type of paths from $a$ to $b$. The type $p =_{a=b} q$ is the type of homotopies between paths. The full homotopy type of $A$ is captured by this tower.

## Truncations and the h-Level Hierarchy

The homotopy hypothesis gives meaning to the h-level hierarchy in HoTT:

- **Contractible types** (h-level $-2$): the ∞-groupoid has a single object $*$ and all morphisms are paths to $*$. Contractibility means the space is homotopy equivalent to a point.

- **Mere propositions** (h-level $-1$): the ∞-groupoid is either empty or contractible (the space is either empty or contractible). There is at most one object up to homotopy — a truth value.

- **Sets** (h-level $0$): the ∞-groupoid is discrete — all morphisms at level $\geq 1$ are trivial. The homotopy type has $\pi_n = 0$ for $n \geq 1$: no loops, no higher structure.

- **1-types** (h-level $1$): the ∞-groupoid has non-trivial 1-morphisms (the fundamental groupoid is non-trivial) but trivial morphisms at levels $\geq 2$. Homotopy types with $\pi_n = 0$ for $n \geq 2$: Eilenberg-MacLane spaces $K(G, 1)$.

- **$n$-types** (h-level $n$): homotopy types with $\pi_k = 0$ for $k > n$. The ∞-groupoid structure is non-trivial only up to level $n$.

The $n$-truncation $\|-\|_n$ sends a type to its $n$-type approximation: it destroys all higher homotopy information but preserves the homotopy groups up to level $n$.

## The Circle as a 1-Type

The circle $S^1$ is the canonical example of a 1-type. In HoTT, $S^1$ is defined as a higher inductive type:

```
HIT S¹ :
  base : S¹
  loop : base = base
```

The fundamental group $\pi_1(S^1, \mathsf{base}) = \mathbb{Z}$ (the integers). This is the canonical computation in HoTT: the type $\mathsf{base} =_{S^1} \mathsf{base}$ is equivalent to $\mathbb{Z}$.

The ∞-groupoid of $S^1$ has:
- One object up to homotopy (since $S^1$ is connected)
- The automorphism ∞-groupoid of the single object is $\mathbb{Z}$ (since $\pi_1(S^1) = \mathbb{Z}$ and $\pi_n(S^1) = 0$ for $n \geq 2$)

So $S^1$ as a homotopy type is completely captured by its fundamental group $\mathbb{Z}$ — it is an Eilenberg-MacLane space $K(\mathbb{Z}, 1)$.

The proof $\pi_1(S^1) = \mathbb{Z}$ in HoTT is a significant theorem, requiring the *encode-decode* method and careful reasoning about the ∞-groupoid structure. It is the canonical benchmark for HoTT proof assistants.

## Consequences for HoTT

The homotopy hypothesis, applied to types in HoTT, has the following consequences:

1. **Types are spaces.** Reasoning about types is reasoning about homotopy types. Theorems about types are synthetic homotopy-theoretic theorems.

2. **Functions are continuous maps.** Every function $f : A \to B$ in HoTT is automatically continuous in the homotopy-theoretic sense — it preserves the ∞-groupoid structure. There is no distinction between "continuous" and "arbitrary" functions.

3. **Equivalences are homotopy equivalences.** An equivalence $f : A \simeq B$ in HoTT corresponds to a homotopy equivalence of spaces: a continuous bijection with a continuous inverse.

4. **Univalence is natural.** If types are spaces, the universe $\mathcal{U}$ is a space of spaces. An "equality" between types $A = B$ in the universe is a path in this space — a homotopy equivalence between $A$ and $B$. Univalence says exactly this: $(A = B) \simeq (A \simeq B)$.

5. **HITs are colimits.** Higher inductive types are homotopy colimits in the ∞-groupoid sense. The circle $S^1$ is the homotopy pushout of a point with itself along two maps. The suspension $\Sigma A$ is the homotopy pushout of two copies of $A$'s cone. Every HIT has an interpretation as a homotopy colimit.

The homotopy hypothesis is not just a mathematical theorem. For HoTT, it is the *explanation* of what HoTT is: a logic and programming language for homotopy types.
