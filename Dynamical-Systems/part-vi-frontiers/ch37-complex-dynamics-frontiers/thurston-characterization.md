# 37.4 Thurston's Topological Characterization

One of the deepest theorems in complex dynamics — proved by Douady and Hubbard but due to Thurston — bridges topology and complex analysis. The question is: when is a topological map of the sphere actually a rational function?

A Thurston map is a branched self-cover of the sphere — like a rational function, but only required to be a continuous branched cover, not holomorphic. The question is: when does such a topological map come from a holomorphic rational function?

**Definition 37.4.1.** A *Thurston map* is an orientation-preserving branched self-cover $f: S^2 \to S^2$ of finite degree with $|\text{PostCrit}(f)| < \infty$ (finite postcritical set).

The postcritical set is the set of all forward iterates of the critical points. Requiring it to be finite is analogous to a "post-critically finite" polynomial.

**Definition 37.4.2 (Thurston Obstruction).** A *Thurston obstruction* is a multicurve $\Gamma = \{\gamma_1, \ldots, \gamma_k\}$ (simple closed curves) invariant under the action of $f^{-1}$ on free homotopy classes, with Thurston matrix $A_\Gamma$ having leading eigenvalue $\lambda(A_\Gamma) \geq 1$.

The Thurston matrix $A_\Gamma$ encodes how the map $f$ acts on the multicurve $\Gamma$: the $(i,j)$ entry counts how many preimages of $\gamma_j$ are homotopic to $\gamma_i$, weighted by degree. If the leading eigenvalue $\geq 1$, the multicurve is "expanding" under the dynamics, and this blocks the realization as a rational function.

**Theorem 37.4.3 (Thurston Rigidity, 1982; proved by Douady-Hubbard).** A Thurston map $f$ is (homotopy-equivalent to) a rational map iff it has no Thurston obstruction. The rational map, if it exists, is unique up to Möbius conjugacy.

**Consequence:** Thurston's theorem reduces the question "is this branched cover realizable as a complex polynomial?" to a combinatorial question about multicurves. This is the bridge between combinatorial topology and complex analysis.

The theorem is "Thurston rigidity" because it says: the rational map, if it exists, is unique. There's no room for deformation — the complex structure is pinned by the topological type of the branched cover.

This theorem is used constantly in the modern theory of complex dynamics. Proving that a combinatorially defined map is realized by a polynomial, or that two polynomials are conformally conjugate, often reduces to checking for Thurston obstructions.
