# Repeated Eigenvalues and Jordan Normal Form

When $A$ has a repeated eigenvalue $\lambda$ of algebraic multiplicity $m$ but geometric multiplicity $k < m$ (fewer than $m$ linearly independent eigenvectors), the eigenvalue method fails to produce $m$ independent solutions. The solution is to use **generalized eigenvectors** and the **Jordan normal form**.

## Generalized Eigenvectors

A vector $\mathbf{v}$ is a **generalized eigenvector of rank $r$** for eigenvalue $\lambda$ if $(A - \lambda I)^r\mathbf{v} = \mathbf{0}$ but $(A - \lambda I)^{r-1}\mathbf{v} \neq \mathbf{0}$. Genuine eigenvectors have rank 1.

For a Jordan block of size $m$ for eigenvalue $\lambda$, there is one eigenvector $\mathbf{v}_1$ and generalized eigenvectors $\mathbf{v}_2, \ldots, \mathbf{v}_m$ satisfying the chain:

$$(A - \lambda I)\mathbf{v}_1 = \mathbf{0}, \quad (A-\lambda I)\mathbf{v}_2 = \mathbf{v}_1, \quad \ldots \quad (A-\lambda I)\mathbf{v}_m = \mathbf{v}_{m-1}.$$

## Solutions from Jordan Chains

For the chain $\{\mathbf{v}_1, \mathbf{v}_2, \ldots, \mathbf{v}_m\}$, the corresponding solutions of $\mathbf{x}' = A\mathbf{x}$ are:

$$\mathbf{x}_1 = e^{\lambda t}\mathbf{v}_1, \quad \mathbf{x}_2 = e^{\lambda t}(t\mathbf{v}_1 + \mathbf{v}_2), \quad \ldots \quad \mathbf{x}_m = e^{\lambda t}\!\left(\frac{t^{m-1}}{(m-1)!}\mathbf{v}_1 + \cdots + t\mathbf{v}_{m-1} + \mathbf{v}_m\right).$$

## Worked Example

Solve $\mathbf{x}' = \begin{pmatrix}2&1\\0&2\end{pmatrix}\mathbf{x}$.

Eigenvalue $\lambda = 2$ (repeated, algebraic multiplicity 2). $(A-2I) = \begin{pmatrix}0&1\\0&0\end{pmatrix}$. Only one eigenvector (up to scaling): $\mathbf{v}_1 = \begin{pmatrix}1\\0\end{pmatrix}$.

Find $\mathbf{v}_2$: $(A-2I)\mathbf{v}_2 = \mathbf{v}_1$: $\begin{pmatrix}0&1\\0&0\end{pmatrix}\mathbf{v}_2 = \begin{pmatrix}1\\0\end{pmatrix}$. So $v_{2,2} = 1$, $v_{2,1}$ free. Take $\mathbf{v}_2 = \begin{pmatrix}0\\1\end{pmatrix}$.

Solutions: $\mathbf{x}_1 = e^{2t}\begin{pmatrix}1\\0\end{pmatrix}$ and $\mathbf{x}_2 = e^{2t}\left(t\begin{pmatrix}1\\0\end{pmatrix} + \begin{pmatrix}0\\1\end{pmatrix}\right) = e^{2t}\begin{pmatrix}t\\1\end{pmatrix}$.

General solution: $\mathbf{x}(t) = c_1 e^{2t}\begin{pmatrix}1\\0\end{pmatrix} + c_2 e^{2t}\begin{pmatrix}t\\1\end{pmatrix} = e^{2t}\begin{pmatrix}c_1 + c_2 t\\ c_2\end{pmatrix}$.

## Jordan Form

The Jordan normal form theorem: every complex matrix $A$ is similar to a Jordan matrix $J = P^{-1}AP$ consisting of Jordan blocks. For real matrices with complex eigenvalues, the real Jordan form uses $2 \times 2$ rotation blocks. The solution theory is expressed cleanly in terms of the Jordan form: $e^{At} = Pe^{Jt}P^{-1}$, where $e^{Jt}$ is block-diagonal with Jordan blocks $e^{\lambda t}(I + (A-\lambda I)t + (A-\lambda I)^2 t^2/2 + \cdots)$ (a finite sum by Cayley-Hamilton).
