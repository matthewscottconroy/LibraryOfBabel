# Section 34.3: Consequences of the Equivalence Principle

---

## What the Equivalence Principle Tells Us (Before We Solve a Single Equation)

The equivalence principle is remarkable because it allows us to derive physical predictions — tested predictions that agree with experiment — *before* writing down the Einstein field equations or solving any differential equations. The principle alone, combined with SR and basic energy conservation arguments, implies:

1. Light bends in a gravitational field.
2. Clocks run slower in a gravitational potential.
3. The gravitational redshift.
4. The Shapiro time delay (roughly speaking).
5. The universality of gravitational coupling — everything is affected by gravity, including electromagnetic radiation.

These are the "soft" consequences. The "hard" consequences — the precise bending angle, the Schwarzschild metric, the exact field equations — require the full machinery of GR. But the equivalence principle alone already tells us a great deal.

---

## Light Bending: The Equivalence-Principle Calculation

In a freely falling elevator of height $H$, a light beam enters through a hole in the wall at height $h$ above the floor, traveling horizontally. By the equivalence principle, in the elevator frame, the light travels in a straight line. It takes time $t = W/c$ to cross the elevator of width $W$.

But in the Earth's rest frame, during the time $t = W/c$, the elevator has fallen a distance:
$$\delta = \frac{1}{2}gt^2 = \frac{1}{2}g\frac{W^2}{c^2}$$
downward. So the light beam, which exits at the same height in the elevator frame, exits at height $h - \delta$ in the Earth frame. It appears to have curved downward by $\delta = gW^2/(2c^2)$.

The angular deflection is:
$$\delta\theta \approx \frac{\delta}{W} \cdot \frac{W}{c} \cdot g \cdot \frac{1}{c} = \frac{gW}{c^2}$$
This is the Newtonian (equivalence-principle) result. For a light ray passing the Sun at impact parameter $b = R_\odot$:
$$\delta\theta_{\rm EP} = \frac{2GM_\odot}{R_\odot c^2} \approx 0.875''$$
The full GR result is twice this: $\delta\theta_{\rm GR} = 4GM_\odot/(R_\odot c^2) \approx 1.75''$.

The factor of 2 discrepancy arises because the equivalence-principle calculation only accounts for the bending of the spatial path. In full GR, there is also a bending of the time dimension: the metric has both $g_{00} \neq 1$ (gravitational time dilation) and $g_{rr} \neq 1$ (spatial curvature), each contributing equally to the deflection of null geodesics.

The 1919 Eddington expedition measured the deflection of starlight during a solar eclipse and obtained $1.61'' \pm 0.30''$ (consistent with GR, not the EP-only prediction). Modern radio astrometry measurements agree with GR to better than $10^{-4}$.

---

## Gravitational Redshift: The Precise Statement

A clock at lower gravitational potential ticks slower than a clock at higher potential. This follows from the equivalence principle as follows.

Consider two observers, Alice at height $z = 0$ and Bob at height $z = H$, in a uniform gravitational field $g$. Alice sends light pulses upward to Bob at a fixed frequency $f_A$ (as measured by Alice's clock). The time between pulses at Alice's location is $\Delta t_A = 1/f_A$.

In the EP-equivalent picture: Bob is in an accelerating elevator with acceleration $g$. Between the time Alice emits and Bob receives a pulse, Bob has accelerated. By the time the second pulse reaches Bob (traveling a distance $H$ at speed $c$, taking time $H/c$), Bob's velocity has increased by $\Delta v = g \cdot (H/c)$. By the Doppler formula, the observed frequency is:
$$f_B = f_A\left(1 - \frac{\Delta v}{c}\right) = f_A\left(1 - \frac{gH}{c^2}\right)$$
So Bob observes a lower frequency: the light is redshifted. The fractional redshift is:
$$\frac{f_A - f_B}{f_A} = \frac{gH}{c^2} = \frac{\Delta\Phi}{c^2}$$
where $\Delta\Phi = gH$ is the change in Newtonian gravitational potential.

This is the **gravitational redshift** formula. It can be stated as a clock rate: a clock at potential $\Phi$ runs at rate proportional to $\sqrt{1 + 2\Phi/c^2} \approx 1 + \Phi/c^2$ (for $|\Phi| \ll c^2$) relative to a clock at potential $\Phi = 0$. A clock deeper in a gravitational well runs slower — gravitational time dilation.

The exact GR formula (Schwarzschild geometry):
$$\frac{f_{\rm obs}}{f_{\rm emit}} = \sqrt{\frac{1 - r_s/r_{\rm emit}}{1 - r_s/r_{\rm obs}}}$$
where $r_s = 2GM/c^2$ is the Schwarzschild radius.

**Experimental confirmations:**
- Pound-Rebka (1959): gamma rays rising 22.5 m in Earth's gravity, confirmed to 10%
- Pound-Snider (1965): improved to 1%
- Gravity Probe A (1976): hydrogen maser at 10,000 km altitude, confirmed to 0.02%
- GP-B, NIST, and Chou et al. (2010): optical atomic clocks separated vertically by 33 cm showed the predicted gravitational redshift — the most sensitive test of gravitational time dilation ever performed

---

## The Shapiro Delay

The equivalence principle also implies that light travels more slowly in a gravitational field — or equivalently, that a gravitational well adds extra path length to light travel. This is the **Shapiro time delay** (Shapiro 1964).

The basic idea: in the freely-falling frame, light travels at $c$. In the fixed-to-the-Earth frame, the free-fall frame is accelerating, which (by the EP) is equivalent to being in a gravitational field. The accumulated effect of this is that light passing through a gravitational well takes longer to travel between two points than it would in flat spacetime.

For a signal passing the Sun at closest approach $b$, the total delay (compared to flat-space travel time) is:
$$\Delta t_{\rm Shapiro} = \frac{2GM_\odot}{c^3}\ln\left(\frac{4r_E r_P}{b^2}\right)$$
where $r_E$ and $r_P$ are the distances from the Sun to Earth and to the reflecting body (planet or spacecraft). For a signal to Venus at superior conjunction: $\Delta t \sim 250$ μs.

Measured by Shapiro et al. (1971) using radar ranging to Venus: confirmed to 3%. The Cassini spacecraft (Bertotti et al. 2003) confirmed it to $2.3\times 10^{-5}$ — the most precise test of GR's weak-field predictions.

---

## Minimal Coupling and the "Comma-Goes-to-Semicolon" Rule

The equivalence principle provides a systematic prescription for writing physical laws in curved spacetime. Starting from any law of special relativity, the **minimal coupling prescription** is:

1. Write the equation in manifestly covariant (tensorial) form using the Minkowski metric $\eta_{\mu\nu}$ and ordinary partial derivatives $\partial_\mu$.
2. Replace $\eta_{\mu\nu} \to g_{\mu\nu}$ (the curved spacetime metric).
3. Replace $\partial_\mu \to \nabla_\mu$ (partial derivatives become covariant derivatives).
4. Replace $d^4x \to \sqrt{-g}\,d^4x$ in any integral.

This is colloquially called the **"comma-goes-to-semicolon" rule**: in index notation, partial derivatives are denoted $T_{\mu\nu,\rho}$ (comma notation) and covariant derivatives are $T_{\mu\nu;\rho}$ (semicolon notation). The rule says: make every comma a semicolon.

**Examples of minimal coupling:**

*Charge conservation:* SR: $\partial_\mu J^\mu = 0$. Curved: $\nabla_\mu J^\mu = \frac{1}{\sqrt{-g}}\partial_\mu(\sqrt{-g}J^\mu) = 0$.

*Maxwell's equations:* SR: $\partial_\mu F^{\mu\nu} = \mu_0 J^\nu$. Curved: $\nabla_\mu F^{\mu\nu} = \mu_0 J^\nu$ (the homogeneous equation $\partial_{[\rho}F_{\mu\nu]} = 0$ is unchanged since torsion is zero).

*Klein-Gordon equation:* SR: $(\Box + m^2)\phi = 0$ where $\Box = \eta^{\mu\nu}\partial_\mu\partial_\nu$. Curved: $g^{\mu\nu}\nabla_\mu\nabla_\nu\phi - m^2\phi = 0$. Note: there is an ambiguity — one could add $\xi R\phi$ for any constant $\xi$ (a non-minimal coupling). Minimal coupling sets $\xi = 0$.

*Energy-momentum conservation:* SR: $\partial_\mu T^{\mu\nu} = 0$. Curved: $\nabla_\mu T^{\mu\nu} = 0$. This is NOT minimal coupling in the usual sense — it follows from the Einstein equations plus the Bianchi identity, not from an external imposition.

The minimal coupling prescription is not guaranteed to be correct — it gives the *simplest* curved-space generalization, but there can be non-minimal couplings (to the Ricci scalar, the Weyl tensor, etc.) that are also consistent. These are constrained by experiment.

---

## Where the Equivalence Principle Does Not Apply

The equivalence principle has limits:

**Tidal forces.** Over a large enough region, tidal forces (second-order effects in the distance from the freely-falling center) are measurable and cannot be removed. These are the Riemann curvature components.

**Self-gravitating bodies.** For a body whose own gravitational field contributes significantly to its dynamics (a neutron star, a black hole), the WEP applies but the SEP may not — in alternative theories of gravity, the internal gravitational energy can cause different accelerations. In GR, SEP holds.

**Quantum gravity scale.** Near the Planck length $\ell_P = \sqrt{\hbar G/c^3} \sim 10^{-35}$ m, quantum gravitational effects are expected to become important and the classical equivalence principle may break down. No experiment has approached this scale.

**Spin.** A spinning test particle does not follow a geodesic; it follows the Mathisson-Papapetrou-Dixon equations, which include a coupling of spin to the Riemann tensor (the Papapetrou force). However, for astrophysically realistic spin parameters of ordinary matter, this effect is negligible compared to geodesic motion.

