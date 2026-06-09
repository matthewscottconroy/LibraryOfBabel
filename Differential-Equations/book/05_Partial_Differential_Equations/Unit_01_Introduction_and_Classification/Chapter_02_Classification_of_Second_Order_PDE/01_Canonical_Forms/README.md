# Canonical Forms of Second-Order Linear PDEs

Every second-order linear PDE in two independent variables can be simplified by a change of variables into one of three standard forms. This reduction to canonical form is the key step in understanding the structure of the equation and choosing the right solution technique. The procedure mirrors the diagonalization of a quadratic form in linear algebra and is grounded in the same mathematics.

## Setup and Transformation Rules

Consider the equation

$$Au_{xx} + Bu_{xy} + Cu_{yy} + Du_x + Eu_y + Fu = G, \tag{1}$$

where $A, B, C, D, E, F, G$ are smooth functions of $(x,y)$, and $A^2 + B^2 + C^2 > 0$ (at least one second-order coefficient is nonzero). Introduce a smooth invertible change of variables

$$\xi = \xi(x,y), \qquad \eta = \eta(x,y),$$

with nonvanishing Jacobian $\xi_x \eta_y - \xi_y \eta_x \neq 0$. By the chain rule, the second partial derivatives of $u$ transform as follows. Setting $\bar{u}(\xi,\eta) = u(x(\xi,\eta), y(\xi,\eta))$, the coefficient of $\bar{u}_{\xi\xi}$ in the transformed equation is

$$\bar{A} = A\xi_x^2 + B\xi_x\xi_y + C\xi_y^2,$$

the coefficient of $\bar{u}_{\xi\eta}$ is

$$\bar{B} = 2A\xi_x\eta_x + B(\xi_x\eta_y + \xi_y\eta_x) + 2C\xi_y\eta_y,$$

and the coefficient of $\bar{u}_{\eta\eta}$ is

$$\bar{C} = A\eta_x^2 + B\eta_x\eta_y + C\eta_y^2.$$

The discriminant transforms as $\bar{B}^2 - 4\bar{A}\bar{C} = (B^2 - 4AC)\cdot J^2$ where $J = \xi_x\eta_y - \xi_y\eta_x$ is the Jacobian. Since $J \neq 0$, the sign of the discriminant is preserved by coordinate changes — it is an intrinsic property of the equation.

## The Characteristic Equations

The idea is to choose $\xi$ and $\eta$ so that $\bar{A}$ and $\bar{C}$ (or $\bar{B}$) vanish. The condition $\bar{A} = 0$ requires $\phi = \xi$ to satisfy

$$A\phi_x^2 + B\phi_x\phi_y + C\phi_y^2 = 0.$$

Dividing through by $\phi_y^2$ and setting $\lambda = \phi_x/\phi_y$ (assuming $\phi_y \neq 0$), this becomes

$$A\lambda^2 + B\lambda + C = 0,$$

with solutions

$$\lambda = \frac{-B \pm \sqrt{B^2 - 4AC}}{2A}.$$

These values of $\lambda = dy/dx$ along level curves of $\phi$ give the **characteristic directions**. The level curves $\phi(x,y) = \text{const}$ are the **characteristics** of the PDE.

## Hyperbolic Canonical Form

When $B^2 - 4AC > 0$, the characteristic equation has two distinct real roots $\lambda_+$ and $\lambda_-$. The characteristics come in two families, satisfying

$$\frac{dy}{dx} = \lambda_\pm(x,y).$$

Choose $\xi = \phi_+$ and $\eta = \phi_-$ (first integrals of the two characteristic ODEs). Then $\bar{A} = \bar{C} = 0$ and $\bar{B} \neq 0$, so the equation reduces to the **first hyperbolic canonical form**:

$$\bar{u}_{\xi\eta} = \bar{D}\bar{u}_\xi + \bar{E}\bar{u}_\eta + \bar{F}\bar{u} + \bar{G}.$$

A further change to characteristic coordinates $\alpha = \xi + \eta$, $\beta = \xi - \eta$ yields the **second hyperbolic canonical form**:

$$\bar{u}_{\alpha\alpha} - \bar{u}_{\beta\beta} = \text{lower order terms.}$$

The wave equation $u_{tt} = c^2 u_{xx}$, with $A = -c^2$, $B = 0$, $C = 1$, has discriminant $\Delta = 4c^2 > 0$ (hyperbolic). Its characteristics are the lines $x \pm ct = \text{const}$. Setting $\xi = x + ct$, $\eta = x - ct$ yields $u_{\xi\eta} = 0$, whose general solution is $u = f(\xi) + g(\eta) = f(x+ct) + g(x-ct)$ — d'Alembert's formula.

## Parabolic Canonical Form

When $B^2 - 4AC = 0$, the characteristic equation has a repeated real root $\lambda = -B/(2A)$. There is only one family of characteristics. Choose $\xi = \phi$ (from the single characteristic family) and $\eta$ any function independent of $\xi$ (the Jacobian is nonzero). Then $\bar{A} = 0$ and $\bar{B} = 0$ (the latter because $B^2 = 4AC$ forces $\bar{B} = 0$ whenever $\bar{A} = 0$ in the parabolic case). The equation reduces to:

$$\bar{C}\,\bar{u}_{\eta\eta} = \bar{D}\bar{u}_\xi + \bar{E}\bar{u}_\eta + \bar{F}\bar{u} + \bar{G}.$$

Renaming and dividing by $\bar{C}$:

$$\bar{u}_{\eta\eta} = \text{lower order terms},$$

which is the **parabolic canonical form**. The heat equation $u_t = ku_{xx}$ already has $A = 0$, $B = 0$, $C = k$, discriminant $0$ — it is already in canonical form with $\xi = t$, $\eta = x$.

## Elliptic Canonical Form

When $B^2 - 4AC < 0$, the characteristic equation has no real solutions — the characteristics are complex. One works over $\mathbb{C}$: the complex characteristics $\xi_\pm = \phi \pm i\psi$ are complex conjugates of each other. Taking the real combinations $\xi \leftarrow \phi$ and $\eta \leftarrow \psi$ (the real and imaginary parts of the complex characteristic), one finds $\bar{A} = \bar{C}$ and $\bar{B} = 0$. The equation reduces to:

$$\bar{u}_{\xi\xi} + \bar{u}_{\eta\eta} = \text{lower order terms},$$

which is the **elliptic canonical form** — Laplace's equation plus lower-order terms.

## Example: Reduction of a Specific Equation

Consider $u_{xx} + 4u_{xy} + 3u_{yy} = 0$.

Here $A = 1$, $B = 4$, $C = 3$, so $\Delta = 16 - 12 = 4 > 0$: hyperbolic.

The characteristic equation is $\lambda^2 - 4\lambda + 3 = 0$, giving $\lambda = 1$ or $\lambda = 3$.

The characteristics are $dy/dx = 1$ (giving $y - x = \text{const}$) and $dy/dx = 3$ (giving $y - 3x = \text{const}$).

Set $\xi = y - x$ and $\eta = y - 3x$. Then:

$$\xi_x = -1,\quad \xi_y = 1,\quad \eta_x = -3,\quad \eta_y = 1.$$

Computing: $\bar{A} = 1(1) + 4(-1)(1) + 3(1) = 0$, $\bar{C} = 1(9) + 4(-3)(1) + 3(1) = 0$.

The remaining coefficient: $\bar{B} = 2(1)(-1)(-3) + 4((-1)(1) + (1)(-3)) + 2(3)(1)(1) = 6 + 4(-4) + 6 = -4 \neq 0$.

So the equation in the new coordinates is $-4u_{\xi\eta} = 0$, i.e., $u_{\xi\eta} = 0$, and the general solution is $u = f(\xi) + g(\eta) = f(y-x) + g(y-3x)$.

## Summary

| Type | Discriminant | Canonical Form | Prototype |
|------|-------------|----------------|-----------|
| Hyperbolic | $B^2 - 4AC > 0$ | $u_{\xi\eta} = \ldots$ or $u_{\xi\xi} - u_{\eta\eta} = \ldots$ | Wave equation |
| Parabolic | $B^2 - 4AC = 0$ | $u_{\eta\eta} = \ldots$ | Heat equation |
| Elliptic | $B^2 - 4AC < 0$ | $u_{\xi\xi} + u_{\eta\eta} = \ldots$ | Laplace's equation |

The canonical form is the simplest member of each equivalence class under smooth coordinate changes, and every equation in that class inherits the qualitative properties of its canonical representative.
