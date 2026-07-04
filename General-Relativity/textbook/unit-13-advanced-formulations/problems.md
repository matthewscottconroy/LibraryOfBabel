# Unit XIII Problems: Advanced General Relativity

*Penrose-Hawking singularity theorems, Hawking radiation, conformal methods, and the Penrose process.*

**Difficulty:** ★ Introductory, ★★ Intermediate, ★★★ Advanced

---

## Part 1: Penrose-Hawking Singularity Theorems

**Problem 1.1** ★★
Prerequisites: energy conditions and trapped surfaces.

(a) The **null energy condition** (NEC): $T_{\mu\nu}k^\mu k^\nu\geq 0$ for all null vectors $k^\mu$. Via the Einstein equations, this implies $R_{\mu\nu}k^\mu k^\nu \geq 0$ (the Ricci curvature is non-negative along null directions). Show that a perfect fluid with $\rho + p/c^2\geq0$ satisfies the NEC.

(b) The **strong energy condition** (SEC): $T_{\mu\nu}t^\mu t^\nu \geq \frac{1}{2}T t^\mu t_\mu$ for all timelike $t^\mu$. For a perfect fluid: $\rho c^2 + 3p\geq0$ and $\rho c^2 + p\geq0$. Does a cosmological constant $\Lambda > 0$ satisfy the SEC?

(c) A **trapped surface**: a compact 2-surface where both ingoing and outgoing null geodesics converge (have negative expansion $\theta$). The expansion of a null congruence with tangent $k^\mu$: $\theta = \nabla_\mu k^\mu$. For the ingoing null normal to a sphere of radius $r$ in Schwarzschild: show $\theta_\text{in} < 0$ for all $r$, and $\theta_\text{out} < 0$ for $r < r_s$ (inside the horizon), while $\theta_\text{out} > 0$ for $r > r_s$.

**Problem 1.2** ★★★
The Penrose singularity theorem (1965): If the NEC holds and spacetime contains a trapped surface, then it must contain a singularity (a geodesic of finite affine length that cannot be extended).

(a) The Raychaudhuri equation for null congruences:
$$\frac{d\theta}{d\lambda} = -\frac{1}{2}\theta^2 - \sigma_{\mu\nu}\sigma^{\mu\nu} + \omega_{\mu\nu}\omega^{\mu\nu} - R_{\mu\nu}k^\mu k^\nu$$
where $\sigma$ is shear, $\omega$ is rotation. Assuming the NEC and irrotational congruences ($\omega = 0$): show $d\theta/d\lambda \leq -\theta^2/2$.

(b) This ODE implies $\theta(\lambda)^{-1} \geq \theta_0^{-1} + \lambda/2$. If $\theta_0 < 0$ (converging congruence): show that $\theta\to-\infty$ within finite affine parameter $\lambda\leq -2/\theta_0$.

(c) Divergence of $\theta$ signals a **caustic** — neighboring null geodesics cross. Penrose proved (using global methods) that the existence of a caustic and trapped surface implies a singularity. State the key topological assumption Penrose made about spacetime (it must be **globally hyperbolic**).

(d) The Hawking singularity theorem (1967) applies the same logic to cosmology: replacing trapped surfaces with the condition that $H > 0$ everywhere (expanding universe), it proves a past singularity (the Big Bang). What energy condition is needed?

**Problem 1.3** ★★★
Cosmic censorship: the Penrose cosmic censorship conjecture states that singularities in GR are always hidden behind event horizons (weak form) or that spacetime is globally hyperbolic (strong form).

(a) A **naked singularity** is one not hidden behind a horizon. Give an example of a classical GR solution with a naked singularity. (Hint: the Kerr metric with $a > M$ in natural units is over-extreme and has no horizon.)

(b) Penrose argued that naked singularities are physically inadmissible because they would violate predictability (the determinism of GR). Relate this to the concept of a Cauchy horizon.

(c) Can you create a naked singularity by throwing matter into a nearly-extreme Kerr black hole to push $a/M > 1$? The third law of black hole mechanics suggests this is impossible. Give a heuristic argument why the last bit of angular momentum cannot be added.

---

## Part 2: Hawking Radiation

**Problem 2.1** ★★
Quantum fields in curved spacetime — the Unruh effect:

(a) A uniformly accelerating observer (Rindler observer) with acceleration $a$ perceives the Minkowski vacuum as a thermal bath at the **Unruh temperature**: $T_U = \hbar a/(2\pi k_B c)$. For $a = 10^{20}$ m/s² (the largest table-top acceleration achievable): compute $T_U$.

(b) The Bogoliubov transformation: the vacuum state $|0\rangle_M$ defined by inertial observers in Minkowski space is expressed in terms of Rindler modes (modes natural to the accelerating observer). The key formula: $a^\text{Rindler}_\omega|0\rangle_M \neq 0$ — the Rindler "particle number operator" annihilator does not annihilate the Minkowski vacuum. This is the formal statement that the accelerating observer sees particles.

(c) By the equivalence principle, an observer hovering at fixed $r$ in the Schwarzschild metric is locally equivalent to an accelerating observer in flat space (they experience a gravitational force). Their local acceleration is $a = GM/(r^2\sqrt{1-r_s/r})$. At the horizon ($r\to r_s$): what happens to $a$?

**Problem 2.2** ★★★
Hawking temperature derivation sketch:

The Hawking temperature can be estimated using the following argument.

(a) Near the Schwarzschild horizon ($r = r_s$), define the Killing parameter $\kappa$ (surface gravity): the norm of the time-translation Killing vector $\xi^\mu = (1,0,0,0)$ satisfies $\nabla^\mu(-\xi_\nu\xi^\nu) = 2\kappa\xi^\mu$ on the horizon. Show that $\kappa = c^4/(4GM) = c^2/(2r_s)$.

(b) The Hawking temperature: $T_H = \hbar\kappa/(2\pi k_B c)$. This follows from the Bogoliubov transformation relating modes defined by an infalling observer (free fall through the horizon) to modes defined by a static observer. The key step is analytic continuation: the modes have the Planckian spectrum $\propto (e^{2\pi\omega/\kappa}-1)^{-1}$ when expanded in terms of static observer modes.

(c) Compute $T_H$ for: (i) a $10 M_\odot$ stellar black hole, (ii) a $10^6 M_\odot$ galactic black hole, (iii) a $10^{-5}$ g micro-black hole (hypothetical). For which case is quantum evaporation important?

(d) The evaporation timescale: $\tau_\text{evap} = 5120\pi G^2 M^3/(\hbar c^4)$. For a $1$ kg black hole: compute $\tau_\text{evap}$. Compare to the age of the universe.

**Problem 2.3** ★★★
The information paradox:

(a) Pure state vs. mixed state: a black hole forms from a pure quantum state (a definite quantum configuration of matter). Hawking radiation is thermal — it is a mixed state (maximal entropy). If the black hole evaporates completely, the final state is mixed, violating unitary evolution. State this as a paradox precisely.

(b) Page's theorem: the entanglement entropy $S_\text{ent}$ between the black hole and the radiation first increases, then decreases, reaching zero when the black hole has evaporated. The "Page time" is approximately $t_\text{Page} \sim M^3$ (same order as half the evaporation time). Before the Page time, the radiation is close to the vacuum; after, it is close to the reference pure state.

(c) The firewall paradox (AMPS, 2013): if information is preserved (unitarity holds), then late-time Hawking radiation must be entangled with early-time radiation. But late-time Hawking modes are also entangled with the interior modes (entanglement across the horizon) — and quantum mechanics forbids a mode being maximally entangled with two independent systems. The apparent resolution requires a "firewall" (high-energy excitations) at the horizon — destroying the smooth spacetime that classical GR predicts. State the three assumptions that cannot simultaneously hold.

---

## Part 3: Global Methods in GR

**Problem 3.1** ★★
Penrose diagrams (conformal compactification):

(a) Minkowski space: the coordinates $t, r$ range over $t\in(-\infty,\infty)$, $r\in[0,\infty)$. Under the compactification $T = \arctan(t+r) + \arctan(t-r)$, $R = \arctan(t+r) - \arctan(t-r)$: the entire Minkowski space maps to a finite diamond. Draw the Penrose diagram, marking $i^\pm$ (future/past timelike infinity), $i^0$ (spacelike infinity), and $\mathscr{I}^\pm$ (null infinity).

(b) The Schwarzschild Penrose diagram (Kruskal extension): draw the diagram with four regions: exterior (I), interior (II, future singularity), another exterior (III), and past interior (IV, past singularity). Mark the event horizon, singularity, and the two exterior regions.

(c) In the Penrose diagram, a black hole is defined as the region from which no null geodesic can escape to $\mathscr{I}^+$. The **event horizon** is the boundary of this region. Show that this is a global (teleological) definition: knowing whether a given spacetime point is inside the event horizon requires knowing the entire future of the spacetime.

**Problem 3.2** ★★★
The Penrose process and the ergosphere:

(a) The Kerr metric has an ergosphere (the region between the horizon and the static limit, where $g_{tt} > 0$). Inside the ergosphere, the Killing vector $\partial_t$ becomes spacelike. What does this imply for the energy $E = -p_\mu(\partial_t)^\mu$ of a particle?

(b) The Penrose process: a particle with energy $E_0 > 0$ enters the ergosphere and splits into two: particle 1 escapes with energy $E_1 > E_0$ and particle 2 falls into the black hole with $E_2 = E_0 - E_1 < 0$. The Hawking area theorem: $M$ decreases but $J$ decreases too. Show that the maximum energy extractable is bounded by $M - M_\text{irr}$ where $M_\text{irr} = \sqrt{r_+/2}/(G/c^2)$ is the irreducible mass.

(c) The Blandford-Znajek mechanism: the electromagnetic generalization of the Penrose process, where magnetic field lines threading the Kerr ergosphere extract rotational energy via the ergospheric Penrose effect. This mechanism powers relativistic jets from active galactic nuclei. State the physical picture without derivation.
