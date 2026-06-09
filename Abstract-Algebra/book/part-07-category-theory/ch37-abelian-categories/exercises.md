# Chapter 37 — Exercises

## Important Figures

- **Alexander Grothendieck (1928–2014)** — *Tôhoku paper* "Sur quelques points d'algèbre homologique" (1957): formalized abelian categories; the axioms AB1–AB6; revolutionary for sheaf cohomology
- **David Buchsbaum (1929–2021)** — "Exact Categories and Duality" (1955): preliminary formulation of abelian categories (called "exact categories")
- **Barry Mitchell (1928–2002) & Peter Freyd (1936–)** — Freyd–Mitchell full embedding theorem: every small abelian category embeds fully and exactly into $R\text{-Mod}$ for some ring $R$; justifies element-chasing arguments

## References and Primary Sources

- **A. Grothendieck, "Sur quelques points d'algèbre homologique" (1957)** — *Tôhoku Math. J.* 9 — the founding text; known as the "Tôhoku paper"
- **P. Freyd, *Abelian Categories* (Harper & Row, 1964; reprint TAC, 2003)** — Freyd–Mitchell embedding theorem
- **C. Weibel, *An Introduction to Homological Algebra* (Cambridge, 1994)**, Ch. 1 — abelian categories for homological algebra

## Examples, Applications, and Thought Experiments

- **$R\text{-Mod}$ is the prototypical abelian category** — kernels and cokernels always exist; every morphism factors as epi (surjection) followed by mono (injection); direct sums and products coincide for finite collections; all the axioms can be verified concretely
- **Sheaves on a space form an abelian category** — the abelian category $\text{Sh}(X, \mathbf{Ab})$; the global sections functor $\Gamma: \text{Sh}(X) \to \mathbf{Ab}$ is left exact but not right exact; the failure is measured by sheaf cohomology $H^n(X, \mathcal{F})$; this failure is what Grothendieck's theory captures
- **The snake lemma** — given a commutative diagram with exact rows $0 \to A' \to A \to A'' \to 0$ and maps to $0 \to B' \to B \to B'' \to 0$, there is a natural exact sequence $\ker(A' \to B') \to \ker(A \to B) \to \ker(A'' \to B'') \xrightarrow{\delta} \text{coker}(A' \to B') \to \cdots$; holds in any abelian category; the connecting homomorphism $\delta$ is the "snake"
- **Thought experiment: why "abelian"?** — the abelian category axioms are exactly the conditions needed for the homological machinery (long exact sequences, derived functors, spectral sequences) to work; they are not arbitrary but the minimal conditions guaranteeing that "linear algebra over a category" makes sense; the Tôhoku paper asked: "what is the minimal framework for homological algebra?" and answered it

## Exercises

1. Show that $\mathbf{Ab}$ is an abelian category by verifying each layer of axioms: show that each hom-set $\text{Hom}(A, B)$ is an abelian group under pointwise addition, that biproducts exist (the direct sum $A \oplus B$ with the canonical inclusion and projection maps), and that kernels and cokernels exist for every homomorphism.

2. Let $\mathcal{C}$ be the category of finitely generated abelian groups. Show that $\mathcal{C}$ is an abelian category. Is $\mathcal{C}$ closed under infinite direct sums? Does $\mathcal{C}$ have all products?

3. Show that the category of free abelian groups is *not* an abelian category: find a morphism whose cokernel does not exist within the category of free abelian groups.

4. Prove the *short five lemma* in an abelian category: given a commutative diagram with exact rows $0 \to A \to B \to C \to 0$ and $0 \to A' \to B' \to C' \to 0$ and vertical maps $\alpha: A \to A'$, $\beta: B \to B'$, $\gamma: C \to C'$, show that if $\alpha$ and $\gamma$ are isomorphisms then $\beta$ is an isomorphism.

5. Apply the snake lemma to the short exact sequence $0 \to \mathbb{Z} \xrightarrow{\times n} \mathbb{Z} \to \mathbb{Z}/n\mathbb{Z} \to 0$ and the short exact sequence $0 \to \mathbb{Z} \xrightarrow{\times m} \mathbb{Z} \to \mathbb{Z}/m\mathbb{Z} \to 0$, connected by the map $\mathbb{Z} \xrightarrow{\times m/\gcd(m,n)} \mathbb{Z}$ (when $\gcd(m,n) | m$). Write down the resulting long exact sequence explicitly.

6. A functor $F: \mathcal{A} \to \mathcal{B}$ between abelian categories is *exact* if it preserves short exact sequences. Show that $F$ is exact if and only if it is both left exact (preserves kernels) and right exact (preserves cokernels). Give an example of a functor that is left exact but not right exact.

7. Prove the *nine lemma* (also called the $3 \times 3$ lemma): given a commutative diagram of abelian groups (or objects of any abelian category) in which all three columns are exact and two of the three rows are exact, the third row is also exact. (You may use the snake lemma.)

8. (Challenge) Let $\mathcal{A}$ be an abelian category and $f: A \to B$ a morphism. Define the *image* of $f$ as $\text{im}(f) = \ker(\text{coker}(f))$ and the *coimage* as $\text{coim}(f) = \text{coker}(\ker(f))$. Show that the abelian category axiom — that the canonical map $\text{coim}(f) \to \text{im}(f)$ is an isomorphism — implies the first isomorphism theorem: $A / \ker(f) \cong \text{im}(f)$. Verify this in $R\text{-Mod}$ by computing both the image and coimage explicitly.
