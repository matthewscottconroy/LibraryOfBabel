# Chapter 42 — Exercises

## Important Figures

- **Ferdinand Georg Frobenius (1849–1917)** — invented character theory (1896–1897); the first systematic study of group representations over $\mathbb{C}$; defined the character of a representation
- **William Burnside (1852–1927)** — *Theory of Groups of Finite Order* (1911): representation theory applied to prove structural results about groups; the $p^a q^b$ theorem
- **Issai Schur (1875–1941)** — Schur's lemma (1905); Schur orthogonality; Schur index; the fundamental constraints on intertwining maps

## References and Primary Sources

- **G. Frobenius, "Über Gruppencharaktere" (1896)** — *Sitzungsber. Preuss. Akad. Wiss.* — birth of representation theory
- **J.-P. Serre, *Linear Representations of Finite Groups* (Springer, 1977)** — the definitive short text; essential reading
- **G. James & M. Liebeck, *Representations and Characters of Groups* (2nd ed., Cambridge, 2001)** — accessible and thorough

## Examples, Applications, and Thought Experiments

- **Representations of $\mathbb{Z}/2\mathbb{Z}$** — two irreducible complex representations: the trivial ($\rho(g) = 1$) and the sign ($\rho(g) = -1$); every complex representation decomposes as a direct sum of these two; $\mathbb{C}[\mathbb{Z}/2\mathbb{Z}] \cong \mathbb{C} \oplus \mathbb{C}$
- **The 2-dimensional representation of $S_3$** — $S_3$ acts on $\mathbb{C}^3$ by permuting coordinates; the invariant subspace $\{(a,a,a)\}$ contributes the trivial rep; the orthogonal complement $\{(a,b,c) : a+b+c=0\}$ is 2-dimensional and irreducible; this is the "standard representation" and the key non-trivial example
- **Schur's lemma in practice** — if $V$ is irreducible over $\mathbb{C}$ and $\phi: V \to V$ is an intertwining map, then $\phi = \lambda \cdot \text{id}$ for some $\lambda \in \mathbb{C}$; consequence: irreducible representations of abelian groups over $\mathbb{C}$ are always 1-dimensional (since all maps between them are scalars, the group acts by scalars, so any 1-d subspace is invariant)
- **Quantum mechanics connection** — a group $G$ of symmetries of a physical system acts on the Hilbert space $\mathcal{H}$ of states by unitary operators; the irreducible representations of $G$ classify the "quantum numbers" of the system; this is why the representation theory of $SO(3)$ gives the angular momentum quantum numbers $l = 0, 1, 2, \ldots$

## Exercises

1. Let $\omega = e^{2\pi i/n}$ be a primitive $n$-th root of unity. For each $k \in \{0, 1, \ldots, n-1\}$, define $\rho_k: \mathbb{Z}/n\mathbb{Z} \to GL_1(\mathbb{C})$ by $\rho_k([m]) = \omega^{km}$. Verify that each $\rho_k$ is a well-defined group homomorphism, and show that these are pairwise non-isomorphic. Conclude that $\mathbb{Z}/n\mathbb{Z}$ has exactly $n$ irreducible complex representations, all of dimension 1.

2. Recall that $S_3 = \langle r, s \mid r^3 = s^2 = e,\, srs = r^{-1} \rangle$ and let $\omega = e^{2\pi i/3}$. Define $\rho: S_3 \to GL_2(\mathbb{C})$ by setting $\rho(r)$ to be the diagonal matrix with entries $\omega, \omega^2$ and $\rho(s)$ to be the matrix with off-diagonal entries $1$ (i.e., $\rho(s)(e_1) = e_2$ and $\rho(s)(e_2) = e_1$). Verify the representation axioms: check that $\rho(r)^3 = I$, $\rho(s)^2 = I$, and $\rho(s)\rho(r)\rho(s) = \rho(r)^{-1}$. Show that this representation is irreducible by proving it has no 1-dimensional invariant subspace.

3. Let $G$ be a finite abelian group. Prove that every irreducible complex representation of $G$ has dimension 1. (Hint: apply Schur's lemma to the matrices $\rho(g)$ for each $g \in G$, noting that they mutually commute and hence share a common eigenvector.)

4. The permutation representation of $S_3$ on $\mathbb{C}^3$ is defined by $\rho(\sigma)(e_i) = e_{\sigma(i)}$ for the standard basis vectors $e_1, e_2, e_3$. Identify the two invariant subspaces $W_1 = \text{span}(e_1 + e_2 + e_3)$ and $W_2 = \{(a,b,c) : a + b + c = 0\}$, verify that both are $G$-stable, and write $\mathbb{C}^3 = W_1 \oplus W_2$ explicitly as a direct sum of subrepresentations. Which irreducible representations do $W_1$ and $W_2$ correspond to?

5. Let $\rho: G \to GL(V)$ and $\sigma: G \to GL(W)$ be representations. Define $(\rho \oplus \sigma)(g)$ as the block-diagonal matrix acting on $V \oplus W$, and $(\rho \otimes \sigma)(g) = \rho(g) \otimes \sigma(g)$ acting on $V \otimes W$. Verify the representation axioms for both constructions. Then compute the matrices for $\rho \otimes \sigma$ in the case where $\rho$ is the sign representation of $S_3$ and $\sigma$ is the standard 2-dimensional representation. Identify the result up to isomorphism.

6. Let $\phi: V \to W$ be an intertwining map (a $G$-equivariant linear map) between representations $\rho$ and $\sigma$ of $G$. Prove that $\ker \phi$ is a subrepresentation of $V$ and $\text{im}\, \phi$ is a subrepresentation of $W$. Use this to give a self-contained proof of the first part of Schur's lemma: if $V$ and $W$ are both irreducible and $\phi \neq 0$, then $\phi$ is an isomorphism.

7. Suppose $V = V_1 \oplus V_2$ is a direct sum of irreducible representations, with $V_1 \not\cong V_2$. Let $\phi: V \to V$ be an intertwining map. Using Schur's lemma, prove that $\phi(V_1) \subseteq V_1$ and $\phi(V_2) \subseteq V_2$, and that $\phi$ acts as a scalar on each summand. Conclude that $\text{End}_G(V) \cong \mathbb{C} \times \mathbb{C}$.

8. (Challenge) Let $\rho: G \to GL(V)$ be any representation over $\mathbb{C}$. Define the map $\pi: V \to V$ by $\pi(v) = \frac{1}{|G|} \sum_{g \in G} \rho(g)(v)$. Prove that $\pi$ is a $G$-equivariant projection onto the subspace $V^G = \{v \in V : \rho(g)(v) = v \text{ for all } g \in G\}$ of $G$-fixed vectors. Deduce that the multiplicity of the trivial representation in any representation $V$ equals $\dim V^G$, and compute $\dim V^G$ for the standard representation of $S_3$.
