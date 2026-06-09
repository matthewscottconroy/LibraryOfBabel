# Chapter 7 — Exercises

## Important Figures

- **Arthur Cayley (1821–1895)** — "A Memoir on the Theory of Matrices" (1858): invented matrix algebra; proved the Cayley–Hamilton theorem
- **James Joseph Sylvester (1814–1897)** — coined the term "matrix" (1850); developed matrix theory alongside Cayley
- **Carl Friedrich Gauss (1777–1855)** — Gaussian elimination for solving linear systems; foundational algorithm of computational linear algebra
- **Charles Hermite (1822–1901)** — Hermitian matrices (self-adjoint under conjugate transpose); pivotal in quantum mechanics

## References and Primary Sources

- **A. Cayley, "A Memoir on the Theory of Matrices" (1858)** — *Phil. Trans. Roy. Soc.* — foundational paper
- **R. Horn & C. Johnson, *Matrix Analysis* (2nd ed., Cambridge, 2012)** — comprehensive modern reference
- **G. Strang, *Introduction to Linear Algebra* (5th ed., Wellesley-Cambridge, 2016)** — accessible with many applications

## Examples, Applications, and Thought Experiments

- **Matrix multiplication is not commutative** — $AB \neq BA$ in general; non-commutativity is not a pathology but reflects the asymmetry of composition of non-symmetric transformations
- **Rotation matrices** — $R(\theta) = \bigl[\begin{smallmatrix}\cos\theta & -\sin\theta \\ \sin\theta & \cos\theta\end{smallmatrix}\bigr]$; then $R(\alpha) R(\beta) = R(\alpha+\beta)$; matrix multiplication encodes composition of rotations; $R(\theta)^{-1} = R(-\theta) = R(\theta)^T$
- **Gaussian elimination** — solving a $3 \times 3$ system via augmented matrix and row operations; the algorithm terminates in $O(n^3)$ steps; yields both the solution and the row echelon form that detects rank
- **Block matrices** — a matrix partitioned into blocks that behave like scalar entries under multiplication; used to decompose large computations and to understand direct sum decompositions of linear maps

## Exercises

1. Let $A = \bigl[\begin{smallmatrix} 1 & 2 \\ 3 & 4 \end{smallmatrix}\bigr]$ and $B = \bigl[\begin{smallmatrix} 0 & 1 \\ 1 & 0 \end{smallmatrix}\bigr]$. Compute $AB$, $BA$, $A^2$, $(AB)^T$, and $B^T A^T$. Verify that $(AB)^T = B^T A^T$.

2. Use row reduction to solve the system $Ax = b$, where
   $$A = \begin{pmatrix} 1 & 2 & -1 \\ 2 & 1 & 3 \\ -1 & 3 & -4 \end{pmatrix}, \quad b = \begin{pmatrix} 1 \\ 5 \\ -2 \end{pmatrix}.$$
   Find the reduced row echelon form of $A$, determine its rank, and find a basis for $\ker A$.

3. Let $T: \mathbb{R}^3 \to \mathbb{R}^2$ be the linear map $T(x_1, x_2, x_3) = (x_1 - x_2,\ 2x_2 + x_3)$. Write down the matrix of $T$ with respect to the standard bases. Now compute the matrix of $T$ with respect to the basis $\mathcal{B} = \{(1,1,0), (0,1,1), (1,0,1)\}$ for $\mathbb{R}^3$ and the standard basis for $\mathbb{R}^2$.

4. Let $\mathcal{B} = \{1, x, x^2\}$ be the standard basis for $\mathbb{R}[x]_{\leq 2}$ and let $\mathcal{B}' = \{1, x-1, (x-1)^2\}$ be another basis. Find the change-of-basis matrix $P$ from $\mathcal{B}$ to $\mathcal{B}'$ (i.e., such that $[v]_{\mathcal{B}'} = P [v]_{\mathcal{B}}$), and verify that $P^{-1}$ correctly converts $\mathcal{B}'$-coordinates back to $\mathcal{B}$-coordinates for the vector $p(x) = 3x^2 - 2x + 1$.

5. Two matrices $A, B \in M_{n \times n}(F)$ are similar if there exists an invertible $P$ with $B = P^{-1}AP$. Prove that similarity is an equivalence relation. Then show that similar matrices have the same trace and the same rank. (The trace of a matrix is the sum of its diagonal entries; you may use the fact that $\mathrm{tr}(AB) = \mathrm{tr}(BA)$ for all square matrices.)

6. Find all $2 \times 2$ matrices that commute with $A = \bigl[\begin{smallmatrix} 1 & 1 \\ 0 & 1 \end{smallmatrix}\bigr]$. (That is, find all $B$ such that $AB = BA$.) Show that this set of matrices forms a subspace of $M_{2\times 2}(\mathbb{R})$, and determine its dimension.

7. Let $A$ be an $n \times n$ matrix with entries in $F$, and let $\mathrm{col}(A)$ denote the column space of $A$ (the span of its columns) and $\mathrm{null}(A)$ the null space. Prove that $\dim \mathrm{col}(A) + \dim \mathrm{null}(A) = n$. Then prove that $\dim \mathrm{col}(A) = \dim \mathrm{col}(A^T)$ (i.e., the row rank equals the column rank).

8. (Challenge) Let $A$ and $B$ be $n \times n$ matrices over a field $F$. Prove the following rank inequalities:
   - (a) $\mathrm{rank}(AB) \leq \min(\mathrm{rank}(A), \mathrm{rank}(B))$
   - (b) $\mathrm{rank}(A + B) \leq \mathrm{rank}(A) + \mathrm{rank}(B)$
   - (c) $\mathrm{rank}(A) + \mathrm{rank}(B) - n \leq \mathrm{rank}(AB)$ (the Sylvester rank inequality)

   For (c), interpret the inequality in terms of the dimensions of $\ker A$, $\ker B$, and $\ker(AB)$.
