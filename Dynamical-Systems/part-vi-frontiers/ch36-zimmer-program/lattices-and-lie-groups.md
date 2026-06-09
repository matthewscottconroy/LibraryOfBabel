# 36.1 Background: Lattices and Lie Groups

To understand the Zimmer conjecture, you need to understand what a lattice is and why higher-rank lattices are special.

A lattice in a Lie group $G$ is a discrete subgroup whose quotient space has finite volume — like how $\mathbb{Z}$ sits inside $\mathbb{R}$, but the quotient is the circle, which has finite measure. The prototype is $SL(n, \mathbb{Z})$ inside $SL(n, \mathbb{R})$.

**Definition 36.1.1.** Let $G$ be a connected semisimple Lie group. A *lattice* $\Gamma \leq G$ is a discrete subgroup with $G/\Gamma$ of finite volume. Examples:
- $SL(n, {\mathbb Z}) \leq SL(n, {\mathbb R})$ (the standard arithmetic lattice)
- $\pi_1(M) \leq G$ for compact hyperbolic manifold $M$ (cocompact lattice)

Lattices in $SL(2, \mathbb{R})$ are relatively free — they're fundamental groups of hyperbolic surfaces, and they can act in many ways. But lattices in $SL(n, \mathbb{R})$ for $n \geq 3$ are extremely rigid, and this rigidity is quantified by Margulis's superrigidity theorem.

**Theorem 36.1.2 (Margulis Superrigidity, 1974).** Let $G, H$ be semisimple Lie groups of real rank $\geq 2$ and $\Gamma \leq G$ a lattice. Every homomorphism $\phi: \Gamma \to H$ with Zariski-dense image extends to a Lie group homomorphism $\Phi: G \to H$.

**Interpretation:** Lattices in higher-rank groups are "rigid" — they cannot have unexpected representations. Their actions are all "algebraic" in origin.

What Margulis proved is remarkable: if you have a homomorphism from a lattice in $SL(n, \mathbb{R})$ (for $n \geq 3$) into another Lie group, it extends to a homomorphism from the whole Lie group. The lattice "knows" the whole group.

This is the superrigidity principle: higher-rank lattices are so large, so interconnected, that any representation must come from the ambient Lie group. They can't act in unexpected ways. This is what makes Zimmer's conjecture plausible — and what makes the dynamics constrained.

The *real rank* of $SL(n, \mathbb{R})$ is $n-1$: it's the dimension of the maximal split torus, which is the group of diagonal matrices with positive entries and determinant 1. Higher rank means $n \geq 3$, or equivalently real rank $\geq 2$.
