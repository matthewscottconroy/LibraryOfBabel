# Chapter 34 — Exercises

## Important Figures

- **Samuel Eilenberg & Saunders Mac Lane** — natural transformations defined in the founding 1945 paper; the concept was the original motivation for inventing category theory
- **Nobuo Yoneda (1930–1996)** — stated the Yoneda lemma (1954) to Mac Lane on a train at Tokyo Central Station; communicated at a conference; the lemma was never published by Yoneda himself
- **Alexander Grothendieck (1928–2014)** — used the Yoneda philosophy extensively in scheme theory: "a scheme is determined by its functor of points"; representable functors as a foundational tool

## References and Primary Sources

- **S. Eilenberg & S. Mac Lane, "General Theory of Natural Equivalences" (1945)** — natural transformations defined
- **S. Mac Lane, *Categories for the Working Mathematician* (2nd ed., Springer, 1998)**, Ch. III — Yoneda lemma and representable functors
- **E. Riehl, *Category Theory in Context* (Dover, 2016; freely available)** — especially good treatment of Yoneda

## Examples, Applications, and Thought Experiments

- **Naturality of the determinant** — $\det: \text{GL}_n(R) \to R^*$ is natural in $R$: for any ring homomorphism $\phi: R \to S$, $\det(\phi(A)) = \phi(\det(A))$; the determinant commutes with base change; this is not a coincidence but follows from naturality
- **The double-dual embedding** — $V \hookrightarrow V^{**}$ defined by $v \mapsto (\phi \mapsto \phi(v))$ is natural (no basis needed); $V \cong V^*$ requires choosing a basis (non-natural); this contrast — natural vs. non-natural isomorphism — is precisely the distinction Eilenberg and Mac Lane invented category theory to formalize
- **Yoneda: an object is its maps** — the Yoneda lemma says $\text{Nat}(\text{Hom}(A,-), F) \cong F(A)$ naturally; a natural transformation from the representable functor $\text{Hom}(A,-)$ to any functor $F$ is completely determined by a single element of $F(A)$; knowing all maps out of $A$ is equivalent to knowing $A$ itself
- **Thought experiment: the Yoneda perspective** — in everyday mathematics, we know a group $G$ by knowing all its homomorphisms into other groups; we know a topological space $X$ by knowing all continuous functions from test spaces into $X$; we know a scheme $X$ by knowing its $R$-points $X(R) = \text{Hom}(\text{Spec}(R), X)$ for all rings $R$; Yoneda says this perspective is not just convenient but complete

## Exercises

1. Let $F, G: \mathcal{C} \to \mathcal{D}$ be functors and $\alpha: F \Rightarrow G$ a family of morphisms $\alpha_A: F(A) \to G(A)$. Write out explicitly what the naturality condition demands, and verify that the identity natural transformation $\text{id}_F$ with $(\text{id}_F)_A = \text{id}_{F(A)}$ satisfies it.

2. Let $F = \text{id}_{\mathbf{Ab}}: \mathbf{Ab} \to \mathbf{Ab}$ and $G = (- \otimes_{\mathbb{Z}} \mathbb{Z}/2\mathbb{Z}): \mathbf{Ab} \to \mathbf{Ab}$. Define a natural transformation $\alpha: F \Rightarrow G$ by setting $\alpha_A: A \to A \otimes \mathbb{Z}/2\mathbb{Z}$ to be the map $a \mapsto a \otimes 1$. Verify the naturality condition: for any group homomorphism $f: A \to B$, confirm that the relevant square commutes.

3. Show that the abelianization functor $\text{Ab}: \mathbf{Grp} \to \mathbf{Ab}$, sending $G$ to $G/[G,G]$, together with the natural projection $\pi_G: G \to G/[G,G]$, defines a natural transformation from the forgetful functor $\mathbf{Ab} \to \mathbf{Grp}$ composed with $\text{Ab}$ to the identity.

4. For a finite-dimensional vector space $V$ over a field $k$, the double-dual embedding $\eta_V: V \to V^{**}$ defined by $\eta_V(v)(\phi) = \phi(v)$ gives a natural transformation $\eta: \text{Id}_{\mathbf{Vect}_k} \Rightarrow (-)^{**}$. Verify naturality: show that for any linear map $f: V \to W$, the square with $\eta_V$, $\eta_W$, $f$, and $f^{**}$ commutes. Conclude that $\eta$ is a natural isomorphism for finite-dimensional spaces.

5. Apply the Yoneda lemma to compute $\text{Nat}(\text{Hom}_{\mathbf{Ab}}(\mathbb{Z}, -), U)$ where $U: \mathbf{Ab} \to \mathbf{Set}$ is the forgetful functor. Identify the set this hom-set of natural transformations is in bijection with, and describe the bijection explicitly.

6. Use the Yoneda lemma to prove that if $\text{Hom}(A, -) \cong \text{Hom}(B, -)$ as functors (i.e., there is a natural isomorphism between them), then $A \cong B$ in $\mathcal{C}$. This is the *Yoneda principle*: objects are determined by their representable functors.

7. Show that a functor $F: \mathcal{C} \to \mathbf{Set}$ is representable — meaning $F \cong \text{Hom}(A, -)$ for some object $A$ — if and only if $F$ has an *initial element*: a pair $(A, u)$ with $u \in F(A)$ such that for every object $B$ and element $x \in F(B)$, there is a unique morphism $f: A \to B$ with $F(f)(u) = x$.

8. (Challenge) The Yoneda embedding $y: \mathcal{C} \to [\mathcal{C}^{\text{op}}, \mathbf{Set}]$ defined by $y(A) = \text{Hom}(-, A)$ is a functor. Prove that $y$ is full and faithful. Conclude that $\mathcal{C}$ embeds as a full subcategory of the presheaf category $[\mathcal{C}^{\text{op}}, \mathbf{Set}]$, and that this embedding reflects isomorphisms.
