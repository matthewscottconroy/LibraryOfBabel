# Unit IX Problems: Foundations of General Relativity

*The Einstein field equations, stress-energy tensor, linearized GR, and the Newtonian limit.*

**Difficulty:** ★ Introductory, ★★ Intermediate, ★★★ Advanced

---

## Part 1: Einstein's Field Equations

**Problem 1.1** ★
The Einstein field equations:

$$G_{\mu\nu} = R_{\mu\nu} - \frac{1}{2}g_{\mu\nu}R = \frac{8\pi G}{c^4}T_{\mu\nu}$$

(a) Count the number of independent equations. ($G_{\mu\nu}$ is symmetric: $4\times5/2 = 10$ components. But $\nabla^\mu G_{\mu\nu} = 0$ provides 4 constraints, leaving 6 independent equations for the 10 components of $g_{\mu\nu}$. The remaining 4 "degrees of freedom" are fixed by gauge choice — diffeomorphism invariance.)

(b) The trace-reversed form: contract with $g^{\mu\nu}$ to obtain $R = -8\pi G T/c^4$ (where $T = g^{\mu\nu}T_{\mu\nu}$). Then substitute back to get $R_{\mu\nu} = \frac{8\pi G}{c^4}(T_{\mu\nu} - \frac{1}{2}g_{\mu\nu}T)$.

(c) Vacuum field equations: $T_{\mu\nu} = 0$. Show that in vacuum, $R_{\mu\nu} = 0$. Does this mean spacetime is flat ($R_{\mu\nu\rho\sigma} = 0$)? Give an example where $R_{\mu\nu} = 0$ but $R_{\mu\nu\rho\sigma}\neq 0$.

**Problem 1.2** ★★
The cosmological constant: Einstein modified his equations to:

$$G_{\mu\nu} + \Lambda g_{\mu\nu} = \frac{8\pi G}{c^4}T_{\mu\nu}$$

(a) Show that $\nabla^\mu(\Lambda g_{\mu\nu}) = 0$ (since $\nabla_\mu g_{\nu\rho} = 0$), so the contracted Bianchi identity still implies $\nabla^\mu T_{\mu\nu} = 0$.

(b) $\Lambda$ acts like a perfect fluid with $\rho_\Lambda = \Lambda c^2/(8\pi G)$ and $p_\Lambda = -\rho_\Lambda c^2$. Verify this by writing $\Lambda g_{\mu\nu}$ as a stress-energy tensor. What equation of state parameter $w = p/(\rho c^2)$ does $\Lambda$ correspond to?

(c) Observational value: $\Lambda \approx 1.1\times10^{-52}$ m⁻². The "dark energy density" $\rho_\Lambda$: compute in SI units. The critical density of the universe is $\rho_c = 3H_0^2/(8\pi G) \approx 9.5\times10^{-27}$ kg/m³ (with $H_0 = 67$ km/s/Mpc). What fraction $\Omega_\Lambda = \rho_\Lambda/\rho_c$?

**Problem 1.3** ★★
The Hilbert action: the Einstein field equations follow from the action principle $\delta S = 0$ where:

$$S = \frac{c^4}{16\pi G}\int (R - 2\Lambda)\sqrt{-g}\,d^4x + S_\text{matter}$$

(a) Vary $S$ with respect to $g^{\mu\nu}$ (treat $g^{\mu\nu}$ as the fundamental field). The variation of $\sqrt{-g}$ is $\delta\sqrt{-g} = -\frac{1}{2}\sqrt{-g}\,g_{\mu\nu}\delta g^{\mu\nu}$.

(b) The variation of the Ricci scalar $R = g^{\mu\nu}R_{\mu\nu}$: $\delta(g^{\mu\nu}R_{\mu\nu}) = R_{\mu\nu}\delta g^{\mu\nu} + g^{\mu\nu}\delta R_{\mu\nu}$. The last term is a total derivative (Palatini identity: $g^{\mu\nu}\delta R_{\mu\nu} = \nabla_\mu v^\mu$ for some vector $v^\mu$) and vanishes at the boundary. Hence $\delta S = 0$ gives...

(c) Define $T_{\mu\nu} = -\frac{2}{\sqrt{-g}}\frac{\delta S_\text{matter}}{\delta g^{\mu\nu}}$. Show that for a perfect fluid with $S_\text{matter} = -\int \rho c^2\sqrt{-g}\,d^4x$: this gives the perfect fluid stress-energy tensor.

---

## Part 2: Stress-Energy Tensor

**Problem 2.1** ★★
Perfect fluid: $T^{\mu\nu} = (\rho + p/c^2)u^\mu u^\nu + p g^{\mu\nu}$ where $\rho$ is the energy density, $p$ the pressure, and $u^\mu$ the 4-velocity.

(a) In the fluid rest frame ($u^\mu = (c,0,0,0)$ in an orthonormal frame): show this reduces to $T^{\mu\nu} = \text{diag}(\rho c^2, p, p, p)$.

(b) The conservation law $\nabla_\mu T^{\mu\nu} = 0$: project along $u_\nu$ (timelike direction) to obtain the energy equation. Project orthogonally to $u_\nu$ to obtain the Euler equation.

(c) In special relativity ($g_{\mu\nu} = \eta_{\mu\nu}$, flat space): show that the energy equation $u_\nu\nabla_\mu T^{\mu\nu} = 0$ gives $\partial_\mu(\rho u^\mu) = 0$ for pressureless dust ($p = 0$). This is relativistic mass conservation.

**Problem 2.2** ★★
Electromagnetic stress-energy: the Maxwell stress-energy tensor is $T^{\mu\nu}_\text{EM} = F^{\mu\lambda}F^\nu_{\ \lambda}/\mu_0 - \frac{1}{4}g^{\mu\nu}F_{\lambda\rho}F^{\lambda\rho}/\mu_0$.

(a) Show $T^\mu_{\ \mu,\text{EM}} = 0$ (trace-free). What does this imply about the equation of state of radiation?

(b) For a uniform magnetic field $\mathbf{B} = B\hat{z}$ in Minkowski space: compute all components of $T^{\mu\nu}_\text{EM}$.

(c) The electromagnetic stress-energy is the source of curvature in the Einstein-Maxwell equations: $G_{\mu\nu} = (8\pi G/c^4)T_{\mu\nu,\text{EM}}$. The Reissner-Nordström metric describes a charged black hole. State (without derivation) how the charge $Q$ modifies the Schwarzschild geometry.

---

## Part 3: Linearized GR and the Newtonian Limit

**Problem 3.1** ★★
Linearized GR: write $g_{\mu\nu} = \eta_{\mu\nu} + h_{\mu\nu}$ where $|h_{\mu\nu}|\ll 1$. Work to first order in $h$.

(a) The linearized Riemann tensor: $R_{\mu\nu\rho\sigma} = \frac{1}{2}(\partial_\rho\partial_\nu h_{\mu\sigma} + \partial_\sigma\partial_\mu h_{\nu\rho} - \partial_\sigma\partial_\nu h_{\mu\rho} - \partial_\rho\partial_\mu h_{\nu\sigma})$.

Verify this is correct by expanding $R_{\mu\nu\rho\sigma}$ to first order in $h$ using the Christoffel symbol formula.

(b) The trace-reversed perturbation: $\bar{h}_{\mu\nu} = h_{\mu\nu} - \frac{1}{2}\eta_{\mu\nu}h$ where $h = \eta^{\mu\nu}h_{\mu\nu}$. In the Lorenz gauge $\partial^\nu\bar{h}_{\mu\nu} = 0$: show the linearized Einstein equations reduce to:
$$\Box\bar{h}_{\mu\nu} = -\frac{16\pi G}{c^4}T_{\mu\nu}$$

(c) The static limit ($\partial_t = 0$) with a pressureless dust source ($T_{00} = \rho c^2$, all other components zero): show that $h_{00} = -2\Phi/c^2$ where $\Phi$ satisfies Poisson's equation $\nabla^2\Phi = 4\pi G\rho$.

**Problem 3.2** ★★
The Newtonian limit: in the weak-field, slow-motion limit, GR must reduce to Newtonian gravity.

(a) A slowly moving particle ($v\ll c$) follows a geodesic $d^2x^\mu/d\tau^2 + \Gamma^\mu_{\nu\rho}(dx^\nu/d\tau)(dx^\rho/d\tau) = 0$. Show that the dominant term in $\Gamma^\mu_{\nu\rho}$ is $\Gamma^i_{00} = -\frac{1}{2}\partial_i h_{00}$ and the spatial geodesic equation reduces to $\ddot{x}^i = -\frac{1}{2}c^2\partial_i h_{00}$.

(b) With $h_{00} = -2\Phi/c^2$: recover Newton's second law $\ddot{\mathbf{x}} = -\nabla\Phi$.

(c) PPN (Parameterized Post-Newtonian) formalism: the Schwarzschild metric in isotropic coordinates at $v \ll c$, $r \gg r_s$:

$$g_{00} \approx -1 + \frac{2GM}{c^2 r} - \frac{2G^2M^2}{c^4r^2} + O(c^{-6})$$
$$g_{ij} \approx \delta_{ij}\left(1 + \frac{2GM}{c^2r}\right)$$

The light deflection angle: $\delta\phi = 4GM/(c^2b)$ (where $b$ is the impact parameter). Why is the GR prediction twice the Newtonian prediction?

**Problem 3.3** ★★★
Post-Newtonian approximation: expand the metric as $g_{\mu\nu} = \eta_{\mu\nu} + h_{\mu\nu}^{(1)} + h_{\mu\nu}^{(2)} + \ldots$ where superscripts denote powers of $v/c$ (or $GM/(c^2r)$). The 1PN (first post-Newtonian) correction introduces:

$$g_{00} = -1 + \frac{2\Phi}{c^2} + \frac{2\Phi^2}{c^4} + O(c^{-6}), \qquad g_{0i} = O(c^{-3}), \qquad g_{ij} = \delta_{ij}\left(1 + \frac{2\Phi}{c^2}\right) + O(c^{-4})$$

(a) At this order, the equation of motion contains additional terms beyond $-\nabla\Phi$. Identify the "gravitomagnetic" force from the $g_{0i}$ term.

(b) The precession of Mercury is a 1PN effect. Verify that the $-2\Phi^2/c^4$ term in $g_{00}$ contributes to the effective potential used in Problem 3.2(a) of Unit V and reproduces the perihelion precession.

(c) In the parametrized post-Newtonian formalism, two parameters $\gamma$ and $\beta$ generalize GR: $g_{00} = -1 + 2\Phi/c^2 - 2\beta\Phi^2/c^4$, $g_{ij} = (1+2\gamma\Phi/c^2)\delta_{ij}$. GR predicts $\gamma = \beta = 1$. Current observational bounds from solar system tests: $|\gamma - 1| < 2.3\times10^{-5}$, $|\beta - 1| < 8\times10^{-5}$. What experiments give these bounds?
