# 3.1 The Homotopy Hypothesis

## Grothendieck's Letter to Quillen

In 1983, Alexander Grothendieck wrote a famous letter to Daniel Quillen. Grothendieck was one of the most influential mathematicians of the 20th century — the inventor of schemes, étale cohomology, toposes, and much of modern algebraic geometry. By 1983 he had largely withdrawn from mathematics, but he returned briefly to sketch a visionary program: *Pursuing Stacks*.

In this letter (and in the manuscript that followed), Grothendieck conjectured that the study of homotopy types could be completely algebraized using a suitable notion of ∞-groupoid:

> "The study of $n$-truncated homotopy types (of semi-simplicial sets, or of topological spaces) is essentially equivalent to the study of so-called $n$-groupoids."

More precisely, and more boldly: the homotopy category of topological spaces (spaces up to weak equivalence) is *equivalent* to the category of ∞-groupoids. Not just analogous to, not just inspired by — equivalent.

This is the **Homotopy Hypothesis**.

Let's make it precise.

## The Setup: Two Worlds

We have two distinct mathematical worlds:

**World 1: Homotopy theory.** The objects are topological spaces. The morphisms are continuous maps. But we consider two maps equivalent if there's a homotopy between them, and two spaces equivalent if there's a weak homotopy equivalence between them (a map inducing isomorphisms on all homotopy groups $\pi_n$ for all $n \geq 0$).

**World 2: Higher groupoid theory.** The objects are ∞-groupoids (or rather, some particular model of them). The morphisms are "functors" between ∞-groupoids, with two considered equivalent if there's a suitable "natural equivalence" between them.

The **Homotopy Hypothesis** says: these two worlds are equivalent.

$$\text{Homotopy types} \simeq \text{∞-groupoids}$$

## Why This Should Be True

Let's first build intuition for why you'd even expect this.

**From topology to groupoids.** Given a topological space $X$, we've seen how to build its fundamental groupoid $\Pi_1(X)$. But $\Pi_1(X)$ only captures 1-dimensional information (paths and homotopy classes of paths). To capture higher homotopy, we need higher structure.

We can try to build:
- $\Pi_0(X)$: the set of connected components (a set = 0-groupoid)
- $\Pi_1(X)$: the fundamental groupoid (a 1-groupoid)
- $\Pi_2(X)$: the "fundamental 2-groupoid" — points, paths, homotopies between paths, up to homotopy of homotopies
- $\Pi_n(X)$: the fundamental $n$-groupoid
- $\Pi_\infty(X)$: the fundamental ∞-groupoid

Each $\Pi_n(X)$ captures more of the homotopy type of $X$. The fundamental ∞-groupoid $\Pi_\infty(X)$ should capture *all* of it.

**From groupoids to topology.** Given an ∞-groupoid $\mathcal{G}$, we want to build a topological space that "looks like" $\mathcal{G}$. The idea: objects of $\mathcal{G}$ become points, morphisms become paths, 2-morphisms become homotopies between paths, and so on.

This is the *geometric realization* operation. It takes combinatorial groupoid data and builds a topological space.

The homotopy hypothesis says these two constructions (singular complex and geometric realization) are mutually inverse equivalences.

## The Simplicial Set Formulation

The most precise and most useful formulation uses simplicial sets. Here's the key theorem:

**Theorem 3.1 (Quillen, 1967).** There is a Quillen equivalence between:
- The model category of topological spaces (with weak homotopy equivalences)
- The model category of simplicial sets with the Kan-Quillen model structure (with weak homotopy equivalences of simplicial sets)

The two functors are:
- **Geometric realization:** $|{-}| : \mathbf{sSet} \to \mathbf{Top}$, sending a simplicial set $K$ to the topological space $|K|$ built by gluing simplices together
- **Singular complex:** $\mathsf{Sing}(-) : \mathbf{Top} \to \mathbf{sSet}$, sending a space $X$ to the simplicial set $\mathsf{Sing}(X)$ where $\mathsf{Sing}(X)_n = \mathsf{Top}(\Delta^n, X)$ (continuous maps from the topological $n$-simplex to $X$)

These are adjoint ($|{-}| \dashv \mathsf{Sing}$) and the adjunction is a Quillen equivalence — meaning it becomes an equivalence of categories after inverting the weak equivalences (passing to homotopy categories).

**And Kan complexes are the ∞-groupoids in this formulation.** A simplicial set is a Kan complex if it satisfies the horn-filling condition (all horns $\Lambda^n_k \to X$ extend to $\Delta^n \to X$). The key:
- Outer horn filling ($k=0$ or $k=n$) corresponds to inverses existing
- Inner horn filling ($0 < k < n$) corresponds to compositions existing

Together, these are exactly the conditions for an "∞-groupoid" structure.

**So:** the homotopy hypothesis (in the simplicial formulation) says:

$$\text{Homotopy types of spaces} \simeq \text{Kan complexes}$$

This is not a conjecture — it's a theorem, proved by Quillen. The homotopy hypothesis in this formulation is established mathematics.

## The $n$-Truncated Case

Before the full ∞-case, it's illuminating to think about $n$-truncated homotopy types.

**0-truncated:** Spaces with $\pi_k(X) = 0$ for $k \geq 1$ (contractible components). These are just sets (up to equivalence). The corresponding 0-groupoids are just sets. ✓

**1-truncated (groupoids):** Spaces with $\pi_k(X) = 0$ for $k \geq 2$ — $K(\pi,1)$ spaces (Eilenberg-MacLane spaces). A $K(\pi,1)$ is a space with fundamental group $\pi$ and all higher homotopy groups trivial.

The homotopy hypothesis for 1-truncated types says: 1-truncated homotopy types correspond to groupoids. This is also established:
- A 1-truncated space corresponds to its fundamental groupoid $\Pi_1(X)$
- Given a groupoid $\mathcal{G}$, the classifying space $B\mathcal{G}$ (geometric realization of $\mathcal{G}$'s nerve) is a 1-truncated space
- These are mutually inverse equivalences ✓

**2-truncated:** Spaces with $\pi_k(X) = 0$ for $k \geq 3$. These correspond to 2-groupoids (groupoids with 2-cells). Proved by various people.

**General $n$:** The homotopy hypothesis for $n$-truncated types corresponds to $n$-groupoids. For strict $n$-groupoids, this is provable using crossed modules and related structures. For weak $n$-groupoids, the statement becomes more subtle.

## The Full Weak Case: Still Open

Here's where things get interesting. For the *fully general* homotopy hypothesis with *weak* ∞-groupoids (in a purely algebraic formulation using globular sets, for example), the situation is more complex.

Different definitions of weak ∞-groupoid give different notions of "∞-groupoid," and the homotopy hypothesis says different things for each. The simplicial formulation (Kan complexes) is the cleanest and is fully proved. But:

**For Grothendieck's original formulation** (using globular weak ∞-groupoids), the full equivalence is still not completely established in complete generality. This remains an active research area.

**For strict ∞-groupoids** (strict ω-groupoids), the homotopy hypothesis is *false* — strict ω-groupoids only model a restricted class of homotopy types (the "linear" ones, roughly). This is a theorem of Simpson (1998), showing that strictness is too strong a requirement.

The takeaway: the homotopy hypothesis is "proved" in the most useful sense (Kan complexes = homotopy types) but remains a guide/conjecture in some more algebraic formulations.

## The HoTT Formulation

For us, the most relevant formulation of the homotopy hypothesis is the internal one in HoTT:

**Types in HoTT are ∞-groupoids.**

Let's spell this out. Given a type $A$, we have a tower of types:
- Level 0: $A$ itself (terms are 0-cells)
- Level 1: $a =_A b$ for $a, b : A$ (identity proofs are 1-cells)
- Level 2: $p =_{a=_A b} q$ for $p, q : a =_A b$ (homotopies between paths are 2-cells)
- Level $n$: iterated identity types

This tower of structure is exactly an ∞-groupoid:
- Composition (path concatenation) at each level
- Inverses (path inversion) at each level
- Units (reflexivity) at each level
- Coherence (groupoid laws at each level, proved by J-based arguments)

The composition, units, and inverses are weak (they hold propositionally, not definitionally), which is exactly what makes this a *weak* ∞-groupoid structure.

**Univalence strengthens the connection.** Without Univalence, we have types as ∞-groupoids. With Univalence, we get that the *universe* $\mathsf{Type}$ is itself an ∞-groupoid, and paths in the universe are exactly equivalences of types. This turns HoTT into a full homotopy theory where you can reason about all homotopy types, not just the ones "built in" to the type theory.

## Consequences: Homotopy Theory Inside HoTT

The homotopy hypothesis means that homotopy-theoretic theorems have type-theoretic content. Let's list some:

**$\pi_1(S^1) = \mathbb{Z}$.** The fundamental group of the circle is the integers. In HoTT: the loop space $\Omega S^1$ (the type of loops at $\mathsf{base} : S^1$) is equivalent to $\mathbb{Z}$. This has a complete HoTT proof (Licata-Shulman).

**$\pi_n(S^n) = \mathbb{Z}$.** The $n$th homotopy group of the $n$-sphere is $\mathbb{Z}$. Partially provable in HoTT.

**Freudenthal suspension theorem.** The suspension map $\pi_k(A) \to \pi_{k+1}(\Sigma A)$ is an isomorphism in a range. Proved in HoTT.

**Blakers-Massey theorem.** A connectivity result about homotopy pushouts. Proved in HoTT (Favonia-Finster-Licata-Lumsdaine, 2016).

**The Hopf fibration.** The fibration $S^1 \to S^3 \to S^2$ can be constructed as a map of HITs in HoTT, giving $\pi_3(S^2) = \mathbb{Z}$ (the Hopf invariant theorem).

All of these are theorems of classical algebraic topology, now proved synthetically in HoTT. The homotopy hypothesis is what makes this translation valid.

## Why This Matters Foundationally

The homotopy hypothesis, combined with the fact that HoTT is the internal logic of ∞-toposes, gives us a new perspective on foundations:

**Classical foundations:** Mathematics is built on sets. Everything is a set (or a class), and equality is set-membership equality.

**HoTT foundations:** Mathematics is built on homotopy types. Types are spaces, terms are points, equality is paths. The structure of equality is homotopy-theoretic.

The homotopy hypothesis says these foundations are *equivalent* at the level of homotopy 0-types (sets). Sets are exactly the 0-truncated homotopy types, and set-level mathematics is preserved. But HoTT also captures higher structure that classical set-based foundations handle awkwardly (isomorphisms, equivalences, etc. — all captured by paths and Univalence).

This is the foundational significance of the homotopy hypothesis: it shows that HoTT's approach to equality (via paths) is not just an alternative convention, but is capturing a genuine mathematical structure (homotopy types) that classical foundations handle less naturally.

## Summary

| Formulation | Status |
|---|---|
| Simplicial: Kan complexes = homotopy types | Proved (Quillen 1967) |
| $n$-truncated: $n$-groupoids = $n$-truncated types | Proved for small $n$, known in general |
| Weak ∞-groupoids (globular) | Partially proved; Kan complexes give cleanest version |
| Strict ω-groupoids | False: not all homotopy types arise |
| HoTT internal: types = ∞-groupoids | Built into the type theory |

The homotopy hypothesis is the conceptual backbone of HoTT. It says that what type theorists have been doing (studying identity types and their structure) and what homotopy theorists have been doing (studying path spaces and their structure) are *the same thing*.
