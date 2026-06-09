# Chapter 03 Total Derivative

The partial derivatives $\partial f/\partial x$ and $\partial f/\partial y$ tell us how $f$ changes in the $x$- and $y$-directions. But they do not, by themselves, determine how $f$ changes in an arbitrary direction, and a function can possess all partial derivatives at a point while failing to behave nicely in other directions. The correct generalization of the derivative to several variables is the **total derivative** (or **differential**): a linear map $Df(\mathbf{a}): \mathbb{R}^n \to \mathbb{R}^m$ that best approximates the function near $\mathbf{a}$ in all directions simultaneously.

## What This Chapter Covers

**Section 1 (Differentiability and Linear Approximation)** gives the definition of differentiability for functions of several variables. A function $f: \mathbb{R}^n \to \mathbb{R}^m$ is differentiable at $\mathbf{a}$ if there exists a linear map $L: \mathbb{R}^n \to \mathbb{R}^m$ such that

$$\lim_{\mathbf{h}\to\mathbf{0}} \frac{\|f(\mathbf{a}+\mathbf{h}) - f(\mathbf{a}) - L(\mathbf{h})\|}{\|\mathbf{h}\|} = 0.$$

When such $L$ exists, it is unique and is called the **total derivative** $Df(\mathbf{a})$ or the **Fréchet derivative**. The condition says that $f(\mathbf{a}+\mathbf{h}) \approx f(\mathbf{a}) + Df(\mathbf{a})(\mathbf{h})$, with error $o(\|\mathbf{h}\|)$ — smaller than first order in $\|\mathbf{h}\|$. The section proves that differentiability implies continuity, and that continuous partial derivatives imply differentiability.

**Section 2 (The Jacobian Matrix)** shows that if $f: \mathbb{R}^n \to \mathbb{R}^m$ is differentiable at $\mathbf{a}$, the matrix of the linear map $Df(\mathbf{a})$ with respect to the standard bases is the **Jacobian matrix** $J_f(\mathbf{a})$, whose $(i,j)$ entry is $\partial f_i/\partial x_j$. For $f: \mathbb{R}^n \to \mathbb{R}$, the Jacobian is a $1\times n$ row vector, which is the transpose of the gradient. For $f: \mathbb{R}^n \to \mathbb{R}^n$, the Jacobian is an $n\times n$ square matrix, and its determinant (the Jacobian determinant) measures the local volume scaling of the map.

**Section 3 (Multivariable Chain Rule)** generalizes the chain rule to compositions of differentiable maps. If $f: \mathbb{R}^n \to \mathbb{R}^m$ is differentiable at $\mathbf{a}$ and $g: \mathbb{R}^m \to \mathbb{R}^k$ is differentiable at $f(\mathbf{a})$, then $g\circ f$ is differentiable at $\mathbf{a}$ and $D(g\circ f)(\mathbf{a}) = Dg(f(\mathbf{a}))\circ Df(\mathbf{a})$. In terms of Jacobian matrices, this is matrix multiplication.

## How the Sections Build on Each Other

Section 1 establishes the definition and proves the key theorem that continuous partial derivatives imply differentiability. Section 2 identifies the Jacobian as the matrix of the total derivative, making it computable. Section 3 derives the chain rule from the definition of differentiability. The three sections together replace the ad hoc notion of "differentiating each component separately" with a coherent linear algebraic framework.

## How This Chapter Fits into the Unit

The total derivative is the central object of multivariable differentiation. The gradient (Chapter 4) is the vector form of the total derivative for scalar functions. The Hessian (Chapter 5) is the total derivative of the gradient. The optimization theory (Chapter 6) uses both. The implicit function theorem (Chapter 7) is a theorem about when a system of equations has a differentiable solution, stated in terms of the Jacobian. The connection to differential equations is direct: the linearization of a system of differential equations at an equilibrium point is determined by the Jacobian matrix at that point, and the stability of the equilibrium depends on the eigenvalues of that Jacobian.
