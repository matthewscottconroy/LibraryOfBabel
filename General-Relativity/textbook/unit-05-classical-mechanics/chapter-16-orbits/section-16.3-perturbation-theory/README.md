# Section 16.3: Perturbation Theory and the Precession of Perihelion

---

## Section Introduction

The pure Kepler problem (two bodies under gravity, no other perturbations) has a remarkable property: the elliptical orbits are exactly closed. The orbital period equals the period of the radial oscillation exactly. This is a special property of the 1/r potential — shared only with the harmonic oscillator (Bertrand's theorem).

Any perturbation to the 1/r potential — from other planets, from the Sun's oblateness, or from GR corrections — causes the orbit to precess. The perihelion (point of closest approach) drifts forward (or backward) with each orbit. Computing this precession is the primary application of orbital perturbation theory.

For Mercury, the observed precession rate is 574 arcseconds per century. Newtonian perturbation theory from the other planets (primarily Venus and Jupiter) accounts for 531 arcseconds. The remaining 43 arcseconds per century had no Newtonian explanation — until Einstein computed the GR prediction in 1915.

---

## 16.3.1 The Orbit Equation and Binet's Formula

For a particle of mass m orbiting a central force F(r), with conserved angular momentum ℓ = mr²φ̇, the orbit equation r = r(φ) is found via the substitution u = 1/r.

**Binet's equation**: Let u = 1/r. The orbit equation becomes:

$$\frac{d^2u}{d\phi^2} + u = -\frac{mF(1/u)}{l^2 u^2}$$

where ℓ = mr²φ̇ is the angular momentum.

**For the Newtonian inverse-square force** F = −GMm/r²:

$$\frac{d^2u}{d\phi^2} + u = \frac{GMm^2}{\ell^2}$$

The constant right-hand side gives a simple harmonic oscillator equation. Solution:

$$u(\phi) = \frac{GMm^2}{\ell^2}(1 + e\cos\phi)$$

i.e., $r(\phi) = \frac{a(1-e^2)}{1 + e\cos\phi}$ (a conic section). The orbit is **exactly closed** — returning to the same r after one revolution (φ → φ + 2π). No precession.

---

## 16.3.2 Perturbation of the Orbit Equation

Suppose the force is $F = -GMm/r^2 + F_{\rm pert}(r)$ (Newtonian gravity plus a small perturbation). Binet's equation becomes:

$$\frac{d^2u}{d\phi^2} + u = \frac{GMm^2}{\ell^2} + f(u)$$

where f(u) = $-mF_{\rm pert}(1/u)/(\ell^2 u^2)$ is the perturbation.

**First-order perturbation theory**: Let $u = u_0 + u_1$ where $u_0 = (GMm^2/\ell^2)(1 + e\cos\phi)$ is the unperturbed orbit and $u_1$ is the small correction. Substituting:

$$u_1'' + u_1 = f(u_0)$$

The right-hand side $f(u_0)$ is a function of φ (through $u_0(\phi)$). By the method of undetermined coefficients:
- **Non-resonant terms** (those with frequency ≠ 1): contribute oscillatory corrections to the orbit.
- **Resonant term** (frequency = 1, i.e., proportional to $\cos\phi$): produces a **secularly growing term** of the form $A\phi\sin\phi$.

A secularly growing term $u_1 \propto \phi\sin\phi$ represents a slowly precessing orbit. If we write $u \approx u_0/(GMm^2/\ell^2) = 1 + e\cos\phi \to 1 + e\cos((1-\epsilon)\phi)$, the perihelion has shifted from φ = 0 to φ = πε/(1−ε) ≈ πε after half an orbit, and 2πε per full orbit.

**General result**: For a power-law correction $F_{\rm pert} = -\alpha/r^n$, the perihelion precession per orbit is:

$$\Delta\phi = \frac{\alpha m}{\ell^2 u_0^{n-2}} \cdot \pi (3 - n) \quad \text{(approximately)}$$

for n ≠ 3 (the case n = 3 gives unbounded precession — orbit immediately unstable).

---

## 16.3.3 The GR Correction to the Newtonian Potential

In the weak-field, slow-motion limit, the Schwarzschild geodesic equation gives an effective potential equivalent to Newton's with an additional term:

$$V_{\rm eff}(r) = -\frac{GMm}{r} + \frac{\ell^2}{2mr^2} - \frac{GM\ell^2}{mc^2 r^3}$$

The third term — $-GM\ell^2/(mc^2 r^3)$ — is the **GR correction**. It falls as 1/r³ (stronger than gravity for small r) and causes orbits to precess.

*Derivation*: The Schwarzschild geodesic equation for radial motion gives:

$$\frac{1}{2}\left(\frac{dr}{d\tau}\right)^2 + V_{\rm eff}(r) = \frac{E^2 - 1}{2}$$

where:

$$V_{\rm eff}(r) = -\frac{GM}{r} + \frac{\ell^2}{2r^2} - \frac{GM\ell^2}{c^2 r^3}$$

(with m = 1, c = 1 for simplicity; restoring units adds the $1/(mc^2)$ factors).

The GR correction $-GM\ell^2/(c^2 r^3)$ corresponds to the perturbation force:

$$F_{\rm GR} = -\frac{dV_{\rm GR}}{dr} = -\frac{3GM\ell^2}{c^2 r^4} = -\frac{3GMm^2\dot\phi^2}{c^2 r^2}$$

Binet's equation with this perturbation:

$$u'' + u = \frac{GMm^2}{\ell^2} + \frac{3GM}{\ell^2}u^2 \cdot \ell^2 = \frac{GMm^2}{\ell^2} + 3\frac{GM}{c^2}u^2$$

(The factor of c² appears from the correct unit-bearing form.)

---

## 16.3.4 Deriving the Perihelion Precession

**Perturbed orbit**: Let $u = u_0(1 + e\cos\phi)$ be the Newtonian orbit (with $u_0 = GMm^2/\ell^2$). The perturbation term $3(GM/c^2)u^2$ on the right-hand side of Binet's equation, evaluated at the unperturbed orbit, is:

$$3\frac{GM}{c^2}u_0^2(1 + e\cos\phi)^2 = 3\frac{GM}{c^2}u_0^2\left(1 + 2e\cos\phi + \frac{e^2}{2}(1 + \cos 2\phi)\right)$$

Only the $\cos\phi$ term is resonant (frequency = 1). The amplitude of the resonant forcing is $6(GM/c^2)u_0^2 e$.

The resonant solution to $u_1'' + u_1 = A\cos\phi$ is $u_1 = (A/2)\phi\sin\phi$.

So the perturbed orbit is:

$$u(\phi) \approx u_0\left(1 + e\cos\phi + \frac{3G^2M^2m^2}{\ell^2 c^2}e\phi\sin\phi\right)$$

$$= u_0\left(1 + e\cos\left(\phi\sqrt{1 - \frac{3G^2M^2m^2}{\ell^2 c^2}}\right)\right) + \ldots$$

(approximating $\cos(\alpha\phi) \approx \cos\phi - \phi\alpha'\sin\phi$ for $\alpha \approx 1-\varepsilon$).

The perihelion returns to $r = r_{\min}$ when $\phi = 2\pi(1+\varepsilon)$, having advanced by:

$$\Delta\phi = \frac{6\pi G^2M^2m^2}{\ell^2 c^2} = \frac{6\pi GM}{c^2 a(1-e^2)}$$

where $a$ is the semi-major axis and $e$ is the eccentricity of the orbit.

---

## 16.3.5 Comparison to Observation: Mercury

**Parameters for Mercury**:
- Semi-major axis: $a = 5.791 \times 10^{10}$ m
- Eccentricity: $e = 0.2056$
- Mass of Sun: $M = 1.989 \times 10^{30}$ kg
- $G = 6.674 \times 10^{-11}$ m³/(kg·s²), $c = 3 \times 10^8$ m/s

**GR precession per orbit**:

$$\Delta\phi = \frac{6\pi GM_\odot}{c^2 a(1-e^2)} = \frac{6\pi \times 6.674\times10^{-11} \times 1.989\times10^{30}}{(3\times10^8)^2 \times 5.791\times10^{10} \times (1-0.0423)}$$

$$= \frac{6\pi \times 1.327\times10^{20}}{9\times10^{16} \times 5.791\times10^{10} \times 0.9577}$$

$$= \frac{2.503\times10^{21}}{4.994\times10^{26}} = 5.01 \times 10^{-6} \text{ radians per orbit}$$

Converting to arcseconds per century: Mercury's orbital period is 87.97 days = 0.2408 years. So there are 100/0.2408 = 415 orbits per century.

$$\Delta\phi_{\rm per\ century} = 415 \times 5.01\times10^{-6} \text{ rad} = 2.08\times10^{-3} \text{ rad} = 2.08\times10^{-3} \times (3600 \times 180/\pi) \text{ arcsec} \approx 43 \text{ arcsec/century}$$

**Observed value**: 42.98 ± 0.04 arcseconds per century [Pireaux and Rozelot (2003), *Astrophysics and Space Science* 284, 1159].

**GR prediction**: $\Delta\phi = 42.98$ arcseconds per century.

**Agreement**: Perfect. No free parameters. This is one of the most precise confirmations of GR.

---

## 16.3.6 Other Contributions to Mercury's Precession

For completeness, the full perihelion precession budget:

| Source | Contribution (arcsec/century) |
|--------|-------------------------------|
| Jupiter | 153.6 |
| Venus | 277.9 |
| Earth | 90.0 |
| Other planets | 9.3 |
| **Newtonian total** | **530.8** |
| GR correction | 42.98 |
| Solar oblateness (J₂) | 0.025 |
| **Total predicted** | **573.8** |
| **Observed** | **574.1 ± 0.5** |

The Newtonian contributions from the other planets are computed by perturbation theory (secular perturbations from the disturbing function R in the Lagrange-Laplace equations). The agreement between theory and observation is at the level of ≲ 0.5 arcseconds per century.

---

## References

- Le Verrier, U.J.J. (1859). "Théorie du mouvement de Mercure." *Annales de l'Observatoire Impérial de Paris*, 5, 1–196. [The original measurement of Mercury's anomalous precession — 38 arcsec/century (the modern value is 43), determined by comparing observations from 1697 to 1848 with Newtonian theory.]
- Einstein, A. (1915). "Erklärung der Perihelbewegung des Merkur aus der allgemeinen Relativitätstheorie." *Sitzungsberichte der Königlich Preußischen Akademie der Wissenschaften*, 831–839. [The paper in which Einstein computes the GR precession of Mercury and gets exactly the right answer. He reportedly could not eat or sleep for days afterward.]
- Pireaux, S. and Rozelot, J.-P. (2003). "Solar quadrupole moment and purely relativistic gravitations effects on Mercury's perihelion advance." *Astrophysics and Space Science*, 284, 1159–1194. [Modern precise measurement: 42.98 ± 0.04 arcsec/century.]
- Misner, C.W., Thorne, K.S., and Wheeler, J.A. (1973). *Gravitation.* W.H. Freeman. [§25.5 on the Schwarzschild effective potential; §40.5 on perihelion precession. The derivation follows the approach of this section.]
- Bertrand, J. (1873). "Théorème relatif au mouvement d'un point attiré vers un centre fixe." *Comptes rendus de l'Académie des sciences*, 77, 849–853. [Proves that the only central force laws with all orbits closed are F ∝ r (harmonic oscillator) and F ∝ 1/r² (Newtonian gravity). Any other force law produces precessing orbits.]
