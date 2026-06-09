# Matrix Multiplication and Composition

Matrix multiplication is the algebraic operation that corresponds to composition of linear maps. This correspondence is not incidental — it is the reason matrix multiplication is defined as it is, rather than entry-by-entry. Understanding the connection between matrices and maps makes matrix algebra both more intuitive and more powerful.

## The Correspondence

Fix standard bases in $\mathbb{R}^n$ and $\mathbb{R}^m$. Any linear map $T: \mathbb{R}^n \to \mathbb{R}^m$ is completely determined by where it sends the standard basis vectors $e_1, \ldots, e_n$. The matrix of $T$ is $[T] = [T(e_1) | T(e_2) | \cdots | T(e_n)]$, where the columns are the images of the basis vectors.

**Theorem.** If $T: \mathbb{R}^n \to \mathbb{R}^m$ has matrix $A = [T]$ and $S: \mathbb{R}^p \to \mathbb{R}^n$ has matrix $B = [S]$, then the composition $T \circ S: \mathbb{R}^p \to \mathbb{R}^m$ has matrix $AB$.

*Proof.* $(T \circ S)(e_j) = T(S(e_j)) = T(Be_j) = T(\text{column }j \text{ of }B) = A \cdot (\text{column }j\text{ of }B) = (AB)e_j$. $\square$

This shows that the $j$-th column of $AB$ is $A$ applied to the $j$-th column of $B$, which matches the definition of matrix multiplication.

## Change of Basis and Similarity

When a linear map $T: V \to V$ is expressed in two different bases $\mathcal{B}$ and $\mathcal{C}$, the resulting matrices $A$ and $A'$ are related by a similarity transformation: $A' = P^{-1}AP$, where $P$ is the change-of-basis matrix.

**Definition.** Matrices $A$ and $B$ are **similar** if there exists an invertible matrix $P$ with $B = P^{-1}AP$.

Similar matrices represent the same linear map in different bases. They have the same eigenvalues, the same characteristic polynomial, the same determinant, and the same trace. The goal of diagonalization (Chapter 4) is to find a basis in which the matrix of $T$ is diagonal — the simplest possible similar matrix.

## Powers of Matrices

For a square matrix $A$:
$$A^0 = I, \quad A^1 = A, \quad A^{n+1} = A \cdot A^n.$$

Powers follow the rule $(A^m)(A^n) = A^{m+n}$ and $(A^m)^n = A^{mn}$.

For diagonal matrices $D = \text{diag}(\lambda_1, \ldots, \lambda_n)$: $D^k = \text{diag}(\lambda_1^k, \ldots, \lambda_n^k)$. This makes powers of diagonalizable matrices easy:
$$A^k = (PDP^{-1})^k = PD^kP^{-1}.$$

## The Matrix Exponential (Preview)

The series $e^A = \sum_{k=0}^\infty \frac{A^k}{k!}$ converges for every square matrix $A$ (in any matrix norm). If $A = PDP^{-1}$:
$$e^A = Pe^DP^{-1} = P\,\text{diag}(e^{\lambda_1}, \ldots, e^{\lambda_n})\,P^{-1}.$$

This is the formula for the solution to $\mathbf{x}' = A\mathbf{x}$: $\mathbf{x}(t) = e^{At}\mathbf{x}(0)$. The ability to compute $A^k$ efficiently — through diagonalization or Jordan form — is what makes the matrix exponential computationally tractable.

## Algebraic Properties

- **Associativity:** $(AB)C = A(BC)$. Proved by direct computation or by associativity of function composition.
- **Non-commutativity:** $AB \neq BA$ in general.
- **Distributivity:** $A(B+C) = AB+AC$ and $(A+B)C = AC+BC$.
- **Transpose reversal:** $(AB)^T = B^TA^T$.
- **Inverse reversal:** $(AB)^{-1} = B^{-1}A^{-1}$ (when both are invertible).

The reversal rules for transpose and inverse reflect the reversal of composition order: $(S \circ T)^{-1} = T^{-1} \circ S^{-1}$.

## Worked Example: Verifying $AB \neq BA$

$$A = \begin{pmatrix}0&1\\0&0\end{pmatrix}, \quad B = \begin{pmatrix}0&0\\1&0\end{pmatrix}.$$
$$AB = \begin{pmatrix}1&0\\0&0\end{pmatrix}, \quad BA = \begin{pmatrix}0&0\\0&1\end{pmatrix}.$$
$AB \neq BA$. In fact, $AB + BA = I$, and $A^2 = B^2 = O$ — these are nilpotent matrices (related to Jordan blocks, discussed in Chapter 4).

## Connection to Systems of ODEs

The system $\mathbf{x}' = A\mathbf{x}$ has solution $\mathbf{x}(t) = e^{At}\mathbf{x}_0$. The matrix $e^{At}$ is computed via the power series, and for this to be tractable one needs either a diagonalization $A = PDP^{-1}$ (giving $e^{At} = Pe^{Dt}P^{-1}$) or the Jordan form (giving the general formula involving polynomial factors times exponentials). Both require understanding matrix multiplication as the composition operation, and the algebraic properties (especially non-commutativity) are what make the Jordan form necessary when $A$ is not diagonalizable.
