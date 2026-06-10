# Unit 05: Topology — The Mathematics of Shape, Continuity, and Identity

There is a question you may not have known you were asking, but which has been lurking behind every mathematical object you have ever met: *when are two things the same kind of thing?*

Not identical — not equal in every detail — but the same *shape*, the same *structure*, the same *thing seen from a different angle*. A square and a circle are not identical. But they are the same kind of thing to a topologist: each is a simple closed curve, a one-dimensional loop embedded in the plane, and there is a continuous bijection between them. A sphere and a torus are not the same kind of thing: no continuous deformation takes one to the other without tearing. Topology is the mathematical discipline that makes this distinction precise — and makes it *computable*.

## What Topology Is

Topology begins with a radical act of abstraction. Classical geometry cares about distances, angles, and rigid structure. Topology throws all of that away. It keeps only the information needed to define *continuity*: which sets are "open," in a precise axiomatic sense. This is surprisingly little data — just a collection of subsets satisfying three axioms — but it is exactly enough to define continuous maps, homeomorphisms, paths, loops, and ultimately the entire algebraic machinery of homotopy theory.

The payoff for this austerity is generality. By working with open sets rather than metrics, you prove theorems that apply simultaneously to the real line, to infinite-dimensional function spaces, to the prime spectra of rings in algebraic geometry, and to the Scott topology of domain theory in computer science. Topology is the common language of continuous mathematics.

But there is a further payoff, one that took the twentieth century to discover: topology turns out to be the *semantics* of type theory.

## Why HoTT Internalizes Topology

Homotopy Type Theory does not treat topology as an external subject — a domain of application, a source of examples. HoTT *is* topology, done synthetically, from the inside. The identity type $a =_A b$ is not an assertion that $a$ and $b$ are definitionally equal; it is a *path* from $a$ to $b$ in the space $A$. The type $A \simeq B$ is not merely "isomorphism" in some abstract sense; it is *homotopy equivalence*. The univalence axiom says that homotopy equivalent types *are* equal — that paths in the universe correspond exactly to equivalences of types.

This is not a metaphor. Voevodsky proved it literally: there is a mathematical model of HoTT built out of *simplicial sets*, in which every type is interpreted as a Kan complex (a combinatorial homotopy type), every term as a vertex, every identity proof as an edge, every higher identity as a higher simplex. In this model, all of HoTT's type-theoretic structure corresponds exactly to the homotopy-theoretic structure of simplicial sets.

To understand HoTT at this depth, you need to understand topology. Not as background knowledge, but as the very thing being captured.

## Unit Structure

This unit develops the topological foundations in three chapters, each with its own character and purpose.

**Chapter 13: Point-Set Topology.** The foundation. We build topological spaces from axioms, study continuous maps and their properties, and examine the key constructions — subspace topology, product topology, quotient topology — that generate the spaces of interest. We meet connectedness, compactness, and the key bridge: CW complexes, the spaces built by attaching cells in dimensions 0, 1, 2, and so on, which are the natural habitat of homotopy theory and whose constructors correspond exactly to HoTT's higher inductive types.

**Chapter 14: Homotopy Theory.** The main event. Homotopy equivalence, the fundamental group, covering spaces, higher homotopy groups, and fibrations. This is where topology becomes genuinely algebraic — where the geometric structure of spaces is captured by groups, where the Galois theory of field extensions reappears as the classification of covering spaces, where the long exact sequence of a fibration becomes the main computational engine. The Hopf fibration, which gives $\pi_3(S^2) = \mathbb{Z}$, is one of the most beautiful objects in mathematics; we will see it here.

**Chapter 15: Simplicial Sets.** The foundation of the foundation. Simplicial sets provide a purely combinatorial account of homotopy theory — no open sets, no continuous maps, just sets and face and degeneracy maps satisfying algebraic identities. The Quillen model structure shows that simplicial sets and topological spaces carry the same homotopy-theoretic information. And Voevodsky's simplicial set model shows that HoTT is consistent, by interpreting every type as a Kan complex and proving that the univalence axiom holds. This chapter answers the question: why does HoTT work?

## Prerequisites

You should be comfortable with:
- Set theory at the level of functions, relations, and cardinality
- Category theory: functors, natural transformations, adjunctions, limits and colimits
- Basic algebra: groups, group homomorphisms, quotient groups

We will not assume prior exposure to topology, but we will move quickly.

## The Thread

Every chapter in this unit returns to the same thread: *what does it mean for two things to be the same?* Point-set topology answers with homeomorphism. Homotopy theory answers with homotopy equivalence. HoTT answers with identity types. These three answers are not different answers to the same question — they are the same answer, expressed at different levels of abstraction. By the end of this unit, you will see why.
