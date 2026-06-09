# Hessian Matrix

In single-variable calculus, the second derivative $f''(a)$ measures the concavity of $f$ at $a$: positive means the graph curves upward (like a bowl), negative means it curves downward (like an arch). At a critical point where $f'(a)=0$, the sign of $f''(a)$ determines whether the point is a local minimum, maximum, or inconclusive. In several variables, the second-order behavior of $f$ is more complex: the function can curve upward in some directions and downward in others. The **Hessian matrix** is the correct generalization of $f''$ to several variables, encoding this directional curvature information in a single symmetric matrix.

## Definition

Let $f: D\subseteq\mathbb{R}^n\to\mathbb{R}$ be of class $C^2$ on an open set $U\subseteq D$. The **Hessian matrix** (or **Hessian**) of $f$ at $\mathbf{a}\in U$ is the $n\times n$ matrix

$$H_f(\mathbf{a}) = \begin{pmatrix} f_{x_1x_1} & f_{x_1x_2} & \cdots & f_{x_1x_n} \\ f_{x_2x_1} & f_{x_2x_2} & \cdots & f_{x_2x_n} \\ \vdots & & \ddots & \vdots \\ f_{x_nx_1} & f_{x_nx_2} & \cdots & f_{x_nx_n} \end{pmatrix}_{\mathbf{a}},$$

where $f_{x_ix_j} = \frac{\partial^2 f}{\partial x_j\partial x_i}$ (differentiate with respect to $x_i$ first, then $x_j$; note: some texts use the opposite order, since for $C^2$ functions it doesn't matter). By Clairaut's theorem, $f_{x_ix_j} = f_{x_jx_i}$ when $f\in C^2$, so $H_f$ is **symmetric**: $H_f = H_f^T$.

Equivalently, $H_f = J(\nabla f)$: the Hessian is the Jacobian of the gradient map $\nabla f:\mathbb{R}^n\to\mathbb{R}^n$. In components, $(H_f)_{ij} = \frac{\partial}{\partial x_j}\left(\frac{\partial f}{\partial x_i}\right) = \frac{\partial^2 f}{\partial x_j\partial x_i}$.

## The Hessian as a Quadratic Form

The Hessian defines a **quadratic form** on $\mathbb{R}^n$:

$$Q_H(\mathbf{h}) = \mathbf{h}^T H_f(\mathbf{a}) \mathbf{h} = \sum_{i,j=1}^n f_{x_ix_j}(\mathbf{a})\,h_i h_j.$$

This quadratic form appears in the second-order Taylor approximation (next section) and determines the local geometry of $f$ near a critical point.

For $f:\mathbb{R}^2\to\mathbb{R}$ with $(x,y)$ coordinates:

$$H_f = \begin{pmatrix}f_{xx} & f_{xy} \\ f_{yx} & f_{yy}\end{pmatrix}, \qquad Q_H(h_1,h_2) = f_{xx}h_1^2 + 2f_{xy}h_1h_2 + f_{yy}h_2^2.$$

## Worked Examples

**Example 1.** $f(x,y) = x^3 + x^2y - y^2$.

$f_x = 3x^2+2xy$, $f_y = x^2-2y$.
$f_{xx} = 6x+2y$, $f_{yy} = -2$, $f_{xy} = f_{yx} = 2x$.

$$H_f(x,y) = \begin{pmatrix}6x+2y & 2x \\ 2x & -2\end{pmatrix}.$$

At $(1,1)$: $H_f = \begin{pmatrix}8 & 2 \\ 2 & -2\end{pmatrix}$.

**Example 2.** $f(x,y,z) = x^2+y^2+z^2+xy$.

$f_x = 2x+y$, $f_y = 2y+x$, $f_z = 2z$.
$f_{xx} = 2$, $f_{yy} = 2$, $f_{zz} = 2$, $f_{xy} = f_{yx} = 1$, $f_{xz} = f_{yz} = 0$.

$$H_f = \begin{pmatrix}2 & 1 & 0 \\ 1 & 2 & 0 \\ 0 & 0 & 2\end{pmatrix}.$$

## Definite, Negative Definite, and Indefinite Matrices

A symmetric matrix $A$ is:
- **Positive definite** if $\mathbf{h}^T A\mathbf{h} > 0$ for all $\mathbf{h}\neq\mathbf{0}$. Equivalently, all eigenvalues of $A$ are positive. Equivalently (Sylvester's criterion), all leading principal minors are positive.
- **Negative definite** if $\mathbf{h}^T A\mathbf{h} < 0$ for all $\mathbf{h}\neq\mathbf{0}$. Equivalently, all eigenvalues are negative.
- **Indefinite** if $\mathbf{h}^T A\mathbf{h}$ takes both positive and negative values. This happens when $A$ has eigenvalues of both signs.
- **Positive semidefinite** if $\mathbf{h}^T A\mathbf{h} \geq 0$ for all $\mathbf{h}$, but some $\mathbf{h}\neq\mathbf{0}$ gives $0$.

For $2\times 2$ symmetric $A = \begin{pmatrix}a&b\\b&d\end{pmatrix}$:
- Positive definite iff $a > 0$ and $ad-b^2 > 0$.
- Negative definite iff $a < 0$ and $ad-b^2 > 0$.
- Indefinite iff $ad-b^2 < 0$.
- Inconclusive if $ad-b^2 = 0$.

The quantity $D = f_{xx}f_{yy} - (f_{xy})^2 = \det H_f$ is the **discriminant**.

## Spectral Theory and Eigenvalues

Since $H_f$ is symmetric, the spectral theorem guarantees it is diagonalizable over $\mathbb{R}$: all eigenvalues are real, and there is an orthonormal basis of eigenvectors. In the eigenvector basis, the quadratic form $Q_H(\mathbf{h}) = \sum_i \lambda_i v_i^2$ where $v_i = \mathbf{h}\cdot\mathbf{e}_i$ (projection of $\mathbf{h}$ onto the $i$-th eigenvector). Thus:
- All $\lambda_i > 0$: $Q_H > 0$ for all $\mathbf{h}\neq\mathbf{0}$ (positive definite, minimum shape).
- All $\lambda_i < 0$: $Q_H < 0$ for all $\mathbf{h}\neq\mathbf{0}$ (negative definite, maximum shape).
- Mixed signs: $Q_H$ can be positive or negative (indefinite, saddle shape).

## Connection to the Principal Axes Theorem

The eigenvectors of $H_f$ at a critical point are the **principal directions** of curvature: the directions in which the function curves most steeply or least steeply. The corresponding eigenvalues are the **principal curvatures** (in the second-order sense). For an elliptic paraboloid $z = ax^2+by^2$, the Hessian is $\begin{pmatrix}2a&0\\0&2b\end{pmatrix}$ with eigenvalues $2a$ and $2b$, and the principal directions are the coordinate axes.

## Hessian in Numerical Analysis

Newton's method for minimizing $f:\mathbb{R}^n\to\mathbb{R}$ iterates:

$$\mathbf{x}_{k+1} = \mathbf{x}_k - [H_f(\mathbf{x}_k)]^{-1}\nabla f(\mathbf{x}_k),$$

using both the gradient and the Hessian. This converges much faster than gradient descent (quadratically rather than linearly) near a minimum, but requires computing and inverting the $n\times n$ Hessian, which is expensive for large $n$. Quasi-Newton methods (like BFGS) approximate the Hessian using gradient information.
