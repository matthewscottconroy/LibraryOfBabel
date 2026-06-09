# Chapter 13 — Exercises

## Important Figures

- **Évariste Galois (1811–1832)** — introduced the notion of a "group" (1831) while studying symmetries of polynomial roots; the concept was hidden in his work but extracted and named posthumously
- **Niels Henrik Abel (1802–1829)** — proved the insolubility of the general quintic; his name attached to commutative (abelian) groups
- **Arthur Cayley (1821–1895)** — first abstract definition of a group as a set with an associative binary operation (1854); Cayley's theorem (every group embeds in a symmetric group)
- **Felix Klein (1849–1925)** — Erlangen Program (1872): geometry as the study of invariants under a group of transformations; unified all geometries via group theory

## References and Primary Sources

- **A. Cayley, "On the Theory of Groups, as Depending on the Symbolic Equation $\theta^n = 1$" (1854)** — first abstract group definition
- **J.J. Rotman, *An Introduction to the Theory of Groups* (4th ed., Springer, 1995)** — comprehensive and readable
- **M. Artin, *Algebra* (2nd ed., Pearson, 2011)** — groups motivated through linear algebra and symmetry; excellent first text
- **D. Dummit & R. Foote, *Abstract Algebra* (3rd ed., Wiley, 2004)** — standard graduate reference

## Examples, Applications, and Thought Experiments

- **The symmetry group of an equilateral triangle** — $D_3$: 3 rotations ($0°, 120°, 240°$) and 3 reflections; 6 elements total; all four group axioms verified concretely; non-abelian (rotation then reflection $\neq$ reflection then rotation)
- **$(\mathbb{Z}, +)$ and its subgroups** — an infinite cyclic group; every subgroup is $n\mathbb{Z} = \{nk : k \in \mathbb{Z}\}$ for some $n \geq 0$; the subgroups are linearly ordered by inclusion; this is the simplest infinite group and the template for cyclic group theory
- **$\text{GL}_2(\mathbb{R})$ — the general linear group** — invertible $2 \times 2$ real matrices under multiplication; non-abelian; contains $\text{SL}_2(\mathbb{R})$ (det $= 1$), $O_2(\mathbb{R})$ (orthogonal), and many other subgroups; the richness of this one example motivates the general theory
- **Klein four-group $V_4$** — $\{e, a, b, c\}$ with $a^2 = b^2 = c^2 = e$ and $ab = c$; the symmetry group of a rectangle; every non-identity element has order 2; abelian and of order 4 but not cyclic; appears as a subgroup of $A_4$

## Exercises

1. Verify that the set $\mathbb{Z}/n\mathbb{Z}$ of residue classes modulo $n$ forms a group under addition. Identify the identity element and the inverse of each element. For which values of $n$ is $(\mathbb{Z}/n\mathbb{Z})^*$ (the units under multiplication) also a group? What is its order in terms of $n$?

2. Let $D_4$ denote the dihedral group of symmetries of the square, with rotation $r$ of order 4 and reflection $s$ satisfying $s^2 = e$ and $srs^{-1} = r^{-1}$. List all elements of $D_4$, find their orders, and identify all subgroups. Draw the subgroup lattice.

3. Prove that in any group $G$, the identity element is unique and the inverse of each element is unique. Deduce the cancellation laws: if $ab = ac$ then $b = c$, and if $ba = ca$ then $b = c$.

4. Let $G$ be a group and let $H$ and $K$ be subgroups of $G$. Prove that $H \cap K$ is a subgroup of $G$. Give an example showing that $H \cup K$ need not be a subgroup.

5. Determine all subgroups of $\mathbb{Z}/30\mathbb{Z}$. For each subgroup $H$, identify a generator and state $|H|$. Verify that the subgroup lattice of $\mathbb{Z}/30\mathbb{Z}$ is isomorphic to the divisor lattice of 30.

6. Find the order of each element in $S_4$. Show directly that the alternating group $A_4$ is a subgroup of $S_4$ of index 2, and determine the center $Z(A_4)$.

7. Prove that every cyclic group of order $n$ is isomorphic to $\mathbb{Z}/n\mathbb{Z}$, and every infinite cyclic group is isomorphic to $(\mathbb{Z}, +)$. Use this to show that any two cyclic groups of the same order are isomorphic.

8. (Challenge) Let $G$ be a finite group in which every non-identity element has order 2. Prove that $G$ is abelian. Then prove that $|G|$ must be a power of 2, and construct an example showing that for every $n \geq 0$, the group $(\mathbb{Z}/2\mathbb{Z})^n$ is a group of order $2^n$ in which every non-identity element has order 2.
