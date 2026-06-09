# Order, Degree, and Linearity

Every ordinary differential equation has a structural fingerprint described by three attributes: its order, its degree, and whether it is linear. These are not superficial labels. They determine which theorems apply, which solution methods are available, and what kind of solution set to expect. Identifying these attributes is the first step whenever one encounters a new differential equation.

## Order

The **order** of an ODE is the order of the highest derivative of the unknown function that appears in the equation. For the equation $y' = f(x, y)$, the highest derivative is the first, so the equation is first-order. For $y'' + p(x)y' + q(x)y = g(x)$, the highest derivative is the second, making it a second-order equation. The equation

$$y''' - 3y'' + 3y' - y = e^x$$

is third-order. In general, the higher the order, the more initial conditions are required to single out a unique solution: an $n$-th order equation typically requires specifying $y(x_0)$, $y'(x_0)$, $\ldots$, $y^{(n-1)}(x_0)$.

Order is determined by inspection and is always a positive integer. It is the single most important attribute for determining the theoretical framework that applies.

## Degree

The **degree** of an ODE is the power to which the highest-order derivative is raised, provided the equation can be written as a polynomial in the derivatives. This concept applies most cleanly when the equation has the form

$$\left(y^{(n)}\right)^m + \text{lower order terms} = 0.$$

The equation $(y'')^3 + 2y' - y = 0$ has order 2 and degree 3. The equation $y'' + \sin(y') = 0$ does not have a well-defined polynomial degree because $\sin(y')$ is not a polynomial in $y'$.

For most theoretical and practical purposes, degree is less important than order or linearity. The cases of most interest are equations of degree 1, which are sometimes called **normal** or **resolved** equations. When an equation has degree greater than 1, it may factor into several first-degree equations, each of which can be treated separately. The Clairaut equation $y = xy' + f(y')$ is a classical example where degree-2 effects produce singular solutions that cannot be obtained by differentiating the general solution.

## Linearity

The distinction between linear and nonlinear equations is the most consequential classification in the subject. An $n$-th order ODE is **linear** if it can be written in the form

$$a_n(x)\,y^{(n)} + a_{n-1}(x)\,y^{(n-1)} + \cdots + a_1(x)\,y' + a_0(x)\,y = g(x),$$

where $a_0, a_1, \ldots, a_n$, and $g$ are functions of $x$ alone. The critical features are: the unknown function $y$ and each of its derivatives appear to the first power only, and no products of $y$ or its derivatives with each other appear. The coefficient functions $a_k(x)$ may be arbitrary functions of $x$; they are not required to be constant.

An equation that cannot be written in this form is **nonlinear**. The source of nonlinearity can be a power of $y$ (as in $y^2$ or $(y')^2$), a product of $y$ with a derivative (as in $yy'$), or a nonlinear function of $y$ or $y'$ (as in $\sin y$ or $e^{y'}$).

**Examples.** The equation $y'' - 2xy' + \lambda y = 0$ (Hermite's equation) is linear because each of $y''$, $y'$, $y$ appears to the first power and the coefficients $1$, $-2x$, $\lambda$ depend only on $x$. The equation $y'' + \sin y = 0$ (the pendulum equation) is nonlinear because $\sin y = y - y^3/6 + \cdots$ contains all powers of $y$. The equation $(y')^2 + y^2 = 1$ is nonlinear because of the $(y')^2$ term. The equation $yy'' = 1$ is nonlinear because of the product $y \cdot y''$.

## Why Linearity Matters: The Superposition Principle

For a linear homogeneous equation (one with $g(x) = 0$), if $y_1$ and $y_2$ are both solutions, then so is $c_1 y_1 + c_2 y_2$ for any constants $c_1, c_2$. This is the **superposition principle**, and it holds precisely because linearity makes the equation compatible with scalar multiplication and addition of functions.

Proof for the second-order case: if $Ly = a_2(x)y'' + a_1(x)y' + a_0(x)y$ denotes the linear operator, then

$$L(c_1 y_1 + c_2 y_2) = c_1 L y_1 + c_2 L y_2 = c_1 \cdot 0 + c_2 \cdot 0 = 0.$$

This works because $L$ is linear: it distributes over addition and commutes with scalar multiplication. A nonlinear operator does not have this property. If $N(y) = y^2$, for instance, then $N(y_1 + y_2) = y_1^2 + 2y_1 y_2 + y_2^2 \neq N(y_1) + N(y_2)$.

The superposition principle implies that the set of solutions of a linear homogeneous $n$-th order ODE forms a **vector space** of dimension $n$. This is a powerful structural result: the general solution is a linear combination of exactly $n$ linearly independent particular solutions, called a **fundamental set**. No such clean structure exists for nonlinear equations.

## Constant versus Variable Coefficients

Within linear equations, a further distinction applies: the coefficient functions $a_k(x)$ may be constants or genuinely variable. The equation $y'' + 5y' + 6y = 0$ has constant coefficients; the equation $x^2 y'' + xy' + (x^2 - n^2)y = 0$ (Bessel's equation) has variable coefficients. Constant-coefficient equations can be solved completely by the characteristic equation method, which reduces the problem to algebra. Variable-coefficient equations generally require series methods, special function theory, or numerical computation, and are substantially harder.

## A Classification Flowchart

Encountering an ODE, one first determines its order (the index of the highest derivative). Then one asks whether it is linear. If linear, one further asks whether the coefficients are constant. If nonlinear, one looks for special structure: is it separable? Exact? Bernoulli? Homogeneous in the sense of $y/x$? Each answer points toward specific solution techniques, which constitute the bulk of the subsequent chapters.

## Examples Classified

The equation $\frac{dy}{dx} = \frac{y}{x}$ is first-order, degree 1, linear with variable coefficients. It is also separable. The equation $\frac{d^2y}{dx^2} + 4y = \cos(2x)$ is second-order, degree 1, linear with constant coefficients and a nonhomogeneous term. The equation $\frac{dy}{dx} = y^2 - x$ is first-order, degree 1, nonlinear (because of $y^2$). The equation $(y'')^2 + y = 0$ is second-order, degree 2, nonlinear (because of $(y'')^2$ as well as the degree). The Airy equation $y'' - xy = 0$ is second-order, degree 1, linear with variable coefficient $a_0(x) = -x$.

Precise classification is a skill developed by working through many examples. The reward is a clear map of which theoretical results and computational tools apply to any given problem.
