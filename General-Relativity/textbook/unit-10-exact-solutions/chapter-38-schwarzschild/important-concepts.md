# Chapter 38: Important Concepts

---

**Schwarzschild Metric**
$ds^2 = -(1-r_s/r)c^2dt^2 + (1-r_s/r)^{-1}dr^2 + r^2d\Omega^2$: unique spherically symmetric, static, asymptotically flat vacuum solution of the Einstein equations. Derived from symmetry ansatz + Birkhoff's theorem. First exact solution of GR (Schwarzschild, 1916).

**Schwarzschild Radius**
$r_s = 2GM/c^2$: the characteristic length scale of the Schwarzschild metric. For the Sun: $r_s = 2.95$ km. For Earth: $r_s = 8.87$ mm. The event horizon of a Schwarzschild black hole is at $r = r_s$.

**Birkhoff's Theorem**
The unique spherically symmetric vacuum solution of the Einstein equations is Schwarzschild. Even a pulsating sphere has a static Schwarzschild exterior. Corollary: no monopole gravitational radiation.

**Area-Radius Coordinate**
The coordinate $r$ in the Schwarzschild metric is defined by $A = 4\pi r^2$ (area of coordinate spheres). It is not the proper radial distance from the origin. Proper radial distance is $\int dr/\sqrt{1-r_s/r} > r$.

**Coordinate Singularity at $r = r_s$**
The metric component $g_{rr} = (1-r_s/r)^{-1}$ diverges at $r = r_s$, but the Kretschner invariant $R_{\mu\nu\rho\sigma}R^{\mu\nu\rho\sigma} = 48G^2M^2/(r^6c^4)$ is finite. This singularity is a coordinate artifact, removable by Eddington-Finkelstein or Kruskal-Szekeres coordinates.

**True Singularity at $r = 0$**
The Kretschner scalar diverges as $r\to 0$. This is a genuine curvature singularity — tidal forces diverge. No change of coordinates can remove it. Every timelike and null geodesic inside the horizon terminates here in finite proper time.

**Tortoise Coordinate**
$r^* = r + r_s\ln|r/r_s - 1|$: a coordinate that maps $r\in(r_s,\infty)$ to $r^*\in(-\infty,\infty)$. Makes the wave equation in Schwarzschild separable (Regge-Wheeler equation). Name: even light "crawls like a tortoise" to reach the horizon in $r^*$ time.

**Eddington-Finkelstein Coordinates**
$(v, r)$ where $v = ct + r^* = ct + r + r_s\ln|r/r_s-1|$: coordinates regular at the horizon. Used to describe ingoing (EF) or outgoing null geodesics continuously through $r = r_s$. Show that the horizon is a one-way membrane.

**Kruskal-Szekeres Coordinates and the Four Regions**
Maximal extension of Schwarzschild: Region I (exterior), Region II (black hole interior, future singularity), Region III (white hole interior, past singularity), Region IV (second exterior). All four regions are connected by the Einstein-Rosen bridge. Light cones always at $45°$ in the Kruskal diagram.

**Penrose Diagram (Conformal Diagram)**
Compactification of the Kruskal diagram to a finite square. Null infinity $\mathscr{I}^\pm$, timelike infinity $i^\pm$, spatial infinity $i^0$ are all visible. The event horizon is the $45°$ line separating Region I from II. Essential tool for understanding causal structure of spacetimes.

**Photon Sphere**
Unstable circular null geodesic at $r = 3GM/c^2 = 1.5r_s$. Photons on this orbit circle the black hole indefinitely (unstably). The photon sphere's image is the bright ring visible around a black hole shadow.

**Innermost Stable Circular Orbit (ISCO)**
The minimum radius for stable circular orbits of massive particles: $r_{\rm ISCO} = 6GM/c^2 = 3r_s$. Below this, orbits are unstable — small perturbations cause inspiral. The ISCO is the inner edge of the accretion disk for a Schwarzschild black hole. For Kerr (rotating), the ISCO depends on spin.

**Flamm's Paraboloid**
Embedding of the equatorial spatial slice of the Schwarzschild metric into flat 3D space: $z = 2\sqrt{r_s(r-r_s)}$ — the classic "gravity well" or funnel visualization. Shows the spatial (not spacetime) curvature.

