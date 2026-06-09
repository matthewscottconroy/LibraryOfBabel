# Chapter 48 — Exercises

## Important Figures

- **Sophus Lie (1842–1899)** — Lie's theorem: every representation of a solvable Lie algebra over $\mathbb{C}$ has a simultaneous eigenvector; the upper triangular form
- **Eugen Engel (1861–1941)** — Engel's theorem: $\mathfrak{g}$ is nilpotent iff every adjoint operator $\text{ad}_x$ is nilpotent
- **Élie Cartan (1869–1951)** — Cartan's criterion for solvability (Killing form); proved Levi decomposition; semisimple structure theory
- **Eugene Levi (1883–1917)** — Levi decomposition theorem (1905): $\mathfrak{g} = \text{rad}(\mathfrak{g}) \rtimes \mathfrak{s}$ with $\mathfrak{s}$ semisimple

## References and Primary Sources

- **J. Humphreys, *Introduction to Lie Algebras and Representation Theory* (Springer, 1972)**, Chs. I–III — solvable, nilpotent, and semisimple algebras; Cartan's criterion
- **N. Bourbaki, *Lie Groups and Lie Algebras*, Chs. 1–3 (Springer, 1989)** — systematic comprehensive treatment
- **É. Cartan, "Sur la structure des groupes de transformations finis et continus" (1894)** — doctoral thesis; complete classification

## Examples, Applications, and Thought Experiments

- **Upper triangular matrices $\mathfrak{b}_n$** — the Borel subalgebra: solvable; Lie's theorem guarantees that any complex representation simultaneously triangularizes; the "standard" solvable Lie algebra; $\mathfrak{b}_n / [\mathfrak{b}_n, \mathfrak{b}_n] \cong \mathfrak{h}_n$ (diagonal matrices)
- **Strictly upper triangular matrices $\mathfrak{n}_n$** — nilpotent Lie algebra; all elements act nilpotently; Engel's theorem characterizes nilpotency this way; $\mathfrak{n}_n^k = 0$ for $k$ large; the derived series and lower central series both terminate
- **Levi decomposition of a non-semisimple algebra** — the Poincaré Lie algebra (symmetries of spacetime) $\cong \mathbb{R}^4 \rtimes \mathfrak{so}(3,1)$; the translation part $\mathbb{R}^4$ is the radical (abelian, solvable); the Lorentz algebra $\mathfrak{so}(3,1)$ is the Levi factor (semisimple); physics is full of non-semisimple Lie algebras built this way
- **Cartan's criterion** — $\mathfrak{g}$ is semisimple iff the Killing form $B(x,y) = \text{tr}(\text{ad}_x \text{ad}_y)$ is non-degenerate; solvable iff $B|_{[\mathfrak{g},\mathfrak{g}]} = 0$; a single bilinear form detects the entire structural hierarchy

## Exercises

1. Compute the derived series $\mathfrak{g}^{(0)} \supset \mathfrak{g}^{(1)} \supset \mathfrak{g}^{(2)} \supset \cdots$ for each of the following Lie algebras and determine which are solvable: (a) the Borel subalgebra $\mathfrak{b}_3 \subset \mathfrak{gl}_3$ of upper triangular matrices; (b) $\mathfrak{sl}_2(\mathbb{C})$; (c) the two-dimensional non-abelian Lie algebra with basis $\{e, f\}$ and bracket $[e,f] = e$.

2. Compute the lower central series $\mathfrak{g}^0 \supset \mathfrak{g}^1 \supset \mathfrak{g}^2 \supset \cdots$ for the Heisenberg algebra $\mathfrak{h}$ with basis $\{p, q, z\}$ and brackets $[p,q] = z$, $[p,z] = [q,z] = 0$. Determine the nilpotency class of $\mathfrak{h}$. Show that $\mathfrak{h}$ is solvable, and compare its derived series with its lower central series.

3. Apply Engel's theorem to the Lie algebra $\mathfrak{n}_3$ of strictly upper triangular $3 \times 3$ matrices: verify explicitly that each $\text{ad}_X$ is a nilpotent linear transformation on $\mathfrak{n}_3$, and find a simultaneous eigenvector (i.e., a weight vector with zero weight) for the $\text{ad}$-action on $\mathfrak{n}_3$.

4. Let $\rho: \mathfrak{b}_2 \to \mathfrak{gl}(V)$ be a representation of the two-dimensional solvable Lie algebra $\mathfrak{b}_2 = \text{span}\{h, e\}$ with $[h,e] = e$ over $\mathbb{C}$, where $V$ is a finite-dimensional complex vector space. Apply Lie's theorem to show that there exists a common eigenvector $v \in V$ for all $\rho(x)$, $x \in \mathfrak{b}_2$. Construct an explicit such representation on $\mathbb{C}^2$ and exhibit the simultaneous eigenvector.

5. Use Cartan's criterion to test for semisimplicity: compute the Killing form of $\mathfrak{sl}_2(\mathbb{C})$ in the basis $\{e, f, h\}$ and show it is non-degenerate. Then compute the Killing form of the three-dimensional Lie algebra $\mathfrak{r}$ with basis $\{x, y, z\}$ and brackets $[x,y] = z$, $[x,z] = [y,z] = 0$, and show it is degenerate. Identify the radical of $\mathfrak{r}$.

6. Verify Weyl's theorem on complete reducibility in a concrete case: take the two-dimensional representation $V$ of $\mathfrak{sl}_2(\mathbb{C})$ (the standard representation) and its dual $V^*$, and decompose the tensor product $V \otimes V^*$ as a direct sum of irreducible $\mathfrak{sl}_2$-modules.

7. Identify the Levi decomposition of the Lie algebra $\mathfrak{p} = \mathfrak{gl}_n(\mathbb{R})$. Show that $\text{rad}(\mathfrak{gl}_n) = \mathbb{R} \cdot I$ (scalar matrices) and that $\mathfrak{gl}_n = \mathbb{R} \cdot I \oplus \mathfrak{sl}_n$, with $\mathfrak{sl}_n$ semisimple. Why does this decomposition not contradict the Levi theorem (which is for non-semisimple ideals)?

8. (Challenge) Let $\mathfrak{g}$ be a finite-dimensional Lie algebra over $\mathbb{C}$ and $\mathfrak{r} = \text{rad}(\mathfrak{g})$ its radical. Prove that $[\mathfrak{g}, \mathfrak{r}] \subset \text{nil-rad}(\mathfrak{g})$, where the nilradical $\text{nil-rad}(\mathfrak{g})$ is the largest nilpotent ideal of $\mathfrak{g}$. Use this, together with Engel's theorem, to show that every element of $[\mathfrak{g}, \mathfrak{r}]$ acts nilpotently in every finite-dimensional representation.
