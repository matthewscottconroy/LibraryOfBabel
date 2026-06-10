# Chapter 24: Exercises

---

## Section 24.1 — The Riemann Curvature Tensor

**24.1.1.** *Computing the Riemann tensor on $S^2$.*

For the 2-sphere with metric $ds^2 = R^2(d\theta^2 + \sin^2\theta\,d\phi^2)$, the Christoffel symbols are $\Gamma^\theta_{\phi\phi} = -\sin\theta\cos\theta$, $\Gamma^\phi_{\theta\phi} = \Gamma^\phi_{\phi\theta} = \cot\theta$.

(a) Compute the non-zero component $R^\theta_{\ \phi\theta\phi}$ from the formula:
$$R^\rho_{\ \sigma\mu\nu} = \partial_\mu\Gamma^\rho_{\nu\sigma} - \partial_\nu\Gamma^\rho_{\mu\sigma} + \Gamma^\rho_{\mu\lambda}\Gamma^\lambda_{\nu\sigma} - \Gamma^\rho_{\nu\lambda}\Gamma^\lambda_{\mu\sigma}$$

(b) Using the symmetries of the Riemann tensor, find all independent non-zero components. Show that they are all determined by $R_{\theta\phi\theta\phi}$ alone (which must be the case for a 2-dimensional manifold — there is only one independent component of the Riemann tensor in $n=2$ dimensions).

(c) Compute the Gaussian curvature $K = R_{\theta\phi\theta\phi}/g = R/2$, where $g = R^4\sin^2\theta$ is the determinant of the metric. Verify that $K = 1/R^2$ — the sphere of radius $R$ has constant positive curvature $1/R^2$.

(d) For a small geodesic triangle on $S^2$ with sides of angular size $\sim\epsilon \ll 1$, estimate the angle-sum deficit $(\alpha + \beta + \gamma) - \pi$ in terms of the area of the triangle and $K$. This is the Gauss-Bonnet theorem locally.

---

**24.1.2.** *Symmetries of the Riemann tensor and independent components.*

In $n$ dimensions, the Riemann tensor has $n^4$ components naively. The symmetries reduce this drastically.

(a) Antisymmetry in the first pair $R_{\rho\sigma\mu\nu} = -R_{\sigma\rho\mu\nu}$ reduces the first pair to $\binom{n}{2} = n(n-1)/2$ independent choices. Antisymmetry in the second pair similarly. So naively we have $[n(n-1)/2]^2$ components. For $n=4$, this is 36.

(b) The pair symmetry $R_{\rho\sigma\mu\nu} = R_{\mu\nu\rho\sigma}$ means the tensor is symmetric as a matrix on the space of antisymmetric pairs. This reduces 36 to $\binom{7}{2} = 21$ components.

(c) The algebraic Bianchi identity $R_{\rho[\sigma\mu\nu]} = 0$ gives additional constraints. Show there is exactly one independent identity per point (the fully antisymmetric part vanishes). For $n=4$, this reduces 21 to 20 independent components.

(d) In $n=2$: how many independent components? In $n=3$: how many? For $n=3$, the 6 independent Riemann components are equivalent (via contraction) to the 6 independent components of the Ricci tensor — so in 3D, the Riemann tensor is completely determined by the Ricci tensor, and vacuum solutions ($R_{\mu\nu} = 0$) are flat.

---

**24.1.3.** *Geodesic deviation and LIGO.*

Two free-falling test masses are separated by the displacement vector $\xi^\mu$. The geodesic deviation equation is $D^2\xi^\mu/d\tau^2 = -R^\mu_{\ \nu\rho\sigma}u^\nu\xi^\rho u^\sigma$.

(a) A gravitational wave traveling in the $z$-direction has the metric perturbation $h_{\mu\nu}$ (in transverse-traceless gauge). The non-zero components are $h_{xx} = -h_{yy} = h_+(t-z/c)$ for the $+$ polarization. Two test masses at rest (so $u^\mu \approx (c,0,0,0)$) are separated in the $x$-direction. Show that the relevant component of the geodesic deviation equation gives:
$$\ddot{\xi}^x = -R^x_{\ 0x0}\xi^x c^2 \approx \frac{1}{2}\ddot{h}_+\xi^x$$

(b) LIGO's arms are $L = 4$ km. At the first detection (GW150914), the peak strain was $h_+ \approx 10^{-21}$. The wave had frequency $f \approx 150$ Hz. Estimate the peak displacement $\delta L$ of the mirrors. Express in units of $10^{-18}$ m (attometers). How does this compare to the classical proton radius $r_p \approx 0.85\times 10^{-15}$ m?

(c) The signal lasted about $0.2$ seconds and chirped from $35$ Hz to $150$ Hz. Estimate the total number of oscillation cycles detected. If the signal-to-noise ratio scales as $\sim\sqrt{N_{\rm cycles}}$, why did the chirp waveform allow detection of such an impossibly small displacement?

(d) Before LIGO, some physicists doubted gravitational waves carry energy. Explain why the geodesic deviation equation demonstrates that they do — a wave with $\ddot{h}_+ \neq 0$ accelerates free masses, and accelerating masses can do work.

---

**24.1.4.** *The Weyl tensor and conformal flatness.*

The Weyl tensor $C_{\rho\sigma\mu\nu}$ is the trace-free part of the Riemann tensor:
$$R_{\rho\sigma\mu\nu} = C_{\rho\sigma\mu\nu} + \frac{2}{n-2}\left(g_{\rho[\mu}R_{\nu]\sigma} - g_{\sigma[\mu}R_{\nu]\rho}\right) - \frac{2}{(n-1)(n-2)}Rg_{\rho[\mu}g_{\nu]\sigma}$$

(a) Show that $g^{\rho\mu}C_{\rho\sigma\mu\nu} = 0$ (Weyl is trace-free in all pairs of indices).

(b) In $n=4$: Riemann has 20 independent components, Ricci has 10, Weyl has 10. Verify this decomposition accounts for all 20 Riemann components.

(c) The FLRW metric (homogeneous, isotropic cosmology) is conformally flat: $ds^2 = a(\eta)^2\eta_{\mu\nu}dx^\mu dx^\nu$ where $a(\eta)$ is the scale factor and $\eta$ is conformal time. Show that the Weyl tensor of any conformally flat metric vanishes. (Hint: under $g_{\mu\nu} \to \Omega^2 g_{\mu\nu}$, the Weyl tensor transforms as $C^\rho_{\ \sigma\mu\nu} \to C^\rho_{\ \sigma\mu\nu}$, so $C = 0$ for Minkowski implies $C = 0$ for all conformally flat metrics.)

(d) The Schwarzschild metric is NOT conformally flat (it has non-zero Weyl tensor in vacuum). Compute the single independent component $C_{trtr}$ of the Schwarzschild Weyl tensor and show that $C_{\mu\nu\rho\sigma}C^{\mu\nu\rho\sigma} = 48G^2M^2/r^6$ (the Kretschner scalar — nonsingular interpretation: tidal forces diverge at the Schwarzschild singularity).

---

## Section 24.2 — From Ricci to Einstein

**24.2.1.** *Deriving the contracted Bianchi identity.*

The differential Bianchi identity is $\nabla_{[\lambda}R_{\rho\sigma]\mu\nu} = 0$, or equivalently $\nabla_\lambda R_{\rho\sigma\mu\nu} + \nabla_\rho R_{\sigma\lambda\mu\nu} + \nabla_\sigma R_{\lambda\rho\mu\nu} = 0$.

(a) Contract this identity on $\lambda$ and $\mu$ (set $\lambda = \mu$ and sum) to get:
$$\nabla^\mu R_{\rho\sigma\mu\nu} + \nabla_\rho R_{\sigma\nu} - \nabla_\sigma R_{\rho\nu} = 0$$

(b) Contract again on $\rho$ and $\nu$ to get:
$$\nabla_\mu R^\mu_{\ \sigma} - \frac{1}{2}\nabla_\sigma R = 0$$
i.e., $\nabla_\mu G^{\mu\nu} = 0$.

(c) Explain why this identity, together with the Einstein equations $G_{\mu\nu} = 8\pi G T_{\mu\nu}$, implies $\nabla_\mu T^{\mu\nu} = 0$ — energy-momentum conservation. Is this a new physical law, or is it a consequence of the geometric structure?

(d) In electromagnetism, $\partial_\nu(\partial_\mu F^{\mu\nu}) = 0$ (because $F^{\mu\nu}$ is antisymmetric) implies $\partial_\nu J^\nu = 0$ (charge conservation). Compare the structure of this argument to the GR case. What is the GR analog of $F^{\mu\nu}$ being antisymmetric?

---

**24.2.2.** *The weak-field limit.*

Consider the metric $g_{\mu\nu} = \eta_{\mu\nu} + h_{\mu\nu}$ with $|h_{\mu\nu}| \ll 1$ and a slow-moving source ($v \ll c$).

(a) Compute the Christoffel symbols to first order in $h_{\mu\nu}$:
$$\Gamma^\rho_{\mu\nu} = \frac{1}{2}\eta^{\rho\sigma}(\partial_\mu h_{\nu\sigma} + \partial_\nu h_{\mu\sigma} - \partial_\sigma h_{\mu\nu})$$

(b) Compute the Ricci tensor to first order:
$$R_{\mu\nu} = \frac{1}{2}(-\Box\bar{h}_{\mu\nu} + \partial_\mu\partial^\alpha\bar{h}_{\alpha\nu} + \partial_\nu\partial^\alpha\bar{h}_{\alpha\mu} - \eta_{\mu\nu}\partial^\alpha\partial^\beta\bar{h}_{\alpha\beta})$$
where $\bar{h}_{\mu\nu} = h_{\mu\nu} - \frac{1}{2}\eta_{\mu\nu}h$ is the trace-reversed perturbation.

(c) In Lorenz gauge $\partial^\mu\bar{h}_{\mu\nu} = 0$, show the Einstein equations reduce to $\Box\bar{h}_{\mu\nu} = -16\pi G T_{\mu\nu}/c^4$.

(d) For a static weak field with $T^{00} = \rho c^2$ and all other components negligible, show the $00$-component gives $\nabla^2 h_{00} = -8\pi G\rho/c^2$, which (with $h_{00} = -2\Phi/c^2$) reduces to Poisson's equation $\nabla^2\Phi = 4\pi G\rho$. This is the Newtonian limit of GR.

---

**24.2.3.** *Lovelock's theorem and why Einstein gravity is unique.*

Lovelock (1971) proved the following theorem: in 4 spacetime dimensions, the only symmetric, divergence-free, rank-2 tensor built from the metric and its first two derivatives is $\alpha G_{\mu\nu} + \Lambda g_{\mu\nu}$.

(a) Why does this theorem "explain" the form of the Einstein equations? What constraints does it impose on modifications of GR?

(b) In $n > 4$ dimensions, there are additional Lovelock tensors. The Gauss-Bonnet term $\mathcal{G} = R^2 - 4R_{\mu\nu}R^{\mu\nu} + R_{\mu\nu\rho\sigma}R^{\mu\nu\rho\sigma}$ contributes a new tensor $H_{\mu\nu} = 2(RR_{\mu\nu} - 2R_{\mu\alpha}R^\alpha_{\ \nu} - 2R_{\alpha\beta}R^\alpha_{\ \mu}_{\ \nu}^{\ \beta} - R_{\mu\alpha\beta\gamma}R_\nu^{\ \alpha\beta\gamma}) - \frac{1}{2}g_{\mu\nu}\mathcal{G}$ in $n\geq 5$ dimensions. Show that $\nabla_\mu H^{\mu\nu} = 0$ identically (it is divergence-free).

(c) $f(R)$ gravity modifies the action to $S = \int f(R)\sqrt{-g}\,d^4x$. The field equations are $f'(R)G_{\mu\nu} + \frac{1}{2}g_{\mu\nu}(f(R) - Rf'(R)) - \nabla_\mu\nabla_\nu f'(R) + g_{\mu\nu}\Box f'(R) = 8\pi GT_{\mu\nu}$. These contain fourth derivatives of the metric and violate Lovelock's conditions. What goes wrong physically with higher-derivative theories?

(d) Solar system tests (perihelion precession, light deflection, Shapiro delay) constrain modifications of GR to less than $\sim 10^{-3}$. What does Lovelock's uniqueness theorem suggest about these tests?

---

**24.2.4.** *The cosmological constant problem.*

The observed vacuum energy density is $\rho_\Lambda = \Lambda c^2/(8\pi G) \approx 6 \times 10^{-27}$ kg/m$^3$.

(a) In quantum field theory, the zero-point energy of a free scalar field with mass $m$ contributes a vacuum energy $\rho_{\rm QFT} \sim \hbar c \Lambda_{\rm UV}^4/(16\pi^2)$ where $\Lambda_{\rm UV}$ is the UV cutoff. Taking $\Lambda_{\rm UV} = E_{\rm Planck}/(\hbar c) = c^3/(G\hbar)^{1/2} \approx 1.9 \times 10^{43}$ m$^{-1}$, compute $\rho_{\rm QFT}$ in kg/m$^3$.

(b) Compute the ratio $\rho_{\rm QFT}/\rho_\Lambda$. This is the "cosmological constant problem" — arguably the worst prediction in the history of physics. Express the discrepancy as a power of 10.

(c) The electroweak phase transition at $T \sim 100$ GeV changes the vacuum energy by $\sim (100\,\text{GeV})^4/(\hbar c)^3$. Compute this contribution in kg/m$^3$ and compare to the observed $\rho_\Lambda$.

(d) Why is it *harder* to explain why $\Lambda$ is small but nonzero than to explain why it might be exactly zero? (Hint: consider the fine-tuning required to cancel the QFT contribution against a bare cosmological constant to leave a residual of the observed magnitude.)

---

## Thought Experiments

**T24.1.** *Gravity as geometry, not force.*

A physicist friend argues: "Gravity is just a force like electromagnetism — there's no need to call it geometry." Construct the strongest possible counter-argument using only phenomena that have been experimentally confirmed. Your argument should explain why no coordinate transformation can globally eliminate gravity (unlike electromagnetism, which can be "gauged away" locally but not globally), why gravity must affect all forms of energy equally (the universality that makes geometric treatment unavoidable), and why the equivalence principle, which has been tested to $10^{-15}$, is most naturally expressed as a geometric statement.

---

**T24.2.** *The Riemann tensor as the "gravitational electric field."*

In electromagnetism, $F_{\mu\nu}$ (the field strength) is the physical observable — not $A_\mu$ (the potential), which can be gauged away. In GR, $\Gamma^\rho_{\mu\nu}$ (the connection) is not a tensor and can be made to vanish at a point by going to normal coordinates — analogous to gauging away the potential. The physical observable is $R^\rho_{\ \sigma\mu\nu}$ (the Riemann tensor) — it cannot be eliminated by any coordinate change.

Develop this analogy as precisely as you can. What is the GR analog of the Bianchi identity $dF = 0$? What is the GR analog of Maxwell's equations $d\star F = \mu_0\star J$? What is the GR analog of the Lorentz force law? Where does the analogy break down?

---

**T24.3.** *Curvature without embedding.*

For most of human history, geometry was conceived as properties of figures in Euclidean space. Riemann's 1854 insight was that curvature is an *intrinsic* property — it can be measured entirely from within the manifold, without reference to any ambient space.

A 2D being living on a sphere can determine the sphere's curvature by measuring the sum of angles in a large triangle, or the circumference of a large circle, or the holonomy of a vector after parallel transport around a loop. None of these measurements require any concept of the sphere being embedded in 3D space.

This is the decisive conceptual shift that made GR possible: if curvature is intrinsic, then spacetime itself (not a field in spacetime) can be curved. Work out what measurements a being in a 3D space would make to determine whether their space is curved. How would they measure the Riemann tensor directly, without reference to any higher-dimensional embedding space?

---

## Laboratory Exercise: Numerical Relativity Warm-Up

**L24.1.** *Computing curvature numerically.*

Write a program that, given a metric $g_{\mu\nu}(x)$ as input, automatically computes the Christoffel symbols, Riemann tensor, Ricci tensor, Ricci scalar, and Einstein tensor.

**Step 1:** Implement numerical differentiation to compute $\partial_\rho g_{\mu\nu}$ at a given point using central differences.

**Step 2:** From the first derivatives, compute $\Gamma^\rho_{\mu\nu} = \frac{1}{2}g^{\rho\sigma}(\partial_\mu g_{\nu\sigma} + \partial_\nu g_{\mu\sigma} - \partial_\sigma g_{\mu\nu})$.

**Step 3:** Implement the Riemann tensor formula:
$$R^\rho_{\ \sigma\mu\nu} = \partial_\mu\Gamma^\rho_{\nu\sigma} - \partial_\nu\Gamma^\rho_{\mu\sigma} + \Gamma^\rho_{\mu\lambda}\Gamma^\lambda_{\nu\sigma} - \Gamma^\rho_{\nu\lambda}\Gamma^\lambda_{\mu\sigma}$$

**Step 4:** Contract to get $R_{\mu\nu}$, $R$, and $G_{\mu\nu}$.

**Test cases:**
1. Flat Minkowski: $g_{\mu\nu} = \eta_{\mu\nu}$. Verify $R^\rho_{\ \sigma\mu\nu} = 0$ everywhere.
2. 2-sphere: $g_{ab} = \text{diag}(R^2, R^2\sin^2\theta)$. Verify $R = 2/R^2$.
3. Schwarzschild metric: Compute $R_{\mu\nu}$ and verify it vanishes for $r > 2GM/c^2$.
4. FLRW metric: $ds^2 = -c^2dt^2 + a(t)^2(dx^2+dy^2+dz^2)$ with $a(t) = t^{2/3}$ (matter-dominated). Compute $G_{00}$ and $G_{ii}$ and identify the energy density and pressure.

**Note:** Use symbolic differentiation (e.g., SymPy in Python) rather than numerical finite differences for the test cases, to avoid accumulation of numerical errors. For the FLRW test, verify the Friedmann equation $3H^2 = 8\pi G\rho$ where $H = \dot{a}/a$.

