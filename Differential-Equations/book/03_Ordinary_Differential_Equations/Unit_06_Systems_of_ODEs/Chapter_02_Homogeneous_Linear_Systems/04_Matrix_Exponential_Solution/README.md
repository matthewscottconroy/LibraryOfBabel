# The Matrix Exponential Solution

For the constant-coefficient system $\mathbf{x}' = A\mathbf{x}$, the unique solution with $\mathbf{x}(0) = \mathbf{x}_0$ is $\mathbf{x}(t) = e^{At}\mathbf{x}_0$, where $e^{At}$ is the **matrix exponential**. This is the matrix analog of the scalar formula $x(t) = e^{at}x_0$ for the scalar equation $x' = ax$.

## Definition

$$e^{At} = \sum_{k=0}^\infty \frac{(At)^k}{k!} = I + At + \frac{A^2t^2}{2!} + \frac{A^3t^3}{3!} + \cdots.$$

This series converges absolutely for all $t$ and all matrices $A$ (by comparison with the scalar series). The matrix $e^{At}$ satisfies:
- $e^{A\cdot 0} = I$
- $\frac{d}{dt}e^{At} = Ae^{At} = e^{At}A$
- $(e^{At})^{-1} = e^{-At}$
- $e^{A(s+t)} = e^{As}e^{At}$ (group property, valid since $A$ commutes with itself)

## Computing $e^{At}$

**Via diagonalization.** If $A = PDP^{-1}$ where $D = \mathrm{diag}(\lambda_1, \ldots, \lambda_n)$, then $e^{At} = Pe^{Dt}P^{-1} = P\,\mathrm{diag}(e^{\lambda_1 t}, \ldots, e^{\lambda_n t})P^{-1}$.

**Via Cayley-Hamilton.** The Cayley-Hamilton theorem states that $A$ satisfies its own characteristic equation: $p(A) = 0$ where $p(\lambda) = \det(\lambda I - A)$. Therefore $A^n, A^{n+1}, \ldots$ are all expressible as linear combinations of $I, A, \ldots, A^{n-1}$. This means $e^{At} = \alpha_0(t)I + \alpha_1(t)A + \cdots + \alpha_{n-1}(t)A^{n-1}$ for scalar functions $\alpha_k(t)$. Applying $e^{\lambda_k t} = \sum \alpha_j(t)\lambda_k^j$ (for each eigenvalue) gives a linear system for the $\alpha_k(t)$.

**Example for $2 \times 2$.** $e^{At} = \alpha_0(t)I + \alpha_1(t)A$. If $\lambda_1 \neq \lambda_2$: $e^{\lambda_1 t} = \alpha_0 + \alpha_1\lambda_1$ and $e^{\lambda_2 t} = \alpha_0 + \alpha_1\lambda_2$, giving $\alpha_1 = (e^{\lambda_1 t} - e^{\lambda_2 t})/(\lambda_1 - \lambda_2)$ and $\alpha_0 = e^{\lambda_1 t} - \alpha_1\lambda_1$. If $\lambda_1 = \lambda_2 = \lambda$: $e^{\lambda t} = \alpha_0 + \alpha_1\lambda$ and $te^{\lambda t} = \alpha_1$ (differentiating w.r.t. $\lambda$), giving $\alpha_1 = te^{\lambda t}$ and $\alpha_0 = (1-\lambda t)e^{\lambda t}$.

## The Fundamental Matrix

$e^{At}$ is the principal fundamental matrix at $t_0 = 0$: the unique solution of $\Phi' = A\Phi$, $\Phi(0) = I$. For the IVP $\mathbf{x}' = A\mathbf{x}$, $\mathbf{x}(0) = \mathbf{x}_0$: $\mathbf{x}(t) = e^{At}\mathbf{x}_0$.

## Connection to Eigenvalues

The eigenvalues of $e^{At}$ are $e^{\lambda_k t}$ where $\lambda_k$ are the eigenvalues of $A$. The spectral radius and stability of $e^{At}$ (as $t \to \infty$) are determined by the eigenvalues of $A$: all eigenvalues of $A$ have negative real part if and only if $e^{At} \to 0$ as $t \to \infty$ (asymptotic stability).
