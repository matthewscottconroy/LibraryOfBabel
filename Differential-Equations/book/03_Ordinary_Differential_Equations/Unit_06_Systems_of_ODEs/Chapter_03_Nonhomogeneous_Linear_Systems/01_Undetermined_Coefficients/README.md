# Undetermined Coefficients for Systems

For the constant-coefficient system $\mathbf{x}' = A\mathbf{x} + \mathbf{g}(t)$ with $\mathbf{g}(t)$ of exponential-polynomial form, undetermined coefficients works by guessing a trial vector of the same type and solving for the unknown coefficients.

## The Trial Vector

The trial form mirrors the scalar case:
- If $\mathbf{g}(t) = \mathbf{a}e^{\alpha t}$, try $\mathbf{x}_p = \mathbf{u}e^{\alpha t}$.
- If $\mathbf{g}(t) = \mathbf{a}\cos\beta t + \mathbf{b}\sin\beta t$, try $\mathbf{x}_p = \mathbf{u}\cos\beta t + \mathbf{w}\sin\beta t$.
- For products: $\mathbf{g}(t) = (P_n(t))\mathbf{a}e^{\alpha t}$, try $(Q_n(t))\mathbf{u}e^{\alpha t}$.

**Modification rule.** If $\alpha$ is an eigenvalue of $A$, the trial must be multiplied by $t^m$ where $m$ is the smallest integer such that the modified trial is not a homogeneous solution.

## Worked Example

Solve $\mathbf{x}' = \begin{pmatrix}1&2\\0&3\end{pmatrix}\mathbf{x} + \begin{pmatrix}2e^t\\0\end{pmatrix}$.

Eigenvalues of $A$: $\lambda = 1, 3$. The forcing is $\mathbf{g} = \begin{pmatrix}2\\0\end{pmatrix}e^t$. Since $\alpha = 1$ is an eigenvalue, the trial $\mathbf{u}e^t$ is a homogeneous solution (for the appropriate $\mathbf{u}$). Multiply by $t$: try $\mathbf{x}_p = (\mathbf{u}t + \mathbf{w})e^t$.

Then $\mathbf{x}_p' = (\mathbf{u} + \mathbf{u}t + \mathbf{w})e^t$. Substituting into $\mathbf{x}' = A\mathbf{x}_p + \mathbf{g}$:

$(\mathbf{u} + \mathbf{u}t + \mathbf{w})e^t = A(\mathbf{u}t + \mathbf{w})e^t + \begin{pmatrix}2\\0\end{pmatrix}e^t$.

Matching coefficients of $te^t$: $\mathbf{u} = A\mathbf{u}$, so $(A - I)\mathbf{u} = \mathbf{0}$: $\mathbf{u}$ is an eigenvector of $A$ for $\lambda = 1$. For $A$: $(A-I)\mathbf{u} = \begin{pmatrix}0&2\\0&2\end{pmatrix}\mathbf{u} = \mathbf{0}$, so $u_2 = 0$, $u_1$ free. Take $\mathbf{u} = \begin{pmatrix}1\\0\end{pmatrix}$.

Matching coefficients of $e^t$ (constant terms): $\mathbf{u} + \mathbf{w} = A\mathbf{w} + \begin{pmatrix}2\\0\end{pmatrix}$, so $(A-I)\mathbf{w} = \mathbf{u} - \begin{pmatrix}2\\0\end{pmatrix} = \begin{pmatrix}-1\\0\end{pmatrix}$.

$(A-I)\mathbf{w} = \begin{pmatrix}0&2\\0&2\end{pmatrix}\begin{pmatrix}w_1\\w_2\end{pmatrix} = \begin{pmatrix}2w_2\\2w_2\end{pmatrix} = \begin{pmatrix}-1\\0\end{pmatrix}$. This requires $2w_2 = -1$ and $2w_2 = 0$: inconsistent. This means a further modification is needed, or we need to reconsider the structure. In practice, the correct approach when the modified trial is still insufficient is to solve the system $(A-\lambda I)\mathbf{w} = \mathbf{u}$ for generalized eigenvectors. The full procedure for this case mirrors the Jordan form analysis.
