# Section 47.2: The Friedmann Equations

---

## The Einstein Equations for the FLRW Universe

The FLRW metric encodes the geometry; the Friedmann equations encode the dynamics — how $a(t)$ evolves given the energy content. They are simply the Einstein field equations applied to the FLRW metric with a perfect fluid stress-energy tensor.

The stress-energy tensor for a homogeneous, isotropic perfect fluid at rest in comoving coordinates is:
$$T^{\mu\nu} = \begin{pmatrix}\rho c^2 & 0 & 0 & 0\\ 0 & p & 0 & 0\\ 0 & 0 & p & 0\\ 0 & 0 & 0 & p\end{pmatrix}$$

where $\rho = \rho(t)$ is the energy density and $p = p(t)$ is the isotropic pressure (both functions of time only, by homogeneity).

For the FLRW metric $ds^2 = -c^2dt^2 + a^2(t)\gamma_{ij}dx^idx^j$, the nonzero Christoffel symbols are:
$$\Gamma^0_{ij} = \frac{a\dot{a}}{c^2}\gamma_{ij}, \quad \Gamma^i_{0j} = \frac{\dot{a}}{a}\delta^i_j, \quad \Gamma^i_{jk} = {}^{(3)}\Gamma^i_{jk}$$

where ${}^{(3)}\Gamma^i_{jk}$ are the 3D Christoffel symbols of the spatial metric $\gamma_{ij}$.

Computing the Ricci tensor:
$$R_{00} = -\frac{3\ddot{a}}{ac^2}$$
$$R_{ij} = \left[-\frac{\ddot{a}}{ac^2} - \frac{2\dot{a}^2}{a^2c^2} - \frac{2k}{a^2}\right]a^2\gamma_{ij}$$

The Ricci scalar:
$$R = -\frac{6}{c^2}\left[\frac{\ddot{a}}{a} + \left(\frac{\dot{a}}{a}\right)^2 + \frac{kc^2}{a^2}\right]$$

Substituting into $G_{\mu\nu} + \Lambda g_{\mu\nu} = 8\pi G T_{\mu\nu}/c^4$:

**From the $00$ component:**
$$\boxed{H^2 \equiv \left(\frac{\dot{a}}{a}\right)^2 = \frac{8\pi G\rho}{3} - \frac{kc^2}{a^2} + \frac{\Lambda c^2}{3}}$$

**From the $ij$ component:**
$$\boxed{\frac{\ddot{a}}{a} = -\frac{4\pi G}{3}\left(\rho + \frac{3p}{c^2}\right) + \frac{\Lambda c^2}{3}}$$

These are the **Friedmann equations**. Together with an equation of state $p = p(\rho)$, they fully determine the expansion history of the universe.

---

## Physical Interpretation of the Friedmann Equations

**The first Friedmann equation** is an energy equation. Multiply both sides by $\frac{1}{2}m a^2$ (where $m$ is any mass):
$$\frac{1}{2}m\dot{a}^2 = \frac{4\pi G\rho m a^2}{3} - \frac{kmc^2}{2} + \frac{\Lambda c^2 m a^2}{6}$$

The left side is kinetic energy. The first term on the right is the Newtonian gravitational potential energy (for a uniform sphere of density $\rho$ and radius $a$). The $k$ term acts like a total energy — positive $k$ gives negative energy (bound, eventually recollapses in the absence of $\Lambda$); negative $k$ gives positive energy (unbound, expands forever). The $\Lambda$ term acts like kinetic energy from a repulsive force.

This Newtonian analogy is not accidental — the first Friedmann equation can be derived from Newtonian gravity plus the covariant conservation of energy, without any relativistic machinery. But the Newtonian derivation is incomplete: it does not explain the curvature term $k$ or the cosmological constant, and it breaks down for pressure-dominated fluids.

**The second Friedmann equation** is a force equation. If only matter were present ($p = 0$, $\Lambda = 0$), then $\ddot{a} < 0$ always — gravity decelerates the expansion. Radiation ($p = \rho c^2/3$) decelerates even faster: the effective source is $\rho + 3p/c^2 = 2\rho > \rho$. A cosmological constant ($\Lambda > 0$) can make $\ddot{a} > 0$ if $\Lambda c^2/3 > 4\pi G(\rho + 3p/c^2)/3$ — this is the condition for an accelerating universe.

**The fluid equation.** Covariant energy conservation $\nabla_\mu T^{\mu\nu} = 0$ gives:
$$\dot{\rho} + 3H\left(\rho + \frac{p}{c^2}\right) = 0 \quad \Leftrightarrow \quad \frac{d}{dt}(\rho a^3) = -\frac{p}{c^2}\frac{d(a^3)}{dt}$$

This is the thermodynamic statement that the work done by pressure equals the change in internal energy: $dU = -p\,dV$. The "volume" is $a^3$ and the "internal energy" is $\rho c^2 a^3$.

The fluid equation is not independent — it follows from differentiating the first Friedmann equation and using the second. But it is useful to solve it first (given an equation of state) to find $\rho(a)$, then substitute into the first Friedmann equation to solve for $a(t)$.

---

## Equations of State and Energy Components

Different forms of energy obey different equations of state $p = w\rho c^2$:

**Matter (dust)**: $w = 0$, $p = 0$. The fluid equation gives $\dot{\rho}_m + 3H\rho_m = 0$, so $\rho_m \propto a^{-3}$. Number density dilutes as $\sim 1/V \propto a^{-3}$; no energy loss per particle.

**Radiation**: $w = 1/3$, $p = \rho c^2/3$. The fluid equation gives $\rho_r \propto a^{-4}$. Number density dilutes as $a^{-3}$; photon energy redshifts as $a^{-1}$, giving an extra factor of $a^{-1}$.

**Cosmological constant**: $w = -1$, $p = -\rho_\Lambda c^2$. The fluid equation gives $\dot{\rho}_\Lambda = 0$ — constant energy density. As the volume expands, the total dark energy grows as $a^3$. This violates intuition but is thermodynamically consistent: the negative pressure does work on the expanding universe.

**Spatial curvature** enters the Friedmann equation as an effective component with $\rho_k \equiv -3kc^2/(8\pi G a^2) \propto a^{-2}$, equivalent to $w = -1/3$.

**Stiff fluid**: $w = 1$, $\rho \propto a^{-6}$. Arises in some models of the very early universe.

| Component | $w$ | $\rho(a)$ | $a(t)$ (flat, dominated) |
|-----------|-----|---------|------------------------|
| Radiation | $1/3$ | $a^{-4}$ | $t^{1/2}$ |
| Matter | $0$ | $a^{-3}$ | $t^{2/3}$ |
| Curvature | $-1/3$ | $a^{-2}$ | $t$ |
| Dark energy | $-1$ | const | $e^{Ht}$ (de Sitter) |

---

## The Critical Density and $\Omega$ Parameters

Setting $k = 0$ and $\Lambda = 0$ in the first Friedmann equation gives the **critical density** — the energy density required for a flat universe:
$$\rho_c = \frac{3H^2}{8\pi G}$$

Today: $\rho_{c,0} = 3H_0^2/(8\pi G) \approx 8.62\times 10^{-27}$ kg/m³ $\approx 5.4\times 10^{-10}$ J/m³. This is extraordinarily dilute — about 5 hydrogen atoms per cubic meter, or 3–4 protons per $\text{m}^3$ of matter.

The **density parameters** normalize each component to the critical density:
$$\Omega_i = \frac{\rho_i}{\rho_c} = \frac{8\pi G\rho_i}{3H^2}$$

The first Friedmann equation becomes:
$$1 = \Omega_m + \Omega_r + \Omega_\Lambda + \Omega_k \quad \text{where } \Omega_k = -\frac{kc^2}{H^2}$$

**Present values (Planck 2018)**:
- $\Omega_\Lambda = 0.6847 \pm 0.0073$ (dark energy)
- $\Omega_m = 0.3153 \pm 0.0073$ (total matter)
  - $\Omega_b = 0.0493 \pm 0.0006$ (baryons — atoms)
  - $\Omega_{\rm DM} = 0.266 \pm 0.007$ (dark matter — unknown particle)
- $\Omega_r = 9.2\times 10^{-5}$ (radiation: photons + neutrinos)
- $\Omega_k = 0.0007 \pm 0.0019$ (spatial curvature — consistent with flat)

The universe is 68% dark energy, 27% dark matter, 5% ordinary matter. Everything we've ever directly observed — stars, planets, gas, dust — is less than 5% of the energy content of the universe.

---

## Expansion History: The Standard $\Lambda$CDM Model

The standard cosmological model is $\Lambda$CDM: cold dark matter ($\Lambda$CDM) plus cosmological constant. The first Friedmann equation in terms of redshift $z$ (with $a = 1/(1+z)$):
$$H(z) = H_0\sqrt{\Omega_r(1+z)^4 + \Omega_m(1+z)^3 + \Omega_k(1+z)^2 + \Omega_\Lambda}$$

**Radiation-matter equality**: $\Omega_r(1+z_{\rm eq})^4 = \Omega_m(1+z_{\rm eq})^3$, giving:
$$1 + z_{\rm eq} = \frac{\Omega_m}{\Omega_r} \approx \frac{0.315}{9.2\times 10^{-5}} \approx 3400$$

So the universe was radiation-dominated for $z > 3400$ (i.e., $T > 9000$ K), and matter-dominated for $3400 > z > 0.3$.

**Matter-$\Lambda$ equality**: $\Omega_m(1+z_{\rm acc})^3 = 2\Omega_\Lambda$ (the condition for $\ddot{a} = 0$) gives:
$$1 + z_{\rm acc} = \left(\frac{2\Omega_\Lambda}{\Omega_m}\right)^{1/3} \approx \left(\frac{2\times 0.685}{0.315}\right)^{1/3} \approx 1.65 \implies z_{\rm acc} \approx 0.65$$

The universe began accelerating at $z \approx 0.65$ (about 6 billion years ago, when the universe was roughly half its current age).

**Age of the universe**:
$$t_0 = \frac{1}{H_0}\int_0^1\frac{da}{\sqrt{\Omega_r a^{-2} + \Omega_m a^{-1} + \Omega_\Lambda a^2 + \Omega_k}}$$

Numerically: $t_0 = 13.787 \pm 0.020$ Gyr (Planck 2018). The inverse Hubble time gives $H_0^{-1} = 14.5$ Gyr, slightly larger because the universe has been decelerating for most of its history.

---

## Cosmic Distances

In an expanding universe, there are several distinct notions of distance:

**Comoving distance** $\chi$: the coordinate separation, independent of $a$. For a flat universe:
$$\chi(z) = \frac{c}{H_0}\int_0^z\frac{dz'}{E(z')}$$
where $E(z) = H(z)/H_0$.

**Proper distance** at time $t$: $d(t) = a(t)\chi$.

**Luminosity distance**: defined so that flux $\propto 1/d_L^2$:
$$d_L = (1+z)\chi$$
(For $z \ll 1$: $d_L \approx cz/H_0$, Hubble's law.)

**Angular diameter distance**: defined so that angular size $\theta = \ell/d_A$:
$$d_A = \frac{\chi}{1+z}$$

**Relation**: $d_L = (1+z)^2 d_A$ — the **Etherington reciprocity relation**, a fundamental consequence of photon number conservation.

Type Ia supernovae (standard candles) measure $d_L$; galaxy angular sizes and CMB acoustic peaks measure $d_A$. The combination constrains $\Omega_\Lambda$ — the discovery of acceleration from SNe Ia in 1998 measured $d_L$ for $z \sim 0.5$–$1$ and found it was larger than expected without $\Lambda$.

---

## The Deceleration Parameter and Expansion History

The Taylor expansion of $a(t)$ around the present time:
$$a(t) = a_0\left[1 + H_0(t-t_0) - \frac{1}{2}q_0 H_0^2(t-t_0)^2 + \cdots\right]$$

defines the **deceleration parameter**:
$$q = -\frac{\ddot{a}a}{\dot{a}^2} = -1 - \frac{\dot{H}}{H^2}$$

For $\Lambda$CDM: $q_0 = \Omega_m/2 - \Omega_\Lambda \approx 0.315/2 - 0.685 \approx -0.53$ — a negative value, confirming the current acceleration.

**History of $q$:**
- During radiation domination: $q = 1$ (strong deceleration)
- During matter domination: $q = 1/2$
- At matter-$\Lambda$ equality: $q = 0$ (inflection point)
- Today: $q_0 \approx -0.53$
- Far future ($a\to\infty$): $q \to -1$ (de Sitter-like)

The measurement of $q_0 < 0$ by Perlmutter and Riess (1998) using Type Ia supernovae was the discovery of dark energy.

---

## The Far Future

In a universe dominated by a positive cosmological constant, the long-term fate is exponential expansion:
$$a(t) \to e^{H_\Lambda t}, \quad H_\Lambda = c\sqrt{\Lambda/3}$$

All galaxies beyond the Milky Way will eventually recede beyond the event horizon and become unreachable and unobservable. In $\sim 150$ billion years, only the Local Group (Milky Way + Andromeda + $\sim 50$ dwarf galaxies) will be visible; the rest of the universe will have been redshifted beyond detection.

In $\sim 2$–$5$ trillion years, the Local Group galaxies will merge into a single elliptical galaxy. Star formation will cease as gas is exhausted. The stars will burn out in $\sim 10^{14}$ years. Black holes will evaporate via Hawking radiation in $\sim 10^{67}$–$10^{100}$ years.

The cosmological constant ensures the universe never recollapses — unless dark energy is dynamical and $w \neq -1$ exactly, which remains an open question.
