# 36.4 The Lyapunov Spectrum and Volume

Lyapunov exponents measure the asymptotic rate of expansion and contraction in a dynamical system. For a smooth group action on a manifold, the Lyapunov exponents of the derivative cocycle encode how the tangent space behaves under the dynamics.

**Theorem 36.4.1 (Zimmer's Inequality).** For a smooth volume-preserving action $\Gamma \curvearrowright M$ with $\Gamma \leq SL(n, {\mathbb R})$ a lattice, the Lyapunov exponents of the action satisfy constraints from the representation theory of $G = SL(n, {\mathbb R})$.

Specifically, the Lyapunov spectrum of the derivative cocycle must "come from" a representation of $G$, so the possible Lyapunov exponents are the weights of a $GL(\dim M, {\mathbb R})$-representation of $G$.

The weights of a representation are determined by the representation theory of the Lie algebra. For $\mathfrak{sl}(n, \mathbb{R})$, the possible weights are well understood — they're constrained by the root system. And the minimal faithful representation has dimension $n - 1$: it's the standard representation $\mathbb{R}^{n-1}$ (or rather, the quotient of the standard $\mathbb{R}^n$ representation by the center, restricted to the Lie algebra level).

**The Dimension Bound:** The minimal faithful representation of $SL(n, {\mathbb R})$ has dimension $n-1$ (the standard representation on ${\mathbb R}^n$ restricted to the Lie algebra). If the derivative cocycle comes from a representation of $G$, then $\dim M \geq n - 1$.

This is the key dimension bound. If the Lyapunov exponents must be weights of a representation, and the smallest faithful representation has dimension $n-1$, then the manifold carrying those exponents must have at least $n-1$ dimensions.

This argument gives Zimmer's conjecture under the hypothesis that the derivative cocycle is exactly algebraic (i.e., the action is linear on some cover). The difficulty is that real smooth actions are not linear — they can be nonlinear in complicated ways, and the derivative cocycle is only "almost algebraic" after a measurable change of frame. Making the measurable change of frame into a smooth one is the hard part, and this is what Brown-Fisher-Hurtado achieved.
