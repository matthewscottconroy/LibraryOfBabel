# Unit II Problems: Calculus

*Single-variable and multi-variable calculus, including chain rule, Jacobians, implicit differentiation, series, and integration techniques central to GR.*

**Difficulty:** ★ Introductory, ★★ Intermediate, ★★★ Advanced

---

## Part 1: Single-Variable Calculus

**Problem 1.1** ★
Compute the following derivatives from first principles (limit definition), then verify using rules:

(a) $\frac{d}{dx}(x^n) = nx^{n-1}$ for positive integer $n$ (use the binomial theorem)
(b) $\frac{d}{dx}(e^x) = e^x$ (use the definition $e = \lim(1+1/n)^n$)
(c) $\frac{d}{dx}(\sin x) = \cos x$ (use the angle addition formula and $\lim_{h\to0}(\sin h)/h = 1$)

**Problem 1.2** ★
Chain rule and implicit differentiation:

(a) Find $\frac{d}{dx}\ln(\sin(x^2 + 1))$ using the chain rule.
(b) The curve $x^2 + y^2 = r^2$ (circle of radius $r$). Find $dy/dx$ by implicit differentiation.
(c) A parametric curve $x = \cos t$, $y = \sin t$. Verify $dy/dx = -\cos t/\sin t$ using $\frac{dy/dt}{dx/dt}$.

**Problem 1.3** ★★
Taylor series: derive the Taylor series for the following about $x = 0$ to at least 4th order, including the general term where possible:

(a) $e^x = \sum_{n=0}^\infty x^n/n!$
(b) $\cos x$ and $\sin x$
(c) $(1+x)^{-1}$ (geometric series — what is its radius of convergence?)
(d) $\ln(1+x)$ — derive by integrating the series for $(1+x)^{-1}$

**Problem 1.4** ★★
Integration techniques relevant to GR:

(a) $\int x^n e^{-ax} dx$ using integration by parts repeatedly. Express in terms of the incomplete gamma function.
(b) $\int_0^\infty e^{-ax^2}dx = \frac{1}{2}\sqrt{\pi/a}$ — prove this using the trick $I^2 = \int\int e^{-a(x^2+y^2)}dx\,dy$ converted to polar coordinates.
(c) $\int \frac{dr}{r^2 - 2Mr}$ — this integral appears in Schwarzschild geodesic equations. Evaluate it by partial fractions.

---

## Part 2: Multivariable Calculus

**Problem 2.1** ★
The gradient, divergence, and curl in Cartesian coordinates:

(a) For $f(x,y,z) = x^2yz$: compute $\nabla f$.
(b) For $\mathbf{F} = (xy, yz, zx)$: compute $\nabla\cdot\mathbf{F}$ and $\nabla\times\mathbf{F}$.
(c) Show that $\nabla\cdot(\nabla\times\mathbf{F}) = 0$ for any smooth $\mathbf{F}$ (in Cartesian coordinates).
(d) Show that $\nabla\times(\nabla f) = \mathbf{0}$ for any smooth $f$.

**Problem 2.2** ★★
The Jacobian matrix $J$ of a map $\mathbf{F}: \mathbb{R}^n\to\mathbb{R}^m$ has components $J_{ij} = \partial F^i/\partial x^j$.

(a) Find the Jacobian of the polar coordinate map: $x = r\cos\theta$, $y = r\sin\theta$. Compute $|\det J|$ and explain why it equals $r$ (the area element $dA = r\,dr\,d\theta$).

(b) Find the Jacobian of the spherical coordinate map: $x = r\sin\theta\cos\phi$, $y = r\sin\theta\sin\phi$, $z = r\cos\theta$. Compute $|\det J|$ and verify $dV = r^2\sin\theta\,dr\,d\theta\,d\phi$.

(c) Chain rule in matrix form: if $\mathbf{h} = \mathbf{g}\circ\mathbf{f}$, then $J_\mathbf{h} = J_\mathbf{g}\cdot J_\mathbf{f}$. For the composition $\mathbb{R}^2 \xrightarrow{f} \mathbb{R}^3 \xrightarrow{g} \mathbb{R}^2$: write the explicit chain rule in component form.

**Problem 2.3** ★★
The multivariable chain rule is the foundation of tensor transformation laws. Let $\phi: M\to M$ be a coordinate change on a 2D space. The components of a vector $V$ transform as $\tilde{V}^i = (\partial\tilde{x}^i/\partial x^j) V^j$.

(a) Under the coordinate change $\tilde{x} = x^2$, $\tilde{y} = y$ (on a region $x > 0$): find the Jacobian $\partial\tilde{x}^i/\partial x^j$.
(b) Transform the vector $V = \partial/\partial x$ (i.e., $V^x = 1$, $V^y = 0$) to the new coordinates.
(c) Why do vectors transform with the Jacobian and covectors with the inverse Jacobian? (A covector $\omega$ is a linear function on vectors: $\tilde{\omega}_i = (\partial x^j/\partial\tilde{x}^i)\omega_j$.)

**Problem 2.4** ★★
Stokes' theorem: $\int_{\partial\Sigma}\boldsymbol{\omega} = \int_\Sigma d\boldsymbol{\omega}$ (differential forms version; in 3D Cartesian: $\oint_C \mathbf{F}\cdot d\boldsymbol{\ell} = \iint_S (\nabla\times\mathbf{F})\cdot d\mathbf{A}$).

(a) Verify Stokes' theorem for $\mathbf{F} = (-y, x, 0)$ and the unit disk $S$ (boundary: unit circle $C$).
(b) Verify the Divergence theorem for $\mathbf{F} = (x,y,z)$ and the unit ball (boundary: unit sphere).
(c) In GR, the contracted Bianchi identity $\nabla^\mu G_{\mu\nu} = 0$ implies local energy-momentum conservation $\nabla^\mu T_{\mu\nu} = 0$. This is analogous to $\nabla\cdot(\nabla\times\mathbf{B}) = 0$. What topological fact underlies all these vanishing-divergence results?

**Problem 2.5** ★★★
Extremization and Euler-Lagrange: the length of a curve $\gamma$ in $\mathbb{R}^2$ from $A$ to $B$ is $L[\gamma] = \int_A^B \sqrt{1 + (dy/dx)^2}\,dx$.

(a) Apply the Euler-Lagrange equation $\frac{\partial f}{\partial y} - \frac{d}{dx}\frac{\partial f}{\partial y'} = 0$ (with $f = \sqrt{1+y'^2}$) to show that the shortest path is a straight line.

(b) In a Riemannian space with metric $g_{ij}$, the length is $L = \int \sqrt{g_{ij}\dot{x}^i\dot{x}^j}\,d\lambda$. Derive the geodesic equation from the Euler-Lagrange equations for the energy functional $E = \int g_{ij}\dot{x}^i\dot{x}^j\,d\lambda$ (which gives the same curves but with affine parameterization).

(c) Show that the result is $\ddot{x}^k + \Gamma^k_{ij}\dot{x}^i\dot{x}^j = 0$ where $\Gamma^k_{ij} = \frac{1}{2}g^{k\ell}(\partial_i g_{j\ell} + \partial_j g_{i\ell} - \partial_\ell g_{ij})$.

**Problem 2.6** ★★★
The Frobenius theorem (integrability): a distribution $D$ (smoothly varying subspace of the tangent bundle) is integrable (i.e., there exist hypersurfaces everywhere tangent to $D$) if and only if $[X,Y]\in D$ for all $X,Y\in D$.

(a) In 3D, the distribution $D = \ker\omega$ for a 1-form $\omega = dx - y\,dz$: is it integrable? Check the Frobenius condition $\omega\wedge d\omega = 0$.
(b) The condition $\omega\wedge d\omega = 0$ is the integrability condition. Compute $d\omega$ for the above, then $\omega\wedge d\omega$, and interpret.
(c) In GR, the hypersurface orthogonality condition for a congruence of curves with tangent $u^\mu$ requires $u_{[\mu}\nabla_\nu u_{\rho]} = 0$. This is the covariant analogue of Frobenius. State the physical meaning for a static spacetime (where $u^\mu$ is the Killing vector of time translation symmetry).
