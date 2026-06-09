# Chapter 28 — Exercises

## Important Figures

- **Woldemar Voigt (1850–1919)** — introduced tensors in physics (1898); tensor products in the context of multilinear algebra
- **Nicolas Bourbaki (collective, founded 1935)** — systematic, rigorous treatment of tensor products in *Algèbre* (Chs. 2–3); the algebraist's definitive reference
- **Alexander Grothendieck (1928–2014)** — tensor products of sheaves and their derived functors; $\mathbf{L} \otimes^{\mathbf{L}}$ in derived categories; used tensor products to build K-theory

## References and Primary Sources

- **N. Bourbaki, *Algèbre*, Ch. 2: "Free Algebras, Tensor Products" (Hermann, 1942–)** — the canonical algebraic reference
- **C. Weibel, *An Introduction to Homological Algebra* (Cambridge, 1994)**, Ch. 2 — Tor and the tensor product
- **S. Lang, *Algebra* (rev. 3rd ed., Springer, 2002)**, Ch. XVI

## Examples, Applications, and Thought Experiments

- **$\mathbb{Z}/2\mathbb{Z} \otimes_{\mathbb{Z}} \mathbb{Z}/3\mathbb{Z} = 0$** — in this tensor product, $m \otimes n = m \otimes 3 \cdot \frac{n}{3}$... For any $m \otimes n$: $m \otimes n = m \otimes (3 \cdot \frac{n}{3}) = 3m \otimes \frac{n}{3} = 0 \otimes \frac{n}{3} = 0$; torsion of coprime orders annihilates in tensor products
- **Base change** — $\mathbb{R} \otimes_{\mathbb{Q}} \mathbb{Q}(\sqrt{2}) \cong \mathbb{R} \times \mathbb{R}$; this is because $x^2 - 2 = (x-\sqrt{2})(x+\sqrt{2})$ splits completely over $\mathbb{R}$; by CRT, $\mathbb{R}[x]/(x^2-2) \cong \mathbb{R} \times \mathbb{R}$; tensoring "extends scalars" and can split previously non-split extensions
- **Hom–Tensor adjunction** — $\text{Hom}_R(M \otimes_R N, P) \cong \text{Hom}_R(M, \text{Hom}_R(N, P))$ naturally; this is the adjunction $(- \otimes N) \dashv \text{Hom}(N, -)$; it is the source of the Tor/Ext relationship and underlies all of derived algebra
- **Tensor product of algebras** — $\mathbb{C} \otimes_{\mathbb{R}} \mathbb{C} \cong \mathbb{C} \times \mathbb{C}$ (since $\mathbb{C}[x]/(x^2+1)$ and $x^2+1 = (x-i)(x+i)$ over $\mathbb{C}$); in contrast, $\mathbb{Q}(\sqrt{2}) \otimes_{\mathbb{Q}} \mathbb{Q}(\sqrt{3}) \cong \mathbb{Q}(\sqrt{2}, \sqrt{3})$; the tensor product is the algebraic "meeting point" of two algebras

## Exercises

1. State the universal property of the tensor product $M \otimes_R N$. Use it to construct, for any $R$-modules $M$, $N$, $P$, a natural isomorphism $(M \oplus N) \otimes_R P \cong (M \otimes_R P) \oplus (N \otimes_R P)$. Explain why this follows from the universal property without any explicit computation.

2. Compute $\mathbb{Z}/m\mathbb{Z} \otimes_{\mathbb{Z}} \mathbb{Z}/n\mathbb{Z}$ for arbitrary positive integers $m$ and $n$. Show that this tensor product is isomorphic to $\mathbb{Z}/\gcd(m,n)\mathbb{Z}$. As a special case, verify that $\mathbb{Z}/4\mathbb{Z} \otimes_{\mathbb{Z}} \mathbb{Z}/6\mathbb{Z} \cong \mathbb{Z}/2\mathbb{Z}$ and that $\mathbb{Z}/4\mathbb{Z} \otimes_{\mathbb{Z}} \mathbb{Z}/9\mathbb{Z} = 0$.

3. Let $R$ be a commutative ring and $I$, $J$ ideals of $R$. Show that $(R/I) \otimes_R (R/J) \cong R/(I + J)$. Apply this to compute $\mathbb{Z}[x]/(x^2 - 1) \otimes_{\mathbb{Z}[x]} \mathbb{Z}[x]/(x - 1)$ and identify the resulting ring.

4. Prove that the tensor product functor $- \otimes_R M$ is right exact: given a short exact sequence $0 \to A \xrightarrow{f} B \xrightarrow{g} C \to 0$ of $R$-modules, the sequence $A \otimes_R M \xrightarrow{f \otimes 1} B \otimes_R M \xrightarrow{g \otimes 1} C \otimes_R M \to 0$ is exact. Give an explicit example showing that $f \otimes 1$ need not be injective by taking $A = \mathbb{Z}$, $B = \mathbb{Z}$, $f$ multiplication by $2$, and $M = \mathbb{Z}/2\mathbb{Z}$.

5. A module $M$ over a commutative ring $R$ is flat if $- \otimes_R M$ is exact. Show that $\mathbb{Q}$ is a flat $\mathbb{Z}$-module by showing that tensoring the injection $\mathbb{Z} \hookrightarrow \mathbb{Q}$ with $\mathbb{Q}$ yields an injection $\mathbb{Q} \otimes_{\mathbb{Z}} \mathbb{Z} \to \mathbb{Q} \otimes_{\mathbb{Z}} \mathbb{Q}$. Then show that $\mathbb{Z}/n\mathbb{Z}$ is not flat for any $n \geq 2$ by exhibiting an injection that becomes non-injective after tensoring.

6. State the Hom–Tensor adjunction: for a right $R$-module $M$, a left $R$-module $N$, and an abelian group $P$, there is a natural isomorphism $\text{Hom}_{\mathbb{Z}}(M \otimes_R N, P) \cong \text{Hom}_R(M, \text{Hom}_{\mathbb{Z}}(N, P))$. Use this adjunction to deduce that $\text{Hom}$ is left exact: if $0 \to A \to B \to C$ is exact, then $0 \to \text{Hom}_R(C, P) \to \text{Hom}_R(B, P) \to \text{Hom}_R(A, P)$ is exact.

7. Let $F$ be a field and $V$, $W$ finite-dimensional $F$-vector spaces with bases $\{v_1, \ldots, v_m\}$ and $\{w_1, \ldots, w_n\}$. Show that $V \otimes_F W$ has basis $\{v_i \otimes w_j\}$ and hence $\dim_F(V \otimes_F W) = mn$. Then identify $V \otimes_F W$ with $\text{Hom}_F(V^*, W)$ where $V^*$ denotes the dual space, and explain the relationship to the space of $m \times n$ matrices over $F$.

8. (Challenge) Let $R$ be a commutative ring and $M$ a finitely presented $R$-module (presented by a matrix). Prove that $M$ is flat if and only if it is locally free: for every prime ideal $\mathfrak{p} \subseteq R$, the localization $M_{\mathfrak{p}}$ is a free $R_{\mathfrak{p}}$-module. As a starting point, show that any finitely generated flat module over a local ring is free, and use the fact that flatness is a local property.
