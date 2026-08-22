# Chapter 43 — Exercises

## Important Figures

- **Heinrich Maschke (1853–1908)** — Maschke's theorem (1898): if $\text{char}(k) \nmid |G|$, every representation decomposes as a direct sum of irreducibles; complete reducibility
- **Joseph Wedderburn (1882–1948)** — Wedderburn's structure theorem for semisimple algebras (1908): a semisimple algebra over an algebraically closed field is a product of matrix algebras
- **Emil Artin (1898–1962)** — Artin–Wedderburn theorem in full generality over non-algebraically-closed fields; semisimple rings

## References and Primary Sources

- **H. Maschke, "Ueber den arithmetischen Charakter der Coefficienten..." (1898)** — *Math. Ann.* 52
- **J.H.M. Wedderburn, "On Hypercomplex Numbers" (1908)** — *Proc. London Math. Soc.* 6
- **J.-P. Serre, *Linear Representations of Finite Groups* (Springer, 1977)**, Chs. 1–2

## Examples, Applications, and Thought Experiments

- **$\mathbb{C}[S_3] \cong \mathbb{C} \oplus \mathbb{C} \oplus M_2(\mathbb{C})$** — by Artin–Wedderburn; $1^2 + 1^2 + 2^2 = 6 = |S_3|$; the three summands correspond to the trivial, sign, and standard representations; the dimension formula $\sum_i (\dim V_i)^2 = |G|$ is a theorem, not a coincidence
- **Maschke's theorem failure in characteristic 2** — for $G = \mathbb{Z}/2\mathbb{Z}$ over $\mathbb{F}_2$: the upper-triangular unipotent matrix (with diagonal entries 1 and upper-right entry 1) represents $g$ in a 2-dimensional representation; the subspace $\text{span}(e_1)$ is $G$-stable but has no $G$-stable complement; complete reducibility fails when $\text{char}(k) \mid |G|$
- **The regular representation** — $k[G]$ acts on itself by left multiplication; over $\mathbb{C}$, it decomposes as $k[G] \cong \bigoplus_i V_i^{\oplus \dim V_i}$; each irrep $V_i$ appears with multiplicity equal to its own dimension; the regular representation is the "universal" representation containing all others
- **Thought experiment: why does Maschke work?** — the key is to average over the group: if $W \subseteq V$ is $G$-stable, project $V \to W$ by any linear projection $\pi$, then "symmetrize" by $\bar\pi = \frac{1}{|G|}\sum_{g \in G} g \pi g^{-1}$; this average is a $G$-equivariant projection onto $W$; its kernel is a $G$-stable complement; the averaging requires dividing by $|G|$, which fails if $\text{char}(k) \mid |G|$

## Exercises

1. Let $G = \mathbb{Z}/3\mathbb{Z}$ and let $k = \mathbb{C}$. Identify all irreducible complex representations of $G$ and write down the isomorphism $\mathbb{C}[G] \cong \mathbb{C} \oplus \mathbb{C} \oplus \mathbb{C}$ explicitly, giving the idempotents in $\mathbb{C}[G]$ corresponding to each summand. Verify that these idempotents sum to $1$ and are mutually orthogonal.

2. Carry out Maschke's averaging construction explicitly for the permutation representation of $\mathbb{Z}/2\mathbb{Z}$ on $\mathbb{C}^2$ (where the generator acts by swapping coordinates). Take the projection $\pi: \mathbb{C}^2 \to \text{span}(e_1 + e_2)$ given by $\pi(a,b) = \frac{a+b}{2}(1,1)$. Compute the symmetrized projection $\bar\pi = \frac{1}{2}\sum_{g \in G} g\pi g^{-1}$ and verify that $\ker \bar\pi$ is $G$-stable.

3. Show that Maschke's theorem fails for $G = \mathbb{Z}/p\mathbb{Z}$ over $\mathbb{F}_p$ by exhibiting a 2-dimensional representation with a $G$-stable subspace that has no $G$-stable complement. (Hint: consider upper-triangular unipotent matrices.) Explain precisely where the averaging argument breaks down.

4. Let $G = S_3$. Use the Artin–Wedderburn isomorphism $\mathbb{C}[S_3] \cong \mathbb{C} \oplus \mathbb{C} \oplus M_2(\mathbb{C})$ to compute the center $Z(\mathbb{C}[S_3])$. Identify a basis for the center explicitly as elements of $\mathbb{C}[S_3]$ (i.e., as linear combinations of group elements), and verify that the number of basis elements equals the number of conjugacy classes of $S_3$.

5. Let $V$ and $W$ be irreducible representations of $G$ over $\mathbb{C}$. Use Schur's lemma together with the module-theoretic interpretation of representations to prove that $\text{Hom}_G(V, W) = 0$ if $V \not\cong W$, and $\text{Hom}_G(V, V) \cong \mathbb{C}$ if $V \cong W$. Interpret this in terms of the Artin–Wedderburn decomposition: which component of $k[G]$ does $V$ "see"?

6. Apply the dimension formula $\sum_i (\dim V_i)^2 = |G|$ to determine all possible sets of irreducible representation dimensions for groups of order 8. (There are two non-abelian groups of order 8: $D_4$ and $Q_8$.) Show that the constraints $\sum_i d_i^2 = 8$ and $\sum_i 1 =$ (number of conjugacy classes) force the same dimension multiset $\{1,1,1,1,2\}$ for both groups, and locate the conjugacy classes of each group to confirm the count.

7. Let $V_{\text{reg}}$ denote the regular representation of $G$ over $\mathbb{C}$, defined by the left $G$-action on $\mathbb{C}[G]$. Compute the character $\chi_{\text{reg}}(g)$ for each $g \in G$ directly from the definition (without invoking the full decomposition theorem), and show that $\chi_{\text{reg}}(e) = |G|$ and $\chi_{\text{reg}}(g) = 0$ for $g \neq e$. Use these values and the inner product formula to confirm that each irreducible representation $V_i$ appears in $V_{\text{reg}}$ with multiplicity $\dim V_i$.

8. (Challenge) Let $A = k[G]$ for a finite group $G$ with $\text{char}(k) \nmid |G|$. Prove directly from the definition of semisimplicity that $A$ is semisimple as a ring, without appealing to Maschke's theorem or Artin–Wedderburn. That is, show that the Jacobson radical $J(A)$ is zero by proving that every left ideal of $A$ is a direct summand. (Hint: use the averaging trick to produce a two-sided idempotent for each minimal left ideal, and show that any idempotent-generated ideal has a complementary ideal also generated by an idempotent.)
