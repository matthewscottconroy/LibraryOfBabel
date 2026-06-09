# Chapter 16 — Exercises

## Important Figures

- **Augustin-Louis Cauchy (1789–1857)** — cycle decomposition of permutations; early orbit theory for permutation groups
- **Peter Ludwig Sylow (1832–1918)** — the Sylow theorems are proved via group actions on coset spaces
- **William Burnside (1852–1927)** — Burnside's lemma (the orbit-counting theorem): number of orbits $= \frac{1}{|G|}\sum_{g \in G}|X^g|$; *Theory of Groups of Finite Order* (1897, 1911)
- **George Pólya (1887–1985)** — Pólya enumeration theorem (1937): a powerful generalization of Burnside's lemma using generating functions

## References and Primary Sources

- **W. Burnside, *Theory of Groups of Finite Order* (2nd ed., Cambridge, 1911)** — the first major English-language group theory text; orbit-counting theorem
- **G. Pólya, "Kombinatorische Anzahlbestimmungen für Gruppen, Graphen und chemische Verbindungen" (1937)** — Pólya enumeration
- **M. Isaacs, *Finite Group Theory* (AMS, 2008)** — modern treatment emphasizing actions

## Examples, Applications, and Thought Experiments

- **Cosets as orbits** — $G$ acts on $G/H$ by left multiplication; orbits $=$ cosets; all orbits have the same size $|H|$; Lagrange's theorem $|G| = |H| \cdot [G:H]$ is immediate from the orbit-stabilizer theorem
- **Burnside's lemma for necklaces** — count colorings of a necklace with 4 beads in 3 colors up to rotation; $G = \mathbb{Z}/4\mathbb{Z}$ acts; apply $|X/G| = \frac{1}{4}\sum|X^g|$; the rotations by $0°, 90°, 180°, 270°$ fix $81, 3, 9, 3$ colorings respectively; result: $(81+3+9+3)/4 = 24$
- **The class equation** — $|G| = |Z(G)| + \sum_{i} [G : C_G(g_i)]$ (sum over non-central conjugacy classes); for $G = S_4$: center $= \{e\}$, conjugacy classes of sizes 1, 6, 3, 8, 6; $1+6+3+8+6 = 24 = |S_4|$; the class equation governs the structure of finite groups
- **Conjugation action** — $G$ acts on itself by conjugation; fixed points are the center $Z(G)$; orbits are conjugacy classes; stabilizers are centralizers; the orbit-stabilizer theorem here gives $|G| / |C_G(g)| =$ size of conjugacy class of $g$

## Exercises

1. Define a left action of $G$ on $G$ by conjugation: $g \cdot x = gxg^{-1}$. Verify the two action axioms. Identify the orbits, the stabilizer of an element $x$, and the fixed-point set. State the class equation of $G$ in terms of this action and verify it for $G = D_4$.

2. Let $G$ act on the set $G/H$ of left cosets by left multiplication: $g \cdot (aH) = (ga)H$. Show this is a well-defined action. Find the stabilizer of the coset $H$, and use the Orbit-Stabilizer Theorem to recover Lagrange's theorem.

3. Use the Orbit-Stabilizer Theorem to count the number of distinct ways to color the faces of a regular tetrahedron with $k$ colors, where two colorings are the same if one can be obtained from the other by a rotation. (The rotation group of the tetrahedron has order 12.)

4. How many distinct necklaces can be made with 6 beads, each bead colored one of 4 colors, if necklaces that can be rotated or flipped to match are considered identical? Apply Burnside's lemma with $G = D_6$, the dihedral group of order 12.

5. Let $G$ act on a set $X$. Prove that the action is transitive if and only if $X$ is a single orbit. Show that any transitive $G$-set $X$ is isomorphic (as a $G$-set) to $G/\text{Stab}(x)$ for any $x \in X$.

6. Prove Cayley's Theorem: every group $G$ is isomorphic to a subgroup of $\text{Sym}(G)$. Determine explicitly, for $G = \mathbb{Z}/3\mathbb{Z}$, which subgroup of $S_3$ it embeds into.

7. Let $G$ be a group of order $n$ acting faithfully on a set $X$ of size $k$. Show that $G$ embeds in $S_k$. Deduce that if $G$ has a subgroup $H$ of index $k$, then $G$ embeds in $S_k$ via the action on cosets, and if additionally $|G| \nmid k!$, then $G$ is not simple.

8. (Challenge) Prove the following generalization of the class equation: if $G$ acts on a finite set $X$ with orbits $\mathcal{O}_1, \ldots, \mathcal{O}_r$, then $|X| = \sum_{i=1}^r [G : \text{Stab}(x_i)]$ where $x_i \in \mathcal{O}_i$. Use this to prove that if $p$ is prime and $|G| = p^n$, then $|X^G| \equiv |X| \pmod{p}$, where $X^G$ denotes the set of fixed points. Deduce that a $p$-group acting on a finite set always fixes a number of elements congruent to $|X|$ modulo $p$.
