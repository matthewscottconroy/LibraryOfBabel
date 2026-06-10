# Chapter 13: Point-Set Topology

A topologist once said that a coffee cup and a donut are the same thing. This is not a joke about breakfast. It is a precise mathematical statement: there exists a continuous deformation taking one into the other without tearing or gluing. Both objects have exactly one handle — one hole that passes all the way through — and that single topological fact makes them, for the purposes of topology, identical.

But here is what makes this statement philosophically remarkable: the equivalence is established without any measurement. You do not need to know the radius of the cup's handle or the diameter of the donut's hole. You do not need angles or lengths or areas. All you need is the abstract notion of *continuous deformation* — and it turns out that continuous deformation can be defined purely in terms of *open sets*, without any notion of distance at all.

Once you make that move — once you abstract away from metrics to the combinatorial structure of open sets — something extraordinary happens. You discover that the circle and the line are different, even though both are one-dimensional. You discover that the sphere and the torus are different, even though both are two-dimensional surfaces. You discover that these differences can be *detected by algebra*: by groups, by integers, by homomorphisms. The algebra that detects the difference between spaces — first homology groups, then homotopy groups — eventually becomes the identity types of Homotopy Type Theory. The circle becomes a higher inductive type with a loop constructor. The sphere becomes a type with a 2-cell constructor. The algebraic invariants of classical topology become the very structure of types in HoTT.

This chapter builds the classical foundation. It is not a detour; it is the thing itself.

## What Topology Abstracts

Here is the key insight of point-set topology, stated as plainly as possible.

In a metric space, a function $f : X \to Y$ is continuous at a point $x$ if: for every $\varepsilon > 0$ there exists $\delta > 0$ such that whenever $d(x, x') < \delta$ we have $d(f(x), f(x')) < \varepsilon$. This is the classical $\varepsilon$-$\delta$ definition. It refers explicitly to distances.

But there is an equivalent reformulation: $f$ is continuous if and only if *the preimage of every open set is open*. This reformulation mentions no distances — only open sets. And once you notice this, you realize that the metric is doing no work in the definition of continuity. What matters is the collection of open sets, and the topology axioms tell you which collections deserve to be called "open."

A topological space is a set $X$ together with a collection $\tau$ of subsets — the open sets — satisfying: the empty set and $X$ itself are open; arbitrary unions of open sets are open; finite intersections of open sets are open. That is all. Three axioms. And from these three axioms, the entire theory of continuous maps, homeomorphisms, paths, and homotopies follows.

The power of this abstraction is that it applies far beyond metric spaces. The Zariski topology of algebraic geometry, where closed sets are zero sets of polynomials, is a topology on the prime spectrum of a ring with no natural metric. The Scott topology of domain theory, used in the semantics of programming languages, is a topology on a partial order with no natural distance. Topology is the right language for continuity in all of these settings.

## What This Chapter Covers

**Section 1 (Topological Spaces)** develops the axiom system, works through the key examples — metric topologies, the discrete and indiscrete topologies, the Sierpiński space, the Sorgenfrey line — and introduces the fundamental constructions: subspace topology, product topology, quotient topology. These constructions are not just examples; they are the categorical operations that build the spaces of interest from simpler pieces.

**Section 2 (Continuous Maps)** develops the preimage definition of continuity, establishes its equivalence with the $\varepsilon$-$\delta$ definition in metric spaces, and studies the morphisms of the category **Top**: homeomorphisms, open maps, closed maps, and quotient maps. The central result is the characterization of continuity via universal properties.

**Section 3 (Connectedness)** distinguishes two notions — connectedness and path-connectedness — that are equivalent for "nice" spaces but differ in general. The topologist's sine curve is the canonical example of a space that is connected but not path-connected. Path-connectedness is more useful for homotopy theory, because it is path-connectedness that corresponds to the 0th homotopy set $\pi_0$.

**Section 4 (Compactness)** develops the open-cover definition of compactness, proves the Heine-Borel theorem characterizing compact subsets of $\mathbb{R}^n$, establishes that compactness is preserved by continuous maps, and surveys Tychonoff's theorem (the product of any family of compact spaces is compact) and its connection to the axiom of choice.

**Section 5 (Quotient Spaces)** examines the quotient construction in depth: the circle as $[0,1]/\{0 \sim 1\}$, the torus as $\mathbb{R}^2/\mathbb{Z}^2$, the Klein bottle, projective spaces, and CW complexes as iterated pushouts of disks. The fundamental theorem of quotient topology — that continuous maps out of a quotient space are exactly continuous maps out of the original space that respect the equivalence relation — is the categorical backbone of the construction.

**Section 6 (CW Complexes and HoTT)** makes the connection explicit. CW complexes are spaces built by attaching cells in increasing dimension: 0-cells are points, 1-cells are edges attached at their endpoints, 2-cells are disks whose boundary circles are attached along paths in the 1-skeleton. This is exactly the structure of higher inductive types in HoTT: 0-dimensional constructors are point constructors, 1-dimensional constructors are path constructors, and $n$-dimensional constructors are higher path constructors. The circle HIT with one point and one loop constructor corresponds to the CW complex with one 0-cell and one 1-cell. HoTT's higher inductive types are synthetic CW complexes.

## The HoTT Connection in One Sentence

Classical topology builds spaces by gluing cells together; HoTT builds types by declaring constructors and path constructors. The mathematics is the same, expressed in two different languages.
