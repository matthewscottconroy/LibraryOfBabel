# 12.7 Conjugacy and Classification

When are two dynamical systems the same? This is the most fundamental classification problem in dynamics, and for SFTs it has a precise algebraic answer — albeit a computationally challenging one.

Two SFTs are *topologically conjugate* if there is a homeomorphism between them that commutes with the shift. Conjugacy is the right notion of "isomorphism" for symbolic dynamical systems. The question is: given two transition matrices $A$ and $B$, can we decide whether $X_A$ and $X_B$ are conjugate?

The answer comes from Williams (1973), via an algebraic relation on matrices called *strong shift equivalence*.

**Definition 12.7.1 (Strong Shift Equivalence).** Matrices $A, B$ over $\mathbb{Z}_{\geq 0}$ are *elementary equivalent* over one step if there exist non-negative integer matrices $R, S$ with $A = RS$ and $B = SR$. They are *strong shift equivalent* if they are connected by a finite sequence of elementary equivalences.

The intuition is: $A = RS$ means we can "split" the graph for $A$ into two parts. $B = SR$ "reassembles" those parts in a different order. This operation changes the matrix but (up to a finite-time lag) preserves the dynamics.

**Theorem 12.7.2 (Williams, 1973).** Two irreducible SFTs $X_A$ and $X_B$ are topologically conjugate if and only if $A$ and $B$ are strong shift equivalent.

What this is saying is: Williams' theorem gives a complete algebraic characterization of conjugacy. If you want to know whether two SFTs are the same dynamical system, you check whether their transition matrices are strong shift equivalent.

But the computational problem is subtle. Strong shift equivalence involves a sequence of elementary steps, and there is no obvious bound on the number of steps needed.

**Corollary 12.7.3 (Kim-Roush).** Strong shift equivalence over $\{0,1\}$ matrices is decidable. Over $\mathbb{Z}$-matrices, it is *undecidable* (Kim-Roush, 1992).

The undecidability over $\mathbb{Z}$ is related to the undecidability of the word problem for groups — there is a deep connection between symbolic dynamics and combinatorial group theory that Chapter 27 will explore.

For a slightly weaker notion — *flow equivalence* (homeomorphism of the suspension flows rather than the shift maps themselves) — the answer is cleaner:

**Theorem 12.7.4 (Williams, 1973).** Two irreducible SFTs are flow equivalent if and only if they have the same *dimension group* — a certain abelian group constructed from the matrix $A$ and invariant under strong shift equivalence.

The dimension group is computable and provides a complete flow equivalence invariant. For conjugacy, one needs stronger invariants, and the full theory involves the *automorphism group* (Section 12.8) and more refined algebraic K-theory tools.

The classification problem for SFTs is one of the central open problems in symbolic dynamics: finding a complete set of computable conjugacy invariants, or showing that no such set exists.
