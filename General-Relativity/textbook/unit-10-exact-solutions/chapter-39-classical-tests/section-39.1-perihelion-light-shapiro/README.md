# Section 39.1: Perihelion Precession, Light Deflection, and the Shapiro Delay

---

## Setup: Geodesics in the Schwarzschild Metric

All four classical tests follow from the geodesic equations of the Schwarzschild metric. Using natural units ($G = c = 1$), the metric in the equatorial plane is:
$$ds^2 = -f(r)dt^2 + f(r)^{-1}dr^2 + r^2d\phi^2, \quad f(r) = 1 - \frac{2M}{r}$$

The two Killing vectors give two conserved quantities:
$$E = f(r)\dot{t} = \left(1-\frac{2M}{r}\right)\frac{dt}{d\tau}, \quad L = r^2\dot{\phi} = r^2\frac{d\phi}{d\tau}$$

Combined with the normalization $g_{\mu\nu}\dot{x}^\mu\dot{x}^\nu = -1$ (timelike, $\epsilon = 1$) or $= 0$ (null, $\epsilon = 0$):

$$\dot{r}^2 = E^2 - V_{\rm eff}(r), \quad V_{\rm eff}(r) = f(r)\left(\epsilon + \frac{L^2}{r^2}\right)$$

Substituting $f = 1 - 2M/r$ for timelike ($\epsilon = 1$):
$$V_{\rm eff}(r) = 1 - \frac{2M}{r} + \frac{L^2}{r^2} - \frac{2ML^2}{r^3}$$

The first three terms are the Newtonian effective potential plus rest energy; the last term $-2ML^2/r^3$ is the GR correction that drives all four classical tests.

---

## Perihelion Precession

**The orbit equation.** Using $d/d\tau = \dot\phi\,d/d\phi = (L/r^2)d/d\phi$, and the substitution $u = 1/r$ (so $r = 1/u$, $dr/d\phi = -u'/u^2$):

$$\left(\frac{du}{d\phi}\right)^2 = \frac{E^2 - 1}{L^2} + \frac{2M}{L^2}u - u^2 + 2Mu^3$$

Differentiating with respect to $\phi$:
$$\frac{d^2u}{d\phi^2} + u = \frac{M}{L^2} + 3Mu^2$$

**Newtonian solution:** Ignore the GR correction $3Mu^2$. The solution is $u_0 = (M/L^2)(1 + e\cos\phi)$ — the conic section (ellipse for $|e| < 1$), with perihelion at $\phi = 0$.

**GR perturbation:** Write $u = u_0 + u_1$ where $u_1$ is the small GR correction. Substituting:
$$u_1'' + u_1 = 3Mu_0^2 = 3M\left(\frac{M}{L^2}\right)^2(1 + e\cos\phi)^2$$

The right-hand side contains a term $\propto e\cos\phi$ that drives resonant forcing — a secular perturbation. The resonant term gives:
$$u_1 = \frac{3M^3}{L^4}e\phi\sin\phi$$

This is a correction that grows secularly with $\phi$. The full orbit is approximately:
$$u \approx \frac{M}{L^2}\left[1 + e\cos(\phi(1-\delta))\right]$$
where the resonant term corresponds to a perihelion at $\phi(1-\delta) = 0$, i.e., $\phi = 2\pi(1+\delta)$ for the next perihelion. The precession per orbit is:
$$\Delta\phi = 2\pi\delta = \frac{6\pi M^2}{L^2} = \frac{6\pi GM}{a(1-e^2)c^2}$$

(restoring $G$ and $c$). For Mercury:
- $a = 5.79\times 10^{10}$ m (semi-major axis)
- $e = 0.206$ (eccentricity)
- $M = M_\odot = 1.989\times 10^{30}$ kg

$$\Delta\phi_{\rm Mercury} = \frac{6\pi GM_\odot}{a(1-e^2)c^2} \approx 5.02\times 10^{-7} \text{ rad/orbit} \approx 0.103'' \text{ per orbit}$$

Mercury's orbital period is 87.97 days, so there are $\sim 415$ orbits per century:
$$\Delta\phi_{\rm century} \approx 42.98'' \text{ per century}$$

The observed anomaly (after subtracting Newtonian perturbations from other planets) is $43.11'' \pm 0.45''$/century. **Agreement to better than 0.3%.**

---

## Deflection of Light

For null geodesics ($\epsilon = 0$) with impact parameter $b = L/E$:
$$\frac{d^2u}{d\phi^2} + u = 3Mu^2$$

**Straight-line solution:** Ignore GR. $u_0 = \sin\phi/b$ — a straight line at closest approach $b$.

**GR correction:** $u = u_0 + u_1$ with $u_1'' + u_1 = 3Mu_0^2 = (3M/b^2)\sin^2\phi = (3M/(2b^2))(1-\cos 2\phi)$.

The solution:
$$u_1 = \frac{3M}{2b^2}\left(1 + \frac{1}{3}\cos 2\phi\right)$$
plus a secular term that corresponds to the deflection.

**Total deflection angle:** At large $r$ (small $u$), the asymptotic directions of the light ray satisfy $u\to 0$. The incoming direction is $\phi\to 0 + \delta_1$ and the outgoing direction is $\phi\to\pi + \delta_2$. The total deflection:
$$\delta\phi = \delta_1 + \delta_2 = \frac{4M}{b} = \frac{4GM}{bc^2}$$

For a light ray grazing the Sun ($b = R_\odot = 6.96\times 10^8$ m):
$$\delta\phi = \frac{4GM_\odot}{R_\odot c^2} = \frac{4\times 6.67\times 10^{-11}\times 1.989\times 10^{30}}{6.96\times 10^8 \times (3\times 10^8)^2} = 1.749'' $$

**Eddington's 1919 measurement:** $1.61'' \pm 0.30''$ and $1.98'' \pm 0.12''$ (two telescope stations, Principe and Sobral). Consistent with GR ($1.75''$), inconsistent with the EP-only "Newtonian" prediction ($0.875''$).

**Modern measurements (VLBI):** Radio astrometry of extragalactic sources near the Sun. Best result: $\gamma = 0.99983 \pm 0.00045$ where $\gamma = 1$ in GR. Confirms GR prediction to $0.04\%$.

---

## Shapiro Time Delay

A signal traveling radially between Earth (at $r_E$) and a planet or spacecraft (at $r_P$) near solar conjunction (close to the Sun) takes longer than in flat space.

From the null geodesic equation, the travel time is:
$$t(r_1, r_2) = \int_{r_1}^{r_2}\frac{dr}{f(r)\sqrt{1-b^2f(r)/r^2}} \approx \int_{r_1}^{r_2}\frac{dr}{\sqrt{1-b^2/r^2}} + 2M\ln\frac{r+\sqrt{r^2-b^2}}{b}$$

The excess delay compared to flat space:
$$\Delta t_{\rm Shapiro} = \frac{2GM_\odot}{c^3}\ln\frac{4r_Er_P}{b^2}$$

For a signal to Venus at superior conjunction ($b \approx R_\odot$, $r_E \approx 1$ AU, $r_P \approx 0.72$ AU):
$$\Delta t \approx \frac{2GM_\odot}{c^3}\ln\left(\frac{4\times1.5\times10^{11}\times1.1\times10^{11}}{(6.96\times10^8)^2}\right) \approx 248\,\mu\text{s}$$

This is a large signal — easily measurable with radar timing.

**Measurements:**
- Shapiro et al. (1971): Venus radar, confirmed to 3%
- Mars (Mariner 6/7, 1971): confirmed to 3%
- Cassini spacecraft (Bertotti et al. 2003): $\gamma = 1 + (2.1\pm 2.3)\times 10^{-5}$ — confirmed to $0.002\%$

---

## Gravitational Redshift (from Schwarzschild)

A photon emitted from radius $r_{\rm emit}$ with frequency $f_{\rm emit}$ is observed at radius $r_{\rm obs}$ with frequency:
$$\frac{f_{\rm obs}}{f_{\rm emit}} = \sqrt{\frac{g_{tt}(r_{\rm emit})}{g_{tt}(r_{\rm obs})}} = \sqrt{\frac{1-2M/r_{\rm emit}}{1-2M/r_{\rm obs}}}$$

This follows from the fact that the Killing vector $\xi^\mu = (1,0,0,0)$ gives a conserved frequency along null geodesics: $\omega = -k_\mu\xi^\mu = f(r)dt/d\lambda = \text{const} \times f(r)/f(r) =$ ... more carefully: the conserved quantity along the null geodesic is $E = f\dot{t}$, and the locally measured frequency by a static observer at $r$ is $\omega_{\rm local} = E/\sqrt{f(r)}$. Hence $\omega_{\rm local}\sqrt{f(r)} = \text{const}$, giving the ratio above.

For weak fields:
$$\frac{f_{\rm obs}}{f_{\rm emit}} \approx 1 + \frac{\Phi_{\rm emit} - \Phi_{\rm obs}}{c^2} = 1 - \frac{g\Delta h}{c^2}$$

For light ascending out of a gravitational potential well ($r_{\rm obs} > r_{\rm emit}$): $f_{\rm obs} < f_{\rm emit}$ (redshift). For light descending: blueshift.

**Pound-Rebka (1959):** $\Delta f/f = 2.57\times 10^{-15}$ for $\Delta h = 22.5$ m. Confirmed GR to 10%; Pound-Snider (1965) to 1%.

**Gravity Probe A (1976):** Hydrogen maser at 10,000 km altitude. Confirmed to $0.02\%$.

**Modern atomic clock tests (2010):** Optical atomic clocks separated by 33 cm confirmed the gravitational redshift — the most sensitive test of gravitational time dilation ever performed.

---

## Summary: All Four Tests from One Formula

All four classical tests follow from the one formula for the effective potential in the Schwarzschild metric:
$$V_{\rm eff}(r) = f(r)\left(\epsilon + \frac{L^2}{r^2}\right) = \left(1 - \frac{2M}{r}\right)\left(\epsilon + \frac{L^2}{r^2}\right)$$

| Test | Observable | GR prediction | Measurement | Status |
|---|---|---|---|---|
| Mercury precession | $\Delta\phi$/century | $42.98''$ | $43.11\pm 0.45''$ | $\checkmark$ (0.3%) |
| Light deflection | $\delta\phi$ at Sun | $1.7495''$ | $1.7498\pm 0.0003''$ (VLBI) | $\checkmark$ (0.004%) |
| Shapiro delay | $\Delta t$ (Cassini) | $248\,\mu$s | Confirmed $\gamma = 1\pm 2.3\times10^{-5}$ | $\checkmark$ (0.002%) |
| Gravitational redshift | $\Delta f/f$ | $gh/c^2$ | Confirmed (Gravity Probe A: 0.02%) | $\checkmark$ (0.02%) |

Every classical test confirms GR. The best tests now constrain PPN parameters to $10^{-5}$. No deviation from GR has been detected in the weak-field, slow-motion regime.

