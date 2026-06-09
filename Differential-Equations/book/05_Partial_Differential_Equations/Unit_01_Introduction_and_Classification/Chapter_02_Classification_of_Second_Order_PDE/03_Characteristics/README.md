# Characteristics of Second-Order PDEs

Characteristic curves are among the most important geometric objects in PDE theory. They are the curves along which the Cauchy problem for a second-order PDE is not uniquely determined — the directions in which the equation provides no new information about the highest-order derivatives. For hyperbolic equations, they are also the curves along which singularities propagate and along which information travels. Understanding characteristics is essential both for constructing explicit solutions (via the method of characteristics) and for understanding the qualitative behavior of solutions.

## The Cauchy Problem and Characteristics

Consider the general second-order linear PDE

$$Au_{xx} + Bu_{xy} + Cu_{yy} = F(x, y, u, u_x, u_y). \tag{1}$$

The Cauchy problem asks: given data on a curve $\Gamma$, can we determine the solution in a neighborhood of $\Gamma$? The data consists of $u$ and its normal derivative $\partial u/\partial n$ on $\Gamma$ — specifying both $u$ and one directional derivative is sufficient, by the chain rule, to determine all first derivatives of $u$ on $\Gamma$. To determine second derivatives, we use equation (1) together with the second derivatives of the data along $\Gamma$.

Let $\Gamma$ be parametrized by arc length $s$, with unit tangent $\mathbf{t} = (t_1, t_2)$ and unit normal $\mathbf{n} = (n_1, n_2)$. On $\Gamma$, the second derivatives $u_{xx}$, $u_{xy}$, $u_{yy}$ can be expressed in terms of: (a) the given data (the second tangential derivative of $u$ along $\Gamma$, which is known), and (b) the unknown normal second derivative $\partial^2 u / \partial n^2$. Using these relations, equation (1) becomes a linear equation for $\partial^2 u/\partial n^2$, with coefficient

$$A n_1^2 + B n_1 n_2 + C n_2^2.$$

If this coefficient is nonzero, we can solve for $\partial^2 u/\partial n^2$ and continue the solution — $\Gamma$ is a non-characteristic curve. If this coefficient vanishes, the second normal derivative is not determined: $\Gamma$ is a **characteristic** of the PDE.

## The Characteristic Equation

Setting the coefficient to zero: a curve $\Gamma$ with normal direction $(n_1, n_2)$ is characteristic if and only if

$$A n_1^2 + B n_1 n_2 + C n_2^2 = 0. \tag{2}$$

Since the normal direction to the curve $y = y(x)$ is proportional to $(dy/dx, -1) = (y', -1)$ (or rather the gradient of $\phi(x,y) = y - y(x)$, which is $(-y', 1)$ up to sign), and the tangent is $(1, y')$, the condition (2) with $(n_1, n_2) = (\phi_x, \phi_y)$ for a characteristic curve $\phi(x,y) = \text{const}$ becomes

$$A\phi_x^2 + B\phi_x\phi_y + C\phi_y^2 = 0.$$

Setting $\lambda = \phi_x/\phi_y = -dy/dx|_\text{char}$ (so $\lambda = dy/dx$ along the characteristic), this gives

$$A\left(\frac{dy}{dx}\right)^2 - B\left(\frac{dy}{dx}\right) + C = 0,$$

with solutions

$$\frac{dy}{dx} = \frac{B \pm \sqrt{B^2 - 4AC}}{2A}.$$

This is the **characteristic ODE**. Its real solutions give the characteristic curves.

## Characteristics for Each Type

**Hyperbolic ($B^2 - 4AC > 0$).** Two distinct real families of characteristics. Every point in the domain has exactly two characteristic directions. In the hyperbolic case, the solution propagates along characteristics: a disturbance initiated at a point spreads along both characteristic families. The general solution of $u_{\xi\eta} = 0$ (wave equation in characteristic coordinates) is $u = f(\xi) + g(\eta)$, showing that the solution is a superposition of functions constant on each characteristic family.

**Parabolic ($B^2 - 4AC = 0$).** One family of real characteristics (a repeated root). The single family of characteristics corresponds to the "time" direction — the direction in which the parabolic equation "flows." The heat equation $u_t = k u_{xx}$ has characteristics $t = \text{const}$ (from $A = 0$, $B = 0$, $C = k$, giving $k\phi_y^2 = 0$, so $\phi_y = 0$ and the level curves are $t = \text{const}$).

**Elliptic ($B^2 - 4AC < 0$).** No real characteristics. The characteristic ODE has only complex solutions. Elliptic equations have no preferred directions of propagation, no directions along which information travels faster than others, and no characteristic curves along which singularities could concentrate. This is why solutions of elliptic equations are smooth: there is no mechanism for singularities to form.

## Propagation of Singularities

One of the most important properties of characteristics is that they carry singularities. For a hyperbolic equation, if the initial data has a jump discontinuity in a derivative, that discontinuity propagates along characteristics and is not smoothed out. More precisely:

**Theorem (Propagation of Singularities).** Let $u$ be a solution of a hyperbolic equation with smooth coefficients, and suppose $u$ is smooth except possibly along a characteristic $\Gamma$. Then the singularity (the jump in some derivative) propagates along $\Gamma$ and cannot enter the interior of a smooth region from outside $\Gamma$.

For elliptic equations, the analogous statement is the regularity theorem: if $\Delta u = f$ and $f$ is smooth, then $u$ is smooth. Singularities cannot propagate along any curve — they are instantly spread out and killed by the elliptic smoothing.

## A Worked Example: the Wave Equation

For $u_{tt} - c^2 u_{xx} = 0$, we have $A = -c^2$, $B = 0$, $C = 1$, and the characteristic ODE is

$$-c^2\left(\frac{dy}{dx}\right)^2 + 1 = 0 \implies \frac{dy}{dx} = \pm \frac{1}{c},$$

where $y = t$ and $x = x$ are the standard variables. The two characteristic families are $t - x/c = \text{const}$ and $t + x/c = \text{const}$, or equivalently $x + ct = \text{const}$ and $x - ct = \text{const}$. These are the lines along which right- and left-traveling waves propagate. The characteristic coordinates $\xi = x + ct$, $\eta = x - ct$ reduce the equation to $u_{\xi\eta} = 0$, giving d'Alembert's solution $u(x,t) = f(x+ct) + g(x-ct)$.

If the initial data $u(x,0) = \phi(x)$, $u_t(x,0) = \psi(x)$ has a discontinuity in $\phi'$ at $x = x_0$, then $u$ will have jump discontinuities in $u_x$ along both characteristics $x + ct = x_0 + c\cdot 0 = x_0$ and $x - ct = x_0$, i.e., along $x = x_0 \pm ct$. The singularity splits into two, each traveling at speed $c$ in opposite directions.

## Cauchy-Kovalevskaya Theorem

A classical existence theorem for the Cauchy problem states conditions under which a solution exists and is unique:

**Theorem (Cauchy-Kovalevskaya).** If the coefficients and data are real-analytic, and if the Cauchy surface $\Gamma$ is non-characteristic at every point, then the Cauchy problem has a unique real-analytic solution in a neighborhood of $\Gamma$.

This theorem applies to all three types of equations (hyperbolic, parabolic, elliptic) when the surface is non-characteristic, but it only guarantees local existence in an analytic setting. For non-analytic data or characteristic surfaces, the theorem does not apply, and the problem may be ill-posed or may require fundamentally different solution techniques.

The Cauchy-Kovalevskaya theorem should be understood as a local existence result for analytic solutions. For global solutions and non-analytic data, the theory of each type requires separate development, which is the subject of the subsequent chapters.
