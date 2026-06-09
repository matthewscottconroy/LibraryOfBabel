# Unit 02 Differentiation in Several Variables

Differentiation in one variable answers a single question: how fast is $f(x)$ changing as $x$ moves along the real line? In several variables, the question multiplies. A function $f: \mathbb{R}^n \to \mathbb{R}$ can be differentiated in any direction — along the $x$-axis, the $y$-axis, or any oblique direction — and the rates of change in different directions are generally different. The deeper question is not what these rates are individually, but whether there is a single linear object that captures all directional information simultaneously. The answer is yes, and that object is the **total derivative** — a linear map from $\mathbb{R}^n$ to $\mathbb{R}$, represented by the **Jacobian matrix**. This unit builds up to that central object step by step.

## What This Unit Covers

The unit comprises seven chapters that proceed from foundational topology through advanced theorems.

**Chapter 1 (Limits and Continuity)** establishes the topological vocabulary — open sets, closed sets, limit points — needed to give rigorous definitions of limit and continuity in $\mathbb{R}^n$. A central phenomenon, absent in one variable, is that a limit in $\mathbb{R}^n$ must be approached from infinitely many directions simultaneously, and a function can fail to have a limit by behaving differently along different paths.

**Chapter 2 (Partial Derivatives)** introduces the simplest differentiation operation: the partial derivative $\partial f/\partial x_i$, obtained by differentiating $f$ with respect to one variable while holding the others fixed. Higher-order partial derivatives and Clairaut's theorem — which asserts the equality of mixed partials under mild conditions — complete this chapter.

**Chapter 3 (Total Derivative)** introduces differentiability as the existence of a linear approximation. A function $f$ is differentiable at $\mathbf{p}$ if there exists a linear map $L$ such that $f(\mathbf{p}+\mathbf{h}) \approx f(\mathbf{p}) + L(\mathbf{h})$ with error smaller than $\|\mathbf{h}\|$. The matrix representing $L$ is the Jacobian. The multivariable chain rule expresses the derivative of a composition as a matrix product of Jacobians.

**Chapter 4 (Gradient and Directional Derivatives)** specializes the theory to scalar-valued functions. The gradient $\nabla f$ is the vector of all partial derivatives, and the directional derivative in direction $\hat{\mathbf{u}}$ is $\nabla f\cdot\hat{\mathbf{u}}$. The gradient is perpendicular to the level sets of $f$ and points in the direction of steepest increase.

**Chapter 5 (Taylor Series in Several Variables)** generalizes the single-variable Taylor series. The first-order approximation uses the gradient; the second-order approximation involves the Hessian matrix of second partial derivatives and quadratic forms. Higher-order terms require multi-index notation.

**Chapter 6 (Optimization)** applies the preceding theory to finding maxima and minima. A critical point is a point where the gradient vanishes; the Hessian determines whether it is a maximum, minimum, or saddle. Lagrange multipliers handle optimization on constraint sets.

**Chapter 7 (Implicit and Inverse Function Theorems)** contains the deepest results: conditions under which a system of equations implicitly defines smooth functions, and conditions under which a map has a smooth local inverse. These theorems underlie the modern theory of manifolds.

## How the Chapters Connect

Each chapter builds directly on the previous. One cannot meaningfully define differentiability without first having a notion of limit (Chapter 1). The total derivative (Chapter 3) subsumes the partial derivatives (Chapter 2) and makes precise why they do not suffice for differentiability on their own. The gradient (Chapter 4) is the special case of the Jacobian for scalar functions. Taylor series (Chapter 5) require higher-order partial derivatives established in Chapter 2. Optimization (Chapter 6) uses both the gradient and the Hessian from Chapters 4 and 5. The implicit function theorem (Chapter 7) requires the full Jacobian and the inverse function theorem to understand when coordinate changes are valid.

## How This Unit Fits into the Course

Differentiation in several variables is the conceptual core of multivariable calculus and the gateway to differential equations. Every partial differential equation involves partial derivatives; the wave equation $u_{tt} = c^2 u_{xx}$ involves second-order partials. The implicit function theorem justifies the method of characteristics for first-order PDEs. The optimization theory of Chapter 6 is the foundation for variational calculus, itself the source of the Euler-Lagrange equations in mechanics. Students who thoroughly master this unit will find partial differential equations a natural next step rather than a conceptual leap.
