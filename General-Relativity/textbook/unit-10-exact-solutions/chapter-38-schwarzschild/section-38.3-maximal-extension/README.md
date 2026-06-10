# Section 38.3: Maximal Extension and Kruskal Coordinates

---

## The Problem with Schwarzschild Coordinates

The Schwarzschild metric in $(t, r, \theta, \phi)$ coordinates covers only the region $r > r_s$ (the exterior of the black hole). The coordinates break down at $r = r_s$, even though the geometry there is smooth (the Kretschner scalar is finite). What does the spacetime look like at and inside $r = r_s$?

To answer this, we need coordinates that are regular at the horizon. We have already seen that ingoing Eddington-Finkelstein (EF) coordinates $(v, r, \theta, \phi)$ work. But these cover only two regions of the full spacetime: the exterior ($r > r_s$) and the interior of the black hole ($r < r_s$). There is more.

The **maximal analytic extension** of the Schwarzschild metric — found by Kruskal (1960) and Szekeres (1960) independently — reveals the complete structure.

---

## Kruskal-Szekeres Coordinates

Define new coordinates $(T, X)$ by:

For $r > r_s$ (exterior):
$$T = \sqrt{r/r_s - 1}\,e^{r/(2r_s)}\sinh(ct/2r_s)$$
$$X = \sqrt{r/r_s - 1}\,e^{r/(2r_s)}\cosh(ct/2r_s)$$

For $r < r_s$ (interior):
$$T = \sqrt{1 - r/r_s}\,e^{r/(2r_s)}\cosh(ct/2r_s)$$
$$X = \sqrt{1 - r/r_s}\,e^{r/(2r_s)}\sinh(ct/2r_s)$$

In Kruskal-Szekeres coordinates, the metric becomes:
$$ds^2 = \frac{4r_s^3}{r}e^{-r/r_s}\left(-dT^2 + dX^2\right) + r^2d\Omega^2$$

where $r = r(T,X)$ is defined implicitly by:
$$X^2 - T^2 = \left(\frac{r}{r_s} - 1\right)e^{r/r_s}$$

This metric is:
- **Regular everywhere except $r = 0$**: the prefactor $e^{-r/r_s}/r$ is smooth for all $r > 0$.
- **Conformally flat in the $(T,X)$ plane**: the factor $(-dT^2 + dX^2)$ is Minkowskian, so light cones are always at $45°$.
- **Four distinct regions**: the Kruskal diagram reveals the complete global structure.

---

## The Four Regions of the Kruskal Diagram

In the $(T, X)$ plane, the four regions are:

**Region I (Exterior, $r > r_s$, $|X| > |T|$, $X > 0$):** The ordinary exterior Schwarzschild spacetime. Schwarzschild coordinates $(t, r)$ cover this region: $t = \text{const}$ curves are straight lines through the origin; $r = \text{const}$ curves are hyperbolas.

**Region II (Black hole interior, $r < r_s$, $T > |X|$):** The interior of the black hole. The singularity $r = 0$ is the hyperbola $X^2 - T^2 = -1$ (in units $r_s = 1$) — a spacelike surface in the future. All timelike curves from Region I that cross the horizon end at the singularity. Once in Region II, $r$ decreases inevitably toward 0 (since $r$ is timelike inside the horizon and the future direction is toward smaller $r$).

**Region III (White hole interior, $T < -|X|$):** The time-reversal of the black hole interior. In Region III, matter *emerges* from the singularity (which is now in the past). This is the "white hole" — a region that can only be exited, never entered from outside. No physical process produces a white hole; it is a mathematical feature of the maximally extended vacuum solution.

**Region IV (Other exterior, $r > r_s$, $X < 0$):** A second exterior region — identical to Region I but connected to it only through the Einstein-Rosen bridge (see below). Causally disconnected from Region I: no signal can travel between them without passing through the interior of the black hole. A second asymptotically flat spacetime, inaccessible from our universe.

---

## The Event Horizons

The two event horizons are the lines $T = \pm X$ in the Kruskal diagram:

**Future horizon** ($T = X$, $X > 0$): The boundary of Region II. Photons emitted from just outside this surface take infinite coordinate time $t$ to escape to infinity (in Region I coordinates). Photons emitted from inside never escape. The future horizon is a null surface — light can cross it only in the future direction (from I to II or from IV to II).

**Past horizon** ($T = -X$, $X > 0$): The boundary of Region III. Light from the white hole interior crosses this surface to the exterior.

The full structure has two horizons forming an "X" in the Kruskal diagram. Region I is to the right, Region II is above, Region III is below, Region IV is to the left.

---

## The Einstein-Rosen Bridge

The spatial section $T = 0$ (fixing time) in the Kruskal diagram connects Regions I and IV through a "throat" — the minimum of the area function $A = 4\pi r^2$.

At $T = 0$: $r = r_s$ on the horizons, and $r$ increases away from the horizons toward the two asymptotically flat regions. The spatial slice forms a "wormhole" connecting two otherwise disconnected universes.

This is the **Einstein-Rosen bridge** (1935). Einstein and Rosen originally introduced it as a model for elementary particles (which turned out to be wrong). The bridge is non-traversable: any traveler attempting to cross from Region I to Region IV must pass through Region II (the black hole interior), where they are inevitably drawn toward the singularity before reaching Region IV. The bridge pinches off dynamically — it closes before light can cross it.

---

## The Penrose (Carter-Penrose) Diagram

While the Kruskal diagram is analytically exact, the regions extend to infinity in $T$ and $X$. A **Penrose diagram** (conformal compactification) brings infinity to a finite location by a further coordinate change that maps the infinite Kruskal plane to a finite diamond.

Define:
$$\tilde{T} = \arctan(T + X) + \arctan(T - X)$$
$$\tilde{X} = \arctan(T + X) - \arctan(T - X)$$

The resulting Penrose diagram is a square (rotated 45°) with:
- **Spacelike future singularity** $r = 0$ at the top (a horizontal line)
- **Future null infinity** $\mathscr{I}^+$ (outgoing null rays that escape to infinity)
- **Past null infinity** $\mathscr{I}^-$ (incoming null rays from infinity)
- **Spacelike past singularity** at the bottom (the white hole singularity)
- **Timelike infinities** $i^+$ (future), $i^-$ (past), $i^0$ (spatial) at the corners

The beauty of the Penrose diagram is that light cones are always at $45°$, so the causal structure is immediately visible. The event horizon divides the diagram: the black hole interior is the region from which no null geodesic can reach $\mathscr{I}^+$.

---

## Physical vs. Mathematical Black Holes

The Kruskal extension reveals that the maximally extended Schwarzschild spacetime has four regions. But this is a mathematical solution to the *vacuum* Einstein equations — no matter anywhere. A real black hole forms from the gravitational collapse of matter.

For a collapsing star, the spacetime is divided into an interior (with matter) and an exterior (Schwarzschild vacuum). The collapsing star uses up Region III (the white hole) and part of Region I and II. The resulting Penrose diagram for a realistic collapsing star has:
- No white hole region (Region III is replaced by the collapsing star's interior)
- No second exterior (Region IV is not present)
- The future singularity at $r = 0$ is still there
- The event horizon forms as the star collapses, starting from the center

The event horizon is not at a fixed location in spacetime — it grows as the star collapses, then settles to the final Schwarzschild radius $r_s = 2GM/c^2$ of the black hole.

---

## Geodesic Completeness of the Kruskal Extension

The original Schwarzschild coordinates were geodesically incomplete — radial null geodesics (infalling light) terminated at $r = r_s$ in finite affine parameter. The Kruskal extension resolves this: all geodesics that enter Region II continue to the true singularity at $r = 0$, where they terminate in finite proper time. The singularity at $r = 0$ is the genuine boundary of the manifold — it is *not* covered by the metric.

The Schwarzschild spacetime (maximally extended) is geodesically incomplete — this is the Penrose singularity theorem in action. No physically reasonable energy condition can avoid the singularity inside the horizon.

