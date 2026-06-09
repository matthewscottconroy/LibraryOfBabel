# Linear First-Order PDEs

The simplest first-order PDE with two independent variables is

$$a(x,y)\,u_x + b(x,y)\,u_y = c(x,y),$$

where $a$, $b$, and $c$ are given smooth functions. This equation is a statement about the directional derivative of $u$ in the direction of the vector field $\mathbf{V} = (a,b)$: it says $\mathbf{V}\cdot\nabla u = c$. The key insight is that if we move along a curve that is always tangent to $\mathbf{V}$, then along that curve $u$ satisfies a simple ODE. These curves are the **characteristics**.

## Characteristics of the Linear Equation

The characteristic curves are the integral curves of the vector field $(a(x,y), b(x,y))$, defined by the ODE system

$$\frac{dx}{dt} = a(x(t),y(t)), \qquad \frac{dy}{dt} = b(x(t),y(t)).$$

If $(x(t), y(t))$ is such a curve and $u(x,y)$ is a solution of the PDE, then

$$\frac{d}{dt}u(x(t),y(t)) = u_x\dot{x} + u_y\dot{y} = u_x\cdot a + u_y\cdot b = c(x(t),y(t)).$$

So along each characteristic, $u$ satisfies the ODE

$$\frac{du}{dt} = c(x(t),y(t)).$$

This is a linear ODE in $t$, which can be integrated explicitly once the characteristic $(x(t),y(t))$ is known.

## The Algorithm

Given the Cauchy problem: find $u(x,y)$ satisfying $a u_x + b u_y = c$ with $u(x,0) = \phi(x)$ (initial data on the line $y=0$):

1. **Solve the characteristic ODE system** with initial condition $(x(0),y(0)) = (x_0, 0)$:
   $$\frac{dx}{dt} = a(x,y), \quad \frac{dy}{dt} = b(x,y), \quad x(0) = x_0,\quad y(0) = 0.$$
   The solution gives characteristic curves parametrized by $t$ and indexed by $x_0$.

2. **Integrate the equation for $u$ along each characteristic:**
   $$u(x(t),y(t)) = \phi(x_0) + \int_0^t c(x(s),y(s))\,ds.$$

3. **Express $(x,y)$ in terms of $(x_0, t)$** by inverting the characteristic map, and substitute to find $u$ as a function of $(x,y)$.

The inversion is possible as long as the Jacobian of the map $(x_0,t)\mapsto(x,y)$ is nonzero, which holds when the initial curve $y=0$ is non-characteristic (i.e., $b(x_0,0) \neq 0$ on the initial curve).

## Example: Pure Transport

Consider $u_t + c u_x = 0$ with $u(x,0) = \phi(x)$ (the transport equation with $y=t$, $a=c$, $b=1$, right-hand side zero).

Characteristic equations: $dx/dt = c$, $dt/dt = 1$, so $x(t) = x_0 + ct$, $t(t) = t$. Along each characteristic, $du/dt = 0$, so $u = \phi(x_0) = \text{const}$.

Inverting: $x_0 = x - ct$. Therefore $u(x,t) = \phi(x-ct)$.

The solution is the initial profile $\phi$ translated rigidly at speed $c$. Every feature of the initial data propagates unchanged along the characteristic lines $x - ct = \text{const}$.

## Example: Variable-Speed Transport

Consider $u_t + x u_x = 0$ with $u(x,0) = \phi(x)$.

Characteristic equations: $dx/dt = x$, $dt/dt = 1$. So $x(t) = x_0 e^t$, $t(t) = t$. The characteristic through $(x_0, 0)$ is the exponential $x = x_0 e^t$.

Along each characteristic, $du/dt = 0$, so $u = \phi(x_0)$.

Inverting: $x_0 = x e^{-t}$. Therefore $u(x,t) = \phi(xe^{-t})$.

Note that characteristics $x = x_0 e^t$ emanating from points to the right of the origin ($x_0 > 0$) diverge, while those to the left ($x_0 < 0$) also diverge. Characteristics never cross in this case, so the solution is globally smooth for all $t$.

## Example: Source Term

Consider $u_t + c u_x = u$ with $u(x,0) = \phi(x)$.

Characteristics: $x = x_0 + ct$, and along each one $du/dt = u$, so $u(x(t),t) = \phi(x_0)e^t$.

Inverting: $u(x,t) = \phi(x-ct)e^t$. The solution grows exponentially in time, reflecting the source term.

## Non-Characteristic Condition and Failure

The method requires that the initial curve $\Gamma$ be non-characteristic: the vector field $(a,b)$ must not be tangent to $\Gamma$ at any point. If the initial data is given on the curve $\Gamma\colon y = y(x)$, then the tangent to $\Gamma$ is $(1, y'(x))$ and the non-characteristic condition is

$$\det\begin{pmatrix} 1 & y'(x) \\ a & b \end{pmatrix} = b - a y'(x) \neq 0.$$

When this condition fails (initial curve is characteristic), the Cauchy problem is either overdetermined (data on a characteristic must be consistent with the ODE along it) or underdetermined (the solution is not unique off the characteristic).

## The Homogeneous Linear Equation and First Integrals

For the homogeneous equation $a u_x + b u_y = 0$, solutions are functions constant on characteristics. If $\xi(x,y) = \text{const}$ describes the characteristic curves (i.e., $\xi$ is a first integral of the vector field $(a,b)$: $a\xi_x + b\xi_y = 0$), then the general solution is $u = f(\xi(x,y))$ for any function $f$.

For the transport equation, $\xi = x - ct$ and $u = f(x-ct)$. For more complex equations, finding the first integral may require solving the characteristic ODE system first.

## Reduction to ODEs: The General Principle

The method of characteristics converts the problem of solving a first-order PDE into the problem of solving a system of ODEs (the characteristic equations). This is a genuine simplification: ODEs have a well-developed existence theory (Picard's theorem), can be solved explicitly in many cases, and are amenable to numerical integration. The geometric picture — solution surfaces in $(x,y,u)$-space swept out by curves called characteristics — provides intuition that extends to higher-order equations and systems.
