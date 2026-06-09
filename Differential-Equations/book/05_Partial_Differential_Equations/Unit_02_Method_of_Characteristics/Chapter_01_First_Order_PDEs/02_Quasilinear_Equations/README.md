# Quasilinear First-Order PDEs

A quasilinear first-order PDE has the form

$$a(x,y,u)\,u_x + b(x,y,u)\,u_y = c(x,y,u), \tag{1}$$

where the coefficients $a$, $b$, $c$ may depend on the unknown function $u$ itself as well as on the independent variables $x$ and $y$. This is a significant generalization from the linear case: the characteristic directions depend on the solution, so different parts of the solution propagate at different speeds. This solution-dependence of propagation speed is the source of the most important nonlinear phenomenon in hyperbolic PDE theory — the formation of shocks.

## Geometric Interpretation

The equation (1) can be read as follows: the directional derivative of $u$ in the direction $(a(x,y,u), b(x,y,u))$ in the $(x,y)$-plane equals $c(x,y,u)$. Equivalently, in the three-dimensional $(x,y,u)$-space, the vector $(a(x,y,u), b(x,y,u), c(x,y,u))$ is tangent to the solution surface $\{(x,y,u(x,y))\}$ at each point.

A curve in $(x,y,u)$-space that is everywhere tangent to this vector field is called a **characteristic strip** or simply a characteristic of the PDE. The solution surface is formed by sweeping out characteristics from an initial curve. This geometric picture is the foundation of the method.

## Characteristic Equations

The characteristic equations for (1) are the ODE system

$$\frac{dx}{dt} = a(x,y,u), \qquad \frac{dy}{dt} = b(x,y,u), \qquad \frac{du}{dt} = c(x,y,u). \tag{2}$$

Unlike the linear case, these three equations are coupled: $(x,y,u)$ must be determined simultaneously because $a$, $b$, $c$ all depend on $u$. Given initial data on a curve $\Gamma$:

$$x(0) = x_0, \quad y(0) = y_0, \quad u(0) = u_0,$$

where $(x_0, y_0)$ is a point on $\Gamma$ and $u_0 = \phi(x_0, y_0)$ is the prescribed value, system (2) is an ODE initial value problem in the three unknowns $(x(t), y(t), u(t))$. By Picard's theorem, if $a$, $b$, $c$ are smooth, this system has a unique local solution.

## The Cauchy Problem

Suppose the initial data $u = \phi(x)$ is given on the curve $\Gamma\colon y = 0$, $x \in \mathbb{R}$. The Cauchy problem for (1) is:

$$a(x,y,u)\,u_x + b(x,y,u)\,u_y = c(x,y,u), \quad u(x,0) = \phi(x).$$

**Algorithm:**
1. For each $x_0 \in \mathbb{R}$, solve the characteristic system (2) with initial conditions $x(0)=x_0$, $y(0)=0$, $u(0)=\phi(x_0)$.
2. This gives a family of characteristic curves $(x(t;x_0), y(t;x_0), u(t;x_0))$ in $(x,y,u)$-space.
3. The map $(x_0, t) \mapsto (x,y)$ is locally invertible when the non-characteristic condition holds: $b(x_0, 0, \phi(x_0)) \neq 0$.
4. Express $u$ as a function of $(x,y)$ by inverting this map.

The non-characteristic condition ensures the characteristics are not tangent to the initial curve $y=0$.

## Example: Inviscid Burgers' Equation

Consider $u_t + u u_x = 0$ with $u(x,0) = \phi(x)$.

Writing $y = t$: $a = u$, $b = 1$, $c = 0$.

Characteristic equations: $dx/dt = u$, $dt/dt = 1$, $du/dt = 0$.

The third equation gives $u = \phi(x_0) = \text{const}$ along each characteristic. The first then gives $x(t) = x_0 + \phi(x_0)t$.

So the characteristic through $(x_0, 0)$ is the line $x = x_0 + \phi(x_0)t$, and $u = \phi(x_0)$ along it.

To find $u(x,t)$: solve $x = x_0 + \phi(x_0)t$ for $x_0$, then $u(x,t) = \phi(x_0)$.

**Key observation:** Characteristics are straight lines (since $\phi(x_0)$ is constant along each one), but their slope $dt/dx = 1/\phi(x_0)$ depends on $x_0$. Different characteristics have different slopes.

## Characteristic Crossing and Shock Formation

For Burgers' equation, the characteristics from two points $x_1 < x_2$ will cross if and only if $\phi(x_1) > \phi(x_2)$ — that is, if the initial profile is decreasing. The crossing time is

$$t^* = -\frac{1}{\min_{x}\phi'(x)},$$

which is finite if $\phi$ has a decreasing portion.

After characteristics cross, the "solution" $u$ would be multi-valued — clearly unphysical. The classical smooth solution breaks down at time $t^*$ (the shock formation time), and the theory must be extended to admit **weak solutions** with jump discontinuities (shocks). This is the subject of Chapter 2.

If $\phi$ is increasing, characteristics diverge and no crossing occurs. For example, $\phi(x) = \arctan(x)$ leads to characteristics that spread out, and the solution remains smooth for all time.

## Example: A Variable-Coefficient Quasilinear Equation

Consider $u u_x + u_y = 1$ with $u(x,0) = x$.

Characteristic equations: $dx/dt = u$, $dy/dt = u$... wait, let me rewrite with $y$ as the "time" variable. Here $a=u$, $b=1$, $c=1$.

Equations: $\dot{x} = u$, $\dot{y} = 1$, $\dot{u} = 1$ with $x(0)=x_0$, $y(0)=0$, $u(0)=x_0$.

Integrating: $u(t) = x_0 + t$, $y(t) = t$, $x(t) = x_0 t + t^2/2$.

Since $y = t$: $u = x_0 + y$, and $x = x_0 y + y^2/2$.

Solving for $x_0 = x/y - y/2$ (for $y \neq 0$): $u = (x/y - y/2) + y = x/y + y/2$.

So $u(x,y) = \frac{x}{y} + \frac{y}{2}$ for $y \neq 0$.

Check: $u_x = 1/y$, $u_y = -x/y^2 + 1/2$.

$u u_x + u_y = (x/y + y/2)(1/y) + (-x/y^2 + 1/2) = x/y^2 + 1/2 - x/y^2 + 1/2 = 1$. Correct.

## Non-Characteristic Condition for Quasilinear Equations

For the equation $a(x,y,u)u_x + b(x,y,u)u_y = c$, with initial data on a curve $\Gamma$ parametrized as $(x_0(s), y_0(s))$, the non-characteristic condition is

$$a(x_0,y_0,\phi)\,\dot{y}_0 - b(x_0,y_0,\phi)\,\dot{x}_0 \neq 0,$$

where $\phi = \phi(s)$ is the initial data and the dot is $d/ds$. When this fails, the initial curve is tangent to the characteristic direction as determined by the solution value on $\Gamma$, and the Cauchy problem may be ill-posed.

## Connection to Conservation Laws

Many quasilinear first-order PDEs arise as conservation laws: $u_t + (f(u))_x = 0$, which expands to $u_t + f'(u)u_x = 0$. This is a quasilinear equation with $a = f'(u)$, $b = 1$ ($y = t$), $c = 0$. The characteristic speed is $f'(u)$, which varies with $u$. The characteristics are straight lines $x = x_0 + f'(\phi(x_0))t$, along which $u$ is constant. The richness of conservation law theory — Riemann problems, shock waves, rarefaction waves, entropy conditions — flows from this simple observation, and is developed in detail in Chapter 2.
