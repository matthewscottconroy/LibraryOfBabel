# Chapter 44 — Exercises

## Important Figures

- **Ferdinand Georg Frobenius (1849–1917)** — character orthogonality relations (1896); defined characters as class functions and proved their orthogonality
- **Issai Schur (1875–1941)** — Schur orthogonality; Schur index; proved that the characters form an orthonormal basis for class functions
- **Richard Brauer (1901–1977)** — Brauer characters in modular representation theory; the characteristic-$p$ analogue
- **William Burnside (1852–1927)** — used characters to prove the $p^a q^b$ theorem (1904); the first major application of representation theory to pure group theory

## References and Primary Sources

- **G. Frobenius, "Über Gruppencharaktere" (1896)** — *Sitzungsber. Preuss. Akad. Wiss.* — founding paper
- **J.-P. Serre, *Linear Representations of Finite Groups* (Springer, 1977)** — the essential text
- **G. James & M. Liebeck, *Representations and Characters of Groups* (2nd ed., Cambridge, 2001)**

## Examples, Applications, and Thought Experiments

- **Character table of $S_3$** — three conjugacy classes ($\{e\}$, $\{(12),(13),(23)\}$, $\{(123),(132)\}$); three irreducible characters (trivial, sign, standard of degree 2); the $3 \times 3$ character table satisfies both row and column orthogonality; it encodes all representation theory of $S_3$ in a $3 \times 3$ matrix
- **Detecting abelianness** — $G$ is abelian iff every irreducible character has degree 1 iff the number of conjugacy classes equals $|G|$; the character table is diagonal (all degrees are 1); abelian groups have "trivial" representation theory
- **Burnside's $p^a q^b$ theorem via characters** — if $G$ has a conjugacy class of size $p^a$ (prime power $> 1$), then $G$ is not simple; the proof uses orthogonality: an irreducible character of degree divisible by $p$ forces a normal subgroup; the theorem follows by induction on $|G|$; pure algebra yields a pure group-theoretic result
- **Inner product on class functions** — $\langle \chi, \psi \rangle = \frac{1}{|G|}\sum_{g \in G} \chi(g)\overline{\psi(g)}$; irreducible characters form an orthonormal basis; $\langle \chi_V, \chi_V \rangle = 1$ iff $V$ is irreducible; the inner product is the "spectral analysis" tool for representations

## Exercises

1. Compute the full character table of $S_3$ from scratch. List the conjugacy classes and their sizes, write down the three irreducible characters (trivial, sign, and standard), and verify both the row orthogonality relations $\langle \chi_i, \chi_j \rangle = \delta_{ij}$ and the column orthogonality relations $\sum_i \chi_i(g)\overline{\chi_i(h)} = |C_G(g)| \delta_{[g],[h]}$.

2. Compute the character table of $\mathbb{Z}/4\mathbb{Z}$. Since this group is abelian, all irreducible representations are 1-dimensional. List all four characters explicitly, verify orthogonality, and write the character table as a $4 \times 4$ matrix. Note the relationship between this table and the discrete Fourier transform on $\mathbb{Z}/4\mathbb{Z}$.

3. Determine the character table of $D_4 = \langle r, s \mid r^4 = s^2 = e,\, srs^{-1} = r^{-1} \rangle$. First, identify all 5 conjugacy classes of $D_4$. Then use the constraints $\sum_i d_i^2 = 8$ (with each $d_i \geq 1$) and the number of conjugacy classes to determine the degrees of the irreducible characters, and compute the character values using the orthogonality relations.

4. Let $\chi$ be a class function on $G$ given by $\chi = 3\chi_1 + \chi_2 - \chi_3$, where $\chi_1, \chi_2, \chi_3$ are distinct irreducible characters of $G$. Is $\chi$ the character of a representation? Justify your answer using the inner product. More generally, state necessary and sufficient conditions (in terms of the inner product) for a class function to be the character of a genuine representation.

5. Let $V$ and $W$ be representations of $G$ over $\mathbb{C}$. Prove that $V \cong W$ as representations if and only if $\chi_V = \chi_W$ as functions on $G$. (This requires showing that two representations with the same character must be isomorphic — the "if" direction — which follows from expressing the multiplicity of each irreducible in terms of the inner product with its character.)

6. Let $G = Q_8 = \{\pm 1, \pm i, \pm j, \pm k\}$ be the quaternion group. List the conjugacy classes of $Q_8$ and use the dimension formula and class count to determine all possible irreducible character degrees. Compute the full $5 \times 5$ character table of $Q_8$. Compare the character table of $Q_8$ with that of $D_4$: in what sense do the two groups have "the same" character table, and what does this imply about the ability of characters to distinguish non-isomorphic groups?

7. Let $\chi$ be an irreducible character of $G$ with $\chi(e) = 2$. Use the first orthogonality relation and the constraint $|\chi(g)| \leq \chi(e)$ to bound the possible values of $\chi(g)$ for elements $g$ of order 2 and order 3. Then apply Burnside's theorem: if $G$ is simple of order $p^a q^b$, derive a contradiction by showing that the existence of a prime-power-sized conjugacy class leads to an irreducible character vanishing on that class, forcing a proper normal subgroup.

8. (Challenge) Let $N \trianglelefteq G$ be a normal subgroup. Prove that $N = \bigcap_{\chi \text{ irred.}} \ker \chi_i^{\phantom{i}}$, where the intersection runs over all irreducible characters $\chi_i$ whose kernel contains $N$, is exactly the set of elements $g \in G$ such that $|\chi_i(g)| = \chi_i(e)$ for every irreducible $\chi_i$. Use this to give a character-theoretic characterization of simple groups, and verify the characterization for $A_4$.
