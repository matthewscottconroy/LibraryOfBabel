# Chapter 34: Exercises

---

**34.1.** *Quantifying the WEP.*

The Eötvös parameter is $\eta(A,B) = 2|a_A - a_B|/(a_A + a_B)$, where $a_A$ and $a_B$ are the gravitational accelerations of two test bodies.

(a) In the MICROSCOPE satellite (2017), the experiment was sensitive to differential accelerations of $\Delta a \sim 10^{-14}$ m/s$^2$ (the instrument's noise floor). The gravitational acceleration in low-Earth orbit is $g \approx 9.3$ m/s$^2$. What is the corresponding constraint on $\eta$?

(b) If $\eta(A,B) = 10^{-15}$ for titanium vs. platinum-rhodium, and the satellite is at altitude $h = 710$ km above Earth, the differential acceleration signal would be $\Delta a = \eta\cdot g \approx \eta \cdot 8.8$ m/s$^2$. What is $\Delta a$ in m/s$^2$ and in units of the Sun's gravitational acceleration at Earth ($g_\odot \approx 6\times 10^{-3}$ m/s$^2$)?

(c) The MICROSCOPE experiment modulated the differential signal at orbital frequency $f_{\rm orb} \approx 1.8\times 10^{-4}$ Hz to separate it from systematic noise. What is the signal integration time needed to achieve signal-to-noise of 10 if the noise power spectral density at $f_{\rm orb}$ is $S_n = (3\times 10^{-12})^2$ m$^2$s$^{-2}$/Hz?

---

**34.2.** *Gravitational redshift in everyday life.*

(a) A clock at the top of the Eiffel Tower (height $h = 324$ m) compared to a clock at ground level. Using $\Delta\tau/\tau = gh/c^2$, compute the time gained per day by the upper clock. How long would it take for the upper clock to be 1 microsecond ahead of the lower clock?

(b) The Pound-Rebka experiment (1959) measured the redshift of 14.4 keV gamma rays from $^{57}$Fe over a height of 22.5 m. The predicted fractional frequency shift is $\Delta f/f = gh/c^2$. Compute $\Delta f/f$ numerically. Compare to the natural linewidth of the $^{57}$Fe Mössbauer resonance, which is $\Gamma/f \approx 1.1\times 10^{-12}$.

(c) GPS satellites orbit at altitude $h = 20,200$ km. Compute the gravitational time dilation (clocks on satellite run faster than ground clocks) in microseconds per day. At what rate would GPS position errors accumulate (in km/day) if this correction were not applied? (Position error $\approx c\times\Delta\tau$.)

---

**34.3.** *Light bending: equivalence principle vs. full GR.*

(a) From the equivalence principle argument (elevator falling distance $\delta = gW^2/(2c^2)$ during a crossing of width $W$), derive the deflection angle for a photon grazing the Sun's limb. Use $g = GM_\odot/R_\odot^2$. Express in arcseconds. This is the "Newtonian" prediction.

(b) The full GR result is exactly twice the EP result. Explain physically why the spatial curvature of the metric (the fact that $g_{rr} \neq 1$ even in the Schwarzschild exterior) accounts for the extra factor of 2.

(c) The 1919 Eddington expedition measured $1.61'' \pm 0.30''$ (Wilson 1919) and $1.98'' \pm 0.12''$ (Sobral 1919). Are these consistent with the GR prediction ($1.75''$) at the $1\sigma$ level? With the Newtonian prediction ($0.875''$)?

(d) Modern VLBI (Very Long Baseline Interferometry) measurements of radio sources near the Sun confirm the GR prediction to $0.01\%$. The current best measurement is $\gamma_{\rm VLBI} = 0.99983 \pm 0.00045$ (where $\gamma = 1$ in GR). What is the constraint on the PN parameter $\gamma$? What class of theories would predict $\gamma \neq 1$?

---

**34.4.** *The Lense-Thirring effect and Gravity Probe B.*

The frame-dragging angular velocity of a gyroscope near a rotating Earth is:
$$\boldsymbol\Omega_{\rm LT} = \frac{G}{c^2 r^3}\left[3(\mathbf{J}\cdot\hat{r})\hat{r} - \mathbf{J}\right]$$

(a) For GP-B orbiting at altitude 642 km with Earth's angular momentum $J = I\omega = 7.07\times 10^{33}$ kg·m$^2$/s (moment of inertia $I \approx 8\times 10^{37}$ kg·m$^2$, $\omega = 7.3\times 10^{-5}$ rad/s), compute the predicted frame-dragging precession rate in milliarcseconds per year (GP-B's orbital radius $\approx 7020$ km).

(b) The geodetic precession rate for the same orbit is $\boldsymbol\Omega_{\rm dS} = \frac{3GM_\oplus}{2c^2 r^3}(\mathbf{v}\times\mathbf{r})$. For a polar orbit at $h = 642$ km, compute the geodetic precession in arcseconds per year. Compare to the measured GP-B value.

(c) The Lense-Thirring effect for pulsars can be measured indirectly through orbital precession of the pulsar's companion. For the double pulsar PSR J0737-3039, the estimated orbital inclination change due to Lense-Thirring precession is $\sim 0.4°$/year. What does this imply about the pulsar's spin angular momentum if $R = 10$ km and $M = 1.4 M_\odot$?

---

## Thought Experiments

**T34.1.** *What would happen without the equivalence principle?*

Imagine a universe in which gravitational mass were not equal to inertial mass — specifically, in which $m_g/m_i$ depended on the chemical composition of a body. In such a universe:
(a) Would all bodies fall at the same rate in a gravitational field? What would Galileo's tower experiment show?
(b) Could a "freely falling" frame be defined in which all physics looks like flat SR? 
(c) Would gravity affect light differently from matter? What would this imply for GR's prediction of light bending?
(d) What would this imply about the equivalence of inertial and gravitational mass at a fundamental level? Can you construct a scalar theory (non-geometric) of gravity in such a universe?

**T34.2.** *Is the equivalence principle really a principle, or a theorem?*

In Newton's mechanics, $m_i = m_g$ is a coincidence with no theoretical explanation. In GR, it is a theorem: the geodesic equation has no mass. Does this change whether we should call it a "principle"?

More broadly: GR was historically discovered by using the equivalence principle as a guiding heuristic. But in the modern formulation of GR (metric manifold, Einstein equations), the equivalence principle is a consequence of the theory, not an axiom. Does this mean the principle was "just scaffolding" that can be thrown away once the building is complete? Or does it remain physically illuminating?

Discuss the difference between a principle as a *historical discovery heuristic* and a principle as a *formal axiom*. Other examples: the principle of relativity (SR), the variational principle (mechanics), the second law of thermodynamics.

---

## Laboratory Exercise: Gravitational Redshift with Atomic Clocks

**L34.1.** *Observing gravitational time dilation with GPS clocks.*

GPS satellites carry cesium and rubidium atomic clocks accurate to nanoseconds. The gravitational blueshift of satellite clocks relative to ground clocks is well-established and used operationally. You can observe this effect using publicly available GPS data.

**Task 1:** From the GPS satellite system specification, the nominal clock correction applied to satellite clocks is $-4.464733\times 10^{-10}$ (dimensionless; clocks are slowed by this fractional amount). This is the combined gravitational blueshift ($+45.9$ μs/day) minus the velocity time dilation ($-7.2$ μs/day) = net $+38.7$ μs/day = $4.47\times 10^{-10}$ fractional. Verify this number from first principles using the Schwarzschild metric: compute the proper time rates of a ground clock and a satellite clock at altitude 20,200 km.

**Task 2:** Using a software-defined radio (SDR, e.g., RTL-SDR ~$20) with a GPS antenna, receive GPS signals and examine the satellite clock correction parameters in the navigation message (the "SV clock bias" parameter). Compare the clock correction values for satellites at different orbital altitudes (some GPS satellites are in slightly elliptical orbits — the eccentricity causes a time-varying clock correction that GPS receivers implement as $\Delta t = -\frac{2\sqrt{\mu a}}{c^2}e\sin E$ where $E$ is the eccentric anomaly).

**Task 3:** Using a Python GPS parser (e.g., `pygnss` or `gnss-tools`), parse a 24-hour RINEX observation file (available from IGS data centers). Compute the correlation between the satellite's height above Earth and its clock correction. Does the clock correction increase at higher altitude, as expected from gravitational time dilation?

