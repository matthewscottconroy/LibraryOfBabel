# Analyticity and Harmonic Functions

One of the most striking consequences of the Cauchy-Riemann equations is that the real and imaginary parts of every analytic function are harmonic: they satisfy Laplace's equation $\Delta \phi = \phi_{xx} + \phi_{yy} = 0$. This connection is not incidental. It places complex analysis at the heart of potential theory — the mathematical study of gravitational, electrostatic, and fluid potential — and yields a powerful method for solving boundary value problems for Laplace's equation: find an analytic function whose real part (or imaginary part) satisfies the prescribed boundary data.

## Laplace's Equation

**Definition.** A real-valued function $\phi : D \to \mathbb{R}$ defined on an open domain $D \subseteq \mathbb{R}^2$ is harmonic if it has continuous second partial derivatives and satisfies Laplace's equation:
$$\Delta \phi = \frac{\partial^2 \phi}{\partial x^2} + \frac{\partial^2 \phi}{\partial y^2} = 0 \quad \text{throughout } D.$$

Harmonic functions are the natural objects of potential theory. In two-dimensional electrostatics, the electric potential $\phi$ in a charge-free region satisfies Laplace's equation. In steady-state heat conduction with no heat sources, the temperature distribution is harmonic. In fluid mechanics, the velocity potential of an irrotational flow is harmonic.

## The Fundamental Theorem: Analytic Implies Harmonic Parts

**Theorem.** If $f = u + iv$ is analytic on a domain $D$, then both $u$ and $v$ are harmonic on $D$.

**Proof.** Since $f$ is analytic, the Cauchy-Riemann equations $u_x = v_y$ and $u_y = -v_x$ hold on $D$. A fundamental result from complex integration theory (proved in Unit 03 via Cauchy's integral formula) states that analytic functions are infinitely differentiable, so all higher-order partial derivatives of $u$ and $v$ exist and are continuous.

Differentiating $u_x = v_y$ with respect to $x$ and $u_y = -v_x$ with respect to $y$:
$$u_{xx} = v_{yx} \quad \text{and} \quad u_{yy} = -v_{xy}.$$
By equality of mixed partials (valid since derivatives are continuous), $v_{yx} = v_{xy}$, so:
$$u_{xx} + u_{yy} = v_{yx} - v_{xy} = 0. \quad \checkmark$$
The argument for $v$ is identical, differentiating $v_y = u_x$ with respect to $y$ and $v_x = -u_y$ with respect to $x$. $\square$

**Worked example.** Verify that $u(x,y) = x^2 - y^2$ is harmonic.

$u_{xx} = 2$, $u_{yy} = -2$, $u_{xx} + u_{yy} = 0$. Yes, harmonic. This is consistent with $u = \mathrm{Re}(z^2)$.

**Worked example.** Verify that $u(x,y) = e^x\cos y$ is harmonic.

$u_x = e^x\cos y$, $u_{xx} = e^x\cos y$. $u_y = -e^x\sin y$, $u_{yy} = -e^x\cos y$. So $u_{xx} + u_{yy} = 0$. Harmonic. And indeed $u = \mathrm{Re}(e^z)$.

## The Converse Is False (Without Simply Connectedness)

Not every harmonic function is the real part of an analytic function on the same domain. The obstruction is topological:

**Example.** The function $u(x,y) = \ln\sqrt{x^2 + y^2} = \frac{1}{2}\ln(x^2 + y^2)$ is harmonic on $\mathbb{C} \setminus \{0\}$:
$$u_{xx} = \frac{y^2 - x^2}{(x^2+y^2)^2}, \quad u_{yy} = \frac{x^2 - y^2}{(x^2+y^2)^2}, \quad u_{xx} + u_{yy} = 0.$$
Its harmonic conjugate should be $v = \arg z = \arctan(y/x)$, and $u + iv = \log z$. But $\arg z$ cannot be made continuous and single-valued on $\mathbb{C} \setminus \{0\}$ (it is multivalued). Thus $u$ is harmonic on the non-simply connected domain $\mathbb{C} \setminus \{0\}$ but is not the real part of any analytic function on that domain.

**Theorem (Converse).** If $u$ is harmonic on a simply connected domain $D$, then $u = \mathrm{Re}(f)$ for some analytic function $f$ on $D$.

This theorem is proved by constructing the harmonic conjugate, which is possible on simply connected domains (see the next section).

## Properties of Harmonic Functions

Harmonic functions inherit remarkable properties from the analytic functions of which they are the real parts:

**Mean value property.** If $u$ is harmonic on a domain containing the closed disk $\overline{D}(z_0, r)$, then
$$u(z_0) = \frac{1}{2\pi} \int_0^{2\pi} u(z_0 + re^{i\theta})\, d\theta.$$
The value of $u$ at the center of any disk equals the average of its values on the circle. This is a consequence of Cauchy's integral formula applied to the analytic function $f$ with $u = \mathrm{Re}(f)$.

**Maximum principle.** If $u$ is harmonic on a bounded domain $D$ and continuous on $\overline{D}$, then $u$ attains its maximum and minimum values on the boundary $\partial D$, not in the interior (unless $u$ is constant).

**Proof sketch.** Suppose $u$ had an interior maximum at $z_0$. The mean value property would require that $u$ equals its maximum on every circle centered at $z_0$, forcing $u$ to be identically equal to its maximum in a neighborhood. By connectedness, $u$ is then constant on $D$, contradicting the assumption that the maximum is attained only at $z_0$. $\square$

The maximum principle is one of the foundational tools in PDE theory and is used to prove uniqueness of solutions to the Dirichlet problem.

**Dirichlet problem.** Given a domain $D$ with boundary $\partial D$ and a continuous function $g : \partial D \to \mathbb{R}$, find a harmonic function $u$ on $D$ that is continuous on $\overline{D}$ and equals $g$ on $\partial D$. The maximum principle guarantees uniqueness: if $u_1$ and $u_2$ both solve the problem, then $u_1 - u_2$ is harmonic with boundary values $0$, so by the maximum principle $u_1 - u_2 \equiv 0$.

## Applications to Physics

**Electrostatics.** In two dimensions, the electric potential $\phi$ in a charge-free region satisfies $\Delta\phi = 0$. If we find an analytic function $f = \phi + i\psi$, then the level curves $\{\phi = c\}$ are equipotential lines and the level curves $\{\psi = d\}$ are electric field lines. The two families are orthogonal (because $\phi$ and $\psi$ are harmonic conjugates, and the gradient fields of harmonic conjugates are perpendicular).

**Heat conduction.** In steady state, temperature $T(x,y)$ satisfies $\Delta T = 0$. Methods from complex analysis — conformal mapping in particular — can be used to solve $\Delta T = 0$ on complicated domains by mapping them conformally to the unit disk or upper half-plane, where the solution can be written explicitly (via the Poisson integral formula).

**Gravitational potential.** The Newtonian gravitational potential in a mass-free region is harmonic. The complex potential $f = \phi + i\psi$ encodes both the scalar potential $\phi$ and the stream function $\psi$ of the corresponding flow.

## Worked Example: Solving a Boundary Value Problem via Analytic Functions

Find a harmonic function $u$ on the upper half-plane $H = \{y > 0\}$ that satisfies $u(x, 0) = 1$ for $x > 0$ and $u(x, 0) = 0$ for $x < 0$.

Consider the analytic function $f(z) = \frac{1}{\pi}\mathrm{Arg}(z) = \frac{1}{\pi}\arctan\frac{y}{x}$ on $H$. Then $u = \mathrm{Im}(f/i) = \frac{1}{\pi}\mathrm{Arg}(z)$ is harmonic. As $y \to 0^+$:
- For $x > 0$: $\mathrm{Arg}(z) \to 0$, so $u \to 0$.
- For $x < 0$: $\mathrm{Arg}(z) \to \pi$, so $u \to 1$.

A slight adjustment gives $u = 1 - \frac{1}{\pi}\mathrm{Arg}(z)$, which satisfies the boundary conditions as stated. $\square$
