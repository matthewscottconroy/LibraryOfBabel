# The Wronskian and Linear Independence

For two functions $y_1$ and $y_2$ on an interval $I$, linear independence is not always easy to assess by inspection. The **Wronskian** provides a computable test: a single determinant whose vanishing or non-vanishing determines whether the functions are linearly dependent or independent, at least when they are solutions of a second-order linear ODE.

## Definition

The **Wronskian** of two differentiable functions $y_1$ and $y_2$ is

$$W(y_1, y_2)(x) = \begin{vmatrix} y_1(x) & y_2(x) \\ y_1'(x) & y_2'(x) \end{vmatrix} = y_1(x)\,y_2'(x) - y_1'(x)\,y_2(x).$$

For $n$ functions $y_1, \ldots, y_n$ that are $n-1$ times differentiable, the Wronskian is the $n \times n$ determinant of the matrix whose $(i,j)$ entry is $y_j^{(i-1)}$.

## Linear Independence in General

Two functions are **linearly dependent** on $I$ if one is a constant multiple of the other: $y_2 = cy_1$ for some constant $c$. More generally, $y_1, \ldots, y_n$ are linearly dependent if there exist constants $c_1, \ldots, c_n$, not all zero, such that $c_1 y_1 + \cdots + c_n y_n = 0$ on $I$.

If $y_2 = cy_1$, then $W(y_1, y_2) = y_1(cy_1)' - y_1'(cy_1) = cy_1 y_1' - cy_1' y_1 = 0$. So linear dependence implies $W = 0$.

The converse is **false** for general functions. For example, $y_1 = x^2$ and $y_2 = x|x|$ have $W(y_1, y_2) = 0$ on $(-1, 1)$ (at $x = 0$ both functions and their derivatives vanish), yet they are linearly independent because $y_2/y_1 = |x|/x = \pm 1$ is not constant.

## The Wronskian for Solutions of Linear ODEs

The situation is cleaner when $y_1$ and $y_2$ are solutions of a second-order linear homogeneous ODE.

**Theorem.** Let $y_1$ and $y_2$ be solutions of $y'' + p(x)y' + q(x)y = 0$ on $I$, with $p, q$ continuous. Then $W(y_1, y_2)(x)$ is either identically zero on $I$ or never zero on $I$.

**Proof.** This follows from Abel's identity: $W(x) = W(x_0)e^{-\int_{x_0}^x p(t)\,dt}$ (proved in the next section). Since the exponential is never zero, $W(x)$ has the same sign as $W(x_0)$ for all $x$. If $W(x_0) \neq 0$, then $W$ is never zero; if $W(x_0) = 0$, then $W \equiv 0$.

**Corollary (Wronskian Test).** Two solutions $y_1, y_2$ of the equation are linearly independent if and only if $W(y_1, y_2)(x_0) \neq 0$ for some (equivalently, any) $x_0 \in I$.

## Worked Examples

**Example 1.** Verify that $y_1 = \cos x$ and $y_2 = \sin x$ are linearly independent solutions of $y'' + y = 0$.

$W(\cos x, \sin x) = \cos x \cdot \cos x - (-\sin x)\cdot\sin x = \cos^2 x + \sin^2 x = 1 \neq 0$.

Since $W$ is never zero, $\{\cos x, \sin x\}$ is a fundamental set.

**Example 2.** Show that $y_1 = e^x$ and $y_2 = xe^x$ are linearly independent.

$W(e^x, xe^x) = e^x(e^x + xe^x) - e^x\cdot xe^x = e^{2x} + xe^{2x} - xe^{2x} = e^{2x} \neq 0$.

**Example 3.** Determine whether $y_1 = \sinh x$ and $y_2 = e^x - e^{-x}$ are linearly dependent.

Since $\sinh x = (e^x - e^{-x})/2$, we have $y_2 = 2\sinh x = 2y_1$. They are linearly dependent. Indeed, $W(y_1, y_2) = y_1(2y_1)' - y_1'(2y_1) = 2y_1 y_1' - 2y_1'y_1 = 0$.

## Geometric Interpretation

The Wronskian $W(y_1, y_2)(x_0)$ equals the area of the parallelogram spanned by the vectors $(y_1(x_0), y_1'(x_0))$ and $(y_2(x_0), y_2'(x_0))$ in the phase plane. Non-zero Wronskian means these vectors are not parallel at $x_0$, i.e., the two solutions have distinct phase-plane trajectories and therefore represent genuinely different behaviors.

When $W = 0$, the two solution curves in the phase plane are the same curve (one is a scalar multiple of the other), reflecting the linear dependence.

## The General $n$-th Order Case

For an $n$-th order linear homogeneous ODE with continuous coefficients, $n$ solutions $y_1, \ldots, y_n$ are linearly independent if and only if their Wronskian $W(y_1, \ldots, y_n)(x_0) \neq 0$ for some $x_0$. The solution space is $n$-dimensional, and any fundamental set consists of exactly $n$ linearly independent solutions.
