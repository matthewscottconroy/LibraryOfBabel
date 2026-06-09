# Chapter 10 — Exercises

## Important Figures

- **Camille Jordan (1838–1922)** — Jordan normal form theorem (1870): every complex matrix is similar to a unique (up to block ordering) Jordan matrix; unified the theory of linear operators
- **Karl Weierstrass (1815–1897)** — rational canonical form via elementary divisors; the invariant factor decomposition
- **Ferdinand Frobenius (1849–1917)** — companion matrix; minimal polynomial; the theory of rational canonical form over arbitrary fields
- **Frobenius & Stickelberger** — structure theorem for finitely generated abelian groups as precursor to the module-theoretic approach

## References and Primary Sources

- **C. Jordan, *Traité des substitutions et des équations algébriques* (1870)** — first appearance of Jordan normal form
- **S. Lang, *Algebra* (rev. 3rd ed., Springer, 2002)**, Ch. XIV — canonical forms via modules over PIDs; the most elegant approach
- **I.N. Herstein, *Topics in Algebra* (2nd ed., Wiley, 1975)** — classical treatment

## Examples, Applications, and Thought Experiments

- **Jordan form of a nilpotent matrix** — $N = \left[\begin{smallmatrix}0 & 1 & 0 \\ 0 & 0 & 1 \\ 0 & 0 & 0\end{smallmatrix}\right]$;$N^2 \neq 0$ but$N^3 = 0$; a single Jordan block of size 3 with eigenvalue 0; the Jordan form makes the "degree of nilpotency" visible in the block size
- **Solving linear ODE systems** — $\mathbf{y}' = A\mathbf{y}$; put$A$ into Jordan form$A = PJP^{-1}$; then$e^{tA} = Pe^{tJ}P^{-1}$; Jordan blocks of size$k$ produce polynomial factors$t^j/j!$ multiplying$e^{\lambda t}$; the form of the solution is determined by the Jordan structure
- **Minimal polynomial detects Jordan structure** — for $A = \left[\begin{smallmatrix}2 & 1 \\ 0 & 2\end{smallmatrix}\right]$, the minimal polynomial is$(x-2)^2$; for$A = 2I$, it is$(x-2)$; the degree of$(x-\lambda)$ in the minimal polynomial equals the size of the largest Jordan block for$\lambda$
- **Why two canonical forms?** — Jordan form requires that all eigenvalues are in the base field; rational canonical form works over any field (uses invariant factors instead of Jordan blocks); over $\mathbb{C}$, both apply; over$\mathbb{Q}$, only rational canonical form is always available

## Exercises

1. Find the Jordan normal form of each of the following matrices over $\mathbb{C}$, and in each case find an invertible matrix $P$ such that $P^{-1}AP = J$:
   (a) $A = \begin{pmatrix} 4 & 1 & 0 \\ 0 & 4 & 1 \\ 0 & 0 & 4 \end{pmatrix}$,
   (b) $B = \begin{pmatrix} 2 & 1 & 0 \\ 0 & 2 & 0 \\ 0 & 0 & 3 \end{pmatrix}$.
   Identify the size and eigenvalue of each Jordan block.

2. Let $N$ be a $5 \times 5$ nilpotent matrix (over $\mathbb{C}$) with $\dim \ker N = 2$, $\dim \ker N^2 = 4$, and $\dim \ker N^3 = 5$. Without computing $N$ explicitly, determine the sizes of all Jordan blocks in the Jordan form of $N$. How many Jordan blocks are there, and what are their sizes?

3. Let $A = \begin{pmatrix} 3 & 1 \\ 0 & 3 \end{pmatrix}$. Compute $e^{tA}$ using the Jordan form of $A$. Verify your answer by checking that the columns of $e^{tA}$ satisfy $\mathbf{y}' = A\mathbf{y}$.

4. Let $A$ and $B$ be $n \times n$ matrices over $\mathbb{C}$. Prove that $A$ and $B$ are similar (i.e., $B = PAP^{-1}$ for some invertible $P$) if and only if they have the same Jordan normal form up to ordering of blocks. (You may assume the Jordan form theorem; the task is to prove that similarity is equivalent to having the same Jordan form.)

5. Find the minimal polynomial of $A = \begin{pmatrix} 2 & 1 & 0 & 0 \\ 0 & 2 & 0 & 0 \\ 0 & 0 & 2 & 1 \\ 0 & 0 & 0 & 2 \end{pmatrix}$ directly from its Jordan block structure, without computing $p(x)$. Explain the relationship between the size of the largest Jordan block for eigenvalue $\lambda$ and the multiplicity of $\lambda$ as a root of the minimal polynomial.

6. Compute the rational canonical form of $A = \begin{pmatrix} 0 & 0 & -2 \\ 1 & 0 & 1 \\ 0 & 1 & 2 \end{pmatrix}$ over $\mathbb{Q}$. Find the invariant factors and write down the companion matrix blocks. (Hint: factor the characteristic polynomial and determine how it splits into invariant factors using the minimal polynomial.)

7. Let $f(x) = x^3 - 6x^2 + 11x - 6 = (x-1)(x-2)(x-3)$. Write down the companion matrix $C_f$ of $f$. Verify that $f(C_f) = 0$ and that the characteristic polynomial of $C_f$ is $f(x)$. What is the rational canonical form of a $3 \times 3$ matrix over $\mathbb{Q}$ whose characteristic and minimal polynomial are both $f(x)$?

8. (Challenge) Suppose $A$ is a $6 \times 6$ complex matrix with characteristic polynomial $(x - 2)^4 (x - 5)^2$ and minimal polynomial $(x - 2)^3 (x - 5)$. List all possible Jordan normal forms for $A$. For each possibility, justify whether it is consistent with both the characteristic and minimal polynomial. How many similarity classes of matrices are there with this characteristic and minimal polynomial?
