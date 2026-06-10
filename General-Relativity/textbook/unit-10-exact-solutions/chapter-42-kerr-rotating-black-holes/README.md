# Chapter 42: Kerr Rotating Black Holes

---

## Chapter Introduction

The Schwarzschild solution describes a non-rotating black hole. But astrophysical black holes are born with angular momentum — from the rotational collapse of their progenitor stars or from mergers — and they retain it. The rotating generalization, found by Roy Kerr in 1963, is arguably the most important exact solution of the Einstein equations.

The discovery was unexpected and celebrated. After nearly 50 years of effort following Schwarzschild's 1916 solution, Kerr found the rotating vacuum solution in a few weeks using an inspired algebraic ansatz. Einstein had died in 1955, and Kerr presented his result to a general relativity conference in 1963 where Penrose and others were immediately struck by its significance. Chandrasekhar later called it "the most shattering experience of my life... the discovery that the Kerr metric is the exact solution for a rotating black hole... made me feel as if Nature had performed a miracle."

The Kerr solution is distinguished by:
- **Frame dragging**: The rotating black hole drags spacetime with it. Near the horizon, it is impossible to remain stationary — you must co-rotate with the black hole. This is the Lense-Thirring effect taken to its extreme limit.
- **The ergosphere**: A region outside the horizon where the time Killing vector becomes spacelike — observers cannot be at rest. Energy can be extracted from this region (Penrose process).
- **Kerr-Newman**: With charge $Q$ added, the Kerr-Newman solution ($M$, $J$, $Q$) is the most general stationary black hole (black hole no-hair theorem).
- **No inner boundary problem**: Unlike the Schwarzschild coordinate singularity at $r = r_s$, Kerr has genuine coordinate-free complexity: a ring singularity at $r = 0$ in the equatorial plane, and an inner horizon inside the outer horizon.

This chapter derives the Kerr metric, analyzes its structure, and explores the physics of rotating black holes.

---

## The Kerr Metric

In **Boyer-Lindquist coordinates** $(t, r, \theta, \phi)$:
$$ds^2 = -\left(1 - \frac{r_s r}{\Sigma}\right)c^2dt^2 - \frac{2r_s r a\sin^2\theta}{\Sigma}c\,dt\,d\phi + \frac{\Sigma}{\Delta}dr^2 + \Sigma\,d\theta^2 + \left(r^2 + a^2 + \frac{r_s r a^2\sin^2\theta}{\Sigma}\right)\sin^2\theta\,d\phi^2$$

where:
$$\Sigma = r^2 + a^2\cos^2\theta, \quad \Delta = r^2 - r_s r + a^2, \quad r_s = \frac{2GM}{c^2}, \quad a = \frac{J}{Mc}$$

Here $J$ is the angular momentum, $a$ is the specific angular momentum (Kerr parameter), and $r_s = 2GM/c^2$ is the Schwarzschild radius.

**Limits**:
- $a = 0$ (Schwarzschild): $\Sigma = r^2$, $\Delta = r^2 - r_s r$, and the metric reduces to Schwarzschild
- $M = 0$ (flat): Kerr in flat spacetime (oblate spheroidal coordinates)
- $\theta = \pi/2$ (equatorial plane): simplest case for geodesic analysis

**Symmetries**: The metric is stationary ($\partial_t g_{\mu\nu} = 0$, Killing vector $\xi_{(t)} = \partial_t$) and axisymmetric ($\partial_\phi g_{\mu\nu} = 0$, Killing vector $\xi_{(\phi)} = \partial_\phi$). The conserved quantities along geodesics are $E = -p_\mu\xi^\mu_{(t)}$ (energy per unit mass) and $L = p_\mu\xi^\mu_{(\phi)}$ (angular momentum per unit mass about the rotation axis).

---

## Horizons and the Ergosphere

**Horizons**: The horizons occur where $\Delta = 0$:
$$r_\pm = \frac{r_s}{2}\pm\sqrt{\left(\frac{r_s}{2}\right)^2 - a^2} = \frac{GM}{c^2}\pm\sqrt{\left(\frac{GM}{c^2}\right)^2 - a^2}$$

- $r_+$: **outer (event) horizon** — the black hole boundary
- $r_-$: **inner (Cauchy) horizon** — inside $r_+$

For $a > GM/c^2$: no real solution — **naked singularity** (cosmic censorship says this shouldn't form from physical initial data).

For **extremal Kerr**: $a = GM/c^2$, $r_+ = r_- = GM/c^2$.

**The ergosphere**: The region where the time Killing vector $\partial_t$ becomes spacelike:
$$g_{tt} = -\left(1 - \frac{r_s r}{\Sigma}\right)c^2 \geq 0 \implies r^2 - r_s r + a^2\cos^2\theta \leq 0$$

This gives the **static limit** $r_{\rm static} = (r_s/2) + \sqrt{(r_s/2)^2 - a^2\cos^2\theta}$.

The ergosphere is the region $r_+\leq r\leq r_{\rm static}$. At $\theta = \pi/2$ (equatorial plane): $r_{\rm static} = r_s = 2GM/c^2 > r_+$. At $\theta = 0, \pi$ (poles): $r_{\rm static} = r_+$. So the ergosphere is thickest at the equator.

Inside the ergosphere, $g_{tt} > 0$ — the time Killing vector is spacelike, and it is impossible for any observer to remain stationary (have $dr = d\theta = d\phi = 0$). All observers must rotate in the same direction as the black hole ("frame dragging").

---

## Frame Dragging and ZAMO Observers

The **zero-angular-momentum observer** (ZAMO) has $L = 0$ — no angular momentum — but is still dragged around the black hole at angular velocity:
$$\Omega_{\rm ZAMO} = \frac{d\phi}{dt}\bigg|_{L=0} = -\frac{g_{t\phi}}{g_{\phi\phi}} = \frac{r_s r a}{\Sigma(r^2+a^2) + r_s r a^2\sin^2\theta}$$

This is the **frame-dragging frequency**. At the outer horizon: $\Omega_H = ac/(r_+^2 + a^2)$ — all objects at the horizon must co-rotate at $\Omega_H$.

The Lense-Thirring effect (gravitomagnetic precession, from linearized GR) is the weak-field limit of frame dragging, measured by Gravity Probe B ($6606$ mas/yr geodetic precession, $39.2$ mas/yr Lense-Thirring).

At strong fields (near a Kerr black hole), frame dragging becomes extreme: the entire spacetime is dragged, and the ISCO and photon sphere shift dramatically from Schwarzschild values.

---

## Geodesics in Kerr: The Carter Constant

In Kerr, the geodesic equations are separable due to the hidden symmetry discovered by Brandon Carter (1968). There are four constants of motion along geodesics:
1. Normalization: $p^\mu p_\mu = -m^2c^2$ (mass, or $0$ for null)
2. Energy: $E = -p_\mu\xi^\mu_{(t)}$
3. Angular momentum: $L = p_\mu\xi^\mu_{(\phi)}$
4. **Carter constant**: $K = p_\mu K^{\mu\nu}p_\nu$ where $K^{\mu\nu}$ is the Killing tensor (a rank-2 symmetric tensor satisfying $\nabla_{(\mu}K_{\nu\rho)} = 0$)

The Carter constant $K$ corresponds to a "hidden" symmetry of the Kerr spacetime — the Killing tensor exists but there is no associated Killing vector. It separates the $r$ and $\theta$ motions.

The geodesic equations in Kerr:
$$\Sigma\frac{dr}{d\lambda} = \pm\sqrt{R(r)}, \quad \Sigma\frac{d\theta}{d\lambda} = \pm\sqrt{\Theta(\theta)}$$

where $R(r)$ and $\Theta(\theta)$ are specific polynomials in $r$ and $\cos\theta$ respectively. For equatorial geodesics ($\theta = \pi/2$, $K = L^2$), these reduce to:
$$\frac{dr}{d\lambda} = \pm\frac{1}{\Sigma}\sqrt{R(r)}, \quad R(r) = E^2(r^2+a^2)^2 - \Delta(m^2c^2 r^2 + L^2 + a^2 E^2 - 2aEL)$$

---

## The ISCO in Kerr

The innermost stable circular orbit (ISCO) is the closest stable orbit for massive particles. In Schwarzschild: $r_{\rm ISCO} = 6GM/c^2$. In Kerr, it depends on the spin:

For **prograde orbits** (co-rotating with the black hole):
$$r_{\rm ISCO}^{(\text{pro})} = \frac{GM}{c^2}\left(3 + Z_2 - \sqrt{(3-Z_1)(3+Z_1+2Z_2)}\right)$$

where $Z_1, Z_2$ are functions of $a/M$. For extremal prograde: $r_{\rm ISCO} = GM/c^2 = r_+$ — the ISCO shrinks to the horizon.

For **retrograde orbits**: $r_{\rm ISCO}$ increases, reaching $9GM/c^2$ for extremal retrograde.

The spin-dependence of the ISCO has major astrophysical consequences: **accretion efficiency**. Matter spiraling into a black hole radiates a fraction of its rest mass energy as it falls from infinity to the ISCO:
$$\eta_{\rm acc} = 1 - E_{\rm ISCO}/mc^2$$

For Schwarzschild: $\eta = 1 - \sqrt{8/9} \approx 5.7\%$. For extremal prograde Kerr: $\eta \approx 42\%$ — dramatically higher, explaining why spinning black holes in AGN can power the most luminous quasars.

---

## The Penrose Process: Extracting Energy from Kerr

Inside the ergosphere, the energy $E = -p_\mu\xi^\mu_{(t)}$ can be negative (because $\xi_{(t)}$ is spacelike there, and $p_\mu\xi^\mu_{(t)}$ can be positive or negative).

**The Penrose process**: A particle falls from infinity into the ergosphere with energy $E_0 > 0$. There it splits into two particles: one with $E_1 < 0$ (negative energy — possible only inside the ergosphere) and one with $E_2 = E_0 - E_1 > E_0$ (more energy than the infalling particle). The negative-energy particle falls into the black hole; the second escapes to infinity with more energy than the initial particle.

By the first law of BH thermodynamics, the black hole loses energy and angular momentum in this process. The energy extraction comes at the cost of slowing the black hole's rotation. The theoretical maximum efficiency is $1 - 1/\sqrt{2} \approx 29\%$ for near-extremal Kerr (the remainder goes to the escaping particle).

**Superradiance**: The wave analog of the Penrose process. Waves with frequency $\omega < m\Omega_H$ (where $m$ is the azimuthal quantum number) are amplified when scattered by a Kerr black hole — the reflected wave has more energy than the incident wave, at the cost of slowing the BH rotation.

**Blandford-Znajek mechanism**: In astrophysics, magnetic fields threading the ergosphere can extract rotational energy electromagnetically — this is believed to power relativistic jets from AGN and gamma-ray bursts. Kerr spin measurements (from X-ray spectroscopy and iron-line profiles) of AGN black holes show many are near-maximal ($a/M > 0.9$).

---

## Important Figures

**Roy Kerr (born 1934)**: New Zealand mathematician. Found the rotating black hole solution in 1963, presenting it at the First Texas Symposium on Relativistic Astrophysics. His derivation used an algebraic approach — requiring only the Kerr-Schild form of the metric — that was far simpler than any brute-force approach. Kerr's solution is the basis of essentially all astrophysical black hole physics.

**Brandon Carter (born 1942)**: Discovered the separability of geodesic equations in Kerr spacetime (1968) and the Carter constant. Also established the "no-hair" theorem for Kerr-Newman black holes and derived the laws of black hole mechanics with Bardeen and Hawking.

**Robert H. Boyer (1932–1966) and Richard W. Lindquist (1930–1966)**: Developed the Boyer-Lindquist coordinate system (1967, published posthumously; both died in an airplane crash in 1966), which is now the standard coordinate system for the Kerr metric.

---

## Exercises

**42.1.** *Limits of the Kerr metric.*

(a) Show that setting $a = 0$ in the Kerr metric with $\Sigma = r^2$, $\Delta = r^2 - r_s r$ reduces to the Schwarzschild metric.

(b) Show that in the limit $M\to 0$ with $a$ fixed, $\Delta = r^2 + a^2$ and the metric reduces to flat spacetime in oblate spheroidal coordinates with $x = \sqrt{r^2+a^2}\sin\theta\cos\phi$, $y = \sqrt{r^2+a^2}\sin\theta\sin\phi$, $z = r\cos\theta$.

(c) In the weak-field, slow-rotation limit ($r\gg r_s$, $a\ll r$), show the Kerr metric reduces to the gravitomagnetic PPN form, and identify the gravitomagnetic potential $\mathbf{h}$ in terms of the angular momentum $J$.

---

**42.2.** *The ergosphere and frame dragging.*

(a) Show that the static limit $r_{\rm static}(\theta) = (r_s/2) + \sqrt{(r_s/2)^2 - a^2\cos^2\theta}$ occurs where $g_{tt} = 0$.

(b) For a maximally rotating Kerr black hole ($a = GM/c^2$): compute $r_+$ and $r_{\rm static}(\pi/2)$. What is the extent of the equatorial ergosphere?

(c) Compute $\Omega_{\rm ZAMO}$ at $r = r_+$ (the horizon). Verify this equals $\Omega_H = ac/(r_+^2 + a^2)$.

(d) A stationary observer at $r = 1.5 GM/c^2$, $\theta = \pi/2$ in a maximally rotating Kerr spacetime: is this position inside or outside the ergosphere? Is it possible to be stationary there?

---

**42.3.** *ISCO and accretion efficiency.*

For a Kerr black hole with dimensionless spin $\chi = a/(GM/c^2) = J c/(G M^2)$:

(a) For $\chi = 0$ (Schwarzschild): verify $r_{\rm ISCO} = 6GM/c^2$ and $E_{\rm ISCO}/mc^2 = \sqrt{8/9} \approx 0.943$. The accretion efficiency is $\eta = 1 - E_{\rm ISCO}/mc^2 \approx 5.7\%$.

(b) For $\chi = 0.9$ (prograde): $r_{\rm ISCO} \approx 2.32 GM/c^2$ and $E_{\rm ISCO}/mc^2 \approx 0.794$. Compute $\eta$.

(c) For extremal prograde ($\chi = 1$): $r_{\rm ISCO} = r_+ = GM/c^2$ and $E_{\rm ISCO}/mc^2 = 1/\sqrt{3}$. Compute $\eta$. Compare to nuclear fusion ($\sim 0.7\%$) and matter-antimatter annihilation ($100\%$).

(d) Why does the efficiency increase so dramatically with spin? Give a physical argument in terms of the binding energy of the ISCO.

---

**Thought Experiment T42.1.** *No hair and the information content of a Kerr black hole.*

The no-hair theorem says a stationary black hole is completely described by three numbers: $M$, $J$, $Q$. All other information about what formed the black hole is lost (from the perspective of external observers).

A star with $M_\odot$ can have a complex magnetic field, rotation profile, chemical composition, flares, eruptions, and a magnetic field configuration. When it collapses to a black hole, all this information is (apparently) lost — the black hole just has $M$, $J$, $Q$.

Is information really lost in this process, or is it encoded in the quantum state of the black hole? How would you operationally distinguish "information is truly lost" from "information is encoded in an inaccessible form"? Is there an observational consequence?

**Thought Experiment T42.2.** *The inner horizon of Kerr.*

The Kerr metric has an inner (Cauchy) horizon at $r_-$ inside the outer horizon at $r_+$. Classical GR predicts that an infalling observer crosses $r_+$, then reaches $r_-$, and can emerge into a different asymptotic region — without hitting a singularity (the singularity is spacelike only in Schwarzschild; in Kerr, it is a timelike ring and can be avoided).

But the inner horizon is suspected to be unstable: infalling radiation is infinitely blueshifted at $r_-$, and this blue-shifted energy perturbs the geometry catastrophically. Strong cosmic censorship predicts the inner horizon is replaced by a singularity in realistic collapse.

If you could fall into a rotating black hole and survive crossing $r_+$ intact, what would happen at $r_-$? Would you be destroyed by the blueshift? Would you emerge in a different universe? Is there a sense in which the Kerr solution "inside $r_-$" is physically meaningful, or is it a mathematical artifact of the vacuum solution?
