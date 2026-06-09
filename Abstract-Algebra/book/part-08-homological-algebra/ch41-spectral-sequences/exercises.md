# Chapter 41 — Exercises

## Important Figures

- **Jean Leray (1906–1998)** — invented spectral sequences (c. 1946, while a prisoner of war) to compute cohomology of fiber bundles; the Leray spectral sequence
- **Jean-Pierre Serre (1926–)** — Serre spectral sequence (1951); used it to compute homotopy groups of spheres; Fields Medal 1954 in part for this work
- **Roger Lyndon (1917–1988) & Gerhard Hochschild (1915–2010)** — Lyndon–Hochschild–Serre (LHS) spectral sequence for group extensions
- **Alexander Grothendieck (1928–2014)** — Grothendieck spectral sequence; the Leray spectral sequence in the setting of derived categories

## References and Primary Sources

- **J. Leray, "Structure de l'anneau d'homologie d'une représentation" (1946)** — *C.R. Acad. Sci. Paris*
- **J.-P. Serre, "Homologie singulière des espaces fibrés" (1951)** — *Ann. Math.* 54 — Serre spectral sequence
- **C. Weibel, *An Introduction to Homological Algebra* (Cambridge, 1994)**, Ch. 5 — spectral sequences for homological algebra
- **J. McCleary, *A User's Guide to Spectral Sequences* (2nd ed., Cambridge, 2001)** — comprehensive reference

## Examples, Applications, and Thought Experiments

- **Serre spectral sequence for the Hopf fibration** — $S^1 \to S^3 \to S^2$; the $E_2$ page is $E_2^{p,q} = H_p(S^2; H_q(S^1))$; the spectral sequence collapses at $E_3$; one differential must be an isomorphism to match $H_*(S^3)$; this recovers the cohomology of the total space from the base and fiber
- **Lyndon–Hochschild–Serre** — for an extension $1 \to N \to G \to Q \to 1$: $E_2^{p,q} = H_p(Q; H_q(N; A)) \Rightarrow H_{p+q}(G; A)$; the spectral sequence decomposes the cohomology of $G$ into contributions from $N$ and $Q$; used to compute cohomology of many groups
- **Thought experiment: "a movie of approximations"** — the pages $E_r$ are successive approximations; each page is the homology of the differential on the previous page; the sequence "converges" like successive refinements of an estimate; one imagines each page as a snapshot of the information, with each differential "canceling" some piece; the limit is the true answer, recovered page by page
- **Derived category perspective** — in the derived category, a spectral sequence arises from filtering a complex; the associated graded of the filtration gives the $E_1$ page; subsequent pages are computed by taking cohomology; the spectral sequence is the process of "unfolding" a filtered complex into its components

## Exercises

1. Consider the double complex $C^{p,q}$ with $C^{0,0} = \mathbb{Z}$, $C^{1,0} = \mathbb{Z}$, $C^{0,1} = \mathbb{Z}$, $C^{1,1} = \mathbb{Z}$, and all other terms zero. Let the horizontal differential $\partial': C^{0,q} \to C^{1,q}$ be multiplication by 2 and the vertical differential $\partial'': C^{p,0} \to C^{p,1}$ be multiplication by 3. Compute the $E_1$ and $E_2$ pages of the spectral sequence associated to the horizontal filtration. Then compute the cohomology of the total complex directly and verify that $E_\infty$ matches.

2. Let $E_r$ be a first-quadrant spectral sequence (so $E_r^{p,q} = 0$ unless $p, q \geq 0$). Prove that for any fixed bidegree $(p,q)$, the module $E_r^{p,q}$ stabilizes for all sufficiently large $r$. (Hint: for degree reasons, both the differential out of and the differential into $E_r^{p,q}$ must eventually land outside the first quadrant.) Conclude that $E_\infty^{p,q}$ is well-defined.

3. Using the Serre spectral sequence for the path-loop fibration $\Omega S^2 \to PS^2 \to S^2$ (where $PS^2$ is contractible), compute the cohomology groups $H^n(\Omega S^2; \mathbb{Z})$ for $n \leq 4$. The $E_2$ page is $E_2^{p,q} = H^p(S^2; H^q(\Omega S^2; \mathbb{Z}))$. Use the fact that $H^*(PS^2; \mathbb{Z}) = \mathbb{Z}$ concentrated in degree 0 to determine all differentials.

4. Let $1 \to \mathbb{Z} \to \mathbb{Z} \to \mathbb{Z}/n\mathbb{Z} \to 1$ be the standard extension (where the first map is multiplication by $n$). Use the Lyndon–Hochschild–Serre spectral sequence for this extension and $A = \mathbb{Z}$ (trivial action) to compute $H^*({\mathbb{Z}/n\mathbb{Z}}; \mathbb{Z})$ from the known cohomology of $\mathbb{Z}$. Check your answer against the direct computation $H^k(\mathbb{Z}/n\mathbb{Z}; \mathbb{Z}) \cong \mathbb{Z}/n\mathbb{Z}$ for $k$ even and positive, and $0$ for $k$ odd and positive.

5. Let $G = \mathbb{Z}/p\mathbb{Z} \times \mathbb{Z}/p\mathbb{Z}$ for a prime $p$, viewed as an extension $1 \to \mathbb{Z}/p\mathbb{Z} \to G \to \mathbb{Z}/p\mathbb{Z} \to 1$. The LHS spectral sequence has $E_2^{i,j} = H^i(\mathbb{Z}/p\mathbb{Z}; H^j(\mathbb{Z}/p\mathbb{Z}; \mathbb{F}_p))$. Since the extension splits, the spectral sequence degenerates at $E_2$. Write down the $E_2$ page in low degrees and identify the resulting ring structure on $H^*(G; \mathbb{F}_p)$.

6. Prove that if a first-quadrant spectral sequence has $E_2^{p,q} = 0$ for all $q > 0$ (i.e., the $E_2$ page is concentrated in the bottom row), then the spectral sequence collapses at $E_2$ and $H_n \cong E_2^{n,0}$ for all $n$. What does this say about the five-term exact sequence?

7. Let $C^{p,q}$ be the double complex with rows $C^{p,q} = R$ for $0 \leq p \leq 2$ and $q = 0, 1$, and all other terms zero. The horizontal differentials are given by a free resolution of an $R$-module $M$ in the bottom row and the induced maps on $N = C^{p,1}$ in the top row. Show that the two spectral sequences of this double complex converge to the same thing, and use this to re-derive the long exact sequence in $\operatorname{Ext}$ associated to a short exact sequence $0 \to N \to E \to M \to 0$.

8. (Challenge) Prove the five-term exact sequence from the Lyndon–Hochschild–Serre spectral sequence: for a group extension $1 \to N \to G \to Q \to 1$ and a trivial $G$-module $A$, show there is an exact sequence $H^1(Q,A) \to H^1(G,A) \to H^1(N,A)^Q \to H^2(Q,A) \to H^2(G,A)$. Identify each map as an edge homomorphism or transgression in the LHS spectral sequence, and prove exactness at each term directly from the $E_2$ page and the differential $d_2: E_2^{0,1} \to E_2^{2,0}$.
