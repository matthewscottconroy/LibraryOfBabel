# Chapter 53: Penrose Diagrams and Conformal Infinity

---

## Chapter Introduction

How do you draw a spacetime that extends to infinity? How do you visualize the global causal structure of a black hole, or of an expanding universe, or of Minkowski space itself? The answer is a **conformal diagram** — also called a Penrose diagram, or Carter-Penrose diagram.

The key idea is a **conformal compactification**: a coordinate transformation that maps the entire infinite spacetime into a finite region, while preserving the causal structure (the light cones). Because only the angles between null directions matter for causality, and conformal transformations preserve angles, the causal structure is faithfully represented even though the scale information is thrown away.

Penrose diagrams are not merely visual aids — they are precise mathematical tools. They reveal the global structure of spacetime: where singularities are, what the boundary ("conformal infinity") looks like, which regions can send signals to which other regions. The discovery that the Schwarzschild singularity is spacelike (not timelike) — a fact with profound implications for the inevitability of hitting the singularity — is most cleanly seen in the Penrose diagram. The existence of the white hole and the second asymptotic region in the Kruskal extension is invisible in Schwarzschild coordinates but immediately obvious in the Penrose diagram.

---

## Conformal Transformations

A **conformal transformation** of the metric:
$$\tilde{g}_{\mu\nu} = \Omega^2(x)\,g_{\mu\nu}$$

where $\Omega(x) > 0$ is a smooth function, preserves the causal structure: null geodesics of $g$ are null geodesics of $\tilde{g}$ (since $\tilde{g}_{\mu\nu}dx^\mu dx^\nu = 0$ iff $g_{\mu\nu}dx^\mu dx^\nu = 0$). The angles between curves are also preserved (hence "conformal").

The price: the geodesics of $\tilde{g}$ are not the same as those of $g$ (the proper length and proper time change), and the curvature changes. But for understanding light cones and causal relationships, only null geodesics matter.

**Conformal compactification**: Choose $\Omega$ to map an infinite spacetime to a finite region. The boundary of this finite region — the set of points at "$\Omega = 0$" — represents "infinity" and is called **conformal infinity** ($\mathscr{I}$, pronounced "scri").

---

## Minkowski Spacetime: The Prototype

In Minkowski spacetime $ds^2 = -c^2dt^2 + dr^2 + r^2d\Omega^2$ (in spherical coordinates), introduce null coordinates:
$$u = t - r/c, \quad v = t + r/c \quad (u \leq v, \text{ with } r = (v-u)c/2)$$

The metric: $ds^2 = -c^2\,du\,dv + r^2 d\Omega^2$.

**Compactify** by $U = \arctan(u/L)$, $V = \arctan(v/L)$ (any length scale $L$): both $U, V\in(-\pi/2, +\pi/2)$, with $V \geq U$.

The compactified metric (up to a conformal factor):
$$\tilde{g} = \frac{1}{\cos^2 U\cos^2 V}\left(-c^2\,dU\,dV + \sin^2\frac{V-U}{2}\,d\Omega^2\right) \propto -c^2\,dU\,dV + \cdots$$

In the new coordinates $T = V + U$, $\chi = V - U$ (both in $[0, \pi]$, $\chi \geq 0$):
$$\tilde{ds}^2 = -dT^2 + d\chi^2 + \sin^2\chi\,d\Omega^2 \quad (\text{after another conformal factor})$$

This is the Einstein static universe metric — Minkowski spacetime conformally embeds into the Einstein universe!

**The Penrose diagram** (suppressing the $S^2$ angles) is a square in the $(T,\chi)$ plane with corners at:

- $i^+$ (future timelike infinity): $T = \pi$, $\chi = 0$ — all timelike geodesics end here
- $i^-$ (past timelike infinity): $T = -\pi$, $\chi = 0$ — all timelike geodesics start here
- $i^0$ (spatial infinity): $T = 0$, $\chi = \pi$ — all spacelike geodesics end here
- $\mathscr{I}^+$ (future null infinity): $T + \chi = \pi$, $0 < \chi < \pi$ — all null geodesics end here
- $\mathscr{I}^-$ (past null infinity): $T - \chi = -\pi$, $0 < \chi < \pi$ — all null geodesics start here

The 45° lines in the diagram are null geodesics. Timelike worldlines go from $i^-$ to $i^+$. Massive particles cannot reach $\mathscr{I}^\pm$.

---

## Schwarzschild: Kruskal and Penrose

The Schwarzschild metric has a coordinate singularity at $r = r_s$. The maximal analytic extension (Kruskal-Szekeres coordinates) reveals the full global structure.

**Kruskal coordinates** $(T_K, X_K)$:

For $r > r_s$: 
$$T_K = \sqrt{\frac{r}{r_s}-1}\,e^{r/(2r_s)}\sinh\left(\frac{ct}{2r_s}\right), \quad X_K = \sqrt{\frac{r}{r_s}-1}\,e^{r/(2r_s)}\cosh\left(\frac{ct}{2r_s}\right)$$

For $r < r_s$ (inside the horizon):
$$T_K = \sqrt{1-\frac{r}{r_s}}\,e^{r/(2r_s)}\cosh\left(\frac{ct}{2r_s}\right), \quad X_K = \sqrt{1-\frac{r}{r_s}}\,e^{r/(2r_s)}\sinh\left(\frac{ct}{2r_s}\right)$$

The Kruskal metric: $ds^2 = \frac{4r_s^3}{r}e^{-r/r_s}(-c^2dT_K^2 + dX_K^2) + r^2d\Omega^2$.

The singularity $r = 0$ corresponds to the hyperbolae $T_K^2 - X_K^2 = e^{1} = e$ — it is **spacelike** (horizontal lines in the diagram).

The horizon $r = r_s$ corresponds to $T_K = \pm X_K$ — the null surfaces at $45°$.

**The four regions** of the maximally extended Schwarzschild:
1. **Region I** ($X_K > |T_K|$): The exterior Schwarzschild spacetime; our universe
2. **Region II** ($T_K > |X_K|$): The black hole interior; inevitable future of matter crossing the horizon
3. **Region III** ($T_K < -|X_K|$): The white hole interior; could have existed in the past
4. **Region IV** ($X_K < -|T_K|$): Another exterior — a second universe, causally disconnected from I

**The Penrose diagram** (conformal compactification of the Kruskal diagram) is a square with the singularity $r = 0$ as horizontal lines at top and bottom, the horizon as diagonal lines, and four regions as quadrants. Conformal infinity $\mathscr{I}^\pm$ appears on the left and right sides.

**What the diagram shows**:
- An observer in Region I who crosses the future horizon enters Region II and hits the spacelike singularity. The singularity is unavoidable — it is in the future, not "to the right."
- No signal from Region II can reach Region I — the horizon is a causal boundary.
- Regions I and IV are causally disconnected (no exchange of signals possible).

---

## de Sitter and Anti-de Sitter

**de Sitter space** (positive $\Lambda$): The maximally symmetric solution with $\Lambda > 0$:
$$ds^2 = -\left(1-\frac{r^2}{\ell^2}\right)c^2dt^2 + \frac{dr^2}{1-r^2/\ell^2} + r^2d\Omega^2, \quad \ell = \sqrt{3/\Lambda}$$

The Penrose diagram is a square: the top and bottom edges are spacelike surfaces ($\mathscr{I}^+$ and $\mathscr{I}^-$). There is a cosmological horizon — an observer at $r = 0$ cannot communicate with anyone beyond $r = \ell$ (similar to a black hole horizon, but observer-dependent).

De Sitter space is relevant for: the inflationary era (exponential expansion), the far future of our $\Lambda$-dominated universe.

**Anti-de Sitter space** (negative $\Lambda$): Maximally symmetric with $\Lambda < 0$:
$$ds^2 = -\left(1+\frac{r^2}{\ell^2}\right)c^2dt^2 + \frac{dr^2}{1+r^2/\ell^2} + r^2d\Omega^2$$

The Penrose diagram: a vertical strip with the timelike boundaries $\mathscr{I}$ on left and right — null rays can reach the boundary and return in finite coordinate time. AdS has a timelike conformal boundary, crucial for AdS/CFT.

---

## Conformal Infinity: the Penrose Classification

For any asymptotically flat spacetime, conformal infinity $\mathscr{I}$ has the following components (the structure of $\mathscr{I}$ encodes physical information about the spacetime):

- $\mathscr{I}^+$ (**future null infinity**, "scri-plus"): where outgoing null rays end; topology $\mathbb{R}\times S^2$
- $\mathscr{I}^-$ (**past null infinity**, "scri-minus"): where incoming null rays begin; topology $\mathbb{R}\times S^2$
- $i^+$ (future timelike infinity): where massive particles end
- $i^-$ (past timelike infinity): where massive particles begin
- $i^0$ (spatial infinity): where spacelike geodesics end

**BMS symmetry**: The asymptotic symmetry group of asymptotically flat spacetimes at $\mathscr{I}$ is not the Poincaré group but the larger **Bondi-van der Burg-Metzner-Sachs (BMS) group** — the Poincaré group extended by "supertranslations." The BMS group is infinite-dimensional. Its conserved charges include energy, momentum, and an infinite tower of "supermomentum" charges.

**Soft theorems and gravitational memory**: The BMS supertranslations are related to Weinberg's soft graviton theorem (the leading behavior of scattering amplitudes as a graviton momentum goes to zero) and to the gravitational memory effect (permanent displacement of test masses after a gravitational wave passes). These connections, elucidated by Strominger et al. in 2014–2016, suggest that BMS symmetry may be important for the black hole information paradox.

---

## FLRW Penrose Diagrams

For the radiation-dominated flat FLRW universe ($a(t)\propto t^{1/2}$), the Penrose diagram has:
- A big bang singularity ($a = 0$) as a spacelike bottom edge
- Future null infinity $\mathscr{I}^+$ as a top edge (or as a point in some models)

The **particle horizon** is visible: an observer at $t = t_0$ can only receive signals from events within their past light cone. In the Penrose diagram, the "initial singularity" is spacelike, so different regions of the sky were in causal contact with the observer but not with each other — the horizon problem made geometrically precise.

For an inflationary period followed by radiation domination, the Penrose diagram shows the initial light cone extended dramatically — inflation solves the horizon problem by expanding the causal past.

---

## Important Concepts

- **Conformal transformation**: $\tilde{g}_{\mu\nu} = \Omega^2 g_{\mu\nu}$; preserves null geodesics and causal structure
- **Conformal compactification**: Coordinate transformation mapping infinite spacetime to finite region
- **Penrose diagram**: 2D (suppressing spheres) depiction of the entire causal structure; light rays at 45°
- **Conformal infinity** $\mathscr{I}^\pm$: Where null geodesics begin/end; topology $\mathbb{R}\times S^2$ for asymptotically flat spacetimes
- **$i^\pm$, $i^0$**: Future/past timelike infinity, spatial infinity
- **Kruskal extension**: Maximal extension of Schwarzschild; reveals white hole, second exterior, spacelike singularity
- **Spacelike singularity**: In the Penrose diagram of Schwarzschild — lies in the *future* of all interior observers; cannot be avoided
- **de Sitter**: Compact Penrose diagram; cosmological horizon; relevant for inflation and late-time universe
- **Anti-de Sitter**: Timelike boundary at $\mathscr{I}$; AdS/CFT lives "on" this boundary
- **BMS symmetry**: Infinite-dimensional asymptotic symmetry at $\mathscr{I}^+$; related to soft theorems and memory effects

---

## Important Figures

**Roger Penrose** (1931–): Introduced conformal diagrams in 1963–1964; with them, proved the Penrose singularity theorem; Nobel Prize 2020.

**Brandon Carter** (1942–): Extended and systematized the use of conformal diagrams (Carter-Penrose diagrams); discovered the Carter constant for Kerr geodesics.

**Martin Kruskal** (1925–2006) and **George Szekeres** (1911–2005): Independently found the maximal extension of Schwarzschild spacetime in 1960.

**Hermann Bondi** (1919–2005), **Rainer Sachs** (1932–): Analyzed gravitational waves at null infinity; discovered the BMS group.

---

## Further Reading

**Primary Sources**
- Penrose, R. (1963). "Asymptotic Properties of Fields and Space-Times." *Phys. Rev. Lett.*, 10, 66.
- Kruskal, M.D. (1960). "Maximal Extension of Schwarzschild Metric." *Physical Review*, 119, 1743.
- Bondi, H., van der Burg, M.G.J., & Metzner, A.W.K. (1962). "Gravitational Waves in General Relativity." *Proc. Royal Society A*, 269, 21.

**Textbooks**
- Wald, R.M. (1984). *General Relativity*. Chicago. — Chapter 11 on global methods; Appendix D on conformal infinity.
- Carroll, S.M. (2004). *Spacetime and Geometry*. Addison-Wesley. — Section 9.4 on Penrose diagrams.
- Hawking, S.W. & Ellis, G.F.R. (1973). *The Large Scale Structure of Space-Time*. Cambridge. — The definitive reference for global methods.

---

## Exercises

**53.1.** *Penrose diagram for Minkowski.*

(a) Verify that the coordinate transformation $U = \arctan u$, $V = \arctan v$ (with $u = t - r$, $v = t + r$ in units $c = 1$) maps the Minkowski plane $(u,v)\in\mathbb{R}^2$ to the finite region $(U,V)\in(-\pi/2,\pi/2)^2$ with $V\geq U$.

(b) Draw the Penrose diagram. Mark $i^\pm$, $i^0$, $\mathscr{I}^\pm$. Show that a radially outgoing null ray from the origin $r = 0$ is a horizontal line at $U = 0$ in $(U,V)$ coordinates — and verify it reaches $\mathscr{I}^+$.

(c) Where does a massive particle on the worldline $r = \text{const}$ go in the Penrose diagram?

---

**53.2.** *Schwarzschild Penrose diagram.*

(a) In Kruskal coordinates, the singularity is at $T_K^2 - X_K^2 = e$ (a hyperbola). Verify this from the relation $T_K^2 - X_K^2 = -(r/r_s - 1)e^{r/r_s}$ and $r = 0$.

(b) An observer in Region I falls into the black hole. On the Penrose diagram, draw their worldline. What is the maximum proper time they can experience between crossing the horizon and hitting the singularity? (Use the Schwarzschild result: $\tau_{\rm max} = \pi r_s/c = \pi GM/c^3$.)

(c) Can the infalling observer send a signal to Region IV? Can they receive a signal from Region IV? Justify using the causal structure.

---

**53.3.** *de Sitter cosmological horizon.*

(a) Verify that in de Sitter space with $\ell = \sqrt{3/\Lambda}$, there is a horizon at $r = \ell$: the $(1-r^2/\ell^2)$ factor in $g_{tt}$ vanishes.

(b) An observer at $r = 0$ can only observe events within their past light cone. Draw the Penrose diagram for de Sitter and mark the cosmological horizon. What is the physical interpretation?

(c) The Hawking temperature of the de Sitter horizon is $T_{\rm dS} = \hbar c/(2\pi k_B\ell)$. For our universe's cosmological constant ($\Lambda \approx 10^{-52}$ m$^{-2}$), compute $T_{\rm dS}$. Compare to the CMB temperature $2.7$ K. Is this effect observable?

---

**Thought Experiment T53.1.** *The white hole and arrow of time.*

The maximal Kruskal extension of Schwarzschild includes a **white hole** (Region III) — a time-reverse of a black hole. Matter can escape from but not enter a white hole. By the time-reversal symmetry of GR (the Einstein equations are invariant under $t\to -t$), if black holes are solutions, so are white holes.

Yet we never observe white holes. Why? 

The answer involves the thermodynamic arrow of time: white holes are unstable — any perturbation converts them to black holes. More fundamentally, the Big Bang itself is an "initial singularity," and a white hole that formed from the Big Bang would immediately be surrounded by matter that converts it to a black hole.

Is the absence of white holes a statement about the laws of physics, or about the initial conditions of the universe? What would you need to change — the laws of GR, or the state of the universe — to produce a white hole?
