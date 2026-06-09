# Matrix Form of Systems

Writing a system of first-order ODEs in matrix form $\mathbf{x}' = A(t)\mathbf{x} + \mathbf{g}(t)$ consolidates the equations into a single compact expression amenable to linear-algebraic analysis.

## The Reduction of Order

Any $n$-th order ODE $y^{(n)} = f(t, y, y', \ldots, y^{(n-1)})$ can be written as a first-order system by the substitution $x_1 = y$, $x_2 = y'$, ..., $x_n = y^{(n-1)}$. Then $x_k' = x_{k+1}$ for $k = 1, \ldots, n-1$ and $x_n' = f(t, x_1, \ldots, x_n)$.

For the linear equation $y^{(n)} + p_{n-1}y^{(n-1)} + \cdots + p_0 y = g(t)$:

$$\mathbf{x}' = \begin{pmatrix}0 & 1 & 0 & \cdots & 0\\ 0 & 0 & 1 & \cdots & 0\\ \vdots & & & \ddots & \vdots\\ 0 & 0 & 0 & \cdots & 1\\ -p_0 & -p_1 & -p_2 & \cdots & -p_{n-1}\end{pmatrix}\mathbf{x} + \begin{pmatrix}0\\0\\\vdots\\0\\g\end{pmatrix}.$$

The $n \times n$ coefficient matrix is the **companion matrix** of the characteristic polynomial $r^n + p_{n-1}r^{n-1} + \cdots + p_0$. Its eigenvalues are exactly the roots of the characteristic polynomial, which explains why the eigenvalue method for systems gives the same exponential solutions as the characteristic equation method for single equations.

## Existence and Uniqueness

For the system $\mathbf{x}' = A(t)\mathbf{x} + \mathbf{g}(t)$ with $A$ and $\mathbf{g}$ continuous on $I$, every IVP has a unique solution on all of $I$. This follows from Picard's theorem for systems: $\mathbf{f}(t, \mathbf{x}) = A(t)\mathbf{x} + \mathbf{g}(t)$ is Lipschitz in $\mathbf{x}$ with constant $\|A(t)\|$ (matrix norm), bounded on compact subsets of $I$.

## Worked Example: Coupled Oscillators

Two masses $m$ connected by springs (spring constants $k$ and $2k$) satisfy:

$$m\ddot{x}_1 = -kx_1 + k(x_2 - x_1) = -2kx_1 + kx_2,$$
$$m\ddot{x}_2 = -k(x_2 - x_1) - kx_2 = kx_1 - 2kx_2.$$

Setting $\omega_0^2 = k/m$: $\ddot{\mathbf{x}} = \omega_0^2\begin{pmatrix}-2&1\\1&-2\end{pmatrix}\mathbf{x}$. This is a second-order system, convertible to a first-order system by $u_1 = x_1$, $u_2 = \dot{x}_1$, $u_3 = x_2$, $u_4 = \dot{x}_2$.

The eigenvalues of $\begin{pmatrix}-2&1\\1&-2\end{pmatrix}$ are $-1$ and $-3$, giving oscillation frequencies $\omega_0$ and $\omega_0\sqrt{3}$: the two normal modes of the system.
