# Row Reduction and Echelon Forms

Row reduction — also called Gaussian elimination — is the systematic algorithm for solving linear systems, computing matrix rank, finding bases for fundamental subspaces, and computing inverses. It is at once a theoretical tool (revealing the structure of a linear system) and a practical algorithm (used in virtually all computational linear algebra). The echelon forms produced by row reduction give a canonical representation from which all relevant information can be read off directly.

## Elementary Row Operations

Three operations on a matrix's rows leave the solution set of $Ax = b$ unchanged:

**(E1) Row swap:** interchange rows $i$ and $j$.

**(E2) Scalar multiplication:** multiply row $i$ by a nonzero scalar $c$.

**(E3) Row replacement:** add $c$ times row $j$ to row $i$.

Each elementary row operation corresponds to left-multiplication by an elementary matrix $E_k$: a matrix that differs from the identity by the corresponding operation. Since each $E_k$ is invertible, row operations preserve the solution set of $Ax = b$ (because the system $E_kAx = E_kb$ has the same solutions as $Ax = b$ when $E_k$ is invertible).

## Row Echelon Form (REF)

A matrix is in **row echelon form** if:
1. All nonzero rows are above any zero rows.
2. In each nonzero row, the **pivot** (leading nonzero entry) is strictly to the right of the pivot in the row above.

$$\begin{pmatrix}2&3&-1&4\\0&0&5&-2\\0&0&0&3\end{pmatrix}$$
is in REF. Pivots are at positions $(1,1)$, $(2,3)$, $(3,4)$.

## Reduced Row Echelon Form (RREF)

A matrix is in **reduced row echelon form** if it is in REF and additionally:
3. Each pivot is $1$.
4. Each pivot column has zeros everywhere except in the pivot row.

$$\begin{pmatrix}1&0&0&2\\0&1&0&-1\\0&0&1&3\end{pmatrix}$$
is in RREF.

**Theorem (Uniqueness of RREF).** Every matrix has a unique RREF.

## The Algorithm

To row-reduce a matrix $A$:
1. Find the leftmost nonzero column. Swap to put a nonzero entry at the top of this column (the pivot).
2. Scale the pivot row to make the pivot $1$ (for RREF).
3. Eliminate all other entries in the pivot column using row replacements.
4. Ignore the pivot row and repeat on the remaining submatrix.

**Example.** Solve $\begin{pmatrix}1&2&-1\\2&5&0\\1&3&1\end{pmatrix}\begin{pmatrix}x\\y\\z\end{pmatrix} = \begin{pmatrix}3\\7\\5\end{pmatrix}$.

Augmented matrix $[A|b]$:
$$\left[\begin{array}{ccc|c}1&2&-1&3\\2&5&0&7\\1&3&1&5\end{array}\right] \xrightarrow{R_2-2R_1,\ R_3-R_1} \left[\begin{array}{ccc|c}1&2&-1&3\\0&1&2&1\\0&1&2&2\end{array}\right] \xrightarrow{R_3-R_2} \left[\begin{array}{ccc|c}1&2&-1&3\\0&1&2&1\\0&0&0&1\end{array}\right].$$

The last row reads $0x + 0y + 0z = 1$, which is inconsistent. So the system has no solution.

## Reading Information from RREF

From the RREF of $A$ (or $[A|b]$):
- **Rank:** number of pivot columns = number of nonzero rows in REF.
- **Consistency:** for $Ax = b$, the system is consistent iff no row of the form $[0 \cdots 0 | c]$ with $c \neq 0$ appears.
- **Pivot variables** and **free variables:** pivot variables correspond to pivot columns; free variables correspond to non-pivot (free) columns. The free variables can take any value; the pivot variables are determined by the free variables.
- **General solution:** with $k$ free variables, the solution set is a $k$-dimensional affine subspace.
- **Null space basis:** set each free variable to $1$ (others to $0$) and solve for the pivot variables; this gives a basis vector for $\ker A$.

**Example.** Find the RREF and null space of $A = \begin{pmatrix}1&2&3&4\\2&4&6&8\end{pmatrix}$.

Row reduce: $R_2 - 2R_1 \to \begin{pmatrix}1&2&3&4\\0&0&0&0\end{pmatrix}$. One pivot (column 1); free variables: $x_2, x_3, x_4$.

From $x_1 + 2x_2 + 3x_3 + 4x_4 = 0$: $x_1 = -2x_2 - 3x_3 - 4x_4$.

Basis for $\ker A$: set $(x_2, x_3, x_4) = (1,0,0), (0,1,0), (0,0,1)$:
$$v_1 = (-2,1,0,0),\quad v_2 = (-3,0,1,0),\quad v_3 = (-4,0,0,1).$$
$\ker A = \text{span}\{v_1, v_2, v_3\}$, confirming $\text{null}(A) = 3 = 4 - 1 = n - \text{rank}(A)$ (Rank-Nullity).

## Back Substitution

For upper triangular systems (as produced by forward elimination/REF), back substitution solves starting from the bottom row:
$$u_{nn}x_n = b_n \Rightarrow x_n = b_n/u_{nn}, \quad u_{n-1,n-1}x_{n-1} = b_{n-1} - u_{n-1,n}x_n, \ldots$$

This $O(n^2)$ step follows the $O(n^3)$ forward elimination.

## Connection to Differential Equations

Row reduction appears in:
1. Computing the Wronskian: the solutions of an ODE system are linearly independent iff the matrix $[\mathbf{y}_1 | \cdots | \mathbf{y}_n]$ has full rank (nonzero determinant), which is checked by row reduction.
2. Solving linear systems arising in ODE boundary value problems discretized to finite differences.
3. Finding bases for eigenspaces: the eigenspace for eigenvalue $\lambda$ is $\ker(\lambda I - A)$, computed by row reduction on $\lambda I - A$.
4. Implementing implicit ODE methods: at each time step, a linear system of the form $(I - hA)x = b$ must be solved, using LU decomposition based on row reduction.
