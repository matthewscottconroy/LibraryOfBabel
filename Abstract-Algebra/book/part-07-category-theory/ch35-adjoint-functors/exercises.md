# Chapter 35 — Exercises

## Important Figures

- **Daniel Kan (1927–2013)** — introduced adjoint functors (1958) in his work on simplicial sets and homotopy theory; the paper "Adjoint Functors" is the founding text
- **Saunders Mac Lane (1909–2005)** — formalized and popularized adjoints in *Categories for the Working Mathematician*; "adjoint functors arise everywhere"
- **F. William Lawvere (1937–2023)** — "Adjointness in Foundations" (1969): argued that adjoint functors are the unifying concept of all of mathematics, not just a technical tool

## References and Primary Sources

- **D. Kan, "Adjoint Functors" (1958)** — *Trans. Amer. Math. Soc.* 87 — original paper
- **S. Mac Lane, *Categories for the Working Mathematician* (2nd ed., Springer, 1998)**, Ch. IV — adjunctions and their applications
- **F.W. Lawvere, "Adjointness in Foundations" (1969)** — *Dialectica* 23 — philosophical and mathematical case for adjunctions

## Examples, Applications, and Thought Experiments

- **Free group $\dashv$ forgetful functor** — the free group functor $F: \mathbf{Set} \to \mathbf{Grp}$ is left adjoint to the forgetful functor $U: \mathbf{Grp} \to \mathbf{Set}$; the adjunction says $\text{Hom}_{\mathbf{Grp}}(F(S), G) \cong \text{Hom}_{\mathbf{Set}}(S, U(G))$ naturally; to specify a homomorphism from a free group you only need to say where the generators go
- **Tensor $\dashv$ Hom** — $(- \otimes_R N) \dashv \text{Hom}_R(N, -)$: maps out of a tensor product are bilinear maps, which correspond to linear maps into a Hom-set; this single adjunction generates most of homological algebra
- **Quantifiers as adjoints** — in logic, $\exists$ (left adjoint to substitution) and $\forall$ (right adjoint to substitution) are adjoints to the pullback functor on predicates; Lawvere's insight: the logical quantifiers are categorical adjoints; logic and category theory are the same subject at different levels
- **Preservation of limits/colimits** — right adjoints preserve limits: e.g., the forgetful functor $\mathbf{Grp} \to \mathbf{Set}$ (right adjoint to free) preserves products; left adjoints preserve colimits: free group functor preserves coproducts (free product of free groups is free); this is a structurally clean explanation of many standard theorems

## Exercises

1. Verify the free-forgetful adjunction explicitly: let $S$ be a set and $G$ a group. Show that the map $\text{Hom}_{\mathbf{Grp}}(F(S), G) \to \text{Hom}_{\mathbf{Set}}(S, U(G))$ given by restricting a homomorphism to the generators is a bijection. Show it is natural in $S$ and $G$.

2. Identify the unit $\eta_S: S \to U(F(S))$ and counit $\varepsilon_G: F(U(G)) \to G$ of the free-forgetful adjunction. Verify the triangle identities: $\varepsilon_{F(S)} \circ F(\eta_S) = \text{id}_{F(S)}$ and $U(\varepsilon_G) \circ \eta_{U(G)} = \text{id}_{U(G)}$.

3. Show that the abelianization functor $\text{Ab}: \mathbf{Grp} \to \mathbf{Ab}$, sending $G$ to $G/[G,G]$, is left adjoint to the inclusion functor $i: \mathbf{Ab} \hookrightarrow \mathbf{Grp}$. Write down the adjunction bijection $\text{Hom}_{\mathbf{Ab}}(\text{Ab}(G), A) \cong \text{Hom}_{\mathbf{Grp}}(G, i(A))$ and verify naturality.

4. Use the tensor-hom adjunction $\text{Hom}_R(M \otimes_R N, P) \cong \text{Hom}_R(M, \text{Hom}_R(N, P))$ to compute $\text{Hom}_{\mathbb{Z}}(\mathbb{Z}/m\mathbb{Z} \otimes_{\mathbb{Z}} \mathbb{Z}/n\mathbb{Z}, A)$ for an abelian group $A$ in terms of $\text{Hom}(\mathbb{Z}/\gcd(m,n)\mathbb{Z}, A)$.

5. Prove that if $F \dashv G$, then $F$ preserves all colimits that exist in $\mathcal{C}$. Specifically, show that if $D: \mathcal{J} \to \mathcal{C}$ is a diagram with colimit $(L, \{\iota_j\})$, then $(F(L), \{F(\iota_j)\})$ is the colimit of $F \circ D$ in $\mathcal{D}$.

6. Show that adjoints are unique up to natural isomorphism: if $F \dashv G$ and $F \dashv G'$, then there is a natural isomorphism $G \cong G'$. Similarly, if $F \dashv G$ and $F' \dashv G$, then $F \cong F'$.

7. Let $R$ be a commutative ring and $f: R \to S$ a ring homomorphism. The restriction-of-scalars functor $f^*: \mathbf{Mod}_S \to \mathbf{Mod}_R$ is right adjoint to the extension-of-scalars functor $f_* = S \otimes_R -: \mathbf{Mod}_R \to \mathbf{Mod}_S$. Verify the adjunction bijection $\text{Hom}_S(S \otimes_R M, N) \cong \text{Hom}_R(M, f^*(N))$ for an $R$-module $M$ and $S$-module $N$.

8. (Challenge) Prove that the forgetful functor $U: \mathbf{CRing} \to \mathbf{Set}$ from commutative rings to sets has a left adjoint — the polynomial ring functor $F(S) = \mathbb{Z}[x_s : s \in S]$. Write down the adjunction bijection and verify the triangle identities for the unit $\eta_S: S \to U(\mathbb{Z}[x_s : s \in S])$ (sending $s$ to $x_s$) and counit $\varepsilon_R: \mathbb{Z}[x_r : r \in R] \to R$ (sending $x_r$ to $r$).
