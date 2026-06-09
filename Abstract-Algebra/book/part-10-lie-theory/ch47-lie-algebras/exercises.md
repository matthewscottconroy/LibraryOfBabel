# Chapter 47 — Exercises

## Important Figures

- **Sophus Lie (1842–1899)** — Lie algebras as infinitesimal symmetries; the Lie bracket from the commutator of vector fields
- **Wilhelm Killing (1847–1923)** — classification program for Lie algebras; Killing form; root systems (1888–1890)
- **Élie Cartan (1869–1951)** — corrected and completed Killing's classification; Cartan subalgebras; Cartan matrix; the theory of roots in its final form
- **Nathan Jacobson (1910–1999)** — *Lie Algebras* (1962): comprehensive algebraic treatment over arbitrary fields

## References and Primary Sources

- **J.-P. Serre, *Lie Algebras and Lie Groups* (Benjamin, 1964; 2nd ed. Springer, 1992)** — elegant short treatment
- **N. Jacobson, *Lie Algebras* (Interscience, 1962; Dover reprint)** — comprehensive algebraic approach
- **J. Humphreys, *Introduction to Lie Algebras and Representation Theory* (Springer, 1972)** — the standard graduate text

## Examples, Applications, and Thought Experiments

- **$\mathfrak{sl}_2(\mathbb{C})$** — basis $\{e, f, h\}$ with $[h,e] = 2e$, $[h,f] = -2f$, $[e,f] = h$; the simplest non-abelian semisimple Lie algebra; representations $V(n)$ of dimension $n+1$ classified by non-negative integers; $e$ and $f$ are "raising" and "lowering" operators; $h$ measures weight
- **$\mathfrak{gl}_n$ as a Lie algebra** — $[A,B] = AB - BA$ (matrix commutator); this is the Lie bracket; $\mathfrak{sl}_n = \{A \in \mathfrak{gl}_n : \text{tr}(A) = 0\}$ is the Lie algebra of $SL_n$; the commutator captures infinitesimal non-commutativity
- **The adjoint representation** — $\text{ad}: \mathfrak{g} \to \mathfrak{gl}(\mathfrak{g})$ defined by $\text{ad}_x(y) = [x,y]$; this is a Lie algebra homomorphism; the Killing form $B(x,y) = \text{tr}(\text{ad}_x \circ \text{ad}_y)$ is symmetric bilinear and $G$-invariant; $B$ is non-degenerate iff $\mathfrak{g}$ is semisimple (Cartan's criterion)
- **The exponential map** — $\exp: \mathfrak{g} \to G$; for matrix groups, $\exp(A) = \sum_{n \geq 0} A^n/n!$; $\exp$ maps Lie algebra elements to Lie group elements; $d(\exp)_0 = \text{id}$; provides local coordinates on the Lie group from the Lie algebra; the BCH formula expresses $\exp(X)\exp(Y)$ in terms of commutators

## Exercises

1. Compute the Lie bracket $[A, B] = AB - BA$ in $\mathfrak{gl}_n(\mathbb{R})$ for the following pairs and verify bilinearity and antisymmetry: (a) $A = E_{12}$, $B = E_{21}$ (standard matrix units); (b) $A = \text{diag}(1, -1)$, $B = \begin{bmatrix} 0 & 1 \\ 0 & 0 \end{bmatrix}$ in $\mathfrak{gl}_2$. For each, determine which Lie subalgebra of $\mathfrak{sl}_2$ the three elements $A$, $B$, $[A,B]$ generate.

2. For $\mathfrak{sl}_2(\mathbb{C})$ with basis $e = \begin{bmatrix} 0 & 1 \\ 0 & 0 \end{bmatrix}$, $f = \begin{bmatrix} 0 & 0 \\ 1 & 0 \end{bmatrix}$, $h = \begin{bmatrix} 1 & 0 \\ 0 & -1 \end{bmatrix}$, verify the bracket relations $[h,e] = 2e$, $[h,f] = -2f$, $[e,f] = h$ by direct matrix computation. Then verify the Jacobi identity for the triple $(e, f, h)$.

3. Compute the Killing form $B(x,y) = \text{tr}(\text{ad}_x \circ \text{ad}_y)$ for $\mathfrak{sl}_2(\mathbb{C})$. Write $B$ as a matrix in the basis $\{e, f, h\}$ and determine whether it is non-degenerate. Repeat for the abelian Lie algebra $\mathfrak{g} = \mathbb{R}^n$ with trivial bracket, and compare.

4. Let $\mathfrak{g}$ be the Lie algebra of $3 \times 3$ strictly upper triangular matrices over $\mathbb{R}$ (zero diagonal). Write down a basis, compute all Lie brackets, and identify $\mathfrak{g}$ as the Heisenberg algebra. Show that the center of $\mathfrak{g}$ is one-dimensional and that $\mathfrak{g}$ is nilpotent.

5. Prove that for any Lie algebra $\mathfrak{g}$, the map $\text{ad}: \mathfrak{g} \to \mathfrak{gl}(\mathfrak{g})$ defined by $\text{ad}_x(y) = [x,y]$ is a Lie algebra homomorphism. (This amounts to showing that the Jacobi identity is equivalent to $\text{ad}$ being a homomorphism.) Identify the kernel of $\text{ad}$ as the center $Z(\mathfrak{g})$.

6. For the matrix exponential on $\mathfrak{sl}_2(\mathbb{C})$, compute $\exp(tX)$ explicitly for $X = e$, $X = h$, and $X = e + f$ (this last one requires diagonalizing $e + f$). In each case, identify the one-parameter subgroup in $SL_2(\mathbb{C})$ and describe its geometry.

7. State and prove the first two terms of the Baker–Campbell–Hausdorff formula: for matrices $X$ and $Y$ sufficiently close to zero, $\log(\exp(X)\exp(Y)) = X + Y + \frac{1}{2}[X,Y] + \cdots$. Verify the $\frac{1}{2}[X,Y]$ term directly by expanding $e^X e^Y$ to second order and applying the matrix logarithm.

8. (Challenge) Let $\phi: G \to H$ be a Lie group homomorphism. Prove that the induced map $d\phi_e: \mathfrak{g} \to \mathfrak{h}$ is a Lie algebra homomorphism, i.e., that it preserves the bracket. Use this to show that if $G$ and $H$ are simply connected and $\mathfrak{g} \cong \mathfrak{h}$ as Lie algebras, then $G \cong H$ as Lie groups.
