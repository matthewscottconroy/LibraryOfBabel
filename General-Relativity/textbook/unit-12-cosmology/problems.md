# Unit XII Problems: Cosmology

*The FLRW metric, Friedmann equations, Hubble parameter evolution, and the thermal history of the universe.*

**Difficulty:** ★ Introductory, ★★ Intermediate, ★★★ Advanced

---

## Part 1: The FLRW Metric and Friedmann Equations

**Problem 1.1** ★
The Friedmann-Lemaître-Robertson-Walker (FLRW) metric:

$$ds^2 = -c^2dt^2 + a^2(t)\left[\frac{dr^2}{1-kr^2} + r^2d\theta^2 + r^2\sin^2\!\theta\,d\phi^2\right]$$

where $a(t)$ is the scale factor and $k = -1, 0, +1$ for open, flat, closed universes.

(a) For $k = 0$ (flat): write the metric explicitly. What is the physical distance between two comoving observers at $r_1 = 0$ and $r_2 = r_0$ as a function of time?

(b) The Hubble parameter $H(t) = \dot{a}/a$. For $a(t) = a_0(t/t_0)^{2/3}$ (matter-dominated universe): compute $H(t)$ and the Hubble constant $H_0 = H(t_0)$.

(c) Redshift: a photon emitted at time $t_e$ with wavelength $\lambda_e$ is received today with wavelength $\lambda_0$. Show:
$$1 + z = \frac{\lambda_0}{\lambda_e} = \frac{a(t_0)}{a(t_e)}$$

(d) For a galaxy at redshift $z = 0.5$: what was the scale factor when the light was emitted (take $a(t_0) = 1$)?

**Problem 1.2** ★★
The Friedmann equations: applying the Einstein equations to the FLRW metric with a perfect fluid (pressure $p$, energy density $\rho$):

$$H^2 = \frac{\dot{a}^2}{a^2} = \frac{8\pi G\rho}{3} - \frac{kc^2}{a^2} + \frac{\Lambda c^2}{3}$$

$$\frac{\ddot{a}}{a} = -\frac{4\pi G}{3}\left(\rho + \frac{3p}{c^2}\right) + \frac{\Lambda c^2}{3}$$

(a) Show that for a flat universe ($k=0$) with $\Lambda = 0$ and equation of state $p = w\rho c^2$: the scale factor evolves as $a(t)\propto t^{2/(3(1+w))}$. Find the exponent for matter ($w=0$), radiation ($w=1/3$), and $\Lambda$ ($w=-1$).

(b) The critical density: for $k=0$, $\Lambda=0$: $\rho_c = 3H^2/(8\pi G)$. For $H_0 = 67.4$ km/s/Mpc: compute $\rho_c$ in kg/m³ and in units of proton masses per m³.

(c) Density parameters: $\Omega_m = \rho_m/\rho_c$, $\Omega_r = \rho_r/\rho_c$, $\Omega_\Lambda = \Lambda c^2/(3H^2)$. The flat universe condition: $\Omega_m + \Omega_r + \Omega_\Lambda = 1$. Observed values: $\Omega_m \approx 0.31$, $\Omega_\Lambda \approx 0.69$, $\Omega_r \approx 9\times10^{-5}$. Verify these sum to (approximately) 1.

**Problem 1.3** ★★
The age of the universe:

(a) The age $t_0 = \int_0^1 \frac{da}{aH(a)}$ where $H(a) = H_0\sqrt{\Omega_m a^{-3} + \Omega_\Lambda}$ (for a flat universe with matter + $\Lambda$).

(b) For pure matter ($\Omega_m = 1$, $\Omega_\Lambda = 0$): show $t_0 = 2/(3H_0)$. Compute numerically for $H_0 = 67.4$ km/s/Mpc.

(c) For the $\Lambda$CDM model ($\Omega_m = 0.31$, $\Omega_\Lambda = 0.69$): numerically integrate to find $t_0$. (You may use the approximation $t_0 \approx 0.96/H_0$ for these values.) Compare to the observed $t_0 = 13.8$ Gyr.

(d) Matter-$\Lambda$ equality: at what redshift $z_\Lambda$ does $\Omega_\Lambda a^0$ equal $\Omega_m a^{-3}$, i.e., when does $\Lambda$ begin to dominate?

---

## Part 2: Cosmological Evolution

**Problem 2.1** ★★
Hubble parameter evolution: $H(z) = H_0\sqrt{\Omega_r(1+z)^4 + \Omega_m(1+z)^3 + \Omega_k(1+z)^2 + \Omega_\Lambda}$.

(a) Evaluate $H(z)$ at $z = 0$, $z = 0.5$, $z = 1$, $z = 1000$ (recombination), $z = 3400$ (matter-radiation equality) using $\Omega_m = 0.31$, $\Omega_\Lambda = 0.69$, $\Omega_r = 9\times10^{-5}$, $H_0 = 67.4$ km/s/Mpc. Normalize to $H_0$.

(b) The deceleration parameter $q = -\ddot{a}a/\dot{a}^2 = -1 - \dot{H}/H^2$. For the $\Lambda$CDM model today: $q_0 = \Omega_m/2 - \Omega_\Lambda$. Compute $q_0$ and determine whether the universe is currently accelerating or decelerating.

(c) At what redshift $z_\text{acc}$ did the universe transition from deceleration to acceleration ($\ddot{a} = 0$)?

**Problem 2.2** ★★★
Particle horizons and the horizon problem:

(a) The comoving particle horizon: the maximum comoving distance from which light can have reached us since the Big Bang:

$$d_H = c\int_0^{t_0}\frac{dt'}{a(t')} = c\int_0^1\frac{da'}{a'^2 H(a')}$$

For a matter-dominated universe ($H = H_0(a_0/a)^{3/2}$): compute $d_H$ in Mpc.

(b) The CMB is a snapshot of the universe at $z = 1100$ (recombination). Two points on the CMB sky separated by $> 2°$ were not in causal contact at recombination (their particle horizons didn't overlap). Yet the CMB temperature is isotropic to 1 part in $10^5$. This is the horizon problem. State why inflationary cosmology resolves it.

(c) Inflation posits a period of exponential expansion $a\propto e^{Ht}$ (de Sitter phase, $w = -1$). During inflation, the particle horizon grows exponentially. For inflation from $t_i$ to $t_f$ with $H_\text{inf} = 10^{38}$ Hz (GUT scale): how much does the horizon grow?

**Problem 2.3** ★★
Nucleosynthesis constraints:

(a) The neutron-to-proton ratio at nucleosynthesis is set by the weak interaction rate $\Gamma_\text{weak} \sim G_F^2 T^5$ and the Hubble rate $H \sim T^2/M_\text{Pl}$ (radiation domination). Freeze-out occurs when $\Gamma = H$. At $T_\text{freeze} \approx 0.8$ MeV: $n/p \approx e^{-\Delta m/T_\text{freeze}} \approx 1/7$ (where $\Delta m c^2 = 1.29$ MeV is the neutron-proton mass difference). Verify this ratio.

(b) At nucleosynthesis ($T \approx 0.1$ MeV, $t \approx 200$ s), essentially all neutrons are bound into $^4$He. The helium mass fraction: $Y_p = 2(n/p)/(1 + n/p)$. Compute $Y_p$ and compare to the observed primordial helium abundance $Y_p^\text{obs} \approx 0.247$.

(c) The baryon-to-photon ratio $\eta = n_b/n_\gamma \approx 6\times10^{-10}$ sets the amount of deuterium produced in BBN ($\eta$ larger $\to$ less deuterium). Explain qualitatively why more baryons means less surviving deuterium. (Hint: more baryons $\to$ faster D+D$\to^4$He reactions.)

---

## Part 3: Distance Measures and Observational Cosmology

**Problem 3.1** ★★
Cosmological distances:

(a) The luminosity distance $d_L = (1+z)\chi$ where $\chi = c\int_0^z dz'/H(z')$ is the comoving distance. The angular diameter distance $d_A = \chi/(1+z)$. Show $d_L = (1+z)^2 d_A$.

(b) A Type Ia supernova at $z = 0.5$ has observed flux $F$ and known luminosity $L$. Using $F = L/(4\pi d_L^2)$: compute the distance modulus $\mu = 5\log_{10}(d_L/10\text{ pc})$ for $\Omega_m = 0.31$, $\Omega_\Lambda = 0.69$, $H_0 = 70$ km/s/Mpc. (Numerical integration required.)

(c) The Supernova Cosmology Project (Perlmutter et al. 1999) and High-Z Team (Riess et al. 1998) found that supernovae at $z \approx 0.5$–$1$ were fainter than expected for an $\Omega_\Lambda = 0$ universe, implying accelerating expansion. By how many magnitudes would the $z = 0.5$ supernova be fainter in $\Lambda$CDM vs. a matter-only flat universe?

**Problem 3.2** ★★★
CMB power spectrum:

(a) The CMB temperature anisotropy $\delta T/T(\hat{n})$ is expanded in spherical harmonics: $\delta T/T = \sum_{\ell m} a_{\ell m} Y_{\ell m}(\hat{n})$. The power spectrum $C_\ell = \langle|a_{\ell m}|^2\rangle$. The angular scale $\theta \sim 180°/\ell$.

(b) The first acoustic peak at $\ell \approx 220$ corresponds to the sound horizon at recombination. The sound horizon: $r_s = c_s\int_0^{t_\text{rec}}dt/a(t)$ where $c_s = c/\sqrt{3}$ (radiation era). Estimate $r_s$ in Mpc (using $t_\text{rec}\sim 380{,}000$ yr, matter-dominated).

(c) The angular diameter distance to the last scattering surface: $d_A(z_\text{rec}) \approx 13{,}700$ Mpc. The peak angle: $\theta_\text{peak} = r_s/d_A$. Compute $\theta_\text{peak}$ and the corresponding $\ell_\text{peak}$.

(d) The position of the first peak in $\ell$ depends on the geometry of the universe (flat, open, or closed). How does $\ell_\text{peak}$ shift for a closed universe ($k=+1$)? The observed $\ell_\text{peak} \approx 220$ is consistent with a flat universe — this is a key piece of evidence for $\Omega_\text{total} \approx 1$.
