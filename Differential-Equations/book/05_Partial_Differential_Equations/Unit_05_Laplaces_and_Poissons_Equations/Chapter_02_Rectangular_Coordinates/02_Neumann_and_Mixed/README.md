# Neumann and Mixed Boundary Conditions for Laplace's Equation

The Dirichlet problem (prescribing $u$ on the boundary) is not the only well-posed boundary value problem for Laplace's equation. The Neumann problem (prescribing $\partial u/\partial n$) and mixed problems (Dirichlet on part of the boundary, Neumann on the rest) arise naturally in heat conduction (insulated boundaries), electrostatics (prescribed surface charge), and fluid dynamics (impermeable walls). Each type leads to different eigenfunctions and different solvability conditions.

## The Neumann Problem

The Neumann problem on $\Omega = (0,a)\times(0,b)$:

$$\Delta u = 0 \text{ in } \Omega, \qquad \frac{\partial u}{\partial n} = h \text{ on } \partial\Omega.$$

**Compatibility condition.** Integrating $\Delta u = 0$ over $\Omega$ and using the divergence theorem:

$$0 = \int_\Omega\Delta u\,dA = \oint_{\partial\Omega}\frac{\partial u}{\partial n}\,ds = \oint_{\partial\Omega}h\,ds.$$

So the total prescribed flux must be zero: $\oint_{\partial\Omega}h\,ds = 0$. If this condition fails, the Neumann problem has no solution.

**Non-uniqueness.** If $u$ is a solution, so is $u + C$ for any constant $C$. Uniqueness is restored by prescribing the average: $\int_\Omega u\,dA = 0$.

## Neumann Problem with Homogeneous Conditions on Three Sides

Consider:

$$\Delta u = 0, \quad u_x(0,y) = 0, \quad u_x(a,y) = 0, \quad u_y(x,0) = 0, \quad u_y(x,b) = f(x).$$

(Neumann conditions on all sides, with nonzero flux on the top.)

Separation of variables with $u = X(x)Y(y)$: $X'' = -\mu X$, $X'(0)=X'(a)=0$ gives $\mu_n = (n\pi/a)^2$ and $X_n = \cos(n\pi x/a)$ for $n = 0, 1, 2, \ldots$ Note the inclusion of $n=0$ (constant mode $X_0 = 1$).

For each $n$: $Y_n'' = (n\pi/a)^2 Y_n$, $Y_n'(0) = 0$.

- $n = 0$: $Y_0 = A_0 + B_0 y$ (linear), with $Y_0'(0) = B_0 = 0$, giving $Y_0 = A_0$ (constant).
- $n \geq 1$: $Y_n = A_n\cosh(n\pi y/a)$ (using $\cosh$ to satisfy $Y_n'(0) = 0$).

The solution:

$$u(x,y) = A_0 + \sum_{n=1}^\infty A_n\cosh\!\left(\frac{n\pi y}{a}\right)\cos\!\left(\frac{n\pi x}{a}\right).$$

Applying $u_y(x,b) = f(x)$:

$$f(x) = \sum_{n=1}^\infty A_n\cdot\frac{n\pi}{a}\sinh\!\left(\frac{n\pi b}{a}\right)\cos\!\left(\frac{n\pi x}{a}\right).$$

The compatibility condition $\int_0^a f\,dx = 0$ ensures no constant term on the right. The coefficients:

$$A_n = \frac{2}{n\pi\sinh(n\pi b/a)}\int_0^a f(x)\cos\!\left(\frac{n\pi x}{a}\right)dx, \qquad n \geq 1.$$

The constant $A_0$ remains free (uniqueness only up to constants, as expected).

## Mixed Boundary Conditions

Consider a strip $\Omega = (0,\infty)\times(0,L)$ (semi-infinite domain) with:

$$u(0,y) = g(y), \quad u_y(x,0) = 0, \quad u(x,L) = 0, \quad u\to 0 \text{ as } x\to\infty.$$

Separation of variables with $u = X(x)Y(y)$: $Y'' = \mu Y$ with $Y'(0) = 0$ and $Y(L) = 0$ gives mixed Sturm-Liouville conditions. The eigenfunctions are $Y_n(y) = \cos\bigl((2n-1)\pi y/(2L)\bigr)$ with eigenvalues $\mu_n = ((2n-1)\pi/(2L))^2$ for $n = 1, 2, \ldots$

For $X$: $X'' = \mu_n X$ with $X\to 0$ as $x\to\infty$ gives $X_n(x) = e^{-\sqrt{\mu_n}x}$.

The solution:

$$u(x,y) = \sum_{n=1}^\infty c_n\,e^{-(2n-1)\pi x/(2L)}\cos\!\left(\frac{(2n-1)\pi y}{2L}\right),$$

with $c_n = \frac{2}{L}\int_0^L g(y)\cos((2n-1)\pi y/(2L))\,dy$.

## Why Mixed Problems Are Important

Mixed boundary conditions arise in nearly every real engineering problem. A heat exchanger has some surfaces insulated (Neumann) and others in contact with a coolant (Dirichlet or Robin). An electrostatic capacitor has conducting plates at fixed potential (Dirichlet) and insulating gaps (Neumann). Understanding the eigenfunction system for each configuration is essential for quantitative analysis.

The key difference between Dirichlet and Neumann eigensystems is the inclusion of the zero mode $\lambda_0 = 0$ (constant eigenfunction) in the Neumann case. This mode corresponds to the physical possibility of net heat or charge accumulation (or lack thereof), and its presence or absence determines the solvability of the problem.
