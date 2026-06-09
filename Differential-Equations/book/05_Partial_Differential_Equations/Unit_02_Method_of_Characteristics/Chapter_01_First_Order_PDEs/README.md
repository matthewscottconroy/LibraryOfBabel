# Chapter 1: First-Order PDEs

First-order PDEs are the simplest class of partial differential equations in terms of derivative order, but they are far from trivial in behavior. They describe transport, advection, and the propagation of information along specific curves in the domain. The method of characteristics converts a first-order PDE into a family of ODEs — one system for each characteristic curve — and thereby reduces the problem of finding a partial differential equation's solution to the (generally more tractable) problem of solving ordinary differential equations.

## Progression of the Chapter

The chapter proceeds in three steps of increasing generality.

**Section 1: Linear First-Order PDEs** treats equations of the form $a(x,y)u_x + b(x,y)u_y = c(x,y)u + d(x,y)$, where the coefficients $a$, $b$, $c$, $d$ depend only on the independent variables and not on $u$. The characteristics are the integral curves of the vector field $(a,b)$, determined by solving $dx/dt = a$, $dy/dt = b$. Along each characteristic, $u$ satisfies a linear ODE with the coefficients $c$ and $d$ evaluated along the curve. The solution is explicit and well-defined wherever characteristics can be traced from initial data without crossing.

**Section 2: Quasilinear Equations** treats $a(x,y,u)u_x + b(x,y,u)u_y = c(x,y,u)$, where the coefficients may depend on the unknown $u$ itself. This is a significant generalization: the characteristic directions depend on the solution, so the characteristics must be determined simultaneously with the solution values along them. The characteristic equations become the coupled ODE system

$$\frac{dx}{dt} = a(x,y,u), \qquad \frac{dy}{dt} = b(x,y,u), \qquad \frac{du}{dt} = c(x,y,u).$$

Even though the characteristics cannot be determined in advance (since $u$ is unknown), this system can be solved as an initial value problem starting from data on the initial curve, because $u$ is specified there.

**Section 3: The Method of Characteristics in Full Generality** extends to fully nonlinear first-order PDEs $F(x,y,u,p,q) = 0$ where $p = u_x$, $q = u_y$. The characteristic equations (Charpit's equations) form a system in five unknowns:

$$\frac{dx}{dt} = F_p, \quad \frac{dy}{dt} = F_q, \quad \frac{dp}{dt} = -(F_x + pF_u), \quad \frac{dq}{dt} = -(F_y + qF_u), \quad \frac{du}{dt} = pF_p + qF_q.$$

This system propagates the initial data (values of $x$, $y$, $u$, $p$, $q$ on the initial curve) along characteristic strips in the $(x,y,u,p,q)$-space.

## Key Theorems Previewed

The central theorem of this chapter is the local existence and uniqueness of solutions via the method of characteristics, under a non-characteristic condition:

**Theorem.** Let the initial data $u = \phi$ be prescribed along a curve $\Gamma$ in the $(x,y)$-plane that is nowhere tangent to the characteristic direction $(a,b)$ (the non-characteristic condition). Then the quasilinear initial value problem has a unique smooth solution in a neighborhood of $\Gamma$.

When the non-characteristic condition fails, the initial curve is tangent to a characteristic, and the problem may have infinitely many solutions, no solutions, or require the data to satisfy a compatibility condition.

The chapter also introduces the concept of characteristic crossing and the formation of multi-valued solutions, which sets up Chapter 2 on shocks and conservation laws.
