# Chapter 60: Global Methods in General Relativity

---

## Chapter Introduction

Most of our work in GR has been local: computing connection coefficients, solving the geodesic equation in a specific spacetime, deriving field equations from an action. But some of the deepest questions in GR are global: Can a singularity be avoided by choosing initial conditions carefully? Are all singularities hidden by horizons? Does a spacetime with certain properties necessarily contain trapped surfaces? Does the past have to be singular?

The **global methods** of GR — developed primarily by Penrose, Hawking, Geroch, and their collaborators in the 1960s–1970s — answer these questions using the large-scale causal and topological structure of spacetime. The tools are those of differential geometry and topology: causal sets, Cauchy surfaces, trapped surfaces, Jacobi fields, and the Raychaudhuri equation for null congruences.

This chapter completes the discussion from Chapter 59 (trapped surfaces and singularity theorems) by developing the global methods more systematically: the domain of dependence, the Cauchy problem in GR, the topology of spacetime, and the theorems about cosmic censorship that constrain where and how singularities can form. We also treat **positive energy theorems** — results showing that the total energy of any asymptotically flat spacetime satisfying energy conditions is non-negative, with equality only for flat spacetime.

---

## Causal Structure: Definitions

Let $(M, g)$ be a spacetime (connected, time-oriented Lorentzian manifold).

**Chronological and causal relations**:
- $p\ll q$ ($p$ chronologically precedes $q$): there exists a future-directed timelike curve from $p$ to $q$
- $p < q$ ($p$ causally precedes $q$): there exists a future-directed causal (timelike or null) curve from $p$ to $q$
- $p\leq q$: $p < q$ or $p = q$

**Chronological and causal futures/pasts**:
- $I^+(p)$: chronological future of $p$ — all $q$ with $p\ll q$
- $J^+(p)$: causal future of $p$ — all $q$ with $p\leq q$
- $I^-(p)$, $J^-(p)$: corresponding pasts

**Causally convex sets**: A set $U$ is causally convex if any causal curve with endpoints in $U$ lies entirely in $U$.

**Cauchy surfaces**: A **Cauchy surface** $\Sigma$ is a spacelike hypersurface such that every inextendible causal curve (timelike or null) intersects $\Sigma$ exactly once. A spacetime admitting a Cauchy surface is **globally hyperbolic**.

**Global hyperbolicity** is the physically relevant condition for the Cauchy problem: given initial data on $\Sigma$ (the fields and their first derivatives), the future evolution is unique and exists for all $J^+(\Sigma)$ (up to singularities). Minkowski space, Schwarzschild exterior, and the FLRW spacetimes are globally hyperbolic. The maximal Kerr spacetime (with its Cauchy horizon) is not.

---

## Domain of Dependence

The **domain of dependence** $D(\Sigma)$ of a spacelike set $\Sigma$ is the set of all points $p$ such that every inextendible causal curve through $p$ must intersect $\Sigma$.

- $D^+(\Sigma)$: future domain of dependence
- $D^-(\Sigma)$: past domain of dependence
- $D(\Sigma) = D^+(\Sigma)\cup D^-(\Sigma)$

**Significance**: Within $D(\Sigma)$, the solution to any well-posed hyperbolic PDE (Einstein equations, Maxwell equations, Klein-Gordon) is uniquely determined by initial data on $\Sigma$. Outside $D(\Sigma)$, the solution may not be determined — other boundary data is needed.

**Cauchy horizon** $H(\Sigma)$: The boundary of $D(\Sigma)$ — the set $H(\Sigma) = \overline{D(\Sigma)}\setminus\text{int}(D(\Sigma))$. The Cauchy horizon is a null surface. If it exists, the Cauchy problem breaks down beyond $H(\Sigma)$.

**Example**: In Kerr spacetime, the inner Cauchy horizon $r = r_-$ is the boundary of the domain of dependence of any initial Cauchy surface. Strong Cosmic Censorship (discussed below) conjectures that this horizon is unstable and becomes singular.

---

## The Focusing Theorem and Conjugate Points

The **Raychaudhuri equation** (derived in Chapter 59) for null geodesic congruences with expansion $\theta$, shear $\sigma_{\mu\nu}$, vorticity $\omega_{\mu\nu}$:
$$\frac{d\theta}{d\lambda} = -\frac{\theta^2}{2} - \sigma_{\mu\nu}\sigma^{\mu\nu} + \omega_{\mu\nu}\omega^{\mu\nu} - R_{\mu\nu}k^\mu k^\nu$$

Under the **null energy condition (NEC)** $R_{\mu\nu}k^\mu k^\nu\geq 0$ (equivalently $T_{\mu\nu}k^\mu k^\nu\geq 0$) and for hypersurface-orthogonal congruences ($\omega = 0$):
$$\frac{d\theta}{d\lambda}\leq -\frac{\theta^2}{2}$$

If $\theta_0 < 0$ at some point, then $\theta\to -\infty$ within affine parameter $2/|\theta_0|$ — the null rays **focus** (converge to a caustic).

**Conjugate points**: Two points $p$ and $q$ on a null geodesic are **conjugate** if there exists a Jacobi field (a solution of the geodesic deviation equation) that vanishes at both. At a conjugate point, the geodesic ceases to be a boundary of the causal future.

**Key theorem**: If the NEC holds and $\theta < 0$ at some point on a null geodesic, then the geodesic has a conjugate point (a caustic) at a finite affine parameter. Beyond this point, the geodesic is not on the boundary of any causal set.

---

## Trapped Surfaces and Apparent Horizons

A **closed trapped surface** $S$ is a compact, spacelike 2-surface such that both families of outgoing null normals have negative expansion: $\theta_+ < 0$ and $\theta_- < 0$.

**Physical interpretation**: In flat space, a sphere of light emitted from a 2-sphere expands. On a trapped surface, even the outgoing light is converging — the surface is so strongly gravitating that nothing can escape.

**Apparent horizon**: The outermost marginally trapped surface — where $\theta_+ = 0$ (the outgoing null expansion just vanishes). The apparent horizon is the observable boundary of the trapping region. It is defined quasi-locally (it depends on the time slice) and is in general inside or coincident with the event horizon.

**Event horizon**: The boundary of $J^-(\mathscr{I}^+)$ — the set of points that can send signals to future null infinity. The event horizon is a global concept (it depends on the entire future history of the spacetime) and is always a null surface. For a stationary black hole in equilibrium, the apparent horizon and event horizon coincide.

**Area theorem** (Hawking): Under the weak energy condition and with cosmic censorship, the area of the event horizon is non-decreasing: $\delta A\geq 0$. This is the second law of black hole mechanics.

---

## Cosmic Censorship

**Naked singularity**: A singularity visible from future null infinity — one that lies in $J^-(\mathscr{I}^+)$. A spacetime with a naked singularity has incomplete predictability: observers can be affected by the singularity without any warning.

**Weak Cosmic Censorship Conjecture (WCCC)** (Penrose, 1969): Generically, naked singularities cannot form from regular initial data satisfying energy conditions. All singularities are hidden inside event horizons.

"Generically" is crucial — there are known naked singularity solutions (Oppenheimer-Snyder with special parameters, certain Kerr-Newman extremal solutions), but these require fine-tuned initial data.

**Evidence for WCCC**:
- Gravitational collapse of realistic matter generally produces black holes (numerical evidence)
- Gedanken experiments to "overcharge" or "overspin" a black hole (violating the cosmic censorship bound $M^2 \geq Q^2/G + J^2c^2/(G^2M^2)$) always fail due to backreaction

**Strong Cosmic Censorship Conjecture (SCCC)**: Maximal Cauchy developments of regular initial data are inextendible as Lorentzian manifolds (no Cauchy horizons). The Cauchy horizon of Kerr is unstable: perturbations diverge on the Cauchy horizon ($r = r_-$), turning it into a singularity, restoring predictability.

**Recent challenge to SCCC**: Dafermos and Luk (2017) showed that generic perturbations of Kerr remain continuous (but not differentiable) across the Cauchy horizon. The singularity at $r_-$ is weak enough that observers can still pass through. Whether this counts as a violation of SCCC depends on what "inextendible" means — and this has sparked ongoing debate.

---

## The Positive Energy Theorem

**Arnowitt-Deser-Misner (ADM) mass**: For an asymptotically flat spacetime, the total energy at spatial infinity:
$$E_{\rm ADM} = \frac{c^4}{16\pi G}\lim_{r\to\infty}\oint({\partial_j g_{ij} - \partial_i g_{jj}})\,dS^i$$

(integral over a 2-sphere at spatial infinity).

**Positive Energy Theorem** (Schoen-Yau 1979, Witten 1981): For any asymptotically flat spacetime satisfying the dominant energy condition ($T_{\mu\nu}v^\mu w^\nu\geq 0$ for all causal $v, w$), the ADM mass satisfies $E_{\rm ADM}\geq 0$, with equality only for flat Minkowski spacetime.

**Proof ideas**:
- *Schoen-Yau (1979)*: Minimal surface methods; uses the stability of minimal hypersurfaces.
- *Witten (1981)*: Spinor methods; find a spinor $\psi$ satisfying a Dirac-type equation, write $E_{\rm ADM}$ as a positive-definite integral involving $\psi$.

**Physical significance**: The positive energy theorem proves that gravity cannot produce negative total energy through any process. It rules out the possibility of "gravitational perpetual motion." The Bondi mass (energy at null infinity) also satisfies a positive energy theorem with equality only for flat space.

**Extensions**:
- *Riemannian Penrose inequality* (Bray, 2001): $E_{\rm ADM}\geq c^2\sqrt{A/(16\pi G^2)}$ for spacetimes containing a black hole with horizon area $A$. This constrains the mass given the horizon area — or equivalently, it constrains the black hole area given the mass.

---

## Topological Censorship

**Topological Censorship Theorem** (Friedman-Schleich-Witt 1993): In an asymptotically flat spacetime satisfying the averaged null energy condition, any causal curve connecting two points at null infinity can be continuously deformed to a curve that lies entirely in the exterior. In other words: the region outside black holes is simply connected.

**Corollary**: A stationary black hole can have at most one asymptotic region (no topological handles connecting the exterior to another universe that can be traversed). Wormholes that are traversable from the outside require negative energy, violating the ANEC.

This theorem rules out macroscopic wormholes that could provide shortcuts through space without negative energy — consistent with our inability to observe such structures.

---

## The Penrose Process and Superradiance (Global Perspective)

The **Penrose process**: A particle with energy $E > 0$ enters the ergosphere of a Kerr black hole, splits into two fragments. One fragment ($E_2 < 0$, measured at infinity) falls through the horizon; the other ($E_1 = E - E_2 > E$) escapes with more energy than the original particle. Energy is extracted from the black hole's rotation.

The extracted energy comes from the rotational kinetic energy of the black hole (which decreases as $J$ decreases). The area theorem requires $\delta A\geq 0$:
$$\delta A = 8\pi r_+\left(\delta M - \Omega_H\delta J\right) \geq 0$$

This means $\delta M\geq\Omega_H\delta J$ — you cannot extract more than $\Omega_H\delta J$ per change $\delta J$ in angular momentum.

**Superradiance**: A wave of frequency $\omega$ and azimuthal quantum number $m$ incident on a Kerr black hole is amplified if $\omega < m\Omega_H$ — the wave extracts energy from the rotation. This is the wave analogue of the Penrose process. Superradiance is relevant for the "black hole bomb" instability (when a massive field is confined between the horizon and its own mass gap).

---

## Important Concepts

- **Globally hyperbolic**: Spacetime admitting a Cauchy surface; Cauchy problem is well-posed
- **Domain of dependence** $D(\Sigma)$: Region where physics is determined by data on $\Sigma$
- **Cauchy horizon**: Boundary of $D(\Sigma)$; where predictability breaks down
- **Trapped surface**: Compact spacelike 2-surface with both null normals having $\theta < 0$
- **Apparent horizon**: Outermost marginally trapped surface ($\theta_+ = 0$); quasi-local
- **Event horizon**: Global boundary of causal past of $\mathscr{I}^+$; null surface
- **Area theorem**: $\delta A\geq 0$ for event horizon under WEC; second law of BH mechanics
- **Weak Cosmic Censorship**: No naked singularities from regular initial data (generically)
- **Strong Cosmic Censorship**: No Cauchy horizons; maximal Cauchy development is inextendible
- **Positive energy theorem**: ADM mass $\geq 0$ under dominant energy condition; equality iff Minkowski
- **Topological censorship**: Simply connected exterior; no traversable wormholes without exotic matter

---

## Important Figures

**Roger Penrose** (1931–): Cosmic censorship conjectures; singularity theorems; Penrose inequalities; Penrose diagrams; Nobel Prize 2020.

**Robert Geroch** (1942–): Systematic global analysis of GR; splitting theorems; topology change; Geroch's theorem.

**Shing-Tung Yau** (1949–) and **Richard Schoen** (1950–): Proved the positive energy theorem using minimal surfaces (1979); Fields Medal for Yau (1982).

**Edward Witten** (1951–): Proved positive energy theorem using spinors (1981); Fields Medal 1990.

**Mihalis Dafermos** (1976–) and **Jonathan Luk**: Analysis of strong cosmic censorship for Kerr; showed Cauchy horizon is weakly singular (2017).

---

## Further Reading

**Primary Sources**
- Penrose, R. (1969). "Gravitational Collapse: The Role of General Relativity." *Rivista del Nuovo Cimento*, 1, 252.
- Schoen, R. & Yau, S.-T. (1979). "On the Proof of the Positive Mass Conjecture in General Relativity." *Comm. Math. Phys.*, 65, 45.
- Witten, E. (1981). "A New Proof of the Positive Energy Theorem." *Comm. Math. Phys.*, 80, 381.
- Friedman, J.L., Schleich, K., & Witt, D.M. (1993). "Topological Censorship." *Phys. Rev. Lett.*, 71, 1486.

**Textbooks**
- Hawking, S.W. & Ellis, G.F.R. (1973). *The Large Scale Structure of Space-Time*. Cambridge. — The definitive reference.
- Wald, R.M. (1984). *General Relativity*. Chapter 9. — Global methods; careful and rigorous.
- Kriele, M. (1999). *Spacetime: Foundations of General Relativity and Differential Geometry*. Springer. — Mathematical treatment.

---

## Exercises

**60.1.** *Causal structure.*

(a) In Minkowski spacetime with coordinates $(t, x)$: the set $S = \{(t,x): t = 0, 0\leq x\leq 1\}$. What is $D^+(S)$? Describe it geometrically.

(b) In the Schwarzschild exterior ($r > r_s$): is a Cauchy surface $\Sigma = \{t = 0\}$ a complete Cauchy surface for the exterior? Does the Cauchy horizon of $\Sigma$ exist?

(c) The Cauchy horizon of Reissner-Nordström (charged black hole) is at $r = r_-$. Explain qualitatively why data given on an exterior Cauchy surface cannot determine the evolution in the region $r < r_-$.

---

**60.2.** *Positive energy theorem.*

(a) For a spherically symmetric, asymptotically flat spacetime with $g_{tt} = -(1 - 2GM(r)/(rc^2))$ (Schwarzschild-like), show that the ADM mass equals $M(\infty)$ — the total mass function evaluated at infinity.

(b) Under what energy conditions is $M(r)$ a non-decreasing function of $r$? (Use the contracted Bianchi identity and the structure of the Einstein equations.)

(c) The Riemannian Penrose inequality states $E_{\rm ADM}\geq c^2\sqrt{A_{\rm min}/(16\pi G^2)}$ where $A_{\rm min}$ is the minimal area of the apparent horizon. For a Schwarzschild black hole with mass $M$: verify this inequality is saturated (equality holds).

---

**60.3.** *Topological censorship application.*

(a) A Morris-Thorne wormhole has two asymptotically flat ends connected by a throat. Using topological censorship, what does this imply about the energy content of the wormhole throat?

(b) The NEC violation required: the minimum exotic energy density is $\rho_{\rm exotic} \sim \hbar c/(r_0^4)$ where $r_0$ is the throat radius. For a traversable wormhole with $r_0 = 1$ m, compute $\rho_{\rm exotic}$. Compare to nuclear matter density ($\sim 10^{17}$ kg/m$^3$).

(c) Could quantum effects (Casimir energy, vacuum fluctuations) provide the required NEC violation? The Casimir energy density is $\rho_{\rm Cas} \sim -\hbar c/(d^4)$ for plate separation $d$. Can $d$ be set so that $|\rho_{\rm Cas}|$ is sufficient?

---

**Thought Experiment T60.1.** *Is the universe predictable?*

Einstein's general covariance and the Cauchy problem: if we know the complete state of the universe on a Cauchy surface today, do the laws of GR uniquely determine the future?

The answer is complicated by:
1. **Singularities**: Einstein's equations break down there, so evolution cannot be continued.
2. **Cauchy horizons** (Kerr, Reissner-Nordström): Beyond them, the future is not determined by the initial data.
3. **Topology change**: Is it possible for the topology of spatial slices to change? (Geroch's theorem says it can, but requires either closed timelike curves or violations of energy conditions.)

Strong cosmic censorship says Cauchy horizons are unstable — they become singular, restoring predictability. But this is a conjecture, not proven for the general case.

Does GR predict its own breakdown (singularities) and its own unpredictability (Cauchy horizons)? Is the universe fundamentally deterministic under GR, or does GR contain seeds of its own incompleteness?
