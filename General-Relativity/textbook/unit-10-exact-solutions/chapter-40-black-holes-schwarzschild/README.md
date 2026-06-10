# Chapter 40: Black Holes — The Schwarzschild Solution

---

## Chapter Introduction

The Schwarzschild solution, derived in December 1915 just weeks after Einstein published his field equations, describes the simplest possible black hole: a point mass surrounded by empty, spherically symmetric spacetime. For decades, the solution's most exotic feature — the Schwarzschild radius $r_s = 2GM/c^2$ — was regarded as a mathematical curiosity, a coordinate artifact with no physical meaning. Real stars, surely, would not collapse to this extreme state.

The theoretical resolution came in stages: Eddington (1924) and Lemaître (1933) showed the coordinate singularity at $r = r_s$ could be removed. Oppenheimer and Snyder (1939) showed that a massive enough star must collapse below $r_s$. Finkelstein (1958) interpreted $r = r_s$ as a one-way membrane — an **event horizon**. Kruskal (1960) found the maximally extended solution. And Wheeler gave the compact object its memorable name: a **black hole** (1967).

Black holes are now known to be real. The Event Horizon Telescope imaged the shadow of M87* in 2019 and Sgr A* in 2022. Gravitational wave detectors have observed dozens of binary black hole mergers. Stellar-mass black holes are abundant in X-ray binaries. Supermassive black holes — $10^6$ to $10^{10}M_\odot$ — inhabit the centers of almost every large galaxy. Black holes are not theoretical curiosities; they are among the most common massive objects in the universe.

---

## The Schwarzschild Metric

The unique spherically symmetric vacuum solution to Einstein's equations (Birkhoff's theorem):

$$ds^2 = -\left(1-\frac{r_s}{r}\right)c^2dt^2 + \frac{dr^2}{1-r_s/r} + r^2d\Omega^2$$

where $r_s = 2GM/c^2$ is the **Schwarzschild radius** and $d\Omega^2 = d\theta^2 + \sin^2\theta\,d\phi^2$.

For the Sun: $r_s^\odot = 2GM_\odot/c^2 \approx 3$ km (compared to $R_\odot = 696{,}000$ km).
For Earth: $r_s^\oplus \approx 9$ mm (compared to $R_\oplus = 6371$ km).

**Birkhoff's theorem**: Any spherically symmetric vacuum solution to Einstein's equations is the Schwarzschild metric (for $r > 0$). This is the GR analogue of Newton's shell theorem: the gravitational field outside a spherically symmetric object depends only on the total mass.

**Physical significance**: Schwarzschild coordinates $(t, r)$ are adapted to an observer at rest at infinity. The coordinate time $t$ measures time for a distant static observer. The radial coordinate $r$ is not a "distance from center" but is defined geometrically: a sphere at radial coordinate $r$ has area $4\pi r^2$.

---

## The Event Horizon

At $r = r_s$: $g_{tt} = 0$ and $g_{rr}\to\infty$. This is a **coordinate singularity** — it can be removed by a change of coordinates (like the coordinate singularity at the north pole of a sphere).

**Eddington-Finkelstein coordinates** (ingoing): Replace $t$ with $v = t + r^*/c$ where $r^* = r + r_s\ln|r/r_s - 1|$ is the tortoise coordinate. The metric becomes:

$$ds^2 = -\left(1-\frac{r_s}{r}\right)c^2dv^2 + 2c\,dv\,dr + r^2d\Omega^2$$

This is regular at $r = r_s$. The coordinate $v$ is constant along ingoing null rays.

**Physical interpretation of $r = r_s$**: In Eddington-Finkelstein coordinates, the outgoing null rays at $r = r_s$ satisfy $dr/dv = 0$ — they are "frozen" at $r = r_s$ and cannot escape. This is the **event horizon**: a one-way membrane. Once inside ($r < r_s$), all future-directed causal paths lead to $r = 0$. No signal can escape.

**Not a physical singularity**: All curvature invariants (e.g., the Kretschner scalar $K = 48G^2M^2/(c^4r^6)$) are finite at $r = r_s$. An infalling observer crosses the horizon without noticing anything local — the tidal forces at $r = r_s$ are $\sim GM/r_s^3 \propto 1/M^2$, becoming small for large black holes.

**The singularity** at $r = 0$: Here $K\to\infty$ — a genuine curvature singularity. All timelike geodesics inside the horizon reach $r = 0$ in finite proper time $\tau_{\rm max} = \pi GM/c^3$ (for a radially infalling observer starting from rest at $r = r_s$). The singularity is a moment in time (spacelike), not a place in space.

---

## The Kruskal-Szekeres Extension

The maximal analytic extension of Schwarzschild reveals four regions:

$$T^2 - X^2 = -(r/r_s - 1)e^{r/r_s}$$

The four regions:
- **Region I** ($X > |T|$): The exterior, $r > r_s$ — our universe
- **Region II** ($T > |X|$): The black hole interior, $r < r_s$ — the future singularity at $r = 0$ (where $T^2 - X^2 = e$)
- **Region III** ($T < -|X|$): The white hole interior — past singularity at $r = 0$
- **Region IV** ($X < -|T|$): A second exterior — causally disconnected from Region I

The **event horizon** is the null surface $T = X$ (future horizon, bounding Region I from Region II) and $T = -X$ (past horizon, bounding Region I from Region III).

**Physical relevance of Regions III and IV**: A black hole formed by gravitational collapse does not have Regions III and IV — those regions correspond to the portion of the Schwarzschild geometry that was "inside" the collapsing star before collapse. The maximal Kruskal extension applies only to the "eternal" Schwarzschild black hole — a mathematical idealization.

---

## Gravitational Redshift

A photon emitted at radius $r_{\rm em}$ and received at radius $r_{\rm obs} > r_{\rm em}$ is redshifted:

$$\frac{\omega_{\rm obs}}{\omega_{\rm em}} = \sqrt{\frac{g_{tt}(r_{\rm em})}{g_{tt}(r_{\rm obs})}} = \sqrt{\frac{1-r_s/r_{\rm em}}{1-r_s/r_{\rm obs}}}$$

For $r_{\rm obs}\to\infty$: $\omega_\infty = \omega_{\rm em}\sqrt{1-r_s/r_{\rm em}}$.

As $r_{\rm em}\to r_s$: $\omega_\infty\to 0$. A photon emitted just outside the horizon is infinitely redshifted when received at infinity.

**Gravitational time dilation**: A clock at radius $r$ ticks at rate $\sqrt{1-r_s/r}$ relative to a clock at infinity. At $r = r_s$: the clock appears frozen from the outside.

**Experimental confirmation**: The Pound-Rebka experiment (1959) measured gravitational redshift over a 22.5 m height: $\Delta\nu/\nu = gh/c^2 \approx 2.5\times 10^{-15}$. GPS satellites must account for both gravitational blueshift ($+45\ \mu$s/day) and special relativistic time dilation ($-7\ \mu$s/day) for a net $+38\ \mu$s/day.

---

## Black Hole Thermodynamics

The **laws of black hole mechanics** (Bardeen-Carter-Hawking 1973):

| Law | Black Hole | Thermodynamics |
|---|---|---|
| Zeroth | $\kappa$ is constant on the horizon | $T$ is uniform in equilibrium |
| First | $\delta M = \kappa\delta A/(8\pi G) + \Omega_H\delta J + \Phi_H\delta Q$ | $dU = TdS + p\,dV + \mu\,dN$ |
| Second | $\delta A\geq 0$ | $\delta S\geq 0$ |
| Third | $\kappa = 0$ is unattainable | $T = 0$ is unattainable |

For Schwarzschild: $\kappa = c^4/(4GM)$ (surface gravity), $A = 4\pi r_s^2 = 16\pi G^2M^2/c^4$.

At first, these were purely classical mechanical analogies. Then Hawking (1974) showed the analogy is exact: black holes radiate thermally at the **Hawking temperature** $T_H = \hbar\kappa/(2\pi ck_B)$, and the entropy is $S_{\rm BH} = k_BA/(4\ell_P^2)$.

For Schwarzschild:
$$T_H = \frac{\hbar c^3}{8\pi G M k_B} \approx \frac{6\times 10^{-8}\ \text{K}}{M/M_\odot}$$

A solar-mass black hole has $T_H\sim 60$ nK — far colder than the CMB. Only microscopic black holes ($M < 10^{11}$ kg, $r_s < 10^{-16}$ m) have high enough $T_H$ to radiate appreciably.

---

## Forming Black Holes: Gravitational Collapse

**Oppenheimer-Snyder collapse** (1939): A homogeneous pressureless dust ball. The interior metric is FLRW; the exterior is Schwarzschild. The surface reaches $r = r_s$ in finite coordinate time (but infinite Schwarzschild $t$), and in finite proper time of the infalling observer ($\tau = \pi R_0/(2c)\sqrt{R_0/r_s}$ where $R_0$ is the initial radius).

**Buchdahl limit**: A star of radius $R$ and mass $M$ with $R < 9r_s/8 = 9GM/(4c^2)$ cannot be in hydrostatic equilibrium (pressure diverges at the center). If a star is compressed below this limit, collapse to a black hole is inevitable.

**Chandrasekhar limit** ($1.44M_\odot$): Maximum mass of a white dwarf supported by electron degeneracy pressure. Above this, the star collapses to a neutron star or black hole.

**Tolman-Oppenheimer-Volkoff (TOV) limit** ($\sim 2$–$3M_\odot$): Maximum mass of a neutron star supported by neutron degeneracy and nuclear forces. Above this, collapse to a black hole is inevitable.

---

## Observational Evidence

**Stellar-mass black holes** (M $\sim$ 5–100 $M_\odot$):
- X-ray binaries: accretion disk emission; compact mass exceeding TOV limit implies black hole
- GW detections (LIGO/Virgo): 90 events in GWTC-3; many confirmed BBH mergers with $M_f > 10M_\odot$
- GW150914: $m_1 = 36M_\odot$, $m_2 = 29M_\odot$, $M_f = 62M_\odot$

**Supermassive black holes** ($M \sim 10^6$–$10^{10}M_\odot$):
- Sgr A* (Milky Way center): $M = 4.15\times 10^6 M_\odot$, $r_s = 0.08$ AU — imaged by EHT 2022
- M87*: $M = 6.5\times 10^9 M_\odot$ — first EHT image 2019 ($r_s = 38$ AU, $r_s\approx 3.6$ light-days)
- Stellar orbits at galactic center (Ghez/Genzel, Nobel 2020): direct kinematic evidence for Sgr A*

**Intermediate black holes** ($10^2$–$10^6 M_\odot$): candidate signals in LIGO (GW190521: $m_1+m_2 \sim 85M_\odot$, $M_f \sim 142M_\odot$), ultraluminous X-ray sources.

---

## Important Concepts

- **Schwarzschild radius**: $r_s = 2GM/c^2$; defines the event horizon size
- **Birkhoff's theorem**: Unique spherically symmetric vacuum solution; exterior depends only on $M$
- **Event horizon**: One-way membrane at $r = r_s$; not a local singularity; observable only globally
- **Coordinate vs. physical singularity**: $r = r_s$ is coordinate; $r = 0$ is physical ($K\to\infty$)
- **Gravitational redshift**: $\omega_\infty = \omega_{\rm em}\sqrt{1-r_s/r_{\rm em}}$; clock slowing; Pound-Rebka
- **Kruskal extension**: Maximal solution; four regions (exterior, BH interior, WH interior, second exterior)
- **Hawking temperature**: $T_H = \hbar c^3/(8\pi GMk_B)$; quantum effect; tiny for astrophysical BHs
- **Gravitational collapse**: Buchdahl limit, Chandrasekhar/TOV limits; inevitability for massive enough stars
- **EHT and LIGO**: Direct observational evidence for astrophysical black holes

---

## Further Reading

**Primary Sources**
- Schwarzschild, K. (1916). "Über das Gravitationsfeld eines Massenpunktes nach der Einsteinschen Theorie." *Sitzungsberichte der Preußischen Akademie der Wissenschaften*, 189–196.
- Oppenheimer, J.R. & Snyder, H. (1939). "On Continued Gravitational Contraction." *Physical Review*, 56, 455.
- Finkelstein, D. (1958). "Past-Future Asymmetry of the Gravitational Field of a Point Particle." *Physical Review*, 110, 965.
- Kruskal, M.D. (1960). "Maximal Extension of Schwarzschild Metric." *Physical Review*, 119, 1743.
- Event Horizon Telescope Collaboration. (2019). "First M87 Event Horizon Telescope Results." *ApJL*, 875, L1.

**Textbooks**
- Wald, R.M. (1984). *General Relativity*. Chapters 6–12.
- Misner, C.W., Thorne, K.S., & Wheeler, J.A. (1973). *Gravitation*. Chapters 31–34.
- Chandrasekhar, S. (1983). *The Mathematical Theory of Black Holes*. Oxford.

---

## Exercises

**40.1.** *Schwarzschild metric properties.*

(a) Verify that the Schwarzschild metric satisfies the vacuum Einstein equations $R_{\mu\nu} = 0$ by computing the Ricci tensor. (You may use the Christoffel symbols from Chapter 38.)

(b) For an observer at rest at radius $r$: compute the proper acceleration (acceleration needed to remain stationary against gravity). Show it diverges as $r\to r_s$.

(c) Two clocks: one at $r = 10r_s$, one at $r = 2r_s$. If both tick at the same rate locally, what is the ratio of their tick rates as seen from infinity?

---

**40.2.** *Infalling observer.*

(a) A particle falls radially from rest at $r = \infty$. Using the geodesic equations and the effective potential, show the proper time to fall from $r = r_0$ to $r = 0$ is:
$$\tau = \frac{\pi r_s}{2c}\left(\frac{r_0}{r_s}\right)^{3/2}$$
(For $r_0 = 3r_s$: compute $\tau$.)

(b) In Schwarzschild $t$-coordinates, the infalling particle never reaches $r = r_s$. Show from the geodesic equation that $dr/dt\to 0$ as $r\to r_s$.

(c) Explain physically why the infalling observer crosses the horizon in finite proper time even though it takes infinite coordinate time.

---

**40.3.** *Black hole evaporation.*

(a) The Hawking temperature for a Schwarzschild black hole: $T_H = \hbar c^3/(8\pi GMk_B)$. For a black hole of mass $M$, the total luminosity (Stefan-Boltzmann, surface area $A = 4\pi r_s^2$): $L = \sigma T_H^4 A$. Show $L = f(G,M,c,\hbar)$ and that $L\propto M^{-2}$.

(b) From $dM/dt = -L/c^2$: integrate to find the evaporation time $t_{\rm evap} = 5120\pi G^2 M^3/(\hbar c^4)$.

(c) For $M = M_\odot$: compute $t_{\rm evap}$ in years. Compare to the age of the universe. For what mass $M$ does $t_{\rm evap}$ equal the Hubble time $\sim 14$ Gyr?

---

**Thought Experiment T40.1.** *What does it feel like to fall into a black hole?*

You are falling freely toward a Schwarzschild black hole of mass $M = 10M_\odot$ (radius $r_s = 30$ km). You start from rest at $r = 1000r_s$.

(a) Estimate the proper time for you to fall from $r = 10r_s$ to $r = 0$.

(b) At $r = r_s$: what tidal force do you feel across your body ($\delta r = 2$ m)? Compare to Earth's surface gravity.

(c) Your friend outside sees you apparently frozen at $r = r_s$ and infinitely redshifted. From your perspective: you cross the horizon and continue falling. How do you reconcile these two descriptions? Who is "right"? 

The answer involves the global causal structure: both descriptions are correct in their respective domains. The outside observer's description is valid outside the horizon; your description is valid on your worldline. There is no contradiction — just different coordinate charts covering different regions of the spacetime.
