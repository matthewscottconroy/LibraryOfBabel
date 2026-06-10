# Chapter 3: Real Analysis

## What Is a Path?

Draw a curve on a piece of paper from a point A to a point B. The curve is a *path*: it connects two points by a continuous route. You can go directly. You can loop around. You can zigzag. The only constraint is that the route must be continuous — no jumps, no teleportation.

Now ask: are two paths "the same"? If you can continuously deform one into the other — push the curve around without lifting it off the paper, without tearing, without changing the endpoints — then the two paths are *homotopic*. They are, in a precise sense, equivalent.

This is the seed of homotopy theory. Two paths are homotopic if you can deform one into the other continuously. And the central question of homotopy theory is: when can we deform? When can we not?

The answer depends on the shape of the space. In the plane ℝ², you can always deform: any two paths from A to B are homotopic, because there are no holes to obstruct the deformation. But on the punctured plane ℝ² \ {0} — the plane with a hole — paths that loop around the hole cannot be deformed into paths that don't. The hole is an obstruction.

This is why homotopy theory matters: it detects the *shape* of spaces by asking how their paths can be deformed. And it is why real analysis is the right starting point: paths are *continuous functions*, and homotopy is *continuous deformation*. We need the precise notion of continuity — the ε-δ definition, generalized to metric spaces — to make the definition of path and homotopy rigorous.

## The Chain from Analysis to HoTT

Real analysis contributes three things to this curriculum:

**Metric spaces as the right framework.** The analytic facts we need — continuity, convergence, compactness — are cleanest when stated for metric spaces. A metric space is a set with a distance function satisfying three axioms. Every normed vector space, every Riemannian manifold, every complete ordered field is a metric space. The metric space axioms isolate exactly the structure needed for analysis, nothing more.

**Paths and homotopies, precisely defined.** A path from x to y in a topological space X is a continuous function γ: [0,1] → X with γ(0) = x and γ(1) = y. A homotopy between two paths γ and δ is a continuous function H: [0,1] × [0,1] → X satisfying boundary conditions. These are definitions — clean, precise, and checkable.

**The identity problem in concrete form.** The real numbers can be constructed as Dedekind cuts or as equivalence classes of Cauchy sequences. These two constructions yield the *same* mathematical object — a complete ordered field — but *different* sets in ZFC. The uniqueness theorem says any two complete ordered fields are isomorphic, but in ZFC they are not equal. The Univalence Axiom resolves this: equivalent types are equal, so the two constructions are *literally* the same type.

## The Path From Analysis to HoTT

Every section of this chapter builds toward the topological content.

**Metric spaces** are the warm-up: axioms, examples, open balls, open sets. The key insight is that topology (what is open, what is closed) is determined by the metric, and conversely, equivalent metrics produce the same topology.

**Convergence and completeness** capture the idea that a space has "no gaps." The real numbers are complete; the rationals are not. Every Cauchy sequence in a complete metric space converges. And the completion theorem provides a universal construction: every metric space embeds into a complete metric space, its completion.

**Continuity** is the property we care about for paths. A continuous function is one that preserves nearness: points close together in the domain map to points close together in the range. The topological definition (preimages of open sets are open) is equivalent to the ε-δ definition and more elegant.

**Compactness** is one of the deepest analytic concepts. Compact spaces are "finite-like" in a precise sense: every open cover has a finite subcover. On compact spaces, continuous functions achieve their maxima, sequences have convergent subsequences, and uniform continuity holds automatically.

**Connectedness** asks whether a space can be split into two disjoint open parts. The Intermediate Value Theorem is the content of connectedness: a continuous function on a connected space cannot jump from one value to another without passing through everything in between.

**Paths and homotopy** are the culmination. We define paths, path composition, homotopy, and the fundamental group. We observe that path composition is associative only *up to homotopy* — not strictly. This is the seed of ∞-groupoid structure. And we state, without full proof, the bridge to HoTT: identity types are path spaces, and the structure of the identity type is the structure of the path space.

## A Note on the Constructive Real Numbers

One of the deep questions in constructive mathematics is: what *are* the real numbers, constructively? The Dedekind cut construction uses subsets of ℚ, which are available constructively. The Cauchy sequence construction uses sequences and an equivalence relation, also available constructively. Both yield structures with the same properties.

In HoTT, the real numbers are defined as the *Cauchy completion* of ℚ — a higher inductive type that freely adjoins limits of Cauchy sequences. The Dedekind reals are defined differently. Whether these two definitions give equal types (not just equivalent ones) is a theorem that requires the Univalence Axiom.

This is analysis in the service of foundations. The real numbers are not just a place to do calculus; they are a test case for what foundational frameworks can handle. Understanding both constructions — their similarities, their differences, the theorem that unifies them — is understanding what real analysis is actually about.
