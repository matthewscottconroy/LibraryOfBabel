# Chapter 18 — Exercises

## Important Figures

- **Camille Jordan (1838–1922)** — Jordan normal series; Jordan's version of the composition uniqueness theorem (1869)
- **Otto Hölder (1859–1937)** — completed Jordan's result; the Jordan–Hölder theorem in full generality (1889); studied composition factors
- **William Burnside (1852–1927)** — proved groups of order $p^a q^b$ are solvable (1904) using character theory; the Burnside $p^a q^b$ theorem
- **Walter Feit (1930–2004) & John Thompson (1932–)** — Feit–Thompson Theorem (1963): every group of odd order is solvable; the 255-page proof opened the era of the Classification of Finite Simple Groups

## References and Primary Sources

- **W. Feit & J.G. Thompson, "Solvability of Groups of Odd Order" (1963)** — *Pacific J. Math.* 13(3) — landmark; launched CFSG
- **D. Gorenstein, *Finite Groups* (2nd ed., Chelsea, 1980)** — toward the classification
- **J.J. Rotman, *An Introduction to the Theory of Groups* (4th ed., Springer, 1995)**, Chs. 5–6 — solvable and nilpotent groups

## Examples, Applications, and Thought Experiments

- **Composition series of $S_4$** — $S_4 \supset A_4 \supset V_4 \supset \mathbb{Z}/2\mathbb{Z} \supset \{e\}$; the composition factors are $\mathbb{Z}/2\mathbb{Z}, \mathbb{Z}/3\mathbb{Z}, \mathbb{Z}/2\mathbb{Z}, \mathbb{Z}/2\mathbb{Z}$; Jordan–Hölder guarantees any other composition series gives the same multiset of factors; this is the unique "prime factorization" of $S_4$
- **The derived series of $S_4$ vs. $S_5$** — $S_4^{(0)} = S_4 \supset A_4 \supset V_4 \supset \{e\}$: reaches $\{e\}$, so $S_4$ is solvable; $S_5^{(1)} = A_5$ is simple and non-abelian, so $A_5^{(1)} = A_5$; the derived series never reaches $\{e\}$; $S_5$ is not solvable — this is why the quintic has no radical formula
- **Free group $F_2 = \langle a, b \rangle$** — every element is a reduced word in $a, b, a^{-1}, b^{-1}$; this group surjects onto any 2-generator group; it is "maximally non-abelian"; the commutator $[a,b] = aba^{-1}b^{-1}$ is non-trivial, measuring non-commutativity
- **Nilpotent groups as iterated central extensions** — a group is nilpotent iff its lower central series $G = G_0 \supset G_1 = [G,G_0] \supset G_2 = [G,G_1] \supset \cdots$ reaches $\{e\}$; $p$-groups are always nilpotent; think of a nilpotent group as one that can be built by successive abelian extensions

## Exercises

1. Find a composition series for $A_4$ and verify the Jordan–Hölder theorem by finding a second composition series and checking that the multisets of composition factors are equal.

2. Compute the derived series $G^{(0)} \supset G^{(1)} \supset G^{(2)} \supset \cdots$ for $G = D_8$ (dihedral group of order 8) and $G = A_4$. In each case, determine whether $G$ is solvable and identify the length of the derived series.

3. Prove that every subgroup and every quotient of a solvable group is solvable. Show by example (using $A_5$ and a suitable larger group) that a group with a solvable normal subgroup and solvable quotient need not itself be solvable. (Such a group would be a non-split extension.)

4. Compute the lower central series and upper central series of the dihedral group $D_{2^n}$ for small $n$. Determine for which $n$ the group is nilpotent. Compare with the quaternion group $Q_8$.

5. Show that every finite $p$-group is nilpotent. (Hint: use the fact that a non-trivial $p$-group has a non-trivial center, and induct on the order of the group by passing to $G/Z(G)$.)

6. Write the dihedral group $D_5$, the quaternion group $Q_8$, and the symmetric group $S_4$ as group presentations $\langle S \mid R \rangle$. For each, verify that the given relations are consistent with the group's multiplication table, and show that no additional relations are needed by counting elements.

7. Let $G = \langle a, b \mid a^3 = b^3 = e,\, ab = ba \rangle$. Identify this group up to isomorphism. Now consider $H = \langle a, b \mid a^4 = b^2 = e,\, bab^{-1} = a^{-1} \rangle$. Identify $H$ and prove your answer.

8. (Challenge) Prove the Jordan–Hölder Theorem: any two composition series of a finite group $G$ have the same length and the same multiset of composition factors up to isomorphism. You may use the Zassenhaus butterfly lemma (also called the Zassenhaus isomorphism lemma), which states that for subgroups $A \trianglelefteq B$ and $C \trianglelefteq D$ of a group, $(A(B \cap D))/(A(B \cap C)) \cong (C(D \cap B))/(C(D \cap A))$.
