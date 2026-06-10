# Chapter 16 Exercises: Orbital Mechanics and Perihelion Precession

---

## Section 16.1: Central Force Motion

**16.1.1** *(Orbit classification)*

The effective potential for Newtonian gravity is $V_{\rm eff}(r) = -GMm/r + \ell^2/(2mr^2)$.

(a) Find the radius $r_0$ of circular orbits and verify it is a stable minimum of $V_{\rm eff}$.
(b) Find the angular frequency $\omega = \dot\phi$ and linear speed $v$ for a circular orbit at radius $r_0$.
(c) Compute the period $T$ of the circular orbit and verify Kepler's third law $T^2 \propto r_0^3$.
(d) Perturb the circular orbit: let $r = r_0 + \eta$ with $|\eta| \ll r_0$. Show that $\eta$ oscillates with frequency $\omega_r = \sqrt{V_{\rm eff}''(r_0)/m}$. Compute $\omega_r$ and verify $\omega_r = \omega$ (radial and azimuthal frequencies are equal for Newtonian gravity — this is why orbits close).

**16.1.2** *(The Schwarzschild effective potential)*

The Schwarzschild effective potential (in units $c = G = 1$) is:
$$V_{\rm eff}(r) = -\frac{M}{r} + \frac{\ell^2}{2r^2} - \frac{M\ell^2}{r^3}$$

(a) Find $dV_{\rm eff}/dr = 0$ and show it gives a quadratic equation in $r$. Find the two roots $r_\pm$.
(b) Show that for $\ell > 2\sqrt{3}M$, there are two roots (unstable and stable circular orbits), and for $\ell = 2\sqrt{3}M$, the two roots merge at $r = 6M$ (the ISCO).
(c) Show that for $\ell < 2\sqrt{3}M$, there are no circular orbits — the effective potential is monotonically decreasing for small $r$. A particle falls inward to the singularity.
(d) Compare $V_{\rm eff}^{\rm Schwarzschild}$ with $V_{\rm eff}^{\rm Newton}$ by plotting both for $\ell = 4M$. Identify the ISCO and describe qualitatively why GR destroys the stability that Newton predicts.

**16.1.3** *(Power-law force orbit stability)*

For a power-law force $F(r) = -k/r^n$ ($k, n > 0$), the effective potential is $V_{\rm eff}(r) = k/(r^{n-1}(n-1)) + \ell^2/(2mr^2)$ (assuming $n \neq 1$; for $n = 1$, $V = k\ln r$).

(a) Find circular orbits and compute $V_{\rm eff}''(r_0)$. Show stability requires $n < 3$.
(b) For stable circular orbits, the radial oscillation frequency is $\omega_r = \sqrt{(3-n)}\,\omega_\phi$. The orbit closes (forms a periodic Lissajous figure) if $\omega_r/\omega_\phi$ is rational. For which integer values of $n$ are orbits closed?
(c) Bertrand's theorem states that only $n = 2$ (inverse square) and $n = -1$ (harmonic oscillator, $V = kr^2/2$) give closed orbits for *all* energies and angular momenta. Verify this for $n = 3$ by computing the orbit (show it is not closed).

---

## Section 16.2: Kepler's Laws

**16.2.1** *(Computing orbital parameters)*

(a) A satellite is launched from Earth's surface ($R = 6.37\times10^6$ m) horizontally with speed $v_0$. Find the semi-major axis $a$ and eccentricity $e$ of the orbit in terms of $v_0$, $R$, and $g = GM/R^2$.
(b) The orbit is elliptical if $v_0 < v_{\rm esc}$. Find the perigee and apogee distances. At what $v_0$ does the orbit become circular? Hyperbolic?
(c) What is the minimum speed $v_{\rm min}$ at apogee for a satellite to maintain orbit (i.e., not re-enter the atmosphere)? Express in km/s.

**16.2.2** *(The vis-viva equation)*

(a) Verify the vis-viva equation $v^2 = GM(2/r - 1/a)$ using energy conservation and the expression $E = -GMm/(2a)$.
(b) A spacecraft is in a circular orbit at radius $r_1$. To transfer to a circular orbit at radius $r_2 > r_1$ (Hohmann transfer), it fires two burns: one at $r_1$ (raising apogee to $r_2$) and one at $r_2$ (circularizing). Compute the $\Delta v$ for each burn and the total $\Delta v_{\rm total}$. Show that the Hohmann transfer is the most fuel-efficient two-burn transfer.
(c) Apply (b) to a transfer from low Earth orbit (LEO, $r_1 = 6.57\times10^6$ m) to geostationary orbit (GEO, $r_2 = 4.22\times10^7$ m). Compute the total $\Delta v$ and the transfer time.

**16.2.3** *(The Laplace-Runge-Lenz vector)*

(a) Verify $\mathbf{A} = \mathbf{p} \times \mathbf{L} - GMm^2\hat{\mathbf{r}}$ is conserved: compute $d\mathbf{A}/dt$ using Newton's second law and $d\mathbf{L}/dt = 0$. Show it equals zero.
(b) Compute $\mathbf{A} \cdot \mathbf{r}$ and derive the orbit equation $r = p/(1 + e\cos\phi)$ where $e = |\mathbf{A}|/(GMm^2)$.
(c) For an orbit with $E = -GMm/(2a)$ and $|\mathbf{L}| = m\sqrt{GMa(1-e^2)}$, compute $|\mathbf{A}|$ and verify $e = \sqrt{1 + 2E\ell^2/(G^2M^2m^3)}$.
(d) In quantum mechanics, the Runge-Lenz operator $\hat{\mathbf{A}} = \frac{1}{2m}(\hat{\mathbf{p}}\times\hat{\mathbf{L}} - \hat{\mathbf{L}}\times\hat{\mathbf{p}}) - \frac{e^2}{r}\hat{\mathbf{r}}$ commutes with the hydrogen Hamiltonian $\hat{H}$. This gives an "accidental" degeneracy: states with the same principal quantum number $n$ but different orbital quantum number $\ell$ have the same energy. Why is this degeneracy not exact in real hydrogen? (Consider: what perturbations break the exact $1/r$ symmetry in real hydrogen?)

---

## Section 16.3: Perturbation Theory and GR Precession

**16.3.1** *(Deriving the GR precession)*

(a) Starting from Binet's equation with the GR perturbation:
$$u'' + u = \frac{GMm^2}{\ell^2} + \frac{3GM}{c^2}u^2$$
substitute $u = u_0(1 + e\cos\phi)$ with $u_0 = GMm^2/\ell^2$ on the right-hand side and expand $(1 + e\cos\phi)^2$.

(b) Identify the resonant term (the $\cos\phi$ term) in the expansion. What is its coefficient?

(c) Solve for the resonant correction $u_1 = A\phi\sin\phi$ and verify $u_1'' + u_1 = A\cos\phi$.

(d) Write $u = u_0[1 + e\cos\phi + \varepsilon e\phi\sin\phi]$ where $\varepsilon = 3G^2M^2m^2/(\ell^2 c^2)$. Use the approximation $\cos(\phi\sqrt{1-\varepsilon}) \approx \cos\phi + \varepsilon\phi\sin\phi/2$ (for small $\varepsilon$) to rewrite $u$ in terms of a precessing cosine. Show the perihelion advances by $\Delta\phi = 2\pi\varepsilon$ per orbit.

(e) Express $\Delta\phi$ in terms of $a$, $e$, $G$, $M$, $c$: $\Delta\phi = 6\pi GM/(c^2 a(1-e^2))$.

**16.3.2** *(Mercury's precession)*

Using Mercury's parameters: $a = 5.791\times10^{10}$ m, $e = 0.2056$, $T = 87.97$ days, $M_\odot = 1.989\times10^{30}$ kg.

(a) Compute $\Delta\phi$ per orbit (in radians).
(b) Convert to arcseconds per century.
(c) The perihelion contribution from each planet is proportional to $m_j a^{3/2}/a_j^3$ (approximately, for $a_j \gg a$) times numerical factors. Given that Venus contributes 277.9 arcsec/century, estimate Jupiter's contribution given its mass ($1.90\times10^{27}$ kg) and orbital radius ($7.78\times10^{11}$ m) vs. Venus's mass ($4.87\times10^{24}$ kg) and orbital radius ($1.08\times10^{11}$ m). Compare to the tabulated value of 153.6 arcsec/century. (Note: the full Lagrange-Laplace secular perturbation theory is needed for exact results; this is just an order-of-magnitude check.)

**16.3.3** *(Other precessing systems)*

The formula $\Delta\phi = 6\pi GM/(c^2 a(1-e^2))$ applies to any Schwarzschild geodesic.

(a) Binary pulsars: the Hulse-Taylor pulsar (PSR B1913+16) has two neutron stars with $a = 1.95\times10^9$ m (semi-major axis of relative orbit), $e = 0.617$, total mass $M_{\rm total} = 2.828 M_\odot$. Compute the GR precession rate in degrees per year. The observed value is 4.2266 deg/year. Compare.

(b) The double pulsar PSR J0737-3039 has $a = 8.79\times10^8$ m, $e = 0.0878$, $M_{\rm total} = 2.587 M_\odot$. Compute the precession rate and compare to the observed 16.9 deg/year.

(c) Both measured precession rates agree with GR to better than 0.05%. What does this precision test tell us about GR vs. alternative theories?

---

## Thought Experiments

**TE 16.1: Closed Orbits and Hidden Symmetry**
Bertrand's theorem says only two force laws ($F \propto r$ and $F \propto 1/r^2$) give closed orbits for all energies. This seems like a special property of these two laws, but it is actually a consequence of hidden symmetry: both correspond to dynamical systems with a symmetry larger than $SO(3)$.

(a) For the harmonic oscillator in 3D (force $\propto r$), the symmetry group is $U(3)$ (or $SU(3)$). What are the conserved quantities beyond $E$ and $\mathbf{L}$?
(b) For the Kepler problem (force $\propto 1/r^2$), the symmetry group is $SO(4)$ (for $E < 0$). The extra symmetry is the LRL vector. Why does extra symmetry → closed orbits?
(c) The hydrogen atom ($V = -e^2/r$, quantum version of the Kepler problem) has the same $SO(4)$ symmetry. This explains the "accidental" degeneracy: states with the same $n$ but different $\ell$ have the same energy. What breaks this symmetry in real hydrogen (what perturbations cause the Lamb shift and fine structure splitting)?

**TE 16.2: What If GR Gave 44 Instead of 43?**
Einstein computed the GR perihelion precession in November 1915 and got exactly 43 arcsec/century — matching the unexplained residual exactly. Suppose the answer had been 44 arcsec/century, or 30, or 0. How would each discrepancy have been interpreted?

(a) 44 arcsec/century: within measurement uncertainty? (The observed value was known to about 1 arcsec/century precision in 1915.) Would the theory still have been accepted?
(b) 0 arcsec/century: GR predicts no precession. Would this have ruled out GR? Or would physicists have looked for errors in the Newtonian calculation?
(c) The observation of exactly 43 arcsec without free parameters — agreeing with an anomaly known for 56 years — is a powerful argument for GR. But could it have been coincidence? Estimate the probability of a random theory giving the right answer to within 1% without a free parameter.

---

## Laboratory Explorations

**Lab 16.1: Orbital Simulation**
Using a computer (Python, Julia, or similar), numerically integrate the equations of motion for a planet orbiting the Sun with and without the GR correction $-GM\ell^2/(c^2 r^3)$ to the potential.

(a) Use Mercury's parameters. Plot 100 orbits. For the pure Newtonian case, verify the orbit closes exactly (perihelion angle is constant).
(b) Add the GR correction. Measure the perihelion angle after each orbit. Plot perihelion angle vs. orbit number. Fit a line to extract the precession rate in arcsec/century.
(c) Verify your numerical result matches the analytical formula $\Delta\phi = 6\pi GM/(c^2 a(1-e^2))$.
(d) Experiment with other values of $\ell$ (larger eccentricity). Does the precession rate per orbit increase or decrease for higher eccentricity? Does the total angular rate ($\Delta\phi/T$) in arcsec/year increase or decrease? Explain physically.

**Lab 16.2: Kepler's Laws with a Conical Pendulum**
A conical pendulum (mass on a string, swinging in a horizontal circle) provides an analogy to orbital mechanics. (a) Derive the relationship between angle $\theta$, radius $r = L\sin\theta$, and angular velocity $\omega$ for the conical pendulum. (b) Show this is equivalent to an effective 1D problem with $V_{\rm eff}(r) = mg\sqrt{L^2 - r^2} + \ell^2/(2mr^2)$ where $\ell = mr^2\omega$ is the angular momentum. (c) Measure the period $T$ as a function of $\theta$ and verify the relation $T = 2\pi\sqrt{L\cos\theta/g}$. (d) Does Kepler's second law hold? (Equal areas in equal times — is the angular velocity constant for this system?)
