# 3.4 Covering Spaces

A covering space is a space that "wraps around" a base space, with each sheet looking locally identical to the base. The prototypical example: the real line $\mathbb{R}$ covers the circle $S^1$ via the map $t \mapsto e^{2\pi it}$. The integer translates $t \mapsto t + n$ are deck transformations that permute the sheets.

**Definition 3.4.1.** A *covering space* of $X$ is a space $\tilde{X}$ with a continuous surjection $p: \tilde{X} \to X$ such that every $x \in X$ has an open neighborhood $U$ with $p^{-1}(U) = \bigsqcup_\alpha V_\alpha$ where each $V_\alpha$ is mapped homeomorphically onto $U$ by $p$.

The neighborhoods $U$ where the covering is "trivial" — where $p^{-1}(U)$ is a disjoint union of homeomorphic copies — are called *evenly covered* neighborhoods. The preimage $p^{-1}(x)$ is the *fiber* over $x$, and for connected covering spaces it has constant cardinality (the *degree* of the covering).

The fundamental theorem of covering space theory classifies all connected covering spaces in terms of the fundamental group:

**Theorem 3.4.2 (Classification of Covering Spaces).** For a connected, locally path-connected, semi-locally simply connected space $X$ and basepoint $x_0$:
- There is a bijection between (isomorphism classes of) connected covering spaces and (conjugacy classes of) subgroups of $\pi_1(X, x_0)$.
- The *universal cover* $\tilde{X}$ corresponds to the trivial subgroup and satisfies $\pi_1(\tilde{X}) = \{e\}$.

What this is really saying: covering spaces are controlled by the fundamental group. The bigger the subgroup, the "smaller" the covering (the universal cover is the "biggest" one, corresponding to the trivial subgroup). If you know $\pi_1(X)$, you know all possible covering spaces of $X$.

For example: $\pi_1(S^1) \cong \mathbb{Z}$. The subgroups are $n\mathbb{Z}$ for each $n \geq 1$ and the trivial group. These correspond to: the $n$-sheeted cyclic covering $S^1 \to S^1$ (multiplication by $n$ on angles), and the universal cover $\mathbb{R} \to S^1$.

**Application in Dynamics.** Covering space theory is used to study the *lifting* of dynamical systems. A map $f: X \to X$ lifts to a map $\tilde{f}: \tilde{X} \to \tilde{X}$ on the universal cover. For $X = S^1$ (the circle), the universal cover is $\mathbb{R}$, and circle maps lift to maps of the real line. The rotation number of a circle homeomorphism is defined in terms of the behavior of this lift: it measures how much $\tilde{f}$ shifts the real line on average. This algebraic data (a real number, rational or irrational) encodes the key dynamical distinction between periodic and dense orbits.

Covering spaces also appear in the study of geodesic flows on surfaces and in the theory of surface diffeomorphisms, where the action of $f$ on $\pi_1$ carries topological information about the orbit structure.
