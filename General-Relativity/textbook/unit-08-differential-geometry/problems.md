# Unit VIII Problems: Differential Geometry

*Christoffel symbols, covariant derivatives, parallel transport, geodesics, and the Riemann curvature tensor.*

**Difficulty:** ★ Introductory, ★★ Intermediate, ★★★ Advanced

---

## Part 1: Christoffel Symbols and Covariant Derivatives

**Problem 1.1** ★★
Compute all nonzero Christoffel symbols for the following metrics:

(a) **Flat 2D space in polar coordinates:** $ds^2 = dr^2 + r^2 d\phi^2$.

Use $\Gamma^\rho_{\mu\nu} = \frac{1}{2}g^{\rho\sigma}(\partial_\mu g_{\nu\sigma} + \partial_\nu g_{\mu\sigma} - \partial_\sigma g_{\mu\nu})$.

(b) **Unit 2-sphere:** $ds^2 = d\theta^2 + \sin^2\!\theta\,d\phi^2$.

(c) **Schwarzschild metric** (radial and time components only — ignore angular parts): 
$g_{tt} = -(1-r_s/r)c^2$, $g_{rr} = (1-r_s/r)^{-1}$, all other components zero.

For (c), compute $\Gamma^r_{tt}$, $\Gamma^t_{tr}$, $\Gamma^r_{rr}$.

**Problem 1.2** ★★
Covariant derivatives:

(a) For the vector $V^\mu = (V^r, V^\phi) = (1, 0)$ (pointing radially) in 2D polar coordinates: compute $\nabla_r V^\phi$ and $\nabla_\phi V^r$. Interpret geometrically — does the vector change when parallel transported?

(b) Compute $\nabla_\mu V^\mu$ (the covariant divergence) for a vector $V^\mu$ in polar coordinates. Show it equals $\frac{1}{\sqrt{g}}\partial_\mu(\sqrt{g}V^\mu)$ where $g = \det g_{\mu\nu} = r^2$.

(c) For a symmetric tensor $S^{\mu\nu} = S^{\nu\mu}$: write out $\nabla_\rho S^{\mu\nu}$ in terms of partial derivatives and Christoffel symbols.

**Problem 1.3** ★★★
The Ricci identity: for any vector $V^\mu$:

$$[\nabla_\mu, \nabla_\nu]V^\rho = R^\rho_{\ \sigma\mu\nu}V^\sigma$$

(a) Derive this by computing $\nabla_\mu(\nabla_\nu V^\rho)$ and antisymmetrizing.

(b) For a 1-form $\omega_\rho$: show $[\nabla_\mu,\nabla_\nu]\omega_\rho = -R^\sigma_{\ \rho\mu\nu}\omega_\sigma$.

(c) For a $(2,0)$ tensor $T^{\rho\sigma}$: write $[\nabla_\mu,\nabla_\nu]T^{\rho\sigma}$ in terms of the Riemann tensor.

---

## Part 2: Parallel Transport and Geodesics

**Problem 2.1** ★★
Parallel transport on the sphere: a vector is parallel-transported around a latitude circle $\theta = \theta_0$ (constant) on the unit sphere.

(a) The parallel transport equations along a curve $(\theta(t), \phi(t)) = (\theta_0, \phi_0 + \omega t)$ are:
$$\frac{dV^\theta}{dt} + \Gamma^\theta_{\phi\phi}\omega V^\phi = 0, \qquad \frac{dV^\phi}{dt} + \Gamma^\phi_{\theta\phi}\omega V^\theta + \Gamma^\phi_{\phi\theta}\omega V^\theta = 0$$

Using the Christoffel symbols from Problem 1.1(b), write out these equations explicitly.

(b) For the equator ($\theta_0 = \pi/2$): solve the parallel transport equations. Starting with $V = \partial_\theta$ (pointing "south"): what is $V$ after one circuit?

(c) For a general latitude $\theta_0$: show the vector rotates by angle $2\pi\cos\theta_0$ after one circuit. This is the holonomy of the latitude circle.

(d) The Foucault pendulum: the pendulum's swing direction is parallel-transported on the sphere as Earth rotates. At latitude $\lambda$ (co-latitude $\theta_0 = \pi/2-\lambda$), the rotation per day is $2\pi\cos\theta_0 = 2\pi\sin\lambda$. For London ($\lambda = 51.5°$): how many days does it take for the pendulum to complete one full rotation?

**Problem 2.2** ★★
Geodesics:

(a) The geodesic equation $\ddot{x}^\rho + \Gamma^\rho_{\mu\nu}\dot{x}^\mu\dot{x}^\nu = 0$. For flat 2D space in polar coordinates (from Problem 1.1(a)):
write out the two geodesic equations.

(b) For a straight line through the origin $r(t) = t$, $\phi(t) = \phi_0$ (constant): verify it satisfies the geodesic equations.

(c) For a circle $r = r_0$, $\phi = \omega t$: show this is NOT a geodesic (radial equation is not satisfied unless $r_0 = 0$).

(d) On the 2-sphere: write the geodesic equations from the Christoffel symbols in Problem 1.1(b). Verify that great circles ($\theta = \pi/2$, $\phi = t$) are geodesics.

**Problem 2.3** ★★★
Geodesic deviation: nearby geodesics $x^\mu(\tau, s)$ (parameterized by proper time $\tau$ and label $s$) define a separation vector $\xi^\mu = \partial x^\mu/\partial s$.

The geodesic deviation equation:

$$\frac{D^2\xi^\mu}{d\tau^2} = R^\mu_{\ \nu\rho\sigma}u^\nu u^\rho\xi^\sigma$$

where $u^\mu = \partial x^\mu/\partial\tau$ is the tangent and $D/d\tau = u^\nu\nabla_\nu$.

(a) Interpret this equation physically: what does it say about tidal forces in a gravitational field?

(b) In a region of constant curvature (e.g., de Sitter space with $R_{\mu\nu\rho\sigma} = K(g_{\mu\rho}g_{\nu\sigma} - g_{\mu\sigma}g_{\nu\rho})$): solve the geodesic deviation equation for $\xi^\mu(\tau)$ given initial conditions $\xi^\mu(0)$ and $\dot\xi^\mu(0) = 0$.

(c) In Newtonian gravity, tidal forces between two particles separated by $\xi^i$ are $\ddot{\xi}^i = -\partial^2\Phi/\partial x^i\partial x^j\,\xi^j$. Identify the Newtonian analogue of the Riemann tensor: $R^i_{\ j0k} \leftrightarrow -\partial_{jk}\Phi/c^2$ in the Newtonian limit.

---

## Part 3: The Riemann Curvature Tensor

**Problem 3.1** ★★
Symmetries of the Riemann tensor $R_{\mu\nu\rho\sigma} = g_{\mu\alpha}R^\alpha_{\ \nu\rho\sigma}$:

(a) Antisymmetry in first pair: $R_{\mu\nu\rho\sigma} = -R_{\nu\mu\rho\sigma}$.
(b) Antisymmetry in second pair: $R_{\mu\nu\rho\sigma} = -R_{\mu\nu\sigma\rho}$.
(c) Symmetry under pair swap: $R_{\mu\nu\rho\sigma} = R_{\rho\sigma\mu\nu}$.
(d) First (algebraic) Bianchi identity: $R_{\mu[\nu\rho\sigma]} = 0$, equivalently $R_{\mu\nu\rho\sigma} + R_{\mu\rho\sigma\nu} + R_{\mu\sigma\nu\rho} = 0$.

Count the number of independent components of $R_{\mu\nu\rho\sigma}$ in $n = 4$ dimensions using these symmetries. (Answer: 20.)

**Problem 3.2** ★★★
Compute the Riemann tensor for the 2-sphere:

(a) Starting from the Christoffel symbols $\Gamma^\theta_{\phi\phi} = -\sin\theta\cos\theta$, $\Gamma^\phi_{\theta\phi} = \cot\theta$ (and zero otherwise): compute all components of $R^\rho_{\ \sigma\mu\nu}$.

(b) The Ricci tensor: $R_{\mu\nu} = R^\rho_{\ \mu\rho\nu}$. Compute $R_{\theta\theta}$ and $R_{\phi\phi}$.

(c) The Ricci scalar: $R = g^{\mu\nu}R_{\mu\nu}$. For the unit 2-sphere: show $R = 2$ (the curvature of a unit sphere).

(d) The Gauss-Bonnet theorem: $\frac{1}{4\pi}\int_M R\,dA = \chi(M)$ where $\chi$ is the Euler characteristic ($\chi = 2$ for $S^2$). Verify this numerically for the unit sphere.

**Problem 3.3** ★★★
The Bianchi identity and Einstein tensor:

(a) The second (differential) Bianchi identity: $\nabla_{[\lambda}R_{\mu\nu]\rho\sigma} = 0$. Written out: $\nabla_\lambda R_{\mu\nu\rho\sigma} + \nabla_\mu R_{\nu\lambda\rho\sigma} + \nabla_\nu R_{\lambda\mu\rho\sigma} = 0$.

Contract once (multiply by $g^{\mu\rho}$ and sum): obtain $\nabla^\mu R_{\nu\mu\sigma} = \ldots$. Perform the contraction explicitly.

(b) Contract again (multiply the result by $g^{\nu\sigma}$ and sum): derive $\nabla^\mu R_{\mu\nu} = \frac{1}{2}\nabla_\nu R$.

(c) Define the Einstein tensor $G_{\mu\nu} = R_{\mu\nu} - \frac{1}{2}g_{\mu\nu}R$. Show $\nabla^\mu G_{\mu\nu} = 0$ (contracted Bianchi identity). This is not an extra condition — it follows automatically from the Bianchi identity.

(d) Why is $\nabla^\mu G_{\mu\nu} = 0$ physically crucial? (Hint: since Einstein's equations say $G_{\mu\nu} = 8\pi G T_{\mu\nu}/c^4$, this implies $\nabla^\mu T_{\mu\nu} = 0$ — energy-momentum conservation.)
