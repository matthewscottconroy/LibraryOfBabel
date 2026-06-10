# Chapter 3: Real Analysis

## What This Chapter Is About

Real analysis is where calculus grows up. In an introductory calculus course, you learn that a sequence converges if it "gets close to a limit," a function is continuous if it "doesn't jump," and a set is compact if it's "closed and bounded." These are excellent intuitions, but they are not definitions. They're poetic descriptions that leave crucial questions unanswered: close in what sense? Bounded with respect to what? What does "doesn't jump" really mean when the function takes infinitely many values?

This chapter develops real analysis from first principles, in the generality of *metric spaces*. The move to metric spaces is not about generality for its own sake — it's because the right level of abstraction makes the proofs cleaner, the theorems more widely applicable, and the underlying logic more visible. When you prove the Intermediate Value Theorem for continuous functions on connected metric spaces, you see exactly which properties of the real line are doing the work.

## Why Analysis Matters for This Curriculum

You might wonder why a curriculum about Homotopy Type Theory spends a chapter on real analysis. The answer is threefold.

**First, topology.** The concepts of metric spaces — open sets, continuity, compactness, connectedness — generalize to *topological spaces*, and topology is the native language of homotopy theory. When we later discuss homotopy groups, fibrations, and fiber bundles, we'll be speaking a language that has its roots in the analysis of metric spaces.

**Second, paths.** A path in a topological space $X$ is a continuous function $\gamma : [0, 1] \to X$. A homotopy between two paths is a continuous function $H : [0, 1] \times [0, 1] \to X$. In Homotopy Type Theory, identity types *are* paths, and the structure of the path type mirrors the structure of the analytic notion of homotopy. The connection is not merely analogical — it's the motivating insight behind the whole subject.

**Third, the identity problem in new clothes.** The real numbers can be constructed in two different ways: as Dedekind cuts (downward-closed subsets of $\mathbb{Q}$) or as equivalence classes of Cauchy sequences. In ZFC, these are different sets. In practice, mathematicians treat them as "the same real numbers." This is the identity problem from Chapter 1 appearing in a concrete mathematical context. The Univalence Axiom will eventually resolve it.

## The Road Through This Chapter

We begin with **metric spaces** — the right generalization of "distance." A metric space is a set equipped with a distance function satisfying three natural axioms. This single definition unifies examples as different as $\mathbb{R}^n$, function spaces, graph theory, and combinatorics.

From distance, we extract the notion of **open sets**, and from open sets, the notions of **convergence** and **continuity**. A sequence converges if its terms eventually fall inside every open ball around the limit. A function is continuous if the preimage of every open set is open. These definitions turn out to be equivalent to the $\varepsilon$-$\delta$ versions you learned in calculus, but they're better: they generalize, they compose cleanly, and they make the structure of arguments transparent.

**Completeness** captures the idea that a metric space has "no gaps." The real numbers are complete; the rationals are not. Cauchy sequences are sequences where the terms become arbitrarily close to each other — and completeness is the property that every Cauchy sequence converges. We prove the completion theorem: every metric space embeds into a complete metric space in a canonical way.

**Compactness** is one of the deepest concepts in analysis. Compact spaces are those where every open cover has a finite subcover — a technical condition that captures the informal sense of being "finite-like." The Heine-Borel theorem characterizes compact subsets of $\mathbb{R}^n$ as those that are closed and bounded. Compact spaces enjoy extraordinary properties: continuous functions on compact spaces are uniformly continuous, attain their maxima, and have compact images.

**Connectedness** asks whether a space can be split into two disjoint open pieces. The real line is connected, and so are intervals. This is the analytic content behind the Intermediate Value Theorem: a continuous function on a connected space cannot jump from one value to another without hitting every value in between.

We then construct **the real numbers** carefully, and discuss the relationship between the two standard constructions. The key theorem is that both constructions yield a *complete ordered field*, and any two complete ordered fields are isomorphic. This is the uniqueness up to isomorphism that the Univalence Axiom will later make into literal equality.

Finally, we preview **paths and homotopy** from an analytic perspective, setting up the conceptual bridge to the later chapters on topology and type theory.

## What You Should Know Coming In

This chapter assumes you're comfortable with:
- The material from Chapter 0 (logic, proof techniques, induction)
- The material from Chapter 1 (sets, functions, cardinality)
- Basic familiarity with limits and continuity from calculus (we'll make these rigorous, but the intuition helps)

Chapter 2 (abstract algebra) is not a strict prerequisite, though the examples will sometimes reference groups and rings.

## A Note on Style

Analysis proofs have a reputation for being technically dense — a wall of $\varepsilon$'s and $\delta$'s and inequalities. This reputation is somewhat deserved, but it's also somewhat misleading. The *ideas* behind analysis proofs are usually quite geometric and intuitive. The $\varepsilon$-$\delta$ machinery is the language we use to make geometric intuitions precise.

In this chapter, we'll try to separate the idea from the machinery. For each major theorem, we'll explain what the proof is doing geometrically before diving into the formal details. If you find yourself lost in the inequalities, step back and ask: what does this say about distances? About open balls? About what's happening geometrically?

That geometric instinct will serve you well when we move to topology, where the distance function goes away and we're left with only the open sets.
