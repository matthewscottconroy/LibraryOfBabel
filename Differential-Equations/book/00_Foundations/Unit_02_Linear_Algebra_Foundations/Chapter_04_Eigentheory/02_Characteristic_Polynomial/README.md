# The Characteristic Polynomial

The characteristic polynomial is the algebraic object that encodes all eigenvalue information for a matrix. It transforms the question "what are the eigenvalues of $A$?" — a question about a linear map — into "what are the roots of $p(\lambda)$?" — a question about a polynomial. The polynomial's coefficients carry additional information: its constant term is (up to sign) the determinant, its second-highest-degree coefficient is the trace, and its full factorization over $\mathbb{C}$ reveals the eigenvalue multiplicities.

## Definition

**Definition.** The **characteristic polynomial** of an $n\times n$ matrix $A$ is
$$p(\lambda) = \det(\lambda I - A).$$

Expanding the determinant, $p(\lambda)$ is a polynomial of degree $n$ in $\lambda$.

**Theorem.** $\lambda_0$ is an eigenvalue of $A$ if and only if $p(\lambda_0) = 0$.

*Proof.* $Av = \lambda_0 v$ with $v \neq 0$ iff $(\lambda_0 I - A)v = 0$ has a nontrivial solution iff $\det(\lambda_0 I - A) = 0$ iff $p(\lambda_0) = 0$. $\square$

## Structure of the Characteristic Polynomial

Expanding $\det(\lambda I - A)$ for the general $n\times n$ case:
$$p(\lambda) = \lambda^n - (\text{tr}\,A)\lambda^{n-1} + \cdots + (-1)^n\det(A).$$

The leading coefficient is always $1$ (monic polynomial). The coefficient of $\lambda^{n-1}$ is $-\text{tr}(A) = -\sum_i a_{ii}$ (the negative of the trace). The constant term is $p(0) = \det(-A) = (-1)^n\det(A)$.

**Example.** $A = \begin{pmatrix}a&b\\c&d\end{pmatrix}$.
$$p(\lambda) = \det\begin{pmatrix}\lambda-a&-b\\-c&\lambda-d\end{pmatrix} = (\lambda-a)(\lambda-d) - bc = \lambda^2 - (a+d)\lambda + (ad-bc) = \lambda^2 - \text{tr}(A)\lambda + \det(A).$$

## Trace, Determinant, and Eigenvalues

Over $\mathbb{C}$, the Fundamental Theorem of Algebra guarantees that $p(\lambda)$ has exactly $n$ roots (counting multiplicity): the eigenvalues $\lambda_1, \ldots, \lambda_n$. The polynomial factors as $p(\lambda) = \prod_{i=1}^n(\lambda - \lambda_i)$. Comparing coefficients:
$$\text{tr}(A) = \sum_{i=1}^n \lambda_i, \qquad \det(A) = \prod_{i=1}^n \lambda_i.$$

These are remarkable relations: the trace is the sum of eigenvalues and the determinant is the product. They give quick information about eigenvalues without solving for them explicitly.

**Example.** If $A$ is $3\times 3$ with $\text{tr}(A) = 6$ and $\det(A) = 8$, and the characteristic polynomial factors as $(\lambda-1)(\lambda-2)(\lambda-??)$: since the product of eigenvalues is $8$ and two are $1$ and $2$, the third is $8/(1\cdot 2) = 4$.

## The Cayley-Hamilton Theorem

**Theorem (Cayley-Hamilton).** Every matrix satisfies its own characteristic polynomial: $p(A) = 0$.

More precisely: if $p(\lambda) = \lambda^n + c_{n-1}\lambda^{n-1} + \cdots + c_0$, then $A^n + c_{n-1}A^{n-1} + \cdots + c_0 I = 0$ (the zero matrix).

*Proof sketch.* Over $\mathbb{C}$, the Cayley-Hamilton theorem follows from the Jordan normal form: if $A = P J P^{-1}$ and $J$ consists of Jordan blocks with eigenvalue $\lambda_i$, then $p(A) = Pp(J)P^{-1}$, and $p(J) = 0$ block by block since $p(\lambda_i) = 0$ and each Jordan block $(\lambda_i I + N)$ satisfies $p(\lambda_i I + N) = 0$ by nilpotency of $N$.

**Application.** Cayley-Hamilton allows expressing $A^n$ and higher powers of $A$ in terms of $\{I, A, A^2, \ldots, A^{n-1}\}$, which bounds the computational cost of computing matrix powers and matrix exponentials.

**Example.** $A = \begin{pmatrix}0&1\\-1&-1\end{pmatrix}$. $p(\lambda) = \lambda^2 + \lambda + 1$. Cayley-Hamilton: $A^2 + A + I = 0$, so $A^2 = -A - I$.

## Algebraic Multiplicity

**Definition.** The **algebraic multiplicity** of eigenvalue $\lambda_0$ is its multiplicity as a root of $p(\lambda)$. The polynomial factors over $\mathbb{C}$ as $p(\lambda) = \prod_i (\lambda - \lambda_i)^{m_i}$ where $m_i$ is the algebraic multiplicity of $\lambda_i$ and $\sum_i m_i = n$.

The geometric multiplicity $\dim E_{\lambda_i}$ satisfies $1 \leq \dim E_{\lambda_i} \leq m_i$.

**Example.** $A = \begin{pmatrix}2&1&0\\0&2&1\\0&0&2\end{pmatrix}$ (Jordan block). $p(\lambda) = (\lambda-2)^3$. Algebraic multiplicity of $\lambda = 2$: $3$. Geometric multiplicity: $\dim\ker(2I - A) = \dim\ker\begin{pmatrix}0&-1&0\\0&0&-1\\0&0&0\end{pmatrix} = 1$. So $1 < 3$ — not diagonalizable.

## Computing the Characteristic Polynomial

For $2\times 2$ and $3\times 3$ matrices, cofactor expansion of $\det(\lambda I - A)$ is practical. For larger matrices:
- Direct computation via LU-type methods works but is $O(n^4)$ if done naively.
- The QR algorithm (Chapter 3.6) finds eigenvalues numerically without computing $p(\lambda)$ explicitly.
- For theoretical purposes, $p(\lambda) = \lambda^n - \text{tr}(A)\lambda^{n-1} + \cdots$ and specific coefficients can be computed via Newton's identities using the traces of powers $\text{tr}(A^k)$.

## Connection to Differential Equations

For a constant-coefficient scalar ODE $a_n y^{(n)} + a_{n-1}y^{(n-1)} + \cdots + a_0 y = 0$, the **characteristic polynomial** is $p(\lambda) = a_n\lambda^n + \cdots + a_0$, and the characteristic equation $p(\lambda) = 0$ gives the exponents $e^{\lambda t}$ that form the basis of solutions. This is the direct analog of the matrix characteristic polynomial: the scalar ODE corresponds to a companion matrix $A$, and $\det(\lambda I - A) = p(\lambda)/a_n$.

For the system $\mathbf{x}' = A\mathbf{x}$, the characteristic polynomial of $A$ determines the solution structure completely — which exponentials appear, with what multiplicities, whether polynomial factors arise (when eigenvalues have algebraic multiplicity $> 1$). The connection between the scalar ODE polynomial and the matrix characteristic polynomial makes the theory of linear ODEs a unified subject.
