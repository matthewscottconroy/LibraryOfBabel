# Chapter 36 — Exercises

## Important Figures

- **Saunders Mac Lane & Samuel Eilenberg** — formalized limits and colimits in the categorical language; limits appeared implicitly in Grothendieck's earlier work
- **Alexander Grothendieck (1928–2014)** — inverse limits as pro-objects; used limits extensively in étale cohomology; the $p$-adic integers as an inverse limit is the prototype
- **F. William Lawvere (1937–2023)** — limits in topos theory; "elementary theory of limits"; categorical foundations

## References and Primary Sources

- **S. Mac Lane, *Categories for the Working Mathematician* (2nd ed., Springer, 1998)**, Ch. V — limits and colimits; complete and cocomplete categories
- **E. Riehl, *Category Theory in Context* (Dover, 2016)**, Ch. 3 — clear treatment with many examples
- **P. Johnstone, *Sketches of an Elephant: A Topos Theory Compendium* (Oxford, 2002)** — limits in topos theory

## Examples, Applications, and Thought Experiments

- **Products as limits** — the product $A \times B$ is the limit of the discrete two-object diagram $\{A, B\}$; the universal property: maps into $A \times B$ are exactly pairs of maps into $A$ and into $B$; works in $\mathbf{Set}$, $\mathbf{Grp}$, $\mathbf{Top}$, and any category with products
- **The pushout as gluing** — given maps $f: A \to B$ and $g: A \to C$, the pushout $B \sqcup_A C$ "glues" $B$ and $C$ along $A$; in topology, this is the attaching construction for CW complexes; in algebra, it is the amalgamated free product of groups
- **The $p$-adic integers as an inverse limit** — $\mathbb{Z}_p = \varprojlim \mathbb{Z}/p^n\mathbb{Z}$: elements are compatible sequences $(a_1, a_2, a_3, \ldots)$ with $a_n \in \mathbb{Z}/p^n\mathbb{Z}$ and $a_{n+1} \equiv a_n \pmod{p^n}$; the inverse limit "accumulates" all finite approximations into a complete object; this is the canonical example of a limit in algebra
- **Equalizers and fiber products** — the equalizer of $f, g: A \rightrightarrows B$ is $\{a \in A : f(a) = g(a)\}$; the pullback (fiber product) $A \times_C B$ for $f: A \to C$, $g: B \to C$ is $\{(a,b) : f(a) = g(b)\}$; both are limits of specific diagram shapes; kernels are equalizers with the zero map

## Exercises

1. Show that the terminal object (an object $T$ such that $\text{Hom}(A, T)$ has exactly one element for all $A$) is the limit of the empty diagram. Show that the initial object is the colimit of the empty diagram. Identify the terminal and initial objects in $\mathbf{Set}$, $\mathbf{Ab}$, and $\mathbf{Grp}$.

2. Show that the product $A \times B$ in a category, equipped with projections $\pi_1: A \times B \to A$ and $\pi_2: A \times B \to B$, is the limit of the discrete two-object diagram with vertices $A$ and $B$. Verify the universal property directly: for any object $C$ with morphisms $f: C \to A$ and $g: C \to B$, there is a unique morphism $(f, g): C \to A \times B$.

3. Compute the pullback $A \times_C B$ of the maps $f: A \to C$ and $g: B \to C$ in $\mathbf{Ab}$, where $A = B = C = \mathbb{Z}$, $f$ is multiplication by $m$, and $g$ is multiplication by $n$. Express your answer as a subgroup of $\mathbb{Z} \oplus \mathbb{Z}$.

4. Show that the equalizer of two group homomorphisms $f, g: G \rightrightarrows H$ is the subgroup $\{x \in G : f(x) = g(x)\}$, with the inclusion as the equalizing morphism. Verify the universal property.

5. Describe the pushout of two group homomorphisms $f: A \to B$ and $g: A \to C$ in $\mathbf{Grp}$. Show that it is the amalgamated free product $B *_A C$, the quotient of the free product $B * C$ by the normal closure of $\{f(a) g(a)^{-1} : a \in A\}$.

6. Show that $\mathbb{Z}_p = \varprojlim \mathbb{Z}/p^n\mathbb{Z}$ satisfies the universal property of the inverse limit: any compatible sequence of ring homomorphisms $\phi_n: R \to \mathbb{Z}/p^n\mathbb{Z}$ (with $\phi_n = \pi_n \circ \phi_{n+1}$) factors uniquely through $\mathbb{Z}_p$.

7. Let $F: \mathcal{C} \to \mathcal{D}$ be a right adjoint functor. Use the adjunction hom-set bijection to prove directly that $F$ preserves binary products: given a product $(A \times B, \pi_1, \pi_2)$ in $\mathcal{C}$, show that $(F(A \times B), F(\pi_1), F(\pi_2))$ satisfies the universal property of the product of $F(A)$ and $F(B)$ in $\mathcal{D}$.

8. (Challenge) Prove that any category with finite products and equalizers has all finite limits. Specifically, show that the limit of any finite diagram $D: \mathcal{J} \to \mathcal{C}$ can be constructed as the equalizer of two morphisms between products. Describe the two parallel morphisms explicitly.
