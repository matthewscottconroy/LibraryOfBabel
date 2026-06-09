# 36.2 The Zimmer Program

In the 1980s, Zimmer asked: what smooth actions can a lattice in a higher-rank Lie group have? The linear actions on tori are "algebraic" — they come from the ambient Lie group's action on symmetric spaces. Are there other smooth actions?

The Zimmer program investigates this question systematically. The central conjecture gives a sharp bound on the minimal dimension of any faithful smooth volume-preserving action.

**Definition 36.2.1 (Zimmer, 1980s).** The *Zimmer program* investigates smooth actions of lattices $\Gamma$ in semisimple Lie groups on compact manifolds. The central question:

*If $\Gamma \leq SL(n, {\mathbb R})$ acts smoothly on a compact manifold $M$, what is the minimal dimension of $M$?*

**Zimmer's Conjecture (strong form).** If $\Gamma$ is an irreducible lattice in a semisimple Lie group $G$ of real rank $r$, and $\Gamma \curvearrowright M$ is a smooth volume-preserving action on a compact manifold $M$, then:
$$\dim M \geq r = \text{rank}_{\mathbb R}(G).$$

For $\Gamma = SL(n, {\mathbb Z})$ (rank $n-1$): any volume-preserving action on a compact manifold $M$ has $\dim M \geq n - 1$.

The example that shows this bound is sharp: $SL(n, \mathbb{Z})$ acts on $\mathbb{T}^{n-1}$ by the standard linear action $A \cdot x = Ax \pmod{\mathbb{Z}^{n-1}}$. This is volume-preserving (since $\det A = 1$) and faithful. So dimension $n-1$ is achievable.

Zimmer's conjecture says you can't go lower. If you want $SL(n, \mathbb{Z})$ to act faithfully and smoothly and volume-preservingly on a compact manifold, that manifold must have dimension at least $n-1$.

The phrase "volume-preserving" is essential. Without it, the conjecture fails — there are actions on lower-dimensional manifolds that are not volume-preserving. The volume-preserving condition is what lets the ergodic theory (KS entropy, cocycle superrigidity) come in.
