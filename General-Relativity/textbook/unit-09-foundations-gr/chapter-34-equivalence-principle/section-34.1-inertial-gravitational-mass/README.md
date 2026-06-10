# Section 34.1: Inertial and Gravitational Mass

---

## The Two Masses

Newton's mechanics requires two entirely distinct concepts of mass, and it is not obvious they are the same.

**Inertial mass** $m_i$ appears in Newton's second law:
$$\mathbf{F} = m_i\mathbf{a}$$
It measures resistance to acceleration. Apply the same force $\mathbf{F}$ to two objects; the one with larger $m_i$ accelerates less. Inertial mass is the dynamical "quantity of matter" — defined purely in terms of the relationship between force and acceleration. To measure it, you push something and observe how it responds.

**Gravitational mass** $m_g$ appears in Newton's law of universal gravitation:
$$\mathbf{F} = -\frac{Gm_g M_g}{r^2}\hat{r}$$
It measures response to a gravitational field. Two objects with the same position and different $m_g$ experience different gravitational forces. Gravitational mass is the gravitational "charge" — defined purely in terms of the gravitational force. To measure it, you compare gravitational forces.

There is no logical reason why $m_i$ and $m_g$ should be related. Gravitational force could depend on composition (like electric force depends on electric charge), in which case different materials would fall at different rates. Newton recognized this:

> *"Hitherto I have not been able to discover the cause of those properties of gravity from phenomena, and I frame no hypotheses... It is enough that gravity does really exist, and act according to the laws which we have explained, and abundantly serves to account for all the motions of the celestial bodies."* — Newton, *Principia*, General Scholium

Newton's own experiments with pendulums of different materials showed $m_i/m_g$ is constant to better than 1 part in 1000. But he had no explanation for why.

---

## Eötvös and the Modern Experiments

The equality $m_i = m_g$ is called the **Weak Equivalence Principle (WEP)** or the **Universality of Free Fall (UFF)**. Testing it precisely is one of the oldest programs in experimental physics.

**Galileo** (ca. 1590–1604): Inclined plane experiments and (possibly) drop experiments showed all materials fall with the same acceleration $g$ to about 1 part in $10^2$.

**Newton** (1687): Pendulum experiments comparing materials of different density and composition: gold, silver, lead, glass, sand, salt, wood, water, wheat. Equal-length pendulums with different bob compositions have the same period if and only if $m_i/m_g$ is the same for all materials. Newton confirmed equality to about 1 part in $10^3$.

**Eötvös** (1890–1909): Roland von Eötvös used a torsion balance — two test masses on a horizontal bar suspended by a thin wire. If the two masses have different ratios $m_i/m_g$, the gravitational force and the centrifugal force (from Earth's rotation) will not balance the same way for the two masses, and the bar will rotate. Eötvös found no rotation, establishing $m_i = m_g$ to 1 part in $10^9$.

The result is parameterized by the **Eötvös parameter**:
$$\eta(A, B) = 2\frac{|a_A - a_B|}{|a_A + a_B|}$$
where $a_A$ and $a_B$ are the free-fall accelerations of materials $A$ and $B$. If $m_i \propto m_g$ universally, $\eta = 0$.

**Braginsky and Panov** (1971): Used an improved torsion balance with aluminum and platinum, taking advantage of Earth's orbital acceleration around the Sun as the driving signal rather than Earth's rotation. Confirmed $\eta < 10^{-12}$.

**MICROSCOPE mission** (2017–2018): A satellite experiment (CNES/ESA) measuring the differential acceleration of two coaxial cylindrical test masses in free fall in orbit. Confirmed $|\eta| < 10^{-15}$ — the best test to date.

**Lunar Laser Ranging**: The Moon and Earth as a whole are in free fall around the Sun. By measuring the Earth-Moon distance to millimeter precision using retroreflectors left by Apollo, the Nordtvedt effect (a violation of SEP) has been constrained to $\eta_{\rm Nordtvedt} < 10^{-13}$.

---

## Why It Must Be Exact

The extraordinary precision of these experiments is evidence that the equality $m_i = m_g$ is not an approximate coincidence but an exact identity. If it were slightly violated, we would need a theory in which different materials fall at slightly different rates, which would require a new force coupling to composition (a "fifth force"). No such force has ever been observed.

In GR, the equality is not a postulate — it is a theorem. The geodesic equation of motion $\ddot{x}^\mu + \Gamma^\mu_{\nu\rho}\dot{x}^\nu\dot{x}^\rho = 0$ is a statement about geometry; it contains no mass at all. Every freely falling object, regardless of its mass or composition, follows the same geodesic. This is the geometric expression of the equality $m_i = m_g$.

More precisely: the motion of a test body in GR depends on the metric of spacetime (through the Christoffel symbols), but not on the properties of the body itself. This is the strong version of the equivalence principle. The body's stress-energy tensor $T^{\mu\nu}$ curves spacetime (the Einstein equations), but its motion through that curved spacetime is then determined by the metric alone.

---

## From WEP to Einstein's Elevator

The WEP ($m_i = m_g$) implies the following: **in a uniform gravitational field, all freely falling observers experience no gravity**. This is because, in a freely falling frame, the acceleration of every object (relative to the frame) is exactly zero — the gravitational acceleration cancels the "acceleration" of the frame itself.

Einstein called this the "happiest thought of my life" (in 1907): if a man falls freely, he will not feel his own weight. For a freely falling observer, gravity vanishes. This is not an approximation — it is exact for a uniform field. For a non-uniform field (which is the realistic case), the field is approximately uniform over small enough regions, and the residual effects (tidal forces) are a signal of genuine curvature that cannot be eliminated.

This thought experiment was Einstein's entry point into general relativity. The freely falling frame is the locally inertial frame; it obeys the physics of special relativity. The fixed-to-the-Earth frame is the *accelerating* frame; in it, gravity appears as a pseudoforce. The curvature of spacetime is the tidal effect that remains when you go to the freely falling frame.

---

## The Three Forms of the Equivalence Principle

There are three commonly distinguished forms of the equivalence principle, of increasing strength:

**Weak Equivalence Principle (WEP):** The trajectory of a freely falling test body is independent of its mass and internal composition. Equivalently: $m_i = m_g$ universally. Tested to $10^{-15}$.

**Einstein Equivalence Principle (EEP):** The WEP holds, and the outcome of any local non-gravitational experiment is independent of the velocity of the freely falling reference frame (Local Lorentz Invariance, LLI) and of where and when in the universe it is performed (Local Position Invariance, LPI). This implies that the laws of physics in a sufficiently small freely-falling laboratory are those of special relativity.

**Strong Equivalence Principle (SEP):** The WEP holds for self-gravitating bodies as well as test bodies, and local measurements of both gravitational and non-gravitational physics are independent of the state of motion and location of the laboratory. The SEP implies that the gravitational constant $G$ does not vary with position or time, and that gravitating bodies fall with the same acceleration as non-gravitating ones (the Nordtvedt effect is absent).

GR satisfies all three. Most alternative theories of gravity (Brans-Dicke, $f(R)$, etc.) satisfy WEP and EEP but violate SEP to some degree.

---

## Mathematical Expression: Normal Coordinates

The EEP has a precise mathematical expression in GR. At any event $p$ in spacetime, there exist coordinates — **normal coordinates** or **Riemann normal coordinates** — in which:
$$g_{\mu\nu}(p) = \eta_{\mu\nu}, \quad \Gamma^\rho_{\mu\nu}(p) = 0$$
The metric is Minkowskian and the connection vanishes at $p$. This is the statement that at any point, you can choose coordinates that are locally inertial — a freely falling frame.

The curvature (second derivatives of the metric) cannot in general be made to vanish at $p$:
$$\partial_\rho\Gamma^\sigma_{\mu\nu}(p) = -\frac{1}{3}(R^\sigma_{\ \mu\rho\nu}(p) + R^\sigma_{\ \nu\rho\mu}(p)) \neq 0 \text{ in general}$$
These are the tidal terms — the unmistakable signature of gravity that no change of coordinates can eliminate.

The size of the locally inertial region is:
$$\ell \sim \min\left(\sqrt{\frac{c^2}{|R^\mu_{\ \nu\rho\sigma}|}}\right)$$
For the Schwarzschild metric near Earth: $\ell \sim c/\sqrt{g/R_\oplus} \sim 100$ km for 1% accuracy. Near a neutron star surface: $\ell \sim$ meters. Near a stellar-mass black hole horizon: $\ell \sim GM/c^2 \sim$ km.

---

## Light and the Equivalence Principle

The EEP immediately implies that **light must be deflected by gravity**. Here is the argument:

Consider a freely falling elevator. Inside, by the EEP, the laws of physics are those of SR. In SR, light travels in straight lines. Therefore, in the freely falling elevator, a light beam travels in a straight line.

Now boost to the fixed-to-the-Earth frame. The elevator is falling downward, so the "straight line" in the elevator frame appears curved downward — a parabolic path — in the fixed frame. Light curves downward in the gravitational field.

This is the same deflection that any other freely falling particle would experience. The equivalence principle says light falls like everything else.

The magnitude of the deflection can be estimated from the Newtonian analogy (treating the photon as a particle with effective mass $m = E/c^2 = hf/c^2$): $\delta\theta_{\rm Newton} = 2GM/(bc^2)$ where $b$ is the impact parameter. The full GR calculation gives **twice** this: $\delta\theta_{\rm GR} = 4GM/(bc^2)$. The factor of 2 comes from the bending of time as well as the bending of space — the equivalence principle gets the "spatial" part right but misses the "temporal" part, which requires the full field equations.

For the Sun: $\delta\theta_{\rm GR} = 1.75''$ at the solar limb. Measured by Eddington in 1919. The modern value, from radio astronomy, agrees to $10^{-4}$.

---

## Gravitational Redshift

The EEP also implies **gravitational redshift**: a photon moving upward in a gravitational field loses energy, which means its frequency decreases.

Argument from energy conservation: A photon of energy $E = hf$ at the bottom of a tower of height $h$ has effective mass $m = E/c^2$. Lifting this mass to height $h$ costs energy $mgh = Egh/c^2$. At the top, the photon has energy $E' = E - Egh/c^2 = E(1 - gh/c^2)$. Therefore $f' = f(1 - gh/c^2) < f$: the photon is redshifted.

The exact GR result (from the Schwarzschild metric) is:
$$\frac{f_{\rm obs}}{f_{\rm emit}} = \sqrt{\frac{g_{00}(r_{\rm emit})}{g_{00}(r_{\rm obs})}} = \sqrt{\frac{1 - 2GM/(r_{\rm emit}c^2)}{1 - 2GM/(r_{\rm obs}c^2)}}$$
For a weak field ($GM \ll rc^2$): $f_{\rm obs}/f_{\rm emit} \approx 1 - g\Delta h/c^2$, recovering the equivalence-principle estimate.

**Pound-Rebka experiment** (1959): Used the Mössbauer effect to measure the gravitational redshift of gamma rays traveling 22.5 m in the Jefferson Tower at Harvard. Confirmed $\Delta f/f = gh/c^2$ to 10% (later improved to 1% by Pound-Snider 1965).

**Gravity Probe A** (1976): A hydrogen maser clock was launched to 10,000 km altitude. Compared to a ground clock, the satellite clock ran fast by $(4.35 \pm 0.02)\times 10^{-10}$, agreeing with GR prediction to $0.02\%$.

**GPS**: Each GPS satellite carries an atomic clock at altitude 20,200 km. The satellite clocks run 45.9 μs/day faster than ground clocks due to the weaker gravitational field (and 7.2 μs/day slower due to velocity time dilation). Without these corrections, GPS position errors would accumulate at 10 km/day.

