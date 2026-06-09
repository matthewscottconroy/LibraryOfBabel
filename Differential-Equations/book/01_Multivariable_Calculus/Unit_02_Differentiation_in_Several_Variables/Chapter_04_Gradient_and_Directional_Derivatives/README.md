# Chapter 04 Gradient and Directional Derivatives

For scalar-valued functions $f: \mathbb{R}^n \to \mathbb{R}$, the total derivative at a point takes a particularly useful form: it is represented not by a matrix but by a vector, the **gradient** $\nabla f$. The gradient encodes the rates of change of $f$ in all directions simultaneously, through the directional derivative formula $D_{\hat{\mathbf{u}}} f = \nabla f \cdot \hat{\mathbf{u}}$. The geometric interpretation of this formula is elegant and powerful: the gradient points in the direction of steepest ascent, is perpendicular to the level sets of $f$, and its magnitude equals the maximum rate of change of $f$ at that point.

## What This Chapter Covers

**Section 1 (Gradient Vector)** defines the gradient as the vector of partial derivatives, $\nabla f = (f_{x_1}, \ldots, f_{x_n})$, and develops its algebraic properties (linearity, product rule, chain rule for the gradient). The gradient can be thought of as the transpose of the Jacobian for scalar functions, or equivalently as the unique vector satisfying $Df(\mathbf{a})(\mathbf{h}) = \nabla f(\mathbf{a})\cdot\mathbf{h}$ for all $\mathbf{h}$.

**Section 2 (Directional Derivatives)** defines the directional derivative in direction $\hat{\mathbf{u}}$ as $D_{\hat{\mathbf{u}}}f(\mathbf{a}) = \lim_{h\to 0}(f(\mathbf{a}+h\hat{\mathbf{u}})-f(\mathbf{a}))/h$, and proves that for differentiable $f$, this equals $\nabla f(\mathbf{a})\cdot\hat{\mathbf{u}}$. The directional derivative in the direction $\mathbf{e}_i$ (the $i$-th coordinate vector) recovers $\partial f/\partial x_i$, so directional derivatives generalize partial derivatives.

**Section 3 (Gradient as Normal Vector to Level Sets)** establishes the fundamental geometric theorem: if $f$ is differentiable at $\mathbf{a}$ and $c = f(\mathbf{a})$ is a regular value, then $\nabla f(\mathbf{a})$ is perpendicular to the level set $\{f = c\}$ at $\mathbf{a}$. This means the gradient is a normal vector to the level surface, and $\nabla f(\mathbf{a})\cdot(\mathbf{x}-\mathbf{a}) = 0$ is the equation of the tangent hyperplane to the level set.

**Section 4 (Steepest Ascent and Descent)** uses the Cauchy-Schwarz inequality to show that $D_{\hat{\mathbf{u}}}f = \nabla f\cdot\hat{\mathbf{u}} \leq \|\nabla f\|$, with equality when $\hat{\mathbf{u}} = \nabla f/\|\nabla f\|$. Therefore the gradient direction is the direction of steepest ascent, and the negative gradient is the direction of steepest descent. This is the theoretical basis for gradient descent optimization.

## How the Sections Build on Each Other

The gradient (Section 1) and directional derivative (Section 2) are defined independently and then related by the key formula in Section 2. The normal-vector interpretation (Section 3) is a geometric consequence of the directional derivative formula: a curve on the level set has zero directional derivative along the curve, so the gradient is perpendicular to all tangent vectors of the level set. The steepest-ascent result (Section 4) is an application of Cauchy-Schwarz to the directional derivative formula.

## How This Chapter Fits into the Unit

The gradient is the workhorse of multivariable calculus. It appears in optimization (Chapter 6), where setting the gradient to zero identifies critical points. It appears in the Lagrange multiplier theorem, where the gradients of the objective and constraint functions must be proportional. The Hessian (Chapter 5) is the gradient of the gradient (the Jacobian of the gradient map). The implicit function theorem (Chapter 7) is stated in terms of whether certain gradients are nonzero. In partial differential equations, the gradient appears in the divergence $\nabla\cdot\mathbf{F}$, the curl $\nabla\times\mathbf{F}$, and the Laplacian $\nabla^2 f = \nabla\cdot(\nabla f)$.
