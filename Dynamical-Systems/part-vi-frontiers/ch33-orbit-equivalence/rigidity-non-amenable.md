# 33.3 Rigidity for Non-Amenable Groups

For amenable groups, orbit equivalence forgets everything about the group. All amenable groups are indistinguishable by their orbit structures. You might expect the same to be true for non-amenable groups — perhaps everything collapses again to some small family of equivalence relations.

The opposite is true. For non-amenable groups, orbit equivalence carries a tremendous amount of information about the group. The rigidity revolution of the 1990s and 2000s showed that for groups like $SL(n, \mathbb{Z})$, you can often recover the group essentially completely from the orbit equivalence relation.

**The Rigidity Revolution:** In the 1990s-2000s, Furman, Gaboriau, Popa, and others showed that non-amenable groups behave very differently — their OE classes carry substantial information about the group.

Furman's 1999 theorem is a landmark:

**Theorem 33.3.1 (Furman, 1999).** For an action of $\Gamma = SL(n, {\mathbb Z})$ on a standard probability space: if $\Lambda \curvearrowright Y$ is orbit equivalent to $\Gamma \curvearrowright X$, then $\Lambda$ is virtually isomorphic to $\Gamma$ (up to finite index).

This is remarkable. $SL(n, \mathbb{Z})$ remembers itself in its orbit structure, up to finite index. No amenable group could act freely and ergodically and produce the same orbit structure — the two systems would have to share an essentially identical group.

Gaboriau's 2000 theorem gave us numerical OE-invariants that distinguish groups from each other:

**Theorem 33.3.2 (Gaboriau, 2000).** The *$\ell^2$-Betti numbers* $\beta_n^{(2)}(\mathcal{R})$ of the orbit equivalence relation $\mathcal{R}$ are OE-invariants. For a free ergodic action $\Gamma \curvearrowright X$: $\beta_n^{(2)}(\mathcal{R}) = \beta_n^{(2)}(\Gamma)$ (the $\ell^2$-Betti numbers of the group).

The $\ell^2$-Betti numbers are group-theoretic invariants computed from the $L^2$-cohomology of the group. Gaboriau showed that these invariants survive the passage from the group to its orbit structure. That means if two actions of different groups are orbit equivalent, their groups must have the same $\ell^2$-Betti numbers.

**Corollary 33.3.3.** Free groups $F_r$ and $F_s$ (with $r \neq s$) have non-orbit-equivalent free ergodic actions, since $\beta_1^{(2)}(F_r) = r - 1 \neq s - 1 = \beta_1^{(2)}(F_s)$.

This is the first proof that the free groups $F_2$ and $F_3$ (for example) have genuinely different orbit structures. The orbit equivalence class of a free group action knows how many generators the group has.
