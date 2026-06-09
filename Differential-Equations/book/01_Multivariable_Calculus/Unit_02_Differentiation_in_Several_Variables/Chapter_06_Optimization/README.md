# Chapter 06 Optimization

Finding the maximum or minimum of a function — subject to constraints or not — is one of the most practically important tasks in mathematics. Single-variable optimization is governed by two simple rules: at an interior extremum, $f'(a) = 0$; and $f''(a)$ determines the type. In several variables, both rules generalize, but the second generalizes in a nontrivial direction: the second derivative becomes the Hessian matrix, and the "sign" of the Hessian must be determined by the theory of quadratic forms.

## What This Chapter Covers

**Section 1 (Critical Points and Classification)** defines a critical point of $f:\mathbb{R}^n\to\mathbb{R}$ as a point where $\nabla f = \mathbf{0}$, and explains why interior extrema must be critical points. Not every critical point is an extremum; in several variables, saddle points arise naturally and must be distinguished.

**Section 2 (Second Derivative Test)** states and proves the second derivative test for functions of two (and more) variables. The test uses the Hessian: if $H_f(\mathbf{a})$ is positive definite, $\mathbf{a}$ is a local minimum; if negative definite, a local maximum; if indefinite, a saddle point. For $n=2$, the discriminant $D = f_{xx}f_{yy} - f_{xy}^2$ captures the definiteness.

**Section 3 (Lagrange Multipliers)** addresses constrained optimization: finding the extrema of $f$ on a constraint set $\{g = c\}$. The method of Lagrange multipliers asserts that at a constrained extremum, the gradient of the objective function is proportional to the gradient of the constraint: $\nabla f = \lambda\nabla g$. The scalar $\lambda$ is the **Lagrange multiplier**. This method converts a constrained optimization problem into a system of equations.

**Section 4 (Constrained Optimization with Multiple Constraints)** generalizes the Lagrange method to $k$ constraints $g_1 = c_1, \ldots, g_k = c_k$. The condition becomes $\nabla f = \lambda_1\nabla g_1 + \cdots + \lambda_k\nabla g_k$, i.e., $\nabla f$ lies in the span of the constraint gradients.

## How the Sections Build on Each Other

Sections 1 and 2 handle unconstrained optimization; Sections 3 and 4 handle constrained optimization. Section 1 sets up the vocabulary; Section 2 develops the test; Section 3 introduces Lagrange multipliers; Section 4 extends them. The Lagrange multiplier condition $\nabla f = \lambda\nabla g$ can be understood via the gradient-perpendicular-to-level-sets geometry (Chapter 4): at a constrained extremum on $\{g=c\}$, the gradient of $f$ must be perpendicular to the constraint surface, which means it must be parallel to $\nabla g$ (the normal to the constraint surface).

## How This Chapter Fits into the Unit

Optimization is the most directly applicable topic in multivariable calculus. In economics, one maximizes utility or profit subject to budget constraints (Lagrange multipliers). In physics, one minimizes energy (variational principles). In machine learning, one minimizes loss functions (gradient descent from Chapter 4, second-order methods using the Hessian). In geometry, one finds geodesics (shortest paths on surfaces) by constrained optimization. The implicit function theorem (Chapter 7) provides the rigorous foundation for why the Lagrange multiplier conditions are necessary: they follow from the fact that the constraint surface is a smooth manifold near a regular point.
