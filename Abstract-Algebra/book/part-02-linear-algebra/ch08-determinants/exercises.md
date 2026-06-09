# Chapter 8 — Exercises

## Important Figures

- **Gottfried Wilhelm Leibniz (1646–1716)** — first systematic use of determinants to express solutions of linear systems (1693)
- **Alexandre-Théophile Vandermonde (1735–1796)** — early systematic treatment; Vandermonde determinant
- **Augustin-Louis Cauchy (1789–1857)** — established the modern theory; proved $\det(AB) = \det(A)\det(B)$; the multiplicative property
- **Carl Jacobi (1804–1851)** — Jacobian determinant and its role in change-of-variables for integration
- **Arthur Cayley (1821–1895)** — determinant as a polynomial in the entries; connection to permanents and alternating multilinear forms

## References and Primary Sources

- **A.-L. Cauchy, "Mémoire sur les fonctions qui ne peuvent obtenir que deux valeurs égales et de signes contraires" (1815)** — major contribution to determinant theory
- **S. Axler, "Down with Determinants!" (1995)** — *Amer. Math. Monthly* 102(2) — argues for building eigentheory first, determinants second
- **R. Horn & C. Johnson, *Matrix Analysis* (2nd ed., Cambridge, 2012)**

## Examples, Applications, and Thought Experiments

- **$2 \times 2$ determinant as signed area** — $\det\bigl[\begin{smallmatrix}a & b \\ c & d\end{smallmatrix}\bigr] = ad - bc$ = signed area of the parallelogram spanned by the columns; sign encodes orientation; $|\det| = 0$ means the vectors are parallel (columns dependent)
- **Cramer's rule** — for a $2 \times 2$ system $Ax = b$: $x_i = \det(A_i)/\det(A)$ where $A_i$ replaces column $i$ with $b$; elegant but $O(n!)$ in general; illuminates why invertibility $\iff \det \neq 0$
- **The Vandermonde determinant** — $\det\begin{pmatrix}1 & x_1 & x_1^2 \\ 1 & x_2 & x_2^2 \\ 1 & x_3 & x_3^2\end{pmatrix} = (x_2-x_1)(x_3-x_1)(x_3-x_2)$; vanishes precisely when two nodes coincide; this is the key to Lagrange interpolation
- **Determinant as alternating multilinear form** — the determinant is the unique alternating multilinear form on $n$ column vectors that equals 1 on the identity; this axiomatic characterization makes the multiplicative property obvious

## Exercises

1. Compute the determinant of each matrix using cofactor expansion along any convenient row or column, and then verify your answer using row reduction.
   $$A = \begin{pmatrix} 2 & 1 & 3 \\ 0 & -1 & 4 \\ 1 & 2 & -2 \end{pmatrix}, \quad B = \begin{pmatrix} 1 & 2 & 0 & 1 \\ 0 & 3 & 1 & 2 \\ 0 & 0 & -1 & 4 \\ 0 & 0 & 0 & 2 \end{pmatrix}.$$

2. Using only the axiomatic properties of the determinant (multilinearity, alternating, and $\det(I) = 1$), prove each of the following without computing any explicit formula:
   - (a) If $A$ has two equal rows, then $\det(A) = 0$.
   - (b) Adding a scalar multiple of one row to another does not change the determinant.
   - (c) $\det(A^T) = \det(A)$.

3. Prove the multiplicativity formula $\det(AB) = \det(A)\det(B)$ for $n \times n$ matrices $A$ and $B$ over a field $F$. (Outline: consider two cases — $A$ singular and $A$ invertible. In the invertible case, write $A$ as a product of elementary matrices and use the known effect of each type on the determinant.)

4. Use Cramer's rule to solve the system $Ax = b$ where
   $$A = \begin{pmatrix} 2 & 1 \\ 5 & 3 \end{pmatrix}, \quad b = \begin{pmatrix} 4 \\ 7 \end{pmatrix}.$$
   Then verify your answer by row reduction. Under what conditions does Cramer's rule apply, and why is it generally inefficient for large $n$?

5. Let $A$ and $B$ be $n \times n$ matrices. Prove or disprove:
   - (a) $\det(A + B) = \det(A) + \det(B)$
   - (b) $\det(cA) = c^n \det(A)$ for any scalar $c$
   - (c) If $\det(A) = \det(B)$, then $A$ and $B$ are similar.

6. The Vandermonde matrix for scalars $x_1, \ldots, x_n$ is $V = (x_i^{j-1})_{1 \leq i,j \leq n}$. For the $3 \times 3$ case, prove directly (by row reduction or cofactor expansion) that $\det(V) = \prod_{1 \leq i < j \leq n}(x_j - x_i)$. Conclude that the columns of $V$ are linearly independent if and only if the $x_i$ are pairwise distinct.

7. Let $T: V \to V$ be a linear operator on an $n$-dimensional vector space, and define the characteristic polynomial $\chi_T(\lambda) = \det(\lambda I - [T]_{\mathcal{B}})$ for some ordered basis $\mathcal{B}$. Prove that $\chi_T$ is independent of the choice of basis (i.e., is a well-defined invariant of $T$). What are the degree, leading coefficient, and constant term of $\chi_T$ in terms of the matrix entries?

8. (Challenge) Let $A$ be an $n \times n$ matrix with $\det(A) \neq 0$. The adjugate (classical adjoint) $\mathrm{adj}(A)$ is defined as the transpose of the cofactor matrix: $(\mathrm{adj}(A))_{ij} = (-1)^{i+j} M_{ji}$, where $M_{ji}$ is the $(j,i)$ minor. Prove the identity $A \cdot \mathrm{adj}(A) = \det(A) \cdot I$ directly from the cofactor expansion, and deduce the formula $A^{-1} = \frac{1}{\det(A)} \mathrm{adj}(A)$. Use this to find the inverse of a general $2 \times 2$ matrix and verify the formula $A^{-1} = \frac{1}{ad-bc}\bigl[\begin{smallmatrix}d & -b \\ -c & a\end{smallmatrix}\bigr]$.
