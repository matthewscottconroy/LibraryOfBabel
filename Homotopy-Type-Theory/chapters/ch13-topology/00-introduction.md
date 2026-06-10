# Chapter 13: Point-Set Topology

## What is Topology, Really?

Suppose you have two shapes: a coffee mug and a donut. To a topologist, these are the same object — both have a single hole, and you can continuously deform one into the other without tearing or gluing. But a sphere and a donut are different: no continuous deformation takes you from one to the other.

Topology is the study of properties that are preserved under continuous deformation. Not rigid properties like distances or angles (those are the domain of geometry), but flexible properties: connectedness, the number of holes, whether a space can be twisted, and so on.

To study these properties rigorously, we need a precise notion of what "continuous deformation" means. That requires a precise notion of "topological space" — a set equipped with the data needed to define continuity.

## Why Topology Matters for HoTT

At first glance, HoTT seems purely algebraic: types, terms, rules. But there's a deep geometric interpretation underneath:

- **Types are spaces.** A type $A$ is interpreted as a topological space (or more precisely, a homotopy type).
- **Terms are points.** An element $a : A$ is a point in the space.
- **Paths are paths.** An identity proof $p : a =_A b$ is a path from $a$ to $b$.
- **Higher paths are homotopies.** An identity $H : p = q$ between two paths is a homotopy from $p$ to $q$.

This interpretation is not just a metaphor — it's a theorem (the simplicial set model of Chapter 11, and Voevodsky's consistency proof). Every construction in HoTT corresponds to a topological construction, and vice versa.

To understand this correspondence, you need to understand topology. This chapter builds the classical foundation.

## The Core Idea: Abstracted Continuity

Classical topology starts from the observation that "open sets" are what you really need to define continuity:

A function $f : X \to Y$ between metric spaces is continuous at $x$ if and only if for every open ball $B(f(x), \varepsilon)$ around $f(x)$, there is an open ball $B(x, \delta)$ around $x$ with $f(B(x,\delta)) \subseteq B(f(x), \varepsilon)$.

This is the $\varepsilon$-$\delta$ definition. But it's equivalent to: *the preimage of every open set is open.* And this equivalent formulation no longer mentions distances! It only mentions "open sets."

So: if you want to talk about continuity without a metric, just axiomatize what "open sets" are allowed to be. That's a topology.

**Definition (informal).** A *topological space* is a set $X$ together with a collection $\tau$ of "open" subsets satisfying:
1. The empty set and all of $X$ are open
2. Arbitrary unions of open sets are open
3. Finite intersections of open sets are open

With this data, you can define continuity (preimages of opens are open), homeomorphism (invertible continuous maps with continuous inverse), and eventually homotopy (continuous deformations).

## Chapter Roadmap

**Section 1 (Topological Spaces):** The definition, key examples (metric topology, discrete, indiscrete, Zariski, Sierpinski), and the main constructions (subspaces, products, quotients).

**Section 2 (Continuous Maps):** Continuity, homeomorphisms, the category **Top** of topological spaces, and what it means for two spaces to be "topologically the same."

**Section 3 (Connectedness):** Connected spaces, path-connected spaces, connected components. These are the precursors to $\pi_0$ (the 0th homotopy group/set) in HoTT.

**Section 4 (Compactness):** The key finiteness condition in topology. Heine-Borel theorem, Tychonoff's theorem, behavior under continuous maps.

**Section 5 (Quotient Spaces):** How to build new spaces by gluing. The circle, torus, projective space, CW complexes. This is central to homotopy theory.

**Section 6 (Topology and HoTT):** How the classical constructions map to HoTT. The synthetic approach: why we don't need to specify open sets in type theory.

## What Makes a Good Topology?

Here's a question worth holding in mind throughout the chapter: *why these axioms?*

The topology axioms (arbitrary unions, finite intersections) come from the study of metric spaces, where:
- Arbitrary unions of open balls are open (because openness is a local condition)
- Finite intersections of open balls are open (you can always find a smaller open ball in the intersection)
- Infinite intersections need not be open (the intersection $\bigcap_{n=1}^\infty (-1/n, 1/n) = \{0\}$ is a single point, not open in $\mathbb{R}$)

So the axioms encode what's true for metric spaces. But the axiom system is more general: there are topological spaces with no underlying metric (non-metrizable spaces) that still satisfy the axioms.

This abstraction is powerful: by working with the axioms directly rather than with specific metrics, we prove theorems that apply to all topological spaces at once — including exotic ones like the Zariski topology (used in algebraic geometry) or the Scott topology (used in domain theory and computer science).

## A Note on Set-Theoretic Foundations

Classical topology takes place within set theory (ZFC). We work with sets $X$, subsets $\tau \subseteq \mathcal{P}(X)$, and functions $f : X \to Y$.

In HoTT, the situation is different: we don't have a separate notion of "set" vs. "type" — types play both roles. And the "topology" of a type is implicit in its identity type structure.

This chapter is about the classical theory, which serves as the semantic foundation for HoTT. In Chapter 20 (Synthetic Homotopy Theory), we'll see how the classical notions are axiomatized internally in HoTT, without needing to specify open sets.

For now: think of this chapter as building the model that HoTT is designed to capture.
