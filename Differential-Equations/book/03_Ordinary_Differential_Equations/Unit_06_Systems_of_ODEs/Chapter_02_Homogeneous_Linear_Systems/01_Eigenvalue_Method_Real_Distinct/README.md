# Eigenvalue Method: Real Distinct Eigenvalues

When the $n \times n$ real matrix $A$ has $n$ distinct real eigenvalues $\lambda_1, \lambda_2, \ldots, \lambda_n$ (all different), the system $\mathbf{x}' = A\mathbf{x}$ has the elegant general solution

$$\mathbf{x}(t) = c_1\mathbf{v}_1 e^{\lambda_1 t} + c_2\mathbf{v}_2 e^{\lambda_2 t} + \cdots + c_n\mathbf{v}_n e^{\lambda_n t},$$

where $\mathbf{v}_k$ is an eigenvector corresponding to $\lambda_k$.

## Linear Independence

Eigenvectors corresponding to distinct eigenvalues are always linearly independent. Consequently, the $n$ solutions $\mathbf{v}_k e^{\lambda_k t}$ are linearly independent (their Wronskian at $t = 0$ is $\det[\mathbf{v}_1 \mid \cdots \mid \mathbf{v}_n] \neq 0$), confirming that they form a fundamental set.

## Worked Example

Solve $\mathbf{x}' = A\mathbf{x}$ where $A = \begin{pmatrix}1 & -1\\ 2 & 4\end{pmatrix}$, $\mathbf{x}(0) = \begin{pmatrix}1\\0\end{pmatrix}$.

**Eigenvalues:** $\det(A - \lambda I) = (1-\lambda)(4-\lambda) + 2 = \lambda^2 - 5\lambda + 6 = (\lambda-2)(\lambda-3) = 0$. So $\lambda_1 = 2$, $\lambda_2 = 3$.

**Eigenvectors:** For $\lambda_1 = 2$: $(A - 2I)\mathbf{v} = 0$: $\begin{pmatrix}-1&-1\\2&2\end{pmatrix}\mathbf{v} = 0$, giving $v_1 = -v_2$. Take $\mathbf{v}_1 = \begin{pmatrix}1\\-1\end{pmatrix}$.

For $\lambda_2 = 3$: $(A-3I)\mathbf{v} = 0$: $\begin{pmatrix}-2&-1\\2&1\end{pmatrix}\mathbf{v} = 0$, giving $2v_1 + v_2 = 0$. Take $\mathbf{v}_2 = \begin{pmatrix}1\\-2\end{pmatrix}$.

**General solution:** $\mathbf{x}(t) = c_1\begin{pmatrix}1\\-1\end{pmatrix}e^{2t} + c_2\begin{pmatrix}1\\-2\end{pmatrix}e^{3t}$.

**Initial condition:** At $t = 0$: $c_1\begin{pmatrix}1\\-1\end{pmatrix} + c_2\begin{pmatrix}1\\-2\end{pmatrix} = \begin{pmatrix}1\\0\end{pmatrix}$. System: $c_1 + c_2 = 1$, $-c_1 - 2c_2 = 0$. From the second: $c_1 = -2c_2$. Substituting: $-2c_2 + c_2 = 1$, $c_2 = -1$, $c_1 = 2$.

**Solution:** $\mathbf{x}(t) = 2\begin{pmatrix}1\\-1\end{pmatrix}e^{2t} - \begin{pmatrix}1\\-2\end{pmatrix}e^{3t} = \begin{pmatrix}2e^{2t}-e^{3t}\\-2e^{2t}+2e^{3t}\end{pmatrix}$.

## Stability

All eigenvalues are positive ($\lambda_1 = 2$, $\lambda_2 = 3$), so all solutions grow exponentially: the equilibrium $\mathbf{x} = \mathbf{0}$ is an **unstable node**. If both eigenvalues were negative, the equilibrium would be a **stable node**; if they had opposite signs, it would be a **saddle point**.
