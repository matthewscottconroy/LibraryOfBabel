# Unit X Problems: Exact Solutions — Schwarzschild and Beyond

*The Schwarzschild metric, geodesics, classical tests of GR, and black hole thermodynamics.*

**Difficulty:** ★ Introductory, ★★ Intermediate, ★★★ Advanced

---

## Part 1: The Schwarzschild Metric

**Problem 1.1** ★
The Schwarzschild metric in standard coordinates:

$$ds^2 = -\left(1-\frac{r_s}{r}\right)c^2dt^2 + \left(1-\frac{r_s}{r}\right)^{-1}dr^2 + r^2d\theta^2 + r^2\sin^2\!\theta\,d\phi^2$$

where $r_s = 2GM/c^2$.

(a) Verify that this is asymptotically flat: as $r\to\infty$, the metric approaches Minkowski.
(b) Compute $r_s$ for: (i) the Sun ($M = 2\times10^{30}$ kg), (ii) the Earth ($M = 6\times10^{24}$ kg), (iii) a stellar black hole ($M = 10 M_\odot$).
(c) The proper circumference of a circle at $r = R$ is $2\pi R$ (the metric is flat in the $\theta$-$\phi$ directions). The proper radial distance between $r = r_1$ and $r = r_2 > r_1$ is $\int_{r_1}^{r_2}(1-r_s/r)^{-1/2}dr$. For $r_1 = r_s$ and $r_2 = 2r_s$: evaluate this integral (substitution $r = r_s\sec^2\theta$).

**Problem 1.2** ★★
Killing vectors and conserved quantities: the Schwarzschild metric has two Killing vectors: $\xi^\mu_{(t)} = (1,0,0,0)$ (time translation) and $\psi^\mu_{(\phi)} = (0,0,0,1)$ (rotational symmetry).

(a) The conserved energy per unit mass: $e = -g_{\mu\nu}\xi^\mu_{(t)}u^\nu = (1-r_s/r)c^2\dot{t}$ (where dots are $d/d\tau$). Show this.

(b) The conserved angular momentum per unit mass: $\ell = g_{\mu\nu}\psi^\mu_{(\phi)}u^\nu = r^2\dot{\phi}$ (in the equatorial plane $\theta = \pi/2$). Show this.

(c) The normalization condition $u^\mu u_\mu = -c^2$: substitute $\dot{t}$ and $\dot{\phi}$ in terms of $e$ and $\ell$ to obtain:

$$\frac{\dot{r}^2}{2} = \frac{e^2 - c^2}{2c^2} - V_\text{eff}(r)$$

Find $V_\text{eff}(r)$ for massive particles. Identify the Newtonian, relativistic correction, and angular momentum terms.

**Problem 1.3** ★★
Circular orbits in Schwarzschild:

(a) Circular orbits satisfy $V_\text{eff}'(r) = 0$. Find $r$ as a function of $\ell$ for circular orbits.
(b) The condition for stability ($V_\text{eff}'' > 0$ at the circular orbit): show that circular orbits are stable only for $r > 6GM/c^2$ (the ISCO).
(c) The ISCO radius $r_\text{ISCO} = 6GM/c^2 = 3r_s$. For a $10 M_\odot$ black hole: compute $r_\text{ISCO}$ in km and the orbital period $T = 2\pi r_\text{ISCO}/v_\text{orb}$ (use $v_\text{orb}$ from the circular orbit condition).

**Problem 1.4** ★★★
Radial geodesics — falling into a black hole:

(a) For a radially infalling particle with $\ell = 0$ starting from rest at $r = r_0$: the energy $e = c^2\sqrt{1-r_s/r_0}$. Write the radial equation $\dot{r}^2 = \ldots$ and solve for $r(\tau)$ (proper time of the infalling particle).

(b) Show that the proper time to fall from $r = r_0$ to $r = 0$ is finite: $\Delta\tau = \frac{\pi r_0}{2c}\sqrt{r_0/r_s}$.

(c) In coordinate time $t$: show that $r(t)\to r_s$ asymptotically (the infalling particle never crosses $r_s$ in coordinate time). Compute $dr/dt$ near $r = r_s$ and show it approaches zero.

(d) Resolve the apparent paradox: the in-falling observer reaches $r = r_s$ in finite proper time; a distant observer never sees this happen. Which description is "correct"?

---

## Part 2: Classical Tests of General Relativity

**Problem 2.1** ★★
Gravitational redshift:

(a) A photon emitted at $r = R$ with frequency $\nu_e$ is received at $r\to\infty$ with frequency $\nu_r$. Using the conservation of $e = (1-r_s/r)c^2\dot{t}$ and the relation $E = h\nu$ for photons: show $\nu_r = \nu_e\sqrt{1-r_s/R}$.

(b) The fractional redshift $z = (\nu_e - \nu_r)/\nu_r$. For a photon emitted from the solar surface ($R = R_\odot$): compute $z$.

(c) The Pound-Rebka experiment (1959) measured the gravitational redshift of gamma rays over $h = 22.5$ m in Earth's gravity. The predicted fractional frequency shift is $\Delta\nu/\nu = gh/c^2$. For $g = 9.8$ m/s²: compute the shift and comment on the precision needed.

**Problem 2.2** ★★
Light deflection:

(a) For a massless particle ($ds^2 = 0$) in the Schwarzschild metric, the effective potential for radial motion (using $e$, $\ell$) is $V_\text{eff,light}(r) = (1-r_s/r)\ell^2/r^2$. Find the maximum (the "photon sphere") at $r = 3GM/c^2 = 3r_s/2$.

(b) For light passing the Sun with impact parameter $b$ (the perpendicular distance from the Sun to the unperturbed trajectory): treat the deflection as a perturbation. The first-order result is $\delta\phi = 4GM/(bc^2) = 2r_s/b$.

For a ray grazing the solar limb ($b = R_\odot$): compute $\delta\phi$ in arcseconds. (Eddington measured $1.98\pm 0.16"$ during the 1919 eclipse; predicted $1.75"$.)

(c) Why does GR predict twice the Newtonian deflection? (The Newtonian calculation, treating photons as particles, gives $\delta\phi = 2GM/(bc^2)$. The extra factor of 2 comes from the spatial curvature of the Schwarzschild metric — the bending of space, not just the "gravitational attraction" of time.)

**Problem 2.3** ★★★
Perihelion precession:

(a) The orbit equation from the geodesic equations (with $u = 1/r$): $d^2u/d\phi^2 + u = GM/\ell^2 + 3GMu^2/c^2$. The last term is the relativistic correction.

(b) Write $u = u_0(1 + \epsilon\cos\phi)$ (Keplerian orbit) with $u_0 = GM/\ell^2/(1-e^2)$... actually, the Keplerian solution is $u_0 = GM c^2/(c^2\ell^2) (1 + e\cos\phi)$ approximately. Substitute into the relativistic term (treat as a small perturbation with $\delta u = 3GMu_0^2/c^2$) and integrate to find the secular precession.

(c) The result: $\Delta\phi_\text{prec} = 6\pi GM/(c^2 a(1-e^2))$ per orbit. Numerically verify for Mercury and match to the observed $43''/\text{century}$.

(d) Other contributions to Mercury's perihelion advance: Venus (277.9"/century), Jupiter (152.6"/century), Earth (90.0"/century), and other planets together with oblateness. The total Newtonian contribution is $\sim 531.5"/\text{century}$; observations give $574.1"/\text{century}$. The residual $42.98\pm0.04"/\text{century}$ is explained by GR.

---

## Part 3: Black Hole Thermodynamics (Conceptual)

**Problem 3.1** ★★
Black hole mechanics: analogues of the laws of thermodynamics:

| Thermodynamics | Black hole mechanics |
|---|---|
| 0th law: $T$ constant in equilibrium | Surface gravity $\kappa$ constant on the event horizon |
| 1st law: $dE = TdS - pdV$ | $dM = \kappa/(8\pi G/c^2) dA + \Omega_H dJ + \Phi_H dQ$ |
| 2nd law: $dS\geq0$ | $dA\geq0$ (area increase theorem) |
| 3rd law: $T\to0$ unattainable | $\kappa\to0$ unattainable |

(a) The surface gravity of a Schwarzschild black hole: $\kappa = c^4/(4GM)$. Compute $\kappa$ for a $10 M_\odot$ black hole.

(b) The Bekenstein-Hawking entropy: $S_\text{BH} = k_B A/(4\ell_P^2)$ where $A = 4\pi r_s^2$ is the horizon area and $\ell_P = \sqrt{\hbar G/c^3}$ is the Planck length. Compute $S_\text{BH}$ for a $10 M_\odot$ black hole in units of $k_B$.

(c) The Hawking temperature: $T_H = \hbar\kappa/(2\pi k_B c)$. Compute $T_H$ for a $10 M_\odot$ black hole. Compare to the CMB temperature $T_\text{CMB} = 2.725$ K.

(d) The information paradox: Hawking radiation is thermal — it carries no information about the matter that formed the black hole. If black holes evaporate completely, where does the information go? State the current status of this open problem.

**Problem 3.2** ★★★
The area increase theorem (Hawking, 1971):

(a) In classical GR (assuming the null energy condition: $T_{\mu\nu}k^\mu k^\nu\geq 0$ for all null vectors $k^\mu$), Hawking proved that the area of the event horizon can never decrease. This is the Penrose-Hawking area theorem.

(b) Two black holes of masses $M_1$ and $M_2$ merge to form a black hole of mass $M_f$. What is the minimum $M_f$ consistent with the area theorem? (Compute $A_1 + A_2 \leq A_f$.)

(c) LIGO observed GW150914: two black holes of $36 M_\odot$ and $29 M_\odot$ merging to produce a $62 M_\odot$ black hole (with $3 M_\odot$ radiated as gravitational waves). Verify that the area theorem is satisfied.

(d) Quantum effects (Hawking radiation) violate the classical area theorem. In the quantum theory, the generalized second law (GSL) states that the total entropy $S_\text{matter} + S_\text{BH}$ never decreases. Explain how the GSL is consistent with Hawking evaporation decreasing the horizon area.
