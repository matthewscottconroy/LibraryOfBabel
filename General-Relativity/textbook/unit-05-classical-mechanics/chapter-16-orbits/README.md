# Chapter 16: Orbital Mechanics and the Precession of Perihelion

---

## Chapter Introduction

The two-body problem — two point masses interacting via Newtonian gravity — is one of the triumphs of classical mechanics: it is exactly solvable. The orbits are conic sections (ellipses, parabolas, hyperbolas), the orbital period satisfies Kepler's third law, and the orbit precesses under perturbations in a computable way.

This chapter develops orbital mechanics in enough depth to:
1. Derive Kepler's laws from Newton's law of gravitation.
2. Compute the orbital precession due to perturbations.
3. Identify exactly where Newtonian mechanics fails: Mercury's orbit precesses by 43 arcseconds per century more than Newtonian theory predicts. This is the most famous test of GR.
4. Derive the GR precession formula and compare to observation.

The Kepler problem — the central force problem with V = −k/r — is the Newtonian limit of geodesic motion in Schwarzschild spacetime. The comparison between Newtonian orbits and Schwarzschild geodesics makes concrete what "GR corrections to Newtonian gravity" means. The 43 arcseconds per century of perihelion precession is one of the three classical tests of GR (along with gravitational light deflection and gravitational redshift), and its exact agreement with GR observation is one of the most dramatic confirmations of the theory.

---

## Chapter Contents

- **Section 16.1**: Central Force Motion
  - Reduction to 1D; effective potential
  - Orbit equation; Binet's equation
  - Classification of orbits (bound, unbound, circular)

- **Section 16.2**: Kepler's Laws
  - Derivation from F = −GM/r²
  - Kepler's first law (ellipse), second law (equal areas), third law (T² ∝ a³)
  - The vis-viva equation; orbital energy and angular momentum
  - The Laplace-Runge-Lenz vector

- **Section 16.3**: Perturbation Theory and Perihelion Precession
  - Orbital perturbation theory; disturbing function
  - Newtonian precession from other planets; secular effects
  - Mercury's "anomalous" precession: 43 arcsec/century
  - GR correction to the Newtonian potential: V_GR = −GM/r − GML²/(mr³c²)
  - Derivation of the GR precession: Δφ = 6πGM/(c²a(1−e²)) per orbit
  - Comparison to observation

---

## The Historical Problem

Between 1840 and 1860, the French astronomer Urbain Le Verrier made two landmark predictions. In 1846, he predicted the position of Neptune from anomalies in Uranus's orbit — and Neptune was found within one degree of his prediction. In 1859, he turned the same machinery on Mercury's orbit and found that the observed perihelion advance exceeded the Newtonian prediction by 43 arcseconds per century.

Le Verrier proposed a hypothetical inner planet (Vulcan) to account for this discrepancy. No Vulcan was ever found. Various other Newtonian explanations — a solar oblateness, a ring of dust, a modification of the inverse-square law — were tried and discarded. The discrepancy persisted for 56 years.

When Einstein computed the GR prediction in 1915 and obtained exactly 43 arcseconds per century, he wrote in a letter: "This discovery was, I believe, by far the strongest emotional experience in Einstein's scientific life, perhaps in all his life. Nature had spoken to him."

The agreement between theory and observation was exact. Not within experimental error — exact. No free parameter. Just the mass of Mercury, the mass of the Sun, and the speed of light, all already measured.

This is why the perihelion precession of Mercury is the canonical example of a precise test of GR.
