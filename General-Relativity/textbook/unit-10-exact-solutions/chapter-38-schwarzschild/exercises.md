# Chapter 38: Exercises

---

**38.1.** *Deriving the Schwarzschild metric.*

(a) For the static, spherically symmetric ansatz $ds^2 = -e^{2\alpha(r)}dt^2 + e^{2\beta(r)}dr^2 + r^2d\Omega^2$, compute the Christoffel symbols $\Gamma^t_{tr}$, $\Gamma^r_{tt}$, $\Gamma^r_{rr}$, $\Gamma^r_{\theta\theta}$, and $\Gamma^\theta_{r\theta}$.

(b) Compute the Ricci tensor components $R_{tt}$, $R_{rr}$, and $R_{\theta\theta}$.

(c) From $R_{tt} = R_{rr} = 0$, show $\alpha + \beta = \text{const}$, and use asymptotic flatness to conclude $\alpha = -\beta$.

(d) From $R_{\theta\theta} = 0$ with $\alpha = -\beta$, solve the ODE for $e^{2\alpha}$ and apply asymptotic flatness to get $e^{2\alpha} = 1 - r_s/r$.

(e) Verify the result by computing the Kretschner invariant $K = R_{\mu\nu\rho\sigma}R^{\mu\nu\rho\sigma}$ and showing it is $48M^2/r^6$ (in $G = c = 1$ units).

---

**38.2.** *Schwarzschild radii and physical significance.*

(a) Compute the Schwarzschild radius $r_s = 2GM/c^2$ for: (i) Earth, (ii) the Sun, (iii) a $10 M_\odot$ black hole, (iv) the Milky Way (mass $\sim 10^{12} M_\odot$), and (v) a proton.

(b) For the Sun, what fraction of the Sun's physical radius is its Schwarzschild radius? At the Solar surface, what is the ratio $r_s/r$?

(c) Compute the tidal force on a 2-meter-tall astronaut falling radially into a $10 M_\odot$ Schwarzschild black hole when they cross the horizon. Is the force survivable? (Use the tidal force $\Delta F = 4GMm\ell/(2r)^3$ where $\ell = 2$ m is the astronaut's height and $m = 70$ kg is their mass.) Repeat for M87* ($M = 6.5\times 10^9 M_\odot$).

(d) What is the proper time from the horizon to the singularity for a freely-falling observer in a $10 M_\odot$ and a $10^9 M_\odot$ Schwarzschild black hole? Use $\tau \approx \pi GM/c^3$.

---

**38.3.** *Effective potential and orbital dynamics.*

In natural units ($G = c = 1$), the effective potential for Schwarzschild geodesics is:
$$V_{\rm eff}(r) = \left(1-\frac{2M}{r}\right)\left(\epsilon + \frac{L^2}{r^2}\right)$$

(a) For massive particles ($\epsilon = 1$, $L = 4M$): Plot $V_{\rm eff}(r)$ for $r\in[2M, 20M]$. Find the local maximum and minimum by solving $dV_{\rm eff}/dr = 0$. Identify the unstable and stable circular orbit radii.

(b) Show that circular orbits exist for $r > 3M$ and the ISCO is at $r = 6M$ by finding the condition that the minimum disappears ($dV_{\rm eff}/dr = d^2V_{\rm eff}/dr^2 = 0$).

(c) For the ISCO: compute the orbital energy per unit mass $E_{\rm ISCO}$ and the orbital frequency $\Omega = d\phi/dt$. What fraction of the rest mass energy is radiated as the particle spirals in to the ISCO from infinity? Compare to the efficiency of burning hydrogen ($\sim 0.7\%$) and to the maximum Kerr efficiency ($\sim 42\%$ for a maximally rotating black hole).

(d) For a photon ($\epsilon = 0$) with impact parameter $b$: find the critical impact parameter $b_c$ for capture. Show it equals $3\sqrt{3}M \approx 5.196M$. For $b < b_c$, the photon is captured; for $b > b_c$, it deflects and escapes.

---

**38.4.** *Kruskal-Szekeres coordinates.*

(a) Show that the Kruskal coordinate transformation $(t,r)\to(T,X)$ gives the metric $ds^2 = \frac{32M^3}{r}e^{-r/(2M)}(-dT^2+dX^2) + r^2d\Omega^2$ (in natural units $G = c = 1$). Verify by computing $dT$ and $dX$ and substituting into the Schwarzschild metric.

(b) Show that $r = r_s$ corresponds to $X^2 - T^2 = 0$ (the horizon) and $r = 0$ corresponds to $X^2 - T^2 = -1$ (the singularity, a hyperbola in the future, $T > 0$).

(c) Verify that null geodesics in Kruskal coordinates are $45°$ lines ($T \pm X = \text{const}$). What does this mean for the causal structure?

(d) An infalling observer falls radially from $r_0 = 10M$ at $t = 0$. Using the Kruskal coordinates, trace their worldline from the exterior (Region I) through the horizon to the singularity. Approximately how much coordinate Kruskal time $T$ elapses between crossing the horizon and hitting the singularity?

---

## Thought Experiments

**T38.1.** *What does "crossing the event horizon" feel like?*

An astronaut falls freely into a stellar-mass black hole ($M = 10 M_\odot$). Describe their experience at three key moments: (1) far from the horizon, (2) as they approach the horizon, (3) after crossing the horizon. Address: Do they see infinite time pass on the distant stars? Do they feel anything special at the horizon? How long do they have before the singularity? What does the singularity feel like?

Now repeat for an astronaut falling into a supermassive black hole ($M = 10^9 M_\odot$). How does the experience differ? Why is the horizon crossing more gentle for a larger black hole?

**T38.2.** *The Schwarzschild singularity: physical reality or theory breakdown?*

The Schwarzschild singularity at $r = 0$ is where classical GR predicts infinite curvature and the complete breakdown of predictability. Most physicists believe this is not a physical singularity but a sign that classical GR breaks down — quantum gravity should take over at the Planck scale $\ell_P \sim 10^{-35}$ m and resolve the singularity.

Consider: what does it mean for a physical theory to "break down"? Is the singularity a physical prediction or a mathematical artifact? What observational consequences would we expect if the singularity is resolved by quantum gravity? Can we ever observe the resolution from outside the horizon?

---

## Laboratory Exercise: Visualizing Schwarzschild Geometry

**L38.1.** *Embedding diagrams and causal diagrams in Python.*

**Task 1 (Flamm's paraboloid):** Plot the embedding of the Schwarzschild equatorial spatial slice: $z = 2\sqrt{r_s(r-r_s)}$ for $r\in[r_s, 10r_s]$. Use `matplotlib` with a 3D surface plot. Show the funnel shape. Label the throat at $r = r_s$.

**Task 2 (Kruskal diagram):** Plot the Kruskal diagram for the Schwarzschild metric. Show:
- The four regions (I, II, III, IV) with different shading
- The horizons ($T = \pm X$)
- The singularities ($X^2 - T^2 = -1$, $T > 0$ for future and $T < 0$ for past)
- Five radial geodesics (2 infalling, 2 outgoing, 1 null at the horizon)
- The worldline of an infalling observer from $r = 10M$ to the singularity

**Task 3 (Penrose diagram):** Apply the transformation $\tilde{T} = \arctan(T+X)+\arctan(T-X)$, $\tilde{X} = \arctan(T+X)-\arctan(T-X)$ to the Kruskal diagram. Plot the resulting Penrose diagram showing null infinities $\mathscr{I}^\pm$, timelike infinities $i^\pm$, and spatial infinity $i^0$.

