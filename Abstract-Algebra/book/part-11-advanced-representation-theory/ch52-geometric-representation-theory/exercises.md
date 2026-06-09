# Chapter 52 — Exercises

## Important Figures

- **Alexandre Beilinson (1957–) & Joseph Bernstein (1945–)** — Beilinson–Bernstein localization theorem (1981): $\mathfrak{g}$-modules with fixed central character $\leftrightarrow$ $\mathcal{D}$-modules on the flag variety $G/B$
- **Pierre Deligne (1944–)** — perverse sheaves; intersection cohomology; proved the Weil conjectures; the formalism of $D^b_c(X)$
- **David Kazhdan (1946–) & George Lusztig (1946–)** — Kazhdan–Lusztig polynomials (1979); the KL conjecture (multiplicities of simples in Verma modules) proved by Brylinski–Kashiwara and BB localization
- **I.N. Bernstein, I.M. Gelfand & S.I. Gelfand** — BGG resolution (1975); Category $\mathcal{O}$ (1976)

## References and Primary Sources

- **A. Beilinson & J. Bernstein, "Localisation de $\mathfrak{g}$-modules" (1981)** — *C.R. Acad. Sci. Paris* 292 — BB localization
- **A. Beilinson, J. Bernstein & P. Deligne, *Faisceaux Pervers* (Astérisque 100, 1982)** — perverse sheaves; the BBD paper
- **J. Humphreys, *Representations of Semisimple Lie Algebras in the BGG Category* $\mathcal{O}$ (AMS, 2008)** — Category $\mathcal{O}$; Kazhdan–Lusztig theory

## Examples, Applications, and Thought Experiments

- **Beilinson–Bernstein localization** — $U(\mathfrak{g})$-modules with the "generic" central character (Harish-Chandra parameter $\rho$) are equivalent to $\mathcal{D}_{G/B}$-modules on the flag variety; algebra becomes geometry; the Verma module $M(\lambda)$ becomes the $\mathcal{D}$-module corresponding to the structure sheaf on a Schubert cell
- **Kazhdan–Lusztig polynomials** — the multiplicity $[M(\mu) : L(\lambda)]$ (of the simple module $L(\lambda)$ in the Verma module $M(\mu)$) equals $P_{w,y}(1)$ where $P_{w,y}$ is the KL polynomial; these polynomials encode the topology of Schubert variety intersections; a purely algebraic multiplicity is computed by topology
- **Category $\mathcal{O}$** — the full subcategory of $U(\mathfrak{g})$-modules that are locally $\mathfrak{n}$-finite and semisimple over $\mathfrak{h}$; it contains the Verma modules $M(\lambda)$ and their irreducible quotients $L(\lambda)$; the BGG resolution $0 \to L(\lambda) \to M(\lambda) \to M(\lambda - \alpha_1) \oplus \cdots$ is an exact complex of Vermas computing cohomology
- **Thought experiment: algebra as topology** — the KL conjecture says that an algebraic question (how many times does an irrep appear in a Verma module?) is answered by a topological computation (intersection cohomology of Schubert varieties); this is a recurring theme in geometric representation theory: algebra predicts topology and topology computes algebra

## Exercises

1. Let $G = GL_3(\mathbb{C})$ and $B$ the Borel subgroup of upper triangular matrices. Describe the complete flag variety $G/B$ concretely as the space of complete flags $0 \subset V_1 \subset V_2 \subset \mathbb{C}^3$. List the Schubert cells $C_w$ for all $w \in S_3 = W$, identify the Bruhat order on $S_3$, and state the dimension of each Schubert cell. Describe the unique Schubert variety of codimension 1 and explain why it is singular.

2. In the Grassmannian $\mathrm{Gr}(2, 4)$ parametrizing 2-planes in $\mathbb{C}^4$, the Schubert classes $\sigma_\lambda$ (indexed by partitions $\lambda$ fitting in a $2 \times 2$ box) form a $\mathbb{Z}$-basis for $H^*(\mathrm{Gr}(2,4), \mathbb{Z})$. Compute the intersection number $\int_{\mathrm{Gr}(2,4)} \sigma_{(1)} \cdot \sigma_{(1)} \cdot \sigma_{(1)} \cdot \sigma_{(1)}$ using the Pieri formula, and interpret the result as a count of 2-planes satisfying four generic incidence conditions.

3. State the Beilinson–Bernstein localization theorem precisely: for a semisimple Lie algebra $\mathfrak{g}$ with flag variety $X = G/B$, and a dominant integral weight $\lambda$, there is an equivalence of categories between $\mathcal{M}(\mathfrak{g}, \lambda)$ (the full subcategory of $U(\mathfrak{g})$-modules with generalized central character $\chi_\lambda$) and $\mathcal{D}_\lambda(X)$-modules. Explain what the twisted sheaf of differential operators $\mathcal{D}_\lambda$ is, and describe what the Verma module $M(\lambda)$ and the simple module $L(\lambda)$ correspond to under this equivalence.

4. Let $\mathfrak{g} = \mathfrak{sl}_2$ with Weyl group $W = \{e, s\}$, where $s$ is the simple reflection. The Kazhdan–Lusztig polynomials are defined by the conditions $P_{y,w} = 0$ if $y \not\leq w$, $P_{w,w} = 1$, and the recursion arising from the Hecke algebra. Compute $P_{e,e}$, $P_{e,s}$, and $P_{s,s}$, and use the Kazhdan–Lusztig conjecture (theorem) to read off the multiplicities $[M(\mu) : L(\lambda)]$ for $\mathfrak{sl}_2$ from these values. Verify consistency with the explicit structure of Verma modules for $\mathfrak{sl}_2$.

5. Let $\mathcal{O}$ be the BGG category for $\mathfrak{g} = \mathfrak{sl}_2$. An object $M \in \mathcal{O}$ is a finitely generated $U(\mathfrak{g})$-module that is semisimple over $\mathfrak{h}$ and locally $\mathfrak{n}$-finite. Show that the Verma module $M(\lambda)$ is in $\mathcal{O}$ for every $\lambda \in \mathfrak{h}^*$. Then show that for $\lambda = -1$ (i.e., the anti-dominant weight for $\mathfrak{sl}_2$), the Verma module $M(-1)$ is irreducible. For which values of $\lambda \in \mathbb{Z}_{\geq 0}$ is $M(\lambda)$ reducible, and what is its unique simple quotient?

6. Describe what a perverse sheaf $\mathcal{F}$ on a Schubert variety $X_w \subset G/B$ is expected to encode in representation theory. Specifically: what does the stalk cohomology $H^k(\mathcal{F})_x$ at a point $x \in C_y \subset X_w$ compute, in terms of Kazhdan–Lusztig polynomials? State the decomposition theorem for the pushforward $Rj_*(\mathcal{L})$ of a local system $\mathcal{L}$ on a smooth Schubert cell $C_w$, and explain the role of intersection cohomology sheaves $IC(\overline{C_w})$ in this decomposition.

7. The BGG resolution of a finite-dimensional simple module $L(\lambda)$ (for $\lambda$ dominant integral) is the exact complex $\cdots \to \bigoplus_{\ell(w)=k} M(w \cdot \lambda) \to \cdots \to M(\lambda) \to L(\lambda) \to 0$, where $w \cdot \lambda = w(\lambda + \rho) - \rho$ denotes the dot action. For $\mathfrak{g} = \mathfrak{sl}_3$ and $\lambda = 0$ (the trivial representation), write out the BGG resolution explicitly, identifying each Verma module by its highest weight. Use the resolution to compute the Euler characteristic and verify it gives the correct dimension of $L(0) = \mathbb{C}$.

8. (Challenge) Let $\mathfrak{g}$ be a semisimple Lie algebra and $\mathcal{O}^0$ the principal block of Category $\mathcal{O}$ (the block containing the trivial module). The indecomposable projective modules $P(w \cdot 0)$ in $\mathcal{O}^0$ are indexed by $w \in W$. State and prove the reciprocity theorem: the multiplicity $(P(w \cdot 0) : M(y \cdot 0))$ of the Verma module $M(y \cdot 0)$ in a Verma flag of $P(w \cdot 0)$ equals the multiplicity $[M(y \cdot 0) : L(w \cdot 0)]$ of the simple module $L(w \cdot 0)$ in $M(y \cdot 0)$. This BGG reciprocity is a formal consequence of the highest weight category structure of $\mathcal{O}^0$.
