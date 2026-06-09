# Part IX — Representation Theory of Finite Groups

**Chapters 42–45**

* * *

A group captures the algebra of symmetry in its most abstract and general form. But abstraction has a cost: when a group is given only by generators and relations or by a multiplication table, its internal structure can be extremely difficult to read. Representation theory offers a different approach: realize the group concretely, as a collection of invertible matrices acting on a vector space, and then apply the full power of linear algebra — eigenvalues, traces, inner products, and diagonalization — to the study of its structure. This is the fundamental strategy, and in the case of finite groups over the complex numbers, it succeeds so completely that the entire representation theory can be organized around a single finite table of complex numbers, the character table, from which the group's representation-theoretic structure can be read with perfect completeness.

The theorem that makes this possible is Maschke's theorem: over a field of characteristic not dividing $|G|$, every representation of a finite group decomposes into irreducible ones. This is not true for modules in general; it is a special feature of group algebras and of the averaging technique (summing over all group elements) that Maschke's proof uses. Combined with Schur's lemma — which asserts that the only endomorphisms of an irreducible representation are scalar multiples of the identity — the Artin–Wedderburn theorem becomes accessible: the group algebra $\mathbb{C}[G]$ decomposes as a direct product of matrix algebras $\prod_i M_{n_i}(\mathbb{C})$, one for each irreducible representation, with dimension $n_i$. The counting formula $\sum_i n_i^2 = |G|$ is a consequence, and the fact that the number of irreducible representations equals the number of conjugacy classes — established by the same decomposition — is among the most striking numerological coincidences in algebra (and it is not a coincidence).

The character $\chi_V(g) = \operatorname{tr}(\rho(g))$ of a representation $V$ strips away the basis-dependence of $\rho$ and retains only the trace. The orthogonality relations — both for rows and columns of the character table — make the space of class functions into an inner-product space in which the irreducible characters form an orthonormal basis, turning the decomposition of any representation into a computation of inner products. The character table, a square matrix encoding all irreducible characters, is a complete invariant of the representation theory of $G$, and it can often be determined almost entirely from dimension counts alone. The applications extend far beyond classification: Burnside's theorem that every group of order $p^a q^b$ is solvable — a deep structural fact about finite groups — was proved in 1904 using character theory, decades before any character-free proof was found.

Part IX develops this theory through four chapters. Chapter 42 introduces representations as group homomorphisms into $GL(V)$, equivalently as $k[G]$-modules, and establishes Schur's lemma as the key structural constraint. Chapter 43 proves Maschke's theorem and the Artin–Wedderburn theorem, computes the decomposition of the regular representation, and establishes the fundamental numerical constraints on the character table. Chapter 44 develops character theory in full: the orthogonality relations, the character table, the degree divisibility theorem, Burnside's $p^a q^b$ theorem, and the detection of normal subgroups from the character table. Chapter 45 develops the induction and restriction functors, proves Frobenius reciprocity — the adjunction $\langle \operatorname{Ind}_H^G V, W \rangle_G = \langle V, \operatorname{Res}_H^G W \rangle_H$ — Mackey's theorem on double cosets, and the theory of Frobenius groups. Together, these four chapters give the reader a complete understanding of the representation theory of finite groups in characteristic zero, providing both the methods and the perspective that Parts X and XI will extend into new domains.

* * *

## Internal Dependency Map

```
Ch 42 (Representations, Schur's Lemma)
            |
            v
Ch 43 (Maschke's Theorem, Artin-Wedderburn, Group Algebra)
            |
            v
Ch 44 (Characters, Orthogonality, Character Tables)
            |
            v
Ch 45 (Induced Representations, Frobenius Reciprocity)
```

* * *
