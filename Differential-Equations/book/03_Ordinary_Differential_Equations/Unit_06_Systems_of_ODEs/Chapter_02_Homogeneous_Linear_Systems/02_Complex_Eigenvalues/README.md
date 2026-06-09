# Complex Eigenvalues

When $A$ is a real matrix with complex eigenvalues $\lambda = \alpha \pm \beta i$ ($\beta \neq 0$), the corresponding eigenvectors are complex conjugates $\mathbf{v} = \mathbf{a} \pm \mathbf{b}i$. The complex solution $(\mathbf{a} + \mathbf{b}i)e^{(\alpha+\beta i)t}$ decomposes into a real part and an imaginary part, each of which is a real solution.

## Deriving Real Solutions

The complex solution corresponding to $\lambda = \alpha + \beta i$ and $\mathbf{v} = \mathbf{a} + \mathbf{b}i$ is:

$$\mathbf{x}^{\text{complex}}(t) = (\mathbf{a}+\mathbf{b}i)e^{(\alpha+\beta i)t} = e^{\alpha t}(\mathbf{a}+\mathbf{b}i)(\cos\beta t + i\sin\beta t).$$

Real part: $\mathbf{x}_1(t) = e^{\alpha t}(\mathbf{a}\cos\beta t - \mathbf{b}\sin\beta t)$.
Imaginary part: $\mathbf{x}_2(t) = e^{\alpha t}(\mathbf{b}\cos\beta t + \mathbf{a}\sin\beta t)$.

Both $\mathbf{x}_1$ and $\mathbf{x}_2$ are real solutions of $\mathbf{x}' = A\mathbf{x}$, and they are linearly independent (their Wronskian is $\beta e^{2\alpha t} \neq 0$). The conjugate eigenvalue $\alpha - \beta i$ gives the same two real solutions.

## Worked Example

Solve $\mathbf{x}' = \begin{pmatrix}1&-2\\1&3\end{pmatrix}\mathbf{x}$.

Characteristic equation: $\lambda^2 - 4\lambda + 5 = 0$, roots $\lambda = 2 \pm i$.

For $\lambda = 2+i$: $(A - (2+i)I)\mathbf{v} = \mathbf{0}$: $\begin{pmatrix}-1-i&-2\\1&1-i\end{pmatrix}\mathbf{v} = \mathbf{0}$. From the first row: $(-1-i)v_1 = 2v_2$, so take $v_1 = 2$, $v_2 = -(1+i) = -1 - i$. Thus $\mathbf{v} = \begin{pmatrix}2\\-1-i\end{pmatrix} = \begin{pmatrix}2\\-1\end{pmatrix} + i\begin{pmatrix}0\\-1\end{pmatrix}$.

So $\mathbf{a} = \begin{pmatrix}2\\-1\end{pmatrix}$ and $\mathbf{b} = \begin{pmatrix}0\\-1\end{pmatrix}$, $\alpha = 2$, $\beta = 1$.

Real solutions:

$$\mathbf{x}_1(t) = e^{2t}\left[\begin{pmatrix}2\\-1\end{pmatrix}\cos t - \begin{pmatrix}0\\-1\end{pmatrix}\sin t\right] = e^{2t}\begin{pmatrix}2\cos t\\\sin t - \cos t\end{pmatrix}.$$

$$\mathbf{x}_2(t) = e^{2t}\left[\begin{pmatrix}0\\-1\end{pmatrix}\cos t + \begin{pmatrix}2\\-1\end{pmatrix}\sin t\right] = e^{2t}\begin{pmatrix}2\sin t\\-\cos t - \sin t\end{pmatrix}.$$

General solution: $\mathbf{x}(t) = c_1\mathbf{x}_1(t) + c_2\mathbf{x}_2(t)$.

## Phase Plane Behavior

With $\alpha = 2 > 0$, solutions spiral outward from the origin (unstable spiral). If $\alpha < 0$, they spiral inward (stable spiral). If $\alpha = 0$ (pure imaginary eigenvalues), they form closed ellipses (center), corresponding to purely oscillatory behavior.
