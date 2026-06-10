# Chapter 47: The Friedmann-Lemaître-Robertson-Walker Metric and Cosmological Dynamics

---

## Chapter Introduction

Look at the night sky on a clear night, far from city lights. Every star you see is in our galaxy — a collection of $\sim 10^{11}$ stars 30,000 light-years across. But the Milky Way is just one galaxy among an estimated $2\times 10^{12}$ in the observable universe. The observable universe spans $\sim 93$ billion light-years, contains $\sim 10^{80}$ baryons, and has been expanding for $13.8$ billion years from an extraordinarily hot, dense initial state.

How do we describe such a thing? The answer, remarkably, is that the universe on the largest scales is almost perfectly uniform and isotropic. The galaxies are not uniformly distributed — they cluster into filaments and voids — but on scales larger than $\sim 300$ million light-years, the distribution is smooth to better than 0.01%. This uniformity, called the **cosmological principle**, licenses us to treat the universe as a homogeneous and isotropic fluid, described by a single evolving scale factor $a(t)$.

General relativity applied to this fluid gives the **Friedmann equations** — two coupled ODEs for $a(t)$ and the energy density $\rho(t)$. These equations, together with an equation of state relating pressure to density, determine the entire history of the universe. They predict that the universe is expanding (Hubble's observation, 1929), that the expansion began in a hot Big Bang (Gamow, 1948), that there is a relic radiation background at $\sim 3$ K (the CMB, Penzias-Wilson 1965), that the expansion is currently accelerating (Perlmutter-Riess-Schmidt 1998), and that the universe is flat to $0.1\%$ (Planck 2018).

This chapter derives the Friedmann-Lemaître-Robertson-Walker (FLRW) metric from the cosmological principle, derives the Friedmann equations from the Einstein field equations applied to a perfect fluid, and analyzes the expansion history of the universe across its different energy-dominated epochs.

---

## Chapter Sections

- [Section 47.1: The FLRW Metric](section-47.1-flrw-metric/README.md)
- [Section 47.2: The Friedmann Equations](section-47.2-friedmann-equations/README.md)

---

## Important Concepts

**The cosmological principle**: On scales larger than $\sim 300$ Mpc, the universe is homogeneous and isotropic. Homogeneous means the same at every point (no special location); isotropic means the same in every direction (no special direction). Together they strongly constrain the form of the metric.

**Comoving coordinates**: In an expanding universe, we use coordinates that expand with the universe. A galaxy at rest in the cosmological fluid (no peculiar motion) has fixed comoving coordinates $(\chi, \theta, \phi)$. Its physical (proper) distance grows as $d(t) = a(t)\chi$.

**Scale factor $a(t)$**: The single function characterizing the size of the universe. Conventionally $a(t_0) = 1$ today. The Hubble parameter $H(t) = \dot{a}/a$ measures the expansion rate. Today's value $H_0 = 67.4 \pm 0.5$ km/s/Mpc (Planck 2018). The "tension" between Planck CMB measurements ($67.4$) and local distance ladder measurements ($73.0 \pm 1.0$ by Riess et al.) is one of the major outstanding problems in cosmology.

**Redshift**: A photon emitted at scale factor $a_{\rm em}$ and received at $a_0 = 1$ has wavelength stretched by $\lambda_{\rm obs}/\lambda_{\rm em} = 1/a_{\rm em} = 1 + z$. Redshift $z$ is related to scale factor by $a = 1/(1+z)$.

**Energy density components**: The universe contains radiation ($\rho_r \propto a^{-4}$), matter ($\rho_m \propto a^{-3}$), and dark energy/cosmological constant ($\rho_\Lambda = \Lambda c^2/(8\pi G) = \text{const}$). Curvature contributes as an effective energy component with $w = -1/3$.

**The $\Omega$ parameters**: Each energy component is expressed as a fraction of the critical density $\rho_c = 3H^2/(8\pi G)$: $\Omega = \rho/\rho_c$. Today: $\Omega_\Lambda \approx 0.685$, $\Omega_m \approx 0.315$ (matter), $\Omega_r \approx 9\times 10^{-5}$ (radiation), $\Omega_k \approx 0.001\pm 0.002$ (spatial curvature — consistent with flat). Total $\Omega = \Omega_\Lambda + \Omega_m + \Omega_r + \Omega_k = 1.000 \pm 0.002$.

**Cosmic epochs**: 
- Radiation domination ($z > 3400$, $T > 9000$ K): $a \propto t^{1/2}$
- Matter domination ($3400 > z > 0.3$): $a \propto t^{2/3}$
- Dark energy domination ($z < 0.3$): $a \propto e^{Ht}$ (de Sitter-like)

**Particle horizons and the observable universe**: Light emitted at the Big Bang ($z\to\infty$) traveling to us today has traced a comoving distance $\chi_H = c\int_0^{t_0}dt/a(t)$. The particle horizon at the time of last scattering (when the CMB was released, $z \approx 1100$) was much smaller than today, explaining why the CMB is so uniform (a problem solved by inflation).

---

## Important Figures

**Alexander Friedmann (1888–1925)**: Russian mathematician. Derived the Friedmann equations in 1922 and showed that Einstein's static universe was unstable. He found expanding universe solutions and predicted the Big Bang (though without knowing about it physically). Died of typhoid at age 37, two years before Hubble's observational confirmation of expansion.

**Georges Lemaître (1894–1966)**: Belgian physicist and Catholic priest. Independently derived the Friedmann equations and in 1927 proposed that the universe was expanding, estimating the Hubble constant from galaxy recession velocities. Proposed in 1931 what he called the "hypothesis of the primeval atom" — the Big Bang. Visited Einstein in 1927; Einstein reportedly said "Your calculations are correct, but your physics is atrocious." By 1932 Einstein called Lemaître's theory "the most beautiful and satisfactory explanation of creation I have ever heard."

**Howard P. Robertson (1903–1961) and Arthur Geoffrey Walker (1909–2001)**: Independently proved (1935–36) that the most general metric consistent with homogeneity and isotropy is the FLRW form with spatial curvature $k = -1, 0, +1$.

**Edwin Hubble (1889–1953)**: Demonstrated in 1924–25 that the Andromeda "nebula" was a separate galaxy 2 million light-years away — resolving the "Great Debate" (Curtis vs. Shapley, 1920) about whether spiral nebulae were within or outside the Milky Way. In 1929, measured the recession velocities of nearby galaxies and found the linear relation $v = H_0 d$ (Hubble's law). Hubble resisted calling his observations evidence for an expanding universe, remaining agnostic about cosmological models throughout his career.

**Arno Penzias (born 1933) and Robert Wilson (born 1936)**: Discovered the cosmic microwave background radiation in 1965, confirming the Big Bang model. Using a horn antenna at Bell Labs, they found an isotropic noise source at $\sim 3.5$ K that they could not eliminate. Dicke, Peebles, Roll, and Wilkinson at Princeton had been preparing to search for exactly this radiation; Penzias and Wilson's accidental discovery preempted them. Nobel Prize in Physics 1978.

**Saul Perlmutter (born 1959), Brian P. Schmidt (born 1967), and Adam G. Riess (born 1969)**: Discovered the accelerating expansion of the universe in 1998 using Type Ia supernovae as standard candles. Two independent teams (the Supernova Cosmology Project and the High-Z Supernova Search Team) found that distant supernovae were $\sim 25\%$ dimmer than expected, implying the expansion was speeding up. This required a positive cosmological constant $\Lambda > 0$, or "dark energy." Nobel Prize in Physics 2011.

---

## Further Reading

**Friedmann, A. (1922). "Über die Krümmung des Raumes." *Zeitschrift für Physik*, 10, 377.**
The original derivation of the Friedmann equations. Friedmann sent the paper to Einstein; Einstein initially claimed it contained an error, then retracted and admitted it was correct.

**Lemaître, G. (1927). "Un Univers homogène de masse constante et de rayon croissant rendant compte de la vitesse radiale des nébuleuses extragalactiques." *Annales de la Société Scientifique de Bruxelles*, 47A, 49.**
Lemaître's 1927 paper proposing an expanding universe with an estimate of the Hubble constant ($\sim 625$ km/s/Mpc — the first measurement, predating Hubble's 1929 paper by two years).

**Hubble, E. (1929). "A Relation between Distance and Radial Velocity among Extra-Galactic Nebulae." *Proceedings of the National Academy of Sciences*, 15, 168.**
The observational discovery of Hubble's law.

**Penzias, A.A. and Wilson, R.W. (1965). "A Measurement of Excess Antenna Temperature at 4080 Mc/s." *Astrophysical Journal Letters*, 142, L419.**
The CMB discovery paper — just two pages, with a carefully understated title.

**Perlmutter, S. et al. (1999). "Measurements of Omega and Lambda from 42 High-Redshift Supernovae." *Astrophysical Journal*, 517, 565.**
The Supernova Cosmology Project result, with the accelerating expansion measurement.

**Riess, A.G. et al. (1998). "Observational Evidence from Supernovae for an Accelerating Universe and a Cosmological Constant." *Astronomical Journal*, 116, 1009.**
The High-Z Supernova Search Team result.

**Peebles, P.J.E. (1993). *Principles of Physical Cosmology.* Princeton University Press.**
The standard comprehensive reference for physical cosmology. Covers FLRW metric, Friedmann equations, CMB, large-scale structure, and early universe. Dense but thorough.

**Kolb, E.W. and Turner, M.S. (1990). *The Early Universe.* Addison-Wesley.**
The classic text on early universe cosmology: Big Bang nucleosynthesis, baryogenesis, inflation, phase transitions, relics. Essential for particle cosmology.

**Weinberg, S. (2008). *Cosmology.* Oxford University Press.**
A thorough modern treatment by a master of clarity. Covers everything from FLRW geometry through inflation, CMB, and dark energy.

**Planck Collaboration (2020). "Planck 2018 Results VI: Cosmological Parameters." *Astronomy & Astrophysics*, 641, A6.**
The definitive current measurement of cosmological parameters from the CMB.

---

## Exercises

**47.1.** *Deriving the FLRW metric.*

(a) The most general spatially homogeneous and isotropic 3-metric can be written in the form $d\ell^2 = f(r)dr^2 + r^2d\Omega^2$. Show that requiring constant curvature ($R_{(3)} = 6k$ for some constant $k$) gives $f(r) = 1/(1-kr^2)$.

(b) Rewrite the FLRW metric using the substitution $\chi = \int_0^r dr'/\sqrt{1-kr'^2}$ (comoving radial distance) to obtain:
$$ds^2 = -c^2dt^2 + a(t)^2\left[d\chi^2 + f_k(\chi)^2 d\Omega^2\right]$$
where $f_k(\chi) = \sin\chi$, $\chi$, or $\sinh\chi$ for $k = +1, 0, -1$ respectively.

(c) Compute the proper distance $d(t) = a(t)\chi$ between two galaxies with comoving separation $\chi$. Show that $\dot{d}/d = \dot{a}/a \equiv H(t)$ — Hubble's law as an identity in the FLRW metric.

(d) Compute the volume of a sphere of comoving radius $\chi$ for $k = +1$ (closed), $k = 0$ (flat), and $k = -1$ (open). For $k = +1$, show the total volume is $2\pi^2 a^3$ (finite and closed).

---

**47.2.** *Friedmann equations from the Einstein field equations.*

(a) For the FLRW metric, compute the nonzero Christoffel symbols. The only independent ones are $\Gamma^0_{ij} = a\dot{a}\gamma_{ij}$ and $\Gamma^i_{0j} = (\dot{a}/a)\delta^i_j$ (where $\gamma_{ij}$ is the spatial metric).

(b) Compute the Ricci tensor components $R_{00}$ and $R_{ij}$. Show:
$$R_{00} = -\frac{3\ddot{a}}{a}, \quad R_{ij} = -\left[\frac{\ddot{a}}{a} + 2\left(\frac{\dot{a}}{a}\right)^2 + \frac{2kc^2}{a^2}\right]\gamma_{ij}$$

(c) Apply the Einstein equations $G_{\mu\nu} = 8\pi G T_{\mu\nu}/c^4$ with $T_{\mu\nu} = \text{diag}(\rho c^2, p, p, p)$ (perfect fluid at rest) to derive the Friedmann equations:
$$H^2 = \left(\frac{\dot{a}}{a}\right)^2 = \frac{8\pi G\rho}{3} - \frac{kc^2}{a^2} + \frac{\Lambda c^2}{3}$$
$$\frac{\ddot{a}}{a} = -\frac{4\pi G}{3}\left(\rho + \frac{3p}{c^2}\right) + \frac{\Lambda c^2}{3}$$

(d) Show that covariant conservation $\nabla_\mu T^{\mu 0} = 0$ gives the fluid equation $\dot{\rho} + 3H(\rho + p/c^2) = 0$. Verify this is not an independent equation — it follows from the two Friedmann equations.

---

**47.3.** *Cosmological redshift and distance measures.*

(a) Show that a photon emitted at time $t_{\rm em}$ with wavelength $\lambda_{\rm em}$ is received at $t_0$ with $\lambda_{\rm obs}/\lambda_{\rm em} = a(t_0)/a(t_{\rm em}) = 1 + z$.

(b) Define the luminosity distance $d_L$ and angular diameter distance $d_A$ in terms of the comoving distance $\chi$ and redshift $z$:
$$d_L = (1+z)\chi, \quad d_A = \frac{\chi}{1+z}$$
Show that $d_L = (1+z)^2 d_A$ (the Etherington reciprocity relation).

(c) For a flat universe with matter and cosmological constant ($\Omega_m = 0.3$, $\Omega_\Lambda = 0.7$), compute the luminosity distance to $z = 0.1$, $z = 0.5$, and $z = 1$ numerically. How does the result compare to the Euclidean estimate $d = cz/H_0$?

(d) A Type Ia supernova has peak absolute magnitude $M = -19.3$. It is observed with apparent magnitude $m = 24.1$. Using the distance modulus $\mu = m - M = 5\log_{10}(d_L/10\text{ pc})$, find $d_L$. What redshift does this correspond to for a flat $\Lambda$CDM universe?

---

**47.4.** *Cosmic epochs and the age of the universe.*

(a) For each component of the cosmic energy budget, find how $\rho$ scales with $a$ by integrating the fluid equation $\dot{\rho} + 3H(\rho + p/c^2) = 0$ using equation of state $p = w\rho c^2$:
- Radiation: $w = 1/3$ → $\rho \propto a^{-4}$
- Matter: $w = 0$ → $\rho \propto a^{-3}$
- Cosmological constant: $w = -1$ → $\rho = \text{const}$

(b) Show that for matter domination ($k = 0$, $\Lambda = 0$): $a(t) \propto t^{2/3}$. For radiation domination: $a(t) \propto t^{1/2}$. For $\Lambda$ domination: $a(t) \propto e^{Ht}$ (de Sitter).

(c) Find the redshift $z_{\rm eq}$ of matter-radiation equality. Using $\Omega_r h^2 = 4.15\times 10^{-5}$ and $\Omega_m h^2 = 0.143$ (where $h = H_0/100$ km/s/Mpc), compute $z_{\rm eq}$.

(d) The age of a flat $\Lambda$CDM universe is:
$$t_0 = \frac{1}{H_0}\int_0^1 \frac{da}{\sqrt{\Omega_m/a + \Omega_\Lambda a^2}}$$
For $H_0 = 67.4$ km/s/Mpc, $\Omega_m = 0.315$, $\Omega_\Lambda = 0.685$: compute $t_0$ numerically. Compare to the inverse Hubble time $1/H_0$.

---

## Thought Experiments

**T47.1.** *What if the universe were closed?*

The flatness of the observed universe ($|\Omega_k| < 0.001$) is either a fundamental fact or a consequence of inflation (which drives $\Omega_k \to 0$ exponentially). Suppose instead the universe had $k = +1$ (positive spatial curvature) with $\Omega_k = -0.01$.

(a) What would be the radius of curvature of the universe in this case?
(b) Would the universe eventually recollapse (a "Big Crunch")? What determines whether a curved universe recollapses, given that we also have dark energy $\Omega_\Lambda \neq 0$?
(c) Could we in principle detect the spatial curvature directly — for example, by measuring the angles of an enormous triangle? What would the triangles have to be?

**T47.2.** *The cosmological constant problem.*

The cosmological constant has a measured value $\Lambda = 1.11\times 10^{-52}$ m$^{-2}$, corresponding to an energy density $\rho_\Lambda = \Lambda c^4/(8\pi G) \approx 5.4\times 10^{-10}$ J/m³. In quantum field theory, the vacuum energy density is expected to be of order $(M_{\rm Pl}c^2)^4/(\hbar c)^3 \approx 10^{113}$ J/m³ — a discrepancy of 123 orders of magnitude.

This is frequently called "the worst prediction in physics." Some approaches: fine-tuning (the vacuum energy is whatever it needs to be), anthropic selection (only universes with small $\Lambda$ allow structure to form and observers to exist), dynamic dark energy (a rolling scalar field whose energy today happens to be small), or new symmetry principles.

Can you construct any argument that makes the cosmological constant problem seem less severe? What would it take for a physical mechanism to explain $\Lambda$?

---

## Laboratory Exercise

**L47.1.** *Numerically solving the Friedmann equations and computing cosmic history.*

Using Python, integrate the Friedmann equation:
$$\frac{da}{dt} = H_0 a\sqrt{\Omega_r a^{-4} + \Omega_m a^{-3} + \Omega_k a^{-2} + \Omega_\Lambda}$$

with parameters $H_0 = 67.4$ km/s/Mpc, $\Omega_r = 9.2\times 10^{-5}$, $\Omega_m = 0.315$, $\Omega_k = 0$, $\Omega_\Lambda = 0.685$, and initial condition $a(t_0) = 1$ (integrating backward in time to $a = 0$).

**Task 1 (Cosmic time):** Plot $a(t)$ from $t = 0$ to $t = 30$ Gyr. Mark the epochs of radiation-matter equality, matter-$\Lambda$ equality, and today.

**Task 2 (Conformal time):** Compute and plot the conformal time $\eta = \int_0^t dt'/a(t')$ as a function of $t$. The comoving particle horizon is $c\eta$.

**Task 3 (Hubble diagram):** Compute the luminosity distance $d_L(z)$ for $z = 0$ to $z = 2$ and plot the Hubble diagram (distance modulus vs. redshift). Overlay the Perlmutter et al. (1999) supernova data (available in their paper or online).

**Task 4 (Deceleration):** Plot the deceleration parameter $q = -\ddot{a}a/\dot{a}^2 = -1 - \dot{H}/H^2$. Show that $q$ transitions from positive (decelerating) to negative (accelerating) at $z_{\rm acc} \approx 0.65$, corresponding to $t_{\rm acc} \approx 7.3$ Gyr.
