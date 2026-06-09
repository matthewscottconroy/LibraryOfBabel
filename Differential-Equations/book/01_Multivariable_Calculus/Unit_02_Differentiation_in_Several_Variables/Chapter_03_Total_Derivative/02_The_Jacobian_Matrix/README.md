# The Jacobian Matrix

The total derivative $Df(\mathbf{a})$ of a differentiable map $f: \mathbb{R}^n \to \mathbb{R}^m$ at a point $\mathbf{a}$ is a linear map from $\mathbb{R}^n$ to $\mathbb{R}^m$. Every linear map between finite-dimensional spaces is represented by a matrix once bases are chosen. In the standard bases of $\mathbb{R}^n$ and $\mathbb{R}^m$, the representing matrix is the **Jacobian matrix**, and its entries are partial derivatives. The Jacobian is simultaneously the most computationally accessible representation of the total derivative and, through its determinant, a measure of local volume change.

## Construction of the Jacobian

Let $f = (f_1, f_2, \ldots, f_m): D\subseteq\mathbb{R}^n\to\mathbb{R}^m$ be differentiable at $\mathbf{a}$. The total derivative $Df(\mathbf{a})$ is the unique linear map $L:\mathbb{R}^n\to\mathbb{R}^m$ satisfying the approximation condition. To find the matrix of $L$, evaluate it on the standard basis vectors $\mathbf{e}_j \in \mathbb{R}^n$.

Since $L(\mathbf{e}_j) = \lim_{h\to 0}\frac{f(\mathbf{a}+h\mathbf{e}_j)-f(\mathbf{a})}{h}$ (using the defining property of $L$ with $\mathbf{h} = h\mathbf{e}_j$), and this limit equals the $j$-th partial derivative vector $\left(\frac{\partial f_1}{\partial x_j}(\mathbf{a}), \ldots, \frac{\partial f_m}{\partial x_j}(\mathbf{a})\right)^T$. So the $j$-th column of the matrix of $L$ is the vector of partial derivatives of $f$ with respect to $x_j$. Assembling all columns, the **Jacobian matrix** is the $m\times n$ matrix:

$$J_f(\mathbf{a}) = \frac{\partial(f_1,\ldots,f_m)}{\partial(x_1,\ldots,x_n)}\bigg|_{\mathbf{a}} = \begin{pmatrix} \frac{\partial f_1}{\partial x_1} & \frac{\partial f_1}{\partial x_2} & \cdots & \frac{\partial f_1}{\partial x_n} \\ \frac{\partial f_2}{\partial x_1} & \frac{\partial f_2}{\partial x_2} & \cdots & \frac{\partial f_2}{\partial x_n} \\ \vdots & & & \vdots \\ \frac{\partial f_m}{\partial x_1} & \frac{\partial f_m}{\partial x_2} & \cdots & \frac{\partial f_m}{\partial x_n} \end{pmatrix}.$$

Row $i$ contains the partial derivatives of $f_i$; column $j$ contains the partial derivatives with respect to $x_j$.

## Special Cases

**Scalar functions ($m=1$):** $f:\mathbb{R}^n\to\mathbb{R}$. The Jacobian is a $1\times n$ row vector $J_f(\mathbf{a}) = (f_{x_1}, f_{x_2}, \ldots, f_{x_n})$. The total derivative acts on $\mathbf{h}\in\mathbb{R}^n$ by $Df(\mathbf{a})(\mathbf{h}) = \nabla f(\mathbf{a})\cdot\mathbf{h}$ (dot product). The gradient $\nabla f$ is the transpose of the Jacobian row vector.

**Functions $f:\mathbb{R}^1\to\mathbb{R}^m$:** These are parametric curves. The Jacobian is an $m\times 1$ column vector $J_f(a) = \mathbf{r}'(a) = (f_1'(a), \ldots, f_m'(a))^T$, the velocity vector.

**Square maps ($n=m$):** $f:\mathbb{R}^n\to\mathbb{R}^n$. The Jacobian is an $n\times n$ matrix. Its determinant $\det J_f(\mathbf{a})$, the **Jacobian determinant**, measures the local volume scaling: near $\mathbf{a}$, the map $f$ stretches or compresses volumes by a factor of approximately $|\det J_f(\mathbf{a})|$. This is the quantity that appears in the change-of-variables formula for multiple integrals.

## Worked Example

Let $f:\mathbb{R}^3\to\mathbb{R}^2$ be $f(x,y,z) = (x^2y + z, \sin(xy))$.

$f_1 = x^2y + z$: $\partial f_1/\partial x = 2xy$, $\partial f_1/\partial y = x^2$, $\partial f_1/\partial z = 1$.

$f_2 = \sin(xy)$: $\partial f_2/\partial x = y\cos(xy)$, $\partial f_2/\partial y = x\cos(xy)$, $\partial f_2/\partial z = 0$.

$$J_f(x,y,z) = \begin{pmatrix} 2xy & x^2 & 1 \\ y\cos(xy) & x\cos(xy) & 0 \end{pmatrix}.$$

At the point $(1, 0, 1)$: $f(1,0,1) = (0+1, 0) = (1,0)$, and

$$J_f(1,0,1) = \begin{pmatrix} 0 & 1 & 1 \\ 0 & 1 & 0\end{pmatrix}.$$

The linear approximation is $f(1+h_1, h_2, 1+h_3) \approx (1,0) + J_f(1,0,1)(h_1,h_2,h_3)^T = (1 + h_2 + h_3, h_2)$.

## The Jacobian Determinant and Volume

For a differentiable map $f:\mathbb{R}^n\to\mathbb{R}^n$, the **Jacobian determinant** $J = \det(J_f(\mathbf{a}))$ has the following geometric meaning:

If $R$ is a small region near $\mathbf{a}$ with volume $V(R)$, then $V(f(R)) \approx |J|\cdot V(R)$ to first order.

This is the multivariable analogue of the single-variable substitution: if $u = g(x)$, then $du = g'(x)\,dx$, and $|g'(x)|$ is the local scaling of the real line. In the change-of-variables formula for double and triple integrals:

$$\iint_D f(\mathbf{x})\,dA = \iint_{D^*} f(\mathbf{g}(\mathbf{u}))\,|\det J_\mathbf{g}(\mathbf{u})|\,dA^*,$$

where $\mathbf{g}: D^* \to D$ is the change of variables, the factor $|\det J_\mathbf{g}|$ accounts for the local area (or volume) distortion.

**Example (polar coordinates).** $g(r,\theta) = (r\cos\theta, r\sin\theta)$.

$$J_g = \begin{pmatrix}\cos\theta & -r\sin\theta \\ \sin\theta & r\cos\theta\end{pmatrix}, \quad \det J_g = r\cos^2\theta + r\sin^2\theta = r.$$

The area element becomes $dA = r\,dr\,d\theta$, as used in double integrals.

## Jacobian of the Composition (Preview of Chain Rule)

If $f:\mathbb{R}^n\to\mathbb{R}^m$ and $g:\mathbb{R}^m\to\mathbb{R}^k$, then $J_{g\circ f}(\mathbf{a}) = J_g(f(\mathbf{a}))\cdot J_f(\mathbf{a})$ (matrix product). This is the chain rule in matrix form, derived in the next section.

## Common Pitfalls

The Jacobian is an $m\times n$ matrix, not an $n\times m$ matrix. Row $i$ corresponds to output component $f_i$; column $j$ corresponds to input variable $x_j$. Transposing the Jacobian gives the Jacobian of the transpose, which is a different linear map. Students sometimes confuse the Jacobian (the matrix) with the Jacobian determinant (a scalar, only defined when $m = n$).

The determinant of a non-square matrix is not defined. If $f:\mathbb{R}^3\to\mathbb{R}^2$, then $J_f$ is $2\times 3$, and one speaks of its rank (at most 2), not its determinant.
