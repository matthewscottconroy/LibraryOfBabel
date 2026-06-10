# Chapter 12: Conceptors

## Jaeger's Most Ambitious Extension

Every framework has a moment of ambition when its creator reaches beyond the original paradigm to ask: how far can this really go? For Herbert Jaeger, that moment produced *conceptors* [Jaeger2014].

The standard reservoir computing framework answers the question: given a fixed reservoir and a sequence of inputs, can we learn a linear mapping from reservoir states to outputs? The answer is yes, and it is computationally efficient and theoretically well-understood. But Jaeger wanted more. He wanted to ask: can a reservoir *remember* multiple distinct patterns, *recall* them on demand, and *compose* them — interpolating or combining patterns in semantically meaningful ways?

This sounds like a memory system, and in some sense it is. But it is a memory system with a mathematical structure that goes far beyond simple storage and retrieval. Conceptors are matrices that encode, in a geometric sense, the *subspace of reservoir state space* that was active when a particular pattern was driven through the reservoir. They can be computed, stored, combined using Boolean-like operations (AND, OR, NOT), and used to constrain the reservoir to reproduce the corresponding pattern.

The resulting system is a fixed reservoir that can:
1. **Store** multiple patterns by associating each pattern with a conceptor matrix.
2. **Recall** a stored pattern by applying its conceptor to the reservoir dynamics.
3. **Interpolate** between patterns by interpolating their conceptors.
4. **Compose** patterns using Boolean operations on their conceptors.

## Why Conceptors Are Mathematically Deep

The mathematical core of conceptors is deceptively simple: a conceptor for a pattern is a matrix $C = R(R + \alpha^{-2}I)^{-1}$, where $R$ is the reservoir state covariance when driven by the pattern and $\alpha$ is an "aperture" parameter. This formula is a regularized projection — a soft projection onto the subspace spanned by the pattern's state activity.

But the structure that emerges from this simple definition is rich. The conceptor matrices form a *bounded distributive lattice* under the Boolean operations NOT ($\lnot$), AND ($\wedge$), and OR ($\vee$) — these operations have precise geometric interpretations and provably preserve the conceptor structure. The lattice structure allows arbitrary logical combinations of patterns, something that static memory systems simply cannot do.

## What This Chapter Covers

**Section 12.2** derives the conceptor definition from the regularized least-squares problem. We show that the conceptor $C$ is the optimal soft projection onto the state subspace of its pattern, interpret it geometrically via the SVD, and explain the role of the aperture $\alpha$.

**Section 12.3** defines the Boolean operations on conceptors, proves they form a bounded lattice, and gives geometric interpretations and examples with two stored patterns.

The exercises ask you to implement conceptors from scratch, explore the aperture parameter, and perform Boolean operations on two learned patterns.

---

*Prerequisites: All previous chapters in Unit 4. The SVD interpretation of conceptors requires comfort with the singular value decomposition as a tool for understanding matrix structure.*
