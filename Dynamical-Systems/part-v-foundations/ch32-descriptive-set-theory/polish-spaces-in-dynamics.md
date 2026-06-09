# 32.2 Polish Spaces in Dynamics

Descriptive set theory lives in Polish spaces — completely metrizable, separable topological spaces. The real line is Polish. The Cantor set is Polish. The Hilbert cube $[0,1]^{\mathbb{N}}$ is Polish. And, crucially for us, the spaces of dynamical systems are Polish.

This is not obvious. The "space of all measure-preserving transformations" or the "space of all homeomorphisms of a compact space" sounds like a wild, infinite-dimensional object. But with the right topology — the weak topology for measure-preserving transformations, the sup norm for homeomorphisms — these spaces are Polish, and descriptive set theory becomes available.

**Theorem 32.2.1 (The Space of Dynamical Systems is Polish).** The following spaces are Polish:
- $\text{MPT}([0,1], \text{Lebesgue})$: measure-preserving transformations, with the weak topology
- $\text{Homeo}(K)$: homeomorphisms of a compact metric space $K$, with the $\sup$ topology
- $\text{Aut}(X, \mu)$: measure-preserving automorphisms, with the weak operator topology

This means: the space of all dynamical systems is itself a dynamical system (acted on by conjugacy).

The last line in this theorem is the key recursive observation. The conjugacy action of $\text{Aut}(X, \mu)$ on itself — by conjugation: $\phi \cdot T = \phi T \phi^{-1}$ — is a continuous action of a Polish group on a Polish space. The orbits of this action are exactly the isomorphism classes of measure-preserving transformations. This is the framework in which the classification problem lives.

**Theorem 32.2.2 (Generic Dynamical Properties).** In $\text{MPT}([0,1])$ (the space of all measure-preserving transformations with the weak topology), the generic (comeager) transformation:
1. Is ergodic (ergodic MPTs form a dense $G_\delta$)
2. Is weakly mixing but not strongly mixing (strongly mixing is meager)
3. Has zero entropy (positive entropy MPTs are meager)

**Remark 32.2.3.** "Generic" in Baire category sense $\neq$ "almost all" in measure-theoretic sense. The generic transformation is weakly mixing, but most natural examples (Bernoulli shifts, rotations) are in the non-generic classes.

This is one of the subtlest points in all of ergodic theory. In the Baire category sense, the "typical" measure-preserving transformation is weakly mixing, zero entropy, and in particular not isomorphic to any Bernoulli shift. But in every explicit example we care about — Bernoulli shifts, rotations, Anosov diffeomorphisms, Gibbs states — the system has special properties that make it non-generic in the category sense.

The Baire category theorem says that comeager sets are "large" in the topological sense. But they can have measure zero in any reasonable measure on the space of transformations. Topology and measure theory can disagree dramatically on which sets are "typical," and ergodic theory provides some of the most striking examples of this disagreement.

The practical lesson: generic properties in $\text{MPT}([0,1])$ tell you what a "random" measure-preserving transformation looks like if you choose it by some topologically natural process. They don't tell you what the "interesting" dynamical systems look like — those are exceptional, non-generic objects.
