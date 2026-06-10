# Chapter 14: Homotopy Theory

## The Core Question

Topology asks: when are two spaces homeomorphic? That's a very fine question — homeomorphism is preserved under no deformation at all. But often we care less about the rigid structure of a space and more about its "shape" in a flexible sense: how many holes does it have? What kinds of loops exist? Can you wrap a sphere around it?

Homotopy theory asks a coarser question: when are two spaces *homotopy equivalent*? Two spaces are homotopy equivalent if you can continuously deform one into the other, even if the deformation is not invertible point-by-point. The real line $\mathbb{R}$ and a single point are homotopy equivalent (you can contract $\mathbb{R}$ to a point), even though they're not homeomorphic.

This coarser notion is exactly the right one for studying *algebraic invariants* of spaces: the fundamental group, higher homotopy groups, homology groups. These invariants are all homotopy-invariant (preserved by homotopy equivalence) but not homeomorphism-invariant in general.

And for HoTT, homotopy equivalence is the key concept — because HoTT's notion of equality between types is exactly homotopy equivalence.

## The Fundamental Idea

A *homotopy* between maps $f, g : X \to Y$ is a continuous one-parameter family of maps $H_t : X \to Y$ (for $t \in [0,1]$) that starts at $f = H_0$ and ends at $g = H_1$. You can think of $H$ as a "movie" that continuously deforms $f$ into $g$.

A *homotopy equivalence* is a map $f : X \to Y$ that has a homotopy inverse $g : Y \to X$ — meaning $f \circ g$ is homotopic to $\mathsf{id}_Y$ and $g \circ f$ is homotopic to $\mathsf{id}_X$. Not inverse on the nose, but inverse up to continuous deformation.

In HoTT:
- A homotopy $f \sim g$ corresponds to a term of type $\prod_{a:A} f(a) = g(a)$
- A homotopy equivalence $A \simeq B$ is a function with a quasi-inverse
- Univalence says: homotopy equivalences *are* paths in the universe ($A \simeq B$ gives $A = B$)

## The Three Big Tools

The machinery of classical homotopy theory rests on three interconnected tools:

**1. The fundamental group $\pi_1(X, x_0)$.** Given a basepoint $x_0$, the fundamental group measures the "1-dimensional holes" in $X$ — the distinct ways to travel around a loop in $X$ and come back to the start. Formally: homotopy classes of loops at $x_0$, with path concatenation as the group operation.

**2. Covering spaces.** A covering space $p : \tilde{X} \to X$ is a space that "covers" $X$ by laying multiple copies (sheets) over it. Covering spaces are completely classified by subgroups of $\pi_1(X)$, establishing a beautiful correspondence between topology and algebra.

**3. Higher homotopy groups $\pi_n(X, x_0)$ and fibrations.** The long exact sequence of a fibration relates $\pi_n$ of base, total space, and fiber. This is the main computational tool for homotopy groups.

## The Key Theorems

The landmark theorems we'll develop:

**$\pi_1(S^1) = \mathbb{Z}$.** The circle has fundamental group $\mathbb{Z}$, with generator the "go around once" loop. This is the prototype of all fundamental group calculations. In HoTT, this is a theorem with a complete proof.

**Seifert-van Kampen theorem.** The fundamental group of a union $U \cup V$ is the pushout (amalgamated free product) of $\pi_1(U)$ and $\pi_1(V)$ over $\pi_1(U \cap V)$. This is the main computational tool for $\pi_1$.

**Classification of covering spaces.** Covering spaces of $X$ correspond to subgroups of $\pi_1(X)$.

**Long exact sequence of a fibration.** For $F \to E \to B$: the sequence $\ldots \to \pi_n(F) \to \pi_n(E) \to \pi_n(B) \to \pi_{n-1}(F) \to \ldots$ is exact.

**Whitehead's theorem.** A map between CW complexes that induces isomorphisms on all homotopy groups is a homotopy equivalence.

## Why HoTT Cares

The connection between homotopy theory and HoTT is deep and two-directional:

**From topology to HoTT:** The classical homotopy-theoretic concepts (paths, homotopies, fibrations, equivalences) are *axiomatized* in HoTT. When you write $p : a = b$ in HoTT, you're talking about a path. When you write $f \simeq g$, you're talking about a homotopy. The type-theoretic operations mirror the topological ones.

**From HoTT to topology:** Theorems proved in HoTT are theorems about all ∞-toposes simultaneously. The synthetic proof of $\pi_1(S^1) = \mathbb{Z}$ in HoTT (Chapter 20) is a proof that works in any ∞-topos with a circle HIT — it's more general than the classical proof.

**The Univalence axis:** The Univalence axiom takes the homotopy-theoretic viewpoint to its logical conclusion: equivalent types are *equal*. This is the type-theoretic assertion that homotopy equivalence is the right notion of sameness for types/spaces.

## Chapter Roadmap

**Section 1 (Homotopy Equivalences):** Homotopy between maps, homotopy equivalence between spaces, deformation retracts. Key examples: contractible spaces, homotopy type.

**Section 2 (Fundamental Group):** Loops, concatenation, the fundamental group as a functor. Van Kampen's theorem. Computing $\pi_1$ for circles, tori, projective spaces.

**Section 3 (Covering Spaces):** Definition, lifting theorems, the classification theorem. The correspondence between covering spaces and subgroups of $\pi_1$.

**Section 4 (Higher Homotopy Groups):** $\pi_n$ as homotopy classes of maps from $S^n$. Abelianness for $n \geq 2$. Eilenberg-MacLane spaces. The Postnikov tower.

**Section 5 (Fibrations):** Hurewicz fibrations, path-loop fibration, long exact sequence. The Hopf fibration and its consequences.

## Prerequisites and Connections

This chapter builds on:
- Point-set topology (Chapter 13): topological spaces, continuous maps, quotient spaces
- Category theory (Chapter 10): functors, natural transformations (for the functoriality of $\pi_n$)

This chapter connects to:
- Simplicial sets (Chapter 15): the categorical model of homotopy theory
- Identity types in HoTT (Chapter 16): the type-theoretic model of path spaces
- Synthetic homotopy theory (Chapter 20): proving classical homotopy theorems inside HoTT
