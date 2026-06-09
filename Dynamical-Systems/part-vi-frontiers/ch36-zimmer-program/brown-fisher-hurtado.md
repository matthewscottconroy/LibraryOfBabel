# 36.5 The Brown-Fisher-Hurtado Resolution

Brown, Fisher, and Hurtado proved the Zimmer conjecture in 2020, after 35 years. The techniques were completely new — no one had combined KAM theory, cocycle superrigidity, and non-stationary normal forms in this way before.

**Theorem 36.5.1 (Brown-Fisher-Hurtado, 2020).** Let $n \geq 3$ and $\Gamma \leq SL(n, {\mathbb R})$ be a cocompact lattice (or $\Gamma = SL(n, {\mathbb Z})$). If $\Gamma \curvearrowright M$ is a $C^\infty$ volume-preserving action on a compact manifold $M$ with $\dim M < n - 1$, then the action factors through a finite group action.

**Corollary 36.5.2 (Zimmer's Conjecture for SL(n)).** $SL(n, {\mathbb Z})$ (and lattices in $SL(n, {\mathbb R})$ for $n \geq 3$) cannot act faithfully by $C^\infty$ volume-preserving diffeomorphisms on any compact manifold of dimension $< n - 1$.

The "factors through a finite group action" means: the action is essentially trivial. Every element of infinite order acts as the identity. For $SL(n, \mathbb{Z})$, which has many elements of infinite order, this means the action is trivial.

**Key Tools in the Proof:**
1. *KAM theory* (Chapter 14): Local linearization of nearly-integrable systems
2. *Cocycle superrigidity* (Theorem 36.3.2): Derivative constraints
3. *Non-stationary normal forms*: Extending KAM to the non-commutative setting
4. *Harmonic analysis on homogeneous spaces*: Spectral estimates for $G/\Gamma$

The non-stationary normal form is the new ingredient. Classical KAM theory linearizes a single map near a fixed point, under a Diophantine condition. Brown-Fisher-Hurtado needed to linearize not one map but an entire group action, simultaneously, at almost every point — and they needed to do it in a way compatible with the group structure.

**Remark 36.5.3.** The proof uses dynamical systems methods (KAM, Lyapunov exponents) in an essential way — it is not purely algebraic. The interplay between dynamics and group theory is the heart of the Zimmer program.

The version for $SL(n, \mathbb{Z})$ (non-cocompact) required additional work, completed in subsequent papers. The case of symplectic lattices — lattices in $Sp(2n, \mathbb{R})$ — and other Lie group families are also resolved in various cases by Brown-Fisher-Hurtado and their collaborators. But the full conjecture for all higher-rank lattices remains open in some cases.
