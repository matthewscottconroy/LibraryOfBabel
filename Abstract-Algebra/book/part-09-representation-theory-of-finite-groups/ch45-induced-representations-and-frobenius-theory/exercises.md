# Chapter 45 — Exercises

## Important Figures

- **Ferdinand Georg Frobenius (1849–1917)** — induction, Frobenius reciprocity (1898), Frobenius groups; the construction of representations from subgroups
- **George Mackey (1916–2006)** — Mackey's irreducibility criterion; the system of imprimitivity; representation theory of locally compact groups
- **William Clifford (1845–1879)** — Clifford theory: analyzing the restriction of irreps to normal subgroups

## References and Primary Sources

- **G. Frobenius, "Über Relationen zwischen den Charakteren einer Gruppe und denen ihrer Untergruppen" (1898)** — Frobenius reciprocity
- **G. Mackey, "Induced Representations of Groups and Quantum Mechanics" (Benjamin, 1968)**
- **J.-P. Serre, *Linear Representations of Finite Groups* (Springer, 1977)**, Chs. 3, 7 — induction and Mackey formula

## Examples, Applications, and Thought Experiments

- **Frobenius reciprocity** — $\langle \text{Ind}_H^G \chi, \psi \rangle_G = \langle \chi, \text{Res}_H^G \psi \rangle_H$; induction is the left adjoint of restriction; to determine how many times an irrep $\psi$ of $G$ appears in $\text{Ind}_H^G \chi$, compute the simpler inner product on $H$; this is the algebraic analogue of the adjunction $(- \otimes) \dashv \text{Hom}$
- **Inducing the trivial character** — $\text{Ind}_H^G 1_H =$ permutation representation on $G/H$; the character value at $g$ counts the number of cosets fixed by $g$; decomposing this induced character into irreps gives all permutation representations
- **Building $S_4$'s characters from $S_3$** — embed $S_3 \hookrightarrow S_4$; induce the characters of $S_3$ to $S_4$; use Frobenius reciprocity to decompose into irreps of $S_4$; the 5 irreps of $S_4$ (dimensions 1,1,2,3,3) can all be found this way
- **Frobenius groups** — $G$ is a Frobenius group with kernel $N$ and complement $H$ if $H \cap H^g = 1$ for all $g \notin H$ and $N = G \setminus \bigcup_{g} H^g \cup \{e\}$; Frobenius' theorem (proved via characters): $N$ is a normal subgroup; the dihedral groups $D_n$ for odd $n$ are Frobenius groups

## Exercises

1. Let $H = \{e, (12)\} \leq S_3$ and let $\sigma$ be the trivial representation of $H$. Compute the induced representation $\text{Ind}_H^{S_3} \sigma$ directly from the definition: write down the cosets of $H$ in $S_3$, construct the induced module, and compute the matrices of the induced action. Then use Frobenius reciprocity to decompose $\text{Ind}_H^{S_3} \sigma$ into irreducibles of $S_3$, and verify that your decomposition matches the direct construction.

2. Let $G = S_4$ and $H = S_3 \leq S_4$ (embedded as the stabilizer of 4). Compute the character of $\text{Ind}_H^{S_4} \sigma$ for each of the three irreducible characters $\sigma$ of $S_3$ using the induced character formula $\chi^G(g) = \frac{1}{|H|} \sum_{x \in G,\, x^{-1}gx \in H} \sigma(x^{-1}gx)$. Then apply Frobenius reciprocity to decompose each induced character into irreducibles of $S_4$.

3. Verify Frobenius reciprocity directly in the following case: let $G = S_3$, $H = \langle (123) \rangle \cong \mathbb{Z}/3\mathbb{Z}$, and let $\sigma$ be the non-trivial 1-dimensional character of $H$ given by $\sigma((123)) = \omega = e^{2\pi i/3}$. Compute both $\langle \text{Ind}_H^G \sigma, \rho \rangle_G$ and $\langle \sigma, \text{Res}_H^G \rho \rangle_H$ for each irreducible representation $\rho$ of $S_3$, and confirm the equality in each case.

4. Let $G = S_4$, $H = \langle (12)(34), (13)(24) \rangle \cong \mathbb{Z}/2\mathbb{Z} \times \mathbb{Z}/2\mathbb{Z}$ (the Klein four-subgroup). The double coset decomposition $H \backslash G / H$ plays a key role in Mackey's theorem. List all double cosets $HgH$ for this pair $(G, H)$, and for the trivial character $\sigma = 1_H$, write down the Mackey formula: $\text{Res}_H^G \text{Ind}_H^G 1_H \cong \bigoplus_{s \in H \backslash G / H} \text{Ind}_{H \cap sHs^{-1}}^H 1_{H \cap sHs^{-1}}$. Compute the character of $\text{Res}_H^G \text{Ind}_H^G 1_H$ and verify that it equals the right-hand side.

5. Let $H \leq G$ and let $\sigma$ be a representation of $H$. State Mackey's irreducibility criterion (that $\text{Ind}_H^G \sigma$ is irreducible if and only if $\sigma$ is irreducible and, for every $g \in G \setminus H$, the representations $\sigma$ and $\sigma^g$ of $H \cap H^g$ have no common irreducible constituent). Apply this criterion to determine whether $\text{Ind}_H^{S_4} \sigma$ is irreducible, where $H = \langle (1234) \rangle \cong \mathbb{Z}/4\mathbb{Z}$ and $\sigma$ is the character given by $\sigma((1234)) = i$.

6. The dihedral group $D_5 = \langle r, s \mid r^5 = s^2 = e,\, srs = r^{-1} \rangle$ is a Frobenius group with Frobenius complement $H = \langle s \rangle \cong \mathbb{Z}/2\mathbb{Z}$ and Frobenius kernel $N = \langle r \rangle \cong \mathbb{Z}/5\mathbb{Z}$. Verify directly that $H \cap H^g = \{e\}$ for all $g \notin H$. Then compute the irreducible characters of $D_5$ by inducing from $N$: induce each of the 5 irreducible characters of $N$ to $D_5$ and decompose them using Frobenius reciprocity. Confirm that you recover all irreducible representations of $D_5$.

7. Let $G = A_4$ and $H = \langle (12)(34), (13)(24) \rangle \cong V_4$ (the Klein four-group, which is normal in $A_4$). Compute the character $\text{Ind}_H^{A_4} \sigma$ for each of the four 1-dimensional characters $\sigma$ of $H$. Use Frobenius reciprocity and the known character table of $A_4$ (which has irreducibles of dimensions 1, 1, 1, 3) to express each induced character as a sum of irreducibles. What is the relationship between the 3-dimensional irreducible of $A_4$ and the induced characters from $H$?

8. (Challenge) Let $G$ be a Frobenius group with complement $H$ and kernel $N$. Without assuming Frobenius's theorem (that $N$ is a subgroup), use the following character-theoretic argument to show it: first, show that every irreducible character of $H$ that is not the restriction of a character of $G$ induces to an irreducible character of $G$; second, use the orthogonality relations and the Frobenius condition $H \cap H^g = \{e\}$ to show that the sum $1 + \sum_{\chi \text{ non-trivial irred. of } H} |\text{Ind}_H^G \chi|^2$ equals $|N|$; finally, identify this sum with the character of the regular representation of a putative group $N$ and conclude that $N$ must be a normal subgroup of $G$.
