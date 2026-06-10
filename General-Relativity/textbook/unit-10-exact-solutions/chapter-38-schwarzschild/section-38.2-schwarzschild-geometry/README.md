# Section 38.2: The Schwarzschild Geometry

---

## What the Metric Tells Us

The Schwarzschild metric $ds^2 = -f(r)c^2dt^2 + f(r)^{-1}dr^2 + r^2d\Omega^2$ with $f(r) = 1 - r_s/r$ encodes rich geometry in its four metric components. Let us read them.

**Angular part:** $r^2d\Omega^2$ is the standard 2-sphere metric. The coordinate $r$ is defined as the *area-radius* — the radius inferred from surface area: $A = 4\pi r^2$. It is not the proper distance from the origin.

**Proper radial distance:** The spatial distance between two radial points at fixed $t,\theta,\phi$ is:
$$d\ell = \int_{r_1}^{r_2}\frac{dr}{\sqrt{1-r_s/r}} > r_2 - r_1$$
The spatial geometry is not Euclidean: the actual distance between two spherical shells is *larger* than the coordinate difference $r_2 - r_1$. This is positive radial curvature. The "embedding diagram" of the Schwarzschild spatial geometry shows a funnel shape — the familiar "gravity well" visualization.

**Proper time for a static observer:** An observer at rest at coordinate radius $r$ experiences proper time:
$$d\tau = \sqrt{f(r)}\,dt = \sqrt{1-r_s/r}\,dt$$
At large $r$: $d\tau\approx dt$ (Minkowski time). At $r = r_s$: $d\tau = 0$ — a static observer at the Schwarzschild radius is on a null surface (cannot be static). At $r < r_s$: $f < 0$ — the metric component $g_{tt} > 0$ — meaning $t$ is spacelike and $r$ is timelike inside $r_s$. Dramatic consequences follow.

**The coordinate singularity at $r = r_s$:** The component $g_{rr} = 1/f(r)$ diverges at $r = r_s$. But the Kretschner scalar $R_{\mu\nu\rho\sigma}R^{\mu\nu\rho\sigma} = 48G^2M^2/(r^6 c^4)$ is perfectly finite there. This singularity is a *coordinate artifact* — a bad choice of coordinates, analogous to the divergence of polar coordinates at the origin of $\mathbb{R}^2$.

**The real singularity at $r = 0$:** The Kretschner scalar diverges as $r\to 0$. This is a genuine curvature singularity — no change of coordinates can remove it. All timelike and null geodesics inside the horizon terminate here in finite proper time.

---

## The Horizon and Its Nature

At $r = r_s = 2GM/c^2$, something remarkable happens. Let a light ray travel radially outward ($d\theta = d\phi = 0$, $ds^2 = 0$):
$$0 = -f(r)c^2dt^2 + f(r)^{-1}dr^2 \implies \frac{dr}{dt} = \pm cf(r) = \pm c\left(1-\frac{r_s}{r}\right)$$

At $r \gg r_s$: $dr/dt \approx \pm c$ — photons travel at speed $c$ in the coordinate sense. As $r\to r_s$: $dr/dt\to 0$ — outgoing light slows to zero in coordinate speed. An outgoing photon emitted from just above $r = r_s$ takes infinite coordinate time $t$ to reach a distant observer.

This does not mean the photon is stuck — in proper time or proper distance, nothing special happens at $r = r_s$ (for a large enough black hole). The slowing is a coordinate effect: the coordinate $t$ is the time measured by a distant static observer, and that observer sees the infalling object approach $r = r_s$ asymptotically, never crossing.

From the *infalling* observer's perspective: crossing $r = r_s$ takes finite proper time. The infalling observer notices nothing locally special at the horizon (if the black hole is large enough that tidal forces are small there).

**The horizon is a null surface:** At $r = r_s$, the normal to the surface $r = \text{const}$ is the 4-vector $n^\mu = g^{\mu r} = (0, g^{rr}, 0, 0) = (0, f(r), 0, 0)$. At $r = r_s$: $g^{rr} = f(r) = 0$, so $n^\mu = 0$ in the sense that the surface has zero "thickness" — it is a null surface. It is future-directed null: once inside, you cannot get back out.

**One-way membrane:** The horizon is a one-way surface: signals can cross from outside to inside, but not from inside to outside. This is not a consequence of infinite curvature (the curvature is finite at the horizon) but of the global causal structure of the spacetime.

---

## What Happens Inside: $r < r_s$

For $r < r_s$: $f(r) = 1 - r_s/r < 0$. Look at the metric components:
- $g_{tt} = -f = |f| > 0$: the $t$-coordinate is *spacelike*
- $g_{rr} = 1/f = -1/|f| < 0$: the $r$-coordinate is *timelike*

The roles of time and space have exchanged. The $r$ coordinate is now timelike — it is inevitably decreasing for any future-directed worldline. You cannot stop moving toward smaller $r$ any more than you can stop aging. The singularity at $r = 0$ is not a "place in space" but a "moment in time" — it lies in the future of every event inside the horizon.

This is geometrically profound. In Minkowski space, the future is defined by the interior of the forward light cone. In Schwarzschild, inside the horizon, the forward light cone is tilted inward — it points toward smaller $r$. All of the future is toward $r = 0$.

A freely falling observer (radial, starting from rest at $r = r_0 \gg r_s$) crosses the horizon in proper time $\tau \sim \pi r_s/(2c)$ after crossing (for a particle falling from infinity: $\tau_{\rm horizon\to singularity} = \pi GM/c^3 \approx 15\,\mu$s for $M = 1 M_\odot$, $\sim 3$ hours for $M = 10^9 M_\odot$).

---

## Null and Timelike Structure Near the Schwarzschild Radius

A useful way to visualize the causal structure is the **ingoing Eddington-Finkelstein (EF) coordinates**. Define:
$$v = t + r^* + \text{const}, \quad r^* = r + r_s\ln|r/r_s - 1|$$
where $r^*$ is the **tortoise coordinate** (so named because it takes infinite coordinate time to reach the horizon, like Zeno's tortoise). In $(v, r, \theta, \phi)$ coordinates:
$$ds^2 = -f(r)c^2dv^2 + 2c\,dv\,dr + r^2d\Omega^2$$

This metric is regular at $r = r_s$. The metric component $g_{vv} = -f(r)$ still vanishes at $r = r_s$ but $g_{vr} = c$ ensures the full metric is non-degenerate.

In these coordinates:
- Ingoing null rays: $v = \text{const}$ (null rays moving inward — radial $v =$ const is a straight line on the $(v,r)$ diagram)
- Outgoing null rays: $dv/dr = 2/(fc)$. For $r > r_s$: $dv/dr > 0$ (moves outward). For $r < r_s$: $f < 0$ so $dv/dr < 0$ (also moves inward — outgoing light is dragged inward).

The horizon ($r = r_s$) is the null surface where outgoing light is trapped — it neither escapes nor falls in.

---

## The Tortoise Coordinate and the Infinite Redshift Surface

The proper time for a static observer at radius $r$ to receive light from the infalling observer at $r = r_s + \epsilon$ is infinite — the infalling signal takes infinite coordinate time to reach the horizon. But the *redshift* of the infalling signal diverges exponentially:

$$z + 1 = \frac{f_{\rm emit}}{f_{\rm obs}} \approx e^{c_s t/4GM}$$

as the proper time $t$ of the distant observer increases, where $c_s = $ appropriate coefficient. The infalling object is seen by the distant observer to freeze at $r = r_s$, dimming exponentially with a characteristic timescale:
$$t_e = 4GM/c^3 = 2r_s/c$$

For a solar-mass black hole: $t_e \approx 20\,\mu$s. A collapsing star, seen from far away, would fade below any detectability within milliseconds (for a stellar black hole).

This is why black holes are black: the image of infalling matter is exponentially redshifted to zero by the time any significant fraction of it has crossed the horizon, from the perspective of a distant observer.

---

## Embedding Diagrams and Spatial Visualization

The spatial metric (fixed $t$) in the equatorial plane ($\theta = \pi/2$) is:
$$d\ell^2 = \frac{dr^2}{1-r_s/r} + r^2d\phi^2$$

This is the metric of a surface in flat 3D space. The embedding satisfies $dz/dr = \sqrt{r_s/(r-r_s)}$, giving:
$$z = 2\sqrt{r_s(r-r_s)}$$

This "Flamm's paraboloid" is the classic funnel visualization of the Schwarzschild spatial geometry. At $r = r_s$, the funnel is vertical — the surface is maximally curved. Outside, the funnel curves smoothly to flat.

Note: the embedding diagram shows the *spatial* geometry, not the full *spacetime* geometry (which includes time). The causal structure — the light cones, the event horizon — is not visible in the spatial embedding. For the full causal picture, one needs the Penrose diagram (Section 38.3).

