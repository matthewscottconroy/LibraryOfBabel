# The Matrix Exponential

The matrix exponential $e^{At}$ is the fundamental solution object for linear ODE systems. Just as the scalar ODE $y' = ay$ is solved by $y(t) = e^{at}y_0$, the system $\mathbf{x}' = A\mathbf{x}$ is solved by $\mathbf{x}(t) = e^{At}\mathbf{x}_0$. The matrix exponential is defined by a power series and inherits the key properties of the scalar exponential, with the important caveat that $e^{A+B} \neq e^A e^B$ unless $A$ and $B$ commute.

## Definition via Power Series

**Definition.** For an $n\times n$ matrix $A$, the **matrix exponential** is
$$e^A = \sum_{k=0}^\infty \frac{A^k}{k!} = I + A + \frac{A^2}{2!} + \frac{A^3}{3!} + \cdots$$

**Theorem.** This series converges for every matrix $A$ (in any matrix norm).

*Proof.* Using the submultiplicative property $\|A^k\| \leq \|A\|^k$:
$$\sum_{k=0}^\infty \frac{\|A^k\|}{k!} \leq \sum_{k=0}^\infty \frac{\|A\|^k}{k!} = e^{\|A\|} < \infty.$$
The partial sums are Cauchy in the complete normed space of matrices, so the series converges. $\square$

## Properties

**(P1) Derivative:** $\frac{d}{dt}e^{At} = Ae^{At} = e^{At}A$.

*Proof.* Differentiate term by term: $\frac{d}{dt}\sum \frac{A^k t^k}{k!} = \sum_{k=1}^\infty \frac{A^k t^{k-1}}{(k-1)!} = A\sum_{j=0}^\infty \frac{A^j t^j}{j!} = Ae^{At}$. Term-by-term differentiation is valid because the series converges uniformly on compact intervals. $\square$

**(P2) Initial value:** $e^{A\cdot 0} = I$.

**(P3) Group property:** $e^{At}e^{As} = e^{A(t+s)}$.

**(P4) Inverse:** $(e^{At})^{-1} = e^{-At}$.

**(P5) Determinant:** $\det(e^{At}) = e^{(\text{tr}\,A)t}$.

*Proof of (P5).* The trace is $\sum \lambda_i$ and $\det(e^{At}) = \prod e^{\lambda_i t} = e^{\sum \lambda_i t} = e^{(\text{tr}\,A)t}$ — this can be made rigorous via Jordan form. $\square$

**Caution:** In general, $e^{A+B} \neq e^Ae^B$ unless $AB = BA$. Non-commutativity of matrices breaks the exponential law.

## Computation of $e^{At}$

**Case 1: Diagonalizable $A = PDP^{-1}$.**
$$e^{At} = Pe^{Dt}P^{-1} = P\begin{pmatrix}e^{\lambda_1 t} & & \\ & \ddots & \\ & & e^{\lambda_n t}\end{pmatrix}P^{-1}.$$

**Example.** $A = \begin{pmatrix}3&1\\1&3\end{pmatrix}$ with eigenvalues $\lambda_1 = 2$, $\lambda_2 = 4$ and eigenvectors $v_1 = (-1,1)^T$, $v_2 = (1,1)^T$.

$P = \begin{pmatrix}-1&1\\1&1\end{pmatrix}$, $P^{-1} = \frac{1}{-2}\begin{pmatrix}1&-1\\-1&-1\end{pmatrix} = \begin{pmatrix}-1/2&1/2\\1/2&1/2\end{pmatrix}$.

$$e^{At} = P\begin{pmatrix}e^{2t}&0\\0&e^{4t}\end{pmatrix}P^{-1} = \frac{1}{2}\begin{pmatrix}e^{2t}+e^{4t}&-e^{2t}+e^{4t}\\-e^{2t}+e^{4t}&e^{2t}+e^{4t}\end{pmatrix}.$$

**Case 2: Jordan form $A = PJP^{-1}$.**

For a $k\times k$ Jordan block $J_k(\lambda) = \lambda I + N$:
$$e^{J_k(\lambda)t} = e^{\lambda t}e^{Nt} = e^{\lambda t}\sum_{j=0}^{k-1}\frac{N^j t^j}{j!} = e^{\lambda t}\begin{pmatrix}1&t&\frac{t^2}{2!}&\cdots&\frac{t^{k-1}}{(k-1)!}\\0&1&t&\cdots&\frac{t^{k-2}}{(k-2)!}\\\vdots&&\ddots&&\vdots\\0&0&\cdots&0&1\end{pmatrix}.$$

(Since $\lambda I$ and $N$ commute, $e^{(\lambda I + N)t} = e^{\lambda It}e^{Nt}$.)

**Case 3: Cayley-Hamilton method (small matrices).**

By Cayley-Hamilton, $A^n = $ linear combination of $\{I, A, \ldots, A^{n-1}\}$. Every term $A^k$ (for $k \geq n$) can be reduced, so $e^{At} = \alpha_0(t)I + \alpha_1(t)A + \cdots + \alpha_{n-1}(t)A^{n-1}$ for some scalar functions $\alpha_i(t)$. These are determined by requiring $e^{\lambda_i t} = \sum_j \alpha_j(t)\lambda_i^j$ for each eigenvalue $\lambda_i$ (with appropriate conditions at repeated eigenvalues).

## The ODE Solution

**Theorem.** The unique solution to the initial value problem $\mathbf{x}' = A\mathbf{x}$, $\mathbf{x}(0) = \mathbf{x}_0$ is
$$\mathbf{x}(t) = e^{At}\mathbf{x}_0.$$

*Proof.* Let $\mathbf{x}(t) = e^{At}\mathbf{x}_0$. Then $\mathbf{x}(0) = e^0\mathbf{x}_0 = \mathbf{x}_0$ and $\mathbf{x}'(t) = Ae^{At}\mathbf{x}_0 = A\mathbf{x}(t)$. Uniqueness: if $\mathbf{y}$ is another solution, $\frac{d}{dt}[e^{-At}\mathbf{y}(t)] = -Ae^{-At}\mathbf{y} + e^{-At}A\mathbf{y} = 0$, so $e^{-At}\mathbf{y}(t) = \mathbf{y}(0) = \mathbf{x}_0$, giving $\mathbf{y}(t) = e^{At}\mathbf{x}_0 = \mathbf{x}(t)$. $\square$

**Non-homogeneous system.** For $\mathbf{x}' = A\mathbf{x} + g(t)$, variation of parameters gives:
$$\mathbf{x}(t) = e^{At}\mathbf{x}_0 + \int_0^t e^{A(t-s)}g(s)\,ds.$$

## Stability

The stability of $\mathbf{x} = \mathbf{0}$ is determined by the eigenvalues of $A$:
- **Asymptotically stable:** all eigenvalues have negative real part ($\text{Re}(\lambda_i) < 0$ for all $i$). Then $\|e^{At}\| \to 0$ as $t\to\infty$.
- **Unstable:** some eigenvalue has positive real part.
- **Neutral stability:** all $\text{Re}(\lambda_i) \leq 0$; if any $\text{Re}(\lambda_i) = 0$ with Jordan block of size $> 1$, the solution grows polynomially (unstable in Lyapunov sense).

The behavior of $e^{At}$ — growing, decaying, oscillating — is completely determined by the eigenvalue structure, and stability analysis reduces to the question of where the eigenvalues lie in the complex plane.
