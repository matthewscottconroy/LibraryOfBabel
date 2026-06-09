# Linear Independence

A collection of vectors is linearly independent when no one of them is redundant — no vector in the collection can be expressed as a linear combination of the others. This concept captures the idea of "non-overlapping directions" in a vector space and is central to the notion of a basis: a basis is a maximal linearly independent set, or equivalently a minimal spanning set. In the context of differential equations, a set of $n$ solutions to an $n$-th order linear ODE is the basis for the solution space iff it is linearly independent.

## Definition

**Definition.** Vectors $v_1, v_2, \ldots, v_k$ in a vector space $V$ are **linearly independent** if the only solution to
$$\alpha_1 v_1 + \alpha_2 v_2 + \cdots + \alpha_k v_k = \mathbf{0}$$
is $\alpha_1 = \alpha_2 = \cdots = \alpha_k = 0$.

They are **linearly dependent** if there exist scalars $\alpha_1, \ldots, \alpha_k$, not all zero, with $\sum \alpha_i v_i = \mathbf{0}$.

Equivalently, $v_1, \ldots, v_k$ are linearly dependent iff some $v_j$ is a linear combination of the others (the one with $\alpha_j \neq 0$ can be solved for in terms of the rest).

## Examples in $\mathbb{R}^n$

**Linearly independent:** The standard basis vectors $e_1 = (1,0,0)$, $e_2 = (0,1,0)$, $e_3 = (0,0,1)$ in $\mathbb{R}^3$. If $\alpha_1 e_1 + \alpha_2 e_2 + \alpha_3 e_3 = (\alpha_1, \alpha_2, \alpha_3) = (0,0,0)$, then $\alpha_1 = \alpha_2 = \alpha_3 = 0$.

**Linearly dependent:** $(1,2)$ and $(2,4)$ in $\mathbb{R}^2$: $2(1,2) - 1(2,4) = (0,0)$.

**Test via determinant:** In $\mathbb{R}^n$, $n$ vectors $v_1, \ldots, v_n$ are linearly independent iff the matrix $[v_1 | v_2 | \cdots | v_n]$ has nonzero determinant.

## Examples in Function Spaces

**$\{1, x, x^2, \ldots, x^n\}$ in $C(\mathbb{R})$:** Linearly independent. If $\alpha_0 + \alpha_1 x + \cdots + \alpha_n x^n = 0$ for all $x$, then all coefficients are $0$ (a nonzero polynomial has finitely many roots, but it equals $0$ for all $x$).

**$\{e^{ax}, e^{bx}\}$ for $a \neq b$:** Linearly independent. If $c_1 e^{ax} + c_2 e^{bx} = 0$ for all $x$, divide by $e^{ax}$: $c_1 + c_2 e^{(b-a)x} = 0$ for all $x$. Differentiate: $c_2(b-a)e^{(b-a)x} = 0$ for all $x$. Since $b \neq a$, $c_2 = 0$. Then $c_1 = 0$.

**$\{\sin x, \cos x\}$:** Linearly independent. If $c_1\sin x + c_2\cos x = 0$ for all $x$, set $x = 0$: $c_2 = 0$. Set $x = \pi/2$: $c_1 = 0$.

## The Wronskian

For $n$ functions $y_1, \ldots, y_n \in C^{n-1}(I)$, the **Wronskian** is
$$W(y_1,\ldots,y_n)(t) = \det\begin{pmatrix} y_1 & y_2 & \cdots & y_n \\ y_1' & y_2' & \cdots & y_n' \\ \vdots & & & \vdots \\ y_1^{(n-1)} & y_2^{(n-1)} & \cdots & y_n^{(n-1)} \end{pmatrix}.$$

**Theorem.** If $y_1, \ldots, y_n$ are solutions of $L[y] = y^{(n)} + p_{n-1}y^{(n-1)} + \cdots + p_0 y = 0$ on $I$, then either $W \equiv 0$ on $I$ or $W$ is never $0$ on $I$. Moreover, $y_1, \ldots, y_n$ are linearly independent iff $W \neq 0$ at some (equivalently, every) point of $I$.

*Proof sketch.* Abel's identity: $W'(t) = -p_{n-1}(t)W(t)$, a first-order linear ODE. Its solution is $W(t) = W(t_0)e^{-\int_{t_0}^t p_{n-1}(s)\,ds}$, which is either identically zero (if $W(t_0) = 0$) or never zero. The connection to linear independence is through the fact that linear dependence implies the rows of the Wronskian matrix are linearly dependent at every point, giving $W \equiv 0$.

**Example.** For $y_1 = e^x$ and $y_2 = xe^x$ (solutions of $y'' - 2y' + y = 0$):
$$W(e^x, xe^x) = \det\begin{pmatrix} e^x & xe^x \\ e^x & e^x + xe^x\end{pmatrix} = e^x(e^x + xe^x) - xe^x\cdot e^x = e^{2x} \neq 0.$$
So they are linearly independent.

## Maximal Linearly Independent Sets

**Theorem.** If $v_1, \ldots, v_k$ are linearly independent and $v_{k+1} \notin \text{span}\{v_1, \ldots, v_k\}$, then $v_1, \ldots, v_{k+1}$ are linearly independent.

This allows one to build up a maximal linearly independent set incrementally — adding vectors as long as they are not in the span of those already chosen. The result is a basis.

## Linear Independence and the Solution Space

The existence and uniqueness theorem for $L[y] = 0$ guarantees exactly $n$ independent solutions for an $n$-th order equation. These $n$ solutions form a basis for the solution space (called a **fundamental system** or **fundamental set** of solutions). The general solution
$$y = c_1 y_1 + c_2 y_2 + \cdots + c_n y_n$$
is well-defined — meaning every solution has a unique representation in terms of $y_1, \ldots, y_n$ — precisely because the $y_i$ are linearly independent.

## Common Pitfalls

**$W = 0$ does not always imply linear dependence for non-solutions.** For general functions, $W = 0$ everywhere does not guarantee linear dependence; it only works when the functions are solutions to the same linear ODE. The functions $f(x) = x^2$ and $g(x) = x|x|$ have $W = 0$ everywhere but are linearly independent on $\mathbb{R}$.

**Checking at one point is not enough.** To test whether arbitrary functions are linearly dependent, checking the equation $\sum \alpha_i v_i = \mathbf{0}$ at a single point is insufficient (a linear combination might vanish at one point but not identically). A genuine proof of independence requires the equation to hold as an identity.
