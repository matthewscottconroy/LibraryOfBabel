# Part III — Group Theory

**Chapters 13–19**

* * *

In 1831, a twenty-year-old Frenchman named Évariste Galois, on the night before a duel in which he would be killed, wrote out the mathematics that would reshape algebra for the next two centuries. He was trying to understand why the quadratic, cubic, and quartic equations had formulas solvable by radicals — expressions involving the coefficients built up by addition, subtraction, multiplication, division, and extraction of roots — while no such formula could exist for the general polynomial of degree five or higher. His answer was not a formula but a structure: each polynomial equation carries an associated collection of symmetries acting on its roots, and the equation is solvable by radicals if and only if those symmetries have a specific internal property that we now call solvability. Galois was working with the right objects before anyone had given them a name. Arthur Cayley named them groups in 1854, and the abstract theory was born. What began as a tool for understanding polynomial equations grew into the algebra of symmetry in its most universal form — the framework in which the symmetries of a crystal, the fundamental forces of physics, the automorphisms of a field extension, and the symmetries of a Riemannian manifold all live together.

Part III introduces the most fundamental algebraic structure: the group. A group is a set equipped with a single binary operation satisfying four axioms — closure, associativity, identity, and inverses. The deceptive simplicity of these axioms conceals extraordinary depth: groups encode the algebra of symmetry in its purest form, and the theory built from these four axioms spans from elementary consequences to one of the great achievements of twentieth-century mathematics, the classification of finite simple groups.

The part moves through several layers of structure. The first layer (Chapters 13–14) establishes what groups are and how a subgroup partitions its parent group into cosets, culminating in Lagrange's theorem — that the order of any subgroup divides the order of the group — a result whose proof is a single observation and whose consequences are enormous. The second layer (Chapter 15) establishes the maps between groups and the isomorphism theorems, the tools that let us recognize when two groups are "the same" even when they look different and that make the quotient construction algebraically tractable. The third layer (Chapter 16) introduces group actions: groups acting on sets, which unifies abstract group theory with combinatorics, provides the machinery for counting orbits, and establishes the orbit-stabilizer theorem as a common generalization of Lagrange's theorem and the class equation. The fourth layer (Chapters 17–18) goes deeper into structure: the Sylow theorems give precise control over prime-power subgroups of finite groups, and the study of simple, solvable, and nilpotent groups reveals the hierarchical manner in which groups are assembled from indecomposable pieces — the Jordan–Hölder theorem, the Schur–Zassenhaus theorem, the free group and its presentations. The part concludes (Chapter 19) with the complete classification of finitely generated abelian groups — a theorem clean enough to state in one sentence and deep enough to require the full machinery of Smith normal form.

By the end of Part III, the reader will be fluent in the language of group theory and ready to encounter groups again in every subsequent part: as coefficient objects in homological algebra, as Galois groups in field theory, as Lie groups in Part X, and as the organizing structure of representation theory in Parts IX and XI.

* * *

## Internal Dependency Map

```
Ch 13 (Groups, Subgroups)
       |
       v
Ch 14 (Cosets, Normal Subgroups, Quotients)
       |
       v
Ch 15 (Homomorphisms, Isomorphism Theorems)
       |
  _____|_____
  |         |
  v         v
Ch 16     Ch 19
(Actions) (Fin. Gen. Abelian)
  |
  v
Ch 17 (Sylow Theorems)
  |
  v
Ch 18 (Structure: Simple, Solvable, Nilpotent, Free)
```

* * *
