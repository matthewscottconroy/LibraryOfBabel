# Chapter 17 — Exercises

## Important Figures

- **Peter Ludwig Sylow (1832–1918)** — proved all three theorems in a single paper (1872): existence, conjugacy, and congruence conditions on the number of Sylow subgroups; one of the most productive single papers in group theory
- **Ferdinand Georg Frobenius (1849–1917)** — gave alternative proofs; proved that if $d \mid |G|$ then $d \mid |\{g \in G : g^d = e\}|$; Frobenius normal $p$-complement theorem
- **Philip Hall (1904–1982)** — Hall's generalization of Sylow to solvable groups: for solvable $G$, subgroups of order $\prod p_i^{a_i}$ (for any subset of primes) exist and are conjugate

## References and Primary Sources

- **P.L.M. Sylow, "Théorèmes sur les groupes de substitutions" (1872)** — *Math. Ann.* — original paper; surprisingly readable
- **G. Frobenius, "Über auflösbare Gruppen" (1893)** — Frobenius' proofs and extensions
- **M. Isaacs, *Finite Group Theory* (AMS, 2008)** — several proofs of the Sylow theorems; comprehensive

## Examples, Applications, and Thought Experiments

- **Groups of order 15 are cyclic** — $n_3 \mid 5$ and $n_3 \equiv 1 \pmod{3}$ forces $n_3 = 1$; $n_5 \mid 3$ and $n_5 \equiv 1 \pmod{5}$ forces $n_5 = 1$; unique Sylow subgroups are normal; their intersection is trivial; $G \cong \mathbb{Z}/3\mathbb{Z} \times \mathbb{Z}/5\mathbb{Z} \cong \mathbb{Z}/15\mathbb{Z}$
- **Groups of order $p^2$ are abelian** — the class equation forces $|Z(G)| \geq p$; if $|Z(G)| = p^2$ we are done; if $|Z(G)| = p$ then $G/Z(G) \cong \mathbb{Z}/p\mathbb{Z}$ is cyclic; but if $G/Z(G)$ is cyclic, $G$ is abelian — a contradiction; so $|Z(G)| = p^2$ and $G$ is abelian
- **Thought experiment: the Sylow subgroup as a "shadow"** — a Sylow $p$-subgroup $P$ is the largest $p$-power piece of $G$; the number $n_p$ measures how "spread out" these pieces are; $n_p = 1$ means $P$ is a well-defined, canonical normal subgroup; the Sylow theorems are a systematic way to extract structure from the prime factorization of $|G|$
- **Simple group obstruction** — to show a group $G$ of order $n$ is not simple: find a Sylow subgroup $P$ such that $n_p = 1$ (hence $P \trianglelefteq G$); e.g., any group of order 200 has a normal Sylow 5-subgroup since $n_5 \mid 8$ and $n_5 \equiv 1 \pmod 5$ force $n_5 = 1$

## Exercises

1. Let $G$ be a group of order 12. Show that $n_3 \in \{1, 4\}$ and $n_2 \in \{1, 3\}$. Analyze both cases for $n_3$: if $n_3 = 4$, show that $G$ contains at least 8 elements of order 3, and deduce constraints on the Sylow 2-subgroup. Classify all groups of order 12 up to isomorphism.

2. Prove that every group of order 15 is cyclic. More generally, prove that if $|G| = pq$ with primes $p < q$ and $p \nmid q-1$, then $G \cong \mathbb{Z}/pq\mathbb{Z}$.

3. Show that no group of order 30 is simple. Find all possible values of $n_2$, $n_3$, $n_5$ and derive a contradiction from each scenario that would require $n_5 > 1$ and $n_3 > 1$ simultaneously.

4. Prove that every group of order $p^2$ (for $p$ prime) is abelian, and conclude that it is isomorphic to either $\mathbb{Z}/p^2\mathbb{Z}$ or $\mathbb{Z}/p\mathbb{Z} \times \mathbb{Z}/p\mathbb{Z}$.

5. Let $G$ be a group of order 20. Determine all possible values of $n_5$ and $n_2$. Show that $n_5 = 1$ always, and classify all groups of order 20 by analyzing the action of the Sylow 2-subgroup on the normal Sylow 5-subgroup.

6. Prove that any group of order $p^n m$ where $p \nmid m$ and $m < p$ must have a normal Sylow $p$-subgroup. Give an explicit family of group orders to which this applies.

7. Let $P$ be a Sylow $p$-subgroup of $G$. Prove that $N_G(N_G(P)) = N_G(P)$. (The normalizer of the normalizer is the normalizer itself.) What does this imply about the number of conjugates of $N_G(P)$?

8. (Challenge) Let $G$ be a simple group of order 60. Use Sylow analysis to show that $n_5 = 6$ and $n_3 \in \{4, 10\}$ and $n_2 = 5$. Deduce that $G$ has 24 elements of order 5, and conclude that $G$ embeds in $S_5$ via the conjugation action on its Sylow 5-subgroups. Using the fact that $|S_5| = 120 = 2 \cdot 60$, show $G \cong A_5$.
