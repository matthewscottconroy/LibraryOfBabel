# Chapter 15 — Exercises

## Important Figures

- **Emmy Noether (1882–1935)** — formulated the isomorphism theorems in their modern abstract form (1927); her 1921 paper *Idealtheorie in Ringbereichen* contains the essential ideas; the theorems are sometimes called "Noether's isomorphism theorems"
- **Bartel van der Waerden (1903–1996)** — disseminated Noether's work in *Moderne Algebra* (1930–1931); the isomorphism theorems appear there in essentially their modern formulation
- **Camille Jordan (1838–1922)** — early concrete versions of the correspondence between subgroups and quotients

## References and Primary Sources

- **E. Noether, "Abstrakter Aufbau der Idealtheorie in algebraischen Zahl- und Funktionenkörpern" (1927)** — the abstract algebra revolution
- **B.L. van der Waerden, *Moderne Algebra* (1930–1931)** — the textbook that defined a generation's approach to the subject
- **J.J. Rotman, *An Introduction to the Theory of Groups* (4th ed., Springer, 1995)**

## Examples, Applications, and Thought Experiments

- **The sign homomorphism** — $\text{sgn}: S_n \to \{\pm 1\}$ sends each permutation to its sign; $\ker(\text{sgn}) = A_n$; First Isomorphism Theorem: $S_n / A_n \cong \{\pm 1\} \cong \mathbb{Z}/2\mathbb{Z}$; the theorem turns a quotient group into a subgroup of the codomain
- **The exponential map** — $\exp: (\mathbb{R}, +) \to (\mathbb{R}_{>0}, \times)$, $t \mapsto e^t$, is a group isomorphism; inverse is $\log$; no kernel; this shows $(\mathbb{R}, +) \cong (\mathbb{R}_{>0}, \times)$ as abstract groups — the additive and multiplicative structures are "the same"
- **Dihedral group as semidirect product** — $D_n \cong \mathbb{Z}/n\mathbb{Z} \rtimes \mathbb{Z}/2\mathbb{Z}$; the $\mathbb{Z}/2\mathbb{Z}$ acts on $\mathbb{Z}/n\mathbb{Z}$ by inversion $k \mapsto -k$; the semidirect product packages the "twist" in the group law; compare with $\mathbb{Z}/n\mathbb{Z} \times \mathbb{Z}/2\mathbb{Z}$ (direct product), which is abelian
- **Second isomorphism theorem (diamond theorem)** — if $H \leq G$ and $N \trianglelefteq G$, then $H/(H \cap N) \cong HN/N$; draw the lattice of subgroups as a diamond; the two "sides" of the diamond give isomorphic quotients; a fact about how subgroups overlap and span

## Exercises

1. Define $\phi: \mathbb{Z} \to \mathbb{Z}/n\mathbb{Z}$ by $\phi(k) = k \bmod n$. Verify that $\phi$ is a group homomorphism, identify $\ker(\phi)$ and $\text{im}(\phi)$, and apply the First Isomorphism Theorem to conclude $\mathbb{Z}/n\mathbb{Z} \cong \mathbb{Z}/n\mathbb{Z}$ (confirming the theorem is consistent). Now define $\psi: \mathbb{Z} \to \mathbb{Z}/2\mathbb{Z} \times \mathbb{Z}/3\mathbb{Z}$ by $\psi(k) = (k \bmod 2, k \bmod 3)$. Show $\psi$ is surjective, find $\ker(\psi)$, and deduce the Chinese Remainder Theorem: $\mathbb{Z}/6\mathbb{Z} \cong \mathbb{Z}/2\mathbb{Z} \times \mathbb{Z}/3\mathbb{Z}$.

2. Prove the First Isomorphism Theorem: if $\phi: G \to H$ is a group homomorphism, then $G/\ker(\phi) \cong \text{im}(\phi)$. Your proof should explicitly construct the isomorphism and verify it is well-defined, injective, surjective, and a homomorphism.

3. Prove the Second Isomorphism Theorem: if $H \leq G$ and $N \trianglelefteq G$, then $N \trianglelefteq HN$, $H \cap N \trianglelefteq H$, and $H/(H \cap N) \cong HN/N$.

4. Prove the Third Isomorphism Theorem: if $N \trianglelefteq G$ and $N \leq K \leq G$ with $K \trianglelefteq G$, then $K/N \trianglelefteq G/N$ and $(G/N)/(K/N) \cong G/K$.

5. Compute $\text{Aut}(\mathbb{Z}/n\mathbb{Z})$ for $n = 6, 8, 12$. Show in general that $\text{Aut}(\mathbb{Z}/n\mathbb{Z}) \cong (\mathbb{Z}/n\mathbb{Z})^*$ and determine its order $\phi(n)$.

6. Show that $D_6 \cong S_3 \times \mathbb{Z}/2\mathbb{Z}$ as groups. (Hint: identify the relevant normal subgroups of $D_6$ and apply the recognition theorem for direct products.)

7. Determine all homomorphisms from $\mathbb{Z}/12\mathbb{Z}$ to $\mathbb{Z}/8\mathbb{Z}$. For each, identify the kernel and image.

8. (Challenge) A group $G$ is called a semidirect product $N \rtimes H$ if $N \trianglelefteq G$, $H \leq G$, $N \cap H = \{e\}$, and $NH = G$. Classify all groups of order 20 up to isomorphism by showing every such group is a semidirect product $\mathbb{Z}/5\mathbb{Z} \rtimes \mathbb{Z}/4\mathbb{Z}$ or $\mathbb{Z}/5\mathbb{Z} \rtimes (\mathbb{Z}/2\mathbb{Z} \times \mathbb{Z}/2\mathbb{Z})$, and determine which semidirect products are non-isomorphic by analyzing the possible actions.
