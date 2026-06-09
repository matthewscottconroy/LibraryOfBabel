# Chapter 9 — Exercises

## Important Figures

- **Augustin-Louis Cauchy (1789–1857)** — proved that real symmetric matrices have real eigenvalues (1829); first rigorous eigenvalue theorem
- **Karl Weierstrass (1815–1897)** — developed the theory of elementary divisors; laid groundwork for canonical forms
- **David Hilbert (1862–1943)** — spectral theorem in infinite dimensions; Hilbert spaces; the name "spectral theory" comes from his work
- **Hermann Weyl (1885–1955)** — Weyl's inequalities for eigenvalue perturbation; connections between eigenvalues and geometry

## References and Primary Sources

- **S. Axler, *Linear Algebra Done Right* (4th ed., Springer, 2024)** — eigenvalues introduced before determinants; structurally clean
- **G. Golub & C. van Loan, *Matrix Computations* (4th ed., Johns Hopkins, 2013)** — numerical eigenvalue algorithms (QR algorithm, power iteration)
- **T. Kato, *Perturbation Theory for Linear Operators* (Springer, 1966)** — eigenvalue sensitivity

## Examples, Applications, and Thought Experiments

- **The Leslie matrix (population dynamics)** — a $2 \times 2$ matrix$A$ models age-structured population growth;$A^n \mathbf{v}_0$ gives the population at time$n$; the dominant eigenvalue$\lambda_1$ gives the long-run growth rate; the corresponding eigenvector gives the stable age distribution; eigenvalues make long-run behavior computable
- **Principal axes of a conic** — a quadratic form $\mathbf{x}^T A \mathbf{x}$ (with$A$ symmetric) defines an ellipse or hyperbola; the eigenvectors of$A$ point along the principal axes; the eigenvalues are the reciprocals of the squared semi-axes
- **Markov chains** — the transition matrix $P$ always has eigenvalue 1; the left eigenvector for eigenvalue 1 is the stationary distribution$\pi$; all other eigenvalues satisfy$|\lambda| \leq 1$; the rate of convergence to$\pi$ is controlled by the second-largest eigenvalue
- **Google's PageRank** — PageRank is the dominant eigenvector of a modified adjacency matrix of the web graph; eigenvalue 1 corresponds to the stationary distribution of a random walk; the algorithm that ranks the world's information is a power iteration for eigenvalue computation

## Exercises

1. Let $A = \begin{pmatrix} 5 & -2 \\ 3 & 0 \end{pmatrix}$. Compute the characteristic polynomial of $A$, find all eigenvalues, and for each eigenvalue find a basis for the corresponding eigenspace. Is $A$ diagonalizable over $\mathbb{R}$? If so, find an invertible matrix $P$ and diagonal matrix $D$ such that $A = PDP^{-1}$.

2. Let $A$ be an $n \times n$ matrix and suppose $\lambda$ is an eigenvalue of $A$ with eigenvector $\mathbf{v}$. Prove that $\lambda^k$ is an eigenvalue of $A^k$ for every positive integer $k$, with the same eigenvector $\mathbf{v}$. Use this to compute $A^{10}$ for a diagonalizable $2 \times 2$ matrix of your choice by first diagonalizing it.

3. Let $A = \begin{pmatrix} 3 & 1 & 0 \\ 0 & 3 & 0 \\ 0 & 0 & 2 \end{pmatrix}$. Find the characteristic polynomial of $A$ and all eigenvalues. For each eigenvalue, compute the algebraic multiplicity and the geometric multiplicity (the dimension of the eigenspace). Is $A$ diagonalizable? Justify your answer.

4. Prove that eigenvectors belonging to distinct eigenvalues are linearly independent. That is, if $T(\mathbf{v}_i) = \lambda_i \mathbf{v}_i$ for $i = 1, \ldots, k$ and all $\lambda_i$ are distinct, then $\mathbf{v}_1, \ldots, \mathbf{v}_k$ are linearly independent. (Hint: induct on $k$ and apply $T - \lambda_k I$ to any dependence relation.)

5. Let $T : V \to V$ be a linear operator on a finite-dimensional vector space over $\mathbb{C}$. Prove that the sum of all eigenspaces $E_{\lambda_1} + \cdots + E_{\lambda_k}$ (one for each distinct eigenvalue) is a direct sum. Conclude that $T$ is diagonalizable if and only if this direct sum equals $V$.

6. Compute the minimal polynomial of each of the following matrices:
   (a) $A = \begin{pmatrix} 2 & 0 \\ 0 & 2 \end{pmatrix}$,
   (b) $B = \begin{pmatrix} 2 & 1 \\ 0 & 2 \end{pmatrix}$,
   (c) $C = \begin{pmatrix} 1 & 0 & 0 \\ 0 & 2 & 0 \\ 0 & 0 & 2 \end{pmatrix}$.
   In each case verify that the minimal polynomial divides the characteristic polynomial, and determine whether the matrix is diagonalizable using the minimal polynomial criterion.

7. The Cayley–Hamilton theorem states that every matrix satisfies its own characteristic polynomial. Verify this for $A = \begin{pmatrix} 1 & 2 \\ 3 & 4 \end{pmatrix}$ by computing $p(A) = A^2 - \text{tr}(A) \cdot A + \det(A) \cdot I$ explicitly. Then use Cayley–Hamilton to express $A^3$ and $A^4$ as linear combinations of $A$ and $I$.

8. (Challenge) Let $A$ be an $n \times n$ real matrix whose characteristic polynomial has no real roots (e.g., the $90^\circ$ rotation matrix in $\mathbb{R}^2$). Explain why $A$ has no real eigenvalues and thus cannot be diagonalized over $\mathbb{R}$. Now regard $A$ as a complex matrix. Show that it has $n$ eigenvalues in $\mathbb{C}$ (counting multiplicity) and find all eigenvalues of the $2 \times 2$ rotation matrix $\begin{pmatrix} 0 & -1 \\ 1 & 0 \end{pmatrix}$. What do the magnitudes of these complex eigenvalues tell you about the geometric effect of the transformation?
