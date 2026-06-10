# Section 19.1: Minkowski Spacetime

---

## Section Introduction

The central concept of special relativity is that space and time are not separate — they form a 4-dimensional continuum called **spacetime**, and the geometry of this continuum is described by the **Minkowski metric**. This is not merely a mathematical convenience; it reflects a profound truth about nature: the interval $ds^2 = -c^2 dt^2 + dx^2 + dy^2 + dz^2$ is frame-independent (an invariant), even though $dt$ and $d\mathbf{r}$ separately are not.

This section introduces Minkowski spacetime, the invariant interval, the causal structure (timelike/spacelike/null), and the light cone. These concepts are the foundation for everything in GR — the curved spacetime of GR is locally Minkowski (the equivalence principle), and all the global structure (event horizons, singularities, causal structure) is built on the causal structure introduced here.

---

## 19.1.1 Events, Spacetime, and the Interval

A **spacetime event** is a point in 4-dimensional spacetime: a location $(t, x, y, z)$ or equivalently $(x^0, x^1, x^2, x^3) = (ct, x, y, z)$ (using $x^0 = ct$ so all coordinates have dimensions of length). An event specifies both when and where something happens.

The **Minkowski metric** defines the spacetime interval between two events $(x^\mu)$ and $(x^\mu + dx^\mu)$:

$$ds^2 = \eta_{\mu\nu}\,dx^\mu\,dx^\nu = -c^2\,dt^2 + dx^2 + dy^2 + dz^2$$

where $\eta_{\mu\nu} = \text{diag}(-1, +1, +1, +1)$ (signature $(-+++)$).

The **invariant interval** $ds^2$ is the same in all inertial frames related by Lorentz transformations. This is the 4D analog of the Euclidean distance $d\ell^2 = dx^2 + dy^2 + dz^2$, with the crucial difference that $ds^2$ can be positive, negative, or zero.

**The sign of $ds^2$ classifies the separation**:

| $ds^2 < 0$ | **Timelike**: the two events can be connected by a massive particle (speed $< c$). There exists a frame where they occur at the same spatial location (but different times). $|ds| = c\,d\tau$ where $d\tau$ is the proper time. |
|---|---|
| $ds^2 = 0$ | **Null (lightlike)**: the two events can be connected by a light ray. No frame can bring them to the same location or the same time. |
| $ds^2 > 0$ | **Spacelike**: no signal (traveling at $\leq c$) can connect the two events. They are causally disconnected. There exists a frame where they occur at the same time (but different locations). |

**Physical interpretation**: The proper time $d\tau$ along a worldline is:

$$d\tau^2 = -\frac{ds^2}{c^2} = dt^2 - \frac{d\mathbf{r}^2}{c^2} = dt^2\left(1 - \frac{v^2}{c^2}\right)$$

A clock measures the proper time along its worldline. This is frame-independent: all observers agree on the reading of a clock at a given event, even if they disagree on the coordinate time elapsed.

---

## 19.1.2 The Light Cone

At any event $p$, the set of events connected to $p$ by null geodesics (light rays) forms the **light cone**:

$$ds^2 = 0 \quad \Leftrightarrow \quad c^2 dt^2 = dx^2 + dy^2 + dz^2$$

This divides all events into three regions relative to $p$:

- **Future light cone** ($dt > 0$, $ds^2 = 0$): events reachable from $p$ by light signals
- **Past light cone** ($dt < 0$, $ds^2 = 0$): events that can send light signals to $p$
- **Timelike future** ($ds^2 < 0$, $dt > 0$): events that can be reached from $p$ by a massive particle
- **Timelike past** ($ds^2 < 0$, $dt < 0$): events that could have causally influenced $p$
- **Spacelike region** ($ds^2 > 0$): causally disconnected from $p$ (no signal from $p$ reaches these events, and no signal from these events reaches $p$)

**Causal structure** is the heart of GR. In flat Minkowski spacetime, every event has the same light cone structure. In curved spacetime, the metric varies from event to event, and the light cone "tips" and "narrows" near strong gravitational fields. At a black hole's event horizon, the future light cone is tipped so severely that all future-directed timelike and null paths lead to the singularity — the horizon is the surface of no return.

**Spacetime diagrams**: Conventionally drawn with $ct$ vertical and $x$ horizontal. Light rays at $\pm 45°$. Massive particle worldlines have slopes $|d(ct)/dx| > 1$ (slope greater than $45°$). A particle at rest is a vertical line.

---

## 19.1.3 The Minkowski Metric in Index Notation

**Raising and lowering indices**: The metric $\eta_{\mu\nu}$ lowers indices:

$$V_\mu = \eta_{\mu\nu}V^\nu$$

For a 4-vector $V^\mu = (V^0, V^1, V^2, V^3) = (V^0, \mathbf{V})$:

$$V_\mu = (-V^0, V^1, V^2, V^3) = (-V^0, \mathbf{V})$$

The inverse metric $\eta^{\mu\nu} = \text{diag}(-1, +1, +1, +1)$ (same as $\eta_{\mu\nu}$) raises indices:

$$V^\mu = \eta^{\mu\nu}V_\nu$$

**Inner product**: The inner product of two 4-vectors is:

$$A\cdot B = \eta_{\mu\nu}A^\mu B^\nu = A_\mu B^\mu = -A^0 B^0 + \mathbf{A}\cdot\mathbf{B}$$

This is Lorentz-invariant (the same in all inertial frames).

**The 4-position**: $x^\mu = (ct, \mathbf{r})$. The invariant interval is $ds^2 = \eta_{\mu\nu}dx^\mu dx^\nu$.

**The norm**: $A^2 = \eta_{\mu\nu}A^\mu A^\nu = -(A^0)^2 + |\mathbf{A}|^2$. This can be negative (timelike), zero (null), or positive (spacelike).

---

## 19.1.4 Inertial Frames and the Poincaré Group

An **inertial frame** is a coordinate system $(t, x, y, z)$ in which free particles (with no forces) move in straight lines at constant speed. Newton's first law defines inertial frames; Newton's second law holds in them.

Two inertial frames related by the transformation:
$$x'^\mu = \Lambda^\mu_{\ \nu} x^\nu + a^\mu$$

where $a^\mu$ is a constant 4-translation and $\Lambda^\mu_{\ \nu}$ is a **Lorentz transformation** satisfying:

$$\eta_{\mu\nu}\Lambda^\mu_{\ \rho}\Lambda^\nu_{\ \sigma} = \eta_{\rho\sigma}$$

This condition says $\Lambda$ preserves the Minkowski inner product — it is an isometry of Minkowski spacetime.

The set of all such transformations forms the **Poincaré group** (also called the inhomogeneous Lorentz group). Its subgroups:
- **Lorentz group** $O(3,1)$: transformations with $a^\mu = 0$ (no translation)
- **Proper orthochronous Lorentz group** $SO^+(3,1)$: $\det\Lambda = +1$ (no parity flip), $\Lambda^0_{\ 0} > 0$ (no time reversal)
- **Translations** $\mathbb{R}^{3,1}$: $\Lambda^\mu_{\ \nu} = \delta^\mu_\nu$ (no rotation or boost)

The Poincaré group has 10 generators: 3 rotations ($J_i$), 3 boosts ($K_i$), 3 spatial translations ($P_i$), 1 time translation ($H = P_0$). By Noether's theorem, the corresponding conserved quantities are: angular momentum, boost momentum (center-of-energy), linear momentum, and energy. These are the 10 Poincaré conservation laws.

---

## 19.1.5 Proper Time and the Twin "Paradox"

The **proper time** elapsed along a worldline $x^\mu(\lambda)$ is:

$$\tau = \int_A^B \sqrt{-\frac{ds^2}{c^2}} = \int_A^B \sqrt{1 - v^2/c^2}\,dt$$

The proper time is frame-independent — it's a physical quantity, the reading of a co-moving clock.

**The twin paradox**: Alice stays at home (straight worldline in any inertial frame). Bob travels at high speed to a distant star and returns. When they reunite, Alice is older.

**Resolution**: Bob's worldline is not straight — it has a kink (the turnaround). The proper time elapsed is:

$$\tau_{\rm Bob} = \int \sqrt{1 - v^2/c^2}\,dt < \int dt = \tau_{\rm Alice}$$

(The integrand $\sqrt{1-v^2/c^2} \leq 1$, with equality only when $v = 0$.)

There is no paradox: the situation is not symmetric. Alice remains in a single inertial frame throughout; Bob does not. The asymmetry is the acceleration at the turnaround (Section 19.2.5).

**The deeper point**: In Minkowski geometry (as in Euclidean geometry), the *longest* path between two events (not shortest) is the straight path — a free particle maximizes proper time. This is the **principle of maximal aging**: a free particle extremizes (in fact maximizes) the proper time. In GR, this becomes the geodesic equation: freely falling particles follow worldlines of extremal proper time.

**Experimental confirmation**: Hafele and Keating (1972) flew atomic clocks around the world on commercial airliners and compared them to clocks on the ground. The relativistic time dilation (including gravitational effects) was confirmed to 10% accuracy. GPS satellites require relativistic corrections of order 38 microseconds per day (SR contribution $-7\,\mu$s/day from velocity; GR contribution $+45\,\mu$s/day from weaker gravity) — without these corrections, GPS would accumulate errors of several kilometers per day.

[Hafele, J.C. and Keating, R.E. (1972). "Around-the-world atomic clocks: predicted relativistic time gains." *Science*, 177, 166–170.]

---

## References

- Einstein, A. (1905). "Zur Elektrodynamik bewegter Körper." *Annalen der Physik*, 17, 891–921. [The founding paper of special relativity. Derives time dilation, length contraction, the Lorentz transformation, and the relativity of simultaneity from two postulates. No references, no acknowledgments.]
- Minkowski, H. (1908). "Raum und Zeit." Address to the 80th Assembly of German Natural Scientists, Cologne. Published in *Physikalische Zeitschrift*, 10 (1909), 104–111. [Introduces 4-dimensional spacetime and the Minkowski metric. The geometric reformulation of SR that Einstein initially found overly mathematical.]
- Hafele, J.C. and Keating, R.E. (1972). "Around-the-world atomic clocks: predicted relativistic time gains." *Science*, 177, 166–170; 168–170. [Experimental confirmation of special and general relativistic time dilation by flying cesium atomic clocks on commercial airliners.]
- Taylor, E.F. and Wheeler, J.A. (1992). *Spacetime Physics*, 2nd ed. W.H. Freeman. [The clearest introductory treatment of special relativity. Emphasizes the spacetime interval and the geometric viewpoint. An excellent first text.]
- Misner, C.W., Thorne, K.S., and Wheeler, J.A. (1973). *Gravitation.* W.H. Freeman. [Part I: "Special Relativity and Flat Spacetime." Chapters 1–6 develop SR in the geometric language needed for GR.]
