# Chapter 31 — Exercises

## Important Figures

- **Évariste Galois (1811–1832)** — invented Galois theory; the fundamental correspondence; wrote down the theory the night before his fatal duel at age 20
- **Joseph-Louis Lagrange (1736–1813)** — Lagrange resolvents; proto-Galois theory for cubics and quartics; the method Galois generalized
- **Ernst Artin (1898–1962)** — reformulated the entire theory via fixed fields of automorphism groups (1942); the modern "Artin approach" is standard today
- **Emil Artin & Richard Dedekind** — independence of characters; the linear independence of automorphisms as characters, the key lemma of the proof

## References and Primary Sources

- **É. Galois, *Manuscripts* (written 1831; published by Liouville, 1846)** — the original; dense and remarkable
- **E. Artin, *Galois Theory* (2nd ed., Notre Dame, 1944)** — the standard modern treatment
- **I. Stewart, *Galois Theory* (4th ed., CRC Press, 2015)** — excellent for building intuition
- **J.-P. Serre, *Topics in Galois Theory* (2nd ed., A.K. Peters, 2008)** — advanced topics; inverse Galois problem

## Examples, Applications, and Thought Experiments

- **$\text{Gal}(\mathbb{Q}(\sqrt{2},\sqrt{3})/\mathbb{Q}) \cong \mathbb{Z}/2\mathbb{Z} \times \mathbb{Z}/2\mathbb{Z}$** — four automorphisms: $\text{id}$; $\sqrt{2} \mapsto -\sqrt{2}$; $\sqrt{3} \mapsto -\sqrt{3}$; both; four subgroups of the Klein four-group correspond to $\mathbb{Q}(\sqrt{2})$, $\mathbb{Q}(\sqrt{3})$, $\mathbb{Q}(\sqrt{6})$, and the full field; the lattice bijection is explicit
- **Cyclotomic fields** — $\text{Gal}(\mathbb{Q}(\zeta_n)/\mathbb{Q}) \cong (\mathbb{Z}/n\mathbb{Z})^*$; the automorphism $\sigma_k: \zeta_n \mapsto \zeta_n^k$ for $(k,n)=1$; for $p$ prime, this is cyclic of order $p-1$; connects Galois theory with number theory
- **The Galois correspondence as duality** — subgroups and intermediate fields form lattices; the Galois correspondence reverses inclusion: larger subgroup $\leftrightarrow$ smaller field; this is an anti-isomorphism of lattices; a beautiful piece of abstract duality
- **Galois group as a permutation group** — $\text{Gal}(K/F)$ acts on the roots of any polynomial in $F[x]$ that splits over $K$; for an irreducible polynomial of degree $n$, this embeds $\text{Gal}$ into $S_n$; the structure of this embedding (transitive, primitive, etc.) reflects the polynomial's arithmetic

## Exercises

1. Let $K = \mathbb{Q}(\sqrt{2}, \sqrt{3})$. List all elements of $\text{Gal}(K/\mathbb{Q})$, write down the complete subgroup lattice of $\text{Gal}(K/\mathbb{Q})$, and identify explicitly the intermediate field corresponding to each proper subgroup via the Galois correspondence.

2. Let $\zeta_5 = e^{2\pi i/5}$ and $K = \mathbb{Q}(\zeta_5)$. Prove that $K/\mathbb{Q}$ is Galois with $\text{Gal}(K/\mathbb{Q}) \cong (\mathbb{Z}/5\mathbb{Z})^* \cong \mathbb{Z}/4\mathbb{Z}$. Identify the unique intermediate field $E$ with $[E:\mathbb{Q}] = 2$, and describe it explicitly as a subfield of $\mathbb{R}$.

3. Compute $\text{Gal}(\mathbb{Q}(\sqrt[3]{2}, \omega)/\mathbb{Q})$ where $\omega = e^{2\pi i/3}$. Show it has order 6 and is isomorphic to $S_3$. Identify all six automorphisms explicitly by specifying where each one sends $\sqrt[3]{2}$ and $\omega$.

4. Let $G = \text{Gal}(K/F)$ be a Galois group and $H \leq G$ a subgroup. The Galois correspondence assigns to $H$ the fixed field $K^H$. Prove that $[K^H : F] = [G : H]$ and $[K : K^H] = |H|$.

5. Prove that if $H$ is a normal subgroup of $G = \text{Gal}(K/F)$ with fixed field $E = K^H$, then $E/F$ is a Galois extension and there is a group isomorphism $\text{Gal}(E/F) \cong G/H$.

6. Let $f(x) = x^4 - 5x^2 + 6 = (x^2-2)(x^2-3)$. Compute the splitting field $K$ of $f$ over $\mathbb{Q}$, determine $\text{Gal}(K/\mathbb{Q})$, and verify the Galois correspondence by listing all intermediate fields and their corresponding subgroups.

7. The discriminant of a degree-$n$ polynomial $f$ with roots $\alpha_1, \ldots, \alpha_n$ is $\Delta = \prod_{i < j}(\alpha_i - \alpha_j)^2$. Prove that $\text{Gal}(f/\mathbb{Q}) \subseteq A_n$ if and only if $\sqrt{\Delta} \in \mathbb{Q}$. Use this to determine whether $\text{Gal}(x^3 - 3x + 1 / \mathbb{Q})$ is contained in $A_3$.

8. (Challenge) Let $f(x) = x^5 - 5x + 12 \in \mathbb{Q}[x]$. Show that $f$ is irreducible over $\mathbb{Q}$, and by computing the discriminant or by other means, show that $\text{Gal}(f/\mathbb{Q}) \cong S_5$ or $D_5$. (Hint: find the number of real roots and use the fact that a transitive subgroup of $S_5$ containing a transposition and a 5-cycle is all of $S_5$.)
