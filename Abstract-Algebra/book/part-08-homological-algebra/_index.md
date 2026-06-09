# Part VIII — Homological Algebra

**Chapters 38–41**

* * *

Homological algebra arose from a simple and beautiful observation in algebraic topology: the boundary of a boundary is always zero. If one decomposes a topological space into simplices — triangles, tetrahedra, and their higher-dimensional analogues — one obtains a sequence of abelian groups recording the chains in each dimension, and the boundary maps connecting them satisfy $\partial \circ \partial = 0$. The quotient group "cycles modulo boundaries" is a topological invariant, unchanged by continuous deformations of the space. When Noether recognized in 1925 that this quotient was a group in its own right — not merely a set of equivalence classes — homological algebra began. What remains, once the topology is stripped away, is a purely algebraic structure: a sequence of abelian groups (or modules) connected by maps whose squares are zero. This structure — the chain complex — is the fundamental object of Part VIII, and its study turns out to be relevant far beyond topology, organizing questions in algebra, geometry, number theory, and representation theory.

The central problem homological algebra solves is the quantification of failure. Many functors of fundamental importance — $\operatorname{Hom}(M, -)$, $\operatorname{Hom}(-, M)$, $- \otimes_R N$, and the global sections functor in algebraic geometry — are not exact: they fail to carry short exact sequences to short exact sequences, losing information in a controlled way. The derived functors $\operatorname{Ext}$ and $\operatorname{Tor}$ measure precisely where and how much this failure occurs. The derived functor $\operatorname{Ext}^1(M, N)$ classifies extensions of $M$ by $N$ — short exact sequences $0 \to N \to L \to M \to 0$ up to isomorphism — recovering the extension theory of abelian groups and central extensions of groups as special cases. $\operatorname{Tor}_1(M, N)$ measures the failure of $M$ to be flat — the degree to which tensoring with $M$ loses exactness. These are not technical artifacts; they are canonical invariants that carry deep information about the modules and the ring, information invisible without this machinery.

Part VIII develops homological algebra in four chapters. Chapter 38 introduces chain complexes, their homology and cohomology groups, and chain homotopy — the relation between chain maps that induce identical maps on homology, the algebraic analogue of homotopy between continuous maps. Chapter 39 develops the art of resolutions: approximating any module by a complex of projective or injective modules, making derived functors computable and establishing the horseshoe lemma and the comparison theorem. Chapter 40 constructs the derived functors $\operatorname{Ext}$ and $\operatorname{Tor}$ via projective and injective resolutions respectively, establishes the long exact sequences they generate from any short exact sequence of modules, proves their independence of the choice of resolution, and develops group cohomology — the case $R = k[G]$ — as a major application, recovering the extension problem for groups from $H^2$, connecting to Galois cohomology, and foreshadowing the arithmetic applications of Parts IX–XI. Chapter 41 develops spectral sequences, the iterated homological machine invented by Jean Leray while imprisoned during World War II in pursuit of a method to compute the cohomology of a fibration from the cohomology of its base and fiber. The formal structure — a bigraded collection of modules $\{E_r^{p,q}\}$ with differentials $d_r$ satisfying $d_r^2 = 0$ and the relation $E_{r+1} = H(E_r, d_r)$ — converges, under appropriate finiteness conditions, to a target that direct computation would not reach. The Lyndon–Hochschild–Serre spectral sequence, computing the cohomology of a group extension from the cohomologies of normal subgroup and quotient, is the central algebraic example. By the end of Part VIII, the reader can construct resolutions, compute derived functors, handle long exact sequences with facility, and read spectral sequences — tools indispensable for the representation theory of Parts IX–XI.

* * *

## Internal Dependency Map

```
Ch 38 (Chain Complexes, Homology, Homotopy)
              |
              v
        Ch 39 (Resolutions)
              |
              v
        Ch 40 (Ext, Tor, Group Cohomology)
              |
              v
        Ch 41 (Spectral Sequences)
```

* * *
