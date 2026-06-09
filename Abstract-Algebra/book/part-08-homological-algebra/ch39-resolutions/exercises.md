# Chapter 39 — Exercises

## Important Figures

- **David Hilbert (1862–1943)** — Hilbert's syzygy theorem (1890): every finitely generated module over $k[x_1,\ldots,x_n]$ has a finite free resolution of length $\leq n$; the first resolution theorem
- **Henri Cartan & Samuel Eilenberg** — general resolution theory in *Homological Algebra* (1956); projective and injective resolutions as the systematic framework
- **Jean-Pierre Serre (1926–)** — used projective resolutions in algebraic geometry; Serre's characterization of regular local rings via finiteness of projective dimension

## References and Primary Sources

- **D. Hilbert, "Über die Theorie der algebraischen Formen" (1890)** — *Math. Ann.* 36 — syzygy theorem; the first finite resolution
- **H. Cartan & S. Eilenberg, *Homological Algebra* (Princeton, 1956)** — resolutions and derived functors
- **C. Weibel, *An Introduction to Homological Algebra* (Cambridge, 1994)**, Ch. 2 — resolutions; projective and injective dimensions

## Examples, Applications, and Thought Experiments

- **Free resolution of $\mathbb{Z}/n\mathbb{Z}$** — $0 \to \mathbb{Z} \xrightarrow{\times n} \mathbb{Z} \to \mathbb{Z}/n\mathbb{Z} \to 0$; the simplest non-trivial projective resolution; length 1; use it to compute $\text{Ext}^1_{\mathbb{Z}}(\mathbb{Z}/n\mathbb{Z}, M) \cong M/nM$
- **The Koszul complex** — for $R = k[x_1,\ldots,x_n]$ and the ideal $I = (x_1,\ldots,x_n)$: the Koszul complex $K_\bullet(x_1,\ldots,x_n)$ is a free resolution of $k = R/I$; it is the algebraic prototype for the de Rham complex; length $n$, matching Hilbert's syzygy theorem
- **Hilbert's syzygy theorem** — every finitely generated $k[x_1,\ldots,x_n]$-module $M$ has a free resolution $0 \to F_n \to \cdots \to F_1 \to F_0 \to M \to 0$ of length $\leq n$; this says $\text{pd}(M) \leq n$ for all $M$; the polynomial ring has "global dimension" $n$; this is the algebraic avatar of the geometric dimension of $\mathbb{A}^n$
- **Thought experiment: resolutions as scaffolding** — you want to study a complicated module $M$; you "scaffold" $M$ with a sequence of nice modules (free or projective) that approximate it from above; the resolution records what it takes to "build" $M$ from free modules; the derived functors then measure how "hard" $M$ is to build

## Exercises

1. Write down an explicit free resolution of $\mathbb{Z}/6\mathbb{Z}$ as a $\mathbb{Z}$-module. Then write down a free resolution of $\mathbb{Z}/2\mathbb{Z} \oplus \mathbb{Z}/3\mathbb{Z}$ as a $\mathbb{Z}$-module. Verify exactness at every term. What is the projective dimension of each module?

2. Let $R = k[x]$ for a field $k$ and let $M = k[x]/(x^2)$. Construct an explicit free resolution of $M$ over $R$. (Hint: the resolution is periodic.) Verify exactness. What does this say about the projective dimension of $M$?

3. Let $R = k[x,y]$ and $M = k = R/(x,y)$. Write down the Koszul complex $K_\bullet(x,y)$ explicitly: it has the form $0 \to R \to R^2 \to R \to 0$. Write down the differentials, verify $\partial^2 = 0$, and verify that the augmented complex $K_\bullet(x,y) \to k \to 0$ is exact. This gives a free resolution of $k$ of length 2.

4. Prove that every module $M$ over any ring $R$ admits a projective resolution. (Proceed inductively: show $M$ is a quotient of a projective module $P_0$, then apply the same argument to $\ker(P_0 \to M)$, and so on.)

5. Let $0 \to M' \to M \to M'' \to 0$ be a short exact sequence of $R$-modules. Suppose $M''$ has projective dimension $\leq d$ and $M'$ has projective dimension $\leq d$. Prove that $M$ has projective dimension $\leq d+1$. (Hint: use the horseshoe lemma to splice projective resolutions of $M'$ and $M''$ into a resolution of $M$.)

6. Show that $\mathbb{Q}$ as a $\mathbb{Z}$-module is not projective but is flat. (For non-projectivity: a projective module over $\mathbb{Z}$ is free, and $\mathbb{Q}$ is not free. For flatness: $\mathbb{Q} \otimes_{\mathbb{Z}} -$ is exact.) What does this tell you about the relationship between projectivity and flatness?

7. Let $R = \mathbb{Z}[G]$ be the group ring of a finite group $G$ and let $\mathbb{Z}$ be the trivial $R$-module. Using the bar resolution, write down the first three terms of a projective resolution of $\mathbb{Z}$ over $R$. (The bar resolution has $P_n = \mathbb{Z}[G^{n+1}]$ with $G$ acting diagonally.) Describe the differential $\partial: P_1 \to P_0$ explicitly.

8. (Challenge) Prove the Comparison Theorem: if $P_\bullet \to M$ and $Q_\bullet \to M$ are two projective resolutions of the same module $M$, then there exist chain maps $f: P_\bullet \to Q_\bullet$ and $g: Q_\bullet \to P_\bullet$ lying over the identity on $M$, and both $g \circ f$ and $f \circ g$ are chain homotopic to the respective identity maps. Conclude that $P_\bullet$ and $Q_\bullet$ are chain homotopy equivalent. (This is the foundation for the well-definedness of derived functors.)
