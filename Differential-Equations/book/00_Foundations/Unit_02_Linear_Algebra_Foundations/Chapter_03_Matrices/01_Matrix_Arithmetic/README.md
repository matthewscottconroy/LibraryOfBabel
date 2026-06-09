# Matrix Arithmetic

A matrix is a rectangular array of numbers. The arithmetic of matrices — addition, scalar multiplication, and the more subtle operation of multiplication — is defined in a way that makes matrices the concrete representation of linear maps. Understanding why matrix operations are defined as they are, rather than treating them as arbitrary rules, makes the algebra both more memorable and more useful.

## Matrices as Arrays

An $m \times n$ matrix over $\mathbb{R}$ is an array with $m$ rows and $n$ columns:
$$A = \begin{pmatrix} a_{11} & a_{12} & \cdots & a_{1n} \\ a_{21} & a_{22} & \cdots & a_{2n} \\ \vdots & & \ddots & \vdots \\ a_{m1} & a_{m2} & \cdots & a_{mn} \end{pmatrix} = (a_{ij})$$
where $a_{ij}$ is the entry in row $i$, column $j$. The set of all $m \times n$ real matrices is denoted $M_{m\times n}(\mathbb{R})$ or $\mathbb{R}^{m\times n}$.

## Addition and Scalar Multiplication

Two matrices of the same size can be added entry-by-entry:
$$(A + B)_{ij} = a_{ij} + b_{ij}.$$
Scalar multiplication also acts entry-by-entry:
$$(\alpha A)_{ij} = \alpha a_{ij}.$$

These operations satisfy all the vector space axioms, making $M_{m\times n}(\mathbb{R})$ a vector space of dimension $mn$.

The **zero matrix** $O$ (all entries $0$) is the additive identity. The **negative** $-A$ has entries $(-A)_{ij} = -a_{ij}$.

## Matrix Multiplication

The definition of matrix multiplication is motivated by composition of linear maps. If $A$ represents $T: \mathbb{R}^n \to \mathbb{R}^m$ and $B$ represents $S: \mathbb{R}^p \to \mathbb{R}^n$, then $AB$ should represent $T \circ S: \mathbb{R}^p \to \mathbb{R}^m$.

**Definition.** If $A$ is $m \times n$ and $B$ is $n \times p$, their product $C = AB$ is the $m \times p$ matrix:
$$c_{ij} = \sum_{k=1}^n a_{ik} b_{kj} = (\text{row }i\text{ of }A) \cdot (\text{column }j\text{ of }B).$$

The inner dimensions must match: $A$ is $m \times \mathbf{n}$ and $B$ is $\mathbf{n} \times p$.

**Example.**
$$\begin{pmatrix}1&2\\3&4\end{pmatrix}\begin{pmatrix}5&6\\7&8\end{pmatrix} = \begin{pmatrix}1\cdot5+2\cdot7&1\cdot6+2\cdot8\\3\cdot5+4\cdot7&3\cdot6+4\cdot8\end{pmatrix} = \begin{pmatrix}19&22\\43&50\end{pmatrix}.$$

## Key Properties of Matrix Multiplication

**Associativity:** $(AB)C = A(BC)$ whenever dimensions are compatible. This mirrors associativity of function composition.

**Distributivity:** $A(B+C) = AB + AC$ and $(A+B)C = AC + BC$.

**Non-commutativity:** In general, $AB \neq BA$. Even if both products are defined, they need not be equal.

**Example of non-commutativity:**
$$\begin{pmatrix}1&0\\0&0\end{pmatrix}\begin{pmatrix}0&1\\0&0\end{pmatrix} = \begin{pmatrix}0&1\\0&0\end{pmatrix}, \quad \begin{pmatrix}0&1\\0&0\end{pmatrix}\begin{pmatrix}1&0\\0&0\end{pmatrix} = \begin{pmatrix}0&0\\0&0\end{pmatrix}.$$

**Identity matrix:** The $n \times n$ identity $I_n$ (1's on diagonal, 0's elsewhere) satisfies $AI_n = A$ and $I_m A = A$ for $A \in M_{m\times n}$.

**Zero divisors:** $AB = O$ does not imply $A = O$ or $B = O$ (shown by example above). This is a crucial difference from the real number field.

## Transpose

The **transpose** of $A = (a_{ij})$ is $A^T = (a_{ji})$ — rows and columns are swapped.

Properties:
- $(A^T)^T = A$
- $(A + B)^T = A^T + B^T$
- $(\alpha A)^T = \alpha A^T$
- $(AB)^T = B^T A^T$ (note the reversal of order)

A matrix is **symmetric** if $A^T = A$ and **skew-symmetric** if $A^T = -A$.

## Matrix-Vector Products

For $A \in M_{m\times n}$ and $x \in \mathbb{R}^n$ (viewed as an $n\times 1$ column vector):
$$Ax = x_1 A_1 + x_2 A_2 + \cdots + x_n A_n,$$
where $A_j$ is the $j$-th column of $A$. This viewpoint — $Ax$ is a linear combination of the columns of $A$ with coefficients from $x$ — is often the most useful perspective.

## Block Matrices

Matrices can be partitioned into blocks, and block matrix multiplication follows the same formula as ordinary multiplication, treating blocks as entries (as long as the block dimensions are compatible). Block structure often simplifies computation and proofs.

**Example.** If $A = \begin{pmatrix}I & 0 \\ C & D\end{pmatrix}$ and $B = \begin{pmatrix}E \\ F\end{pmatrix}$, then $AB = \begin{pmatrix}E \\ CE + DF\end{pmatrix}$.

## Connection to Differential Equations

The matrix exponential $e^{At}$ — the key solution formula for $\mathbf{x}' = A\mathbf{x}$ — is defined by the matrix power series $e^{At} = \sum_{k=0}^\infty \frac{(At)^k}{k!}$. The computation of $(At)^k = A^k t^k$ requires matrix multiplication, and the convergence of the series requires the norm structure of matrices (which can be derived from the operator norm). Matrix arithmetic is the foundation of this entire computation.

Similarly, numerical ODE methods at each time step solve a linear system (e.g., the backward Euler method requires $(I - hA)x_{n+1} = x_n$), which requires efficient matrix arithmetic.
