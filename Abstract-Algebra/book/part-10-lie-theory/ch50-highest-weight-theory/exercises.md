# Chapter 50 — Exercises

## Important Figures

- **Élie Cartan (1869–1951)** — highest weight classification of irreducible representations of semisimple Lie algebras (1913)
- **Hermann Weyl (1885–1955)** — Weyl character formula (1925–1926); Weyl dimension formula; Weyl integration formula; the complete theory of representations of compact Lie groups
- **Bertram Kostant (1928–2017)** — Kostant multiplicity formula; partition function; Borel–Weil–Bott theorem
- **Victor Kac (1943–)** — Kac–Moody algebras (1968): infinite-dimensional Lie algebras extending highest weight theory; now central to conformal field theory

## References and Primary Sources

- **H. Weyl, "Theorie der Darstellung kontinuierlicher halb-einfacher Gruppen durch lineare Transformationen" (1925–1926)** — *Math. Z.* 23–24 — Weyl's character formula
- **J. Humphreys, *Introduction to Lie Algebras and Representation Theory* (Springer, 1972)**, Ch. VI — highest weight modules
- **W. Fulton & J. Harris, *Representation Theory: A First Course* (Springer, 1991)** — intuitive and geometric treatment with many examples

## Examples, Applications, and Thought Experiments

- **Representations of $\mathfrak{sl}_2(\mathbb{C})$** — irreps $V(n)$ of dimension $n+1$ for $n \in \mathbb{N}$; highest weight $n$; weight spaces $\{n, n-2, \ldots, -n\}$; the raising operator $e$ and lowering operator $f$ move between weight spaces; the entire representation is determined by the single highest weight vector
- **The standard and adjoint representations of $\mathfrak{sl}_3$** — standard: highest weight $(1,0)$; 3-dimensional; the weight diagram is an equilateral triangle; adjoint: highest weight $(1,1)$; 8-dimensional; the weight diagram is a hexagon with a central point; $\mathfrak{sl}_3 \cong A_2$
- **Quarks and $SU(3)$ flavor symmetry** — the 3-dimensional representation of $SU(3)$ labels the three quark flavors (up, down, strange); the 8-dimensional adjoint representation labels the eight gluons; highest weight theory provides the mathematical foundation for the quark model in particle physics
- **Weyl dimension formula** — $\dim V(\lambda) = \prod_{\alpha > 0} \frac{\langle \lambda + \rho, \alpha \rangle}{\langle \rho, \alpha \rangle}$ where $\rho =$ half-sum of positive roots; for $\mathfrak{sl}_3$: $\dim V(m,n) = \frac{(m+1)(n+1)(m+n+2)}{2}$; the formula encodes the full dimension in a ratio of inner products

## Exercises

1. For $\mathfrak{sl}_2(\mathbb{C})$ with standard basis $\{e, f, h\}$, explicitly construct the irreducible representation $V(3)$ of highest weight $3$: find a basis of weight vectors $v_3, v_1, v_{-1}, v_{-3}$, write down the action of $e$, $f$, and $h$ on each basis vector, and verify that the representation is irreducible. Draw the weight diagram.

2. Using the Weyl dimension formula $\dim L(\lambda) = \prod_{\alpha > 0} \frac{\langle \lambda + \rho, \alpha \rangle}{\langle \rho, \alpha \rangle}$, compute the dimensions of the following irreducible $\mathfrak{sl}_3(\mathbb{C})$-modules (with $\rho = \omega_1 + \omega_2$): (a) $L(\omega_1)$; (b) $L(\omega_2)$; (c) $L(2\omega_1)$; (d) $L(\omega_1 + \omega_2)$; (e) $L(2\omega_1 + \omega_2)$.

3. Apply the Clebsch–Gordan formula to decompose the following tensor products of $\mathfrak{sl}_2(\mathbb{C})$-modules into irreducibles: (a) $V(2) \otimes V(2)$; (b) $V(3) \otimes V(1)$; (c) $V(2) \otimes V(2) \otimes V(1)$ (decompose the last by iterating). Verify the dimensions on both sides.

4. Decompose the tensor product $L(\omega_1) \otimes L(\omega_1)$ for $\mathfrak{sl}_3(\mathbb{C})$ (the tensor square of the standard 3-dimensional representation) into irreducible submodules. Identify the symmetric and exterior square as representations, give their highest weights, and verify dimensions.

5. Let $\lambda = n\omega_1$ be a dominant integral weight for $\mathfrak{sl}_2(\mathbb{C})$ (so $L(\lambda) = V(n)$). Apply the Weyl character formula to compute the formal character $\text{ch}(V(n)) = \sum_{k=-n}^{n} e^{k\alpha/2}$ (where $\alpha$ is the positive root) and verify that specializing to $e^\alpha = t$ gives the character polynomial $t^n + t^{n-2} + \cdots + t^{-n}$. Use this to verify the dimension formula for $V(n)$.

6. Verify that the highest weight of the adjoint representation of $\mathfrak{sl}_3(\mathbb{C})$ is $\alpha_1 + \alpha_2 = \omega_1 + \omega_2$. Draw the weight diagram (listing all weights with multiplicities), confirm the 8-dimensional count, and identify which weights are roots and which is the zero weight.

7. For $\mathfrak{sl}_2(\mathbb{C})$, the Casimir element is $C = ef + fe + \frac{1}{2}h^2 \in \mathcal{U}(\mathfrak{sl}_2)$. Show that $C$ is central in $\mathcal{U}(\mathfrak{sl}_2)$ (i.e., commutes with all of $\mathfrak{sl}_2$) and compute its eigenvalue on the irreducible module $V(n)$. Express the eigenvalue in terms of $n$ and use it to show that non-isomorphic irreducibles have distinct Casimir eigenvalues.

8. (Challenge) State and prove the Weyl denominator identity for $\mathfrak{sl}_2(\mathbb{C})$: $e^\rho \prod_{\alpha > 0}(1 - e^{-\alpha}) = \sum_{w \in W} (-1)^{\ell(w)} e^{w(\rho)}$, where for $\mathfrak{sl}_2$ the Weyl group is $W = \{1, s\}$, $\rho = \frac{1}{2}\alpha$, and $s(\rho) = -\rho$. Then use the Weyl character formula to derive the dimension formula $\dim L(n\omega) = n+1$ by computing $\lim_{e^\alpha \to 1} \text{ch}(V(n))$ using L'Hôpital's rule on the formal quotient.
