# Chapter 14 — Exercises

## Important Figures

- **Joseph-Louis Lagrange (1736–1813)** — proved the order of a subgroup divides the group order (for permutation groups, 1770–1771); the theorem now named for him
- **Évariste Galois (1811–1832)** — introduced normal subgroups explicitly as those whose left and right cosets coincide; normality is what his correspondence requires
- **Camille Jordan (1838–1922)** — systematic development of quotient groups; *Traité des substitutions* (1870)
- **Otto Hölder (1859–1937)** — studied the composition factors of a quotient series; Jordan–Hölder theorem

## References and Primary Sources

- **J.-L. Lagrange, "Réflexions sur la résolution algébrique des équations" (1770–1771)** — the proto-Lagrange theorem in the language of permutations
- **C. Jordan, *Traité des substitutions et des équations algébriques* (1870)** — systematic group theory; normal subgroups and quotient groups
- **I.N. Herstein, *Topics in Algebra* (2nd ed., Wiley, 1975)** — clean and efficient treatment

## Examples, Applications, and Thought Experiments

- **$\mathbb{Z}/n\mathbb{Z}$ as a quotient group** — $\mathbb{Z}/6\mathbb{Z}$ has 6 cosets: $0+6\mathbb{Z}, 1+6\mathbb{Z}, \ldots, 5+6\mathbb{Z}$; addition is defined by adding representatives and reducing mod 6; this is exactly modular arithmetic, now given a group-theoretic interpretation
- **Why normality is needed** — $H = \langle (12) \rangle \leq S_3$; the left cosets of $H$ are $\{e,(12)\}$, $\{(13),(132)\}$, $\{(23),(123)\}$, but these sets do not form a group under coset multiplication; normality fails; contrast with $A_3 \trianglelefteq S_3$ (index 2, hence always normal)
- **$\mathbb{R}/\mathbb{Z}$ is the circle group** — each coset $r + \mathbb{Z}$ is an equivalence class of real numbers with the same fractional part; the quotient group is $[0,1)$ with "addition mod 1"; this is the circle $\mathbb{R}/\mathbb{Z} \cong S^1$; quotient groups "forget" integer parts
- **Lagrange's theorem in number theory** — the order of $a \in (\mathbb{Z}/p\mathbb{Z})^*$ divides $p-1$ (Fermat's little theorem); Lagrange's theorem says the order of any element divides the order of the group; the group here is $(\mathbb{Z}/p\mathbb{Z})^*$, which has order $p-1$

## Exercises

1. List all left cosets and all right cosets of $H = \langle (123) \rangle$ in $S_3$. Are the left and right cosets the same? Is $H$ normal in $S_3$? Repeat the exercise for $H = \langle (12) \rangle$.

2. Use Lagrange's theorem to prove that if $G$ is a group of order $p$ where $p$ is prime, then $G$ is cyclic. Deduce that $G \cong \mathbb{Z}/p\mathbb{Z}$.

3. Let $G$ be a finite group and $H \leq G$. Prove that $[G:H] = |G|/|H|$ and use this to compute the index $[S_4 : A_4]$, $[D_6 : \langle r \rangle]$, and $[\text{GL}_2(\mathbb{F}_2) : \text{SL}_2(\mathbb{F}_2)]$.

4. Prove that any subgroup of index 2 is normal. Use this to show $A_n \trianglelefteq S_n$ for all $n \geq 2$.

5. Let $N \trianglelefteq G$ and $H \leq G$. Prove that $HN = \{hn : h \in H, n \in N\}$ is a subgroup of $G$. Is $HN$ normal in $G$ if $H$ is also normal?

6. Construct the quotient group $A_4 / V_4$ explicitly, where $V_4 = \{e, (12)(34), (13)(24), (14)(23)\}$ is the Klein four-group. Write out its multiplication table and identify which abstract group it is isomorphic to.

7. Let $G$ be a group of order $pq$ where $p$ and $q$ are distinct primes with $p < q$. Use Lagrange's theorem to show that the only possible orders for proper non-trivial subgroups are $p$ and $q$. If $q \not\equiv 1 \pmod{p}$, prove that $G$ is cyclic.

8. (Challenge) Prove Wilson's theorem: for any prime $p$, $(p-1)! \equiv -1 \pmod{p}$. Do this by considering the group $(\mathbb{Z}/p\mathbb{Z})^*$ and pairing each element with its inverse, noting which elements are their own inverses.
