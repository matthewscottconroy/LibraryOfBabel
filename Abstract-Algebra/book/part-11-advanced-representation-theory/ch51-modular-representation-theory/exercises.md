# Chapter 51 — Exercises

## Important Figures

- **Richard Brauer (1901–1977)** — founded modular representation theory; Brauer characters; defect groups; Brauer trees; the *modular atlas*
- **Jon Alperin (1937–)** — Alperin's weight conjecture; Alperin–McKay conjecture; local representation theory
- **Reinhold Baer (1902–1979)** — contributions to group rings in prime characteristic; Baer correspondence
- **James Alperin, Leonard Scott, & Jonathan Carlson** — module varieties; complexity and cohomological support

## References and Primary Sources

- **R. Brauer, "Representations of Finite Groups" (1963)** — in *Lectures on Modern Mathematics* (Wiley) — survey of the theory by its founder
- **J. Alperin, *Local Representation Theory* (Cambridge, 1986)** — blocks, defect groups, and local-global conjectures
- **D. Benson, *Representations and Cohomology* (2 vols., Cambridge, 1991)** — comprehensive treatment including cohomology of modules

## Examples, Applications, and Thought Experiments

- **$\mathbb{F}_2[\mathbb{Z}/2\mathbb{Z}]$: non-semisimple group ring** — over characteristic 2 for $G = \mathbb{Z}/2\mathbb{Z}$: $\mathbb{F}_2[G] \cong \mathbb{F}_2[x]/(x^2)$ (since $(x-1)^2 = x^2 - 1 = x^2 + 1$ in characteristic 2 and $g$ maps to $x$ with $g^2 = 1$); the module $\mathbb{F}_2[x]/(x-1)^2$ has a non-splitting composition series with two trivial summands; complete reducibility fails
- **Brauer characters** — analogous to ordinary characters but defined only for $p'$-elements (group elements of order not divisible by $p$); lift to characteristic 0 and then reduce; Brauer characters recover information about the ordinary character table "reduced mod $p$"; the number of Brauer characters = number of $p$-regular conjugacy classes
- **Blocks and defect groups** — the center $Z(k[G])$ decomposes $k[G] = B_1 \times \cdots \times B_r$ into blocks; each block $B_i$ has a $p$-subgroup $D_i$ (the defect group) measuring how far that block is from being semisimple; defect 0 means the block is a matrix algebra (the irreducible representations in that block are projective)
- **Thought experiment: modular representation as "shadow"** — ordinary representation theory over $\mathbb{C}$ is the "full picture"; modular representation theory over $\mathbb{F}_p$ is its "shadow" — some information is retained, some is lost; the lifts and reductions (Brauer character theory) formalize the relationship between the full picture and the shadow

## Exercises

1. Let $G = S_3$ and $p = 3$. The group has order 6, and $3 \mid 6$, so Maschke's theorem fails over $\mathbb{F}_3$. List the $p$-regular conjugacy classes of $G$ (those consisting of elements of order coprime to $p$), and determine the number of irreducible Brauer characters of $G$ in characteristic 3. Then compute the Brauer character of the tautological 2-dimensional representation of $S_3$ over $\mathbb{F}_3$ by evaluating its trace on each $p$-regular element.

2. Consider $G = S_4$ and $p = 2$. The ordinary character table of $S_4$ has five irreducible characters of degrees $1, 1, 2, 3, 3$. Determine the $2$-regular conjugacy classes, identify which ordinary characters are in the same $2$-block by computing the central characters modulo $2$, and find the decomposition matrix $D$ expressing each ordinary character restricted to $2$-regular elements as a $\mathbb{Z}_{\geq 0}$-linear combination of the irreducible Brauer characters.

3. Let $G = \mathbb{Z}/p\mathbb{Z}$ and $k = \mathbb{F}_p$. Show that $k[G] \cong k[x]/(x^p)$, and prove that the indecomposable $k[G]$-modules are precisely the modules $V_j = k[x]/(x^j)$ for $j = 1, \ldots, p$. Which of these are projective? Compute $\mathrm{Ext}^1_{k[G]}(k, k)$ and explain why this extension group is nonzero, demonstrating that the trivial module is not projective.

4. Let $B$ be a block of $k[G]$ with defect group $D$. Prove that if $D = 1$ (defect zero), then $B$ is a matrix algebra over $k$, and hence every module in $B$ is projective. Conclude that if $p \nmid |G|$, then $k[G]$ is semisimple. (This gives an independent proof of Maschke's theorem via block theory.)

5. Let $V$ be a $kG$-module, and let $P \to V \to 0$ be its projective cover. Define the Heller translate (syzygy) $\Omega V$ to be the kernel of this map. Show that $\Omega$ is well-defined up to isomorphism and defines an autoequivalence of the stable module category $\underline{\mathrm{mod}}(kG)$. Compute $\Omega^n(k)$ for $G = \mathbb{Z}/p\mathbb{Z}$ and $k = \mathbb{F}_p$ for small values of $n$.

6. State Green's theorem on vertices and sources: every indecomposable $kG$-module $M$ has a vertex $Q \leq G$ (a $p$-subgroup, minimal with the property that $M$ is relatively $Q$-projective) and a source (an indecomposable $kQ$-module $S$ with $M \mid \mathrm{Ind}_Q^G S$), both unique up to conjugacy. Use Green's theorem to show that if $M$ is an indecomposable $kG$-module with vertex $Q$, then $|Q|$ divides $\dim_k M$.

7. Let $\mathfrak{g}$ be a restricted Lie algebra over $\mathbb{F}_p$ with $[p]$-power map $x \mapsto x^{[p]}$. The restricted enveloping algebra is $u(\mathfrak{g}) = U(\mathfrak{g})/(x^p - x^{[p]} : x \in \mathfrak{g})$. Show that $\dim_k u(\mathfrak{g}) = p^{\dim \mathfrak{g}}$. For $\mathfrak{g} = \mathfrak{sl}_2$ over $\mathbb{F}_p$, describe the center of $u(\mathfrak{g})$ in terms of the $[p]$-power of the standard generators $e, f, h$, and state how many simple $u(\mathfrak{g})$-modules there are.

8. (Challenge) State and prove the Cartan–Brauer–Robinson theorem relating the Cartan matrix $C$ and decomposition matrix $D$ of a block: specifically, that $C = D^T D$ as integer matrices, where $D_{ij}$ counts the multiplicity of the $j$-th irreducible Brauer character in the reduction mod $p$ of the $i$-th ordinary irreducible character. Use this to deduce that $\det C$ is a power of $p$, and explain the significance of this integrality constraint for the structure of projective indecomposable modules.
