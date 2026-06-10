# Chapter 43: Other Exact Solutions

---

## Chapter Introduction

Schwarzschild and Kerr are the most physically important vacuum solutions, but the space of exact solutions to the Einstein equations is vast. Over the past century, physicists and mathematicians have discovered hundreds of exact solutions representing cosmological models, gravitational waves, gravitational instantons, wormholes, and spacetimes with exotic matter distributions. Each solution illuminates a different aspect of general relativity.

This chapter surveys several important families beyond Schwarzschild and Kerr: the Reissner-Nordström (charged) solution, the Kerr-Newman (charged rotating) solution, the interior Schwarzschild solution for uniform-density stars, the de Sitter and anti-de Sitter spacetimes (maximally symmetric with $\Lambda > 0$ and $\Lambda < 0$), and gravitational wave solutions.

The guiding principle in finding exact solutions is symmetry. The field equations simplify enormously when we assume spacetime has a Killing vector field (or several). The strategy is to write down the most general metric consistent with the assumed symmetry, then solve the resulting reduced equations. The classification of spacetimes by their symmetry groups (the Petrov classification) is a powerful organizing principle.

---

## Reissner-Nordström: The Charged Black Hole

The Reissner-Nordström (RN) metric describes a spherically symmetric, static, electrically charged black hole. It solves the Einstein-Maxwell equations with charge $Q$ and mass $M$:

$$ds^2 = -f(r)c^2dt^2 + f(r)^{-1}dr^2 + r^2d\Omega^2$$

where:
$$f(r) = 1 - \frac{r_s}{r} + \frac{r_Q^2}{r^2}, \quad r_s = \frac{2GM}{c^2}, \quad r_Q^2 = \frac{GQ^2}{4\pi\varepsilon_0 c^4}$$

**Horizons**: $f(r) = 0$ gives $r_\pm = \frac{r_s}{2}\pm\sqrt{\frac{r_s^2}{4} - r_Q^2}$.

Three cases:
- $|Q| < Q_{\rm max}$ (sub-extremal): two horizons $r_+ > r_-$; causal structure similar to Kerr
- $|Q| = Q_{\rm max}$ (extremal, $r_Q = r_s/2$): $r_+ = r_-$, $T_H = 0$; extremal RN
- $|Q| > Q_{\rm max}$ (super-extremal): no horizons; naked singularity (unphysical)

**Singularity**: The RN singularity at $r = 0$ is timelike (like Kerr), not spacelike. An infalling observer can theoretically avoid it and emerge into another universe via the inner horizon.

**Physical relevance**: Astrophysical black holes are essentially uncharged — any charge is quickly neutralized by accreting the opposite sign. RN is more important as a theoretical toy model and as a building block for the full Kerr-Newman solution.

---

## Kerr-Newman: The General Black Hole

The Kerr-Newman solution generalizes Kerr to include electric charge $Q$. It has mass $M$, angular momentum $J = Mac$, and charge $Q$. The metric is:

$$ds^2 = -\frac{\Delta}{\Sigma}\left(dt - a\sin^2\theta\,d\phi\right)^2 + \frac{\Sigma}{\Delta}dr^2 + \Sigma\,d\theta^2 + \frac{\sin^2\theta}{\Sigma}\left[(r^2+a^2)d\phi - a\,dt\right]^2$$

where $\Sigma = r^2 + a^2\cos^2\theta$ and $\Delta = r^2 - r_s r + a^2 + r_Q^2$.

**The no-hair theorem**: The Kerr-Newman family is the unique stationary, axisymmetric solution to the Einstein-Maxwell equations. Any uncharged, rotating black hole settled into equilibrium must be Kerr. Three parameters $(M, J, Q)$ completely describe a stationary black hole — all other information about the progenitor is radiated away or inaccessible. This remarkable result (proved by Israel, Carter, Hawking, Wald in the 1970s) is known as the no-hair theorem.

---

## Interior Solutions: Schwarzschild Interior

The Schwarzschild exterior metric $ds^2 = -(1-r_s/r)c^2dt^2 + (1-r_s/r)^{-1}dr^2 + r^2d\Omega^2$ applies only in vacuum. Inside a star, the Einstein equations are sourced by the stellar matter.

For a uniform-density star ($\rho = \text{const}$) of radius $R$, the interior solution is the **Schwarzschild interior metric**:
$$ds^2 = -\left[\frac{3}{2}\sqrt{1-\frac{r_s}{R}} - \frac{1}{2}\sqrt{1-\frac{r_s r^2}{R^3}}\right]^2 c^2dt^2 + \frac{dr^2}{1 - r_s r^2/R^3} + r^2d\Omega^2$$

This is matched to the Schwarzschild exterior at $r = R$ using the Israel junction conditions (continuity of induced metric and extrinsic curvature across the surface).

**Buchdahl's theorem**: There is a maximum compactness for any static spherically symmetric star:
$$\frac{2GM}{Rc^2} \leq \frac{8}{9} \implies R \geq \frac{9}{4}r_s$$

If a star is compressed below this limit, the central pressure becomes infinite. Any object more compact than $R = 9r_s/8$ must be a black hole. This result holds for any equation of state with $dp/d\rho \geq 0$.

---

## De Sitter and Anti-de Sitter Spacetimes

**De Sitter spacetime (dS)**: The maximally symmetric solution with positive cosmological constant $\Lambda > 0$ and no matter. It is the solution to $G_{\mu\nu} + \Lambda g_{\mu\nu} = 0$. In static coordinates:
$$ds^2 = -\left(1 - \frac{r^2}{\ell^2}\right)c^2dt^2 + \left(1 - \frac{r^2}{\ell^2}\right)^{-1}dr^2 + r^2d\Omega^2$$

where $\ell = \sqrt{3/\Lambda}$ is the de Sitter radius. At $r = \ell$: a cosmological horizon. The de Sitter temperature (Gibbons-Hawking) is $T_{\rm dS} = \hbar c/(2\pi k_B\ell)$.

In flat (inflationary) coordinates:
$$ds^2 = -c^2dt^2 + e^{2Ht}(dx^2+dy^2+dz^2), \quad H = c/\ell = c\sqrt{\Lambda/3}$$

This is the FLRW metric for a $\Lambda$-dominated universe — exponential expansion, exactly the de Sitter geometry. Our universe asymptotically approaches de Sitter as dark energy dominates.

**Anti-de Sitter spacetime (AdS)**: Negative cosmological constant $\Lambda < 0$. In static coordinates:
$$ds^2 = -\left(1 + \frac{r^2}{\ell^2}\right)c^2dt^2 + \left(1 + \frac{r^2}{\ell^2}\right)^{-1}dr^2 + r^2d\Omega^2$$

AdS has a timelike boundary at $r\to\infty$ (unlike dS which has a cosmological horizon). This boundary plays a crucial role in AdS/CFT — the holographic dual lives on this boundary.

AdS has negative curvature, and massive particles can orbit the center: AdS is a "gravitational potential well" — gravity gets stronger at large distances. This is why strings and branes in AdS can form stable bound states (unlike flat spacetime).

---

## Gravitational Wave Exact Solutions: pp-Waves and Plane Waves

**pp-waves** (plane-fronted gravitational waves with parallel rays): A family of exact solutions with metric:
$$ds^2 = -2du\,dv + H(u, x, y)du^2 + dx^2 + dy^2$$

where $u = t - z/c$ (retarded time) and $v = t + z/c$ (advanced time) are null coordinates. The vacuum Einstein equations require $H$ to satisfy:
$$\frac{\partial^2 H}{\partial x^2} + \frac{\partial^2 H}{\partial y^2} = 0$$

(Laplace equation in the transverse plane). For a monochromatic plane wave in the $+$-polarization:
$$H(u, x, y) = f(u)(x^2 - y^2)$$

for some profile function $f(u)$. This is the exact nonlinear gravitational plane wave — valid for arbitrarily large amplitudes, not just the linearized approximation.

**Sandwich waves**: A burst of gravitational radiation with $f(u) = 0$ outside $u\in(u_1, u_2)$ and nonzero inside. Test particles are displaced by the wave passage (the "memory effect").

**pp-waves and string theory**: pp-wave spacetimes are maximally supersymmetric backgrounds in string theory. The Green-Schwarz superstring is exactly solvable in pp-wave backgrounds (Berenstein-Maldacena-Nastase 2002).

---

## The Einstein-Rosen Bridge and Wormholes

The Kruskal extension of Schwarzschild reveals an Einstein-Rosen bridge connecting two exterior regions. But this "wormhole" is not traversable — it pinches off before any signal can cross.

**Morris-Thorne wormholes** (1988): Static, spherically symmetric, traversable wormhole metric:
$$ds^2 = -e^{2\Phi(r)}c^2dt^2 + \frac{dr^2}{1-b(r)/r} + r^2d\Omega^2$$

where $b(r)$ is the "shape function" (throat at $b(r_0) = r_0$) and $\Phi(r)$ is the redshift function. For traversability, one needs $b'(r_0) < 1$ (throat is stable) and $\Phi$ finite (no horizons).

**Exotic matter requirement**: The Einstein equations applied to this metric require $T_{\mu\nu}$ to violate the null energy condition (NEC) — $T_{\mu\nu}n^\mu n^\nu < 0$ for some null vector $n^\mu$. All known classical matter obeys the NEC. But the Casimir effect between parallel plates involves negative energy density, and quantum fields can violate the NEC in semiclassical GR.

Whether traversable wormholes can exist with realistic quantum matter remains an open question. The Chronology Protection Conjecture (Hawking) and Topological Censorship Theorem (Friedman-Schleich-Witt) suggest that if wormholes form, they are unstable and quickly collapse.

---

## Exercises

**43.1.** *Reissner-Nordström horizons.*

(a) For $r_s = 2$ (in geometric units where $G = c = 1$) and $r_Q = 0.5$, find the horizon radii $r_\pm$.

(b) At what value of $Q$ does the RN black hole become extremal? What fraction of $M$ is this in SI units?

(c) Show that the temperature of the RN black hole is:
$$T_{\rm RN} = \frac{\hbar c}{4\pi k_B r_+}\left(1 - \frac{r_Q^2}{r_+^2}\right)$$
Verify that $T_{\rm RN}\to 0$ as $r_+\to r_-$ (extremal limit).

---

**43.2.** *De Sitter horizon.*

(a) For the current universe ($\Lambda = 1.11\times 10^{-52}$ m$^{-2}$), compute the de Sitter radius $\ell = \sqrt{3/\Lambda}$ in light-years.

(b) In the far future when $\Lambda$ dominates, compute the de Sitter temperature $T_{\rm dS} = \hbar c/(2\pi k_B\ell)$. Compare to the current CMB temperature.

(c) Show that the de Sitter static metric is related to the FLRW flat metric $ds^2 = -c^2dt^2 + e^{2Ht}d\mathbf{x}^2$ by a coordinate transformation. (Hint: find the static observer's trajectory in FLRW coordinates.)

---

**43.3.** *Wormhole throat.*

For a Morris-Thorne wormhole with $b(r) = r_0^2/r$ (a specific shape function):

(a) Verify the throat is at $r = r_0$ where $b(r_0) = r_0$.

(b) Show $b'(r_0) = -1 < 1$ (flaring-out condition).

(c) Compute $T_{\mu\nu}$ (assuming $\Phi = 0$, no redshift) and check the NEC: $T_{\mu\nu}n^\mu n^\nu = ?$ for a null vector $n^\mu$. Is the NEC satisfied or violated?

---

**Thought Experiment T43.1.** *Is our universe de Sitter?*

Our universe, asymptotically dominated by dark energy, approaches de Sitter spacetime. De Sitter has a cosmological horizon — a boundary beyond which distant observers are forever inaccessible. Our current event horizon is at $\sim 16$ Gly.

In a de Sitter universe, all information about the past is eventually lost behind the cosmological horizon. In $\sim 150$ billion years, only the Local Group will remain visible. An observer in the far future will not be able to observe the CMB or measure the expansion history — the Big Bang will be unobservable from a de Sitter universe.

If the universe is truly de Sitter in the future, what does this imply for the long-term validity of the cosmological principle? For the measurability of the Hubble constant? For the testability of inflation? Is there a sense in which the universe "forgets" its history as it approaches de Sitter?

**Thought Experiment T43.2.** *Wormholes and time travel.*

A traversable wormhole whose two mouths are at different times (a Misner-Wheeler wormhole) would permit closed timelike curves — paths that return to their starting point in spacetime. These would allow time travel.

Hawking's Chronology Protection Conjecture argues that quantum effects always destroy traversable wormholes before they become time machines. The argument: radiation would loop through the wormhole indefinitely, amplifying the energy and collapsing the throat.

Is Chronology Protection a consequence of known physics, or an additional postulate? What observational test could distinguish "chronology protection is a law of physics" from "traversable wormholes simply don't form in nature"?
