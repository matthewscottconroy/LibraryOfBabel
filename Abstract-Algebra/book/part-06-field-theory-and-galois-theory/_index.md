# Part VI — Field Theory and Galois Theory

**Chapters 29–32**

* * *

There is a polynomial — $x^5 - x - 1$, say, or any "generic" degree-five polynomial with rational coefficients — that no formula built from addition, subtraction, multiplication, division, and the extraction of roots can solve. This is not a matter of insufficient cleverness or insufficient time. It is a theorem, and it is one of the most startling facts in all of mathematics: the operations available in a pocket calculator, iterated in any order any finite number of times, are simply not enough to express the roots of a generic quintic. The proof of this fact — in full generality, for all polynomial equations of degree five or higher — is one of the two great achievements of Part VI, and its strategy is unlike any impossibility argument that came before it. The proof does not exhibit a particular quintic, try to find a formula, and show that none works. Instead, it associates to each polynomial equation a group — its Galois group, the collection of symmetries of the roots — and shows that the existence of a solution by radicals is equivalent to a structural property of that group called solvability. The group of the general quintic, being $S_5$, is not solvable; and so the formula cannot exist.

The other great achievement of Part VI is the Galois correspondence itself: a perfect dictionary between the intermediate fields of a Galois extension and the subgroups of its Galois group, reversing inclusions and converting the question "how are the fields arranged?" into the question "how are the subgroups arranged?" — the latter being a question that group theory (Part III) answers completely. This correspondence is the bridge between field theory and group theory, and it is not an accident or a technical coincidence; it is the deepest structural result about field extensions, making the abstract symmetry of a Galois group concretely visible as the geometry of an intermediate field lattice.

Part VI develops the algebra of field extensions in four chapters. Chapter 29 introduces the basic vocabulary: algebraic versus transcendental elements, the degree $[E:F]$ of an extension, simple extensions obtained by adjoining a root of an irreducible polynomial, and the existence and uniqueness of algebraic closures — the "universal" field extension in which every polynomial splits. Chapter 30 identifies the two properties a Galois extension must have: normality (the extension is a splitting field over $F$, so that every root of every irreducible polynomial that has one root in $E$ has all its roots in $E$) and separability (the minimal polynomial of every element has no repeated roots, which fails only in a controlled way in characteristic $p$). Chapter 31 proves the fundamental theorem of Galois theory: for a finite Galois extension $E/F$, the intermediate fields and the subgroups of $\operatorname{Gal}(E/F)$ are in bijection, with the Galois correspondence reversing inclusions and converting normal subgroups to Galois sub-extensions. Chapter 32 harvests the applications: Galois's original criterion that a polynomial is solvable by radicals if and only if its Galois group is a solvable group; the resolution of the three classical Greek impossibility problems (doubling the cube, trisecting an angle, squaring the circle via constructible numbers); and the complete and beautiful classification of finite fields — there is exactly one field of order $p^n$ for each prime $p$ and positive integer $n$, unique up to isomorphism. By the end of Part VI, the reader has seen groups and fields interlock in one of the most profound correspondences in mathematics, and has glimpsed in the Galois group the precursor to the Galois representations at the heart of the Langlands program.

* * *

## Internal Dependency Map

```
Ch 29 (Field Extensions: degree, algebraic, algebraic closure)
                     |
                     v
Ch 30 (Normal, Separable, Primitive Element Theorem)
                     |
                     v
Ch 31 (Galois Theory: FTGT)
                     |
              _______|_______
              |             |
              v             v
         Ch 32a          Ch 32b/c
   (Solvability by     (Constructions,
     Radicals)         Finite Fields)
```

* * *
