# Chapter 05 Taylor Series in Several Variables

The Taylor series of a smooth function of one variable expresses $f(a+h)$ as a power series in $h$: $f(a+h) = f(a) + f'(a)h + \frac{1}{2}f''(a)h^2 + \cdots$. Each term provides a better local approximation than the previous. In several variables, the same idea applies: $f(\mathbf{a}+\mathbf{h})$ can be expanded as a sum of terms that are polynomial in $\mathbf{h}$, organized by degree. The first-order term is the linear approximation (the total derivative); the second-order term involves a new object, the **Hessian matrix**, which plays the role of the second derivative; and higher-order terms involve higher-degree polynomial expressions in the components of $\mathbf{h}$.

## What This Chapter Covers

**Section 1 (First-Order Approximation)** reviews the linear approximation $f(\mathbf{a}+\mathbf{h}) \approx f(\mathbf{a}) + \nabla f(\mathbf{a})\cdot\mathbf{h}$, and quantifies the error via Taylor's theorem with remainder. This is already established in Chapter 3; the section revisits it in the Taylor-series context.

**Section 2 (Hessian Matrix)** introduces the matrix of second-order partial derivatives. For $f:\mathbb{R}^n\to\mathbb{R}$ of class $C^2$, the Hessian is the symmetric $n\times n$ matrix $H_f(\mathbf{a})$ with entries $(H_f)_{ij} = \frac{\partial^2 f}{\partial x_i\partial x_j}(\mathbf{a})$. By Clairaut's theorem, $H_f = H_f^T$. The Hessian is the multivariable analogue of the second derivative.

**Section 3 (Second-Order Approximation)** derives the second-order Taylor expansion: $f(\mathbf{a}+\mathbf{h}) \approx f(\mathbf{a}) + \nabla f(\mathbf{a})\cdot\mathbf{h} + \frac{1}{2}\mathbf{h}^T H_f(\mathbf{a})\mathbf{h}$. The quadratic term $\mathbf{h}^T H_f\mathbf{h}$ is a quadratic form. The Taylor theorem with remainder in two variables is proved, including both Lagrange-form and Peano-form remainders.

**Section 4 (Higher-Order Terms)** introduces multi-index notation to state the full Taylor series compactly, and discusses convergence. For analytic functions, the series converges in an open ball around $\mathbf{a}$.

## How the Sections Build on Each Other

Sections 1 and 3 are the essential applied content. Section 2 (the Hessian) is the technical tool needed for Section 3. Section 4 generalizes both, requiring the multi-index notation from Chapter 2's higher-order partials. The progression mirrors the one-variable story: constant, linear, quadratic, and then higher-order approximations.

## How This Chapter Fits into the Unit

The second-order Taylor expansion is the key to optimization (Chapter 6). A critical point $\mathbf{a}$ (where $\nabla f = \mathbf{0}$) has $f(\mathbf{a}+\mathbf{h}) \approx f(\mathbf{a}) + \frac{1}{2}\mathbf{h}^T H_f(\mathbf{a})\mathbf{h}$. Whether this is a minimum, maximum, or saddle point depends on whether the quadratic form $\mathbf{h}^T H_f\mathbf{h}$ is always positive, always negative, or mixed — i.e., on whether $H_f$ is positive definite, negative definite, or indefinite. This is precisely the content of the second derivative test. The Hessian and Taylor series also appear in numerical methods: Newton's method for root finding uses the Hessian, and the stability of equilibria in differential equations is determined by the linearization (first-order Taylor approximation) of the system.
